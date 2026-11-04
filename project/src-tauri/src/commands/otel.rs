//! OpenTelemetry 导出命令。

use std::time::{SystemTime, UNIX_EPOCH};

use tauri::State;
use zkui_core::error::{AppResult, app};
use zkui_core::ipc::metrics::OtelExportStatus;

use crate::state::AppState;

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or_default()
}

pub async fn export_once(state: &AppState) -> AppResult<()> {
    let settings = state.settings.snapshot();
    if !settings.observability.otel.enabled {
        return Err(Box::new(app::invalid_argument("OTel export is disabled")));
    }
    let endpoint = settings.observability.otel.endpoint.trim().to_string();
    if endpoint.is_empty() {
        return Err(Box::new(app::invalid_argument(
            "OTel endpoint is not configured",
        )));
    }
    let result = zkui_core::otel::export(&endpoint, &state.manager.metrics()).await;
    let mut runtime = state.otel.lock().expect("otel lock poisoned");
    match result {
        Ok(()) => {
            runtime.last_export_ts = Some(now_ms());
            runtime.last_error = None;
            Ok(())
        }
        Err(error) => {
            runtime.last_error = Some(error.clone());
            Err(Box::new(app::otel_failed(&error)))
        }
    }
}

#[tauri::command]
pub async fn otel_export_now(state: State<'_, AppState>) -> AppResult<()> {
    export_once(&state).await
}

#[tauri::command]
pub fn otel_status(state: State<'_, AppState>) -> OtelExportStatus {
    let settings = state.settings.snapshot();
    let runtime = state.otel.lock().expect("otel lock poisoned");
    OtelExportStatus {
        enabled: settings.observability.otel.enabled,
        endpoint: settings.observability.otel.endpoint,
        last_export_ts: runtime.last_export_ts,
        last_error: runtime.last_error.clone(),
    }
}
