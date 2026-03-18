import { describe, it, expect, vi, beforeEach } from "vitest";
import { llmApi } from "./api";

// Mock Tauri invoke
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";

describe("llmApi", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("saveKey", () => {
    it("should call llm_save_key command with model and apiKey", async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);

      await llmApi.saveKey("gpt-4o", "test-api-key");

      expect(invoke).toHaveBeenCalledWith("llm_save_key", {
        model: "gpt-4o",
        apiKey: "test-api-key",
      });
    });
  });

  describe("getKeyStatus", () => {
    it("should call llm_get_key_status command and return boolean", async () => {
      vi.mocked(invoke).mockResolvedValue(true);

      const result = await llmApi.getKeyStatus("gpt-4o");

      expect(invoke).toHaveBeenCalledWith("llm_get_key_status", {
        model: "gpt-4o",
      });
      expect(result).toBe(true);
    });

    it("should return false when no key is stored", async () => {
      vi.mocked(invoke).mockResolvedValue(false);

      const result = await llmApi.getKeyStatus("gpt-4o-mini");

      expect(result).toBe(false);
    });
  });

  describe("deleteKey", () => {
    it("should call llm_delete_key command", async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);

      await llmApi.deleteKey("gpt-4o");

      expect(invoke).toHaveBeenCalledWith("llm_delete_key", {
        model: "gpt-4o",
      });
    });
  });

  describe("chat", () => {
    it("should call llm_chat command with messages", async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);

      const messages = [
        { role: "user" as const, content: "Hello" },
        { role: "assistant" as const, content: "Hi there!" },
      ];

      await llmApi.chat(messages);

      expect(invoke).toHaveBeenCalledWith("llm_chat", {
        messages,
        projectId: undefined,
        model: undefined,
      });
    });

    it("should pass projectId and model when provided", async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);

      const messages = [{ role: "user" as const, content: "Help me" }];

      await llmApi.chat(messages, "project-123", "gpt-4o");

      expect(invoke).toHaveBeenCalledWith("llm_chat", {
        messages,
        projectId: "project-123",
        model: "gpt-4o",
      });
    });
  });

  describe("getModels", () => {
    it("should call llm_get_models and return model list", async () => {
      vi.mocked(invoke).mockResolvedValue([
        { id: "gpt-4o", name: "GPT-4o", description: "Latest flagship" },
        { id: "gpt-4o-mini", name: "GPT-4o Mini", description: "Smaller, faster" },
      ]);

      const result = await llmApi.getModels();

      expect(invoke).toHaveBeenCalledWith("llm_get_models");
      expect(result).toHaveLength(2);
      expect(result[0].id).toBe("gpt-4o");
    });
  });
});
