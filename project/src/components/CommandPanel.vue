<script lang="ts" setup>
import {computed, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {type InvokeRejection, toAppError} from "@/api/client";
import {COMMAND_NAMES, commandExecute, commandPreview} from "@/api/command";
import {nodeChildren} from "@/api/node";
import type {AppError} from "@/api/types/AppError";
import {useHistoryStore} from "@/stores/history";

const props = defineProps<{
  connectionId: string;
  connectionName: string;
}>();

const emit = defineEmits<{
  refresh: [];
}>();

const {t} = useI18n();
const input = ref("");
const output = ref("");
const running = ref(false);
const error = ref<AppError | null>(null);
const history = useHistoryStore();
const cursor = ref(-1);
const completion = ref<{ prefix: string; index: number } | null>(null);

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
    completion.value = {prefix: token, index};
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
    completion.value = {prefix: token, index};
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
    history.push(mask(line), true);
    cursor.value = -1;
    emit("refresh");
  } catch (raw) {
    const failure = toAppError(raw as InvokeRejection);
    error.value = failure;
    const detail = failure.fields.detail ? `: ${failure.fields.detail}` : "";
    output.value = `${t(failure.message_key)}${detail}`;
    history.push(mask(line), false);
  } finally {
    running.value = false;
  }
}

watch(
    () => history.draft,
    (line) => {
      if (line) {
        input.value = line;
        history.setDraft("");
        void run();
      }
    },
);

function previous(): void {
  const items = history.entries;
  if (items.length === 0) {
    return;
  }
  cursor.value = Math.min(cursor.value + 1, items.length - 1);
  input.value = items[cursor.value]?.line ?? "";
}

function next(): void {
  if (cursor.value <= 0) {
    cursor.value = -1;
    input.value = "";
    return;
  }
  cursor.value -= 1;
  input.value = history.entries[cursor.value]?.line ?? "";
}
</script>

<template>
  <section class="command">
    <header class="head">
      <span>{{ t("command.title") }}</span>

    </header>

    <div class="body">
      <pre class="output" :class="{ 'output-error': error }">{{ output || t("command.emptyOutput") }}</pre>
    </div>

    <form class="input-row" @submit.prevent="run">
      <input
          v-model="input"
          :disabled="running"
          :placeholder="t('command.placeholder')"
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
  box-sizing: border-box;
  display: grid;
  grid-template-rows: auto 1fr auto;
  gap: 6px;
  width: 100%;
  height: 100%;
  min-height: 0;
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
  font-size: 12.5px;
  line-height: 1.55;
  white-space: pre-wrap;
}

.output-error {
  border-color: rgb(248 113 113 / 35%);
  color: #f87171;
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
</style>
