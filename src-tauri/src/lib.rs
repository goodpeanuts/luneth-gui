mod db;
mod extract;
mod luneth;
mod scrap;

use std::sync::Arc;

// Import all items from the extract module, including process_text
use ::luneth::crawl::CrawlError;
use extract::{export_to_file, process_text, toggle_line_selection};
use luneth::{
    get_all_op_history, get_all_records, launch_auto_scrap_task, launch_manual_scrap_task,
    set_crawl_base_url,
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
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            process_text,
            toggle_line_selection,
            export_to_file,
            get_all_records,
            launch_auto_scrap_task,
            launch_manual_scrap_task,
            get_all_op_history,
            set_crawl_base_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
