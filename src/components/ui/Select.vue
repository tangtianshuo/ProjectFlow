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
    class="w-full rounded-lg border border-white/10 bg-[#1a1a25] px-4 py-2.5 text-sm text-white transition-all duration-200 focus:border-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/20 disabled:cursor-not-allowed disabled:opacity-50"
  >
    <slot />
  </select>
</template>
