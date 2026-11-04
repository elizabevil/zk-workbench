//! 命令区命令。

use tauri::State;
use zkui_core::audit::AuditEvent;
use zkui_core::command::Command;
use zkui_core::error::AppResult;
use zkui_core::ipc::audit::AuditOp;

use crate::state::AppState;

#[tauri::command]
pub fn command_preview(line: String) -> AppResult<Option<String>> {
    let command = zkui_core::command::parse(&line)?;
    Ok(command.write_summary())
}

#[tauri::command]
pub async fn command_execute(
    state: State<'_, AppState>,
    connection_id: String,
    line: String,
) -> AppResult<String> {
    let command = zkui_core::command::parse(&line)?;
    let generation = state.manager.generation_of(&connection_id)?;
    let actor = state.manager.actor_of(&connection_id)?;
    let result = zkui_core::command::execute(&state.manager, &connection_id, &command).await;

    if let Some((op, path, before)) = audit_target(&command) {
        match &result {
            Ok(_) => {
                let mut event = AuditEvent::success(op, &connection_id, generation, &actor, path);
                if let Some(version) = before {
                    event = event.with_versions(version, VERSION_UNKNOWN);
                }
                event = event.with_detail("via command");
                state.audit.record(event)?;
            }
            Err(error) => {
                state.audit.record(
                    AuditEvent::failure(op, &connection_id, generation, &actor, path, error)
                        .with_detail("via command"),
                )?;
            }
        }
    }
    result
}

const VERSION_UNKNOWN: i32 = -1;

fn audit_target(command: &Command) -> Option<(AuditOp, &str, Option<i32>)> {
    match command {
        Command::Set { path, version, .. } => Some((AuditOp::Set, path, *version)),
        Command::Create { path, .. } => Some((AuditOp::Create, path, None)),
        Command::Delete { path, .. } => Some((AuditOp::Delete, path, None)),
        Command::DeleteAll { path } => Some((AuditOp::Delete, path, None)),
        Command::SetAcl { path, .. } => Some((AuditOp::SetAcl, path, None)),
        _ => None,
    }
}
