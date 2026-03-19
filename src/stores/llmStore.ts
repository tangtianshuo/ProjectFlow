import { defineStore } from "pinia";
import { ref, watch } from "vue";
import { llmApi, type LlmMessage, type ModelConfig } from "../lib/api";

export type LLMProvider = "openai" | "anthropic" | "openai-compatible";

export const DEFAULT_MODEL_CONFIGS: Record<string, { name: string; base_url: string; default_model: string }> = {
  "openai": { name: "OpenAI", base_url: "https://api.openai.com/v1", default_model: "gpt-4o" },
  "anthropic": { name: "Anthropic", base_url: "https://api.anthropic.com", default_model: "claude-3-5-sonnet-20241022" },
  "openai-compatible": { name: "OpenAI Compatible", base_url: "https://api.openai.com/v1", default_model: "gpt-4o-mini" },
  "kimi": { name: "Kimi", base_url: "https://api.moonshot.cn/v1", default_model: "moonshot-v1-8k" },
  "deepseek": { name: "DeepSeek", base_url: "https://api.deepseek.com/v1", default_model: "deepseek-chat" },
  "minimax": { name: "MiniMax", base_url: "https://api.minimax.chat/v1", default_model: "abab6.5s-chat" },
};

export const useLlmStore = defineStore("llm", () => {
  // State
  const messages = ref<LlmMessage[]>([]);
  const isStreaming = ref(false);
  const streamingContent = ref("");
  const settingsOpen = ref(false);
  const selectedModel = ref("gpt-4o");
  const error = ref<string | null>(null);
  const sidecarStarted = ref(false);
  const isStarting = ref(false);
  const isLoading = ref(true);
  
  // Model Config (loaded from database)
  const modelConfig = ref<ModelConfig | null>(null);

  // Load saved model from localStorage
  const savedModelId = localStorage.getItem("selectedModelId");
  const selectedModelId = ref(savedModelId || "openai");

  // Watch for model changes and save to localStorage
  watch(selectedModelId, (newValue) => {
    localStorage.setItem("selectedModelId", newValue);
  });

  // Load settings from database
  async function loadSettings() {
    isLoading.value = true;
    try {
      const settings = await llmApi.getSettings();
      
      modelConfig.value = {
        model_id: settings.provider,
        base_url: settings.api_url,
        model_name: settings.model,
        api_key: settings.api_key || "",
      };
      
      selectedModelId.value = settings.provider;
      console.log("Loaded LLM settings from database:", settings);
    } catch (e) {
      console.error("Failed to load LLM settings:", e);
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }

  // Initialize sidecar on store creation
  async function initializeSidecar() {
    if (sidecarStarted.value || isStarting.value) return;
    
    isStarting.value = true;
    try {
      const status = await llmApi.getSidecarStatus();
      if (status) {
        sidecarStarted.value = true;
        console.log("Sidecar already running on port:", status);
      } else {
        const port = await llmApi.startSidecar();
        sidecarStarted.value = true;
        console.log("Sidecar started on port:", port);
      }
    } catch (e) {
      console.error("Failed to start sidecar:", e);
      error.value = String(e);
    } finally {
      isStarting.value = false;
    }
  }

  // Actions
  function addMessage(message: LlmMessage) {
    messages.value.push(message);
  }

  function clearMessages() {
    messages.value = [];
    streamingContent.value = "";
    error.value = null;
  }

  function setStreaming(streaming: boolean) {
    isStreaming.value = streaming;
    if (!streaming) {
      streamingContent.value = "";
    }
  }

  function updateStreamingContent(content: string) {
    streamingContent.value = content;
  }

  async function sendMessage(content: string, systemPrompt?: string) {
    addMessage({ role: "user", content });

    if (!modelConfig.value) {
      error.value = "LLM config is not set. Please set it in settings.";
      return;
    }

    if (!sidecarStarted.value) {
      await initializeSidecar();
    }

    isStreaming.value = true;
    streamingContent.value = "";
    error.value = null;

    try {
      const chatMessages: LlmMessage[] = [];
      
      if (systemPrompt) {
        chatMessages.push({ role: "system", content: systemPrompt });
      }
      
      chatMessages.push(...messages.value);

      // Use streaming API
      const response = await llmApi.chatWithLlmStream(
        chatMessages,
        0.7,
        undefined
      );

      streamingContent.value = response;
      addMessage({ role: "assistant", content: response });
    } catch (e) {
      error.value = String(e);
    } finally {
      isStreaming.value = false;
      streamingContent.value = "";
    }
  }

  async function saveModelConfig(config: ModelConfig) {
    modelConfig.value = config;
    
    try {
      // Save to database (doesn't need sidecar)
      await llmApi.saveSettings(
        config.model_id || "openai",
        config.api_key || "",
        config.base_url,
        config.model_name
      );
      
      console.log("LLM settings saved to database");
      
      // Try to update sidecar config (requires sidecar to be running)
      if (sidecarStarted.value) {
        try {
          await llmApi.updateConfig(
            config.model_id || "openai",
            config.api_key || "",
            config.base_url,
            config.model_name
          );
          console.log("Sidecar config updated");
        } catch (sidecarError) {
          console.warn("Sidecar not available, will use config on next restart:", sidecarError);
        }
      } else {
        // Try to initialize sidecar
        await initializeSidecar();
        if (sidecarStarted.value) {
          await llmApi.updateConfig(
            config.model_id || "openai",
            config.api_key || "",
            config.base_url,
            config.model_name
          );
        }
      }
    } catch (e) {
      console.error("Failed to save config:", e);
      error.value = String(e);
    }
  }

  return {
    // State
    messages,
    isStreaming,
    streamingContent,
    settingsOpen,
    selectedModel,
    error,
    modelConfig,
    selectedModelId,
    sidecarStarted,
    isStarting,
    isLoading,
    
    // Actions
    loadSettings,
    initializeSidecar,
    addMessage,
    clearMessages,
    setStreaming,
    updateStreamingContent,
    sendMessage,
    saveModelConfig,
  };
});
