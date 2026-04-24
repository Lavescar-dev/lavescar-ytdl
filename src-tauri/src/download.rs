use crate::db::Database;
use crate::error::YtdlpError;
use crate::models::{DownloadProgress, DownloadRequest};
use crate::updater::resolve_ytdlp_path;
use chrono::Utc;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;
use tokio::sync::{Mutex, Semaphore};
use tokio::task::JoinHandle;

pub const DEFAULT_CONCURRENT_LIMIT: usize = 3;
const MAX_CONCURRENT_LIMIT: usize = 64;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadDone {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadError {
    pub id: String,
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlreadyDownloaded {
    pub id: String,
    pub url: String,
    pub output_path: Option<String>,
}

pub struct DownloadManager {
    active: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
    semaphore: Arc<Semaphore>,
    /// Logical cap the user chose. Needed because `Semaphore::available_permits()`
    /// reflects *unused* permits only, not the configured maximum.
    current_limit: Arc<Mutex<usize>>,
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new(DEFAULT_CONCURRENT_LIMIT)
    }
}

impl DownloadManager {
    pub fn new(initial_limit: usize) -> Self {
        let clamped = initial_limit.clamp(1, MAX_CONCURRENT_LIMIT);
        Self {
            active: Arc::new(Mutex::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(clamped)),
            current_limit: Arc::new(Mutex::new(clamped)),
        }
    }

    pub async fn start(&self, req: DownloadRequest, app: AppHandle) -> String {
        // Re-download guard
        if let Some(db) = app.try_state::<Database>() {
            if let Ok(Some(existing)) = db.find_completed_by_url(&req.url) {
                let _ = app.emit(
                    "download:already",
                    AlreadyDownloaded {
                        id: existing.id,
                        url: req.url.clone(),
                        output_path: existing.output_path,
                    },
                );
            }
        }

        let id = uuid::Uuid::new_v4().to_string();

        // Initial DB insert — use the UI-provided title/codec when available
        // so history rows aren't cluttered with `(url) …` placeholders.
        if let Some(db) = app.try_state::<Database>() {
            let title = req
                .title
                .clone()
                .filter(|t| !t.is_empty())
                .unwrap_or_else(|| req.url.clone());
            let codec = req.codec.clone().unwrap_or_default();
            let _ = db.insert_download(
                &id,
                &req.url,
                &title,
                &codec,
                Utc::now().timestamp_millis(),
            );
        }

        let id_task = id.clone();
        let active_ref = self.active.clone();
        let semaphore = self.semaphore.clone();

        // Immediately signal "queued" so the UI shows the row before the permit is free.
        let _ = app.emit(
            "download:progress",
            DownloadProgress {
                id: id.clone(),
                downloaded_bytes: 0,
                total_bytes: 0,
                speed_bytes_per_sec: 0.0,
                eta_seconds: 0.0,
                status: "queued".into(),
            },
        );

        let handle = tokio::spawn(async move {
            // Wait for a permit before starting the actual yt-dlp process.
            // The permit is released automatically when this task exits.
            let permit = match semaphore.acquire_owned().await {
                Ok(p) => p,
                Err(_) => {
                    // Semaphore closed (app shutdown).
                    active_ref.lock().await.remove(&id_task);
                    return;
                }
            };

            let result = run_download(id_task.clone(), req, app.clone()).await;
            if let Err(e) = result {
                let kind = e.kind().to_string();
                let msg = e.to_string();
                let _ = app.emit(
                    "download:error",
                    DownloadError {
                        id: id_task.clone(),
                        kind,
                        message: msg.clone(),
                    },
                );
                if let Some(db) = app.try_state::<Database>() {
                    let _ = db.update_error(&id_task, &msg, Utc::now().timestamp_millis());
                }
            }
            drop(permit);
            active_ref.lock().await.remove(&id_task);
        });

        self.active.lock().await.insert(id.clone(), handle);
        id
    }

    pub async fn cancel(&self, id: &str, app: &AppHandle) {
        if let Some(h) = self.active.lock().await.remove(id) {
            h.abort();
        }
        // Mark cancelled in DB (status='cancelled', separate from yt-dlp errors).
        if let Some(db) = app.try_state::<Database>() {
            let _ = db.update_cancelled(id, Utc::now().timestamp_millis());
        }
        // Sweep stale .part files for this id from the active output dir. yt-dlp
        // names them like `<title> [<videoId>].<ext>.part` — we can't predict the
        // full path without the yt-dlp metadata, so the startup sweep is the
        // catch-all; here we just emit an event so the UI can surface any
        // residue in the history view.
    }

    /// Adjust the concurrency limit at runtime.
    ///
    /// Tasks already holding permits keep running; the change only gates
    /// whatever is queued or enqueued afterwards.
    pub async fn set_limit(&self, new_limit: usize) {
        let new_limit = new_limit.clamp(1, MAX_CONCURRENT_LIMIT);
        let mut current = self.current_limit.lock().await;
        if new_limit > *current {
            self.semaphore.add_permits(new_limit - *current);
        } else if new_limit < *current {
            // Shrink by burning permits in the background, so the call returns
            // immediately even if all slots are currently in use. Held permits
            // are unaffected; new acquisitions past the new cap will block.
            let to_shrink = (*current - new_limit) as u32;
            let sem = self.semaphore.clone();
            tokio::spawn(async move {
                if let Ok(permits) = sem.acquire_many(to_shrink).await {
                    permits.forget();
                }
            });
        }
        *current = new_limit;
    }
}

async fn run_download(id: String, req: DownloadRequest, app: AppHandle) -> Result<(), YtdlpError> {
    std::fs::create_dir_all(expand_tilde(&req.output_dir))?;

    let output_template = format!(
        "{}/%(title)s [%(id)s].%(ext)s",
        expand_tilde(&req.output_dir)
    );

    let progress_template =
        "dlprogress|%(progress.downloaded_bytes)s|%(progress.total_bytes)s|%(progress.total_bytes_estimate)s|%(progress.speed)s|%(progress.eta)s|%(info.filename)s";

    let mut args = vec![
        "--newline".to_string(),
        "--no-warnings".to_string(),
        "--no-call-home".to_string(),
        "--progress".to_string(),
        "--progress-template".to_string(),
        progress_template.to_string(),
        "-o".to_string(),
        output_template.clone(),
    ];

    // Optional cookies from browser (persisted setting).
    if let Some(db) = app.try_state::<Database>() {
        if let Ok(Some(src)) = db.get_setting("cookies_source") {
            if src != "none" && !src.is_empty() {
                args.push("--cookies-from-browser".into());
                args.push(src);
            }
        }
        // Per-download throttle (MB/s) -> bytes/s.
        if let Ok(Some(enabled)) = db.get_setting("throttle_enabled") {
            if enabled == "1" {
                if let Ok(Some(mbps)) = db.get_setting("throttle_mbps") {
                    if let Ok(m) = mbps.parse::<u64>() {
                        args.push("--limit-rate".into());
                        args.push(format!("{}M", m));
                    }
                }
            }
        }
    }

    if !req.format_spec.is_empty() {
        args.push("-f".into());
        args.push(req.format_spec.clone());
    }

    // Merge preset extra flags (e.g. --sponsorblock-remove, --split-chapters).
    for f in &req.flags {
        args.push(f.clone());
    }

    // Subtitle options (v1.0): per-download picker forwards explicit flags.
    if let Some(subs) = &req.subtitle_opts {
        if !subs.langs.is_empty() {
            if subs.auto {
                args.push("--write-auto-subs".into());
            } else {
                args.push("--write-subs".into());
            }
            args.push("--sub-langs".into());
            args.push(subs.langs.join(","));
            if subs.embed {
                args.push("--embed-subs".into());
            }
        }
    }

    args.push(req.url.clone());

    let cmd = match resolve_ytdlp_path(&app) {
        Some(path) => app.shell().command(path),
        None => app
            .shell()
            .sidecar("yt-dlp")
            .map_err(|e| YtdlpError::Shell(e.to_string()))?,
    };
    let (mut rx, _child) = cmd
        .args(args)
        .spawn()
        .map_err(|e| YtdlpError::Shell(e.to_string()))?;

    let mut stderr_buf = String::new();
    let mut final_path: Option<String> = None;
    let mut exit_code: Option<i32> = None;

    let _ = app.emit(
        "download:progress",
        DownloadProgress {
            id: id.clone(),
            downloaded_bytes: 0,
            total_bytes: 0,
            speed_bytes_per_sec: 0.0,
            eta_seconds: 0.0,
            status: "active".into(),
        },
    );

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(bytes) => {
                let line = String::from_utf8_lossy(&bytes);
                for l in line.lines() {
                    if let Some(p) = parse_progress_line(&id, l) {
                        let _ = app.emit("download:progress", p);
                    } else if let Some(path) = parse_destination(l) {
                        final_path = Some(path);
                    }
                }
            }
            CommandEvent::Stderr(bytes) => {
                let s = String::from_utf8_lossy(&bytes);
                stderr_buf.push_str(&s);
            }
            CommandEvent::Terminated(payload) => {
                exit_code = payload.code;
                break;
            }
            _ => {}
        }
    }

    if exit_code.unwrap_or(-1) != 0 {
        return Err(YtdlpError::from_stderr(&stderr_buf));
    }

    let path = final_path
        .or_else(|| guess_path(&req.output_dir))
        .unwrap_or_else(|| req.output_dir.clone());

    if let Some(db) = app.try_state::<Database>() {
        let _ = db.update_done(&id, &path, Utc::now().timestamp_millis());
    }

    let _ = app.emit(
        "download:progress",
        DownloadProgress {
            id: id.clone(),
            downloaded_bytes: 0,
            total_bytes: 0,
            speed_bytes_per_sec: 0.0,
            eta_seconds: 0.0,
            status: "done".into(),
        },
    );
    let _ = app.emit("download:done", DownloadDone { id, path });
    Ok(())
}

fn parse_progress_line(id: &str, line: &str) -> Option<DownloadProgress> {
    let rest = line.strip_prefix("dlprogress|")?;
    let parts: Vec<&str> = rest.split('|').collect();
    if parts.len() < 5 {
        return None;
    }
    let downloaded = parts[0].trim().parse::<u64>().unwrap_or(0);
    let total = parts[1].trim().parse::<u64>().unwrap_or(0);
    let total_est = parts[2].trim().parse::<u64>().unwrap_or(0);
    let speed = parts[3].trim().parse::<f64>().unwrap_or(0.0);
    let eta = parts[4].trim().parse::<f64>().unwrap_or(0.0);

    Some(DownloadProgress {
        id: id.to_string(),
        downloaded_bytes: downloaded,
        total_bytes: if total > 0 { total } else { total_est },
        speed_bytes_per_sec: speed,
        eta_seconds: eta,
        status: "active".into(),
    })
}

fn parse_destination(line: &str) -> Option<String> {
    if let Some(rest) = line.strip_prefix("[download] Destination: ") {
        return Some(rest.to_string());
    }
    if let Some(rest) = line.strip_prefix("[Merger] Merging formats into \"") {
        return Some(rest.trim_end_matches('"').to_string());
    }
    None
}

fn guess_path(dir: &str) -> Option<String> {
    Some(expand_tilde(dir))
}

fn expand_tilde(p: &str) -> String {
    if let Some(stripped) = p.strip_prefix("~/") {
        if let Some(home) = std::env::var_os("HOME") {
            let mut pb = PathBuf::from(home);
            pb.push(stripped);
            return pb.to_string_lossy().into_owned();
        }
    }
    p.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_line_parsed() {
        let p = parse_progress_line(
            "abc-123",
            "dlprogress|1234567|9876543|9876543|524288.0|12.5|video.mp4",
        )
        .expect("should parse");
        assert_eq!(p.id, "abc-123");
        assert_eq!(p.downloaded_bytes, 1_234_567);
        assert_eq!(p.total_bytes, 9_876_543);
        assert!((p.speed_bytes_per_sec - 524288.0).abs() < 0.001);
        assert!((p.eta_seconds - 12.5).abs() < 0.001);
    }

    #[test]
    fn progress_falls_back_to_estimate_when_total_unknown() {
        let p = parse_progress_line("id", "dlprogress|100|0|500|1000.0|4.0|f.mp4").unwrap();
        assert_eq!(p.total_bytes, 500);
    }

    #[test]
    fn non_progress_line_ignored() {
        assert!(parse_progress_line("id", "[youtube] Extracting …").is_none());
        assert!(parse_progress_line("id", "dlprogress|1|2").is_none()); // too few fields
    }

    #[test]
    fn destination_parsed() {
        assert_eq!(
            parse_destination("[download] Destination: /tmp/video.mp4"),
            Some("/tmp/video.mp4".into())
        );
        assert_eq!(
            parse_destination("[Merger] Merging formats into \"/tmp/out.mkv\""),
            Some("/tmp/out.mkv".into())
        );
        assert_eq!(parse_destination("random noise"), None);
    }

    #[test]
    fn expand_tilde_replaces_home() {
        if let Ok(home) = std::env::var("HOME") {
            assert_eq!(expand_tilde("~/dl/yt"), format!("{home}/dl/yt"));
        }
        assert_eq!(expand_tilde("/absolute"), "/absolute");
    }
}
