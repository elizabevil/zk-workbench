//! 界面语言。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 支持的语言。序列化值与 BCP 47 一致，前端直接使用。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../src/api/types/")]
pub enum Locale {
    #[default]
    #[serde(rename = "zh-CN")]
    #[ts(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "en-US")]
    #[ts(rename = "en-US")]
    EnUs,
}

impl Locale {
    pub const ALL: [Locale; 2] = [Locale::ZhCn, Locale::EnUs];

    pub fn as_str(self) -> &'static str {
        match self {
            Locale::ZhCn => "zh-CN",
            Locale::EnUs => "en-US",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Locale;

    #[test]
    fn serializes_bcp47_values() {
        assert_eq!(serde_json::to_string(&Locale::ZhCn).unwrap(), "\"zh-CN\"");
        assert_eq!(serde_json::to_string(&Locale::EnUs).unwrap(), "\"en-US\"");
    }

    #[test]
    fn all_covers_variants() {
        assert_eq!(Locale::ALL.len(), 2);
    }
}
