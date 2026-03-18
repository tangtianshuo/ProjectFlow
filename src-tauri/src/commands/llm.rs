//! LLM Tauri commands
//!
//! Provides commands for API key management and chat functionality.

use crate::db::Database;
use crate::llm::{self, Message, ModelConfig, LitellmClient};
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

/// Request to save model configuration
#[derive(Debug, Deserialize)]
pub struct ModelConfigRequest {
    pub model_id: String,
    pub base_url: String,
    pub model_name: String,
    pub api_key: String,
}

/// Response for model config
#[derive(Debug, Serialize)]
pub struct ModelConfigResponse {
    pub base_url: String,
    pub model_name: String,
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

/// Save model configuration including API key
///
/// Stores both the model configuration (base_url, model_name) and the API key
/// securely in the system keychain.
#[tauri::command]
pub fn llm_save_model_config(config: ModelConfigRequest) -> Result<(), String> {
    log::info!("[llm_save_model_config] Saving config for model: {}", config.model_id);
    // Store API key
    llm::store_api_key(&config.model_id, &config.api_key)?;
    log::info!("[llm_save_model_config] API key stored successfully for model: {}", config.model_id);

    // Store model config
    let model_config = ModelConfig {
        base_url: config.base_url,
        model_name: config.model_name,
    };
    llm::store_model_config(&config.model_id, &model_config)?;
    log::info!("[llm_save_model_config] Model config stored for: {}", config.model_id);

    Ok(())
}

/// Get model configuration
///
/// Retrieves the model configuration (base_url, model_name) from the system keychain.
/// Does not return the API key for security reasons.
#[tauri::command]
pub fn llm_get_model_config(model_id: String) -> Result<Option<ModelConfigResponse>, String> {
    match llm::retrieve_model_config(&model_id)? {
        Some(config) => Ok(Some(ModelConfigResponse {
            base_url: config.base_url,
            model_name: config.model_name,
        })),
        None => Ok(None),
    }
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
    log::info!("[llm_chat] Received model: {:?}", model);
    let model_name = model.unwrap_or_else(|| "gpt-4o".to_string());
    log::info!("[llm_chat] Using model_name: {}", model_name);

    // DEBUG: Check all available keys in keyring
    log::info!("[llm_chat] DEBUG: Attempting to retrieve API key for model: '{}'", model_name);

    // Check if key exists using has_api_key first
    let key_exists = llm::has_api_key(&model_name)?;
    log::info!("[llm_chat] DEBUG: has_api_key({}) = {}", model_name, key_exists);

    // Also check for common model IDs
    log::info!("[llm_chat] DEBUG: Checking for 'minimax' key: {}", llm::has_api_key("minimax")?);
    log::info!("[llm_chat] DEBUG: Checking for 'gpt-4o' key: {}", llm::has_api_key("gpt-4o")?);
    log::info!("[llm_chat] DEBUG: Checking for 'kimi' key: {}", llm::has_api_key("kimi")?);
    let api_key = match llm::retrieve_api_key(&model_name) {
        Ok(Some(key)) => {
            log::info!("[llm_chat] API key found for model: {}", model_name);
            key
        }
        Ok(None) => {
            log::warn!("[llm_chat] No API key found for model: {}", model_name);
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

    // Retrieve model config for base_url
    log::info!("[llm_chat] DEBUG: Retrieving model config for: {}", model_name);
    let base_url = match llm::retrieve_model_config(&model_name) {
        Ok(Some(config)) => {
            log::info!("[llm_chat] DEBUG: Found config for {}: base_url={}, model_name={}",
                model_name, config.base_url, config.model_name);
            Some(config.base_url)
        }
        Ok(None) => {
            log::info!("[llm_chat] DEBUG: No config found for {}", model_name);
            None
        }
        Err(e) => {
            log::warn!("[llm_chat] DEBUG: Error retrieving config for {}: {}", model_name, e);
            None
        }
    };

    // Check configs for other models too
    log::info!("[llm_chat] DEBUG: Checking config for 'minimax': {:?}", llm::retrieve_model_config("minimax"));
    log::info!("[llm_chat] DEBUG: Checking config for 'gpt-4o': {:?}", llm::retrieve_model_config("gpt-4o"));

    // Create client and stream chat
    let client = LitellmClient::new(api_key, model_name, base_url);

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
/// Returns a list of supported LLM models including Chinese providers.
#[tauri::command]
pub fn llm_get_models() -> Vec<ModelInfo> {
    vec![
        // OpenAI models
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
        // Kimi (Moonshot AI)
        ModelInfo {
            id: "kimi".to_string(),
            name: "Kimi".to_string(),
            description: "Moonshot AI - Long context support".to_string(),
        },
        ModelInfo {
            id: "kimi-flash".to_string(),
            name: "Kimi Flash".to_string(),
            description: "Moonshot AI - Fast response".to_string(),
        },
        // DeepSeek
        ModelInfo {
            id: "deepseek-chat".to_string(),
            name: "DeepSeek Chat".to_string(),
            description: "DeepSeek - Open source AI assistant".to_string(),
        },
        ModelInfo {
            id: "deepseek-coder".to_string(),
            name: "DeepSeek Coder".to_string(),
            description: "DeepSeek - Code specialist".to_string(),
        },
        // MiniMax
        ModelInfo {
            id: "minimax".to_string(),
            name: "MiniMax".to_string(),
            description: "MiniMax - Multi-modal AI".to_string(),
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
