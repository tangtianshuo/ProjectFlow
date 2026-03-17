<script setup lang="ts">
import { computed } from "vue";

interface Props {
  variant?: "primary" | "secondary" | "danger" | "ghost" | "gradient" | "outline";
  size?: "sm" | "md" | "lg";
  disabled?: boolean;
  loading?: boolean;
  icon?: boolean;
  glow?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  variant: "primary",
  size: "md",
  disabled: false,
  loading: false,
  icon: false,
  glow: false,
});

const emit = defineEmits<{
  (e: "click", event: MouseEvent): void;
}>();

const classes = computed(() => {
  const base = "inline-flex items-center justify-center font-medium transition-all duration-200 focus:outline-none disabled:opacity-50 disabled:pointer-events-none";

  const variants = {
    primary: "bg-[var(--bg-elevated)] text-[var(--text-primary)] border border-[var(--border-default)] hover:bg-[var(--bg-tertiary)] hover:border-[var(--border-hover)]",
    secondary: "bg-[var(--bg-tertiary)] text-[var(--text-secondary)] border border-[var(--border-default)] hover:bg-[var(--bg-elevated)] hover:border-[var(--border-hover)]",
    danger: "bg-red-500/15 text-[var(--accent-red)] border border-red-500/25 hover:bg-red-500/25",
    ghost: "bg-transparent text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-white/5",
    gradient: "bg-gradient-to-r from-[var(--accent-primary)] to-[var(--accent-secondary)] text-white hover:shadow-lg hover:shadow-[var(--accent-primary)]/20",
    outline: "bg-transparent text-[var(--text-primary)] border border-[var(--border-default)] hover:bg-white/5 hover:border-[var(--border-hover)]",
  };

  const sizes = {
    sm: props.icon ? "p-1.5" : "px-2.5 py-1.5 text-xs",
    md: props.icon ? "p-2" : "px-3.5 py-2 text-sm",
    lg: props.icon ? "p-2.5" : "px-5 py-2.5 text-sm",
  };

  const glowClass = props.glow ? "shadow-lg shadow-indigo-500/20" : "";
  const iconClass = props.icon ? "rounded-lg" : "rounded-lg";

  return `${base} ${variants[props.variant]} ${sizes[props.size]} ${glowClass} ${iconClass}`;
});

function handleClick(event: MouseEvent) {
  if (!props.disabled && !props.loading) {
    emit("click", event);
  }
}
</script>

<template>
  <button
    :class="classes"
    :disabled="disabled || loading"
    @click="handleClick"
  >
    <svg
      v-if="loading"
      class="animate-spin -ml-1 mr-2 h-4 w-4"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      />
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
    <slot />
  </button>
</template>
