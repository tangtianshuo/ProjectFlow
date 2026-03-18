<script setup lang="ts">
import { ref, nextTick, watch, onMounted } from "vue";
import Button from "../../ui/Button.vue";
import ChatMessage from "./ChatMessage.vue";
import LlmSettings from "./LlmSettings.vue";
import { useLlmStore } from "../../../stores/llmStore";
import { useUiStore } from "../../../stores/uiStore";

const llmStore = useLlmStore();
const uiStore = useUiStore();

const messageInput = ref("");
const messagesContainer = ref<HTMLElement | null>(null);

// Scroll to bottom when new messages arrive
watch(
  () => llmStore.messages.length,
  async () => {
    await nextTick();
    scrollToBottom();
  }
);

// Scroll to bottom when streaming content changes
watch(
  () => llmStore.currentStreamingContent,
  async () => {
    await nextTick();
    scrollToBottom();
  }
);

function scrollToBottom() {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight;
  }
}

async function sendMessage() {
  if (!messageInput.value.trim() || llmStore.isStreaming) return;

  const content = messageInput.value.trim();
  messageInput.value = "";

  await llmStore.sendMessage(content, uiStore.selectedProjectId || undefined);
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === "Enter" && !event.shiftKey) {
    event.preventDefault();
    sendMessage();
  }
}

function openSettings() {
  llmStore.settingsOpen = true;
}

function closeSettings() {
  llmStore.settingsOpen = false;
}

onMounted(() => {
  scrollToBottom();
});
</script>

<template>
  <div class="flex flex-col h-full bg-[var(--bg-primary)]">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-[var(--border-subtle)] px-4 py-3">
      <h2 class="text-lg font-semibold text-[var(--text-primary)]">AI 助手</h2>
      <Button variant="ghost" size="sm" icon @click="openSettings">
        <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </Button>
    </div>

    <!-- Messages -->
    <div
      ref="messagesContainer"
      class="flex-1 overflow-y-auto p-4 space-y-4"
    >
      <!-- Empty state -->
      <div
        v-if="llmStore.messages.length === 0 && !llmStore.isStreaming"
        class="flex flex-col items-center justify-center h-full text-center"
      >
        <svg class="h-16 w-16 text-[var(--text-tertiary)] mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
        </svg>
        <p class="text-[var(--text-secondary)]">开始与 AI 助手对话</p>
        <p class="text-sm text-[var(--text-tertiary)] mt-1">发送消息开始对话</p>
      </div>

      <!-- Messages list -->
      <ChatMessage
        v-for="(message, index) in llmStore.messages"
        :key="index"
        :message="message"
      />

      <!-- Streaming indicator -->
      <div v-if="llmStore.isStreaming" class="flex items-center gap-2 text-[var(--text-tertiary)]">
        <div class="flex gap-1">
          <span class="w-2 h-2 bg-indigo-500 rounded-full animate-bounce" style="animation-delay: 0ms" />
          <span class="w-2 h-2 bg-indigo-500 rounded-full animate-bounce" style="animation-delay: 150ms" />
          <span class="w-2 h-2 bg-indigo-500 rounded-full animate-bounce" style="animation-delay: 300ms" />
        </div>
        <span class="text-sm">AI 正在思考...</span>
      </div>

      <!-- Current streaming content -->
      <ChatMessage
        v-if="llmStore.currentStreamingContent"
        :message="{ role: 'assistant', content: llmStore.currentStreamingContent }"
      />
    </div>

    <!-- Error message -->
    <div
      v-if="llmStore.error"
      class="mx-4 mb-2 p-3 bg-red-500/10 border border-red-500/20 rounded-lg text-sm text-red-400"
    >
      {{ llmStore.error }}
    </div>

    <!-- Input area -->
    <div class="border-t border-[var(--border-subtle)] p-4">
      <div class="flex gap-2">
        <textarea
          v-model="messageInput"
          placeholder="输入消息... (Enter 发送, Shift+Enter 换行)"
          class="flex-1 bg-[var(--bg-tertiary)] border border-[var(--border-default)] rounded-lg px-4 py-3 text-sm text-[var(--text-primary)] placeholder-[var(--text-tertiary)] resize-none focus:outline-none focus:border-[var(--accent-primary)]"
          rows="2"
          :disabled="llmStore.isStreaming"
          @keydown="handleKeydown"
        />
        <Button
          variant="gradient"
          :disabled="!messageInput.trim() || llmStore.isStreaming"
          @click="sendMessage"
        >
          <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
          </svg>
        </Button>
      </div>
    </div>

    <!-- Settings modal -->
    <LlmSettings :open="llmStore.settingsOpen" @close="closeSettings" />
  </div>
</template>
