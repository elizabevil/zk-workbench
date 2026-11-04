//! 事件广播：把 core 的状态事件转发为 Tauri 事件。

use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::broadcast::error::RecvError;
use zkui_core::ipc::events::CONNECTION_STATE_CHANGED;

use crate::state::AppState;

/// 启动事件转发任务。需在 setup 阶段调用。
pub fn spawn_forwarder(app: AppHandle) {
    let receiver = {
        let state = app.state::<AppState>();
        state.manager.subscribe()
    };
    let mut receiver = receiver;
    tauri::async_runtime::spawn(async move {
        loop {
            match receiver.recv().await {
                Ok(event) => {
                    let _ = app.emit(CONNECTION_STATE_CHANGED, event);
                }
                Err(RecvError::Lagged(_)) => continue,
                Err(RecvError::Closed) => break,
            }
        }
    });
}
