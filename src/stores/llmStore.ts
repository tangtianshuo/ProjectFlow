import { defineStore } from "pinia";
import { ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { llmApi, type LlmMessage } from "../lib/api";

export const useLlmStore = defineStore("llm", () => {
  // State
  const messages = ref<LlmMessage[]>([]);
  const isStreaming = ref(false);
  const currentStreamingContent = ref("");
  const apiKeyStatus = ref<Record<string, boolean>>({});
  const settingsOpen = ref(false);
  const selectedModel = ref("gpt-4o");
  const error = ref<string | null>(null);

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

    try {
      await llmApi.chat(
        messages.value,
        projectId,
        selectedModel.value
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
    cleanupEventListeners,
  };
});
