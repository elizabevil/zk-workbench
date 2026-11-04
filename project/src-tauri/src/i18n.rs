//! 后端文案：从内置 JSON 读取，避免在 Rust 代码里散落字面量。

use std::sync::OnceLock;

use serde::Deserialize;
use zkui_core::config::Locale;

#[derive(Debug, Clone, Deserialize)]
pub struct MenuLabels {
    pub app: String,
    pub edit: String,
    pub settings: String,
    pub about: String,
    pub close: String,
}

static ZH: OnceLock<MenuLabels> = OnceLock::new();
static EN: OnceLock<MenuLabels> = OnceLock::new();

pub fn labels(locale: Locale) -> &'static MenuLabels {
    match locale {
        Locale::ZhCn => ZH.get_or_init(|| load(include_str!("../i18n/menu.zh-CN.json"))),
        Locale::EnUs => EN.get_or_init(|| load(include_str!("../i18n/menu.en-US.json"))),
    }
}

fn load(raw: &str) -> MenuLabels {
    serde_json::from_str(raw).expect("menu i18n resource must be valid")
}
