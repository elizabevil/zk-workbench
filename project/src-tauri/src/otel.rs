//! OTel 周期导出：仅在用户启用且配置端点时运行。

use std::time::Duration;

use tauri::{AppHandle, Manager};

use crate::state::AppState;

pub fn spawn_periodic(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;
            let state = app.state::<AppState>();
            let settings = state.settings.snapshot();
            if !settings.observability.otel.enabled
                || settings.observability.otel.endpoint.trim().is_empty()
            {
                continue;
            }
            if let Err(error) = crate::commands::otel::export_once(&state).await {
                tracing::warn!(?error, "otel export failed");
            }
        }
    });
}
