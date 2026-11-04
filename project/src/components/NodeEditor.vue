<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import { type InvokeRejection, toAppError } from "@/api/client";
import { nodeSet } from "@/api/node";
import type { AppError } from "@/api/types/AppError";
import type { NodeStat } from "@/api/types/NodeStat";
import { diffLines } from "@/utils/diff";
import { encodeUtf8Base64 } from "@/utils/encoding";

const props = defineProps<{
  open: boolean;
  connectionId: string;
  path: string;
  originalText: string;
  version: number;
}>();

const emit = defineEmits<{
  close: [];
  saved: [stat: NodeStat];
  reload: [];
}>();

const { t } = useI18n();
const draft = ref("");
const showDiff = ref(false);
const saving = ref(false);
const error = ref<AppError | null>(null);
const find = ref("");
const replaceWith = ref("");
const editorEl = ref<HTMLTextAreaElement | null>(null);
const undoSnapshot = ref<string | null>(null);

watch(
  () => props.open,
  (open) => {
    if (open) {
      draft.value = props.originalText;
      showDiff.value = false;
      error.value = null;
    }
  },
);

const diff = computed(() => diffLines(props.originalText, draft.value));
const changed = computed(() => diff.value.some((line) => line.kind !== "same"));
const matchCount = computed(() => {
  if (!find.value) {
    return 0;
  }
  return draft.value.split(find.value).length - 1;
});

function findNext(): void {
  const element = editorEl.value;
  if (!element || !find.value) {
    return;
  }
  const start = element.selectionEnd;
  const index = draft.value.indexOf(find.value, start);
  const position = index >= 0 ? index : draft.value.indexOf(find.value);
  if (position < 0) {
    return;
  }
  element.focus();
  element.setSelectionRange(position, position + find.value.length);
}

function replaceCurrent(): void {
  const element = editorEl.value;
  if (!element || !find.value) {
    return;
  }
  const selected = element.value.slice(element.selectionStart, element.selectionEnd);
  const start = selected === find.value ? element.selectionStart : element.selectionStart;
  const index = selected === find.value ? start : draft.value.indexOf(find.value, start);
  if (index < 0) {
    return;
  }
  draft.value = draft.value.slice(0, index) + replaceWith.value + draft.value.slice(index + find.value.length);
  findNext();
}

function replaceAll(): void {
  if (!find.value) {
    return;
  }
  if (!window.confirm(t("editor.confirmReplaceAll", { count: matchCount.value }))) {
    return;
  }
  undoSnapshot.value = draft.value;
  draft.value = draft.value.split(find.value).join(replaceWith.value);
}

function undoReplace(): void {
  if (undoSnapshot.value === null) {
    return;
  }
  draft.value = undoSnapshot.value;
  undoSnapshot.value = null;
}

async function save(): Promise<void> {
  saving.value = true;
  error.value = null;
  try {
    const stat = await nodeSet(
      props.connectionId,
      props.path,
      encodeUtf8Base64(draft.value),
      props.version,
    );
    emit("saved", stat);
  } catch (raw) {
    error.value = toAppError(raw as InvokeRejection);
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="overlay">
      <div class="dialog">
        <header class="head">
          <h2>{{ t("editor.title") }}</h2>
          <span class="path">{{ path }}</span>
        </header>

        <template v-if="!showDiff">
          <div class="find-bar">
            <input v-model="find" :placeholder="t('editor.find')" @keydown.enter.prevent="findNext" />
            <button type="button" class="tiny" :disabled="!find" @click="findNext">
              {{ t("editor.next") }} ({{ matchCount }})
            </button>
            <input v-model="replaceWith" :placeholder="t('editor.replaceWith')" />
            <button type="button" class="tiny" :disabled="!find" @click="replaceCurrent">
              {{ t("editor.replace") }}
            </button>
            <button type="button" class="tiny" :disabled="!find" @click="replaceAll">
              {{ t("editor.replaceAll") }}
            </button>
            <button
              type="button"
              class="tiny"
              :disabled="undoSnapshot === null"
              @click="undoReplace"
            >
              {{ t("editor.undo") }}
            </button>
          </div>
          <textarea ref="editorEl" v-model="draft" class="editor" spellcheck="false" />
          <p class="muted">{{ t("editor.versionHint", { version }) }}</p>
        </template>

        <div v-else class="diff">
          <div
            v-for="(line, index) in diff"
            :key="index"
            class="diff-line"
            :data-kind="line.kind"
          >
            <span class="mark">{{ line.kind === "add" ? "+" : line.kind === "remove" ? "-" : " " }}</span>
            <span>{{ line.text }}</span>
          </div>
        </div>

        <p v-if="error" class="error">
          {{ t(error.message_key) }}
          <span v-if="error.fields.detail">（{{ error.fields.detail }}）</span>
        </p>

        <footer class="actions">
          <button type="button" @click="emit('close')">{{ t("editor.cancel") }}</button>
          <button v-if="showDiff" type="button" @click="showDiff = false">
            {{ t("editor.back") }}
          </button>
          <button
            v-if="!showDiff"
            type="button"
            class="primary"
            :disabled="!changed"
            @click="showDiff = true"
          >
            {{ t("editor.preview") }}
          </button>
          <button
            v-else
            type="button"
            class="primary"
            :disabled="saving"
            @click="save"
          >
            {{ saving ? t("editor.saving") : t("editor.confirm") }}
          </button>
          <button v-if="error" type="button" @click="emit('reload')">
            {{ t("editor.reload") }}
          </button>
        </footer>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  display: grid;
  place-items: center;
  background: rgb(0 0 0 / 55%);
  z-index: 90;
}
.dialog {
  width: min(820px, 94vw);
  max-height: 86vh;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 18px 20px;
  border-radius: 10px;
  background: #171c22;
  border: 1px solid #2a323c;
}
.head {
  display: flex;
  align-items: baseline;
  gap: 12px;
}
h2 {
  margin: 0;
  font-size: 17px;
}
.path {
  color: #9aa4b2;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}
.find-bar {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}
.find-bar input {
  padding: 5px 8px;
  border-radius: 5px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
  font-size: 12px;
}
.tiny {
  padding: 4px 8px;
  border-radius: 5px;
  border: 1px solid #2a323c;
  background: #1d242c;
  color: #cbd5e1;
  cursor: pointer;
  font-size: 12px;
}
.tiny:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}
.editor {
  min-height: 300px;
  max-height: 52vh;
  padding: 10px 12px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
  font-family: ui-monospace, monospace;
  font-size: 13px;
  resize: vertical;
}
.diff {
  max-height: 52vh;
  overflow: auto;
  border: 1px solid #2a323c;
  border-radius: 6px;
  background: #101418;
  padding: 8px 10px;
  font-family: ui-monospace, monospace;
  font-size: 13px;
  white-space: pre-wrap;
}
.diff-line {
  display: flex;
  gap: 8px;
}
.diff-line[data-kind="add"] {
  color: #4ade80;
}
.diff-line[data-kind="remove"] {
  color: #f87171;
}
.mark {
  width: 10px;
  color: #9aa4b2;
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
button {
  padding: 7px 13px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #1d242c;
  color: inherit;
  cursor: pointer;
}
.primary {
  background: #2563eb;
  border-color: #2563eb;
}
.muted {
  color: #9aa4b2;
  font-size: 12px;
  margin: 0;
}
.error {
  color: #ef4444;
  margin: 0;
  font-size: 13px;
}
</style>
