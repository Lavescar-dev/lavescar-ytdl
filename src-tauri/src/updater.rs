//! yt-dlp self-update.
//!
//! Downloads the latest release into `app_data_dir/binaries/` and points future
//! invocations at the user copy via [`resolve_ytdlp_path`]. The bundled sidecar
//! is left untouched (read-only in installed bundles, and we want a clean
//! rollback path if an update turns out to be broken).

use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager};

const GITHUB_LATEST: &str = "https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest";
const USER_AGENT: &str = concat!("lavescar-ytdl/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateResult {
    pub new_version: String,
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProgress {
    phase: &'static str,
    bytes: u64,
    total: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct GhRelease {
    tag_name: String,
    assets: Vec<GhAsset>,
}

#[derive(Debug, Deserialize)]
struct GhAsset {
    name: String,
    browser_download_url: String,
    #[serde(default)]
    size: u64,
}

/// Returns the filename we expect inside `src-tauri/binaries/` for this
/// platform (tauri sidecar convention).
pub fn platform_binary_name() -> &'static str {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        "yt-dlp-x86_64-unknown-linux-gnu"
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        "yt-dlp-aarch64-unknown-linux-gnu"
    }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        "yt-dlp-x86_64-pc-windows-msvc.exe"
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        "yt-dlp-x86_64-apple-darwin"
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        "yt-dlp-aarch64-apple-darwin"
    }
}

/// GitHub release asset name for this platform.
fn github_asset_name() -> &'static str {
    #[cfg(target_os = "linux")]
    {
        "yt-dlp_linux"
    }
    #[cfg(target_os = "windows")]
    {
        "yt-dlp.exe"
    }
    #[cfg(target_os = "macos")]
    {
        "yt-dlp_macos"
    }
}

/// If the user has installed a newer yt-dlp via `update_ytdlp`, returns its
/// path. Otherwise returns `None` — caller should fall back to the bundled
/// sidecar via `shell().sidecar("yt-dlp")`.
pub fn resolve_ytdlp_path(app: &AppHandle) -> Option<PathBuf> {
    let data_dir = app.path().app_data_dir().ok()?;
    let p = data_dir.join("binaries").join(platform_binary_name());
    if p.exists() {
        Some(p)
    } else {
        None
    }
}

#[tauri::command]
pub async fn update_ytdlp(app: AppHandle) -> Result<UpdateResult, String> {
    run_update(&app).await.map_err(|e| e.to_string())
}

async fn run_update(
    app: &AppHandle,
) -> Result<UpdateResult, Box<dyn std::error::Error + Send + Sync>> {
    let _ = app.emit(
        "ytdlp:update:progress",
        UpdateProgress {
            phase: "resolving",
            bytes: 0,
            total: None,
        },
    );

    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

    let release: GhRelease = client
        .get(GITHUB_LATEST)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let wanted = github_asset_name();
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == wanted)
        .ok_or_else(|| format!("release {} missing asset {wanted}", release.tag_name))?;

    let data_dir = app.path().app_data_dir()?;
    let bin_dir = data_dir.join("binaries");
    std::fs::create_dir_all(&bin_dir)?;
    let target = bin_dir.join(platform_binary_name());
    let staging = bin_dir.join(format!("{}.new", platform_binary_name()));

    let _ = app.emit(
        "ytdlp:update:progress",
        UpdateProgress {
            phase: "downloading",
            bytes: 0,
            total: if asset.size > 0 {
                Some(asset.size)
            } else {
                None
            },
        },
    );

    // Stream download with running SHA-256 + progress events.
    let resp = client
        .get(&asset.browser_download_url)
        .send()
        .await?
        .error_for_status()?;
    let total = resp.content_length().or(if asset.size > 0 {
        Some(asset.size)
    } else {
        None
    });
    let mut stream = resp.bytes_stream();
    let mut hasher = Sha256::new();
    let mut downloaded: u64 = 0;
    let mut file = std::fs::File::create(&staging)?;
    let mut last_emit_bytes: u64 = 0;

    use std::io::Write;
    while let Some(chunk) = stream.next().await {
        let bytes = chunk?;
        hasher.update(&bytes);
        file.write_all(&bytes)?;
        downloaded += bytes.len() as u64;
        if downloaded - last_emit_bytes > 256 * 1024 {
            last_emit_bytes = downloaded;
            let _ = app.emit(
                "ytdlp:update:progress",
                UpdateProgress {
                    phase: "downloading",
                    bytes: downloaded,
                    total,
                },
            );
        }
    }
    file.sync_all()?;
    drop(file);

    let digest = hasher.finalize();
    let sha256 = format!("{:x}", digest);

    let _ = app.emit(
        "ytdlp:update:progress",
        UpdateProgress {
            phase: "installing",
            bytes: downloaded,
            total,
        },
    );

    // Atomic swap; on Unix also mark executable.
    if target.exists() {
        let _ = std::fs::remove_file(&target);
    }
    std::fs::rename(&staging, &target)?;
    set_executable(&target)?;

    let _ = app.emit(
        "ytdlp:update:progress",
        UpdateProgress {
            phase: "done",
            bytes: downloaded,
            total,
        },
    );

    Ok(UpdateResult {
        new_version: release.tag_name,
        path: target.to_string_lossy().into_owned(),
        bytes: downloaded,
        sha256,
    })
}

#[cfg(unix)]
fn set_executable(p: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(p)?.permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(p, perms)
}

#[cfg(not(unix))]
fn set_executable(_p: &Path) -> std::io::Result<()> {
    Ok(())
}

/// Query the current yt-dlp binary (user copy if present, else bundled sidecar)
/// and return its `--version` output.
pub async fn current_ytdlp_version(app: &AppHandle) -> Option<String> {
    use tauri_plugin_shell::ShellExt;
    let shell = app.shell();
    let output = match resolve_ytdlp_path(app) {
        Some(path) => shell
            .command(path)
            .args(["--version"])
            .output()
            .await
            .ok()?,
        None => shell
            .sidecar("yt-dlp")
            .ok()?
            .args(["--version"])
            .output()
            .await
            .ok()?,
    };
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
