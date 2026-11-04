//! 节点读取与写入命令。

use base64::Engine;
use tauri::State;
use zkui_core::audit::AuditEntry;
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
    let data = decode(&data_b64)?;
    let result = state
        .manager
        .set_data(&connection_id, &path, data, expected_version)
        .await;
    match &result {
        Ok(stat) => {
            let mut entry = AuditEntry::success("set", &connection_id, &path);
            entry.before_version = Some(expected_version);
            entry.after_version = Some(stat.version);
            state.audit.record(entry)?;
        }
        Err(error) => {
            state.audit.record(AuditEntry::failure(
                "set",
                &connection_id,
                &path,
                error.as_ref(),
            ))?;
        }
    }
    result
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
    let data = decode(&data_b64)?;
    let result = state
        .manager
        .create_node(&connection_id, &path, data, mode, create_parents)
        .await;
    match &result {
        Ok(actual) => {
            let mut entry = AuditEntry::success("create", &connection_id, actual);
            entry.detail = Some(format!("mode={mode:?}"));
            state.audit.record(entry)?;
        }
        Err(error) => {
            state.audit.record(AuditEntry::failure(
                "create",
                &connection_id,
                &path,
                error.as_ref(),
            ))?;
        }
    }
    result
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
    let result = state
        .manager
        .delete_node(&connection_id, &path, recursive)
        .await;
    match &result {
        Ok(outcome) => match &outcome.failed_error {
            Some(error) => {
                let mut entry = AuditEntry::failure("delete", &connection_id, &path, error);
                entry.detail = Some(format!(
                    "deleted={}, unexecuted={}",
                    outcome.deleted, outcome.unexecuted
                ));
                state.audit.record(entry)?;
            }
            None => {
                let mut entry = AuditEntry::success("delete", &connection_id, &path);
                entry.detail = Some(format!("deleted={}", outcome.deleted));
                state.audit.record(entry)?;
            }
        },
        Err(error) => {
            state.audit.record(AuditEntry::failure(
                "delete",
                &connection_id,
                &path,
                error.as_ref(),
            ))?;
        }
    }
    result
}
