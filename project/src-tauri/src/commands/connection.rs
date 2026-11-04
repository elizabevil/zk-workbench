//! 连接方向命令。

use tauri::State;
use zkui_core::connection::ConnectionManager;
use zkui_core::error::AppResult;
use zkui_core::ipc::connection::{ConnectionSnapshot, CreateConnection};

use crate::state::AppState;

#[tauri::command]
pub async fn connection_create(
    state: State<'_, AppState>,
    request: CreateConnection,
) -> AppResult<ConnectionSnapshot> {
    let manager: &ConnectionManager = &state.manager;
    manager.create(request).await
}

#[tauri::command]
pub fn connection_snapshot(
    state: State<'_, AppState>,
    connection_id: String,
) -> AppResult<ConnectionSnapshot> {
    state.manager.snapshot(&connection_id)
}

#[tauri::command]
pub fn connection_close(state: State<'_, AppState>, connection_id: String) -> AppResult<()> {
    state.manager.close(&connection_id)
}

#[tauri::command]
pub fn connection_list(state: State<'_, AppState>) -> Vec<ConnectionSnapshot> {
    state.manager.list()
}
