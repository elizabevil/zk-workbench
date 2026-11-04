<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { type InvokeRejection, toAppError } from "@/api/client";
import { commandExecute, commandPreview } from "@/api/command";
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

const canRun = computed(() => input.value.trim().length > 0 && !running.value);

function mask(line: string): string {
  return line.replace(/^(addauth\s+\S+\s+).+$/i, "$1***");
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
      <span v-if="error" class="error">{{ t(error.message_key) }}</span>
    </header>
    <pre v-if="output" class="output">{{ output }}</pre>
    <form class="input-row" @submit.prevent="run">
      <input
        v-model="input"
        :placeholder="t('command.placeholder')"
        :disabled="running"
        spellcheck="false"
        @keydown.up.prevent="previous"
        @keydown.down.prevent="next"
      />
      <button :disabled="!canRun" type="submit">
        {{ running ? t("command.running") : t("command.run") }}
      </button>
    </form>
    <div v-if="history.length" class="history">
      <span class="muted">{{ t("command.history") }}</span>
      <button
        v-for="(item, index) in history.slice(0, 8)"
        :key="`${item}-${index}`"
        type="button"
        class="history-item"
        @click="rerun(item)"
      >
        {{ item }}
      </button>
    </div>
  </section>
</template>

<style scoped>
.command {
  display: flex;
  flex-direction: column;
  gap: 6px;
  height: 190px;
  padding: 8px 12px;
  border-top: 1px solid rgb(255 255 255 / 8%);
  background: rgb(10 13 16 / 90%);
}
.head {
  display: flex;
  justify-content: space-between;
  font-size: 12px;
  color: #9aa4b2;
}
.output {
  flex: 1;
  margin: 0;
  padding: 8px;
  overflow: auto;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 80%);
  font-family: ui-monospace, monospace;
  font-size: 12px;
  white-space: pre-wrap;
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
.history {
  display: flex;
  align-items: center;
  gap: 6px;
  overflow: auto;
  font-size: 12px;
}
.history-item {
  padding: 2px 8px;
  white-space: nowrap;
  font-family: ui-monospace, monospace;
  font-size: 11px;
}
.muted {
  color: #9aa4b2;
  white-space: nowrap;
}
.error {
  color: #ef4444;
}
</style>
