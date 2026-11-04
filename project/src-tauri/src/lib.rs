//! Tauri 应用入口：注册状态、命令与事件转发。

mod commands;
mod events;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::connection::connection_create,
            commands::connection::connection_snapshot,
            commands::connection::connection_close,
        ])
        .setup(|app| {
            events::spawn_forwarder(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
