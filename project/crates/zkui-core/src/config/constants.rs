//! 集中常量：避免在代码里散落字面量。

/// 系统凭据存储的服务名。
pub const KEYRING_SERVICE: &str = "zkui";
/// 代理密码的账号前缀。
pub const PROXY_ACCOUNT_PREFIX: &str = "proxy:";
/// 默认主题名。
pub const DEFAULT_THEME: &str = "compact";
/// 日志级别默认值。
pub const DEFAULT_LOG_LEVEL: &str = "info";
/// OTel 协议：仅支持 http/protobuf。
pub const OTEL_PROTOCOL_HTTP_PROTOBUF: &str = "http/protobuf";
/// 桌面通知默认级别。
pub const NOTIFICATION_ERRORS: &str = "errors";
/// 应用数据目录名（配置、日志、审计共用）。
pub const APP_DIR: &str = "zkui";
/// 日志子目录名。
pub const LOGS_DIR: &str = "logs";
