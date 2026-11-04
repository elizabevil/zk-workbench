//! 设置方向命令。

use tauri::{AppHandle, State};
use zkui_core::config::Settings;
use zkui_core::error::AppResult;

use crate::state::AppState;

#[tauri::command]
pub fn settings_get(state: State<'_, AppState>) -> Settings {
    state.settings.snapshot()
}

#[tauri::command]
pub fn settings_save(
    app: AppHandle,
    state: State<'_, AppState>,
    settings: Settings,
) -> AppResult<Settings> {
    let locale = settings.general.locale;
    let saved = state.settings.save(settings)?;
    crate::menu::apply(&app, locale);
    Ok(saved)
}

#[tauri::command]
pub fn settings_reset(state: State<'_, AppState>) -> AppResult<Settings> {
    state.settings.reset()
}
