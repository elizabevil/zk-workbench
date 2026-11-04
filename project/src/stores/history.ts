//! 命令历史：会话内共享，供命令面板与右侧历史栏使用。

import { defineStore } from "pinia";
import { type Ref, ref } from "vue";

export interface CommandHistoryEntry {
  line: string;
  ts: number;
  ok: boolean;
}

export interface HistoryStoreSetup {
  entries: Ref<CommandHistoryEntry[]>;
  draft: Ref<string>;
  push: (line: string, ok: boolean) => void;
  setDraft: (line: string) => void;
  clear: () => void;
}

export const useHistoryStore = defineStore("history", (): HistoryStoreSetup => {
  const entries = ref<CommandHistoryEntry[]>([]);
  const draft = ref("");

  function push(line: string, ok: boolean): void {
    entries.value = [{ line, ts: Date.now(), ok }, ...entries.value].slice(0, 200);
  }

  function setDraft(line: string): void {
    draft.value = line;
  }

  function clear(): void {
    entries.value = [];
  }

  return { entries, draft, push, setDraft, clear };
});
