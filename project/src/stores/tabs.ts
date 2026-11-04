//! 工作区数据标签：按连接保存，默认只保留最近 3 个连接。

import { defineStore } from "pinia";
import { type Ref, ref } from "vue";

const MAX_CONNECTIONS = 3;

export interface TabsStoreSetup {
  byConnection: Ref<Record<string, string[]>>;
  order: Ref<string[]>;
  tabsFor: (connectionId: string) => string[];
  open: (connectionId: string, path: string) => void;
  close: (connectionId: string, path: string) => void;
  closeConnection: (connectionId: string) => void;
}

export const useTabsStore = defineStore("tabs", (): TabsStoreSetup => {
  const byConnection = ref<Record<string, string[]>>({});
  const order = ref<string[]>([]);

  function ensure(connectionId: string): void {
    if (order.value.includes(connectionId)) {
      return;
    }
    order.value.push(connectionId);
    while (order.value.length > MAX_CONNECTIONS) {
      const removed = order.value.shift();
      if (removed) {
        delete byConnection.value[removed];
      }
    }
  }

  function tabsFor(connectionId: string): string[] {
    return byConnection.value[connectionId] ?? [];
  }

  function open(connectionId: string, path: string): void {
    ensure(connectionId);
    const tabs = byConnection.value[connectionId] ?? [];
    if (!tabs.includes(path)) {
      tabs.push(path);
      byConnection.value[connectionId] = tabs;
    }
  }

  function close(connectionId: string, path: string): void {
    const tabs = byConnection.value[connectionId];
    if (!tabs) {
      return;
    }
    byConnection.value[connectionId] = tabs.filter((item) => item !== path);
  }

  function closeConnection(connectionId: string): void {
    delete byConnection.value[connectionId];
    order.value = order.value.filter((item) => item !== connectionId);
  }

  return { byConnection, order, tabsFor, open, close, closeConnection };
});
