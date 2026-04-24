mod cleanup;
mod clipboard;
mod commands;
mod db;
mod download;
mod error;
mod extras;
mod models;
mod settings;
mod updater;
mod ytdlp;

use db::Database;
use download::{DownloadManager, DEFAULT_CONCURRENT_LIMIT};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to resolve app_data_dir");
            std::fs::create_dir_all(&data_dir).ok();
            let db_path = data_dir.join("lavescar.db");
            let db = Database::open(&db_path).expect("failed to open sqlite database");

            // Seed DownloadManager with the persisted concurrent limit (if any).
            let initial_limit = db
                .get_setting("concurrent_limit")
                .ok()
                .flatten()
                .and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(DEFAULT_CONCURRENT_LIMIT);

            app.manage(db);
            app.manage(DownloadManager::new(initial_limit));

            // Fire a one-shot orphan scan after the frontend is ready.
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                cleanup::emit_startup_scan(&handle);
            });

            // Start the clipboard watcher loop; it self-gates on the setting.
            clipboard::spawn_watcher(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::extract_info,
            commands::inspect_url,
            commands::start_download,
            commands::cancel_download,
            commands::runtime_info,
            commands::list_history,
            commands::list_presets,
            commands::upsert_preset,
            commands::delete_preset,
            commands::get_setting,
            commands::set_setting,
            commands::set_concurrent_limit,
            settings::pick_directory,
            settings::detect_tooling,
            extras::open_in_mpv,
            extras::reveal_in_file_manager,
            updater::update_ytdlp,
            cleanup::scan_orphan_parts,
            cleanup::delete_orphan_parts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
