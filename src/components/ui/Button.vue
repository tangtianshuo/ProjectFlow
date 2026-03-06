<script setup lang="ts">
interface Props {
  variant?: "primary" | "secondary" | "danger" | "ghost" | "gradient";
  size?: "sm" | "md" | "lg";
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  variant: "gradient",
  size: "md",
  disabled: false,
});
</script>

<template>
  <button
    class="inline-flex items-center justify-center rounded-lg font-medium transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-[#0a0a0f] disabled:opacity-50 disabled:pointer-events-none"
    :class="{
      'bg-gradient-to-r from-indigo-500 to-violet-600 text-white hover:from-indigo-600 hover:to-violet-700 focus:ring-indigo-500 shadow-lg shadow-indigo-500/25': variant === 'gradient',
      'bg-[#1a1a25] text-gray-200 border border-white/10 hover:bg-[#252535] hover:border-white/20 focus:ring-gray-500': variant === 'secondary',
      'bg-red-500/20 text-red-400 border border-red-500/30 hover:bg-red-500/30 focus:ring-red-500': variant === 'danger',
      'bg-transparent text-gray-400 hover:text-white hover:bg-white/5': variant === 'ghost',
      'px-3 py-1.5 text-xs': size === 'sm',
      'px-4 py-2 text-sm': size === 'md',
      'px-6 py-3 text-base': size === 'lg',
    }"
    :disabled="disabled"
  >
    <slot />
  </button>
</template>
