//! 连接相关的 UI 状态。远端数据以 Rust 为准，这里只保存展示所需快照。

import { defineStore } from "pinia";
import { ref } from "vue";

import { connectionClose, connectionCreate } from "../api/connection";
import type { AppError } from "../api/types/AppError";
import type { ConnectionSnapshot } from "../api/types/ConnectionSnapshot";
import type { ConnectionStateEvent } from "../api/types/ConnectionStateEvent";

export const useConnectionStore = defineStore("connection", () => {
  const snapshot = ref<ConnectionSnapshot | null>(null);
  const error = ref<AppError | null>(null);
  const loading = ref(false);

  async function connect(endpoint: string, displayName: string): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      snapshot.value = await connectionCreate({
        endpoint,
        display_name: displayName,
        identity: null,
      });
    } catch (raw) {
      error.value = raw as AppError;
    } finally {
      loading.value = false;
    }
  }

  async function close(): Promise<void> {
    if (!snapshot.value) {
      return;
    }
    try {
      await connectionClose(snapshot.value.connection_id);
      snapshot.value = null;
    } catch (raw) {
      error.value = raw as AppError;
    }
  }

  function applyEvent(event: ConnectionStateEvent): void {
    if (!snapshot.value || snapshot.value.connection_id !== event.connection_id) {
      return;
    }
    snapshot.value = {
      ...snapshot.value,
      state: event.state,
      generation: event.generation,
    };
  }

  return { snapshot, error, loading, connect, close, applyEvent };
});
