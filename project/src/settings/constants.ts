//! 设置候选项常量，避免在模板里散落字面量。

import type {NotificationLevel} from "@/api/types/NotificationLevel";

export const NOTIFICATION_LEVELS: readonly NotificationLevel[] = [
    "off",
    "errors",
    "warnings",
    "all",
];
export const PROXY_KINDS = ["socks5", "http_connect"] as const;
export const THEME_PRESETS = ["compact", "comfortable"] as const;
