<script lang="ts" setup>
import {getCurrentWindow} from "@tauri-apps/api/window";
import hljs from "highlight.js/lib/core";
import jsonLanguage from "highlight.js/lib/languages/json";
import yamlLanguage from "highlight.js/lib/languages/yaml";
import {computed, onMounted, onUnmounted, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {NButton, NDropdown, NInputNumber, NModal} from "naive-ui";
import {useRouter} from "vue-router";
import {isMap, isSeq, parseDocument} from "yaml";

import {KEY_ESCAPE} from "@/constants/keyboard";
import {auditList} from "@/api/audit";
import {type InvokeRejection, toAppError} from "@/api/client";
import {nodeAcl, nodeChildren, nodeData, nodeStat} from "@/api/node";
import type {AclEntry} from "@/api/types/AclEntry";
import type {AppError} from "@/api/types/AppError";
import type {AuditRecord} from "@/api/types/AuditRecord";
import type {ConnectionSnapshot} from "@/api/types/ConnectionSnapshot";
import type {DeleteOutcome} from "@/api/types/DeleteOutcome";
import type {NodeData} from "@/api/types/NodeData";
import type {NodeStat} from "@/api/types/NodeStat";
import CommandPanel from "@/components/CommandPanel.vue";
import ConnectionDialog from "@/components/ConnectionDialog.vue";
import NodeBinaryEditor from "@/components/NodeBinaryEditor.vue";
import NodeCreateDialog from "@/components/NodeCreateDialog.vue";
import NodeDeleteDialog from "@/components/NodeDeleteDialog.vue";
import NodeEditor from "@/components/NodeEditor.vue";
import TreeNodeItem from "@/components/TreeNodeItem.vue";
import {useConnectionStore} from "@/stores/connection";
import {useHistoryStore} from "@/stores/history";
import {useTabsStore} from "@/stores/tabs";
import {useUiStore} from "@/stores/ui";
import {decodeUtf8Base64, hexPreview} from "@/utils/encoding";
import {clampCommandHeight, loadCommandHeight, saveCommandHeight,} from "@/utils/layout";
import {applyNumber} from "@/utils/number";
import {normalizePath} from "@/utils/path";
import {loadShortcuts, matchesShortcut, type ShortcutMap} from "@/utils/shortcuts";
import {toRfc3339} from "@/utils/time";

hljs.registerLanguage("json", jsonLanguage);
hljs.registerLanguage("yaml", yamlLanguage);

const {t} = useI18n();
const router = useRouter();
const store = useConnectionStore();
const tabs = useTabsStore();
const ui = useUiStore();

const menuOpen = ref(false);
const dialogOpen = ref(false);
const treeKey = ref(0);
const selectedPath = ref<string | null>("/");
const data = ref<NodeData | null>(null);
const dataError = ref<AppError | null>(null);
const activeTab = ref<"data" | "stat" | "acl">("data");
const acls = ref<AclEntry[] | null>(null);
const aclError = ref<AppError | null>(null);
const format = ref<"raw" | "json" | "yaml">("raw");
const loadingData = ref(false);
const statView = ref<NodeStat | null>(null);
const largePending = ref(false);
const editorOpen = ref(false);
const createOpen = ref(false);
const deleteOpen = ref(false);
const binaryOpen = ref(false);
const deleteTarget = ref<string | null>(null);
const retryPath = ref<string | null>(null);
const toast = ref("");
let toastTimer = 0;
const autoRefreshSeconds = ref<number>();
const customRefreshOpen = ref(false);
const customRefreshSeconds = ref(30);
let autoRefreshTimer = 0;
const jumpPath = ref("");
const jumpInput = ref<HTMLInputElement | null>(null);
const jumpCompletion = ref<{
  candidates: string[];
  index: number;
  prefixText: string;
  lastValue: string;
} | null>(null);
const shortcuts = ref<ShortcutMap>(loadShortcuts());
const toolsOpen = ref(false);

function showAbout(): void {
  ui.openAbout();
}

async function closeWindow(): Promise<void> {
  await getCurrentWindow().close();
}

function showToast(message: string, duration = 3000): void {
  toast.value = message;
  window.clearTimeout(toastTimer);
  toastTimer = window.setTimeout(() => {
    toast.value = "";
  }, duration);
}

const history = useHistoryStore();
const timelineOpen = ref(true);
const statOpen = ref(true);
const sideTab = ref<"recent" | "history">("recent");
const treeWidth = ref(Number(localStorage.getItem("zkui.tree.width") ?? "22"));
const inspectorWidth = ref(Number(localStorage.getItem("zkui.inspector.width") ?? "20"));
const commandHeight = ref(loadCommandHeight());

function startResize(which: "tree" | "inspector", event: MouseEvent): void {
  event.preventDefault();
  const body = document.querySelector(".workspace-main > .body") as HTMLElement | null;
  if (!body) {
    return;
  }
  const rect = body.getBoundingClientRect();
  const move = (moveEvent: MouseEvent): void => {
    const ratio =
        which === "tree"
            ? ((moveEvent.clientX - rect.left) / rect.width) * 100
            : ((rect.right - moveEvent.clientX) / rect.width) * 100;
    const clamped = Math.min(40, Math.max(12, ratio));
    if (which === "tree") {
      treeWidth.value = Math.round(clamped);
      localStorage.setItem("zkui.tree.width", String(treeWidth.value));
    } else {
      inspectorWidth.value = Math.round(clamped);
      localStorage.setItem("zkui.inspector.width", String(inspectorWidth.value));
    }
  };
  const stop = (): void => {
    window.removeEventListener("mousemove", move);
    window.removeEventListener("mouseup", stop);
    document.body.style.userSelect = "";
    document.body.style.cursor = "";
  };
  document.body.style.userSelect = "none";
  document.body.style.cursor = "col-resize";
  window.addEventListener("mousemove", move);
  window.addEventListener("mouseup", stop);
}

function startResizeCommand(event: MouseEvent): void {
  event.preventDefault();
  const main = document.querySelector(".workspace-main") as HTMLElement | null;
  if (!main) {
    return;
  }
  const rect = main.getBoundingClientRect();
  const move = (moveEvent: MouseEvent): void => {
    const ratio = ((rect.bottom - moveEvent.clientY) / rect.height) * 100;
    commandHeight.value = clampCommandHeight(ratio);
  };
  const stop = (): void => {
    window.removeEventListener("mousemove", move);
    window.removeEventListener("mouseup", stop);
    document.body.style.userSelect = "";
    document.body.style.cursor = "";
    saveCommandHeight(commandHeight.value);
  };
  document.body.style.userSelect = "none";
  document.body.style.cursor = "row-resize";
  window.addEventListener("mousemove", move);
  window.addEventListener("mouseup", stop);
}

function resizeCommandBy(delta: number): void {
  commandHeight.value = clampCommandHeight(commandHeight.value + delta);
  saveCommandHeight(commandHeight.value);
}

const expandedPaths = ref<Set<string>>(new Set());
const createDefaultPath = ref<string | undefined>(undefined);

function expandedKey(id: string): string {
  return `zkui.tree.expanded.${id}`;
}

function loadExpanded(id: string): void {
  try {
    const raw = localStorage.getItem(expandedKey(id));
    expandedPaths.value = new Set<string>(raw ? (JSON.parse(raw) as string[]) : []);
  } catch {
    expandedPaths.value = new Set<string>();
  }
}

function persistExpanded(): void {
  if (connectionId.value) {
    localStorage.setItem(expandedKey(connectionId.value), JSON.stringify([...expandedPaths.value]));
  }
}

function toggleExpanded(path: string): void {
  const next = new Set(expandedPaths.value);
  if (next.has(path)) {
    next.delete(path);
  } else {
    next.add(path);
  }
  expandedPaths.value = next;
  persistExpanded();
}

async function expandAll(): Promise<void> {
  if (!connectionId.value) {
    return;
  }
  const next = new Set<string>(["/"]);
  const queue = ["/"];
  let count = 0;
  while (queue.length > 0 && count < 2000) {
    const current = queue.shift();
    if (!current) {
      break;
    }
    try {
      const children = await nodeChildren(connectionId.value, current);
      count += children.length;
      for (const child of children) {
        const path = current === "/" ? `/${child}` : `${current}/${child}`;
        next.add(path);
        queue.push(path);
      }
    } catch {
      // 忽略不可读子树
    }
  }
  expandedPaths.value = next;
  persistExpanded();
  showToast(t("shell.expanded", {count}), 3000);
}

function collapseAll(): void {
  expandedPaths.value = new Set<string>();
  persistExpanded();
}

function createFromTree(path: string): void {
  createDefaultPath.value = path === "/" ? "/" : `${path}/`;
  createOpen.value = true;
}

function deleteFromTree(path: string): void {
  deleteTarget.value = path;
  deleteOpen.value = true;
}

function openCreate(): void {
  createDefaultPath.value = undefined;
  createOpen.value = true;
}

const FORMAT_OPTIONS = [
  {value: "raw" as const, label: t("data.formatRaw")},
  {value: "json" as const, label: "JSON"},
  {value: "yaml" as const, label: "YAML"},
];
const AUTO_REFRESH_PRESETS = [1, 5, 10] as const;
const loadToken = ref(0);
const timeline = ref<AuditRecord[]>([]);

const connectionId = computed(() => store.snapshot?.connection_id ?? "");
const stat = computed<NodeStat | null>(() => data.value?.stat ?? statView.value);
const stateLabel = computed(() =>
    store.snapshot ? t(`connection.state.${store.snapshot.state}`) : t("shell.noConnection"),
);

const text = computed(() =>
    data.value?.is_text ? decodeUtf8Base64(data.value.data_b64) : "",
);
const jsonText = computed(() => {
  if (!text.value) {
    return null;
  }
  try {
    return JSON.stringify(JSON.parse(text.value), null, 2);
  } catch {
    return null;
  }
});
const yamlText = computed(() => {
  if (!text.value) {
    return null;
  }
  try {
    const document = parseDocument(text.value);
    if (document.errors.length > 0) {
      return null;
    }
    const contents = document.contents;
    if (!contents || (!isMap(contents) && !isSeq(contents))) {
      return null;
    }
    return document.toString({indent: 2});
  } catch {
    return null;
  }
});
const displayText = computed(() => {
  if (format.value === "json") {
    return jsonText.value ?? text.value;
  }
  if (format.value === "yaml") {
    return yamlText.value ?? text.value;
  }
  return text.value;
});
const highlighted = computed(() => {
  if (format.value === "raw") {
    return "";
  }
  return hljs.highlight(displayText.value, {language: format.value}).value;
});
// biome-ignore lint/correctness/noUnusedVariables: used in template
const hexDump = computed(() => (data.value ? hexPreview(data.value.data_b64) : ""));
const defaultChildPath = computed(() =>
    selectedPath.value === "/" ? "/new" : `${selectedPath.value}/new`,
);

const PREVIEW_BYTES = 256 * 1024;

async function loadData(path: string, full = false): Promise<void> {
  if (!connectionId.value) {
    return;
  }
  loadingData.value = true;
  dataError.value = null;
  try {
    data.value = await nodeData(connectionId.value, path, full ? null : PREVIEW_BYTES);
    statView.value = data.value.stat;
    largePending.value = false;
  } catch (raw) {
    data.value = null;
    dataError.value = toAppError(raw as InvokeRejection);
  } finally {
    loadingData.value = false;
  }
}

async function loadStat(path: string): Promise<void> {
  if (!connectionId.value) {
    return;
  }
  const token = ++loadToken.value;
  loadingData.value = true;
  dataError.value = null;
  data.value = null;
  statView.value = null;
  largePending.value = false;
  try {
    const value = await nodeStat(connectionId.value, path);
    if (token !== loadToken.value) {
      return;
    }
    statView.value = value;
    if (value.data_length > PREVIEW_BYTES) {
      largePending.value = true;
      return;
    }
    await loadData(path);
  } catch (raw) {
    if (token === loadToken.value) {
      dataError.value = toAppError(raw as InvokeRejection);
    }
  } finally {
    if (token === loadToken.value) {
      loadingData.value = false;
    }
  }
}

async function select(path: string): Promise<void> {
  const canonical = normalizePath(path);
  selectedPath.value = canonical;
  activeTab.value = "data";
  format.value = "raw";
  acls.value = null;
  aclError.value = null;
  if (connectionId.value) {
    tabs.open(connectionId.value, canonical);
  }
  await loadStat(canonical);
  void loadTimeline(canonical);
}

async function openEditorAt(path: string): Promise<void> {
  await select(path);
  if (data.value?.is_text && !data.value.truncated) {
    editorOpen.value = true;
  }
}

function closeTab(path: string): void {
  tabs.close(connectionId.value, path);
  if (selectedPath.value === path) {
    const remaining = tabs.tabsFor(connectionId.value);
    select(remaining[remaining.length - 1] ?? "/");
  }
}

async function loadTimeline(path: string): Promise<void> {
  try {
    timeline.value = await auditList(10, 0, path);
  } catch {
    timeline.value = [];
  }
}

async function loadAcl(): Promise<void> {
  if (!connectionId.value || !selectedPath.value) {
    return;
  }
  aclError.value = null;
  try {
    acls.value = await nodeAcl(connectionId.value, selectedPath.value);
  } catch (raw) {
    acls.value = null;
    aclError.value = toAppError(raw as InvokeRejection);
  }
}

function showAcl(): void {
  activeTab.value = "acl";
  if (acls.value === null) {
    void loadAcl();
  }
}

async function copyPath(): Promise<void> {
  if (!selectedPath.value) {
    return;
  }
  try {
    await navigator.clipboard.writeText(selectedPath.value);
    showToast(t("data.pathCopied", {path: selectedPath.value}), 2500);
  } catch {
    showToast(t("data.copyFailed"), 2500);
  }
}

async function copyAddress(): Promise<void> {
  if (!store.snapshot) {
    return;
  }
  const address = `${store.snapshot.host}:${store.snapshot.port}`;
  try {
    await navigator.clipboard.writeText(address);
    showToast(t("shell.copied", {address}), 2500);
  } catch {
    showToast(t("shell.copyFailed"), 2500);
  }
  menuOpen.value = false;
}

function applyJumpCompletion(
  state: { candidates: string[]; index: number; prefixText: string; lastValue: string },
  index: number,
): void {
  state.index = index;
  jumpPath.value = state.prefixText + state.candidates[index];
  state.lastValue = jumpPath.value;
}

async function completeJump(): Promise<void> {
  const value = jumpPath.value;
  if (jumpCompletion.value?.lastValue === value) {
    const state = jumpCompletion.value;
    applyJumpCompletion(state, (state.index + 1) % state.candidates.length);
    return;
  }

  const slash = value.lastIndexOf("/");
  const rawParent = slash >= 0 ? value.slice(0, slash) : "";
  const prefix = slash >= 0 ? value.slice(slash + 1) : value;
  const parent = rawParent === "" ? "/" : rawParent.startsWith("/") ? rawParent : `/${rawParent}`;
  if (!connectionId.value) {
    return;
  }
  try {
    const children = await nodeChildren(connectionId.value, parent);
    const candidates = children
        .filter((name) => name.startsWith(prefix))
        .map((name) => `${parent === "/" ? "" : parent}/${name}`);
    if (candidates.length === 0) {
      jumpCompletion.value = null;
      return;
    }
    const state = {
      candidates,
      index: 0,
      prefixText: value.slice(0, slash + 1),
      lastValue: "",
    };
    jumpCompletion.value = state;
    applyJumpCompletion(state, 0);
  } catch {
    // 补全失败不打断输入
    jumpCompletion.value = null;
  }
}

function jump(): void {
  if (!jumpPath.value.trim()) {
    return;
  }
  void select(normalizePath(jumpPath.value));
}

function loadFull(): void {
  if (selectedPath.value) {
    void loadData(selectedPath.value, true);
  }
}

function refresh(): void {
  treeKey.value += 1;
}

function reloadData(): void {
  if (selectedPath.value) {
    void loadStat(selectedPath.value);
  }
}

function refreshAll(): void {
  refresh();
  reloadData();
}

const autoRefreshTitle = computed(() => {
  if (autoRefreshSeconds.value === undefined) {
    return t("data.autoRefresh");
  }
  return t("data.autoRefreshSeconds", {seconds: autoRefreshSeconds.value});
});

const autoRefreshOptions = computed(() => {
  const active = autoRefreshSeconds.value;
  const mark = (value: number | undefined) => (value === active ? "✓ " : "");
  const customActive = active !== undefined && !AUTO_REFRESH_PRESETS.some((value) => value === active);
  return [
    {label: `${mark(undefined)}${t("data.autoRefreshOff")}`, key: "off"},
    ...AUTO_REFRESH_PRESETS.map((seconds) => ({
      label: `${mark(seconds)}${t("data.autoRefreshSeconds", {seconds})}`,
      key: seconds,
    })),
    {label: `${customActive ? "✓ " : ""}${t("data.autoRefreshCustom")}`, key: "custom"},
  ];
});

function setAutoRefresh(seconds: number | undefined): void {
  window.clearInterval(autoRefreshTimer);
  autoRefreshTimer = 0;
  autoRefreshSeconds.value = seconds;
  if (seconds === undefined) {
    return;
  }
  if (selectedPath.value) {
    void reloadData();
  }
  autoRefreshTimer = window.setInterval(() => {
    if (selectedPath.value && !loadingData.value) {
      void reloadData();
    }
  }, Math.max(1, seconds) * 1000);
}

function selectAutoRefresh(key: string | number): void {
  if (key === "off") {
    setAutoRefresh(undefined);
    return;
  }
  if (key === "custom") {
    customRefreshSeconds.value = autoRefreshSeconds.value ?? 30;
    customRefreshOpen.value = true;
    return;
  }
  if (typeof key === "number") {
    setAutoRefresh(key);
  }
}

function setCustomRefreshSeconds(value: number): void {
  customRefreshSeconds.value = value;
}

function applyCustomRefresh(): void {
  setAutoRefresh(Math.max(1, Math.round(customRefreshSeconds.value)));
  customRefreshOpen.value = false;
}

watch(connectionId, (id) => {
  if (!id) {
    setAutoRefresh(undefined);
  }
  treeKey.value += 1;
  selectedPath.value = "/";
  data.value = null;
  void loadStat("/");
});

watch(
    () => store.snapshot?.generation,
    () => {
      treeKey.value += 1;
      if (selectedPath.value) {
        void loadStat(selectedPath.value);
      }
    },
);

onMounted(async () => {
  await store.refresh();
  if (!store.snapshot) {
    const first = store.connections[0];
    if (first) {
      store.activate(first.connection_id);
    } else {
      await router.replace("/");
      return;
    }
  }
  selectedPath.value = "/";
  void loadStat("/");
  loadExpanded(connectionId.value);
  window.addEventListener("keydown", onKeydown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", onKeydown);
  window.clearTimeout(toastTimer);
  window.clearInterval(autoRefreshTimer);
});

function onKeydown(event: KeyboardEvent): void {
  if (event.key === KEY_ESCAPE) {
    if (ui.aboutOpen) {
      ui.closeAbout();
    } else if (editorOpen.value || createOpen.value || deleteOpen.value || binaryOpen.value || customRefreshOpen.value) {
      editorOpen.value = false;
      createOpen.value = false;
      deleteOpen.value = false;
      binaryOpen.value = false;
      customRefreshOpen.value = false;
    } else if (dialogOpen.value) {
      dialogOpen.value = false;
    } else if (menuOpen.value || toolsOpen.value) {
      menuOpen.value = false;
      toolsOpen.value = false;
    } else {
      void router.push("/");
    }
    event.preventDefault();
    return;
  }
  const target = event.target as HTMLElement | null;
  const typing =
      target?.tagName === "INPUT" || target?.tagName === "TEXTAREA" || target?.tagName === "SELECT";
  if (typing && !event.ctrlKey && !event.metaKey) {
    return;
  }
  const map = shortcuts.value;
  if (matchesShortcut(event, map.jump)) {
    event.preventDefault();
    jumpInput.value?.focus();
  } else if (matchesShortcut(event, map.edit) && selectedPath.value) {
    event.preventDefault();
    void openEditorAt(selectedPath.value);
  } else if (matchesShortcut(event, map.create)) {
    event.preventDefault();
    createOpen.value = true;
  } else if (matchesShortcut(event, map.delete) && selectedPath.value) {
    event.preventDefault();
    deleteTarget.value = selectedPath.value;
    deleteOpen.value = true;
  } else if (matchesShortcut(event, map.refresh)) {
    event.preventDefault();
    refreshAll();
  }
}

function switchTo(snapshot: ConnectionSnapshot): void {
  store.activate(snapshot.connection_id);
  menuOpen.value = false;
}

function onConnected(snapshot: ConnectionSnapshot): void {
  dialogOpen.value = false;
  store.activate(snapshot.connection_id);
}

async function disconnect(): Promise<void> {
  menuOpen.value = false;
  await store.close();
  if (store.connections.length === 0) {
    await router.replace("/");
  }
}

function openDelete(): void {
  deleteTarget.value = selectedPath.value;
  deleteOpen.value = true;
}

function retryFailedSubtree(): void {
  if (!retryPath.value) {
    return;
  }
  deleteTarget.value = retryPath.value;
  deleteOpen.value = true;
}

async function openBinaryEditor(): Promise<void> {
  if (data.value?.truncated && selectedPath.value) {
    await loadData(selectedPath.value, true);
  }
  binaryOpen.value = true;
}

function onSaved(stat: NodeStat): void {
  editorOpen.value = false;
  showToast(t("editor.saved", {version: stat.version}), 3000);
  if (selectedPath.value) {
    void loadData(selectedPath.value);
  }
}

function onCreated(actualPath: string): void {
  createOpen.value = false;
  showToast(t("create.created", {path: actualPath}), 3500);
  treeKey.value += 1;
  select(actualPath);
}

function onDeleted(outcome: DeleteOutcome): void {
  deleteOpen.value = false;
  retryPath.value = outcome.failed_path ?? null;
  showToast(
      outcome.failed_path
          ? t("delete.failed", {path: outcome.failed_path, done: outcome.deleted})
          : t("delete.done", {done: outcome.deleted}),
      outcome.failed_path ? 7000 : 3500,
  );
  treeKey.value += 1;
  const parent = selectedPath.value?.replace(/\/[^/]+$/, "") || "/";
  select(parent || "/");
}
</script>

<template>
  <div :style="{ '--tree-width': `${treeWidth}%`, '--inspector-width': `${inspectorWidth}%` }" class="shell">
    <header class="topbar">
      <div class="conn">
        <button class="chip" type="button" @click="menuOpen = !menuOpen">
          <span :data-state="store.snapshot?.state ?? 'none'" class="dot"/>
          <span class="name">{{ store.snapshot?.display_name ?? t("shell.noConnection") }}</span>
          <span v-if="store.snapshot" class="endpoint">
            {{ store.snapshot.host }}:{{ store.snapshot.port }}
          </span>
          <span class="caret">▾</span>
        </button>

        <div v-if="menuOpen" class="menu">
          <button
              v-for="item in store.connections"
              :key="item.connection_id"
              :class="{ active: item.connection_id === store.snapshot?.connection_id }"
              class="menu-item"
              type="button"
              @click="switchTo(item)"
          >
            <span :data-state="item.state" class="dot"/>
            <span>{{ item.display_name }}</span>
            <span class="muted">{{ item.host }}:{{ item.port }}</span>
          </button>
          <div class="sep"/>
          <button
              v-if="store.snapshot"
              class="menu-item"
              type="button"
              @click="copyAddress"
          >
            {{ t("shell.copyAddress") }}
          </button>
          <button class="menu-item" type="button" @click="dialogOpen = true; menuOpen = false">
            {{ t("shell.newConnection") }}
          </button>
          <button class="menu-item" type="button" @click="router.push('/')">
            {{ t("shell.home") }}
          </button>
          <button class="menu-item danger" type="button" @click="disconnect">
            {{ t("shell.disconnect") }}
          </button>
        </div>
      </div>

      <div class="spacer"/>
      <div class="tools">
        <button class="ghost" type="button" @click="toolsOpen = !toolsOpen">
          {{ t("shell.tools") }} ▾
        </button>
        <div v-if="toolsOpen" class="tools-menu">
          <button type="button" @click="router.push('/audit')">{{ t("home.audit") }}</button>
          <button type="button" @click="router.push('/observability')">
            {{ t("home.observability") }}
          </button>
          <button type="button" @click="router.push('/settings')">{{ t("home.settings") }}</button>
          <div class="tools-sep"/>
          <button type="button" @click="toolsOpen = false; showAbout()">{{ t("shell.about") }}</button>
          <button type="button" @click="toolsOpen = false; closeWindow()">
            {{ t("shell.closeWindow") }}
          </button>
        </div>
      </div>
    </header>

    <div class="workspace-main">
      <div class="body">
        <aside class="tree">
          <div class="aside-head">
            <span>{{ t("shell.tree") }}</span>
            <div class="tree-actions">
              <NButton size="tiny" @click="expandAll">{{ t("shell.expandAll") }}</NButton>
              <NButton size="tiny" @click="collapseAll">{{ t("shell.collapseAll") }}</NButton>
              <NButton size="tiny" @click="refresh">{{ t("shell.refresh") }}</NButton>
            </div>
          </div>
          <form class="jump" @submit.prevent="jump">
            <input
                ref="jumpInput"
                v-model="jumpPath"
                :aria-label="t('shell.path')"
                :placeholder="t('shell.jumpPlaceholder')"
                @keydown.tab.prevent="completeJump"
            />
            <button class="tiny" type="submit">{{ t("shell.go") }}</button>
          </form>
          <div v-if="connectionId" class="tree-scroll">
            <TreeNodeItem
                :key="`${connectionId}-${store.snapshot?.generation ?? 0}-${treeKey}`"
                :connection-id="connectionId"
                :depth="0"
                :expanded-paths="expandedPaths"
                :selected-path="selectedPath"
                name="/"
                path="/"
                @create="createFromTree"
                @dblclick="openEditorAt"
                @delete="deleteFromTree"
                @edit="openEditorAt"
                @select="select"
                @toggle="toggleExpanded"
            />
          </div>
          <p v-else class="muted">{{ t("shell.noConnection") }}</p>
        </aside>
        <button
            :aria-label="t('shell.resizeTree')"
            class="resizer"
            type="button"
            @mousedown="startResize('tree', $event)"
        />

        <section class="content">
          <div v-if="!selectedPath" class="placeholder">
            <h2>{{ t("shell.contentTitle") }}</h2>
            <p class="muted">{{ t("shell.contentHint") }}</p>
          </div>

          <div v-else class="detail">
            <div class="detail-head">
              <button
                  :title="t('data.copyPath')"
                  class="path"
                  type="button"
                  @click="copyPath"
              >
                {{ selectedPath }}
              </button>
              <div class="toolbar">
                <div :class="{active: autoRefreshSeconds !== undefined}" class="refresh-control">
                  <NButton size="tiny" :title="t('data.refresh')" @click="reloadData">
                    <template #icon>
                      <svg aria-hidden="true" class="clock-icon" viewBox="0 0 16 16">
                        <circle cx="8" cy="8" r="5.5"/>
                        <path d="M8 4.5V8l2.5 1.5"/>
                      </svg>
                    </template>
                    {{ t("data.refresh") }}
                  </NButton>
                  <NDropdown :options="autoRefreshOptions" trigger="click" @select="selectAutoRefresh">
                    <button :title="autoRefreshTitle" class="refresh-caret" type="button">
                      <span class="refresh-caret-triangle"/>
                    </button>
                  </NDropdown>
                </div>
                <NButton size="tiny" @click="openCreate">{{ t("data.create") }}</NButton>
                <NButton
                  :disabled="!data?.is_text || data.truncated"
                  :title="data?.is_text === false ? t('data.notEditable') : ''"
                  size="tiny"
                  @click="editorOpen = true"
                >
                  {{ t("data.edit") }}
                </NButton>
                <NButton size="tiny" type="error" @click="openDelete">{{ t("data.delete") }}</NButton>
              </div>
            </div>

            <div class="tabs">
              <button
                  :class="{ active: activeTab === 'data' }"
                  type="button"
                  @click="activeTab = 'data'"
              >
                {{ t("data.tab") }}
              </button>
              <button
                  :class="{ active: activeTab === 'stat' }"
                  type="button"
                  @click="activeTab = 'stat'"
              >
                {{ t("data.statTab") }}
              </button>
              <button
                  :class="{ active: activeTab === 'acl' }"
                  type="button"
                  @click="showAcl"
              >
                {{ t("data.aclTab") }}
              </button>
            </div>

            <p v-if="dataError" class="error">{{ t(dataError.message_key) }}</p>
            <p v-else-if="loadingData" class="muted">{{ t("delete.loading") }}</p>

            <template v-else-if="stat">
              <div v-if="activeTab === 'data'" class="data-pane">
                <div v-if="!data" class="large-pending">
                  <p class="warn">{{ t("data.largeHint", {bytes: stat?.data_length}) }}</p>
                  <button class="tiny" type="button" @click="loadData(selectedPath ?? '/')">
                    {{ t("data.loadData") }}
                  </button>
                </div>
                <template v-else>
                  <div v-if="!data.is_text" class="binary">
                    <p class="warn">{{ t("data.binaryHint") }}</p>
                    <div>
                      <button class="tiny" type="button" @click="openBinaryEditor">
                        {{ t("data.replaceBinary") }}
                      </button>
                    </div>
                    <pre class="hex">{{ hexDump }}</pre>
                    <p class="muted binary-meta">{{ t("data.bytes") }}: {{ data.byte_len }}</p>
                  </div>
                  <template v-else>
                    <pre v-if="format === 'raw'" class="text">{{ displayText || t("data.empty") }}</pre>
                    <pre v-else class="text hljs" v-html="highlighted"/>
                    <div class="data-toolbar">
                      <span class="muted">{{ t("data.bytes") }}: {{ data.byte_len }}</span>
                      <span class="muted">{{ t("data.encoding") }}: {{
                          data.is_text ? "UTF-8" : t("data.binary")
                        }}</span>
                      <span class="muted">{{ t("shell.version") }}: {{ stat?.version }}</span>
                      <span class="muted">{{ t("shell.mtime") }}: {{ stat ? toRfc3339(stat.mtime) : "" }}</span>
                      <div class="format-group">
                        <NButton
                          v-for="option in FORMAT_OPTIONS"
                          :key="option.value"
                          :type="format === option.value ? 'primary' : 'default'"
                          size="tiny"
                          @click="format = option.value"
                        >
                          {{ option.label }}
                        </NButton>
                      </div>
                      <template v-if="data.truncated">
                        <span class="warn">{{ t("data.truncated") }}</span>
                        <button class="tiny" type="button" @click="loadFull">
                          {{ t("data.loadFull") }}
                        </button>
                      </template>
                    </div>
                  </template>
                </template>
              </div>

              <div v-else-if="activeTab === 'acl'" class="acl-pane">
                <p v-if="aclError" class="error">{{ t(aclError.message_key) }}</p>
                <table v-else-if="acls?.length" class="acl-table">
                  <thead>
                  <tr>
                    <th>{{ t("data.scheme") }}</th>
                    <th>{{ t("data.identity") }}</th>
                    <th>{{ t("data.permissions") }}</th>
                  </tr>
                  </thead>
                  <tbody>
                  <tr v-for="entry in acls" :key="`${entry.scheme}:${entry.id}`">
                    <td>{{ entry.scheme }}</td>
                    <td class="mono">{{ entry.id }}</td>
                    <td class="mono">{{ entry.permissions }}</td>
                  </tr>
                  </tbody>
                </table>
                <p v-else class="muted">{{ t("data.aclEmpty") }}</p>
                <p class="hint">{{ t("data.aclReadOnly") }}</p>
              </div>

              <div v-else class="stat-groups">
                <section class="stat-group">
                  <h3>{{ t("data.statVersion") }}</h3>
                  <dl class="stat-table">
                    <div>
                      <dt>{{ t("shell.czxid") }}</dt>
                      <dd class="mono">{{ stat?.czxid }}</dd>
                    </div>
                    <div>
                      <dt>{{ t("shell.mzxid") }}</dt>
                      <dd class="mono">{{ stat?.mzxid }}</dd>
                    </div>
                    <div>
                      <dt>{{ t("shell.version") }}</dt>
                      <dd>{{ stat?.version }}</dd>
                    </div>
                    <div>
                      <dt>{{ t("shell.cversion") }}</dt>
                      <dd>{{ stat?.cversion }}</dd>
                    </div>
                    <div>
                      <dt>{{ t("shell.aversion") }}</dt>
                      <dd>{{ stat?.aversion }}</dd>
                    </div>
                  </dl>
                </section>
                <section class="stat-group">
                  <h3>{{ t("data.statTime") }}</h3>
                  <dl class="stat-table">
                    <div>
                      <dt>{{ t("shell.ctime") }}</dt>
                      <dd>{{ stat ? toRfc3339(stat.ctime) : "" }}</dd>
                    </div>
                    <div>
                      <dt>{{ t("shell.mtime") }}</dt>
                      <dd>{{ stat ? toRfc3339(stat.mtime) : "" }}</dd>
                    </div>
                  </dl>
                </section>
                <section class="stat-group">
                  <h3>{{ t("data.statStructure") }}</h3>
                  <dl class="stat-table">
                    <div>
                      <dt>{{ t("shell.dataLength") }}</dt>
                      <dd>{{ stat?.data_length }}</dd>
                    </div>
                    <div>
                      <dt>{{ t("shell.children") }}</dt>
                      <dd>{{ stat?.num_children }}</dd>
                    </div>
                    <div>
                      <dt>{{ t("shell.ephemeral") }}</dt>
                      <dd>{{ stat?.ephemeral_owner === 0 ? t("shell.persistent") : stat?.ephemeral_owner }}</dd>
                    </div>
                  </dl>
                </section>
              </div>
            </template>
          </div>
        </section>
        <button
            :aria-label="t('shell.resizeInspector')"
            class="resizer"
            type="button"
            @mousedown="startResize('inspector', $event)"
        />

        <aside class="inspector">
          <button
              :aria-expanded="statOpen"
              class="aside-head inspector-toggle"
              type="button"
              @click="statOpen = !statOpen"
          >
            <span>{{ t("shell.inspector") }}</span>
            <span class="chevron">{{ statOpen ? "▾" : "▸" }}</span>
          </button>
          <dl v-if="statOpen && stat" class="inspector-grid">
            <div>
              <dt>{{ t("shell.path") }}</dt>
              <dd class="mono">{{ stat.path }}</dd>
            </div>
            <div>
              <dt>{{ t("shell.version") }}</dt>
              <dd>{{ stat.version }}</dd>
            </div>
            <div>
              <dt>{{ t("shell.cversion") }}</dt>
              <dd>{{ stat.cversion }}</dd>
            </div>
            <div>
              <dt>{{ t("shell.children") }}</dt>
              <dd>{{ stat.num_children }}</dd>
            </div>
            <div>
              <dt>{{ t("shell.dataLength") }}</dt>
              <dd>{{ stat.data_length }}</dd>
            </div>
            <div>
              <dt>{{ t("shell.ephemeral") }}</dt>
              <dd>{{ stat.ephemeral_owner === 0 ? t("shell.persistent") : stat.ephemeral_owner }}</dd>
            </div>
            <div>
              <dt>{{ t("shell.mtime") }}</dt>
              <dd>{{ toRfc3339(stat.mtime) }}</dd>
            </div>
          </dl>

          <div class="side-tabs">
            <button
                :class="{ active: sideTab === 'recent' }"
                type="button"
                @click="sideTab = 'recent'"
            >
              {{ t("shell.recentNodes") }} ({{ tabs.tabsFor(connectionId).length }})
            </button>
            <button
                :class="{ active: sideTab === 'history' }"
                type="button"
                @click="sideTab = 'history'"
            >
              {{ t("shell.commandHistory") }} ({{ history.entries.length }})
            </button>
          </div>

          <div class="side-body">
            <template v-if="sideTab === 'recent'">
              <ul v-if="tabs.tabsFor(connectionId).length" class="timeline">
                <li v-for="path in [...tabs.tabsFor(connectionId)].reverse()" :key="path">
                  <button :title="path" class="history-line mono" type="button" @click="select(path)">
                    {{ path }}
                  </button>
                  <button
                      :aria-label="t('shell.closeTab')"
                      class="tiny"
                      type="button"
                      @click="closeTab(path)"
                  >
                    ×
                  </button>
                </li>
              </ul>
              <p v-else class="muted">{{ t("shell.recentEmpty") }}</p>
            </template>
            <template v-else>
              <ul v-if="history.entries.length" class="timeline">
                <li v-for="(entry, index) in history.entries" :key="`${entry.ts}-${index}`">
                  <button
                      :title="entry.line"
                      class="history-line mono"
                      type="button"
                      @click="history.setDraft(entry.line)"
                  >
                    {{ entry.line }}
                  </button>
                  <span class="muted">{{ entry.ok ? "" : "!" }}</span>
                </li>
              </ul>
              <p v-else class="muted">{{ t("shell.historyEmpty") }}</p>
            </template>
          </div>

          <button
              :aria-expanded="timelineOpen"
              class="aside-head section-toggle"
              type="button"
              @click="timelineOpen = !timelineOpen"
          >
            <span>{{ t("shell.timeline") }}</span>
            <span class="chevron">{{ timelineOpen ? "▾" : "▸" }}</span>
          </button>
          <ul v-if="timelineOpen && timeline.length" class="timeline">
            <li v-for="entry in timeline" :key="entry.id">
              <span class="mono">{{ entry.op }}</span>
              <span class="muted">{{ toRfc3339(entry.ts) }}</span>
            </li>
          </ul>
          <p v-if="timelineOpen && !timeline.length" class="muted">{{ t("shell.timelineEmpty") }}</p>
          <button
              v-if="timelineOpen"
              class="tiny audit-link"
              type="button"
              @click="router.push('/audit')"
          >
            {{ t("shell.viewAudit") }}
          </button>
        </aside>
      </div>

      <template v-if="connectionId">
        <hr
            :aria-label="t('shell.resizeCommand')"
            :aria-valuemax="75"
            :aria-valuemin="20"
            :aria-valuenow="commandHeight"
            aria-orientation="horizontal"
            class="h-resizer"
            tabindex="0"
            @mousedown="startResizeCommand"
            @keydown.up.prevent="resizeCommandBy(2)"
            @keydown.down.prevent="resizeCommandBy(-2)"
        />
        <div :style="{ flexBasis: `${commandHeight}%` }" class="command-dock">
          <CommandPanel
              :connection-id="connectionId"
              :connection-name="store.snapshot?.display_name ?? ''"
              @refresh="refreshAll"
          />
        </div>
      </template>
    </div>

    <footer class="statusbar">
      <span class="state">
        <span :data-state="store.snapshot?.state ?? 'none'" class="dot"/>
        {{ stateLabel }}
      </span>
      <span>{{ t("shell.connection") }}: {{ store.snapshot?.display_name ?? "-" }}</span>
      <span v-if="(store.snapshot?.generation ?? 0) > 0">
        {{ t("shell.generation") }}: {{ store.snapshot?.generation }}
      </span>
      <div class="spacer"/>
      <span class="muted">{{ store.snapshot?.connection_id ?? "" }}</span>
    </footer>

    <p v-if="toast" class="toast" role="status">
      {{ toast }}
      <button v-if="retryPath" class="tiny" type="button" @click="retryFailedSubtree">
        {{ t("delete.retryFailed") }}
      </button>
    </p>

    <NModal
      v-model:show="customRefreshOpen"
      :close-on-esc="false"
      :style="{width: '320px'}"
      :title="t('data.autoRefreshCustomTitle')"
      preset="card"
    >
      <div class="custom-refresh">
        <span>{{ t("data.autoRefreshInterval") }}</span>
        <NInputNumber
          :max="3600"
          :min="1"
          :show-button="false"
          :value="customRefreshSeconds"
          class="custom-refresh-input"
          @update:value="(value) => applyNumber(value, setCustomRefreshSeconds)"
        />
        <span>{{ t("data.seconds") }}</span>
      </div>
      <template #footer>
        <div class="custom-refresh-actions">
          <NButton @click="customRefreshOpen = false">{{ t("home.cancel") }}</NButton>
          <NButton type="primary" @click="applyCustomRefresh">{{ t("settings.save") }}</NButton>
        </div>
      </template>
    </NModal>

    <ConnectionDialog :open="dialogOpen" @close="dialogOpen = false" @connected="onConnected"/>
    <NodeEditor
        v-if="data"
        :connection-id="connectionId"
        :open="editorOpen"
        :original-text="text"
        :path="selectedPath ?? '/'"
        :version="data.stat.version"
        @close="editorOpen = false"
        @reload="editorOpen = false; loadData(selectedPath ?? '/')"
        @saved="onSaved"
    />
    <NodeBinaryEditor
        v-if="data && !data.is_text"
        :byte-len="data.byte_len"
        :connection-id="connectionId"
        :open="binaryOpen"
        :original-b64="data.data_b64"
        :path="selectedPath ?? '/'"
        :version="data.stat.version"
        @close="binaryOpen = false"
        @saved="binaryOpen = false; onSaved($event)"
    />
    <NodeCreateDialog
        :connection-id="connectionId"
        :default-path="createDefaultPath ?? defaultChildPath"
        :open="createOpen"
        @close="createOpen = false"
        @created="onCreated"
    />
    <NodeDeleteDialog
        v-if="deleteTarget"
        :connection-id="connectionId"
        :has-children="(stat?.num_children ?? 0) > 0 || retryPath !== null"
        :initial-recursive="retryPath !== null"
        :open="deleteOpen"
        :path="deleteTarget"
        @close="deleteOpen = false"
        @deleted="onDeleted"
    />
  </div>
</template>

<style scoped>
.shell {
  display: flex;
  flex-direction: column;
  height: 100%;
  color: #e6e6e6;
}

.topbar {
  position: relative;
  z-index: 30;
  display: flex;
  align-items: center;
  height: 48px;
  padding: 0 12px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 80%);
  backdrop-filter: blur(6px);
}

.conn {
  position: relative;
}

.chip {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 12%);
  background: rgb(255 255 255 / 6%);
  color: inherit;
  cursor: pointer;
}

.chip:hover {
  background: rgb(255 255 255 / 10%);
}

.name {
  font-weight: 600;
}

.endpoint {
  color: #9aa4b2;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}

.caret {
  color: #9aa4b2;
  font-size: 11px;
}

.menu {
  position: absolute;
  top: calc(100% + 6px);
  left: 0;
  width: 320px;
  display: flex;
  flex-direction: column;
  padding: 6px;
  border-radius: 8px;
  border: 1px solid #2a323c;
  background: #171c22;
  box-shadow: 0 18px 40px rgb(0 0 0 / 45%);
  z-index: 20;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: inherit;
  cursor: pointer;
  text-align: left;
}

.menu-item:hover,
.menu-item.active {
  background: rgb(255 255 255 / 8%);
}

.menu-item.danger {
  color: #f87171;
}

.sep {
  height: 1px;
  margin: 4px 2px;
  background: rgb(255 255 255 / 10%);
}

.spacer {
  flex: 1;
}

.workspace-main {
  display: flex;
  flex: 1;
  flex-direction: column;
  min-height: 0;
}

.body {
  display: flex;
  flex: 1;
  min-height: 0;
}

.resizer,
.h-resizer {
  background: var(--panel-bg);
  backdrop-filter: blur(10px) saturate(120%);
}

.command-dock {
  display: flex;
  flex: 0 0 auto;
  min-height: 140px;
  max-height: 75%;
  overflow: hidden;
}

.h-resizer {
  position: relative;
  flex: 0 0 6px;
  margin: 0;
  padding: 0;
  border: none;
  cursor: row-resize;
}

.h-resizer::after {
  content: "";
  position: absolute;
  top: 2px;
  left: 0;
  right: 0;
  height: 2px;
  background: rgb(255 255 255 / 8%);
}

.h-resizer:hover::after,
.h-resizer:focus-visible::after {
  background: #22d3ee;
}

.command-dock > * {
  flex: 1;
  min-width: 0;
  min-height: 0;
}

.tree {
  flex: 0 0 var(--tree-width, 22%);
  min-width: 12rem;
  max-width: 45%;
  display: flex;
  flex-direction: column;
  padding: 12px 8px 12px 12px;
  border-right: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 45%);
}

.aside-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-weight: 600;
  margin-bottom: 8px;
  padding-right: 4px;
}

.tree > .aside-head {
  flex-direction: column;
  align-items: stretch;
  gap: 6px;
}

.tiny {
  padding: 2px 8px;
  border-radius: 5px;
  border: 1px solid rgb(255 255 255 / 15%);
  background: transparent;
  color: #cbd5e1;
  cursor: pointer;
  font-size: 12px;
}

.tiny:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.tiny.danger {
  color: #f87171;
  border-color: rgb(248 113 113 / 40%);
}

.jump {
  display: flex;
  gap: 4px;
  margin-bottom: 8px;
}

.jump input {
  flex: 1;
  min-width: 0;
  padding: 4px 7px;
  border-radius: 5px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}

.tree-actions {
  display: flex;
  gap: 4px;
}

.tree-actions :deep(.n-button) {
  flex: 1;
  min-width: 0;
  white-space: nowrap;
}

.tree-scroll {
  padding: 0.5rem 1.2rem 0.5rem 0;
  flex: 1;
  overflow: auto;
}

.content {
  flex: 1;
  min-width: 0;
  overflow: auto;
  padding: 20px 24px;
}

.placeholder {
  display: grid;
  place-items: center;
  height: 100%;
  text-align: center;
}

.placeholder h2 {
  margin: 0 0 6px;
  font-size: 18px;
}

.detail {
  display: flex;
  flex-direction: column;
  gap: 10px;
  height: 100%;
}

.detail-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.path-button,
.path {
  margin: 0;
  border: none;
  background: transparent;
  color: inherit;
  cursor: pointer;
  font-family: ui-monospace, monospace;
  font-size: 17px;
  padding: 0;
  text-align: left;
  word-break: break-all;
}

.toolbar {
  display: flex;
  gap: 6px;
  flex: none;
}

.refresh-control {
  position: relative;
  display: inline-flex;
  align-items: center;
}

.refresh-control :deep(.n-button) {
  padding-right: 14px;
}

.clock-icon {
  width: 12px;
  height: 12px;
  fill: none;
  stroke: currentColor;
  stroke-linecap: round;
  stroke-linejoin: round;
  stroke-width: 1.4;
}

.refresh-control.active .clock-icon {
  color: #22d3ee;
}

.refresh-caret {
  position: absolute;
  right: 0;
  bottom: 0;
  z-index: 2;
  width: 10px;
  height: 10px;
  padding: 0;
  border: none;
  background: transparent;
  cursor: pointer;
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
}

.refresh-caret-triangle {
  width: 0;
  height: 0;
  border-left: 3px solid transparent;
  border-right: 3px solid transparent;
  border-top: 4px solid #9aa4b2;
}

.refresh-control.active .refresh-caret-triangle {
  border-top-color: #22d3ee;
}

.custom-refresh {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
}

.custom-refresh-input {
  width: 120px;
}

.custom-refresh-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.tabs {
  display: flex;
  gap: 4px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
}

.tabs button {
  padding: 6px 12px;
  border: none;
  background: transparent;
  color: #9aa4b2;
  cursor: pointer;
  border-bottom: 2px solid transparent;
}

.tabs button.active {
  color: inherit;
  border-bottom-color: #2563eb;
}

.data-pane {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.format-select {
  padding: 4px 8px;
  border-radius: 5px;
  border: 1px solid var(--control-border, #3b4654);
  background: var(--control-bg, #0d1117);
  color: var(--control-fg, #e6edf3);
  font-family: ui-monospace, monospace;
  font-size: 12px;
}

.data-toolbar {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px 12px;
  padding-top: 8px;
  border-top: 1px dashed rgb(255 255 255 / 18%);
}

.format-group {
  display: flex;
  gap: 4px;
  margin-left: auto;
}


.binary-meta {
  margin: 0;
  font-family: ui-monospace, monospace;
}

.text,
.hex {
  margin: 0;
  padding: 12px;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 70%);
  font-family: ui-monospace, monospace;
  font-size: 13px;
  white-space: pre-wrap;
  word-break: break-all;
  max-height: 60vh;
  overflow: auto;
}

.binary {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.warn {
  color: #fbbf24;
  margin: 0;
  font-size: 13px;
}

.acl-pane {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.acl-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.acl-table th,
.acl-table td {
  text-align: left;
  padding: 6px 8px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
}

.mono {
  font-family: ui-monospace, monospace;
}

.hint {
  color: #9aa4b2;
  font-size: 12px;
}

.stat-groups {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.stat-group h3 {
  margin: 0 0 6px;
  font-size: 12px;
  font-weight: 600;
  color: #9aa4b2;
}

.stat-table {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2px 18px;
  margin: 0;
}

.stat-table div {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  padding: 3px 0;
  border-bottom: 1px solid rgb(255 255 255 / 5%);
  font-size: 12px;
}

.stat-table dt {
  color: #9aa4b2;
}

.stat-table dd {
  margin: 0;
}

.stat-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 10px 20px;
  margin: 0;
}

.stat-grid div {
  padding: 8px 10px;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 55%);
}

.stat-grid dt {
  color: #9aa4b2;
  font-size: 12px;
  margin-bottom: 4px;
}

.stat-grid dd {
  margin: 0;
  font-family: ui-monospace, monospace;
}

.node-tabs {
  display: flex;
  gap: 4px;
  overflow-x: auto;
}

.node-tab {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  border: 1px solid rgb(255 255 255 / 10%);
  border-radius: 6px 6px 0 0;
  background: rgb(16 20 24 / 60%);
  color: #cbd5e1;
  cursor: pointer;
  font-family: ui-monospace, monospace;
  font-size: 12px;
  white-space: nowrap;
}

.node-tab.active {
  background: rgb(37 99 235 / 25%);
  color: #fff;
}

.node-tab .close {
  color: #9aa4b2;
}

.inspector {
  flex: 0 0 var(--inspector-width, 20%);
  min-width: 12rem;
  padding: 14px 22px;
  overflow: auto;
  border-left: 1px solid rgb(255 255 255 / 8%);
  background: rgb(10 13 16 / 70%);
}

.resizer {
  position: relative;
  flex: 0 0 6px;
  padding: 0;
  border: none;
  cursor: col-resize;
}

.resizer::after {
  content: "";
  position: absolute;
  top: 0;
  bottom: 0;
  left: 1px;
  width: 1px;
  background: rgb(255 255 255 / 8%);
}

.resizer:hover::after {
  background: #22d3ee;
}

.side-tabs {
  display: flex;
  gap: 4px;
  margin-top: 14px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
}

.side-tabs button {
  flex: 1;
  padding: 6px 4px;
  border: none;
  border-bottom: 2px solid transparent;
  background: transparent;
  color: #9aa4b2;
  cursor: pointer;
  font-size: 11px;
}

.side-tabs button.active {
  color: #fff;
  border-bottom-color: #22d3ee;
}

.side-body {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 6px 2px 6px 0;
}

.side-body .timeline {
  font-size: 11px;
}

.side-body .history-line {
  font-size: 11px;
}

.side-body .timeline .muted {
  font-size: 11px;
}

.inspector {
  display: flex;
  flex-direction: column;
}

.inspector-grid {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin: 0;
}

.inspector-grid div {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  font-size: 12px;
  padding: 3px 0;
  border-bottom: 1px solid rgb(255 255 255 / 5%);
}

.inspector-grid dt {
  flex: 0 0 auto;
  color: #9aa4b2;
  white-space: nowrap;
}

.inspector-grid dd {
  flex: 1;
  min-width: 0;
  margin: 0;
  text-align: right;
  overflow-wrap: anywhere;
  word-break: break-word;
}

.mono {
  font-family: ui-monospace, monospace;
}

.inspector-toggle {
  width: 100%;
  padding: 0;
  border: none;
  background: transparent;
  color: inherit;
  cursor: pointer;
  font: inherit;
  text-align: left;
}

.chevron {
  color: #9aa4b2;
  font-size: 11px;
}

.section-toggle {
  margin-top: 16px;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  text-align: left;
  border: none;
  background: transparent;
  color: #9aa4b2;
  cursor: pointer;
  font-size: 12px;
  padding: 4px 0;
}

.history-line {
  flex: 1;
  min-width: 0;
  border: none;
  background: transparent;
  color: #cbd5e1;
  cursor: pointer;
  padding: 0;
  text-align: left;
  line-height: 1.45;
  overflow-wrap: anywhere;
  white-space: normal;
}

.audit-link {
  margin-top: 6px;
}

.history-line:hover {
  color: #fff;
  text-decoration: underline;
}

.timeline {
  margin: 0;
  padding: 0;
  list-style: none;
  font-size: 12px;
}

.timeline li {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 8px;
  padding: 3px 0;
  border-bottom: 1px solid rgb(255 255 255 / 5%);
}

.tools {
  position: relative;
}

.tools-menu {
  position: absolute;
  z-index: 100;
  right: 0;
  top: calc(100% + 6px);
  display: flex;
  flex-direction: column;
  min-width: 10rem;
  padding: 4px;
  border: 1px solid #2a323c;
  border-radius: 6px;
  background: #161b22;
}

.tools-menu button {
  text-align: left;
  padding: 6px 8px;
  border: none;
  border-radius: 4px;
  background: transparent;
  color: inherit;
  cursor: pointer;
}

.tools-menu button:hover {
  background: rgb(255 255 255 / 8%);
}

.tools-sep {
  height: 1px;
  margin: 4px 2px;
  background: rgb(255 255 255 / 10%);
}

.statusbar .state {
  display: flex;
  align-items: center;
  gap: 6px;
}

.statusbar .dot {
  display: inline-block;
}

.statusbar {
  display: flex;
  align-items: center;
  gap: 16px;
  height: 28px;
  padding: 0 12px;
  font-size: 12px;
  color: #9aa4b2;
  border-top: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 80%);
}

.toast {
  position: fixed;
  top: 64px;
  right: 16px;
  max-width: 32rem;
  z-index: 50;
  padding: 8px 14px;
  border-radius: 6px;
  background: #1d242c;
  border: 1px solid #2a323c;
  font-size: 13px;
}

.muted {
  color: #9aa4b2;
  font-size: 12px;
}

.error {
  color: #ef4444;
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #6b7280;
  flex: none;
}

.dot[data-state="connected"] {
  background: #22c55e;
}

.dot[data-state="disconnected"],
.dot[data-state="expired"] {
  background: #ef4444;
}

.ghost {
  padding: 6px 12px;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 18%);
  background: transparent;
  color: inherit;
  cursor: pointer;
}
</style>
