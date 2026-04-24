use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubLang {
    pub code: String,
    pub auto: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMeta {
    pub url: String,
    pub title: String,
    pub uploader: String,
    pub duration: String,
    pub best_video: String,
    pub best_audio: String,
    pub size_estimate: String,
    pub subtitles: String,
    pub chapters: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    #[serde(default)]
    pub available_subs: Vec<SubLang>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatOption {
    pub id: String,
    pub label: String,
    pub spec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubtitleOpts {
    pub langs: Vec<String>,
    pub auto: bool,
    pub embed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRequest {
    pub url: String,
    pub format_spec: String,
    pub preset_id: String,
    pub output_dir: String,
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(default)]
    pub subtitle_opts: Option<SubtitleOpts>,
    /// Optional display title from the frontend (already fetched via
    /// `extract_info`). Saves the DB a placeholder like `(url) …` row.
    #[serde(default)]
    pub title: Option<String>,
    /// Optional codec/format label from the UI (e.g. `av1+opus`).
    #[serde(default)]
    pub codec: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub id: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub speed_bytes_per_sec: f64,
    pub eta_seconds: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistEntry {
    pub id: String,
    pub url: String,
    pub title: String,
    pub duration: Option<String>,
    pub uploader: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistInfo {
    pub title: String,
    pub uploader: Option<String>,
    pub entries: Vec<PlaylistEntry>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", content = "data", rename_all = "camelCase")]
pub enum UrlInspection {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "playlist")]
    Playlist(PlaylistInfo),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
    pub yt_dlp_version: String,
    pub ffmpeg_version: String,
    pub aria2c_version: String,
    pub cookies_source: Option<String>,
    pub disk_free_gb: f64,
    pub output_dir: String,
}
