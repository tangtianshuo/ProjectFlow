<script setup lang="ts">
import { computed } from "vue";

interface Props {
  modelValue?: string | null;
  placeholder?: string;
  type?: string;
  disabled?: boolean;
  error?: boolean;
  icon?: string;
}

withDefaults(defineProps<Props>(), {
  modelValue: "",
  placeholder: "",
  type: "text",
  disabled: false,
  error: false,
});

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

const inputClasses = computed(() => {
  const base = "w-full bg-[var(--bg-tertiary)] border text-[var(--text-primary)] placeholder-[var(--text-tertiary)] transition-all duration-200 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50";
  const errorState = "border-red-500/50 focus:border-red-500 focus:ring-2 focus:ring-red-500/20";
  const normalState = "border-[var(--border-default)] focus:border-[var(--accent-primary)] focus:ring-2 focus:ring-[var(--accent-primary)]/20";

  return `${base} ${errorState ? errorState : normalState}`;
});

function onInput(event: Event) {
  const target = event.target as HTMLInputElement;
  emit("update:modelValue", target.value);
}
</script>

<template>
  <div class="relative">
    <input
      :type="type"
      :value="modelValue"
      :placeholder="placeholder"
      :disabled="disabled"
      @input="onInput"
      :class="[
        inputClasses,
        icon ? 'pl-10' : '',
        'rounded-lg px-3.5 py-2.5 text-sm'
      ]"
    />
    <div
      v-if="icon"
      class="absolute left-3 top-1/2 -translate-y-1/2 text-[var(--text-tertiary)]"
    >
      <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
    </div>
  </div>
</template>
