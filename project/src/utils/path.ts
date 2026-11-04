//! 节点路径规范化：`/a/`、`//a` 与 `/a` 视为同一节点。

export function normalizePath(raw: string): string {
    let value = raw.trim();
    if (!value) {
        return "/";
    }
    if (!value.startsWith("/")) {
        value = `/${value}`;
    }
    value = value.replace(/\/{2,}/g, "/");
    if (value.length > 1) {
        value = value.replace(/\/+$/, "");
    }
    return value;
}
