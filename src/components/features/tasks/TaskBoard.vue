<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useProjectStore } from "../../../stores/projectStore";
import { useUiStore } from "../../../stores/uiStore";
import Button from "../../ui/Button.vue";
import Modal from "../../ui/Modal.vue";
import Input from "../../ui/Input.vue";
import Select from "../../ui/Select.vue";
import GanttChart from "./GanttChart.vue";
import type { Task } from "../../../lib/api";

const projectStore = useProjectStore();
const uiStore = useUiStore();

// Refresh data when switching to gantt view
async function switchToGantt() {
  uiStore.setTaskViewMode('gantt');
  // Ensure tasks are loaded
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

// Map status to default progress
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
      priority: newTask.value.priority,
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

function openEditModal(task: Task) {
  editingTask.value = {
    ...task,
    startDate: task.startDate || "",
    dueDate: task.dueDate || "",
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
      priority: editingTask.value.priority,
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

// Quick status switch
async function quickStatusChange(task: Task) {
  const currentStatus = task.status;
  let nextStatus: number;

  if (currentStatus === 3) {
    // If completed, go back to todo
    nextStatus = 0;
  } else {
    // Move to next status
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
    "bg-yellow-500/20 text-yellow-400 hover:bg-yellow-500/30",
    "bg-green-500/20 text-green-400 hover:bg-green-500/30",
    "bg-gray-500/20 text-gray-400 hover:bg-gray-500/30",
  ];
  return colors[currentStatus] || colors[0];
}

function getPriorityLabel(priority: number) {
  const labels = ["最低", "低", "中", "高", "最高"];
  return labels[priority] || "中";
}

function getPriorityColor(priority: number) {
  const colors = ["bg-gray-500/20 text-gray-400", "bg-gray-500/20 text-gray-400", "bg-blue-500/20 text-blue-400", "bg-orange-500/20 text-orange-400", "bg-red-500/20 text-red-400"];
  return colors[priority] || colors[2];
}

function getProgressColor(progress: number) {
  if (progress >= 100) return "bg-green-500";
  if (progress >= 75) return "bg-yellow-500";
  if (progress >= 25) return "bg-blue-500";
  return "bg-gray-500";
}
</script>

<template>
  <div class="flex h-full flex-col">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 border-b border-white/5 bg-[#0d0d12]/50 px-4 lg:px-6 py-3 lg:py-4">
      <div class="min-w-0">
        <h1 class="text-lg lg:text-xl font-bold text-white truncate">
          {{ projectStore.currentProject?.name || "任务管理" }}
        </h1>
        <p v-if="projectStore.currentProject?.description" class="text-sm text-gray-400 truncate hidden sm:block">
          {{ projectStore.currentProject.description }}
        </p>
      </div>
      <div class="flex gap-2">
        <!-- View Toggle -->
        <div v-if="uiStore.selectedProjectId" class="flex rounded-lg bg-white/5 p-0.5">
          <button
            @click="uiStore.setTaskViewMode('kanban')"
            class="rounded-md px-2 lg:px-3 py-1 text-xs transition-colors"
            :class="uiStore.taskViewMode === 'kanban' ? 'bg-indigo-500 text-white' : 'text-gray-400 hover:text-white'"
          >
            <span class="hidden sm:inline">看板</span>
            <span class="sm:hidden">看</span>
          </button>
          <button
            @click="switchToGantt()"
            class="rounded-md px-2 lg:px-3 py-1 text-xs transition-colors"
            :class="uiStore.taskViewMode === 'gantt' ? 'bg-indigo-500 text-white' : 'text-gray-400 hover:text-white'"
          >
            <span class="hidden sm:inline">甘特图</span>
            <span class="sm:hidden">图</span>
          </button>
        </div>
        <Button v-if="uiStore.selectedProjectId" @click="showCreateModal = true">
          <svg class="mr-1 lg:mr-2 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span class="hidden sm:inline">新建任务</span>
          <span class="sm:hidden">新建</span>
        </Button>
      </div>
    </div>

    <!-- No Project Selected -->
    <div v-if="!uiStore.selectedProjectId" class="flex flex-1 items-center justify-center">
      <div class="text-center px-4">
        <div class="mx-auto mb-4 flex h-14 w-14 lg:h-16 lg:w-16 items-center justify-center rounded-2xl bg-white/5">
          <svg class="h-7 w-7 lg:h-8 lg:w-8 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
          </svg>
        </div>
        <p class="text-gray-500">请先选择一个项目</p>
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
          class="flex h-full w-64 lg:w-72 flex-col rounded-xl lg:rounded-2xl"
        >
          <!-- Column Header -->
          <div class="flex items-center justify-between rounded-t-xl lg:rounded-t-2xl border border-b-0 border-white/10 bg-[#12121a]/80 px-3 lg:px-4 py-2.5 lg:py-3">
            <div class="flex items-center gap-2">
              <div :class="`h-2 w-2 rounded-full bg-${column.color}-500`" />
              <span class="text-sm font-medium text-white">{{ column.name }}</span>
              <span class="rounded-full bg-white/10 px-1.5 lg:px-2 py-0.5 text-xs text-gray-400">
                {{ tasksByStatus[column.id]?.length || 0 }}
              </span>
            </div>
          </div>

          <!-- Tasks -->
          <div class="flex-1 overflow-y-auto rounded-b-xl lg:rounded-b-2xl border border-t-0 border-white/10 bg-[#12121a]/40 p-2">
            <div class="space-y-2">
              <div
                v-for="task in tasksByStatus[column.id]"
                :key="task.id"
                class="group rounded-lg lg:rounded-xl border border-white/5 bg-[#1a1a25]/80 p-2.5 lg:p-3 transition-all duration-200"
                :class="task.status === 3 ? 'opacity-75' : 'cursor-pointer hover:border-indigo-500/30 hover:shadow-lg hover:shadow-indigo-500/10'"
                @click="task.status !== 3 && openEditModal(task)"
              >
                <div class="flex items-start justify-between gap-1">
                  <h4 class="text-sm font-medium line-clamp-2" :class="task.status === 3 ? 'text-gray-400' : 'text-white'">{{ task.title }}</h4>
                  <button
                    v-if="task.status !== 3"
                    class="opacity-0 transition-opacity group-hover:opacity-100 flex-shrink-0"
                    @click.stop="deleteTask(task.id)"
                  >
                    <svg class="h-4 w-4 text-gray-500 hover:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
                <p v-if="task.description" class="mt-1.5 text-xs text-gray-400 line-clamp-2">
                  {{ task.description }}
                </p>
                <div class="mt-2 lg:mt-3 flex flex-wrap items-center gap-1.5 lg:gap-2">
                  <span
                    class="rounded-full px-1.5 lg:px-2 py-0.5 text-xs font-medium"
                    :class="getPriorityColor(task.priority)"
                  >
                    {{ getPriorityLabel(task.priority) }}
                  </span>
                  <span v-if="task.dueDate" class="text-xs text-gray-500 truncate">
                    {{ task.dueDate.split("T")[0] }}
                  </span>
                </div>
                <!-- Progress bar -->
                <div class="mt-2 flex items-center gap-2">
                  <div class="flex-1 h-1.5 bg-white/10 rounded-full overflow-hidden">
                    <div
                      class="h-full rounded-full transition-all duration-300"
                      :class="getProgressColor(task.progress)"
                      :style="{ width: `${task.progress}%` }"
                    />
                  </div>
                  <span class="text-xs text-gray-500 min-w-[32px] text-right">{{ task.progress }}%</span>
                </div>
                <!-- Status button -->
                <button
                  class="mt-2 w-full rounded-lg py-1.5 text-xs font-medium transition-all duration-200"
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

    <!-- Gantt Chart / Kanban End -->

    <!-- Create Task Modal -->
    <Modal :open="showCreateModal" title="创建新任务" @close="showCreateModal = false">
      <form @submit.prevent="createTask" class="space-y-4">
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">任务标题</label>
          <Input v-model="newTask.title" placeholder="请输入任务标题" required />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">任务描述</label>
          <textarea
            v-model="newTask.description"
            placeholder="请输入任务描述（可选）"
            rows="3"
            class="w-full rounded-lg border border-white/10 bg-[#1a1a25] px-4 py-2.5 text-sm text-white placeholder-gray-500 transition-all duration-200 focus:border-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/20"
          />
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="mb-1.5 block text-sm font-medium text-gray-300">优先级</label>
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
              <label class="mb-1.5 block text-sm font-medium text-gray-300">开始日期</label>
              <Input v-model="newTask.startDate" type="date" />
            </div>
            <div>
              <label class="mb-1.5 block text-sm font-medium text-gray-300">截止日期</label>
              <Input v-model="newTask.dueDate" type="date" />
            </div>
          </div>
        </div>
        <div class="flex justify-end gap-2 pt-4">
          <Button variant="secondary" type="button" @click="showCreateModal = false">取消</Button>
          <Button type="submit">创建任务</Button>
        </div>
      </form>
    </Modal>

    <!-- Edit Task Modal -->
    <Modal :open="showEditModal" title="编辑任务" @close="showEditModal = false">
      <form v-if="editingTask" @submit.prevent="saveTask" class="space-y-4">
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">任务标题</label>
          <Input v-model="editingTask.title" required />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">任务描述</label>
          <textarea
            v-model="editingTask.description"
            rows="3"
            class="w-full rounded-lg border border-white/10 bg-[#1a1a25] px-4 py-2.5 text-sm text-white placeholder-gray-500 transition-all duration-200 focus:border-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/20"
          />
        </div>
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="mb-1.5 block text-sm font-medium text-gray-300">优先级</label>
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
              <label class="mb-1.5 block text-sm font-medium text-gray-300">开始日期</label>
              <Input v-model="editingTask.startDate" type="date" />
            </div>
            <div>
              <label class="mb-1.5 block text-sm font-medium text-gray-300">截止日期</label>
              <Input v-model="editingTask.dueDate" type="date" />
            </div>
          </div>
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">
            进度: {{ editingTask.progress }}%
          </label>
          <input
            v-model.number="editingTask.progress"
            type="range"
            min="0"
            max="100"
            step="5"
            class="w-full h-2 bg-white/10 rounded-lg appearance-none cursor-pointer accent-indigo-500"
          />
          <div class="flex justify-between text-xs text-gray-500 mt-1">
            <span>0%</span>
            <span>50%</span>
            <span>100%</span>
          </div>
        </div>
        <div class="flex justify-end gap-2 pt-4">
          <Button variant="secondary" type="button" @click="showEditModal = false">取消</Button>
          <Button type="submit">保存</Button>
        </div>
      </form>
    </Modal>
  </div>
</template>
