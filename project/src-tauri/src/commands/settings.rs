//! 设置方向命令。

use tauri::State;
use zkui_core::config::Settings;
use zkui_core::error::AppResult;

use crate::state::AppState;

#[tauri::command]
pub fn settings_get(state: State<'_, AppState>) -> Settings {
    state.settings.snapshot()
}

#[tauri::command]
pub fn settings_save(state: State<'_, AppState>, settings: Settings) -> AppResult<Settings> {
    state.settings.save(settings)
}

#[tauri::command]
pub fn settings_reset(state: State<'_, AppState>) -> AppResult<Settings> {
    state.settings.reset()
}
