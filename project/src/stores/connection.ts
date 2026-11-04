//! 连接相关的 UI 状态。远端数据以 Rust 为准，这里只保存展示所需快照。

import { defineStore } from "pinia";
import { type Ref, ref } from "vue";

import { type InvokeRejection, toAppError } from "@/api/client";
import { connectionClose, connectionCreate, connectionList } from "@/api/connection";
import type { AppError } from "@/api/types/AppError";
import type { ConnectionSnapshot } from "@/api/types/ConnectionSnapshot";
import type { ConnectionStateEvent } from "@/api/types/ConnectionStateEvent";

export interface ConnectInput {
  host: string;
  port: number;
  displayName: string;
}

export interface ConnectionStoreSetup {
  connections: Ref<ConnectionSnapshot[]>;
  snapshot: Ref<ConnectionSnapshot | null>;
  error: Ref<AppError | null>;
  loading: Ref<boolean>;
  refresh: () => Promise<void>;
  connect: (input: ConnectInput) => Promise<ConnectionSnapshot | null>;
  activate: (connectionId: string) => void;
  close: (connectionId?: string) => Promise<void>;
  applyEvent: (event: ConnectionStateEvent) => void;
}

export const useConnectionStore = defineStore("connection", (): ConnectionStoreSetup => {
  const connections = ref<ConnectionSnapshot[]>([]);
  const snapshot = ref<ConnectionSnapshot | null>(null);
  const error = ref<AppError | null>(null);
  const loading = ref(false);

  function replaceLocal(next: ConnectionSnapshot): void {
    const index = connections.value.findIndex((item) => item.connection_id === next.connection_id);
    if (index >= 0) {
      connections.value[index] = next;
    } else {
      connections.value.push(next);
    }
    connections.value.sort((a, b) => a.connection_id.localeCompare(b.connection_id));
  }

  async function refresh(): Promise<void> {
    try {
      connections.value = await connectionList();
      if (snapshot.value) {
        const current = connections.value.find(
          (item) => item.connection_id === snapshot.value?.connection_id,
        );
        snapshot.value = current ?? null;
      }
    } catch (raw) {
      error.value = toAppError(raw as InvokeRejection);
    }
  }

  async function connect(input: ConnectInput): Promise<ConnectionSnapshot | null> {
    loading.value = true;
    error.value = null;
    try {
      const created = await connectionCreate({
        host: input.host,
        port: input.port,
        display_name: input.displayName,
        identity: null,
        proxy: null,
      });
      replaceLocal(created);
      snapshot.value = created;
      return created;
    } catch (raw) {
      error.value = toAppError(raw as InvokeRejection);
      return null;
    } finally {
      loading.value = false;
    }
  }

  function activate(connectionId: string): void {
    snapshot.value = connections.value.find((item) => item.connection_id === connectionId) ?? null;
  }

  async function close(connectionId?: string): Promise<void> {
    const target = connectionId ?? snapshot.value?.connection_id;
    if (!target) {
      return;
    }
    try {
      await connectionClose(target);
      connections.value = connections.value.filter((item) => item.connection_id !== target);
      if (snapshot.value?.connection_id === target) {
        snapshot.value = connections.value[0] ?? null;
      }
    } catch (raw) {
      error.value = toAppError(raw as InvokeRejection);
    }
  }

  function applyEvent(event: ConnectionStateEvent): void {
    const apply = (item: ConnectionSnapshot): ConnectionSnapshot =>
      item.connection_id === event.connection_id
        ? { ...item, state: event.state, generation: event.generation }
        : item;
    connections.value = connections.value.map(apply);
    if (snapshot.value?.connection_id === event.connection_id) {
      snapshot.value = apply(snapshot.value);
    }
  }

  return { connections, snapshot, error, loading, refresh, connect, activate, close, applyEvent };
});
