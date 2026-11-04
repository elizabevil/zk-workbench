//! 审计命令。

use tauri::State;
use zkui_core::error::AppResult;
use zkui_core::ipc::audit::{AuditRecord, AuditVerifyReport};

use crate::state::AppState;

#[tauri::command]
pub fn audit_list(
    state: State<'_, AppState>,
    limit: u32,
    offset: u32,
    path_filter: Option<String>,
) -> AppResult<Vec<AuditRecord>> {
    state
        .audit
        .list(limit, offset, path_filter.as_deref().unwrap_or(""))
}

#[tauri::command]
pub fn audit_verify(state: State<'_, AppState>) -> AppResult<AuditVerifyReport> {
    state.audit.verify()
}

#[tauri::command]
pub fn audit_clear(state: State<'_, AppState>) -> AppResult<u64> {
    state.audit.clear()
}

#[tauri::command]
pub fn audit_prune(state: State<'_, AppState>, retention_days: u32) -> AppResult<u64> {
    state.audit.prune(retention_days)
}

#[tauri::command]
pub fn audit_export(state: State<'_, AppState>, target: String) -> AppResult<()> {
    state.audit.export(&target)
}
