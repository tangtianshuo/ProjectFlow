<script setup lang="ts">
import { watch } from "vue";

interface Props {
  open: boolean;
  title?: string;
}

const props = defineProps<Props>();
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
        class="fixed inset-0 z-50 flex items-center justify-center"
      >
        <!-- Backdrop with blur -->
        <div
          class="absolute inset-0 bg-black/60 backdrop-blur-sm"
          @click="onBackdropClick"
        />

        <!-- Dialog -->
        <div
          class="relative z-50 w-full max-w-lg overflow-hidden rounded-2xl border border-white/10 bg-[#12121a]/95 shadow-2xl shadow-black/50 backdrop-blur-xl"
          role="dialog"
          aria-modal="true"
        >
          <!-- Gradient border effect -->
          <div class="absolute inset-0 rounded-2xl bg-gradient-to-r from-indigo-500/10 to-violet-500/10 pointer-events-none" />

          <div v-if="title" class="relative border-b border-white/5 px-6 py-4">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-semibold text-white">{{ title }}</h3>
              <button
                @click="emit('close')"
                class="rounded-lg p-1.5 text-gray-500 transition-colors hover:bg-white/5 hover:text-white"
              >
                <svg class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
              </button>
            </div>
          </div>
          <div class="relative px-6 py-4">
            <slot />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>
