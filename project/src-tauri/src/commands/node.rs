//! 节点读取与写入命令。

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use base64::Engine;
use tauri::State;
use zkui_core::audit::AuditEvent;
use zkui_core::error::{AppResult, app};
use zkui_core::ipc::audit::AuditOp;
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
    max_bytes: Option<u64>,
) -> AppResult<NodeData> {
    state.manager.data(&connection_id, &path, max_bytes).await
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
    let generation = state.manager.generation_of(&connection_id)?;
    let actor = state.manager.actor_of(&connection_id)?;
    match &result {
        Ok(stat) => {
            state.audit.record(
                AuditEvent::success(AuditOp::Set, &connection_id, generation, &actor, &path)
                    .with_versions(expected_version, stat.version),
            )?;
        }
        Err(error) => {
            state.audit.record(AuditEvent::failure(
                AuditOp::Set,
                &connection_id,
                generation,
                &actor,
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
    let generation = state.manager.generation_of(&connection_id)?;
    let actor = state.manager.actor_of(&connection_id)?;
    match &result {
        Ok(actual) => {
            state.audit.record(
                AuditEvent::success(AuditOp::Create, &connection_id, generation, &actor, actual)
                    .with_detail(format!("mode={mode:?}")),
            )?;
        }
        Err(error) => {
            state.audit.record(AuditEvent::failure(
                AuditOp::Create,
                &connection_id,
                generation,
                &actor,
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
    request_id: String,
) -> AppResult<DeletePreview> {
    let cancel = Arc::new(AtomicBool::new(false));
    state
        .preview_cancels
        .lock()
        .expect("preview cancels lock poisoned")
        .insert(request_id.clone(), cancel.clone());
    let result = state
        .manager
        .delete_preview(&connection_id, &path, recursive, &cancel)
        .await;
    state
        .preview_cancels
        .lock()
        .expect("preview cancels lock poisoned")
        .remove(&request_id);
    result
}

#[tauri::command]
pub fn node_delete_preview_cancel(state: State<'_, AppState>, request_id: String) -> bool {
    let cancels = state
        .preview_cancels
        .lock()
        .expect("preview cancels lock poisoned");
    match cancels.get(&request_id) {
        Some(flag) => {
            flag.store(true, Ordering::SeqCst);
            true
        }
        None => false,
    }
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
    let generation = state.manager.generation_of(&connection_id)?;
    let actor = state.manager.actor_of(&connection_id)?;
    match &result {
        Ok(outcome) => match &outcome.failed_error {
            Some(error) => {
                state.audit.record(
                    AuditEvent::failure(
                        AuditOp::Delete,
                        &connection_id,
                        generation,
                        &actor,
                        &path,
                        error,
                    )
                    .with_detail(format!(
                        "deleted={}, unexecuted={}",
                        outcome.deleted, outcome.unexecuted
                    )),
                )?;
            }
            None => {
                state.audit.record(
                    AuditEvent::success(AuditOp::Delete, &connection_id, generation, &actor, &path)
                        .with_detail(format!("deleted={}", outcome.deleted)),
                )?;
            }
        },
        Err(error) => {
            state.audit.record(AuditEvent::failure(
                AuditOp::Delete,
                &connection_id,
                generation,
                &actor,
                &path,
                error.as_ref(),
            ))?;
        }
    }
    result
}

#[tauri::command]
pub async fn node_acl(
    state: State<'_, AppState>,
    connection_id: String,
    path: String,
) -> AppResult<Vec<zkui_core::ipc::node::AclEntry>> {
    state.manager.acl(&connection_id, &path).await
}
