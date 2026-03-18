import { defineStore } from "pinia";
import { ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { llmApi, type LlmMessage, type ModelConfig } from "../lib/api";

// Default model configurations
export const DEFAULT_MODEL_CONFIGS: Record<string, { name: string; base_url: string; default_model: string }> = {
  "kimi": { name: "Kimi", base_url: "https://api.moonshot.cn", default_model: "moonshot-v1-8k" },
  "deepseek": { name: "DeepSeek", base_url: "https://api.deepseek.com", default_model: "deepseek-chat" },
  "minimax": { name: "MiniMax", base_url: "https://api.minimax.chat", default_model: "abab6.5s-chat" },
  "gpt-4o": { name: "GPT-4o", base_url: "https://api.openai.com/v1", default_model: "gpt-4o" },
  "gpt-4o-mini": { name: "GPT-4o Mini", base_url: "https://api.openai.com/v1", default_model: "gpt-4o-mini" },
  "gpt-4-turbo": { name: "GPT-4 Turbo", base_url: "https://api.openai.com/v1", default_model: "gpt-4-turbo" },
  "gpt-3.5-turbo": { name: "GPT-3.5 Turbo", base_url: "https://api.openai.com/v1", default_model: "gpt-3.5-turbo" },
};

export const useLlmStore = defineStore("llm", () => {
  // State
  const messages = ref<LlmMessage[]>([]);
  const isStreaming = ref(false);
  const currentStreamingContent = ref("");
  const apiKeyStatus = ref<Record<string, boolean>>({});
  const settingsOpen = ref(false);
  const selectedModel = ref("gpt-4o");
  const error = ref<string | null>(null);
  const modelConfigs = ref<Record<string, ModelConfig>>({});
  const selectedModelId = ref("gpt-4o");

  // Tauri event listeners
  let unlistenToken: UnlistenFn | null = null;
  let unlistenDone: UnlistenFn | null = null;
  let unlistenError: UnlistenFn | null = null;

  // Initialize event listeners
  async function initEventListeners() {
    unlistenToken = await listen<string>("llm-token", (event) => {
      appendStreamingContent(event.payload);
    });

    unlistenDone = await listen("llm-done", () => {
      finishStreaming();
    });

    unlistenError = await listen<string>("llm-error", (event) => {
      error.value = event.payload;
      isStreaming.value = false;
    });
  }

  // Clean up event listeners
  function cleanupEventListeners() {
    if (unlistenToken) unlistenToken();
    if (unlistenDone) unlistenDone();
    if (unlistenError) unlistenError();
  }

  // Actions
  function addMessage(message: LlmMessage) {
    messages.value.push(message);
  }

  function clearMessages() {
    messages.value = [];
    error.value = null;
  }

  function setStreaming(streaming: boolean) {
    isStreaming.value = streaming;
  }

  function appendStreamingContent(content: string) {
    currentStreamingContent.value += content;
  }

  function finishStreaming() {
    // Add the streaming content as an assistant message
    if (currentStreamingContent.value) {
      messages.value.push({
        role: "assistant",
        content: currentStreamingContent.value,
      });
    }
    currentStreamingContent.value = "";
    isStreaming.value = false;
  }

  async function sendMessage(content: string, projectId?: string) {
    // Add user message
    addMessage({ role: "user", content });

    // Start streaming
    isStreaming.value = true;
    error.value = null;

    // Get the model to use (selectedModelId takes precedence, then selectedModel)
    const modelToUse = selectedModelId.value || selectedModel.value;

    try {
      await llmApi.chat(
        messages.value,
        projectId,
        modelToUse
      );
    } catch (e) {
      error.value = String(e);
      isStreaming.value = false;
    }
  }

  async function saveApiKey(model: string, apiKey: string) {
    await llmApi.saveKey(model, apiKey);
    apiKeyStatus.value[model] = true;
  }

  async function deleteApiKey(model: string) {
    await llmApi.deleteKey(model);
    apiKeyStatus.value[model] = false;
  }

  async function checkKeyStatus(model: string) {
    const hasKey = await llmApi.getKeyStatus(model);
    apiKeyStatus.value[model] = hasKey;
  }

  async function saveModelConfig(modelId: string, config: ModelConfig) {
    await llmApi.saveModelConfig(config);
    modelConfigs.value[modelId] = config;
  }

  async function getModelConfig(modelId: string): Promise<ModelConfig | null> {
    const config = await llmApi.getModelConfig(modelId);
    if (config) {
      modelConfigs.value[modelId] = config;
    }
    return config;
  }

  async function loadAllModelConfigs() {
    // Load configs for all known models
    const modelIds = Object.keys(DEFAULT_MODEL_CONFIGS);
    for (const modelId of modelIds) {
      const config = await llmApi.getModelConfig(modelId);
      if (config) {
        modelConfigs.value[modelId] = config;
      }
    }
  }

  // Initialize on store creation
  initEventListeners();

  return {
    // State
    messages,
    isStreaming,
    currentStreamingContent,
    apiKeyStatus,
    settingsOpen,
    selectedModel,
    error,
    modelConfigs,
    selectedModelId,
    // Actions
    addMessage,
    clearMessages,
    setStreaming,
    appendStreamingContent,
    finishStreaming,
    sendMessage,
    saveApiKey,
    deleteApiKey,
    checkKeyStatus,
    saveModelConfig,
    getModelConfig,
    loadAllModelConfigs,
    cleanupEventListeners,
  };
});
