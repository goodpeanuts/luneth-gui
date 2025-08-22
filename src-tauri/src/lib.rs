mod client;
mod db;
mod extract;
mod luneth;
mod scrap;

use std::sync::Arc;

// Import all items from the extract module, including process_text
use crate::luneth::{
    check_local_image_exists, clear_client_auth, get_all_op_history, get_all_records,
    get_app_local_data_dir, launch_auto_scrap_task, launch_manual_scrap_task, pull_record_slim,
    set_client_auth, set_task_base_url,
};
use ::luneth::crawl::CrawlError;
use extract::{export_to_file, process_text, toggle_line_selection};
use tauri::Manager as _;

#[cfg(debug_assertions)]
use log::LevelFilter;

#[cfg(not(debug_assertions))]
use log::LevelFilter;

pub(crate) struct AppState {
    pub db: Arc<luneth_db::DbOperator>,
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Crawl error: {0}")]
    CrawlError(#[from] CrawlError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] luneth_db::DbError),

    #[error("File error: {0}")]
    ImageError(#[from] std::io::Error),

    #[error("File system error: {0}")]
    FileSystemError(String),

    #[error("Crawl image error: {0}")]
    CrawlImageError(String),
}

impl From<AppError> for String {
    fn from(e: AppError) -> Self {
        e.to_string()
    }
}

#[expect(clippy::large_stack_frames)]
#[expect(clippy::exit)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: None,
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level({
                    #[cfg(debug_assertions)]
                    {
                        LevelFilter::Info
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        LevelFilter::Info
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            log::info!("Starting Tauri application setup");

            // Initialize database connection using a runtime
            let app_handle = app.handle().clone();
            let rt = tokio::runtime::Runtime::new().map_err(|e| {
                log::error!("Failed to create tokio runtime: {e}");
                Box::new(std::io::Error::other(e.to_string()))
            })?;

            log::debug!("Initializing database connection");
            let db_result = rt.block_on(async { luneth_db::DbOperator::init(&app_handle).await });

            match db_result {
                Ok(db) => {
                    log::info!("Database initialized successfully");
                    let app_state = AppState { db: Arc::new(db) };
                    app.manage(Arc::new(app_state));
                    log::info!("Application setup completed successfully");
                    Ok(())
                }
                Err(e) => {
                    log::error!("Failed to initialize database: {e}");
                    Err(Box::new(std::io::Error::other(e.to_string())))
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            process_text,
            toggle_line_selection,
            export_to_file,
            get_all_records,
            launch_auto_scrap_task,
            launch_manual_scrap_task,
            get_all_op_history,
            set_task_base_url,
            set_client_auth,
            clear_client_auth,
            pull_record_slim,
            get_app_local_data_dir,
            check_local_image_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
