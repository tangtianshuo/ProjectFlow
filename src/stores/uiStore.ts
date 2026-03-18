import { defineStore } from "pinia";
import { ref } from "vue";

export type ViewMode = "dashboard" | "projects" | "tasks" | "documents" | "settings" | "recycleBin" | "llm";
export type TaskViewMode = "kanban" | "gantt";
export type Theme = "dark" | "light";

export const useUiStore = defineStore("ui", () => {
  const currentView = ref<ViewMode>("dashboard");
  const taskViewMode = ref<TaskViewMode>("kanban");
  const sidebarCollapsed = ref(false);
  const selectedProjectId = ref<string | null>(null);
  const theme = ref<Theme>("dark");

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

  function initTheme() {
    const saved = localStorage.getItem("projectflow-theme");
    if (saved === "light" || saved === "dark") {
      theme.value = saved;
    } else if (window.matchMedia("(prefers-color-scheme: light)").matches) {
      theme.value = "light";
    }
    document.documentElement.setAttribute("data-theme", theme.value);
  }

  function toggleTheme() {
    theme.value = theme.value === "dark" ? "light" : "dark";
    document.documentElement.setAttribute("data-theme", theme.value);
    localStorage.setItem("projectflow-theme", theme.value);
  }

  // Initialize theme on store creation
  initTheme();

  return {
    currentView,
    taskViewMode,
    sidebarCollapsed,
    selectedProjectId,
    theme,
    setView,
    setTaskViewMode,
    toggleSidebar,
    selectProject,
    initTheme,
    toggleTheme,
  };
});
