<script setup lang="ts">
import { watch } from "vue";

interface Props {
  open: boolean;
  title?: string;
  size?: "sm" | "md" | "lg" | "xl";
}

const props = withDefaults(defineProps<Props>(), {
  open: false,
  title: "",
  size: "md",
});

const emit = defineEmits<{
  (e: "close"): void;
}>();

function onBackdropClick() {
  emit("close");
}

function onEscape(event: KeyboardEvent) {
  if (event.key === "Escape") {
    emit("close");
  }
}

watch(
  () => props.open,
  (isOpen) => {
    if (isOpen) {
      document.addEventListener("keydown", onEscape);
      document.body.style.overflow = "hidden";
    } else {
      document.removeEventListener("keydown", onEscape);
      document.body.style.overflow = "";
    }
  }
);

const sizeClasses = {
  sm: "max-w-sm",
  md: "max-w-lg",
  lg: "max-w-2xl",
  xl: "max-w-4xl",
};
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="open"
        class="fixed inset-0 z-50 flex items-center justify-center p-4"
      >
        <!-- Backdrop with blur -->
        <div
          class="absolute inset-0 bg-black/70 backdrop-blur-md"
          @click="onBackdropClick"
        />

        <!-- Dialog -->
        <Transition
          enter-active-class="transition duration-200 ease-out"
          enter-from-class="opacity-0 scale-95 translate-y-4"
          enter-to-class="opacity-100 scale-100 translate-y-0"
          leave-active-class="transition duration-150 ease-in"
          leave-from-class="opacity-100 scale-100 translate-y-0"
          leave-to-class="opacity-0 scale-95 translate-y-4"
        >
          <div
            v-if="open"
            :class="[
              'relative w-full overflow-hidden rounded-2xl border border-[var(--border-default)] bg-[var(--bg-secondary)]/95 shadow-2xl shadow-black/50 backdrop-blur-xl',
              sizeClasses[size]
            ]"
            role="dialog"
            aria-modal="true"
          >
            <!-- Gradient border effect -->
            <div class="absolute inset-0 rounded-2xl bg-gradient-to-r from-indigo-500/5 via-violet-500/5 to-cyan-500/5 pointer-events-none" />

            <!-- Header -->
            <div v-if="title" class="relative border-b border-[var(--border-subtle)] px-6 py-4">
              <div class="flex items-center justify-between">
                <h3 class="text-lg font-semibold text-zinc-100">{{ title }}</h3>
                <button
                  @click="emit('close')"
                  class="rounded-lg p-1.5 text-[var(--text-secondary)] transition-all hover:bg-[var(--bg-tertiary)] hover:text-[var(--text-primary)]"
                >
                  <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                  </svg>
                </button>
              </div>
            </div>

            <!-- Body -->
            <div class="relative px-6 py-5">
              <slot />
            </div>

            <!-- Footer -->
            <div v-if="$slots.footer" class="relative border-t border-[var(--border-subtle)] px-6 py-4 bg-[var(--bg-tertiary)]">
              <slot name="footer" />
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>
