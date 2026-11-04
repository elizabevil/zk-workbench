//! 代理相关命令：测试连通性、管理密码。

use tauri::State;
use zkui_core::error::AppResult;
use zkui_core::secrets;
use zkui_core::transport::ProxyProfile;

#[tauri::command]
pub async fn proxy_test(profile: ProxyProfile, target: String) -> AppResult<()> {
    let stream = zkui_core::transport::upstream::connect(&target, Some(&profile)).await?;
    drop(stream);
    Ok(())
}

#[tauri::command]
pub fn proxy_password_set(profile: String, password: String) -> AppResult<()> {
    secrets::set_proxy_password(&profile, &password)
}

#[tauri::command]
pub fn proxy_password_clear(profile: String) -> AppResult<()> {
    secrets::delete_proxy_password(&profile)
}

#[tauri::command]
pub fn proxy_password_has(profile: String) -> AppResult<bool> {
    Ok(secrets::get_proxy_password(&profile)?.is_some())
}

#[tauri::command]
pub fn app_paths(state: State<'_, crate::state::AppState>) -> zkui_core::ipc::app::AppPaths {
    let log_dir = dirs::document_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("zkui");
    zkui_core::ipc::app::AppPaths {
        config_path: state.settings.path().display().to_string(),
        log_dir: log_dir.display().to_string(),
    }
}
