<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { type InvokeRejection, toAppError } from "@/api/client";
import { COMMAND_NAMES, commandExecute, commandPreview } from "@/api/command";
import { nodeChildren } from "@/api/node";
import type { AppError } from "@/api/types/AppError";

const props = defineProps<{
  connectionId: string;
  connectionName: string;
}>();

const emit = defineEmits<{
  refresh: [];
}>();

const { t } = useI18n();
const input = ref("");
const output = ref("");
const running = ref(false);
const error = ref<AppError | null>(null);
const history = ref<string[]>([]);
const cursor = ref(-1);
const completion = ref<{ prefix: string; index: number } | null>(null);
const historyOpen = ref(false);

const canRun = computed(() => input.value.trim().length > 0 && !running.value);

function mask(line: string): string {
  return line.replace(/^(addauth\s+\S+\s+).+$/i, "$1***");
}

async function complete(): Promise<void> {
  const value = input.value;
  const lastSpace = Math.max(value.lastIndexOf(" "), value.lastIndexOf("\t"));
  const token = value.slice(lastSpace + 1);

  if (lastSpace < 0) {
    const candidates = COMMAND_NAMES.filter((name) => name.startsWith(token.toLowerCase()));
    if (candidates.length === 0) {
      return;
    }
    const index =
      completion.value?.prefix === token ? (completion.value.index + 1) % candidates.length : 0;
    completion.value = { prefix: token, index };
    input.value = `${candidates[index]} `;
    return;
  }

  const slash = token.lastIndexOf("/");
  const rawParent = slash >= 0 ? token.slice(0, slash) : "";
  const prefix = slash >= 0 ? token.slice(slash + 1) : token;
  const parent = rawParent === "" ? "/" : rawParent.startsWith("/") ? rawParent : `/${rawParent}`;
  try {
    const children = await nodeChildren(props.connectionId, parent);
    const candidates = children
      .filter((name) => name.startsWith(prefix))
      .map((name) => `${parent === "/" ? "" : parent}/${name}`);
    if (candidates.length === 0) {
      return;
    }
    const index =
      completion.value?.prefix === token ? (completion.value.index + 1) % candidates.length : 0;
    completion.value = { prefix: token, index };
    input.value = value.slice(0, lastSpace + 1) + candidates[index];
  } catch {
    // 补全失败不打断输入
  }
}

async function run(): Promise<void> {
  const line = input.value.trim();
  if (!line || running.value) {
    return;
  }
  running.value = true;
  error.value = null;
  output.value = "";
  try {
    const summary = await commandPreview(line);
    if (summary && !window.confirm(`${props.connectionName}\n${summary}`)) {
      return;
    }
    output.value = await commandExecute(props.connectionId, line);
    history.value = [mask(line), ...history.value.filter((item) => item !== mask(line))].slice(0, 100);
    cursor.value = -1;
    emit("refresh");
  } catch (raw) {
    error.value = toAppError(raw as InvokeRejection);
  } finally {
    running.value = false;
  }
}

function previous(): void {
  if (history.value.length === 0) {
    return;
  }
  cursor.value = Math.min(cursor.value + 1, history.value.length - 1);
  input.value = history.value[cursor.value] ?? "";
}

function next(): void {
  if (cursor.value <= 0) {
    cursor.value = -1;
    input.value = "";
    return;
  }
  cursor.value -= 1;
  input.value = history.value[cursor.value] ?? "";
}

function rerun(line: string): void {
  input.value = line;
  void run();
}
</script>

<template>
  <section class="command">
    <header class="head">
      <span>{{ t("command.title") }}</span>
      <div class="head-actions">
        <span v-if="error" class="error">{{ t(error.message_key) }}</span>
        <button type="button" class="tiny" @click="historyOpen = !historyOpen">
          {{ t("command.history") }} ({{ history.length }})
        </button>
      </div>
    </header>

    <div class="body">
      <pre class="output">{{ output || t("command.emptyOutput") }}</pre>
      <aside v-if="historyOpen" class="history">
        <span class="muted">{{ t("command.history") }}</span>
        <button
          v-for="(item, index) in history"
          :key="`${item}-${index}`"
          type="button"
          class="history-item"
          @click="rerun(item)"
        >
          {{ item }}
        </button>
      </aside>
    </div>

    <form class="input-row" @submit.prevent="run">
      <input
        v-model="input"
        :placeholder="t('command.placeholder')"
        :disabled="running"
        spellcheck="false"
        @keydown.up.prevent="previous"
        @keydown.down.prevent="next"
        @keydown.tab.prevent="complete"
      />
      <button :disabled="!canRun" type="submit">
        {{ running ? t("command.running") : t("command.run") }}
      </button>
    </form>
  </section>
</template>

<style scoped>
.command {
  display: grid;
  grid-template-rows: auto 1fr auto;
  gap: 6px;
  height: 24vh;
  min-height: 150px;
  padding: 8px 12px;
  border-top: 1px solid rgb(255 255 255 / 8%);
  background: rgb(10 13 16 / 92%);
}
.head-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}
.head {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #9aa4b2;
}
.body {
  display: flex;
  gap: 10px;
  min-height: 0;
}
.output {
  flex: 1;
  margin: 0;
  padding: 8px 10px;
  overflow: auto;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 85%);
  font-family: ui-monospace, monospace;
  font-size: 12px;
  white-space: pre-wrap;
}
.history {
  flex: 0 0 26%;
  display: flex;
  flex-direction: column;
  gap: 4px;
  overflow: auto;
  padding: 4px 6px;
  border-left: 1px solid rgb(255 255 255 / 8%);
}
.history-item {
  text-align: left;
  padding: 3px 6px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: #cbd5e1;
  font-family: ui-monospace, monospace;
  font-size: 11px;
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.history-item:hover {
  background: rgb(255 255 255 / 8%);
}
.input-row {
  display: flex;
  gap: 6px;
}
.input-row input {
  flex: 1;
  padding: 6px 9px;
  border-radius: 5px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}
button {
  padding: 6px 12px;
  border-radius: 5px;
  border: 1px solid #2a323c;
  background: #1d242c;
  color: inherit;
  cursor: pointer;
}
.muted {
  color: #9aa4b2;
  font-size: 11px;
}
.error {
  color: #ef4444;
}
</style>
