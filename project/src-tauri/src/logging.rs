//! 文件日志：用户文档目录 + 简单脱敏。

use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::OnceLock;

use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::fmt::MakeWriter;

static GUARD: OnceLock<WorkerGuard> = OnceLock::new();

/// 初始化文件日志。返回是否成功；失败时不影响应用启动。
pub fn init(configured: &str) {
    let Some(directory) = log_dir(configured) else {
        return;
    };
    if std::fs::create_dir_all(&directory).is_err() {
        return;
    }
    let appender = tracing_appender::rolling::daily(&directory, "zkui.log");
    let (writer, guard) = tracing_appender::non_blocking(appender);
    let _ = GUARD.set(guard);
    let _ = tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(true)
        .with_writer(MakeRedacting { inner: writer })
        .try_init();
}

pub fn log_dir(configured: &str) -> Option<PathBuf> {
    use zkui_core::config::constants::{APP_DIR, LOGS_DIR};
    if !configured.trim().is_empty() {
        return Some(PathBuf::from(configured.trim()));
    }
    dirs::document_dir().map(|dir| dir.join(APP_DIR).join(LOGS_DIR))
}

struct MakeRedacting {
    inner: NonBlocking,
}

impl<'a> MakeWriter<'a> for MakeRedacting {
    type Writer = RedactingWriter<NonBlocking>;

    fn make_writer(&'a self) -> Self::Writer {
        RedactingWriter {
            inner: self.inner.clone(),
        }
    }
}

struct RedactingWriter<W: Write> {
    inner: W,
}

impl<W: Write> Write for RedactingWriter<W> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let text = String::from_utf8_lossy(buffer);
        self.inner.write_all(redact(&text).as_bytes())?;
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
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
                while cursor < text.len() && text[cursor..].starts_with(|c: char| c != '\n') {
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
