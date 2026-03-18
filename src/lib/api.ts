import { invoke } from "@tauri-apps/api/core";

// Types
export interface Project {
  id: string;
  name: string;
  description: string | null;
  status: number;
  startDate: string | null;
  endDate: string | null;
  ownerId: string | null;
  settings: string | null;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
}

export interface Task {
  id: string;
  projectId: string;
  parentId: string | null;
  title: string;
  description: string | null;
  status: number;
  priority: number;
  assigneeId: string | null;
  startDate: string | null;
  dueDate: string | null;
  estimatedHours: number | null;
  actualHours: number;
  progress: number;
  position: number;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
}

export interface Document {
  id: string;
  projectId: string | null;
  title: string;
  content: string | null;
  filePath: string | null;
  currentVersion: number;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
}

export interface CreateProjectRequest {
  name: string;
  description?: string;
  startDate?: string;
  endDate?: string;
}

export interface UpdateProjectRequest {
  name?: string;
  description?: string;
  status?: number;
  startDate?: string;
  endDate?: string;
}

export interface CreateTaskRequest {
  projectId: string;
  parentId?: string;
  title: string;
  description?: string;
  priority?: number;
  assigneeId?: string;
  startDate?: string;
  dueDate?: string;
  estimatedHours?: number;
}

export interface UpdateTaskRequest {
  title?: string;
  description?: string;
  status?: number;
  priority?: number;
  assigneeId?: string;
  startDate?: string;
  dueDate?: string;
  estimatedHours?: number;
  actualHours?: number;
  progress?: number;
}

export interface CreateDocumentRequest {
  projectId?: string;
  title: string;
  content?: string;
}

export interface Milestone {
  id: string;
  projectId: string;
  title: string;
  description: string | null;
  targetDate: string | null;
  status: number;
  createdAt: string;
  updatedAt: string;
  deletedAt: string | null;
}

export interface CreateMilestoneRequest {
  projectId: string;
  title: string;
  description?: string;
  targetDate?: string;
}

export interface UpdateMilestoneRequest {
  title?: string;
  description?: string;
  targetDate?: string;
  status?: number;
}

// LLM Types
export type MessageRole = "user" | "assistant" | "system";

export interface LlmMessage {
  role: MessageRole;
  content: string;
}

export interface ModelInfo {
  id: string;
  name: string;
  description: string;
}

// API functions
export const projectApi = {
  create: (data: CreateProjectRequest): Promise<Project> =>
    invoke("create_project", {
      name: data.name,
      description: data.description,
      startDate: data.startDate,
      endDate: data.endDate,
    }),

  getAll: (): Promise<Project[]> => invoke("get_all_projects"),

  get: (id: string): Promise<Project | null> => invoke("get_project", { id }),

  delete: (id: string): Promise<void> => invoke("delete_project", { id }),

  restore: (id: string): Promise<void> => invoke("restore_project", { id }),

  getDeleted: (): Promise<Project[]> => invoke("get_deleted_projects"),

  update: (id: string, data: UpdateProjectRequest): Promise<void> =>
    invoke("update_project", {
      id,
      name: data.name,
      description: data.description,
      status: data.status,
      startDate: data.startDate,
      endDate: data.endDate,
    }),
};

export const taskApi = {
  create: (data: CreateTaskRequest): Promise<Task> =>
    invoke("create_task", {
      projectId: data.projectId,
      parentId: data.parentId,
      title: data.title,
      description: data.description,
      assigneeId: data.assigneeId,
      startDate: data.startDate,
      dueDate: data.dueDate,
      estimatedHours: data.estimatedHours,
    }),

  getByProject: (projectId: string): Promise<Task[]> =>
    invoke("get_tasks_by_project", { projectId }),

  update: (id: string, data: UpdateTaskRequest): Promise<void> =>
    invoke("update_task", {
      id,
      title: data.title,
      description: data.description,
      status: data.status,
      priority: data.priority,
      assigneeId: data.assigneeId,
      startDate: data.startDate,
      dueDate: data.dueDate,
      estimatedHours: data.estimatedHours,
      actualHours: data.actualHours,
      progress: data.progress,
    }),

  delete: (id: string): Promise<void> => invoke("delete_task", { id }),

  restore: (id: string): Promise<void> => invoke("restore_task", { id }),

  getDeleted: (): Promise<Task[]> => invoke("get_deleted_tasks"),
};

export const documentApi = {
  create: (data: CreateDocumentRequest): Promise<Document> =>
    invoke("create_document", {
      projectId: data.projectId,
      title: data.title,
      content: data.content,
    }),

  getByProject: (projectId: string): Promise<Document[]> =>
    invoke("get_documents_by_project", { projectId }),

  getAll: (): Promise<Document[]> => invoke("get_all_documents"),

  update: (id: string, title?: string, content?: string): Promise<void> =>
    invoke("update_document", { id, title, content }),

  delete: (id: string): Promise<void> => invoke("delete_document", { id }),

  restore: (id: string): Promise<void> => invoke("restore_document", { id }),

  getDeleted: (): Promise<Document[]> => invoke("get_deleted_documents"),
};

export const milestoneApi = {
  create: (data: CreateMilestoneRequest): Promise<Milestone> =>
    invoke("create_milestone", {
      projectId: data.projectId,
      title: data.title,
      description: data.description,
      targetDate: data.targetDate,
    }),

  getByProject: (projectId: string): Promise<Milestone[]> =>
    invoke("get_milestones_by_project", { projectId }),

  update: (id: string, data: UpdateMilestoneRequest): Promise<void> =>
    invoke("update_milestone", {
      id,
      title: data.title,
      description: data.description,
      targetDate: data.targetDate,
      status: data.status,
    }),

  delete: (id: string): Promise<void> => invoke("delete_milestone", { id }),
};

// LLM API functions
export const llmApi = {
  saveKey: (model: string, apiKey: string): Promise<void> =>
    invoke("llm_save_key", { model, apiKey }),

  getKeyStatus: (model: string): Promise<boolean> =>
    invoke("llm_get_key_status", { model }),

  deleteKey: (model: string): Promise<void> =>
    invoke("llm_delete_key", { model }),

  chat: (
    messages: LlmMessage[],
    projectId?: string,
    model?: string
  ): Promise<void> =>
    invoke("llm_chat", { messages, projectId, model }),

  getModels: (): Promise<ModelInfo[]> => invoke("llm_get_models"),
};
