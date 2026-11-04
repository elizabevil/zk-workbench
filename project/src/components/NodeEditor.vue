<script lang="ts" setup>
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {KEY_ESCAPE} from "@/constants/keyboard";

import {type InvokeRejection, toAppError} from "@/api/client";
import {nodeSet} from "@/api/node";
import type {AppError} from "@/api/types/AppError";
import type {NodeStat} from "@/api/types/NodeStat";
import {diffLines} from "@/utils/diff";
import {encodeUtf8Base64} from "@/utils/encoding";

type LineKind = "same" | "add" | "change";

interface SearchMatch {
  index: number;
  length: number;
}

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

const {t} = useI18n();
const draft = ref("");
const saving = ref(false);
const error = ref<AppError | null>(null);
const find = ref("");
const replaceWith = ref("");
const editorEl = ref<HTMLTextAreaElement | null>(null);
const findInput = ref<HTMLInputElement | null>(null);
const undoSnapshot = ref<string | null>(null);
const findOpen = ref(false);
const replaceOpen = ref(false);
const matchCase = ref(false);
const wholeWord = ref(false);
const regexMode = ref(false);
const currentMatch = ref(-1);
const scrollTop = ref(0);
const scrollLeft = ref(0);

watch(
    () => props.open,
    (open) => {
      if (open) {
        draft.value = props.originalText;
        error.value = null;
        findOpen.value = false;
        replaceOpen.value = false;
        currentMatch.value = -1;
        undoSnapshot.value = null;
        scrollTop.value = 0;
        scrollLeft.value = 0;
      }
    },
);

const changed = computed(() => draft.value !== props.originalText);
const diff = computed(() => (changed.value ? diffLines(props.originalText, draft.value) : []));
const addedCount = computed(() => diff.value.filter((line) => line.kind === "add").length);
const removedCount = computed(() => diff.value.filter((line) => line.kind === "remove").length);
const draftLines = computed(() => draft.value.split("\n"));

const lineKinds = computed<LineKind[]>(() => {
  const kinds = draftLines.value.map(() => "same" as LineKind);
  if (!changed.value) {
    return kinds;
  }
  let index = 0;
  let removed = false;
  for (const entry of diff.value) {
    if (entry.kind === "same") {
      if (removed && index < kinds.length) {
        kinds[index] = "change";
      }
      removed = false;
      index += 1;
    } else if (entry.kind === "add") {
      if (index < kinds.length) {
        kinds[index] = removed ? "change" : "add";
      }
      removed = false;
      index += 1;
    } else {
      removed = true;
    }
  }
  return kinds;
});

const gutterLines = computed(() =>
    draftLines.value.map((_, index) => ({
      number: index + 1,
      kind: lineKinds.value[index] ?? "same",
    })),
);

function searchPattern(): { re: RegExp | null; invalid: boolean } {
  if (!find.value) {
    return {re: null, invalid: false};
  }
  let source = regexMode.value ? find.value : find.value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  if (wholeWord.value) {
    source = `\\b(?:${source})\\b`;
  }
  try {
    return {re: new RegExp(source, matchCase.value ? "g" : "gi"), invalid: false};
  } catch {
    return {re: null, invalid: true};
  }
}

const searchInvalid = computed(() => searchPattern().invalid);

const matches = computed<SearchMatch[]>(() => {
  const {re} = searchPattern();
  if (!re) {
    return [];
  }
  const list: SearchMatch[] = [];
  re.lastIndex = 0;
  let result: RegExpExecArray | null;
  while ((result = re.exec(draft.value)) !== null) {
    list.push({index: result.index, length: result[0].length});
    if (result[0].length === 0) {
      re.lastIndex += 1;
    }
    if (list.length >= 5000) {
      break;
    }
  }
  return list;
});

const resultLabel = computed(() => {
  if (!find.value || searchInvalid.value || matches.value.length === 0) {
    return t("editor.noResult");
  }
  const current = currentMatch.value >= 0 ? currentMatch.value + 1 : 0;
  return `${current}/${matches.value.length}`;
});

function syncScroll(): void {
  const element = editorEl.value;
  if (!element) {
    return;
  }
  scrollTop.value = element.scrollTop;
  scrollLeft.value = element.scrollLeft;
}

function selectMatch(index: number): void {
  const match = matches.value[index];
  if (!match) {
    currentMatch.value = -1;
    return;
  }
  currentMatch.value = index;
  const element = editorEl.value;
  if (!element) {
    return;
  }
  element.setSelectionRange(match.index, match.index + match.length);
  const line = draft.value.slice(0, match.index).split("\n").length - 1;
  element.scrollTop = Math.max(0, (line - 2) * 20);
  syncScroll();
}

function findNext(): void {
  if (matches.value.length === 0) {
    return;
  }
  const next = currentMatch.value < 0 ? 0 : (currentMatch.value + 1) % matches.value.length;
  selectMatch(next);
}

function findPrev(): void {
  if (matches.value.length === 0) {
    return;
  }
  const prev =
      currentMatch.value <= 0 ? matches.value.length - 1 : (currentMatch.value - 1) % matches.value.length;
  selectMatch(prev);
}

function openFind(withReplace: boolean): void {
  if (!props.open) {
    return;
  }
  findOpen.value = true;
  replaceOpen.value = withReplace;
  const element = editorEl.value;
  const selected = element ? element.value.slice(element.selectionStart, element.selectionEnd) : "";
  if (selected && !selected.includes("\n")) {
    find.value = selected;
  }
  nextTick(() => {
    findInput.value?.focus();
    findInput.value?.select();
    if (find.value && matches.value.length > 0) {
      selectMatch(currentMatch.value >= 0 ? currentMatch.value : 0);
    }
  });
}

function closeFind(): void {
  findOpen.value = false;
  replaceOpen.value = false;
  currentMatch.value = -1;
  nextTick(() => editorEl.value?.focus());
}

function replaceCurrent(): void {
  if (!find.value || matches.value.length === 0) {
    return;
  }
  const index = currentMatch.value >= 0 ? currentMatch.value : 0;
  const match = matches.value[index];
  if (!match) {
    return;
  }
  undoSnapshot.value = draft.value;
  draft.value =
      draft.value.slice(0, match.index) + replaceWith.value + draft.value.slice(match.index + match.length);
  nextTick(() => {
    const position = match.index + replaceWith.value.length;
    let nextIndex = matches.value.findIndex((item) => item.index >= position);
    if (nextIndex < 0) {
      nextIndex = 0;
    }
    if (matches.value.length === 0) {
      currentMatch.value = -1;
    } else {
      selectMatch(nextIndex);
    }
  });
}

function replaceAll(): void {
  if (!find.value || matches.value.length === 0) {
    return;
  }
  if (!window.confirm(t("editor.confirmReplaceAll", {count: matches.value.length}))) {
    return;
  }
  const {re} = searchPattern();
  if (!re) {
    return;
  }
  undoSnapshot.value = draft.value;
  re.lastIndex = 0;
  draft.value = draft.value.replace(re, () => replaceWith.value);
  currentMatch.value = -1;
  nextTick(() => {
    if (matches.value.length > 0) {
      selectMatch(0);
    }
  });
}

function undoReplace(): void {
  if (undoSnapshot.value === null) {
    return;
  }
  draft.value = undoSnapshot.value;
  undoSnapshot.value = null;
  currentMatch.value = -1;
}

function onKeydown(event: KeyboardEvent): void {
  if (!props.open) {
    return;
  }
  const modifier = event.ctrlKey || event.metaKey;
  const key = event.key.toLowerCase();
  if (modifier && key === "f") {
    event.preventDefault();
    openFind(false);
    return;
  }
  if (modifier && key === "r") {
    event.preventDefault();
    openFind(true);
    return;
  }
  if (event.key === KEY_ESCAPE && findOpen.value) {
    event.preventDefault();
    event.stopImmediatePropagation();
    closeFind();
  }
}

onMounted(() => window.addEventListener("keydown", onKeydown, true));
onUnmounted(() => window.removeEventListener("keydown", onKeydown, true));

watch([find, matchCase, wholeWord, regexMode], () => {
  currentMatch.value = -1;
  if (!findOpen.value || !find.value) {
    return;
  }
  nextTick(() => {
    if (matches.value.length > 0) {
      selectMatch(0);
    }
  });
});

watch(draft, () => {
  if (currentMatch.value >= matches.value.length) {
    currentMatch.value = -1;
  }
});

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
          <span v-if="changed" class="diff-summary">+{{ addedCount }} / -{{ removedCount }}</span>
        </header>

        <div class="editor-area">
          <div class="gutter">
            <div :style="{ transform: `translateY(-${scrollTop}px)` }" class="gutter-inner">
              <div
                  v-for="line in gutterLines"
                  :key="line.number"
                  :class="['gutter-line', line.kind]"
              >
                <span class="sign">{{ line.kind === "add" ? "+" : line.kind === "change" ? "~" : " " }}</span>
                <span class="number">{{ line.number }}</span>
              </div>
            </div>
          </div>

          <div class="code">
            <div
                :style="{ transform: `translate(${-scrollLeft}px, ${-scrollTop}px)` }"
                class="diff-layer"
            >
              <div
                  v-for="(_, index) in draftLines"
                  :key="index"
                  :class="['diff-line', lineKinds[index]]"
              />
            </div>
            <textarea
                ref="editorEl"
                v-model="draft"
                class="editor"
                spellcheck="false"
                wrap="off"
                @scroll="syncScroll"
            />
          </div>

          <div v-if="findOpen" class="search-panel" @keydown.esc.stop.prevent="closeFind">
            <div class="search-row">
              <button
                  :title="replaceOpen ? t('editor.collapseReplace') : t('editor.expandReplace')"
                  class="expand"
                  type="button"
                  @click="replaceOpen = !replaceOpen"
              >
                {{ replaceOpen ? "⌄" : "›" }}
              </button>
              <input
                  ref="findInput"
                  v-model="find"
                  :class="{ invalid: searchInvalid }"
                  :placeholder="t('editor.find')"
                  @keydown.enter.prevent="findNext"
                  @keydown.shift.enter.prevent="findPrev"
              />
              <button
                  :class="{ active: matchCase }"
                  :title="t('editor.matchCase')"
                  class="toggle"
                  type="button"
                  @click="matchCase = !matchCase"
              >
                Cc
              </button>
              <button
                  :class="{ active: wholeWord }"
                  :title="t('editor.wholeWord')"
                  class="toggle"
                  type="button"
                  @click="wholeWord = !wholeWord"
              >
                W
              </button>
              <button
                  :class="{ active: regexMode }"
                  :title="t('editor.regex')"
                  class="toggle"
                  type="button"
                  @click="regexMode = !regexMode"
              >
                .*
              </button>
              <span :class="{ empty: find && (!matches.length || searchInvalid) }" class="result">
                {{ resultLabel }}
              </span>
              <button
                  :aria-label="t('editor.previous')"
                  :disabled="matches.length === 0"
                  :title="t('editor.previous')"
                  class="icon"
                  type="button"
                  @click="findPrev"
              >
                ↑
              </button>
              <button
                  :aria-label="t('editor.nextMatch')"
                  :disabled="matches.length === 0"
                  :title="t('editor.nextMatch')"
                  class="icon"
                  type="button"
                  @click="findNext"
              >
                ↓
              </button>
              <button
                  :aria-label="t('editor.closeSearch')"
                  :title="t('editor.closeSearch')"
                  class="icon"
                  type="button"
                  @click="closeFind"
              >
                ×
              </button>
            </div>

            <div v-if="replaceOpen" class="replace-row">
              <input
                  v-model="replaceWith"
                  :placeholder="t('editor.replaceWith')"
                  @keydown.enter.prevent="replaceCurrent"
              />
              <button :disabled="matches.length === 0" class="tiny" type="button" @click="replaceCurrent">
                {{ t("editor.replace") }}
              </button>
              <button :disabled="matches.length === 0" class="tiny" type="button" @click="replaceAll">
                {{ t("editor.replaceAll") }}
              </button>
              <button v-if="undoSnapshot !== null" class="tiny" type="button" @click="undoReplace">
                {{ t("editor.undo") }}
              </button>
            </div>
          </div>
        </div>

        <p class="muted">{{ t("editor.versionHint", {version}) }}</p>

        <p v-if="error" class="error">
          {{ t(error.message_key) }}
          <span v-if="error.fields.detail">（{{ error.fields.detail }}）</span>
        </p>

        <footer class="actions">
          <button type="button" @click="emit('close')">{{ t("editor.cancel") }}</button>
          <button
              :disabled="!changed || saving"
              class="primary"
              type="button"
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
  width: min(880px, 94vw);
  max-height: 88vh;
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
  align-items: center;
  gap: 12px;
}

h2 {
  margin: 0;
  font-size: 17px;
}

.path {
  color: var(--app-text-muted, #9aa4b2);
  font-family: ui-monospace, monospace;
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.diff-summary {
  margin-left: auto;
  color: var(--app-text-muted, #9aa4b2);
  font-family: ui-monospace, monospace;
  font-size: 11px;
  white-space: nowrap;
}

.editor-area {
  position: relative;
  display: flex;
  height: clamp(280px, 48vh, 420px);
  border: 1px solid #2a323c;
  border-radius: 6px;
  background: #101418;
  overflow: hidden;
}

.editor-area:focus-within {
  border-color: #2563eb;
}

.gutter {
  flex: 0 0 3.6rem;
  padding: 10px 6px;
  overflow: hidden;
  border-right: 1px solid rgb(255 255 255 / 6%);
  background: rgb(255 255 255 / 3%);
}

.gutter-inner {
  will-change: transform;
}

.gutter-line {
  display: flex;
  align-items: center;
  gap: 4px;
  height: 20px;
  color: var(--app-text-dim, #6b7280);
  font-family: ui-monospace, monospace;
  font-size: 11px;
  line-height: 20px;
  white-space: nowrap;
}

.gutter-line.add {
  color: #4ade80;
}

.gutter-line.change {
  color: #fbbf24;
}

.sign {
  width: 0.8rem;
  text-align: center;
}

.number {
  min-width: 1.4rem;
  text-align: right;
}

.code {
  position: relative;
  flex: 1;
  min-width: 0;
}

.diff-layer {
  position: absolute;
  inset: 0;
  padding: 10px 12px;
  overflow: hidden;
  pointer-events: none;
}

.diff-line {
  width: 100%;
  height: 20px;
}

.diff-line.add {
  background: rgb(34 197 94 / 14%);
}

.diff-line.change {
  background: rgb(251 191 36 / 14%);
}

.editor {
  box-sizing: border-box;
  position: relative;
  z-index: 1;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 10px 12px;
  border: none;
  outline: none;
  background: transparent;
  color: var(--app-text, #e6e6e6);
  font-family: ui-monospace, monospace;
  font-size: 13px;
  line-height: 20px;
  white-space: pre;
  overflow: auto;
  resize: none;
  caret-color: #fff;
}

.search-panel {
  position: absolute;
  z-index: 3;
  top: 8px;
  right: 8px;
  width: min(560px, calc(100% - 16px));
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 6px;
  border: 1px solid #2a323c;
  border-radius: 8px;
  background: #1d242c;
  box-shadow: 0 10px 24px rgb(0 0 0 / 35%);
}

.search-row,
.replace-row {
  display: flex;
  align-items: center;
  gap: 3px;
}

.search-row input,
.replace-row input {
  flex: 1;
  min-width: 6rem;
  padding: 4px 7px;
  border: 1px solid #2a323c;
  border-radius: 5px;
  background: #101418;
  color: inherit;
  font-size: 12px;
}

.search-row input.invalid {
  border-color: #f87171;
}

.expand,
.toggle,
.icon {
  height: 24px;
  padding: 0;
  border: 1px solid transparent;
  border-radius: 4px;
  background: transparent;
  color: var(--app-text-muted, #9aa4b2);
  cursor: pointer;
  line-height: 1;
}

.expand {
  width: 1.1rem;
  font-size: 16px;
}

.toggle {
  width: 1.7rem;
  border-color: rgb(255 255 255 / 12%);
  font-size: 11px;
}

.toggle.active {
  border-color: #2563eb;
  background: rgb(37 99 235 / 30%);
  color: #fff;
}

.icon {
  width: 1.4rem;
  font-size: 13px;
}

.icon:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}

.result {
  min-width: 3.2rem;
  color: var(--app-text-muted, #9aa4b2);
  font-size: 11px;
  text-align: right;
  white-space: nowrap;
}

.result.empty {
  color: #f87171;
}

.replace-row {
  padding-top: 2px;
}

.tiny {
  height: 24px;
  padding: 0 7px;
  border-radius: 5px;
  border: 1px solid #2a323c;
  background: #1d242c;
  color: var(--app-text-soft, #cbd5e1);
  cursor: pointer;
  font-size: 11px;
}

.tiny:disabled {
  opacity: 0.45;
  cursor: not-allowed;
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
  color: var(--app-text-muted, #9aa4b2);
  font-size: 12px;
  margin: 0;
}

.error {
  color: #ef4444;
  margin: 0;
  font-size: 13px;
}
</style>
