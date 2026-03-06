import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { projectApi, taskApi, type Project, type Task, type CreateProjectRequest, type CreateTaskRequest, type UpdateTaskRequest, type UpdateProjectRequest } from "../lib/api";

export const useProjectStore = defineStore("projects", () => {
  const projects = ref<Project[]>([]);
  const currentProject = ref<Project | null>(null);
  const tasks = ref<Task[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  const activeProjects = computed(() =>
    projects.value.filter((p) => p.status === 1)
  );

  const completedProjects = computed(() =>
    projects.value.filter((p) => p.status === 2)
  );

  // Compute project progress based on tasks
  const projectProgress = computed(() => {
    const progressMap: Record<string, number> = {};
    projects.value.forEach((project) => {
      const projectTasks = tasks.value.filter((t) => t.projectId === project.id);
      if (projectTasks.length === 0) {
        progressMap[project.id] = 0;
      } else {
        const completedCount = projectTasks.filter((t) => t.status === 3).length;
        progressMap[project.id] = Math.round((completedCount / projectTasks.length) * 100);
      }
    });
    return progressMap;
  });

  async function fetchProjects() {
    loading.value = true;
    error.value = null;
    try {
      projects.value = await projectApi.getAll();
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function fetchProject(id: string) {
    loading.value = true;
    error.value = null;
    try {
      currentProject.value = await projectApi.get(id);
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function createProject(data: CreateProjectRequest) {
    loading.value = true;
    error.value = null;
    try {
      const project = await projectApi.create(data);
      projects.value.unshift(project);
      return project;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteProject(id: string) {
    loading.value = true;
    error.value = null;
    try {
      await projectApi.delete(id);
      projects.value = projects.value.filter((p) => p.id !== id);
      if (currentProject.value?.id === id) {
        currentProject.value = null;
      }
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateProject(id: string, data: UpdateProjectRequest) {
    loading.value = true;
    error.value = null;
    try {
      await projectApi.update(id, data);
      const index = projects.value.findIndex((p) => p.id === id);
      if (index !== -1) {
        projects.value[index] = { ...projects.value[index], ...data };
      }
      if (currentProject.value?.id === id) {
        currentProject.value = { ...currentProject.value, ...data };
      }
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function fetchTasks(projectId: string) {
    loading.value = true;
    error.value = null;
    try {
      tasks.value = await taskApi.getByProject(projectId);
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function createTask(data: CreateTaskRequest) {
    loading.value = true;
    error.value = null;
    try {
      const task = await taskApi.create(data);
      tasks.value.push(task);
      return task;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateTask(id: string, data: UpdateTaskRequest) {
    loading.value = true;
    error.value = null;
    try {
      await taskApi.update(id, data);
      const index = tasks.value.findIndex((t) => t.id === id);
      if (index !== -1) {
        tasks.value[index] = { ...tasks.value[index], ...data };
      }
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteTask(id: string) {
    loading.value = true;
    error.value = null;
    try {
      await taskApi.delete(id);
      tasks.value = tasks.value.filter((t) => t.id !== id);
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  return {
    projects,
    currentProject,
    tasks,
    loading,
    error,
    activeProjects,
    completedProjects,
    projectProgress,
    fetchProjects,
    fetchProject,
    createProject,
    deleteProject,
    updateProject,
    fetchTasks,
    createTask,
    updateTask,
    deleteTask,
  };
});
