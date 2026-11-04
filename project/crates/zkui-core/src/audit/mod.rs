//! 变更审计：SQLite（WAL）+ 哈希链，只记录本应用发起的增删改。

use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, params};
use sha2::{Digest, Sha256};

use crate::error::{AppResult, app};
use crate::ipc::audit::{AuditOp, AuditRecord, AuditResult, AuditVerifyReport};

/// 版本号不适用时的约定值（ZooKeeper version 从 0 开始，-1 不会出现）。
pub const VERSION_NONE: i32 = -1;
/// generation 未知时的约定值。
pub const GENERATION_NONE: u64 = 0;

const SCHEMA_VERSION: i64 = 2;
const GENESIS: &str = "genesis";

/// 一条待写入的审计事件。
#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub op: AuditOp,
    pub connection_id: String,
    pub generation: u64,
    pub path: String,
    pub before_version: i32,
    pub after_version: i32,
    pub result: AuditResult,
    pub error_kind: String,
    pub detail: String,
    pub actor: String,
}

impl AuditEvent {
    pub fn success(
        op: AuditOp,
        connection_id: &str,
        generation: u64,
        actor: &str,
        path: &str,
    ) -> Self {
        Self {
            op,
            connection_id: connection_id.to_string(),
            generation,
            path: path.to_string(),
            before_version: VERSION_NONE,
            after_version: VERSION_NONE,
            result: AuditResult::Success,
            error_kind: String::new(),
            detail: String::new(),
            actor: actor.to_string(),
        }
    }

    pub fn failure(
        op: AuditOp,
        connection_id: &str,
        generation: u64,
        actor: &str,
        path: &str,
        error: &crate::error::AppError,
    ) -> Self {
        Self {
            op,
            connection_id: connection_id.to_string(),
            generation,
            path: path.to_string(),
            before_version: VERSION_NONE,
            after_version: VERSION_NONE,
            result: AuditResult::Failure,
            error_kind: format!("{}.{}", error.scope.as_str(), error.kind.as_str()),
            detail: error.fields.detail.clone(),
            actor: actor.to_string(),
        }
    }

    pub fn with_versions(mut self, before: i32, after: i32) -> Self {
        self.before_version = before;
        self.after_version = after;
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = detail.into();
        self
    }
}

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
        migrate(&connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn record(&self, event: AuditEvent) -> AppResult<()> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        let prev_hash = last_hash(&connection)?;
        let ts = now_ms();
        let hash = compute_event_hash(&event, ts, &prev_hash);
        connection
            .execute(
                "INSERT INTO audit_log (ts, op, connection_id, generation, path, before_version,
                    after_version, result, error_kind, detail, actor, prev_hash, hash)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![
                    ts,
                    event.op.as_str(),
                    event.connection_id,
                    event.generation as i64,
                    event.path,
                    event.before_version,
                    event.after_version,
                    event.result.as_str(),
                    event.error_kind,
                    event.detail,
                    event.actor,
                    prev_hash,
                    hash
                ],
            )
            .map_err(|error| audit_error(&format!("failed to append audit record: {error}")))?;
        Ok(())
    }

    pub fn list(&self, limit: u32, offset: u32, path_filter: &str) -> AppResult<Vec<AuditRecord>> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        let mut statement = connection
            .prepare(
                "SELECT id, ts, op, connection_id, generation, path, before_version, after_version,
                        result, error_kind, detail, actor, prev_hash, hash
                 FROM audit_log
                 WHERE (?1 = '' OR path = ?1)
                 ORDER BY id DESC LIMIT ?2 OFFSET ?3",
            )
            .map_err(|error| audit_error(&format!("failed to prepare audit query: {error}")))?;
        let rows = statement
            .query_map(params![path_filter, limit, offset], map_record)
            .map_err(|error| audit_error(&format!("failed to query audit records: {error}")))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|error| audit_error(&format!("failed to read audit rows: {error}")))
    }

    pub fn verify(&self) -> AppResult<AuditVerifyReport> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        let mut statement = connection
            .prepare(
                "SELECT id, ts, op, connection_id, generation, path, before_version, after_version,
                        result, error_kind, detail, actor, prev_hash, hash
                 FROM audit_log ORDER BY id ASC",
            )
            .map_err(|error| audit_error(&format!("failed to prepare verify query: {error}")))?;
        let rows = statement
            .query_map([], map_record)
            .map_err(|error| audit_error(&format!("failed to read audit rows: {error}")))?;

        let mut checked = 0u64;
        let mut last = GENESIS.to_string();
        for row in rows {
            let record =
                row.map_err(|error| audit_error(&format!("failed to read audit row: {error}")))?;
            if checked > 0 && record.prev_hash != last {
                return Ok(AuditVerifyReport {
                    ok: false,
                    checked,
                    broken_at: record.id,
                    reason: "prev_hash does not match previous record".to_string(),
                });
            }
            let expected = compute_record_hash(&record);
            if expected != record.hash {
                return Ok(AuditVerifyReport {
                    ok: false,
                    checked,
                    broken_at: record.id,
                    reason: "hash does not match record content".to_string(),
                });
            }
            last = record.hash;
            checked += 1;
        }
        Ok(AuditVerifyReport {
            ok: true,
            checked,
            broken_at: 0,
            reason: String::new(),
        })
    }

    /// 清空审计并写入一条 clear 标记；返回清理条数。
    pub fn clear(&self) -> AppResult<u64> {
        let connection = self.connection.lock().expect("audit lock poisoned");
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))
            .map_err(|error| audit_error(&format!("failed to count audit rows: {error}")))?;
        connection
            .execute("DELETE FROM audit_log", [])
            .map_err(|error| audit_error(&format!("failed to clear audit log: {error}")))?;
        drop(connection);
        let event = AuditEvent {
            op: AuditOp::Clear,
            connection_id: String::new(),
            generation: GENERATION_NONE,
            path: String::new(),
            before_version: VERSION_NONE,
            after_version: VERSION_NONE,
            result: AuditResult::Success,
            error_kind: String::new(),
            detail: format!("cleared {count} records"),
            actor: String::new(),
        };
        self.record(event)?;
        Ok(count as u64)
    }

    pub fn prune(&self, retention_days: u32) -> AppResult<u64> {
        let cutoff = now_ms() - i64::from(retention_days) * 24 * 60 * 60 * 1000;
        let connection = self.connection.lock().expect("audit lock poisoned");
        let deleted = connection
            .execute("DELETE FROM audit_log WHERE ts < ?1", params![cutoff])
            .map_err(|error| audit_error(&format!("failed to prune audit log: {error}")))?;
        Ok(deleted as u64)
    }

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

fn map_record(row: &rusqlite::Row<'_>) -> rusqlite::Result<AuditRecord> {
    let op_text: String = row.get(2)?;
    let op = AuditOp::from_db(&op_text).ok_or_else(|| {
        rusqlite::Error::InvalidColumnType(2, "op".to_string(), rusqlite::types::Type::Text)
    })?;
    let result_text: String = row.get(8)?;
    let result = AuditResult::from_db(&result_text).ok_or_else(|| {
        rusqlite::Error::InvalidColumnType(8, "result".to_string(), rusqlite::types::Type::Text)
    })?;
    Ok(AuditRecord {
        id: row.get(0)?,
        ts: row.get(1)?,
        op,
        connection_id: row.get(3)?,
        generation: row.get::<_, i64>(4)? as u64,
        path: row.get(5)?,
        before_version: row.get(6)?,
        after_version: row.get(7)?,
        result,
        error_kind: row.get(9)?,
        detail: row.get(10)?,
        actor: row.get(11)?,
        prev_hash: row.get(12)?,
        hash: row.get(13)?,
    })
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

struct HashPayload<'a> {
    ts: i64,
    op: &'a str,
    connection_id: &'a str,
    generation: u64,
    path: &'a str,
    before: i32,
    after: i32,
    result: &'a str,
    error_kind: &'a str,
    detail: &'a str,
    actor: &'a str,
    prev_hash: &'a str,
}

fn hash_payload(payload: &HashPayload<'_>) -> String {
    let canonical = format!(
        "{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}",
        payload.ts,
        payload.op,
        payload.connection_id,
        payload.generation,
        payload.path,
        payload.before,
        payload.after,
        payload.result,
        payload.error_kind,
        payload.detail,
        payload.actor,
        payload.prev_hash,
    );
    hex::encode(Sha256::digest(canonical.as_bytes()))
}

fn compute_event_hash(event: &AuditEvent, ts: i64, prev_hash: &str) -> String {
    hash_payload(&HashPayload {
        ts,
        op: event.op.as_str(),
        connection_id: &event.connection_id,
        generation: event.generation,
        path: &event.path,
        before: event.before_version,
        after: event.after_version,
        result: event.result.as_str(),
        error_kind: &event.error_kind,
        detail: &event.detail,
        actor: &event.actor,
        prev_hash,
    })
}

fn compute_record_hash(record: &AuditRecord) -> String {
    hash_payload(&HashPayload {
        ts: record.ts,
        op: record.op.as_str(),
        connection_id: &record.connection_id,
        generation: record.generation,
        path: &record.path,
        before: record.before_version,
        after: record.after_version,
        result: record.result.as_str(),
        error_kind: &record.error_kind,
        detail: &record.detail,
        actor: &record.actor,
        prev_hash: &record.prev_hash,
    })
}

fn audit_error(detail: &str) -> Box<crate::error::AppError> {
    Box::new(app::audit_failed(detail))
}

fn migrate(connection: &Connection) -> AppResult<()> {
    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|error| audit_error(&format!("failed to read audit schema version: {error}")))?;
    if version >= SCHEMA_VERSION {
        return Ok(());
    }

    let old_exists: bool = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'audit_log'",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map(|count| count > 0)
        .map_err(|error| audit_error(&format!("failed to inspect audit schema: {error}")))?;

    connection
        .execute_batch(
            "BEGIN;
             DROP TABLE IF EXISTS audit_log_v2;
             CREATE TABLE audit_log_v2 (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                 ts INTEGER NOT NULL,
                 op TEXT NOT NULL,
                 connection_id TEXT NOT NULL DEFAULT '',
                 generation INTEGER NOT NULL DEFAULT 0,
                 path TEXT NOT NULL DEFAULT '',
                 before_version INTEGER NOT NULL DEFAULT -1,
                 after_version INTEGER NOT NULL DEFAULT -1,
                 result TEXT NOT NULL,
                 error_kind TEXT NOT NULL DEFAULT '',
                 detail TEXT NOT NULL DEFAULT '',
                 actor TEXT NOT NULL DEFAULT '',
                 prev_hash TEXT NOT NULL,
                 hash TEXT NOT NULL
             );",
        )
        .map_err(|error| audit_error(&format!("failed to create audit schema: {error}")))?;

    if old_exists {
        connection
            .execute_batch(
                "INSERT INTO audit_log_v2 (id, ts, op, connection_id, generation, path,
                    before_version, after_version, result, error_kind, detail, actor, prev_hash, hash)
                 SELECT id, ts,
                    CASE WHEN op = 'audit.clear' THEN 'clear' ELSE COALESCE(op, 'set') END,
                    COALESCE(connection_id, ''), COALESCE(generation, 0), COALESCE(path, ''),
                    COALESCE(before_version, -1), COALESCE(after_version, -1),
                    COALESCE(result, 'success'), COALESCE(error_kind, ''), COALESCE(detail, ''),
                    '', COALESCE(prev_hash, 'genesis'), COALESCE(hash, '')
                 FROM audit_log;",
            )
            .map_err(|error| audit_error(&format!("failed to copy audit rows: {error}")))?;
    }

    connection
        .execute_batch(
            "DROP TABLE IF EXISTS audit_log;
             ALTER TABLE audit_log_v2 RENAME TO audit_log;
             CREATE INDEX IF NOT EXISTS idx_audit_ts ON audit_log(ts);
             CREATE INDEX IF NOT EXISTS idx_audit_path ON audit_log(path);
             CREATE INDEX IF NOT EXISTS idx_audit_connection ON audit_log(connection_id);
             CREATE INDEX IF NOT EXISTS idx_audit_op ON audit_log(op);
             PRAGMA user_version = 2;
             COMMIT;",
        )
        .map_err(|error| audit_error(&format!("failed to apply audit schema: {error}")))?;

    if old_exists {
        recompute_chain(connection)?;
    }
    Ok(())
}

fn recompute_chain(connection: &Connection) -> AppResult<()> {
    let mut statement = connection
        .prepare(
            "SELECT id, ts, op, connection_id, generation, path, before_version, after_version,
                    result, error_kind, detail, actor, prev_hash, hash
             FROM audit_log ORDER BY id ASC",
        )
        .map_err(|error| audit_error(&format!("failed to prepare migration query: {error}")))?;
    let rows = statement
        .query_map([], map_record)
        .map_err(|error| audit_error(&format!("failed to read audit rows: {error}")))?;
    let mut records = Vec::new();
    for row in rows {
        records
            .push(row.map_err(|error| audit_error(&format!("failed to read audit row: {error}")))?);
    }
    drop(statement);

    let mut previous = GENESIS.to_string();
    for mut record in records {
        record.prev_hash = previous.clone();
        record.hash = compute_record_hash(&record);
        connection
            .execute(
                "UPDATE audit_log SET prev_hash = ?1, hash = ?2 WHERE id = ?3",
                params![record.prev_hash, record.hash, record.id],
            )
            .map_err(|error| audit_error(&format!("failed to rewrite audit hash: {error}")))?;
        previous = record.hash;
    }
    Ok(())
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
        store
            .record(
                AuditEvent::success(AuditOp::Set, "conn-1", 3, "digest:user", "/a")
                    .with_versions(1, 2),
            )
            .expect("record");
        store
            .record(AuditEvent::success(
                AuditOp::Create,
                "conn-1",
                3,
                "digest:user",
                "/b",
            ))
            .expect("record");
        let report = store.verify().expect("verify");
        assert!(report.ok);
        assert_eq!(report.checked, 2);
        let rows = store.list(10, 0, "").expect("list");
        assert_eq!(rows[0].op, AuditOp::Create);
        assert_eq!(rows[1].before_version, 1);
        assert_eq!(rows[1].after_version, 2);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn tampered_record_is_detected() {
        let (store, dir) = store();
        store
            .record(AuditEvent::success(
                AuditOp::Set,
                "conn-1",
                1,
                "digest:user",
                "/a",
            ))
            .expect("record");
        {
            let conn = store.connection.lock().unwrap();
            conn.execute("UPDATE audit_log SET path = '/tampered' WHERE id = 1", [])
                .expect("tamper");
        }
        let report = store.verify().expect("verify");
        assert!(!report.ok);
        assert_eq!(report.broken_at, 1);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn clear_resets_chain() {
        let (store, dir) = store();
        store
            .record(AuditEvent::success(
                AuditOp::Set,
                "conn-1",
                1,
                "digest:user",
                "/a",
            ))
            .expect("record");
        let cleared = store.clear().expect("clear");
        assert_eq!(cleared, 1);
        let report = store.verify().expect("verify");
        assert!(report.ok);
        assert_eq!(report.checked, 1);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn schema_is_versioned() {
        let (store, dir) = store();
        let version: i64 = store
            .connection
            .lock()
            .unwrap()
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .expect("version");
        assert_eq!(version, SCHEMA_VERSION);
        let _ = std::fs::remove_dir_all(dir);
    }
}
