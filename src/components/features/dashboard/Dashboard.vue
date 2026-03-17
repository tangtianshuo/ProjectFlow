<script setup lang="ts">
import { onMounted, computed, ref } from "vue";
import { useProjectStore } from "../../../stores/projectStore";
import { useUiStore } from "../../../stores/uiStore";
import { taskApi } from "../../../lib/api";
import Button from "../../ui/Button.vue";
import Icon from "../../ui/Icon.vue";

const projectStore = useProjectStore();
const uiStore = useUiStore();
const allTasks = ref<any[]>([]);

function createProject() {
  uiStore.setView("projects");
}

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
    <div class="mb-6 lg:mb-8 animate-fade-in">
      <h1 class="text-xl lg:text-2xl font-bold text-[var(--text-primary)]">仪表盘</h1>
      <p class="mt-1 text-sm text-[var(--text-secondary)]">欢迎回来，这是您的工作概览</p>
    </div>

    <!-- Stats Cards -->
    <div class="mb-6 lg:mb-8 grid grid-cols-2 lg:grid-cols-4 gap-3 lg:gap-4">
      <!-- Total Projects -->
      <div
        class="group relative overflow-hidden rounded-2xl border border-[var(--border-default)] bg-[var(--bg-card)] p-4 lg:p-5 transition-all duration-300 hover:border-indigo-500/30 hover:shadow-lg hover:shadow-indigo-500/10"
        style="animation: fadeIn 0.3s ease-out; animation-fill-mode: backwards;"
      >
        <div class="absolute inset-0 bg-gradient-to-br from-indigo-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-xs lg:text-sm text-[var(--text-secondary)]">总项目数</p>
            <p class="mt-1 text-2xl lg:text-3xl font-bold text-[var(--text-primary)]">{{ stats.totalProjects }}</p>
          </div>
          <div class="flex h-11 w-11 lg:h-12 lg:w-12 items-center justify-center rounded-xl bg-gradient-to-br from-indigo-500/20 to-violet-500/20">
            <Icon name="folder-kanban" :size="22" class="text-indigo-400" />
          </div>
        </div>
      </div>

      <!-- Active Projects -->
      <div
        class="group relative overflow-hidden rounded-2xl border border-[var(--border-default)] bg-[var(--bg-card)] p-4 lg:p-5 transition-all duration-300 hover:border-emerald-500/30 hover:shadow-lg hover:shadow-emerald-500/10"
        style="animation: fadeIn 0.3s ease-out 0.05s; animation-fill-mode: backwards;"
      >
        <div class="absolute inset-0 bg-gradient-to-br from-emerald-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-xs lg:text-sm text-[var(--text-secondary)]">进行中</p>
            <p class="mt-1 text-2xl lg:text-3xl font-bold text-[var(--text-primary)]">{{ stats.activeProjects }}</p>
          </div>
          <div class="flex h-11 w-11 lg:h-12 lg:w-12 items-center justify-center rounded-xl bg-gradient-to-br from-emerald-500/20 to-cyan-500/20">
            <Icon name="trending-up" :size="22" class="text-emerald-400" />
          </div>
        </div>
      </div>

      <!-- Total Tasks -->
      <div
        class="group relative overflow-hidden rounded-2xl border border-[var(--border-default)] bg-[var(--bg-card)] p-4 lg:p-5 transition-all duration-300 hover:border-blue-500/30 hover:shadow-lg hover:shadow-blue-500/10"
        style="animation: fadeIn 0.3s ease-out 0.1s; animation-fill-mode: backwards;"
      >
        <div class="absolute inset-0 bg-gradient-to-br from-blue-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-xs lg:text-sm text-[var(--text-secondary)]">总任务数</p>
            <p class="mt-1 text-2xl lg:text-3xl font-bold text-[var(--text-primary)]">{{ stats.totalTasks }}</p>
          </div>
          <div class="flex h-11 w-11 lg:h-12 lg:w-12 items-center justify-center rounded-xl bg-gradient-to-br from-blue-500/20 to-cyan-500/20">
            <Icon name="check-square" :size="22" class="text-blue-400" />
          </div>
        </div>
      </div>

      <!-- Completed Tasks -->
      <div
        class="group relative overflow-hidden rounded-2xl border border-[var(--border-default)] bg-[var(--bg-card)] p-4 lg:p-5 transition-all duration-300 hover:border-violet-500/30 hover:shadow-lg hover:shadow-violet-500/10"
        style="animation: fadeIn 0.3s ease-out 0.15s; animation-fill-mode: backwards;"
      >
        <div class="absolute inset-0 bg-gradient-to-br from-violet-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />
        <div class="relative flex items-center justify-between">
          <div>
            <p class="text-xs lg:text-sm text-[var(--text-secondary)]">已完成</p>
            <p class="mt-1 text-2xl lg:text-3xl font-bold text-[var(--text-primary)]">{{ stats.completedTasks }}</p>
          </div>
          <div class="flex h-11 w-11 lg:h-12 lg:w-12 items-center justify-center rounded-xl bg-gradient-to-br from-violet-500/20 to-purple-500/20">
            <Icon name="check-circle" :size="22" class="text-violet-400" />
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Projects -->
    <div
      class="rounded-2xl border border-[var(--border-default)] bg-[var(--bg-card)] p-4 lg:p-6"
      style="animation: fadeIn 0.3s ease-out 0.2s; animation-fill-mode: backwards;"
    >
      <div class="mb-4 flex items-center justify-between">
        <h2 class="text-base lg:text-lg font-semibold text-[var(--text-primary)]">最近项目</h2>
      </div>

      <div v-if="recentProjects.length === 0" class="py-8 lg:py-12 text-center">
        <div class="mx-auto mb-4 flex h-14 w-14 lg:h-16 lg:w-16 items-center justify-center rounded-2xl bg-[var(--bg-tertiary)]">
          <Icon name="folder-kanban" :size="28" class="text-[var(--text-tertiary)]" />
        </div>
        <p class="text-sm lg:text-base text-[var(--text-secondary)]">暂无项目</p>
        <Button class="mt-4" variant="gradient" @click="createProject">创建第一个项目</Button>
      </div>

      <div v-else class="space-y-2.5 lg:space-y-3">
        <div
          v-for="(project, index) in recentProjects"
          :key="project.id"
          class="group flex items-center justify-between rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-tertiary)]/50 p-3.5 lg:p-4 transition-all duration-200 hover:border-[var(--border-hover)] hover:bg-[var(--bg-tertiary)]"
          :style="{ animationDelay: `${0.25 + index * 0.05}s` }"
        >
          <div class="flex items-center gap-3.5 lg:gap-4">
            <div class="flex h-9 w-9 lg:h-10 lg:w-10 items-center justify-center rounded-xl bg-gradient-to-br from-indigo-500/20 to-violet-500/20">
              <Icon name="folder-kanban" :size="18" class="text-indigo-400" />
            </div>
            <div>
              <h3 class="text-sm lg:text-base font-medium text-[var(--text-primary)]">{{ project.name }}</h3>
              <p v-if="project.description" class="text-xs lg:text-sm text-[var(--text-secondary)] hidden sm:block">{{ project.description }}</p>
            </div>
          </div>
          <span
            class="rounded-full px-2.5 lg:px-3 py-1 text-xs font-medium"
            :class="{
              'bg-zinc-500/15 text-zinc-400': project.status === 0,
              'bg-emerald-500/15 text-emerald-400': project.status === 1,
              'bg-blue-500/15 text-blue-400': project.status === 2,
              'bg-zinc-500/15 text-zinc-500': project.status === 3,
            }"
          >
            {{ ["未开始", "进行中", "已完成", "已归档"][project.status] }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
