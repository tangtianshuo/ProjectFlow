mod commands;
mod db;
mod models;

use db::Database;
use std::panic;
use tauri::Manager;

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
        .setup(|app| {
            // 优先使用 exe 同级目录的 data 文件夹，便于分发
            let data_dir = if let Ok(exe_path) = std::env::current_exe() {
                exe_path.parent().map(|p| p.join("data")).unwrap_or_default()
            } else {
                std::path::PathBuf::new()
            };

            // 如果同级目录不存在或无效，回退到 AppData
            let app_data_dir = if data_dir.exists() || std::fs::create_dir_all(&data_dir).is_ok() {
                data_dir
            } else {
                app.path().app_data_dir().expect("Failed to get app data directory")
            };

            let db = Database::new(&app_data_dir).expect("Failed to initialize database");
            app.manage(db);
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
