//! 应用元信息命令。

use tauri::AppHandle;
use zkui_core::ipc::app::AppInfo;

#[tauri::command]
pub fn app_info(app: AppHandle) -> AppInfo {
    let info = app.package_info();
    AppInfo {
        name: info.name.clone(),
        version: info.version.to_string(),
    }
}
