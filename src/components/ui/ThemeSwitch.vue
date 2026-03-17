<script setup lang="ts">
import { useUiStore } from "../../stores/uiStore";

const uiStore = useUiStore();

function toggle() {
  uiStore.toggleTheme();
}
</script>

<template>
  <button
    @click="toggle"
    class="theme-switch relative inline-flex h-8 w-14 items-center rounded-full transition-colors duration-300 focus:outline-none focus:ring-2 focus:ring-[var(--accent-primary)] focus:ring-offset-2 focus:ring-offset-[var(--bg-secondary)]"
    :class="uiStore.theme === 'dark' ? 'bg-[var(--bg-tertiary)]' : 'bg-[var(--accent-primary)]'"
    :title="uiStore.theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'"
    role="switch"
    :aria-checked="uiStore.theme === 'light'"
  >
    <!-- Track -->
    <span
      class="absolute inset-1 rounded-full transition-colors duration-300"
      :class="uiStore.theme === 'dark' ? 'bg-[var(--bg-tertiary)]' : 'bg-white/20'"
    />

    <!-- Sliding Circle -->
    <span
      class="absolute h-6 w-6 rounded-full shadow-md flex items-center justify-center transition-transform duration-300 ease-spring"
      :class="uiStore.theme === 'dark' ? 'translate-x-1.5 bg-[var(--bg-elevated)]' : 'translate-x-7 bg-white'"
    >
      <!-- Moon Icon (dark mode - left) -->
      <svg
        v-if="uiStore.theme === 'dark'"
        xmlns="http://www.w3.org/2000/svg"
        class="w-3.5 h-3.5 text-[var(--text-secondary)]"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
      </svg>

      <!-- Sun Icon (light mode - right) -->
      <svg
        v-else
        xmlns="http://www.w3.org/2000/svg"
        class="w-3.5 h-3.5 text-[var(--accent-primary)]"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <circle cx="12" cy="12" r="5" />
        <line x1="12" y1="1" x2="12" y2="3" />
        <line x1="12" y1="21" x2="12" y2="23" />
        <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
        <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
        <line x1="1" y1="12" x2="3" y2="12" />
        <line x1="21" y1="12" x2="23" y2="12" />
        <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
        <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
      </svg>
    </span>
  </button>
</template>

<style scoped>
/* Spring-like easing for smooth animation */
.ease-spring {
  transition: transform 300ms cubic-bezier(0.34, 1.56, 0.64, 1);
}
</style>
