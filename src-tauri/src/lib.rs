mod commands;
mod db;
mod llm;
mod models;
pub mod state;

use db::Database;
use db::LlmSettings;
use state::AppState;
use std::panic;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_http::reqwest;

static SIDECAR_PORT: Mutex<Option<u16>> = Mutex::new(None);

#[tauri::command]
async fn get_llm_settings(db: State<'_, Database>) -> Result<LlmSettings, String> {
    db.get_llm_settings().map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_llm_settings(
    db: State<'_, Database>,
    provider: String,
    api_key: String,
    api_url: String,
    model: String,
) -> Result<(), String> {
    db.update_llm_settings(&provider, &api_key, &api_url, &model)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_sidecar(app: AppHandle) -> Result<u16, String> {
    {
        let port = SIDECAR_PORT.lock().unwrap();
        if port.is_some() {
            return Ok(port.unwrap());
        }
    }

    let port = 8765u16;

    let exe_dir = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default();

    let sidecar_dir = exe_dir.join("sidecar");

    log::info!("Looking for sidecar in: {:?}", sidecar_dir);

    #[cfg(debug_assertions)]
    {
        let python_path = if cfg!(windows) { "python" } else { "python3" };

        log::info!("Starting sidecar with Python: {:?}", python_path);

        let result = tauri_plugin_shell::ShellExt::shell(&app)
            .command(python_path)
            .args(["main.py", "--port", &port.to_string()])
            .current_dir(&sidecar_dir)
            .spawn();

        match result {
            Ok(_) => log::info!("Sidecar spawn command executed"),
            Err(e) => log::error!("Failed to spawn sidecar: {}", e),
        }
    }

    #[cfg(not(debug_assertions))]
    {
        let exe_name = if cfg!(windows) {
            "llm_sidecar.exe"
        } else {
            "llm_sidecar"
        };
        let exe_path = sidecar_dir.join("dist").join(exe_name);

        log::info!("Starting sidecar exe: {:?}", exe_path);

        tauri_plugin_shell::ShellExt::shell(&app)
            .command(&exe_path)
            .args(["--port", &port.to_string()])
            .current_dir(&sidecar_dir.join("dist"))
            .spawn()
            .map_err(|e| format!("Failed to start sidecar: {}", e))?;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    {
        let mut port_guard = SIDECAR_PORT.lock().unwrap();
        *port_guard = Some(port);
    }

    log::info!("LLM Sidecar started on port {}", port);
    Ok(port)
}

#[tauri::command]
async fn get_sidecar_status() -> Result<Option<u16>, String> {
    let port = SIDECAR_PORT.lock().unwrap();
    Ok(*port)
}

#[tauri::command]
async fn update_llm_config_sidecar(
    state: State<'_, AppState>,
    provider: String,
    api_key: String,
    api_url: String,
    model: String,
) -> Result<serde_json::Value, String> {
    let port = {
        let port_guard = SIDECAR_PORT.lock().unwrap();
        *port_guard
    }
    .ok_or("Sidecar not started")?;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://127.0.0.1:{}/config", port))
        .json(&serde_json::json!({
            "provider": provider,
            "api_key": api_key,
            "api_url": api_url,
            "model": model
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Config update failed: {}", response.status()));
    }

    let result = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;

    state.update_config(&provider, &api_key, &api_url, &model);

    Ok(result)
}

#[tauri::command]
async fn chat_with_llm_sidecar(
    state: State<'_, AppState>,
    messages: Vec<serde_json::Value>,
    temperature: f64,
    max_tokens: Option<u32>,
) -> Result<serde_json::Value, String> {
    let port = {
        let port_guard = SIDECAR_PORT.lock().unwrap();
        *port_guard
    }
    .ok_or("Sidecar not started")?;

    let config = state.get_config();

    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://127.0.0.1:{}/chat", port))
        .json(&serde_json::json!({
            "messages": messages,
            "model": config.model,
            "temperature": temperature,
            "max_tokens": max_tokens,
            "stream": false
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Chat request failed: {}", response.status()));
    }

    let result = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(result)
}

#[tauri::command]
async fn chat_with_llm_stream(
    state: State<'_, AppState>,
    messages: Vec<serde_json::Value>,
    temperature: f64,
    max_tokens: Option<u32>,
) -> Result<String, String> {
    let port = {
        let port_guard = SIDECAR_PORT.lock().unwrap();
        *port_guard
    }
    .ok_or("Sidecar not started")?;

    let config = state.get_config();

    let client = reqwest::Client::new();
    let response = client
        .post(format!("http://127.0.0.1:{}/chat", port))
        .json(&serde_json::json!({
            "messages": messages,
            "model": config.model,
            "temperature": temperature,
            "max_tokens": max_tokens,
            "stream": true
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("Chat request failed: {}", response.status()));
    }

    let mut result = String::new();
    let mut stream = response.bytes_stream();

    use futures_util::stream::StreamExt;
    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => {
                if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let data = line.trim_start_matches("data: ");
                            if data == "[DONE]" {
                                break;
                            }
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                                if let Some(content) = json.get("content").and_then(|v| v.as_str())
                                {
                                    result.push_str(content);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Stream error: {}", e);
                break;
            }
        }
    }

    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::init();

    // Set up panic hook for logging
    panic::set_hook(Box::new(|panic_info| {
        log::error!("Application panic: {}", panic_info);
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .setup(|app| {
            // 优先使用 exe 同级目录的 data 文件夹，便于分发
            let data_dir = if let Ok(exe_path) = std::env::current_exe() {
                exe_path
                    .parent()
                    .map(|p| p.join("data"))
                    .unwrap_or_default()
            } else {
                std::path::PathBuf::new()
            };

            // 如果同级目录不存在或无效，回退到 AppData
            let app_data_dir = if data_dir.exists() || std::fs::create_dir_all(&data_dir).is_ok() {
                data_dir
            } else {
                app.path()
                    .app_data_dir()
                    .expect("Failed to get app data directory")
            };

            let db = Database::new(&app_data_dir).expect("Failed to initialize database");
            app.manage(db);
            app.manage(AppState::new());
            log::info!("ProjectFlow initialized at: {:?}", app_data_dir);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_project,
            commands::get_all_projects,
            commands::get_project,
            commands::delete_project,
            commands::restore_project,
            commands::get_deleted_projects,
            commands::update_project,
            commands::create_task,
            commands::get_tasks_by_project,
            commands::update_task,
            commands::delete_task,
            commands::restore_task,
            commands::get_deleted_tasks,
            commands::create_document,
            commands::get_documents_by_project,
            commands::get_all_documents,
            commands::update_document,
            commands::delete_document,
            commands::restore_document,
            commands::get_deleted_documents,
            commands::create_milestone,
            commands::get_milestones_by_project,
            commands::update_milestone,
            commands::delete_milestone,
            llm::llm::update_llm_config,
            llm::llm::chat_with_llm,
            start_sidecar,
            get_sidecar_status,
            update_llm_config_sidecar,
            chat_with_llm_sidecar,
            chat_with_llm_stream,
            get_llm_settings,
            save_llm_settings,
        ])
        .run(tauri::generate_context!())
        .expect("Failed to generate Tauri context; ensure build script sets OUT_DIR")
        .unwrap_or_else(|e| {
            eprintln!("Error running tauri application: {}", e);
            std::process::exit(1);
        });
}
