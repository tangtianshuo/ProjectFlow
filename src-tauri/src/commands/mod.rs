use crate::db::Database;
use crate::models::*;
use tauri::State;

// Project commands
#[tauri::command]
pub fn create_project(
    db: State<'_, Database>,
    name: String,
    description: Option<String>,
    startDate: Option<String>,
    endDate: Option<String>,
) -> Result<Project, String> {
    db.create_project(&name, description.as_deref(), startDate.as_deref(), endDate.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_projects(db: State<'_, Database>) -> Result<Vec<Project>, String> {
    db.get_all_projects().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_project(db: State<'_, Database>, id: String) -> Result<Option<Project>, String> {
    db.get_project(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_project(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.delete_project(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_project(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.restore_project(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_deleted_projects(db: State<'_, Database>) -> Result<Vec<Project>, String> {
    db.get_deleted_projects().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_project(
    db: State<'_, Database>,
    id: String,
    name: Option<String>,
    description: Option<String>,
    status: Option<i32>,
    startDate: Option<String>,
    endDate: Option<String>,
) -> Result<(), String> {
    db.update_project(
        &id,
        name.as_deref(),
        description.as_deref(),
        status,
        startDate.as_deref(),
        endDate.as_deref(),
    )
    .map_err(|e| e.to_string())
}

// Task commands
#[tauri::command]
pub fn create_task(
    db: State<'_, Database>,
    projectId: String,
    parentId: Option<String>,
    title: String,
    description: Option<String>,
    assigneeId: Option<String>,
    startDate: Option<String>,
    dueDate: Option<String>,
    estimatedHours: Option<f64>,
) -> Result<Task, String> {
    db.create_task(
        &projectId,
        parentId.as_deref(),
        &title,
        description.as_deref(),
        assigneeId.as_deref(),
        startDate.as_deref(),
        dueDate.as_deref(),
        estimatedHours,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tasks_by_project(db: State<'_, Database>, projectId: String) -> Result<Vec<Task>, String> {
    db.get_tasks_by_project(&projectId).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_task(
    db: State<'_, Database>,
    id: String,
    title: Option<String>,
    description: Option<String>,
    status: Option<i32>,
    priority: Option<i32>,
    assigneeId: Option<String>,
    startDate: Option<String>,
    dueDate: Option<String>,
    estimatedHours: Option<f64>,
    actualHours: Option<f64>,
    progress: Option<f64>,
) -> Result<(), String> {
    db.update_task(
        &id,
        title.as_deref(),
        description.as_deref(),
        status,
        priority,
        assigneeId.as_deref(),
        startDate.as_deref(),
        dueDate.as_deref(),
        estimatedHours,
        actualHours,
        progress,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_task(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.delete_task(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_task(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.restore_task(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_deleted_tasks(db: State<'_, Database>) -> Result<Vec<Task>, String> {
    db.get_deleted_tasks().map_err(|e| e.to_string())
}

// Document commands
#[tauri::command]
pub fn create_document(
    db: State<'_, Database>,
    projectId: Option<String>,
    title: String,
    content: Option<String>,
) -> Result<Document, String> {
    db.create_document(projectId.as_deref(), &title, content.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_documents_by_project(db: State<'_, Database>, projectId: String) -> Result<Vec<Document>, String> {
    db.get_documents_by_project(&projectId).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_documents(db: State<'_, Database>) -> Result<Vec<Document>, String> {
    db.get_all_documents().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_document(
    db: State<'_, Database>,
    id: String,
    title: Option<String>,
    content: Option<String>,
) -> Result<(), String> {
    db.update_document(&id, title.as_deref(), content.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_document(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.delete_document(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_document(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.restore_document(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_deleted_documents(db: State<'_, Database>) -> Result<Vec<Document>, String> {
    db.get_deleted_documents().map_err(|e| e.to_string())
}

// Milestone commands
#[tauri::command]
pub fn create_milestone(
    db: State<'_, Database>,
    projectId: String,
    title: String,
    description: Option<String>,
    targetDate: Option<String>,
) -> Result<Milestone, String> {
    db.create_milestone(&projectId, &title, description.as_deref(), targetDate.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_milestones_by_project(db: State<'_, Database>, projectId: String) -> Result<Vec<Milestone>, String> {
    db.get_milestones_by_project(&projectId).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_milestone(
    db: State<'_, Database>,
    id: String,
    title: Option<String>,
    description: Option<String>,
    targetDate: Option<String>,
    status: Option<i32>,
) -> Result<(), String> {
    db.update_milestone(&id, title.as_deref(), description.as_deref(), targetDate.as_deref(), status)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_milestone(db: State<'_, Database>, id: String) -> Result<(), String> {
    db.delete_milestone(&id).map_err(|e| e.to_string())
}
