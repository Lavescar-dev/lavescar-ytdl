//! Optional clipboard watcher.
//!
//! When the user opts in (settings `clipboard_listening = 1`), a background
//! task polls the clipboard once per second. If the content matches a known
//! video-URL pattern and differs from what we saw last time, the frontend
//! gets a `clipboard:url` event and can offer to pre-fill the URL bar.
//!
//! Polling (rather than an event listener) is deliberate — portable across
//! macOS/Windows/Linux+Wayland+X11 without platform-specific hooks.

use crate::db::Database;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardUrl {
    pub url: String,
    pub source: &'static str,
}

fn detect_source(url: &str) -> Option<&'static str> {
    let u = url.trim();
    if u.len() > 2048 {
        return None;
    }
    if !(u.starts_with("http://") || u.starts_with("https://")) {
        return None;
    }
    let host = u
        .split('/')
        .nth(2)
        .unwrap_or("")
        .trim_start_matches("www.")
        .to_ascii_lowercase();
    let mapping: &[(&str, &str)] = &[
        ("youtube.com", "youtube"),
        ("youtu.be", "youtube"),
        ("music.youtube.com", "youtube"),
        ("twitch.tv", "twitch"),
        ("vimeo.com", "vimeo"),
        ("soundcloud.com", "soundcloud"),
        ("bandcamp.com", "bandcamp"),
        ("dailymotion.com", "dailymotion"),
        ("bilibili.com", "bilibili"),
        ("nicovideo.jp", "niconico"),
    ];
    for (needle, label) in mapping {
        if host.ends_with(needle) {
            return Some(label);
        }
    }
    None
}

/// Kick off the background clipboard watcher.
pub fn spawn_watcher(app: AppHandle) {
    let last: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(1000)).await;

            // Only poll when the user has enabled it.
            let enabled = app
                .try_state::<Database>()
                .and_then(|db| db.get_setting("clipboard_listening").ok().flatten())
                .map(|v| v == "1")
                .unwrap_or(false);
            if !enabled {
                continue;
            }

            let text = match app.clipboard().read_text() {
                Ok(t) => t,
                Err(_) => continue,
            };
            let trimmed = text.trim().to_string();
            let source = match detect_source(&trimmed) {
                Some(s) => s,
                None => continue,
            };

            let mut seen = last.lock().await;
            if seen.as_deref() == Some(trimmed.as_str()) {
                continue;
            }
            *seen = Some(trimmed.clone());
            drop(seen);

            let _ = app.emit(
                "clipboard:url",
                ClipboardUrl {
                    url: trimmed,
                    source,
                },
            );
        }
    });
}

#[cfg(test)]
mod tests {
    use super::detect_source;

    #[test]
    fn youtube_variants_detected() {
        assert_eq!(
            detect_source("https://www.youtube.com/watch?v=abc"),
            Some("youtube")
        );
        assert_eq!(detect_source("https://youtu.be/abc"), Some("youtube"));
        assert_eq!(
            detect_source("https://music.youtube.com/watch?v=abc"),
            Some("youtube")
        );
    }

    #[test]
    fn other_platforms() {
        assert_eq!(
            detect_source("https://www.twitch.tv/somestream"),
            Some("twitch")
        );
        assert_eq!(detect_source("https://vimeo.com/12345"), Some("vimeo"));
        assert_eq!(
            detect_source("https://soundcloud.com/user/track"),
            Some("soundcloud")
        );
    }

    #[test]
    fn non_matching_ignored() {
        assert_eq!(detect_source("https://example.com/foo"), None);
        assert_eq!(detect_source("not a url"), None);
        assert_eq!(detect_source("ftp://youtube.com/x"), None); // non-http
    }

    #[test]
    fn oversized_rejected() {
        let huge = format!("https://youtube.com/?q={}", "a".repeat(3000));
        assert_eq!(detect_source(&huge), None);
    }
}
