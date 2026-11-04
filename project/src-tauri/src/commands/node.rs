//! 节点读取命令。

use tauri::State;
use zkui_core::error::AppResult;
use zkui_core::ipc::node::{NodeData, NodeStat};

use crate::state::AppState;

#[tauri::command]
pub async fn node_children(
    state: State<'_, AppState>,
    connection_id: String,
    path: String,
) -> AppResult<Vec<String>> {
    state.manager.children(&connection_id, &path).await
}

#[tauri::command]
pub async fn node_stat(
    state: State<'_, AppState>,
    connection_id: String,
    path: String,
) -> AppResult<NodeStat> {
    state.manager.stat(&connection_id, &path).await
}

#[tauri::command]
pub async fn node_data(
    state: State<'_, AppState>,
    connection_id: String,
    path: String,
) -> AppResult<NodeData> {
    state.manager.data(&connection_id, &path).await
}
