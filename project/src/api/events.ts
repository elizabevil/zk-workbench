//! 事件订阅。事件名与 Rust 侧常量保持一致。

import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { ConnectionStateEvent } from "./types/ConnectionStateEvent";

export const CONNECTION_STATE_CHANGED = "connection.state.changed";

export function listenConnectionState(
  handler: (event: ConnectionStateEvent) => void,
): Promise<UnlistenFn> {
  return listen<ConnectionStateEvent>(CONNECTION_STATE_CHANGED, (event) => handler(event.payload));
}
