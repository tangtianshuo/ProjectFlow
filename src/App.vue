<script setup lang="ts">
import { onMounted } from "vue";
import { useProjectStore } from "./stores/projectStore";
import { useUiStore } from "./stores/uiStore";
import Sidebar from "./components/layout/Sidebar.vue";
import Dashboard from "./components/features/dashboard/Dashboard.vue";
import ProjectList from "./components/features/projects/ProjectList.vue";
import TaskBoard from "./components/features/tasks/TaskBoard.vue";
import DocumentCenter from "./components/features/documents/DocumentCenter.vue";
import RecycleBin from "./components/features/recycleBin/RecycleBin.vue";

const projectStore = useProjectStore();
const uiStore = useUiStore();

onMounted(() => {
  projectStore.fetchProjects();
});
</script>

<template>
  <div class="flex min-h-screen bg-[#0a0a0f]">
    <!-- Sidebar -->
    <Sidebar />

    <!-- Main Content -->
    <main class="flex-1 overflow-hidden pt-16 lg:pt-0">
      <div class="h-[calc(100vh-4rem)] lg:h-screen overflow-y-auto">
        <Dashboard v-if="uiStore.currentView === 'dashboard'" />
        <ProjectList v-else-if="uiStore.currentView === 'projects'" />
        <TaskBoard v-else-if="uiStore.currentView === 'tasks'" />
        <DocumentCenter v-else-if="uiStore.currentView === 'documents'" />
        <RecycleBin v-else-if="uiStore.currentView === 'recycleBin'" />
        <div v-else-if="uiStore.currentView === 'settings'" class="p-4 lg:p-6">
          <h1 class="text-xl lg:text-2xl font-bold text-white">设置</h1>
          <p class="mt-4 text-gray-500">设置功能开发中...</p>
        </div>
      </div>
    </main>
  </div>
</template>
