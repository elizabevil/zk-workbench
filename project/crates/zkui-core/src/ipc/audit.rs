//! 审计 DTO。

use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct AuditRecord {
    #[ts(type = "number")]
    pub id: i64,
    #[ts(type = "number")]
    pub ts: i64,
    pub connection_id: Option<String>,
    #[ts(type = "number | null")]
    pub generation: Option<u64>,
    pub path: Option<String>,
    pub op: String,
    pub before_version: Option<i32>,
    pub after_version: Option<i32>,
    pub result: String,
    pub error_kind: Option<String>,
    pub detail: Option<String>,
    pub prev_hash: String,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct AuditVerifyReport {
    pub ok: bool,
    #[ts(type = "number")]
    pub checked: u64,
    #[ts(type = "number | null")]
    pub broken_at: Option<i64>,
    pub reason: Option<String>,
}
