use crate::db::{Database, HistoryItem, PresetRow};
use crate::download::DownloadManager;
use crate::models::{DownloadRequest, RuntimeInfo, UrlInspection, VideoMeta};
use crate::settings;
use crate::updater;
use crate::ytdlp;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn extract_info(url: String, app: AppHandle) -> Result<VideoMeta, String> {
    ytdlp::extract_info(&url, &app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn inspect_url(url: String, app: AppHandle) -> Result<UrlInspection, String> {
    ytdlp::inspect_url(&url, &app)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_download(
    req: DownloadRequest,
    app: AppHandle,
    manager: State<'_, DownloadManager>,
) -> Result<String, String> {
    Ok(manager.start(req, app).await)
}

#[tauri::command]
pub async fn cancel_download(
    id: String,
    app: AppHandle,
    manager: State<'_, DownloadManager>,
) -> Result<(), String> {
    manager.cancel(&id, &app).await;
    Ok(())
}

#[tauri::command]
pub async fn runtime_info(app: AppHandle, db: State<'_, Database>) -> Result<RuntimeInfo, String> {
    let output_dir = db
        .get_setting("output_dir")
        .map_err(|e| e.to_string())?
        .unwrap_or_else(default_output_dir);
    let yt_dlp_version = updater::current_ytdlp_version(&app)
        .await
        .unwrap_or_else(|| "unknown".into());
    Ok(RuntimeInfo {
        yt_dlp_version,
        ffmpeg_version: settings::detect_ffmpeg(),
        aria2c_version: settings::detect_aria2c(),
        cookies_source: db
            .get_setting("cookies_source")
            .map_err(|e| e.to_string())?,
        disk_free_gb: settings::disk_free_gb(&output_dir),
        output_dir,
    })
}

#[tauri::command]
pub fn list_history(
    limit: u32,
    offset: u32,
    db: State<'_, Database>,
) -> Result<Vec<HistoryItem>, String> {
    db.list_history(limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_presets(db: State<'_, Database>) -> Result<Vec<PresetRow>, String> {
    db.list_presets().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn upsert_preset(preset: PresetRow, db: State<'_, Database>) -> Result<(), String> {
    db.upsert_preset(&preset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_preset(id: String, db: State<'_, Database>) -> Result<(), String> {
    db.delete_preset(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_setting(key: String, db: State<'_, Database>) -> Result<Option<String>, String> {
    db.get_setting(&key).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_setting(key: String, value: String, db: State<'_, Database>) -> Result<(), String> {
    db.set_setting(&key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_concurrent_limit(
    limit: usize,
    manager: State<'_, DownloadManager>,
    db: State<'_, Database>,
) -> Result<(), String> {
    manager.set_limit(limit).await;
    db.set_setting("concurrent_limit", &limit.to_string())
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn default_output_dir() -> String {
    match std::env::var("HOME") {
        Ok(h) => format!("{h}/dl/yt"),
        Err(_) => String::from("./downloads"),
    }
}
