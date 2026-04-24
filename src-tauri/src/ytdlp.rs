use crate::error::YtdlpError;
use crate::models::{FormatOption, PlaylistEntry, PlaylistInfo, SubLang, UrlInspection, VideoMeta};
use crate::updater::resolve_ytdlp_path;
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

fn seconds_to_hms(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

fn human_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("≈ {:.1} GB", b / GB)
    } else if b >= MB {
        format!("≈ {:.0} MB", b / MB)
    } else {
        format!("≈ {:.0} KB", b / KB)
    }
}

pub async fn extract_info(url: &str, app: &AppHandle) -> Result<VideoMeta, YtdlpError> {
    let args = [
        "--dump-json",
        "--no-warnings",
        "--no-playlist",
        "--no-call-home",
        url,
    ];

    let cmd = match resolve_ytdlp_path(app) {
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

    let mut stdout = Vec::new();
    let mut stderr = String::new();
    let mut exit_code: Option<i32> = None;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(bytes) => stdout.extend_from_slice(&bytes),
            CommandEvent::Stderr(bytes) => {
                stderr.push_str(&String::from_utf8_lossy(&bytes));
            }
            CommandEvent::Terminated(payload) => {
                exit_code = payload.code;
                break;
            }
            _ => {}
        }
    }

    if exit_code.unwrap_or(-1) != 0 {
        return Err(YtdlpError::from_stderr(&stderr));
    }

    let v: Value = serde_json::from_slice(&stdout)?;
    Ok(to_video_meta(url, &v))
}

fn to_video_meta(url: &str, v: &Value) -> VideoMeta {
    let title = v
        .get("title")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let uploader = v
        .get("uploader")
        .or_else(|| v.get("channel"))
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let duration_secs = v.get("duration").and_then(|x| x.as_u64()).unwrap_or(0);
    let duration = if duration_secs > 0 {
        seconds_to_hms(duration_secs)
    } else {
        String::from("—")
    };

    let (best_video, best_audio, size_estimate) = summarize_formats(v);

    let subtitles = summarize_subtitles(v);
    let chapters = summarize_chapters(v);
    let thumbnail_url = v
        .get("thumbnail")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string());
    let available_subs = collect_sub_langs(v);

    VideoMeta {
        url: url.to_string(),
        title,
        uploader,
        duration,
        best_video,
        best_audio,
        size_estimate,
        subtitles,
        chapters,
        thumbnail_url,
        available_subs,
    }
}

fn collect_sub_langs(v: &Value) -> Vec<SubLang> {
    let mut out = Vec::new();
    if let Some(manual) = v.get("subtitles").and_then(|x| x.as_object()) {
        for k in manual.keys() {
            out.push(SubLang {
                code: k.clone(),
                auto: false,
            });
        }
    }
    if let Some(auto) = v.get("automatic_captions").and_then(|x| x.as_object()) {
        for k in auto.keys() {
            out.push(SubLang {
                code: k.clone(),
                auto: true,
            });
        }
    }
    out
}

fn summarize_formats(v: &Value) -> (String, String, String) {
    let formats = v.get("formats").and_then(|x| x.as_array());
    let mut best_v: Option<(&Value, u64, u64)> = None; // (fmt, height, tbr)
    let mut best_a: Option<(&Value, u64)> = None;

    if let Some(arr) = formats {
        for f in arr {
            let vcodec = f.get("vcodec").and_then(|x| x.as_str()).unwrap_or("none");
            let acodec = f.get("acodec").and_then(|x| x.as_str()).unwrap_or("none");
            let height = f.get("height").and_then(|x| x.as_u64()).unwrap_or(0);
            let tbr = f.get("tbr").and_then(|x| x.as_f64()).unwrap_or(0.0) as u64;
            let abr = f.get("abr").and_then(|x| x.as_f64()).unwrap_or(0.0) as u64;

            if vcodec != "none"
                && acodec == "none"
                && best_v
                    .map(|(_, h, t)| (height, tbr) > (h, t))
                    .unwrap_or(true)
            {
                best_v = Some((f, height, tbr));
            } else if acodec != "none"
                && vcodec == "none"
                && best_a.map(|(_, b)| abr > b).unwrap_or(true)
            {
                best_a = Some((f, abr));
            }
        }
    }

    let best_video = best_v
        .map(|(f, h, t)| {
            let vcodec = f.get("vcodec").and_then(|x| x.as_str()).unwrap_or("?");
            let fps = f.get("fps").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let fps_part = if fps > 30.0 {
                format!("{h}p{:.0}", fps)
            } else if h > 0 {
                format!("{h}p")
            } else {
                "?".into()
            };
            format!("{vcodec} · {fps_part} · {:.1} Mbps", t as f64 / 1000.0)
        })
        .unwrap_or_else(|| String::from("—"));

    let best_audio = best_a
        .map(|(f, b)| {
            let acodec = f.get("acodec").and_then(|x| x.as_str()).unwrap_or("?");
            let asr = f.get("asr").and_then(|x| x.as_u64()).unwrap_or(0);
            format!(
                "{acodec} · {b} kbps{}",
                if asr > 0 {
                    format!(" · {} kHz", asr / 1000)
                } else {
                    String::new()
                }
            )
        })
        .unwrap_or_else(|| String::from("—"));

    let size_bytes = best_v
        .and_then(|(f, _, _)| f.get("filesize_approx").and_then(|x| x.as_u64()))
        .or_else(|| best_v.and_then(|(f, _, _)| f.get("filesize").and_then(|x| x.as_u64())))
        .unwrap_or(0);
    let size_estimate = if size_bytes > 0 {
        human_size(size_bytes)
    } else {
        String::from("—")
    };

    (best_video, best_audio, size_estimate)
}

fn summarize_subtitles(v: &Value) -> String {
    let mut parts: Vec<String> = Vec::new();
    if let Some(subs) = v.get("subtitles").and_then(|x| x.as_object()) {
        for k in subs.keys() {
            parts.push(format!("{k} (manual)"));
        }
    }
    if let Some(auto) = v.get("automatic_captions").and_then(|x| x.as_object()) {
        let mut auto_langs: Vec<String> = auto.keys().map(|k| format!("{k} (auto)")).collect();
        auto_langs.truncate(3);
        parts.extend(auto_langs);
    }
    if parts.is_empty() {
        String::from("none")
    } else {
        parts.join(", ")
    }
}

fn summarize_chapters(v: &Value) -> String {
    match v.get("chapters").and_then(|x| x.as_array()) {
        Some(arr) if !arr.is_empty() => format!("{} chapters detected", arr.len()),
        _ => String::from("—"),
    }
}

pub async fn inspect_url(url: &str, app: &AppHandle) -> Result<UrlInspection, YtdlpError> {
    let args = [
        "--flat-playlist",
        "--dump-single-json",
        "--no-warnings",
        "--no-call-home",
        url,
    ];

    let cmd = match resolve_ytdlp_path(app) {
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

    let mut stdout = Vec::new();
    let mut stderr = String::new();
    let mut exit_code: Option<i32> = None;

    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(bytes) => stdout.extend_from_slice(&bytes),
            CommandEvent::Stderr(bytes) => stderr.push_str(&String::from_utf8_lossy(&bytes)),
            CommandEvent::Terminated(payload) => {
                exit_code = payload.code;
                break;
            }
            _ => {}
        }
    }

    if exit_code.unwrap_or(-1) != 0 {
        return Err(YtdlpError::from_stderr(&stderr));
    }

    let v: Value = serde_json::from_slice(&stdout)?;
    if v.get("_type").and_then(|x| x.as_str()) == Some("playlist") {
        Ok(UrlInspection::Playlist(to_playlist(&v)))
    } else {
        Ok(UrlInspection::Single)
    }
}

fn to_playlist(v: &Value) -> PlaylistInfo {
    let title = v
        .get("title")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let uploader = v
        .get("uploader")
        .or_else(|| v.get("channel"))
        .and_then(|x| x.as_str())
        .map(|s| s.to_string());
    let entries = v
        .get("entries")
        .and_then(|x| x.as_array())
        .map(|arr| arr.iter().filter_map(to_entry).collect())
        .unwrap_or_default();
    PlaylistInfo {
        title,
        uploader,
        entries,
    }
}

fn to_entry(v: &Value) -> Option<PlaylistEntry> {
    let id = v.get("id").and_then(|x| x.as_str())?.to_string();
    let title = v
        .get("title")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    // Playlist entries may not include a direct URL — reconstruct from ie + id when missing.
    let url = v
        .get("url")
        .or_else(|| v.get("webpage_url"))
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("https://www.youtube.com/watch?v={id}"));
    let duration = v
        .get("duration")
        .and_then(|x| x.as_u64())
        .filter(|d| *d > 0)
        .map(seconds_to_hms);
    let uploader = v
        .get("uploader")
        .or_else(|| v.get("channel"))
        .and_then(|x| x.as_str())
        .map(|s| s.to_string());
    Some(PlaylistEntry {
        id,
        url,
        title,
        duration,
        uploader,
    })
}

#[allow(dead_code)]
pub fn derive_format_options(v: &Value) -> Vec<FormatOption> {
    // Faz 2'de format chip'leri için kullanılacak; şimdilik iskelet.
    let mut out = Vec::new();
    if let Some(arr) = v.get("formats").and_then(|x| x.as_array()) {
        for f in arr.iter().rev().take(6) {
            let id = f
                .get("format_id")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            let label = f
                .get("format_note")
                .or_else(|| f.get("format"))
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            if !id.is_empty() {
                out.push(FormatOption {
                    id: id.clone(),
                    label,
                    spec: id,
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn hms_formatting() {
        assert_eq!(seconds_to_hms(5), "0:05");
        assert_eq!(seconds_to_hms(65), "1:05");
        assert_eq!(seconds_to_hms(3661), "1:01:01");
    }

    #[test]
    fn human_size_thresholds() {
        assert_eq!(human_size(500), "≈ 0 KB");
        assert_eq!(human_size(2_500_000), "≈ 2 MB");
        assert_eq!(human_size(2_500_000_000), "≈ 2.3 GB");
    }

    #[test]
    fn format_summary_picks_best() {
        let v = json!({
            "formats": [
                { "vcodec": "av01.0.08M.08", "acodec": "none", "height": 1080, "tbr": 4200.0, "fps": 60.0 },
                { "vcodec": "avc1", "acodec": "none", "height": 720, "tbr": 1500.0 },
                { "vcodec": "none", "acodec": "opus", "abr": 160.0, "asr": 48000 },
                { "vcodec": "none", "acodec": "mp4a", "abr": 128.0, "asr": 44100 }
            ]
        });
        let (bv, ba, _) = summarize_formats(&v);
        assert!(bv.contains("av01"));
        assert!(bv.contains("1080p60"));
        assert!(ba.contains("opus"));
        assert!(ba.contains("160"));
    }

    #[test]
    fn subtitle_langs_merged() {
        let v = json!({
            "subtitles": { "en": [], "tr": [] },
            "automatic_captions": { "de": [], "en": [] }
        });
        let subs = collect_sub_langs(&v);
        // manual en + manual tr + auto de + auto en
        assert_eq!(subs.len(), 4);
        assert!(subs.iter().any(|s| s.code == "en" && !s.auto));
        assert!(subs.iter().any(|s| s.code == "de" && s.auto));
    }

    #[test]
    fn playlist_entries_parsed() {
        let v = json!({
            "_type": "playlist",
            "title": "Great Talks",
            "entries": [
                { "id": "aaa", "title": "Talk 1", "duration": 1200 },
                { "id": "bbb", "title": "Talk 2", "url": "https://youtu.be/bbb" }
            ]
        });
        let p = to_playlist(&v);
        assert_eq!(p.title, "Great Talks");
        assert_eq!(p.entries.len(), 2);
        assert_eq!(p.entries[0].id, "aaa");
        assert!(p.entries[0].duration.is_some());
        assert!(p.entries[1].url.contains("youtu.be"));
    }
}
