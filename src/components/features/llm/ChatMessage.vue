<script setup lang="ts">
import { computed } from "vue";
import { marked } from "marked";
import type { LlmMessage } from "../../../lib/api";

interface Props {
  message: LlmMessage;
}

const props = defineProps<Props>();

const renderedContent = computed(() => {
  return marked(props.message.content);
});

const isUser = computed(() => props.message.role === "user");
const isAssistant = computed(() => props.message.role === "assistant");
</script>

<template>
  <div
    :class="[
      'flex w-full',
      isUser ? 'justify-end' : 'justify-start'
    ]"
  >
    <div
      :class="[
        'max-w-[80%] rounded-2xl px-4 py-3',
        isUser
          ? 'bg-gradient-to-r from-indigo-500 to-violet-600 text-white'
          : 'bg-[var(--bg-tertiary)] text-[var(--text-primary)]'
      ]"
    >
      <!-- Role indicator for assistant -->
      <div v-if="isAssistant" class="mb-1 text-xs text-[var(--text-tertiary)]">
        AI
      </div>

      <!-- Message content with markdown rendering -->
      <div
        class="prose prose-invert prose-sm max-w-none"
        v-html="renderedContent"
      />

      <!-- Role indicator for user -->
      <div v-if="isUser" class="mt-1 text-right text-xs text-white/70">
        你
      </div>
    </div>
  </div>
</template>
