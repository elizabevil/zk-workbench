//! 原生应用菜单。

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Emitter, Manager, Wry};

use crate::i18n;

pub const ACTION_EVENT: &str = "menu.action";
pub mod actions {
    pub const SETTINGS: &str = "settings";
    pub const ABOUT: &str = "about";
}

pub mod ids {
    pub const SETTINGS: &str = "menu.settings";
    pub const ABOUT: &str = "menu.about";
    pub const CLOSE: &str = "menu.close";
}

pub fn build(app: &AppHandle<Wry>, locale: zkui_core::config::Locale) -> tauri::Result<Menu<Wry>> {
    let labels = i18n::labels(locale);
    let app_name = app.package_info().name.clone();
    let about = labels.about.replace("{name}", &app_name);

    let app_menu = Submenu::with_items(
        app,
        &labels.app,
        true,
        &[
            &MenuItem::with_id(app, ids::SETTINGS, &labels.settings, true, None::<&str>)?,
            &MenuItem::with_id(app, ids::ABOUT, &about, true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, ids::CLOSE, &labels.close, true, None::<&str>)?,
        ],
    )?;

    let edit_menu = Submenu::with_items(
        app,
        &labels.edit,
        true,
        &[
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::select_all(app, None)?,
        ],
    )?;

    Menu::with_items(app, &[&app_menu, &edit_menu])
}

pub fn apply(app: &AppHandle<Wry>, locale: zkui_core::config::Locale) {
    if let Ok(menu) = build(app, locale) {
        let _ = app.set_menu(menu);
    }
}

pub fn handle_event(app: &AppHandle<Wry>, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        ids::SETTINGS => {
            let _ = app.emit(ACTION_EVENT, actions::SETTINGS);
        }
        ids::ABOUT => {
            let _ = app.emit(ACTION_EVENT, actions::ABOUT);
        }
        ids::CLOSE => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.close();
            }
        }
        _ => {}
    }
}
