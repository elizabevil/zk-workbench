//! 连接方向命令。

import { call } from "./client";
import type { ConnectionSnapshot } from "./types/ConnectionSnapshot";
import type { CreateConnection } from "./types/CreateConnection";

export function connectionCreate(request: CreateConnection): Promise<ConnectionSnapshot> {
  return call<ConnectionSnapshot>("connection_create", { request });
}

export function connectionSnapshot(connectionId: string): Promise<ConnectionSnapshot> {
  return call<ConnectionSnapshot>("connection_snapshot", { connectionId });
}

export function connectionList(): Promise<ConnectionSnapshot[]> {
  return call<ConnectionSnapshot[]>("connection_list");
}

export function connectionClose(connectionId: string): Promise<void> {
  return call<void>("connection_close", { connectionId });
}
