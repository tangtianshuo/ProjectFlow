import { defineStore } from "pinia";
import { ref } from "vue";

export type ViewMode = "dashboard" | "projects" | "tasks" | "documents" | "settings" | "recycleBin";
export type TaskViewMode = "kanban" | "gantt";

export const useUiStore = defineStore("ui", () => {
  const currentView = ref<ViewMode>("dashboard");
  const taskViewMode = ref<TaskViewMode>("kanban");
  const sidebarCollapsed = ref(false);
  const selectedProjectId = ref<string | null>(null);

  function setView(view: ViewMode) {
    currentView.value = view;
  }

  function setTaskViewMode(mode: TaskViewMode) {
    taskViewMode.value = mode;
  }

  function toggleSidebar() {
    sidebarCollapsed.value = !sidebarCollapsed.value;
  }

  function selectProject(projectId: string | null) {
    selectedProjectId.value = projectId;
  }

  return {
    currentView,
    taskViewMode,
    sidebarCollapsed,
    selectedProjectId,
    setView,
    setTaskViewMode,
    toggleSidebar,
    selectProject,
  };
});
