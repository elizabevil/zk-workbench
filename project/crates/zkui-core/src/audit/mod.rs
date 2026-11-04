//! 变更审计：SQLite（WAL）+ 哈希链，只记录本应用发起的增删改。

use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, params};
use sha2::{Digest, Sha256};

use crate::error::{AppResult, app};
use crate::ipc::audit::{AuditRecord, AuditVerifyReport};

const GENESIS: &str = "genesis";

/// 一条待写入的审计事件。
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub connection_id: Option<String>,
    pub generation: Option<u64>,
    pub path: Option<String>,
    pub op: String,
    pub before_version: Option<i32>,
    pub after_version: Option<i32>,
    pub result: String,
    pub error_kind: Option<String>,
    pub detail: Option<String>,
}

impl AuditEntry {
    pub fn success(op: &str, connection_id: &str, path: &str) -> Self {
        Self {
            connection_id: Some(connection_id.to_string()),
            generation: None,
            path: Some(path.to_string()),
            op: op.to_string(),
            before_version: None,
            after_version: None,
            result: "success".to_string(),
            error_kind: None,
            detail: None,
        }
    }

    pub fn failure(
        op: &str,
        connection_id: &str,
        path: &str,
        error: &crate::error::AppError,
    ) -> Self {
        Self {
            connection_id: Some(connection_id.to_string()),
            generation: error.fields.generation,
            path: Some(path.to_string()),
            op: op.to_string(),
            before_version: None,
            after_version: None,
            result: "failure".to_string(),
            error_kind: Some(format!("{}.{}", error.scope.as_str(), error.kind.as_str())),
            detail: error.fields.detail.clone(),
        }
    }
}

/// 审计存储。
pub struct AuditStore {
    connection: Mutex<Connection>,
}

impl AuditStore {
    pub fn open(path: &Path) -> AppResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                audit_error(&format!("failed to create audit directory: {error}"))
            })?;
        }
        let connection = Connection::open(path)
            .map_err(|error| audit_error(&format!("failed to open audit database: {error}")))?;
        connection
            .pragma_update(None, "journal_mode", "WAL")
            .map_err(|error| audit_error(&format!("failed to enable WAL: {error}")))?;
        connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS audit_log (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    ts INTEGER NOT NULL,
                    connection_id TEXT,
                    generation INTEGER,
                    path TEXT,
                    op TEXT NOT NULL,
                    before_version INTEGER,
                    after_version INTEGER,
                    result TEXT NOT NULL,
                    error_kind TEXT,
                    detail TEXT,
                    prev_hash TEXT NOT NULL,
                    hash TEXT NOT NULL
                );",
            )
            .map_err(|error| audit_error(&format!("failed to create audit table: {error}")))?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn record(&self, entry: AuditEntry) -> AppResult<()> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        let prev_hash = last_hash(&connection)?;
        let ts = now_ms();
        let hash = compute_hash(&entry, ts, &prev_hash);
        connection
            .execute(
                "INSERT INTO audit_log (ts, connection_id, generation, path, op, before_version, after_version, result, error_kind, detail, prev_hash, hash)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                params![
                    ts,
                    entry.connection_id,
                    entry.generation.map(|value| value as i64),
                    entry.path,
                    entry.op,
                    entry.before_version,
                    entry.after_version,
                    entry.result,
                    entry.error_kind,
                    entry.detail,
                    prev_hash,
                    hash
                ],
            )
            .map_err(|error| audit_error(&format!("failed to append audit record: {error}")))?;
        Ok(())
    }

    pub fn list(
        &self,
        limit: u32,
        offset: u32,
        path_filter: Option<&str>,
    ) -> AppResult<Vec<AuditRecord>> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        let mut statement = connection
            .prepare(
                "SELECT id, ts, connection_id, generation, path, op, before_version, after_version,
                        result, error_kind, detail, prev_hash, hash
                 FROM audit_log
                 WHERE (?1 IS NULL OR path = ?1)
                 ORDER BY id DESC LIMIT ?2 OFFSET ?3",
            )
            .map_err(|error| audit_error(&format!("failed to prepare audit query: {error}")))?;
        let rows = statement
            .query_map(params![path_filter, limit, offset], |row| {
                Ok(AuditRecord {
                    id: row.get(0)?,
                    ts: row.get(1)?,
                    connection_id: row.get(2)?,
                    generation: row.get::<_, Option<i64>>(3)?.map(|value| value as u64),
                    path: row.get(4)?,
                    op: row.get(5)?,
                    before_version: row.get(6)?,
                    after_version: row.get(7)?,
                    result: row.get(8)?,
                    error_kind: row.get(9)?,
                    detail: row.get(10)?,
                    prev_hash: row.get(11)?,
                    hash: row.get(12)?,
                })
            })
            .map_err(|error| audit_error(&format!("failed to query audit records: {error}")))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| audit_error(&format!("failed to read audit rows: {error}")))
    }

    pub fn verify(&self) -> AppResult<AuditVerifyReport> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        let mut statement = connection
            .prepare(
                "SELECT id, ts, connection_id, generation, path, op, before_version, after_version,
                        result, error_kind, detail, prev_hash, hash
                 FROM audit_log ORDER BY id ASC",
            )
            .map_err(|error| audit_error(&format!("failed to prepare verify query: {error}")))?;
        let rows = statement
            .query_map([], |row| {
                Ok(AuditRecord {
                    id: row.get(0)?,
                    ts: row.get(1)?,
                    connection_id: row.get(2)?,
                    generation: row.get::<_, Option<i64>>(3)?.map(|value| value as u64),
                    path: row.get(4)?,
                    op: row.get(5)?,
                    before_version: row.get(6)?,
                    after_version: row.get(7)?,
                    result: row.get(8)?,
                    error_kind: row.get(9)?,
                    detail: row.get(10)?,
                    prev_hash: row.get(11)?,
                    hash: row.get(12)?,
                })
            })
            .map_err(|error| audit_error(&format!("failed to read audit rows: {error}")))?;

        let mut checked = 0u64;
        let mut last = GENESIS.to_string();
        for row in rows {
            let record =
                row.map_err(|error| audit_error(&format!("failed to read audit row: {error}")))?;
            // 第一条允许是任意锚点（清理或过期裁剪后继续校验）。
            if checked > 0 && record.prev_hash != last {
                return Ok(AuditVerifyReport {
                    ok: false,
                    checked,
                    broken_at: Some(record.id),
                    reason: Some("prev_hash does not match previous record".to_string()),
                });
            }
            let expected = compute_record_hash(&record);
            if expected != record.hash {
                return Ok(AuditVerifyReport {
                    ok: false,
                    checked,
                    broken_at: Some(record.id),
                    reason: Some("hash does not match record content".to_string()),
                });
            }
            last = record.hash;
            checked += 1;
        }
        Ok(AuditVerifyReport {
            ok: true,
            checked,
            broken_at: None,
            reason: None,
        })
    }

    /// 清空审计并写入一条 genesis 标记；返回清理的条数。
    pub fn clear(&self) -> AppResult<u64> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))
            .map_err(|error| audit_error(&format!("failed to count audit rows: {error}")))?;
        connection
            .execute("DELETE FROM audit_log", [])
            .map_err(|error| audit_error(&format!("failed to clear audit log: {error}")))?;
        let entry = AuditEntry {
            connection_id: None,
            generation: None,
            path: None,
            op: "audit.clear".to_string(),
            before_version: None,
            after_version: None,
            result: "success".to_string(),
            error_kind: None,
            detail: Some(format!("cleared {count} records")),
        };
        drop(connection);
        self.record(entry)?;
        Ok(count as u64)
    }

    /// 删除早于保留期的记录；返回删除条数。
    pub fn prune(&self, retention_days: u32) -> AppResult<u64> {
        let cutoff = now_ms() - i64::from(retention_days) * 24 * 60 * 60 * 1000;
        let connection = self.connection.lock().expect("audit lock poisoned");
        let deleted = connection
            .execute("DELETE FROM audit_log WHERE ts < ?1", params![cutoff])
            .map_err(|error| audit_error(&format!("failed to prune audit log: {error}")))?;
        Ok(deleted as u64)
    }

    /// 导出为 SQLite 快照。
    pub fn export(&self, target: &str) -> AppResult<()> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        if Path::new(target).exists() {
            return Err(audit_error("export target already exists"));
        }
        let escaped = target.replace('\'', "''");
        connection
            .execute_batch(&format!("VACUUM INTO '{escaped}'"))
            .map_err(|error| audit_error(&format!("failed to export audit log: {error}")))
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or_default()
}

fn last_hash(connection: &Connection) -> AppResult<String> {
    match connection.query_row(
        "SELECT hash FROM audit_log ORDER BY id DESC LIMIT 1",
        [],
        |row| row.get::<_, String>(0),
    ) {
        Ok(hash) => Ok(hash),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(GENESIS.to_string()),
        Err(error) => Err(audit_error(&format!(
            "failed to read last audit hash: {error}"
        ))),
    }
}

fn compute_hash(entry: &AuditEntry, ts: i64, prev_hash: &str) -> String {
    let canonical = format!(
        "{ts}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{prev_hash}",
        entry.connection_id.as_deref().unwrap_or(""),
        entry
            .generation
            .map(|value| value.to_string())
            .unwrap_or_default(),
        entry.path.as_deref().unwrap_or(""),
        entry.op,
        entry
            .before_version
            .map(|value| value.to_string())
            .unwrap_or_default(),
        entry
            .after_version
            .map(|value| value.to_string())
            .unwrap_or_default(),
        entry.result,
        entry.error_kind.as_deref().unwrap_or(""),
        entry.detail.as_deref().unwrap_or(""),
    );
    hex::encode(Sha256::digest(canonical.as_bytes()))
}

fn compute_record_hash(record: &AuditRecord) -> String {
    let canonical = format!(
        "{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}",
        record.ts,
        record.connection_id.as_deref().unwrap_or(""),
        record
            .generation
            .map(|value| value.to_string())
            .unwrap_or_default(),
        record.path.as_deref().unwrap_or(""),
        record.op,
        record
            .before_version
            .map(|value| value.to_string())
            .unwrap_or_default(),
        record
            .after_version
            .map(|value| value.to_string())
            .unwrap_or_default(),
        record.result,
        record.error_kind.as_deref().unwrap_or(""),
        record.detail.as_deref().unwrap_or(""),
        record.prev_hash,
    );
    hex::encode(Sha256::digest(canonical.as_bytes()))
}

fn audit_error(detail: &str) -> Box<crate::error::AppError> {
    Box::new(app::audit_failed(detail))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (AuditStore, std::path::PathBuf) {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let seq = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "zkui-audit-{}-{}-{seq}",
            std::process::id(),
            now_ms()
        ));
        let path = dir.join("audit.db");
        (AuditStore::open(&path).expect("open"), dir)
    }

    #[test]
    fn append_and_verify() {
        let (store, dir) = store();
        let mut entry = AuditEntry::success("set", "conn-1", "/a");
        entry.after_version = Some(2);
        store.record(entry).expect("record");
        store
            .record(AuditEntry::success("create", "conn-1", "/b"))
            .expect("record");
        let report = store.verify().expect("verify");
        assert!(report.ok);
        assert_eq!(report.checked, 2);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn tampered_record_is_detected() {
        let (store, dir) = store();
        store
            .record(AuditEntry::success("set", "conn-1", "/a"))
            .expect("record");
        {
            let conn = store.connection.lock().unwrap();
            conn.execute("UPDATE audit_log SET path = '/tampered' WHERE id = 1", [])
                .expect("tamper");
        }
        let report = store.verify().expect("verify");
        assert!(!report.ok);
        assert_eq!(report.broken_at, Some(1));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn clear_resets_chain() {
        let (store, dir) = store();
        store
            .record(AuditEntry::success("set", "conn-1", "/a"))
            .expect("record");
        let cleared = store.clear().expect("clear");
        assert_eq!(cleared, 1);
        let report = store.verify().expect("verify");
        assert!(report.ok);
        assert_eq!(report.checked, 1);
        let _ = std::fs::remove_dir_all(dir);
    }
}
