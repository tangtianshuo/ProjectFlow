<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useProjectStore } from "../../../stores/projectStore";
import { useUiStore } from "../../../stores/uiStore";
import { milestoneApi, type Milestone } from "../../../lib/api";
import Button from "../../ui/Button.vue";
import Modal from "../../ui/Modal.vue";
import Input from "../../ui/Input.vue";

const projectStore = useProjectStore();
const uiStore = useUiStore();

const showCreateMilestoneModal = ref(false);
const newMilestone = ref({
  title: "",
  description: "",
  targetDate: "",
});

const milestones = ref<Milestone[]>([]);

// Gantt chart date range
const ganttStartDate = ref<Date>(new Date());
const ganttEndDate = ref<Date>(new Date());
const dayWidth = 40;

const tasks = computed(() => projectStore.tasks);

const ganttDays = computed(() => {
  const days: Date[] = [];
  const current = new Date(ganttStartDate.value);
  while (current <= ganttEndDate.value) {
    days.push(new Date(current));
    current.setDate(current.getDate() + 1);
  }
  return days;
});

// Parse date string correctly (handle YYYY-MM-DD format from HTML date input)
function parseDate(dateStr: string | null | undefined): Date | null {
  if (!dateStr) return null;
  // Handle ISO format with timezone
  if (dateStr.includes('T')) {
    return new Date(dateStr);
  }
  // Handle YYYY-MM-DD format (from HTML date input)
  const parts = dateStr.split('-');
  if (parts.length === 3) {
    return new Date(parseInt(parts[0]), parseInt(parts[1]) - 1, parseInt(parts[2]));
  }
  return new Date(dateStr);
}

// Calculate gantt position for task
function getTaskStyle(task: any) {
  const start = parseDate(task.startDate);
  const end = parseDate(task.dueDate);

  if (!start && !end) {
    return { display: "none" };
  }

  const taskStart = start || new Date();
  const taskEnd = end || new Date(taskStart);

  const ganttStart = ganttStartDate.value;
  const ganttEnd = ganttEndDate.value;

  // Calculate position
  const taskStartDays = (taskStart.getTime() - ganttStart.getTime()) / (1000 * 60 * 60 * 24);
  const taskDurationDays = (taskEnd.getTime() - taskStart.getTime()) / (1000 * 60 * 60 * 24) || 1;

  // Ensure task is visible within the gantt range
  const left = Math.max(0, taskStartDays) * dayWidth;
  const width = Math.max(dayWidth, taskDurationDays * dayWidth);

  // If task ends before gantt starts or starts after gantt ends, still show it
  if (taskEnd.getTime() < ganttStart.getTime() || taskStart.getTime() > ganttEnd.getTime()) {
    return { display: "none" };
  }

  return {
    left: `${left}px`,
    width: `${width}px`,
  };
}

function getStatusColor(status: number) {
  const colors = [
    "bg-gray-500",
    "bg-blue-500",
    "bg-yellow-500",
    "bg-green-500",
  ];
  return colors[status] || colors[0];
}

async function loadMilestones() {
  if (!uiStore.selectedProjectId) return;
  try {
    milestones.value = await milestoneApi.getByProject(uiStore.selectedProjectId);
  } catch (e) {
    console.error("Failed to load milestones:", e);
  }
}

async function createMilestone() {
  if (!newMilestone.value.title.trim() || !uiStore.selectedProjectId) return;

  try {
    await milestoneApi.create({
      projectId: uiStore.selectedProjectId,
      title: newMilestone.value.title,
      description: newMilestone.value.description || undefined,
      targetDate: newMilestone.value.targetDate || undefined,
    });
    showCreateMilestoneModal.value = false;
    newMilestone.value = { title: "", description: "", targetDate: "" };
    await loadMilestones();
  } catch (e) {
    console.error("Failed to create milestone:", e);
  }
}

async function toggleMilestoneStatus(milestone: Milestone) {
  try {
    await milestoneApi.update(milestone.id, {
      status: milestone.status === 0 ? 1 : 0,
    });
    await loadMilestones();
  } catch (e) {
    console.error("Failed to update milestone:", e);
  }
}

async function deleteMilestone(id: string) {
  if (confirm("确定要删除这个里程碑吗？")) {
    try {
      await milestoneApi.delete(id);
      await loadMilestones();
    } catch (e) {
      console.error("Failed to delete milestone:", e);
    }
  }
}

function getMilestonePosition(date: string | null) {
  if (!date) return 0;
  const d = parseDate(date);
  if (!d) return 0;
  const days = (d.getTime() - ganttStartDate.value.getTime()) / (1000 * 60 * 60 * 24);
  return days * dayWidth;
}

function formatDate(date: Date) {
  return `${date.getMonth() + 1}/${date.getDate()}`;
}

function formatDateFull(dateStr: string | null) {
  if (!dateStr) return "";
  return new Date(dateStr).toLocaleDateString("zh-CN");
}

// Watch for project changes and load data
watch(
  () => uiStore.selectedProjectId,
  async (projectId) => {
    if (projectId) {
      await projectStore.fetchTasks(projectId);
      await loadMilestones();
      updateDateRange();
    }
  },
  { immediate: true }
);

// Also watch for tasks changes to update gantt
watch(
  () => projectStore.tasks,
  () => {
    updateDateRange();
  },
  { deep: true }
);

// Watch for milestones changes
watch(
  () => milestones.value,
  () => {
    updateDateRange();
  },
  { deep: true }
);

function updateDateRange() {
  const allTasks = projectStore.tasks;
  if (!allTasks || allTasks.length === 0) {
    const now = new Date();
    ganttStartDate.value = new Date(now.getFullYear(), now.getMonth(), 1);
    ganttEndDate.value = new Date(now.getFullYear(), now.getMonth() + 2, 0);
    return;
  }

  let minDate: Date | null = null;
  let maxDate: Date | null = null;

  allTasks.forEach((task) => {
    const startD = parseDate(task.startDate);
    const endD = parseDate(task.dueDate);
    if (startD) {
      if (!minDate || startD < minDate) minDate = startD;
    }
    if (endD) {
      if (!maxDate || endD > maxDate) maxDate = endD;
    }
  });

  milestones.value.forEach((m) => {
    const d = parseDate(m.targetDate);
    if (d) {
      if (!minDate || d < minDate) minDate = d;
      if (!maxDate || d > maxDate) maxDate = d;
    }
  });

  // Default range if no dates
  const now = new Date();
  ganttStartDate.value = minDate || new Date(now.getFullYear(), now.getMonth(), 1);
  ganttEndDate.value = maxDate || new Date(now.getFullYear(), now.getMonth() + 2, 0);
}
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- Header -->
    <div class="flex items-center justify-between border-b border-white/5 px-4 py-3">
      <h2 class="text-lg font-semibold text-white">甘特图</h2>
      <Button size="sm" @click="showCreateMilestoneModal = true">
        <svg class="mr-1 h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        添加里程碑
      </Button>
    </div>

    <!-- Milestones -->
    <div v-if="milestones.length > 0" class="border-b border-white/5 px-4 py-2 bg-white/5">
      <div class="text-xs text-gray-400 mb-2">里程碑</div>
      <div class="flex flex-wrap gap-2">
        <div
          v-for="milestone in milestones"
          :key="milestone.id"
          class="flex items-center gap-2 rounded-full px-3 py-1 text-xs"
          :class="milestone.status === 1 ? 'bg-green-500/20 text-green-400' : 'bg-indigo-500/20 text-indigo-400'"
        >
          <button @click="toggleMilestoneStatus(milestone)">
            <svg v-if="milestone.status === 1" class="h-3.5 w-3.5" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
            </svg>
            <svg v-else class="h-3.5 w-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </button>
          <span>{{ milestone.title }}</span>
          <span v-if="milestone.targetDate" class="text-gray-500">{{ formatDateFull(milestone.targetDate) }}</span>
          <button @click="deleteMilestone(milestone.id)" class="text-gray-500 hover:text-red-400">
            <svg class="h-3 w-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Gantt Chart -->
    <div class="flex-1 overflow-auto">
      <!-- Timeline header -->
      <div class="sticky top-0 z-10 flex border-b border-white/5 bg-[#0d0d12]">
        <div class="w-48 flex-shrink-0 border-r border-white/5 px-3 py-2 text-sm font-medium text-gray-400">
          任务
        </div>
        <div class="flex" :style="{ width: `${ganttDays.length * dayWidth}px` }">
          <div
            v-for="day in ganttDays"
            :key="day.toISOString()"
            class="flex-shrink-0 text-center text-xs text-gray-500 border-r border-white/5 py-1"
            :class="{ 'bg-indigo-500/10': day.getDay() === 0 || day.getDay() === 6 }"
            :style="{ width: `${dayWidth}px` }"
          >
            <div class="text-[10px]">{{ formatDate(day) }}</div>
            <div class="text-[9px]">{{ ["日", "一", "二", "三", "四", "五", "六"][day.getDay()] }}</div>
          </div>
        </div>
      </div>

      <!-- Milestone timeline markers -->
      <div v-if="milestones.length > 0" class="sticky top-10 z-5 flex border-b border-white/5 bg-yellow-500/10">
        <div class="w-48 flex-shrink-0 border-r border-white/5 px-3 py-1 text-xs font-medium text-yellow-400">
          里程碑
        </div>
        <div class="relative" :style="{ width: `${ganttDays.length * dayWidth}px`, height: '24px' }">
          <template v-for="milestone in milestones" :key="'header-m-' + milestone.id">
            <div
              v-if="milestone.targetDate"
              class="absolute top-0 h-full w-0.5 bg-yellow-500/50 flex items-center"
              :style="{ left: `${getMilestonePosition(milestone.targetDate)}px` }"
            >
              <span class="text-[9px] text-yellow-400 ml-1 whitespace-nowrap">{{ milestone.title }}</span>
            </div>
          </template>
        </div>
      </div>

      <!-- Tasks -->
      <div class="divide-y divide-white/5">
        <div
          v-for="task in tasks"
          :key="task.id"
          class="flex items-center hover:bg-white/5"
        >
          <div class="w-48 flex-shrink-0 border-r border-white/5 px-3 py-2">
            <div class="text-sm text-white truncate">{{ task.title }}</div>
            <div class="text-xs text-gray-500">
              {{ formatDateFull(task.startDate) }} - {{ formatDateFull(task.dueDate) }}
            </div>
          </div>
          <div class="relative h-10" :style="{ width: `${ganttDays.length * dayWidth}px` }">
            <!-- Grid lines -->
            <div class="absolute inset-0 flex">
              <div
                v-for="day in ganttDays"
                :key="day.toISOString()"
                class="flex-shrink-0 border-r border-white/5"
                :class="{ 'bg-indigo-500/5': day.getDay() === 0 || day.getDay() === 6 }"
                :style="{ width: `${dayWidth}px` }"
              />
            </div>
            <!-- Task bar -->
            <div
              v-if="task.startDate || task.dueDate"
              class="absolute top-1.5 h-7 rounded-md flex items-center px-2 text-xs text-white truncate cursor-pointer transition-all hover:opacity-80"
              :class="getStatusColor(task.status)"
              :style="getTaskStyle(task)"
            >
              {{ task.progress }}%
            </div>
            <!-- Milestone markers per row -->
            <template v-for="milestone in milestones" :key="'m-' + milestone.id">
              <div
                v-if="milestone.targetDate"
                class="absolute top-0 h-full w-0.5 bg-yellow-500"
                :style="{ left: `${getMilestonePosition(milestone.targetDate)}px` }"
              />
            </template>
          </div>
        </div>

        <!-- Empty state -->
        <div v-if="tasks.length === 0" class="px-4 py-12 text-center text-gray-500">
          暂无任务，请先添加任务
        </div>
      </div>
    </div>

    <!-- Create Milestone Modal -->
    <Modal :open="showCreateMilestoneModal" title="添加里程碑" @close="showCreateMilestoneModal = false">
      <form @submit.prevent="createMilestone" class="space-y-4">
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">里程碑名称</label>
          <Input v-model="newMilestone.title" placeholder="请输入里程碑名称" required />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">目标日期</label>
          <Input v-model="newMilestone.targetDate" type="date" />
        </div>
        <div>
          <label class="mb-1.5 block text-sm font-medium text-gray-300">描述（可选）</label>
          <Input v-model="newMilestone.description" placeholder="请输入描述" />
        </div>
        <div class="flex justify-end gap-2">
          <Button type="button" variant="secondary" @click="showCreateMilestoneModal = false">取消</Button>
          <Button type="submit">创建</Button>
        </div>
      </form>
    </Modal>
  </div>
</template>
