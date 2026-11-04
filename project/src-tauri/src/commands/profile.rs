//! 连接档案命令。

use tauri::State;
use zkui_core::error::AppResult;
use zkui_core::profiles::{ConnectionProfile, ProfileInput};

use crate::state::AppState;

#[tauri::command]
pub fn profile_list(state: State<'_, AppState>) -> Vec<ConnectionProfile> {
    state.profiles.list()
}

#[tauri::command]
pub fn profile_save(
    state: State<'_, AppState>,
    input: ProfileInput,
) -> AppResult<ConnectionProfile> {
    state.profiles.save(input)
}

#[tauri::command]
pub fn profile_delete(state: State<'_, AppState>, id: String) -> AppResult<()> {
    state.profiles.delete(&id)
}
