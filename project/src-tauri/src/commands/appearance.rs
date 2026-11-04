//! 背景图片存取命令。

use base64::Engine;
use tauri::State;
use zkui_core::error::{AppResult, app};

use crate::state::AppState;

fn image_dir() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("zkui")
}

/// 保存背景图片，返回本地路径。
#[tauri::command]
pub fn background_image_store(
    _state: State<'_, AppState>,
    data_b64: String,
    extension: String,
) -> AppResult<String> {
    let extension = match extension.to_ascii_lowercase().as_str() {
        "png" => "png",
        "jpg" | "jpeg" => "jpg",
        "webp" => "webp",
        "gif" => "gif",
        _ => {
            return Err(Box::new(app::invalid_argument(
                "unsupported image extension",
            )));
        }
    };
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data_b64.as_bytes())
        .map_err(|_| Box::new(app::invalid_argument("invalid image data")))?;
    if bytes.len() > 10 * 1024 * 1024 {
        return Err(Box::new(app::invalid_argument("image is larger than 10MB")));
    }
    let directory = image_dir();
    std::fs::create_dir_all(&directory).map_err(|error| {
        Box::new(app::audit_failed(&format!(
            "failed to create image dir: {error}"
        )))
    })?;
    let path = directory.join(format!("background.{extension}"));
    std::fs::write(&path, bytes).map_err(|error| {
        Box::new(app::audit_failed(&format!(
            "failed to write image: {error}"
        )))
    })?;
    Ok(path.to_string_lossy().to_string())
}

/// 读取背景图片并返回 data URL。
#[tauri::command]
pub fn background_image_load(path: String) -> AppResult<String> {
    let bytes = std::fs::read(&path)
        .map_err(|error| Box::new(app::audit_failed(&format!("failed to read image: {error}"))))?;
    if bytes.len() > 10 * 1024 * 1024 {
        return Err(Box::new(app::invalid_argument("image is larger than 10MB")));
    }
    let extension = std::path::Path::new(&path)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("png")
        .to_ascii_lowercase();
    let mime = match extension.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "image/png",
    };
    Ok(format!(
        "data:{mime};base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}
