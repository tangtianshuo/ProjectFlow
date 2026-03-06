<script setup lang="ts">
import { ref, onMounted } from "vue";
import { projectApi, taskApi, documentApi, type Project, type Task, type Document } from "../../../lib/api";
import Button from "../../ui/Button.vue";

const deletedProjects = ref<Project[]>([]);
const deletedTasks = ref<Task[]>([]);
const deletedDocuments = ref<Document[]>([]);
const activeTab = ref<"projects" | "tasks" | "documents">("projects");
const loading = ref(false);

async function loadDeletedItems() {
  loading.value = true;
  try {
    deletedProjects.value = await projectApi.getDeleted();
    deletedTasks.value = await taskApi.getDeleted();
    deletedDocuments.value = await documentApi.getDeleted();
  } catch (e) {
    console.error("Failed to load deleted items:", e);
  } finally {
    loading.value = false;
  }
}

async function restoreProject(id: string) {
  try {
    await projectApi.restore(id);
    await loadDeletedItems();
  } catch (e) {
    console.error("Failed to restore project:", e);
  }
}

async function restoreTask(id: string) {
  try {
    await taskApi.restore(id);
    await loadDeletedItems();
  } catch (e) {
    console.error("Failed to restore task:", e);
  }
}

async function restoreDocument(id: string) {
  try {
    await documentApi.restore(id);
    await loadDeletedItems();
  } catch (e) {
    console.error("Failed to restore document:", e);
  }
}

function formatDate(dateStr: string | null) {
  if (!dateStr) return "";
  return new Date(dateStr).toLocaleDateString("zh-CN");
}

onMounted(() => {
  loadDeletedItems();
});
</script>

<template>
  <div class="p-4 lg:p-6">
    <!-- Header -->
    <div class="mb-6 lg:mb-8">
      <h1 class="text-xl lg:text-2xl font-bold text-white">回收站</h1>
      <p class="mt-1 text-sm text-gray-400">查看和管理已删除的项目、任务和文档</p>
    </div>

    <!-- Tabs -->
    <div class="mb-4 flex border-b border-white/10">
      <button
        @click="activeTab = 'projects'"
        class="px-4 py-2 text-sm font-medium transition-colors"
        :class="activeTab === 'projects' ? 'text-indigo-400 border-b-2 border-indigo-400' : 'text-gray-400 hover:text-white'"
      >
        项目 ({{ deletedProjects.length }})
      </button>
      <button
        @click="activeTab = 'tasks'"
        class="px-4 py-2 text-sm font-medium transition-colors"
        :class="activeTab === 'tasks' ? 'text-indigo-400 border-b-2 border-indigo-400' : 'text-gray-400 hover:text-white'"
      >
        任务 ({{ deletedTasks.length }})
      </button>
      <button
        @click="activeTab = 'documents'"
        class="px-4 py-2 text-sm font-medium transition-colors"
        :class="activeTab === 'documents' ? 'text-indigo-400 border-b-2 border-indigo-400' : 'text-gray-400 hover:text-white'"
      >
        文档 ({{ deletedDocuments.length }})
      </button>
    </div>

    <!-- Content -->
    <div v-if="loading" class="py-12 text-center text-gray-500">
      加载中...
    </div>

    <div v-else-if="activeTab === 'projects'" class="space-y-3">
      <div v-if="deletedProjects.length === 0" class="py-12 text-center text-gray-500">
        暂无已删除的项目
      </div>
      <div
        v-for="project in deletedProjects"
        :key="project.id"
        class="flex items-center justify-between rounded-lg border border-white/5 bg-white/5 p-4"
      >
        <div>
          <h3 class="font-medium text-white">{{ project.name }}</h3>
          <p v-if="project.description" class="text-sm text-gray-400">{{ project.description }}</p>
          <p class="text-xs text-gray-500 mt-1">删除于: {{ formatDate(project.deletedAt) }}</p>
        </div>
        <Button size="sm" @click="restoreProject(project.id)">
          还原
        </Button>
      </div>
    </div>

    <div v-else-if="activeTab === 'tasks'" class="space-y-3">
      <div v-if="deletedTasks.length === 0" class="py-12 text-center text-gray-500">
        暂无已删除的任务
      </div>
      <div
        v-for="task in deletedTasks"
        :key="task.id"
        class="flex items-center justify-between rounded-lg border border-white/5 bg-white/5 p-4"
      >
        <div>
          <h3 class="font-medium text-white">{{ task.title }}</h3>
          <p v-if="task.description" class="text-sm text-gray-400">{{ task.description }}</p>
          <p class="text-xs text-gray-500 mt-1">删除于: {{ formatDate(task.deletedAt) }}</p>
        </div>
        <Button size="sm" @click="restoreTask(task.id)">
          还原
        </Button>
      </div>
    </div>

    <div v-else-if="activeTab === 'documents'" class="space-y-3">
      <div v-if="deletedDocuments.length === 0" class="py-12 text-center text-gray-500">
        暂无已删除的文档
      </div>
      <div
        v-for="doc in deletedDocuments"
        :key="doc.id"
        class="flex items-center justify-between rounded-lg border border-white/5 bg-white/5 p-4"
      >
        <div>
          <h3 class="font-medium text-white">{{ doc.title }}</h3>
          <p class="text-xs text-gray-500 mt-1">删除于: {{ formatDate(doc.deletedAt) }}</p>
        </div>
        <Button size="sm" @click="restoreDocument(doc.id)">
          还原
        </Button>
      </div>
    </div>
  </div>
</template>
