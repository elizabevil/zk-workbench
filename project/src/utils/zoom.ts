//! WebView 缩放：字号设置与 Ctrl+滚轮共用同一档位。

export const MIN_ZOOM = 0.6;
export const MAX_ZOOM = 2.4;
export const ZOOM_STEP = 0.1;
export const BASE_FONT_SIZE = 14;

const STORAGE_KEY = "zkui.zoom";

export function clampZoom(value: number): number {
    if (!Number.isFinite(value)) {
        return 1;
    }
    return Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, Math.round(value * 100) / 100));
}

export function nextZoom(current: number, deltaY: number): number {
    return clampZoom(current + (deltaY < 0 ? ZOOM_STEP : -ZOOM_STEP));
}

/** 字号设置只改变基础比例，Ctrl+滚轮在其上叠加。 */
export function effectiveZoom(fontSize: number, uiZoom: number): number {
    return clampZoom((fontSize / BASE_FONT_SIZE) * clampZoom(uiZoom));
}

export function loadZoom(): number {
    try {
        const raw = localStorage.getItem(STORAGE_KEY);
        return raw === null ? 1 : clampZoom(Number(raw));
    } catch {
        return 1;
    }
}

export function saveZoom(value: number): void {
    try {
        localStorage.setItem(STORAGE_KEY, String(clampZoom(value)));
    } catch {
        // 本地存储不可用时只在当前会话生效。
    }
}
