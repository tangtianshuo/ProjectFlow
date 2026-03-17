<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useProjectStore } from "../../../stores/projectStore";
import { useUiStore } from "../../../stores/uiStore";
import Button from "../../ui/Button.vue";
import Modal from "../../ui/Modal.vue";
import Input from "../../ui/Input.vue";
import Select from "../../ui/Select.vue";
import Icon from "../../ui/Icon.vue";
import GanttChart from "./GanttChart.vue";
import type { Task } from "../../../lib/api";

const projectStore = useProjectStore();
const uiStore = useUiStore();

async function switchToGantt() {
  uiStore.setTaskViewMode('gantt');
  if (uiStore.selectedProjectId && projectStore.tasks.length === 0) {
    await projectStore.fetchTasks(uiStore.selectedProjectId);
  }
}

const columns = [
  { id: 0, name: "待办", color: "gray" },
  { id: 1, name: "进行中", color: "blue" },
  { id: 2, name: "审核中", color: "yellow" },
  { id: 3, name: "已完成", color: "green" },
];

const statusToProgress: Record<number, number> = {
  0: 0,
  1: 25,
  2: 75,
  3: 100,
};

const showCreateModal = ref(false);
const showEditModal = ref(false);
const newTask = ref({
  title: "",
  description: "",
  priority: 1,
  startDate: "",
  dueDate: "",
});

const editingTask = ref<Task | null>(null);

const tasksByStatus = computed(() => {
  const result: Record<number, Task[]> = {};
  columns.forEach((col) => {
    result[col.id] = projectStore.tasks.filter((t) => t.status === col.id);
  });
  return result;
});

watch(
  () => uiStore.selectedProjectId,
  async (projectId) => {
    if (projectId) {
      await projectStore.fetchTasks(projectId);
    }
  },
  { immediate: true }
);

async function createTask() {
  if (!newTask.value.title.trim() || !uiStore.selectedProjectId) return;

  try {
    await projectStore.createTask({
      projectId: uiStore.selectedProjectId,
      title: newTask.value.title,
      description: newTask.value.description || undefined,
      priority: Number(newTask.value.priority),
      startDate: newTask.value.startDate || undefined,
      dueDate: newTask.value.dueDate || undefined,
    });
    showCreateModal.value = false;
    newTask.value = { title: "", description: "", priority: 1, startDate: "", dueDate: "" };
  } catch (e) {
    console.error("Failed to create task:", e);
  }
}

async function deleteTask(id: string) {
  if (confirm("确定要删除这个任务吗？")) {
    try {
      await projectStore.deleteTask(id);
    } catch (e) {
      console.error("Failed to delete task:", e);
    }
  }
}

// 辅助函数：将 RFC3339 日期转换为 YYYY-MM-DD 格式
function formatDateForInput(dateStr: string | undefined | null): string {
  if (!dateStr) return "";
  // 如果已经包含 T，说明是 RFC3339 格式
  if (dateStr.includes("T")) {
    return dateStr.split("T")[0];
  }
  // 否则直接返回
  return dateStr;
}

function openEditModal(task: Task) {
  editingTask.value = {
    ...task,
    startDate: formatDateForInput(task.startDate),
    dueDate: formatDateForInput(task.dueDate),
    description: task.description || "",
  };
  showEditModal.value = true;
}

async function saveTask() {
  if (!editingTask.value) return;

  try {
    await projectStore.updateTask(editingTask.value.id, {
      title: editingTask.value.title,
      description: editingTask.value.description || undefined,
      priority: Number(editingTask.value.priority),
      startDate: editingTask.value.startDate || undefined,
      dueDate: editingTask.value.dueDate || undefined,
      progress: editingTask.value.progress,
    });
    showEditModal.value = false;
    editingTask.value = null;
  } catch (e) {
    console.error("Failed to update task:", e);
  }
}

async function quickStatusChange(task: Task) {
  const currentStatus = task.status;
  let nextStatus: number;

  if (currentStatus === 3) {
    nextStatus = 0;
  } else {
    nextStatus = currentStatus + 1;
  }

  try {
    await projectStore.updateTask(task.id, {
      status: nextStatus,
      progress: statusToProgress[nextStatus],
    });
  } catch (e) {
    console.error("Failed to update task status:", e);
  }
}

function getStatusButtonLabel(currentStatus: number) {
  const labels = ["开始", "进行", "审核", "完成"];
  return labels[currentStatus] || "开始";
}

function getStatusButtonColor(currentStatus: number) {
  const colors = [
    "bg-blue-500/20 text-blue-400 hover:bg-blue-500/30",
    "bg-amber-500/20 text-amber-400 hover:bg-amber-500/30",
    "bg-emerald-500/20 text-emerald-400 hover:bg-emerald-500/30",
    "bg-zinc-500/20 text-zinc-400 hover:bg-zinc-500/30",
  ];
  return colors[currentStatus] || colors[0];
}

function getPriorityLabel(priority: number) {
  const labels = ["最低", "低", "中", "高", "最高"];
  return labels[priority] || "中";
}

function getPriorityColor(priority: number) {
  const colors = [
    "bg-cyan-500/15 text-cyan-400",     // priority 0: lowest
    "bg-cyan-500/15 text-cyan-400",     // priority 1: low
    "bg-amber-500/15 text-amber-400",   // priority 2: medium
    "bg-red-500/15 text-red-400",       // priority 3: high
    "bg-red-500/15 text-red-400",       // priority 4: highest
  ];
  return colors[priority] || colors[2];
}

const getProgressStyle = (progress: number) => {
  return {
    width: `${progress}%`,
    background: progress >= 100
      ? 'var(--accent-green)'
      : `linear-gradient(90deg, var(--accent-primary) 0%, var(--accent-cyan) 100%)`
  };
};

function getColumnColor(color: string) {
  const colors: Record<string, string> = {
    gray: "bg-zinc-500",
    blue: "bg-blue-500",
    yellow: "bg-amber-500",
    green: "bg-emerald-500",
  };
  return colors[color] || colors.gray;
}
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 border-b border-[var(--border-subtle)] bg-[var(--bg-secondary)]/50 px-4 lg:px-6 py-3 lg:py-4">
      <div class="min-w-0">
        <h1 class="text-lg lg:text-xl font-bold text-[var(--text-primary)] truncate">
          {{ projectStore.currentProject?.name || "任务管理" }}
        </h1>
        <p v-if="projectStore.currentProject?.description" class="text-sm text-[var(--text-secondary)] truncate hidden sm:block">
          {{ projectStore.currentProject.description }}
        </p>
      </div>
      <div class="flex gap-2">
        <!-- View Toggle -->
        <div v-if="uiStore.selectedProjectId" class="flex rounded-lg bg-[var(--bg-tertiary)] p-0.5">
          <button
            @click="uiStore.setTaskViewMode('kanban')"
            class="rounded-md px-2 lg:px-3 py-1 text-xs transition-colors"
            :class="uiStore.taskViewMode === 'kanban' ? 'bg-indigo-500 text-white' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)]'"
          >
            <span class="hidden sm:inline">看板</span>
            <span class="sm:hidden">看</span>
          </button>
          <button
            @click="switchToGantt()"
            class="rounded-md px-2 lg:px-3 py-1 text-xs transition-colors"
            :class="uiStore.taskViewMode === 'gantt' ? 'bg-indigo-500 text-white' : 'text-[var(--text-secondary)] hover:text-[var(--text-primary)]'"
          >
            <span class="hidden sm:inline">甘特图</span>
            <span class="sm:hidden">图</span>
          </button>
        </div>
        <Button v-if="uiStore.selectedProjectId" @click="showCreateModal = true" variant="gradient">
          <Icon name="plus" :size="16" class="mr-1.5 lg:mr-2" />
          <span class="hidden sm:inline">新建任务</span>
          <span class="sm:hidden">新建</span>
        </Button>
      </div>
    </div>

    <!-- No Project Selected -->
    <div v-if="!uiStore.selectedProjectId" class="flex flex-1 items-center justify-center">
      <div class="text-center px-4">
        <div class="mx-auto mb-4 flex h-14 w-14 lg:h-16 lg:w-16 items-center justify-center rounded-2xl bg-[var(--bg-tertiary)]">
          <Icon name="check-square" :size="28" class="text-[var(--text-tertiary)]" />
        </div>
        <p class="text-[var(--text-secondary)]">请先选择一个项目</p>
      </div>
    </div>

    <!-- Gantt Chart View -->
    <div v-else-if="uiStore.taskViewMode === 'gantt'" class="flex-1 overflow-hidden">
      <GanttChart />
    </div>

    <!-- Kanban Board -->
    <div v-else-if="uiStore.taskViewMode === 'kanban'" class="flex-1 overflow-x-auto p-3 lg:p-4">
      <div class="flex gap-3 lg:gap-4 min-w-max">
        <div
          v-for="column in columns"
          :key="column.id"
          class="flex h-full w-64 lg:w-72 flex-col rounded-2xl"
        >
          <!-- Column Header -->
          <div class="flex items-center justify-between rounded-t-2xl border border-b-0 border-[var(--border-default)] bg-[var(--bg-card)] px-3 lg:px-4 py-2.5 lg:py-3">
            <div class="flex items-center gap-2">
              <div :class="['h-2 w-2 rounded-full', getColumnColor(column.color)]" />
              <span class="text-sm font-medium text-[var(--text-primary)]">{{ column.name }}</span>
              <span class="rounded-full bg-[var(--bg-tertiary)] px-1.5 lg:px-2 py-0.5 text-xs text-[var(--text-secondary)]">
                {{ tasksByStatus[column.id]?.length || 0 }}
              </span>
            </div>
          </div>

          <!-- Tasks -->
          <div class="flex-1 overflow-y-auto rounded-b-2xl border border-t-0 border-[var(--border-default)] bg-[var(--bg-tertiary)]/30 p-2">
            <div class="space-y-2">
              <div
                v-for="task in tasksByStatus[column.id]"
                :key="task.id"
                class="group rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-card)] p-3 transition-all duration-200"
                :class="task.status === 3 ? 'opacity-60' : 'cursor-pointer hover:border-indigo-500/30 hover:shadow-lg hover:shadow-indigo-500/10'"
                @click="task.status !== 3 && openEditModal(task)"
              >
                <div class="flex items-start justify-between gap-1">
                  <h4 class="text-sm font-medium line-clamp-2" :class="task.status === 3 ? 'text-[var(--text-tertiary)]' : 'text-[var(--text-primary)]'">{{ task.title }}</h4>
                  <button
                    v-if="task.status !== 3"
                    class="opacity-0 transition-opacity group-hover:opacity-100 flex-shrink-0"
                    @click.stop="deleteTask(task.id)"
                  >
                    <Icon name="x" :size="14" class="text-[var(--text-tertiary)] hover:text-red-400" />
                  </button>
                </div>
                <p v-if="task.description" class="mt-1.5 text-xs text-[var(--text-secondary)] line-clamp-2">
                  {{ task.description }}
                </p>
                <div class="mt-2.5 lg:mt-3 flex flex-wrap items-center gap-1.5 lg:gap-2">
                  <span
                    class="rounded-full px-2 py-0.5 text-xs font-medium"
                    :class="getPriorityColor(task.priority)"
                  >
                    {{ getPriorityLabel(task.priority) }}
                  </span>
                  <span v-if="task.dueDate" class="text-xs text-[var(--text-secondary)] truncate flex items-center gap-1">
                    <Icon name="calendar" :size="12" />
                    {{ task.dueDate.split("T")[0] }}
                  </span>
                </div>
                <!-- Progress bar -->
                <div class="mt-2.5 flex items-center gap-2">
                  <div class="flex-1 h-1.5 bg-[var(--bg-tertiary)] rounded-full overflow-hidden">
                    <div
                      class="h-full rounded-full transition-all duration-500 ease-out"
                      :style="getProgressStyle(task.progress)"
                    />
                  </div>
                  <span class="text-xs text-[var(--text-secondary)] min-w-[32px] text-right">{{ task.progress }}%</span>
                </div>
                <!-- Status button -->
                <button
                  class="mt-2.5 w-full rounded-lg py-1.5 text-xs font-medium transition-all duration-200"
                  :class="getStatusButtonColor(task.status)"
                  @click.stop="quickStatusChange(task)"
                >
                  {{ getStatusButtonLabel(task.status) }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Task Modal -->
    <Modal :open="showCreateModal" title="创建新任务" @close="showCreateModal = false">
      <form @submit.prevent="createTask" class="space-y-4">
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">任务标题</label>
          <Input v-model="newTask.title" placeholder="请输入任务标题" required />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">任务描述</label>
          <textarea
            v-model="newTask.description"
            placeholder="请输入任务描述（可选）"
            rows="3"
            class="w-full rounded-lg border border-[var(--border-default)] bg-[var(--bg-tertiary)] px-4 py-2.5 text-sm text-[var(--text-primary)] placeholder-[var(--text-tertiary)] transition-all duration-200 focus:border-indigo-500/50 focus:outline-none focus:ring-2 focus:ring-indigo-500/20"
          />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">优先级</label>
          <Select v-model="newTask.priority">
            <option value="0">最低</option>
            <option value="1">低</option>
            <option value="2">中</option>
            <option value="3">高</option>
            <option value="4">最高</option>
          </Select>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">开始日期</label>
            <Input v-model="newTask.startDate" type="date" />
          </div>
          <div>
            <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">截止日期</label>
            <Input v-model="newTask.dueDate" type="date" />
          </div>
        </div>
        <div class="flex justify-end gap-2 pt-4">
          <Button variant="secondary" type="button" @click="showCreateModal = false">取消</Button>
          <Button type="submit" variant="gradient">创建任务</Button>
        </div>
      </form>
    </Modal>

    <!-- Edit Task Modal -->
    <Modal :open="showEditModal" title="编辑任务" @close="showEditModal = false">
      <form v-if="editingTask" @submit.prevent="saveTask" class="space-y-4">
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">任务标题</label>
          <Input v-model="editingTask.title" required />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">任务描述</label>
          <textarea
            v-model="editingTask.description"
            rows="3"
            class="w-full rounded-lg border border-[var(--border-default)] bg-[var(--bg-tertiary)] px-4 py-2.5 text-sm text-[var(--text-primary)] placeholder-[var(--text-tertiary)] transition-all duration-200 focus:border-indigo-500/50 focus:outline-none focus:ring-2 focus:ring-indigo-500/20"
          />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">优先级</label>
          <Select v-model="editingTask.priority">
            <option value="0">最低</option>
            <option value="1">低</option>
            <option value="2">中</option>
            <option value="3">高</option>
            <option value="4">最高</option>
          </Select>
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">开始日期</label>
            <Input v-model="editingTask.startDate" type="date" />
          </div>
          <div>
            <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">截止日期</label>
            <Input v-model="editingTask.dueDate" type="date" />
          </div>
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-[var(--text-primary)]">
            进度: {{ editingTask.progress }}%
          </label>
          <input
            v-model.number="editingTask.progress"
            type="range"
            min="0"
            max="100"
            step="5"
            class="w-full h-2 bg-[var(--bg-tertiary)] rounded-lg appearance-none cursor-pointer accent-indigo-500"
          />
          <div class="flex justify-between text-xs text-[var(--text-secondary)] mt-1">
            <span>0%</span>
            <span>50%</span>
            <span>100%</span>
          </div>
        </div>
        <div class="flex justify-end gap-2 pt-4">
          <Button variant="secondary" type="button" @click="showEditModal = false">取消</Button>
          <Button type="submit" variant="gradient">保存</Button>
        </div>
      </form>
    </Modal>
  </div>
</template>
