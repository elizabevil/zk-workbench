//! 指标命令。

use tauri::State;
use zkui_core::ipc::metrics::MetricsSnapshot;

use crate::state::AppState;

#[tauri::command]
pub fn metrics_snapshot(state: State<'_, AppState>) -> MetricsSnapshot {
    state.manager.metrics()
}
