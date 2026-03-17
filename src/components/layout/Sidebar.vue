<script setup lang="ts">
import { ref } from "vue";
import { useUiStore, type ViewMode } from "../../stores/uiStore";
import Icon from "../ui/Icon.vue";

const uiStore = useUiStore();
const mobileMenuOpen = ref(false);

const menuItems: { id: ViewMode; label: string; icon: string }[] = [
  { id: "dashboard", label: "仪表盘", icon: "home" },
  { id: "projects", label: "项目管理", icon: "folder-kanban" },
  { id: "tasks", label: "任务管理", icon: "check-square" },
  { id: "documents", label: "文档中心", icon: "file-text" },
  { id: "recycleBin", label: "回收站", icon: "trash-2" },
];

function onMenuClick(view: ViewMode) {
  uiStore.setView(view);
  mobileMenuOpen.value = false;
}
</script>

<template>
  <!-- Mobile Menu Button -->
  <div class="lg:hidden fixed top-0 left-0 right-0 z-40 flex items-center justify-between border-b border-[var(--border-default)] bg-[var(--bg-secondary)]/95 backdrop-blur-xl px-4 py-3">
    <div class="flex items-center gap-3">
      <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500 to-violet-600 shadow-lg shadow-indigo-500/30">
        <Icon name="folder-kanban" :size="18" class="text-white" />
      </div>
      <span class="text-sm font-bold text-[var(--text-primary)]">ProjectFlow</span>
    </div>
    <button
      @click="mobileMenuOpen = !mobileMenuOpen"
      class="rounded-lg p-2 text-[var(--text-secondary)] hover:bg-[var(--bg-tertiary)] hover:text-[var(--text-primary)] transition-colors"
    >
      <Icon :name="mobileMenuOpen ? 'x' : 'menu'" :size="22" />
    </button>
  </div>

  <!-- Mobile Menu Overlay -->
  <Transition
    enter-active-class="transition duration-200 ease-out"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition duration-150 ease-in"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div
      v-if="mobileMenuOpen"
      class="fixed inset-0 z-40 bg-black/70 backdrop-blur-sm lg:hidden"
      @click="mobileMenuOpen = false"
    />
  </Transition>

  <!-- Sidebar -->
  <aside
    class="fixed inset-y-0 left-0 z-50 flex w-64 flex-col border-r border-[var(--border-default)] bg-[var(--bg-secondary)]/95 backdrop-blur-xl transition-transform duration-300 lg:relative lg:translate-x-0"
    :class="mobileMenuOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'"
  >
    <!-- Logo (Desktop only) -->
    <div class="hidden lg:flex h-16 items-center border-b border-[var(--border-subtle)] px-4">
      <div class="flex items-center gap-3">
        <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-gradient-to-br from-indigo-500 to-violet-600 shadow-lg shadow-indigo-500/30">
          <Icon name="folder-kanban" :size="20" class="text-white" />
        </div>
        <div class="flex flex-col">
          <span class="text-sm font-bold text-[var(--text-primary)]">ProjectFlow</span>
          <span class="text-[10px] text-[var(--text-tertiary)]">项目管理工具</span>
        </div>
      </div>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 space-y-1.5 p-3 pt-20 lg:pt-4">
      <button
        v-for="item in menuItems"
        :key="item.id"
        @click="onMenuClick(item.id)"
        class="group flex w-full items-center gap-3 rounded-xl px-3.5 py-3 text-sm font-medium transition-all duration-200"
        :class="
          uiStore.currentView === item.id
            ? 'bg-gradient-to-r from-indigo-500/15 to-violet-500/15 text-[var(--text-primary)] border border-indigo-500/25'
            : 'text-[var(--text-secondary)] hover:bg-[var(--bg-tertiary)] hover:text-[var(--text-primary)]'
        "
      >
        <div
          class="flex h-9 w-9 items-center justify-center rounded-lg transition-all duration-200"
          :class="uiStore.currentView === item.id ? 'bg-gradient-to-br from-indigo-500 to-violet-600 shadow-lg shadow-indigo-500/20' : 'bg-[var(--bg-tertiary)] group-hover:bg-[var(--bg-tertiary)]'"
        >
          <Icon :name="item.icon" :size="18" :class="uiStore.currentView === item.id ? 'text-white' : 'text-[var(--text-secondary)] group-hover:text-[var(--text-primary)]'" />
        </div>
        <span class="truncate">{{ item.label }}</span>
      </button>
    </nav>

    <!-- Collapse Button (Desktop only) -->
    <div class="hidden lg:block border-t border-[var(--border-subtle)] p-3">
      <button
        @click="uiStore.toggleSidebar"
        class="flex w-full items-center justify-center gap-2 rounded-xl p-2.5 text-sm text-[var(--text-tertiary)] transition-all duration-200 hover:bg-[var(--bg-tertiary)] hover:text-[var(--text-secondary)]"
      >
        <Icon
          name="chevron-left"
          :size="16"
          class="transition-transform duration-300"
          :class="uiStore.sidebarCollapsed ? 'rotate-180' : ''"
        />
        <span v-if="!uiStore.sidebarCollapsed" class="text-xs">收起</span>
      </button>
    </div>
  </aside>
</template>
