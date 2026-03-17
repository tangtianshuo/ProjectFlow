<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { useProjectStore } from "../../../stores/projectStore";
import { useUiStore } from "../../../stores/uiStore";
import { taskApi, type Task } from "../../../lib/api";
import Button from "../../ui/Button.vue";
import Modal from "../../ui/Modal.vue";
import Input from "../../ui/Input.vue";
import Icon from "../../ui/Icon.vue";

const projectStore = useProjectStore();
const uiStore = useUiStore();

const showCreateModal = ref(false);
const newProject = ref({
  name: "",
  description: "",
  startDate: "",
  endDate: "",
});

const allProjectTasks = ref<Record<string, Task[]>>({});

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

async function toggleProjectStatus(project: { id: string; status: number }) {
  try {
    if (project.status === 0) {
      await projectStore.updateProject(project.id, { status: 1 });
    } else if (project.status === 1) {
      await projectStore.updateProject(project.id, { status: 2 });
    } else if (project.status === 2) {
      await projectStore.updateProject(project.id, { status: 1 });
    }
  } catch (e) {
    console.error("Failed to update project status:", e);
  }
}

function getProgressColor(progress: number) {
  if (progress >= 100) return "bg-emerald-500";
  if (progress >= 75) return "bg-amber-500";
  if (progress >= 25) return "bg-blue-500";
  return "bg-zinc-600";
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
        <h1 class="text-xl lg:text-2xl font-bold text-[var(--text-primary)]">项目管理</h1>
        <p class="mt-1 text-sm text-[var(--text-secondary)]">创建和管理您的项目</p>
      </div>
      <Button @click="showCreateModal = true" variant="gradient">
        <Icon name="plus" :size="16" class="mr-2" />
        <span class="hidden sm:inline">新建项目</span>
        <span class="sm:hidden">新建</span>
      </Button>
    </div>

    <!-- Projects Grid -->
    <div v-if="projectStore.loading" class="py-12 text-center text-[var(--text-secondary)]">
      加载中...
    </div>

    <div v-else-if="projectStore.projects.length === 0" class="rounded-2xl border border-[var(--border-default)] bg-[var(--bg-card)] py-12 lg:py-16 text-center px-4">
      <div class="mx-auto mb-4 flex h-16 w-16 lg:h-20 lg:w-20 items-center justify-center rounded-2xl bg-gradient-to-br from-indigo-500/10 to-violet-500/10">
        <Icon name="folder-kanban" :size="32" class="text-[var(--text-tertiary)]" />
      </div>
      <p class="text-[var(--text-secondary)]">暂无项目</p>
      <Button class="mt-4" variant="gradient" @click="showCreateModal = true">创建第一个项目</Button>
    </div>

    <div v-else class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-3 lg:gap-4">
      <div
        v-for="(project, index) in projectStore.projects"
        :key="project.id"
        class="group relative overflow-hidden rounded-2xl border border-[var(--border-default)] bg-[var(--bg-card)] p-4 lg:p-5 transition-all duration-300 hover:border-indigo-500/30 hover:shadow-lg hover:shadow-indigo-500/10"
        :style="{ animation: 'fadeIn 0.3s ease-out', animationDelay: `${index * 0.05}s`, animationFillMode: 'backwards' }"
      >
        <!-- Gradient background on hover -->
        <div class="absolute inset-0 bg-gradient-to-br from-indigo-500/5 to-transparent opacity-0 transition-opacity duration-300 group-hover:opacity-100" />

        <div class="relative">
          <div class="flex items-start justify-between gap-2">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2.5 lg:gap-3">
                <div class="flex h-9 w-9 lg:h-10 lg:w-10 items-center justify-center rounded-xl bg-gradient-to-br from-indigo-500/20 to-violet-500/20 flex-shrink-0">
                  <Icon name="folder-kanban" :size="18" class="text-indigo-400" />
                </div>
                <h3 class="font-semibold text-[var(--text-primary)] truncate">{{ project.name }}</h3>
              </div>
              <p v-if="project.description" class="mt-2 text-sm text-[var(--text-secondary)] line-clamp-2">
                {{ project.description }}
              </p>
            </div>
            <span
              class="rounded-full px-2.5 py-1 text-xs font-medium flex-shrink-0"
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

          <div v-if="project.startDate || project.endDate" class="mt-3 lg:mt-4 flex items-center gap-2 text-xs text-[var(--text-secondary)]">
            <Icon name="calendar" :size="14" class="flex-shrink-0" />
            <span class="truncate">{{ project.startDate?.split("T")[0] }} - {{ project.endDate?.split("T")[0] }}</span>
          </div>

          <!-- Progress bar -->
          <div class="mt-3 lg:mt-4 flex items-center gap-2">
            <div class="flex-1 h-1.5 bg-[var(--bg-tertiary)] rounded-full overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-500"
                :class="getProgressColor(projectProgressMap[project.id] || 0)"
                :style="{ width: `${projectProgressMap[project.id] || 0}%` }"
              />
            </div>
            <span class="text-xs text-[var(--text-secondary)] min-w-[32px] text-right">{{ projectProgressMap[project.id] || 0 }}%</span>
          </div>

          <div class="mt-3 lg:mt-4 flex gap-2">
            <Button
              v-if="project.status !== 3"
              size="sm"
              :variant="project.status === 0 ? 'primary' : project.status === 2 ? 'primary' : 'secondary'"
              class="flex-1 text-xs lg:text-sm"
              @click="toggleProjectStatus(project)"
            >
              <Icon v-if="project.status === 0" name="arrow-right" :size="14" class="mr-1.5" />
              <Icon v-else-if="project.status === 2" name="refresh-cw" :size="14" class="mr-1.5" />
              <Icon v-else name="check-circle" :size="14" class="mr-1.5" />
              {{ getProjectActionLabel(project.status) }}
            </Button>
            <Button size="sm" class="flex-1 text-xs lg:text-sm" @click="selectProject(project.id)">
              <Icon name="eye" :size="14" class="mr-1.5" />
              <span class="hidden sm:inline">查看任务</span>
              <span class="sm:hidden">查看</span>
            </Button>
            <Button size="sm" variant="danger" @click="deleteProject(project.id)">
              <Icon name="trash-2" :size="16" />
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Project Modal -->
    <Modal :open="showCreateModal" title="创建新项目" @close="showCreateModal = false">
      <form @submit.prevent="createProject" class="space-y-4">
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">项目名称</label>
          <Input
            v-model="newProject.name"
            placeholder="请输入项目名称"
            required
          />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">项目描述</label>
          <textarea
            v-model="newProject.description"
            placeholder="请输入项目描述（可选）"
            rows="3"
            class="w-full rounded-lg border border-[var(--border-default)] bg-[var(--bg-tertiary)] px-4 py-2.5 text-sm text-[var(--text-primary)] placeholder-[var(--text-tertiary)] transition-all duration-200 focus:border-indigo-500/50 focus:outline-none focus:ring-2 focus:ring-indigo-500/20"
          />
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">开始日期</label>
            <Input v-model="newProject.startDate" type="date" />
          </div>
          <div>
            <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">结束日期</label>
            <Input v-model="newProject.endDate" type="date" />
          </div>
        </div>
        <div class="flex justify-end gap-2 pt-4">
          <Button variant="secondary" type="button" @click="showCreateModal = false">取消</Button>
          <Button type="submit" variant="gradient">创建项目</Button>
        </div>
      </form>
    </Modal>
  </div>
</template>
