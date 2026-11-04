//! Tauri 应用入口：注册状态、命令与事件转发。

mod commands;
mod events;
mod logging;
mod otel;
mod state;

use tauri::Manager;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::connection::connection_create,
            commands::connection::connection_snapshot,
            commands::connection::connection_close,
            commands::app::app_info,
            commands::app::open_external,
            commands::appearance::background_image_store,
            commands::appearance::background_image_load,
            commands::connection::connection_list,
            commands::metrics::metrics_snapshot,
            commands::audit::audit_list,
            commands::audit::audit_verify,
            commands::audit::audit_clear,
            commands::audit::audit_prune,
            commands::audit::audit_export,
            commands::command::command_preview,
            commands::command::command_execute,
            commands::otel::otel_export_now,
            commands::otel::otel_status,
            commands::node::node_children,
            commands::node::node_stat,
            commands::node::node_acl,
            commands::node::node_data,
            commands::node::node_set,
            commands::node::node_create,
            commands::node::node_delete_preview,
            commands::node::node_delete_preview_cancel,
            commands::node::node_delete,
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
            let log_directory = app.state::<AppState>().settings.snapshot().logs.directory;
            let logs = app.state::<AppState>().settings.snapshot().logs;
            logging::init(
                &log_directory,
                logs.max_size_mb * 1024 * 1024,
                logs.retention_days,
            );
            events::spawn_forwarder(app.handle().clone());
            otel::spawn_periodic(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
