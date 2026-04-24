//! Orphan `.part` file housekeeping.
//!
//! yt-dlp leaves `<title>.<ext>.part` on cancel/crash/network drop. These
//! files are safe to delete once the associated download is no longer active,
//! but we never touch them automatically: users may be resuming manually or
//! want to keep partial evidence for debugging.

use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrphanPart {
    pub path: String,
    pub size_bytes: u64,
    pub modified_ms: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrphanScan {
    pub dir: String,
    pub items: Vec<OrphanPart>,
}

fn output_dir(app: &AppHandle) -> PathBuf {
    use crate::db::Database;
    let custom = app
        .try_state::<Database>()
        .and_then(|db| db.get_setting("output_dir").ok().flatten());
    let raw = custom.unwrap_or_else(crate::commands::default_output_dir);
    expand_tilde(&raw)
}

fn expand_tilde(p: &str) -> PathBuf {
    if let Some(stripped) = p.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            let mut pb = PathBuf::from(home);
            pb.push(stripped);
            return pb;
        }
    }
    PathBuf::from(p)
}

fn collect(dir: &PathBuf) -> Vec<OrphanPart> {
    let mut out = Vec::new();
    if let Ok(rd) = std::fs::read_dir(dir) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("part") {
                continue;
            }
            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            let modified_ms = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as i64)
                .unwrap_or(0);
            out.push(OrphanPart {
                path: p.to_string_lossy().into_owned(),
                size_bytes: meta.len(),
                modified_ms,
            });
        }
    }
    out
}

#[tauri::command]
pub fn scan_orphan_parts(app: AppHandle) -> Result<OrphanScan, String> {
    let dir = output_dir(&app);
    let items = collect(&dir);
    Ok(OrphanScan {
        dir: dir.to_string_lossy().into_owned(),
        items,
    })
}

#[tauri::command]
pub fn delete_orphan_parts(paths: Vec<String>) -> Result<usize, String> {
    let mut removed = 0usize;
    for p in paths {
        let path = PathBuf::from(&p);
        // Safety: only touch `.part` files to avoid wiping real downloads if
        // the frontend ever sends the wrong list.
        if path.extension().and_then(|s| s.to_str()) != Some("part") {
            continue;
        }
        if std::fs::remove_file(&path).is_ok() {
            removed += 1;
        }
    }
    Ok(removed)
}

/// Emits an `orphans:found` event at startup so the UI can prompt the user.
pub fn emit_startup_scan(app: &AppHandle) {
    let dir = output_dir(app);
    let items = collect(&dir);
    if !items.is_empty() {
        let _ = app.emit(
            "orphans:found",
            OrphanScan {
                dir: dir.to_string_lossy().into_owned(),
                items,
            },
        );
    }
}
