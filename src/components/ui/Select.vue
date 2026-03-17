<script setup lang="ts">
interface Props {
  modelValue?: string | number;
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  modelValue: "",
  disabled: false,
});

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

function onChange(event: Event) {
  const target = event.target as HTMLSelectElement;
  emit("update:modelValue", target.value);
}
</script>

<template>
  <select
    :value="modelValue"
    :disabled="disabled"
    @change="onChange"
    class="w-full rounded-lg border border-[var(--border-default)] bg-[var(--bg-tertiary)] px-4 py-2.5 text-sm text-[var(--text-primary)] transition-all duration-200 focus:border-[var(--accent-primary)] focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)]/20 disabled:cursor-not-allowed disabled:opacity-50"
  >
    <slot />
  </select>
</template>
