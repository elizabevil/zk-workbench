//! 节点读取与写入命令。

use base64::Engine;
use tauri::State;
use zkui_core::error::{AppResult, app};
use zkui_core::ipc::node::{DeleteOutcome, DeletePreview, NodeCreateMode, NodeData, NodeStat};

use crate::state::AppState;

fn decode(data_b64: &str) -> AppResult<Vec<u8>> {
    base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|error| {
            Box::new(app::invalid_argument(&format!(
                "invalid base64 data: {error}"
            )))
        })
}

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

#[tauri::command]
pub async fn node_set(
    state: State<'_, AppState>,
    connection_id: String,
    path: String,
    data_b64: String,
    expected_version: i32,
) -> AppResult<NodeStat> {
    state
        .manager
        .set_data(&connection_id, &path, decode(&data_b64)?, expected_version)
        .await
}

#[tauri::command]
pub async fn node_create(
    state: State<'_, AppState>,
    connection_id: String,
    path: String,
    data_b64: String,
    mode: NodeCreateMode,
    create_parents: bool,
) -> AppResult<String> {
    state
        .manager
        .create_node(
            &connection_id,
            &path,
            decode(&data_b64)?,
            mode,
            create_parents,
        )
        .await
}

#[tauri::command]
pub async fn node_delete_preview(
    state: State<'_, AppState>,
    connection_id: String,
    path: String,
    recursive: bool,
) -> AppResult<DeletePreview> {
    state
        .manager
        .delete_preview(&connection_id, &path, recursive)
        .await
}

#[tauri::command]
pub async fn node_delete(
    state: State<'_, AppState>,
    connection_id: String,
    path: String,
    recursive: bool,
) -> AppResult<DeleteOutcome> {
    state
        .manager
        .delete_node(&connection_id, &path, recursive)
        .await
}
