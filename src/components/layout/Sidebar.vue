<script setup lang="ts">
import { ref } from "vue";
import { useUiStore, type ViewMode } from "../../stores/uiStore";

const uiStore = useUiStore();
const mobileMenuOpen = ref(false);

const menuItems: { id: ViewMode; label: string; icon: string }[] = [
  { id: "dashboard", label: "仪表盘", icon: "home" },
  { id: "projects", label: "项目管理", icon: "folder" },
  { id: "tasks", label: "任务管理", icon: "tasks" },
  { id: "documents", label: "文档中心", icon: "document" },
  { id: "recycleBin", label: "回收站", icon: "trash" },
];

function onMenuClick(view: ViewMode) {
  uiStore.setView(view);
  mobileMenuOpen.value = false;
}
</script>

<template>
  <!-- Mobile Menu Button -->
  <div class="lg:hidden fixed top-0 left-0 right-0 z-40 flex items-center justify-between border-b border-white/5 bg-[#0d0d12]/95 backdrop-blur-xl px-4 py-3">
    <div class="flex items-center gap-3">
      <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500 to-violet-600">
        <svg class="h-4 w-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
        </svg>
      </div>
      <span class="text-sm font-bold text-white">ProjectFlow</span>
    </div>
    <button
      @click="mobileMenuOpen = !mobileMenuOpen"
      class="rounded-lg p-2 text-gray-400 hover:bg-white/5 hover:text-white"
    >
      <svg v-if="!mobileMenuOpen" class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
      </svg>
      <svg v-else class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
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
      class="fixed inset-0 z-40 bg-black/60 lg:hidden"
      @click="mobileMenuOpen = false"
    />
  </Transition>

  <!-- Sidebar -->
  <aside
    class="fixed inset-y-0 left-0 z-50 flex w-60 flex-col border-r border-white/5 bg-[#0d0d12]/95 backdrop-blur-xl transition-transform duration-300 lg:relative lg:translate-x-0"
    :class="mobileMenuOpen ? 'translate-x-0' : '-translate-x-full lg:translate-x-0'"
  >
    <!-- Logo (Desktop only) -->
    <div class="hidden lg:flex h-16 items-center border-b border-white/5 px-4">
      <div class="flex items-center gap-3">
        <div class="flex h-9 w-9 items-center justify-center rounded-xl bg-gradient-to-br from-indigo-500 to-violet-600 shadow-lg shadow-indigo-500/30">
          <svg class="h-5 w-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
        </div>
        <div class="flex flex-col">
          <span class="text-sm font-bold text-white">ProjectFlow</span>
          <span class="text-[10px] text-gray-500">项目管理工具</span>
        </div>
      </div>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 space-y-1 p-3 pt-20 lg:pt-3">
      <button
        v-for="item in menuItems"
        :key="item.id"
        @click="onMenuClick(item.id)"
        class="group flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-all duration-200"
        :class="
          uiStore.currentView === item.id
            ? 'bg-gradient-to-r from-indigo-500/20 to-violet-500/20 text-white border border-indigo-500/30'
            : 'text-gray-400 hover:bg-white/5 hover:text-white'
        "
      >
        <div
          class="flex h-8 w-8 items-center justify-center rounded-lg transition-all duration-200"
          :class="uiStore.currentView === item.id ? 'bg-gradient-to-br from-indigo-500 to-violet-600 shadow-lg shadow-indigo-500/20' : 'bg-white/5 group-hover:bg-white/10'"
        >
          <svg v-if="item.icon === 'home'" class="h-4 w-4" :class="uiStore.currentView === item.id ? 'text-white' : 'text-gray-400 group-hover:text-white'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
          </svg>
          <svg v-else-if="item.icon === 'folder'" class="h-4 w-4" :class="uiStore.currentView === item.id ? 'text-white' : 'text-gray-400 group-hover:text-white'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
          <svg v-else-if="item.icon === 'tasks'" class="h-4 w-4" :class="uiStore.currentView === item.id ? 'text-white' : 'text-gray-400 group-hover:text-white'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
          </svg>
          <svg v-else-if="item.icon === 'document'" class="h-4 w-4" :class="uiStore.currentView === item.id ? 'text-white' : 'text-gray-400 group-hover:text-white'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <svg v-else-if="item.icon === 'trash'" class="h-4 w-4" :class="uiStore.currentView === item.id ? 'text-white' : 'text-gray-400 group-hover:text-white'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </div>
        <span class="truncate">{{ item.label }}</span>
      </button>
    </nav>

    <!-- Collapse Button (Desktop only) -->
    <div class="hidden lg:block border-t border-white/5 p-3">
      <button
        @click="uiStore.toggleSidebar"
        class="flex w-full items-center justify-center gap-2 rounded-xl p-2.5 text-sm text-gray-500 transition-all duration-200 hover:bg-white/5 hover:text-white"
      >
        <svg
          class="h-4 w-4 transition-transform duration-300"
          :class="uiStore.sidebarCollapsed ? 'rotate-180' : ''"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 19l-7-7 7-7m8 14l-7-7 7-7" />
        </svg>
        <span v-if="!uiStore.sidebarCollapsed" class="text-xs">收起</span>
      </button>
    </div>
  </aside>
</template>
