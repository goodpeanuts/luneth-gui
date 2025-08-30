mod db {
    pub mod log;
    pub mod read;
    pub mod write;
}
mod common;
mod handlers;
mod command {
    pub mod config;
    pub mod extract;
    pub mod image;
    pub mod interaction;
    pub mod log;
    pub mod task;
}

use ::luneth::crawl::CrawlError;
use log::{Level, LevelFilter};
use std::sync::Arc;
use tauri::Manager as _;
use tauri_plugin_log::{RotationStrategy, Target, TargetKind, TimezoneStrategy};

use crate::command::{
    config::{clear_client_auth, pull_record_slim, set_client_auth, set_task_base_url},
    extract::{export_to_file, process_text, toggle_line_selection},
    image::{get_app_local_data_dir, read_local_record_image},
    interaction::{
        get_all_exist_records, get_all_op_history, get_local_records_paginator, mark_record_liked,
        mark_record_unliked, mark_record_viewed, query_record_count,
    },
    log::get_log_dir,
    task::{
        launch_auto_scrap_task, launch_batch_scrap_task, launch_idol_scrap_task,
        launch_record_pull_task, launch_submit_task, launch_update_task,
    },
};

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

    #[error("Unknown error occurred: {0}")]
    UnknownError(String),

    #[error("Auth not set: {0}")]
    GetAuthFailed(String),

    #[error("Send request Failed: {0}")]
    SendRequestFailed(String),
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
            tauri_plugin_log::Builder::new()
                // 清除默认目标，使用自定义配置
                .clear_targets()
                // 配置多个目标：debug单独文件，其他级别合并文件
                .targets([
                    // Debug及Trace日志单独输出到debug.log，使用APP_LOCAL_DATA目录
                    // linux: $XDG_DATA_HOME/{bundleIdentifier}/logs` or `$HOME/.local/share/{bundleIdentifier}/logs`
                    // mac: {homeDir}/Library/Logs/{bundleIdentifier}
                    // windows: C:\Users\Alice\AppData\Local\com.tauri.dev\logs
                    Target::new(TargetKind::LogDir {
                        file_name: Some("debug".into()),
                    })
                    .filter(|metadata| matches!(metadata.level(), Level::Debug | Level::Trace)),
                    // 其他级别（Error, Warn, Info）输出到app.log
                    Target::new(TargetKind::LogDir {
                        file_name: Some("app".into()),
                    })
                    .filter(|metadata| {
                        matches!(metadata.level(), Level::Error | Level::Warn | Level::Info)
                    }),
                    // 保留控制台输出用于开发调试
                    Target::new(TargetKind::Stdout),
                    // Webview输出便于前端调试
                    Target::new(TargetKind::Webview),
                ])
                // 设置全局日志级别
                .level({
                    #[cfg(debug_assertions)]
                    {
                        LevelFilter::Debug
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        LevelFilter::Info
                    }
                })
                // 设置文件大小限制为2GB
                .max_file_size(2 * 1024 * 1024 * 1024) // 2GB
                // 轮转策略：保留所有文件
                .rotation_strategy(RotationStrategy::KeepAll)
                // 自定义日志格式
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}] [{}] [{}:{}] {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                        record.level(),
                        record.file().unwrap_or("unknown"),
                        record.line().unwrap_or(0),
                        message
                    ));
                })
                // 使用本地时区
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            log::info!("Starting Tauri application setup");
            log::debug!("Debug logging is enabled - this should go to debug.log");

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
                    log::debug!("App state managed and ready for operations");
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
            get_local_records_paginator,
            query_record_count,
            launch_auto_scrap_task,
            launch_batch_scrap_task,
            get_all_op_history,
            set_task_base_url,
            set_client_auth,
            clear_client_auth,
            pull_record_slim,
            get_app_local_data_dir,
            read_local_record_image,
            mark_record_viewed,
            mark_record_liked,
            mark_record_unliked,
            launch_idol_scrap_task,
            launch_record_pull_task,
            launch_submit_task,
            get_log_dir,
            launch_update_task,
            get_all_exist_records
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
