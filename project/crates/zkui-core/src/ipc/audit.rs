//! 审计 DTO。所有字段非空，缺失语义由约定值表达（见 `audit::VERSION_NONE`）。

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// 审计操作类型。只覆盖本应用发起的变更。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case", export)]
pub enum AuditOp {
    Set,
    Create,
    Delete,
    SetAcl,
    /// 清空审计（管理员操作，不含连接与路径）。
    Clear,
}

impl AuditOp {
    pub const fn as_str(self) -> &'static str {
        match self {
            AuditOp::Set => "set",
            AuditOp::Create => "create",
            AuditOp::Delete => "delete",
            AuditOp::SetAcl => "set_acl",
            AuditOp::Clear => "clear",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "set" => Some(AuditOp::Set),
            "create" => Some(AuditOp::Create),
            "delete" => Some(AuditOp::Delete),
            "set_acl" => Some(AuditOp::SetAcl),
            "clear" | "audit.clear" => Some(AuditOp::Clear),
            _ => None,
        }
    }
}

/// 审计结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "snake_case", export)]
pub enum AuditResult {
    Success,
    Failure,
}

impl AuditResult {
    pub const fn as_str(self) -> &'static str {
        match self {
            AuditResult::Success => "success",
            AuditResult::Failure => "failure",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "success" => Some(AuditResult::Success),
            "failure" => Some(AuditResult::Failure),
            _ => None,
        }
    }
}

/// 审计记录。`before_version`/`after_version` 为 -1 表示不适用；
/// `generation` 为 0 表示未知；空字符串表示无值。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct AuditRecord {
    #[ts(type = "number")]
    pub id: i64,
    #[ts(type = "number")]
    pub ts: i64,
    pub op: AuditOp,
    pub connection_id: String,
    #[ts(type = "number")]
    pub generation: u64,
    pub path: String,
    pub before_version: i32,
    pub after_version: i32,
    pub result: AuditResult,
    pub error_kind: String,
    pub detail: String,
    pub actor: String,
    pub prev_hash: String,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct AuditVerifyReport {
    pub ok: bool,
    #[ts(type = "number")]
    pub checked: u64,
    #[ts(type = "number")]
    pub broken_at: i64,
    pub reason: String,
}
