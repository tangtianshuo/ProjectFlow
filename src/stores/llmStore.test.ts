import { describe, it, expect, vi, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useLlmStore } from "./llmStore";

// Mock Tauri events
vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(),
  UnlistenFn: vi.fn(),
}));

// Mock api.ts
vi.mock("../lib/api", () => ({
  llmApi: {
    chat: vi.fn(),
    saveKey: vi.fn(),
    deleteKey: vi.fn(),
    getKeyStatus: vi.fn().mockResolvedValue(true),
    getModels: vi.fn().mockResolvedValue([
      { id: "gpt-4o", name: "GPT-4o", description: "Latest" },
    ]),
  },
}));

import { type LlmMessage } from "../lib/api";

describe("useLlmStore", () => {
  beforeEach(() => {
    const pinia = createPinia();
    setActivePinia(pinia);
    vi.clearAllMocks();
  });

  it("should initialize with empty messages", () => {
    const store = useLlmStore();
    expect(store.messages).toEqual([]);
  });

  it("should initialize with isStreaming false", () => {
    const store = useLlmStore();
    expect(store.isStreaming).toBe(false);
  });

  it("should add a message to messages array", () => {
    const store = useLlmStore();
    const message: LlmMessage = { role: "user", content: "Hello" };

    store.addMessage(message);

    expect(store.messages).toHaveLength(1);
    expect(store.messages[0].content).toBe("Hello");
  });

  it("should clear all messages", () => {
    const store = useLlmStore();
    store.addMessage({ role: "user", content: "Hello" });
    store.addMessage({ role: "assistant", content: "Hi" });

    store.clearMessages();

    expect(store.messages).toEqual([]);
  });

  it("should set streaming state", () => {
    const store = useLlmStore();

    store.setStreaming(true);
    expect(store.isStreaming).toBe(true);

    store.setStreaming(false);
    expect(store.isStreaming).toBe(false);
  });

  it("should append streaming content", () => {
    const store = useLlmStore();

    store.appendStreamingContent("Hello");
    expect(store.currentStreamingContent).toBe("Hello");

    store.appendStreamingContent(" World");
    expect(store.currentStreamingContent).toBe("Hello World");
  });

  it("should finish streaming and add message", () => {
    const store = useLlmStore();
    store.setStreaming(true);
    store.appendStreamingContent("Response text");

    store.finishStreaming();

    expect(store.isStreaming).toBe(false);
    expect(store.currentStreamingContent).toBe("");
    expect(store.messages).toHaveLength(1);
    expect(store.messages[0].role).toBe("assistant");
    expect(store.messages[0].content).toBe("Response text");
  });

  it("should toggle settings open state", () => {
    const store = useLlmStore();
    expect(store.settingsOpen).toBe(false);

    store.settingsOpen = true;
    expect(store.settingsOpen).toBe(true);
  });

  it("should have default model", () => {
    const store = useLlmStore();
    expect(store.selectedModel).toBe("gpt-4o");
  });
});
