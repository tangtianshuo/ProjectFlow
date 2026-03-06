<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useProjectStore } from "../../../stores/projectStore";
import { useUiStore } from "../../../stores/uiStore";
import { taskApi, type Task } from "../../../lib/api";
import Button from "../../ui/Button.vue";
import Modal from "../../ui/Modal.vue";
import Input from "../../ui/Input.vue";

const projectStore = useProjectStore();
const uiStore = useUiStore();

const showCreateModal = ref(false);
const newProject = ref({
  name: "",
  description: "",
  startDate: "",
  endDate: "",
});

// Store tasks for all projects
const allProjectTasks = ref<Record<string, Task[]>>({});

// Compute progress for each project based on tasks
const projectProgressMap = computed(() => {
  const progressMap: Record<string, number> = {};
  for (const project of projectStore.projects) {
    const tasks = allProjectTasks.value[project.id] || [];
    if (tasks.length === 0) {
      progressMap[project.id] = 0;
    } else {
      const completedCount = tasks.filter((t) => t.status === 3).length;
      progressMap[project.id] = Math.round((completedCount / tasks.length) * 100);
    }
  }
  return progressMap;
});

onMounted(async () => {
  await projectStore.fetchProjects();
  // Fetch tasks for all projects to calculate progress
  for (const project of projectStore.projects) {
    try {
      const tasks = await taskApi.getByProject(project.id);
      allProjectTasks.value[project.id] = tasks;
    } catch (e) {
      console.error(`Failed to fetch tasks for project ${project.id}:`, e);
    }
  }
});

async function createProject() {
  if (!newProject.value.name.trim()) return;

  try {
    const project = await projectStore.createProject({
      name: newProject.value.name,
      description: newProject.value.description || undefined,
      startDate: newProject.value.startDate || undefined,
      endDate: newProject.value.endDate || undefined,
    });
    // Initialize empty tasks array for new project
    allProjectTasks.value[project.id] = [];
    showCreateModal.value = false;
    newProject.value = { name: "", description: "", startDate: "", endDate: "" };
  } catch (e) {
    console.error("Failed to create project:", e);
  }
}

async function deleteProject(id: string) {
  if (confirm("确定要删除这个项目吗？")) {
    try {
      await projectStore.deleteProject(id);
      delete allProjectTasks.value[id];
    } catch (e) {
      console.error("Failed to delete project:", e);
    }
  }
}

function selectProject(projectId: string) {
  uiStore.selectProject(projectId);
  uiStore.setView("tasks");
}

// Start or complete project
async function toggleProjectStatus(project: { id: string; status: number }) {
  try {
    if (project.status === 0) {
      // Start project: 0 -> 1
      await projectStore.updateProject(project.id, { status: 1 });
    } else if (project.status === 1) {
      // Complete project: 1 -> 2
      await projectStore.updateProject(project.id, { status: 2 });
    } else if (project.status === 2) {
      // Reopen project: 2 -> 1
      await projectStore.updateProject(project.id, { status: 1 });
    }
  } catch (e) {
    console.error("Failed to update project status:", e);
  }
}

function getProgressColor(progress: number) {
  if (progress >= 100) return "bg-green-500";
  if (progress >= 75) return "bg-yellow-500";
  if (progress >= 25) return "bg-blue-500";
  return "bg-gray-500";
}

function getProjectActionLabel(status: number) {
  if (status === 0) return "开始项目";
  if (status === 1) return "完成项目";
  if (status === 2) return "重新开启";
  return "";
}
</script>

<template>
  <div class="p-4 lg:p-6">
    <!-- Header -->
    <div class="mb-4 lg:mb-6 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
      <div>
        <h1 class="text-xl lg:text-2xl font-bold text-white">项目管理</h1>
        <p class="mt-1 text-sm text-gray-400">创建和管理您的项目</p>
      </div>
      <Button @click="showCreateModal = true">
        <svg class="mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        <span class="hidden sm:inline">新建项目</span>
        <span class="sm:hidden">新建</span>
      </Button>
    </div>

    <!-- Projects Grid -->
    <div v-if="projectStore.loading" class="py-12 text-center text-gray-500">
      加载中...
    </div>

    <div v-else-if="projectStore.projects.length === 0" class="rounded-xl lg:rounded-2xl border border-white/10 bg-[#12121a]/80 py-12 lg:py-16 text-center px-4">
      <div class="mx-auto mb-4 flex h-16 w-16 lg:h-20 lg:w-20 items-center justify-center rounded-2xl bg-gradient-to-br from-indigo-500/10 to-violet-500/10">
        <svg class="h-8 w-8 lg:h-10 lg:w-10 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
        </svg>
      </div>
      <p class="text-gray-500">暂无项目</p>
      <Button class="mt-4" @click="showCreateModal = true">创建第一个项目</Button>
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-3 lg:gap-4">
      <div
        v-for="project in projectStore.projects"
        :key="project.id"
        class="group relative overflow-hidden rounded-xl lg:rounded-2xl border border-white/10 bg-[#12121a]/80 p-4 lg:p-5 transition-all duration-300 hover:border-indigo-500/30 hover:shadow-lg hover:shadow-indigo-500/10"
      >
        <!-- Gradient background on hover -->
        <div class="absolute inset-0 bg-gradient-to-br from-indigo-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />

        <div class="relative">
          <div class="flex items-start justify-between gap-2">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 lg:gap-3">
                <div class="flex h-8 w-8 lg:h-10 lg:w-10 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500/20 to-violet-500/20 flex-shrink-0">
                  <svg class="h-4 w-4 lg:h-5 lg:w-5 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                  </svg>
                </div>
                <h3 class="font-semibold text-white truncate">{{ project.name }}</h3>
              </div>
              <p v-if="project.description" class="mt-2 text-sm text-gray-400 line-clamp-2">
                {{ project.description }}
              </p>
            </div>
            <span
              class="rounded-full px-2 py-1 text-xs font-medium flex-shrink-0"
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

          <div v-if="project.startDate || project.endDate" class="mt-3 lg:mt-4 flex items-center gap-2 text-xs text-gray-500">
            <svg class="h-4 w-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
            </svg>
            <span class="truncate">{{ project.startDate?.split("T")[0] }} - {{ project.endDate?.split("T")[0] }}</span>
          </div>

          <!-- Progress bar -->
          <div class="mt-3 lg:mt-4 flex items-center gap-2">
            <div class="flex-1 h-1.5 bg-white/10 rounded-full overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300"
                :class="getProgressColor(projectProgressMap[project.id] || 0)"
                :style="{ width: `${projectProgressMap[project.id] || 0}%` }"
              />
            </div>
            <span class="text-xs text-gray-500 min-w-[32px] text-right">{{ projectProgressMap[project.id] || 0 }}%</span>
          </div>

          <div class="mt-3 lg:mt-4 flex gap-2">
            <Button
              v-if="project.status !== 3"
              size="sm"
              :variant="project.status === 0 ? 'primary' : project.status === 2 ? 'primary' : 'secondary'"
              class="flex-1 text-xs lg:text-sm"
              @click="toggleProjectStatus(project)"
            >
              <svg v-if="project.status === 0" class="mr-1 h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <svg v-else-if="project.status === 2" class="mr-1 h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              <svg v-else class="mr-1 h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              {{ getProjectActionLabel(project.status) }}
            </Button>
            <Button size="sm" class="flex-1 text-xs lg:text-sm" @click="selectProject(project.id)">
              <svg class="mr-1 h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
              <span class="hidden sm:inline">查看任务</span>
              <span class="sm:hidden">查看</span>
            </Button>
            <Button size="sm" variant="danger" @click="deleteProject(project.id)">
              <svg class="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
              </svg>
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Project Modal -->
    <Modal :open="showCreateModal" title="创建新项目" @close="showCreateModal = false">
      <form @submit.prevent="createProject" class="space-y-4">
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">项目名称</label>
          <Input
            v-model="newProject.name"
            placeholder="请输入项目名称"
            required
          />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">项目描述</label>
          <textarea
            v-model="newProject.description"
            placeholder="请输入项目描述（可选）"
            rows="3"
            class="w-full rounded-lg border border-white/10 bg-[#1a1a25] px-4 py-2.5 text-sm text-white placeholder-gray-500 transition-all duration-200 focus:border-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/20"
          />
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="mb-1.5 block text-sm font-medium text-gray-300">开始日期</label>
            <Input v-model="newProject.startDate" type="date" />
          </div>
          <div>
            <label class="mb-1.5 block text-sm font-medium text-gray-300">结束日期</label>
            <Input v-model="newProject.endDate" type="date" />
          </div>
        </div>
        <div class="flex justify-end gap-2 pt-4">
          <Button variant="secondary" type="button" @click="showCreateModal = false">取消</Button>
          <Button type="submit">创建项目</Button>
        </div>
      </form>
    </Modal>
  </div>
</template>
