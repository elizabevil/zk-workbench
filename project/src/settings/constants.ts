//! 设置候选项常量，避免在模板里散落字面量。

export const NOTIFICATION_LEVELS = ["off", "errors", "warnings", "all"] as const;
export const PROXY_KINDS = ["socks5", "http_connect"] as const;
export const THEME_PRESETS = ["compact", "comfortable"] as const;

export type NotificationLevel = (typeof NOTIFICATION_LEVELS)[number];
