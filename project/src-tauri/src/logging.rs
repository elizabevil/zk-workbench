//! 文件日志：按等级拆分、按大小轮转、按保留期清理。

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, SystemTime};

use tracing::{Level, Metadata};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::Layer;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static GUARDS: OnceLock<Vec<WorkerGuard>> = OnceLock::new();

/// 初始化文件日志。失败时不影响应用启动。
pub fn init(configured: &str, max_size_bytes: u64, retention_days: u32) {
    let Some(directory) = log_dir(configured) else {
        return;
    };
    if std::fs::create_dir_all(&directory).is_err() {
        return;
    }
    cleanup(&directory, retention_days);

    let max = max_size_bytes.max(4096);
    let mut guards = Vec::new();
    let mut open = |name: &str| {
        let path = directory.join(name);
        let writer = RotatingFile::new(path, max).ok()?;
        let (non_blocking, guard) = tracing_appender::non_blocking(writer);
        guards.push(guard);
        Some(non_blocking)
    };

    let (Some(error), Some(warn), Some(info), Some(debug)) = (
        open("error.log"),
        open("warn.log"),
        open("info.log"),
        open("debug.log"),
    ) else {
        return;
    };

    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_writer(error)
                .with_filter(filter_fn(|meta: &Metadata<'_>| {
                    *meta.level() == Level::ERROR
                })),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_writer(warn)
                .with_filter(filter_fn(|meta: &Metadata<'_>| {
                    *meta.level() == Level::WARN
                })),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_writer(info)
                .with_filter(filter_fn(|meta: &Metadata<'_>| {
                    *meta.level() == Level::INFO
                })),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_writer(debug)
                .with_filter(filter_fn(|meta: &Metadata<'_>| {
                    *meta.level() == Level::DEBUG || *meta.level() == Level::TRACE
                })),
        )
        .try_init();
    let _ = GUARDS.set(guards);
}

pub fn log_dir(configured: &str) -> Option<PathBuf> {
    use zkui_core::config::constants::{APP_DIR, LOGS_DIR};
    if !configured.trim().is_empty() {
        return Some(PathBuf::from(configured.trim()));
    }
    dirs::document_dir().map(|dir| dir.join(APP_DIR).join(LOGS_DIR))
}

struct RotatingFile {
    path: PathBuf,
    file: File,
    size: u64,
    max: u64,
}

impl RotatingFile {
    fn new(path: PathBuf, max: u64) -> io::Result<Self> {
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        let size = file.metadata()?.len();
        Ok(Self {
            path,
            file,
            size,
            max,
        })
    }

    fn rotate(&mut self) -> io::Result<()> {
        self.file.flush()?;
        let backup = self.path.with_extension("log.1");
        let _ = std::fs::remove_file(&backup);
        std::fs::rename(&self.path, &backup)?;
        self.file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;
        self.size = 0;
        Ok(())
    }
}

impl Write for RotatingFile {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let text = String::from_utf8_lossy(buffer);
        let redacted = redact(&text);
        let bytes = redacted.as_bytes();
        if self.size + bytes.len() as u64 > self.max {
            self.rotate()?;
        }
        self.file.write_all(bytes)?;
        self.size += bytes.len() as u64;
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

fn cleanup(directory: &Path, retention_days: u32) {
    if retention_days == 0 {
        return;
    }
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(
            u64::from(retention_days) * 24 * 60 * 60,
        ))
        .unwrap_or(SystemTime::UNIX_EPOCH);
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let ours = ["error.log", "warn.log", "info.log", "debug.log"]
            .iter()
            .any(|prefix| name.starts_with(prefix));
        if !ours {
            continue;
        }
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        let Ok(modified) = metadata.modified() else {
            continue;
        };
        if modified < cutoff {
            let _ = std::fs::remove_file(path);
        }
    }
}

/// 对常见敏感字段做掩码：`password=`、`token=`、`secret=`、`authorization:`。
pub fn redact(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let lower = text.to_ascii_lowercase();
    let keys = ["password", "token", "secret", "authorization"];
    let mut index = 0;
    while index < text.len() {
        let rest = &lower[index..];
        if let Some((key_len, separator)) = keys.iter().find_map(|key| {
            if rest.starts_with(key) {
                let after = index + key.len();
                let sep = lower[after..].trim_start();
                if sep.starts_with('=') || sep.starts_with(':') {
                    return Some((key.len(), sep.chars().next()?));
                }
            }
            None
        }) {
            let start = index + key_len;
            output.push_str(&text[index..start]);
            output.push(separator);
            let mut cursor = start + 1;
            let mut spaced = false;
            while cursor < text.len() && text[cursor..].starts_with(char::is_whitespace) {
                cursor += 1;
                spaced = true;
            }
            if spaced {
                output.push(' ');
            }
            while cursor < text.len()
                && !text[cursor..]
                    .starts_with(|c: char| c.is_whitespace() || c == '"' || c == ',' || c == '&')
            {
                cursor += 1;
            }
            output.push_str("***");
            if text[index..]
                .to_ascii_lowercase()
                .starts_with("authorization")
            {
                while cursor < text.len() && !text[cursor..].starts_with('\n') {
                    cursor += 1;
                }
            }
            index = cursor;
        } else {
            let ch = text[index..].chars().next().expect("char");
            output.push(ch);
            index += ch.len_utf8();
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::redact;

    #[test]
    fn masks_sensitive_values() {
        assert_eq!(
            redact("password=secret123 token: abc"),
            "password=*** token: ***"
        );
        assert_eq!(redact("authorization: Bearer xyz"), "authorization: ***");
    }

    #[test]
    fn leaves_normal_text() {
        assert_eq!(
            redact("connected to 127.0.0.1:2181"),
            "connected to 127.0.0.1:2181"
        );
    }
}
