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
            commands::connection::connection_list,
            commands::node::node_children,
            commands::node::node_stat,
            commands::node::node_data,
            commands::node::node_set,
            commands::profile::profile_list,
            commands::profile::profile_save,
            commands::profile::profile_delete,
            commands::settings::settings_get,
            commands::settings::settings_save,
            commands::settings::settings_reset,
            commands::proxy::proxy_test,
            commands::proxy::proxy_password_set,
            commands::proxy::proxy_password_clear,
            commands::proxy::proxy_password_has,
            commands::proxy::app_paths,
        ])
        .setup(|app| {
            events::spawn_forwarder(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
