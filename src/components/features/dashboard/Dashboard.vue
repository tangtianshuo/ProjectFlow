<script setup lang="ts">
import { onMounted, computed, ref } from "vue";
import { useProjectStore } from "../../../stores/projectStore";
import { taskApi } from "../../../lib/api";
import Button from "../../ui/Button.vue";

const projectStore = useProjectStore();
const allTasks = ref<any[]>([]);

const stats = computed(() => {
  const totalProjects = projectStore.projects.length;
  const activeProjects = projectStore.activeProjects.length;
  const completedProjects = projectStore.completedProjects.length;
  const totalTasks = allTasks.value.length;
  const completedTasks = allTasks.value.filter((t) => t.status === 3).length;

  return {
    totalProjects,
    activeProjects,
    completedProjects,
    totalTasks,
    completedTasks,
  };
});

const recentProjects = computed(() => {
  return projectStore.projects.slice(0, 5);
});

onMounted(async () => {
  await projectStore.fetchProjects();
  // Fetch all tasks from all projects
  for (const project of projectStore.projects) {
    try {
      const tasks = await taskApi.getByProject(project.id);
      allTasks.value.push(...tasks);
    } catch (e) {
      console.error(`Failed to fetch tasks for project ${project.id}:`, e);
    }
  }
});
</script>

<template>
  <div class="p-4 lg:p-6">
    <!-- Header -->
    <div class="mb-6 lg:mb-8">
      <h1 class="text-xl lg:text-2xl font-bold text-white">仪表盘</h1>
      <p class="mt-1 text-sm text-gray-400">欢迎回来，这是您的工作概览</p>
    </div>

    <!-- Stats Cards -->
    <div class="mb-6 lg:mb-8 grid grid-cols-2 lg:grid-cols-4 gap-3 lg:gap-4">
      <!-- Total Projects -->
      <div class="group relative overflow-hidden rounded-xl lg:rounded-2xl border border-white/10 bg-[#12121a]/80 p-4 lg:p-5 transition-all duration-300 hover:border-indigo-500/30 hover:shadow-lg hover:shadow-indigo-500/10">
        <div class="absolute inset-0 bg-gradient-to-br from-indigo-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-xs lg:text-sm text-gray-400">总项目数</p>
            <p class="mt-1 text-2xl lg:text-3xl font-bold text-white">{{ stats.totalProjects }}</p>
          </div>
          <div class="flex h-10 w-10 lg:h-12 lg:w-12 items-center justify-center rounded-lg lg:rounded-xl bg-gradient-to-br from-indigo-500/20 to-violet-500/20">
            <svg class="h-5 w-5 lg:h-6 lg:w-6 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Active Projects -->
      <div class="group relative overflow-hidden rounded-xl lg:rounded-2xl border border-white/10 bg-[#12121a]/80 p-4 lg:p-5 transition-all duration-300 hover:border-green-500/30 hover:shadow-lg hover:shadow-green-500/10">
        <div class="absolute inset-0 bg-gradient-to-br from-green-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-xs lg:text-sm text-gray-400">进行中</p>
            <p class="mt-1 text-2xl lg:text-3xl font-bold text-white">{{ stats.activeProjects }}</p>
          </div>
          <div class="flex h-10 w-10 lg:h-12 lg:w-12 items-center justify-center rounded-lg lg:rounded-xl bg-gradient-to-br from-green-500/20 to-emerald-500/20">
            <svg class="h-5 w-5 lg:h-6 lg:w-6 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Total Tasks -->
      <div class="group relative overflow-hidden rounded-xl lg:rounded-2xl border border-white/10 bg-[#12121a]/80 p-4 lg:p-5 transition-all duration-300 hover:border-blue-500/30 hover:shadow-lg hover:shadow-blue-500/10">
        <div class="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-xs lg:text-sm text-gray-400">总任务数</p>
            <p class="mt-1 text-2xl lg:text-3xl font-bold text-white">{{ stats.totalTasks }}</p>
          </div>
          <div class="flex h-10 w-10 lg:h-12 lg:w-12 items-center justify-center rounded-lg lg:rounded-xl bg-gradient-to-br from-blue-500/20 to-cyan-500/20">
            <svg class="h-5 w-5 lg:h-6 lg:w-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
            </svg>
          </div>
        </div>
      </div>

      <!-- Completed Tasks -->
      <div class="group relative overflow-hidden rounded-xl lg:rounded-2xl border border-white/10 bg-[#12121a]/80 p-4 lg:p-5 transition-all duration-300 hover:border-violet-500/30 hover:shadow-lg hover:shadow-violet-500/10">
        <div class="absolute inset-0 bg-gradient-to-br from-violet-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-xs lg:text-sm text-gray-400">已完成</p>
            <p class="mt-1 text-2xl lg:text-3xl font-bold text-white">{{ stats.completedTasks }}</p>
          </div>
          <div class="flex h-10 w-10 lg:h-12 lg:w-12 items-center justify-center rounded-lg lg:rounded-xl bg-gradient-to-br from-violet-500/20 to-purple-500/20">
            <svg class="h-5 w-5 lg:h-6 lg:w-6 text-violet-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Projects -->
    <div class="rounded-xl lg:rounded-2xl border border-white/10 bg-[#12121a]/80 p-4 lg:p-6">
      <div class="mb-4 flex items-center justify-between">
        <h2 class="text-base lg:text-lg font-semibold text-white">最近项目</h2>
      </div>

      <div v-if="recentProjects.length === 0" class="py-8 lg:py-12 text-center">
        <div class="mx-auto mb-4 flex h-12 w-12 lg:h-16 lg:w-16 items-center justify-center rounded-xl lg:rounded-2xl bg-white/5">
          <svg class="h-6 w-6 lg:h-8 lg:w-8 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
        </div>
        <p class="text-sm lg:text-base text-gray-500">暂无项目</p>
        <Button class="mt-4" @click="$emit('createProject')">创建第一个项目</Button>
      </div>

      <div v-else class="space-y-2 lg:space-y-3">
        <div
          v-for="project in recentProjects"
          :key="project.id"
          class="group flex items-center justify-between rounded-lg lg:rounded-xl border border-white/5 bg-white/5 p-3 lg:p-4 transition-all duration-200 hover:border-white/10 hover:bg-white/10"
        >
          <div class="flex items-center gap-3 lg:gap-4">
            <div class="flex h-8 w-8 lg:h-10 lg:w-10 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500/20 to-violet-500/20">
              <svg class="h-4 w-4 lg:h-5 lg:w-5 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
              </svg>
            </div>
            <div>
              <h3 class="text-sm lg:text-base font-medium text-white">{{ project.name }}</h3>
              <p v-if="project.description" class="text-xs lg:text-sm text-gray-500 hidden sm:block">{{ project.description }}</p>
            </div>
          </div>
          <span
            class="rounded-full px-2 lg:px-3 py-0.5 text-xs font-medium"
            :class="{
              'bg-gray-500/20 text-gray-400': project.status === 0,
              'bg-green-500/20 text-green-400': project.status === 1,
              'bg-blue-500/20 text-blue-400': project.status === 2,
              'bg-gray-500/20 text-gray-500': project.status === 3,
            }"
          >
            {{ ["未开始", "进行中", "已完成", "已归档"][project.status] }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
