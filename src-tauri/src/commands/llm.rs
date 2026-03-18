//! LLM Tauri commands
//!
//! Provides commands for API key management and chat functionality.

use crate::db::Database;
use crate::llm::{self, Message, OpenAIClient};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

/// Request to save API key
#[derive(Debug, Deserialize)]
pub struct SaveKeyRequest {
    pub model: String,
    pub api_key: String,
}

/// Response for chat request
#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub content: String,
}

/// Save API key securely
///
/// Encrypts and stores the API key using the system keychain.
#[tauri::command]
pub fn llm_save_key(model: String, api_key: String) -> Result<(), String> {
    llm::store_api_key(&model, &api_key)
}

/// Get API key status
///
/// Returns true if an API key is stored for the given model.
#[tauri::command]
pub fn llm_get_key_status(model: String) -> Result<bool, String> {
    llm::has_api_key(&model)
}

/// Delete API key
///
/// Removes the stored API key from the system keychain.
#[tauri::command]
pub fn llm_delete_key(model: String) -> Result<(), String> {
    llm::delete_api_key(&model)
}

/// Send chat message and receive streaming response
///
/// Gets project context if project_id is provided, then sends the
/// messages to the LLM and streams the response back via events.
#[tauri::command]
pub async fn llm_chat(
    app: AppHandle,
    db: State<'_, Database>,
    messages: Vec<Message>,
    project_id: Option<String>,
    model: Option<String>,
) -> Result<(), String> {
    // Get API key
    let model_name = model.unwrap_or_else(|| "gpt-4o".to_string());
    let api_key = match llm::retrieve_api_key(&model_name) {
        Ok(Some(key)) => key,
        Ok(None) => {
            let _ = app.emit("llm-error", "No API key found. Please save your API key first.");
            return Err("No API key found".to_string());
        }
        Err(e) => {
            let _ = app.emit("llm-error", format!("Failed to retrieve API key: {}", e));
            return Err(format!("Failed to retrieve API key: {}", e));
        }
    };

    // Build messages with project context if project_id provided
    let mut all_messages = messages;

    if let Some(ref pid) = project_id {
        // Get project info
        if let Ok(Some(project)) = db.get_project(pid) {
            // Get tasks for the project
            let tasks = db.get_tasks_by_project(pid).unwrap_or_default();

            // Build system prompt with project context
            let system_prompt = llm::build_system_prompt(&project, &tasks);

            // Insert system message at the beginning
            all_messages.insert(
                0,
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
            );
        }
    }

    // Create client and stream chat
    // TODO: Retrieve base_url from model config in Task 3
    let client = OpenAIClient::new(api_key, Some(model_name), None);

    match client.stream_chat(all_messages).await {
        Ok(mut stream) => {
            // Stream tokens to frontend
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(token) => {
                        let _ = app.emit("llm-token", token);
                    }
                    Err(e) => {
                        let _ = app.emit("llm-error", e);
                        return Err("Streaming error".to_string());
                    }
                }
            }

            // Signal completion
            let _ = app.emit("llm-done", ());
            Ok(())
        }
        Err(e) => {
            let _ = app.emit("llm-error", e.clone());
            Err(e)
        }
    }
}

/// Get available models
///
/// Returns a list of supported LLM models.
#[tauri::command]
pub fn llm_get_models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "gpt-4o".to_string(),
            name: "GPT-4o".to_string(),
            description: "OpenAI's latest flagship model".to_string(),
        },
        ModelInfo {
            id: "gpt-4o-mini".to_string(),
            name: "GPT-4o Mini".to_string(),
            description: "Smaller, faster GPT-4o".to_string(),
        },
        ModelInfo {
            id: "gpt-4-turbo".to_string(),
            name: "GPT-4 Turbo".to_string(),
            description: "Previous generation GPT-4".to_string(),
        },
        ModelInfo {
            id: "gpt-3.5-turbo".to_string(),
            name: "GPT-3.5 Turbo".to_string(),
            description: "Fast and cost-effective".to_string(),
        },
    ]
}

/// Model information
#[derive(Debug, Serialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}
