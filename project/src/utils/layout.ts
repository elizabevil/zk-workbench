//! 工作区布局尺寸：命令区高度以百分比表示。

export const DEFAULT_COMMAND_HEIGHT = 40;
export const MIN_COMMAND_HEIGHT = 20;
export const MAX_COMMAND_HEIGHT = 75;

const STORAGE_KEY = "zkui.command.height";

export function clampCommandHeight(value: number): number {
    if (!Number.isFinite(value)) {
        return DEFAULT_COMMAND_HEIGHT;
    }
    return Math.min(MAX_COMMAND_HEIGHT, Math.max(MIN_COMMAND_HEIGHT, Math.round(value)));
}

export function loadCommandHeight(): number {
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        return raw === null ? DEFAULT_COMMAND_HEIGHT : clampCommandHeight(Number(raw));
    } catch {
        return DEFAULT_COMMAND_HEIGHT;
    }
}

export function saveCommandHeight(value: number): void {
    try {
        localStorage.setItem(STORAGE_KEY, String(clampCommandHeight(value)));
    } catch {
        // 本地存储不可用时只在当前会话生效。
    }
}
