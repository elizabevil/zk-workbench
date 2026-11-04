//! 命令区命令。

use tauri::State;
use zkui_core::error::AppResult;

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
    zkui_core::command::execute(&state.manager, &connection_id, &command).await
}
