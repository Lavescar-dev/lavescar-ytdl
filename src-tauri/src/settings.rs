use crate::db::Database;
use std::process::Command;
use tauri::State;

fn detect_version(bin: &str, arg: &str) -> Option<String> {
    let out = Command::new(bin).arg(arg).output().ok()?;
    if !out.status.success() {
        return None;
    }
    let s = String::from_utf8_lossy(&out.stdout);
    s.lines().next().map(|l| l.trim().to_string())
}

pub fn detect_ffmpeg() -> String {
    detect_version("ffmpeg", "-version")
        .and_then(|l| l.split_whitespace().nth(2).map(|v| v.to_string()))
        .unwrap_or_else(|| "not found".into())
}

pub fn detect_aria2c() -> String {
    detect_version("aria2c", "--version")
        .and_then(|l| l.split_whitespace().nth(2).map(|v| v.to_string()))
        .unwrap_or_else(|| "—".into())
}

pub fn disk_free_gb(path: &str) -> f64 {
    match fs2_space_bytes(path) {
        Some(bytes) => (bytes as f64) / 1_073_741_824.0,
        None => 0.0,
    }
}

#[cfg(unix)]
fn fs2_space_bytes(path: &str) -> Option<u64> {
    use std::ffi::CString;
    use std::mem::MaybeUninit;

    let c_path = CString::new(path).ok()?;
    unsafe {
        let mut statvfs: MaybeUninit<libc::statvfs> = MaybeUninit::uninit();
        if libc::statvfs(c_path.as_ptr(), statvfs.as_mut_ptr()) != 0 {
            return None;
        }
        let s = statvfs.assume_init();
        // Field widths differ per platform (u32 on some BSD/macos, u64 on Linux);
        // the `as u64` normalizes them and lets us multiply safely.
        #[allow(clippy::unnecessary_cast)]
        Some((s.f_bavail as u64).saturating_mul(s.f_frsize as u64))
    }
}

#[cfg(not(unix))]
fn fs2_space_bytes(_path: &str) -> Option<u64> {
    None
}

#[tauri::command]
pub async fn pick_directory(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog().file().pick_folder(move |p| {
        let _ = tx.send(p.map(|f| f.to_string()));
    });
    rx.await.map_err(|e| e.to_string())
}

#[tauri::command]
pub fn detect_tooling(db: State<'_, Database>) -> Result<serde_json::Value, String> {
    let output_dir = db
        .get_setting("output_dir")
        .map_err(|e| e.to_string())?
        .unwrap_or_else(crate::commands::default_output_dir);
    Ok(serde_json::json!({
        "ffmpeg": detect_ffmpeg(),
        "aria2c": detect_aria2c(),
        "diskFreeGb": disk_free_gb(&output_dir)
    }))
}
