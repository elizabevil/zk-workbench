//! 事件订阅。事件名与 Rust 侧常量保持一致。

import {listen, type UnlistenFn} from "@tauri-apps/api/event";
import type {ConnectionStateEvent} from "./types/ConnectionStateEvent";

export const CONNECTION_STATE_CHANGED = "connection.state.changed";
export const MENU_ACTION = "menu.action";
export const MENU_ACTION_SETTINGS = "settings";
export const MENU_ACTION_ABOUT = "about";

export function listenConnectionState(
    handler: (event: ConnectionStateEvent) => void,
): Promise<UnlistenFn> {
    return listen<ConnectionStateEvent>(CONNECTION_STATE_CHANGED, (event) => handler(event.payload));
}
