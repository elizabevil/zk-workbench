<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { type InvokeRejection, toAppError } from "@/api/client";
import { nodeData } from "@/api/node";
import type { AppError } from "@/api/types/AppError";
import type { ConnectionSnapshot } from "@/api/types/ConnectionSnapshot";
import type { DeleteOutcome } from "@/api/types/DeleteOutcome";
import type { NodeData } from "@/api/types/NodeData";
import type { NodeStat } from "@/api/types/NodeStat";
import ConnectionDialog from "@/components/ConnectionDialog.vue";
import NodeCreateDialog from "@/components/NodeCreateDialog.vue";
import NodeDeleteDialog from "@/components/NodeDeleteDialog.vue";
import NodeEditor from "@/components/NodeEditor.vue";
import TreeNodeItem from "@/components/TreeNodeItem.vue";
import { useConnectionStore } from "@/stores/connection";
import { decodeUtf8Base64, hexPreview } from "@/utils/encoding";

const { t } = useI18n();
const router = useRouter();
const store = useConnectionStore();

const menuOpen = ref(false);
const dialogOpen = ref(false);
const treeKey = ref(0);
const selectedPath = ref<string | null>("/");
const data = ref<NodeData | null>(null);
const dataError = ref<AppError | null>(null);
const activeTab = ref<"data" | "stat">("data");
const formatted = ref(false);
const loadingData = ref(false);
const editorOpen = ref(false);
const createOpen = ref(false);
const deleteOpen = ref(false);
const toast = ref("");
const jumpPath = ref("");

const connectionId = computed(() => store.snapshot?.connection_id ?? "");
const stat = computed<NodeStat | null>(() => data.value?.stat ?? null);
const stateLabel = computed(() =>
  store.snapshot ? t(`connection.state.${store.snapshot.state}`) : t("shell.noConnection"),
);

const text = computed(() =>
  data.value?.is_text ? decodeUtf8Base64(data.value.data_b64) : "",
);
const pretty = computed(() => {
  if (!text.value) {
    return null;
  }
  try {
    return JSON.stringify(JSON.parse(text.value), null, 2);
  } catch {
    return null;
  }
});
const displayText = computed(() => (formatted.value && pretty.value ? pretty.value : text.value));
const hexDump = computed(() => (data.value ? hexPreview(data.value.data_b64) : ""));
const defaultChildPath = computed(() =>
  selectedPath.value === "/" ? "/new" : `${selectedPath.value}/new`,
);

async function loadData(path: string): Promise<void> {
  if (!connectionId.value) {
    return;
  }
  loadingData.value = true;
  dataError.value = null;
  try {
    data.value = await nodeData(connectionId.value, path);
  } catch (raw) {
    data.value = null;
    dataError.value = toAppError(raw as InvokeRejection);
  } finally {
    loadingData.value = false;
  }
}

function select(path: string): void {
  selectedPath.value = path;
  activeTab.value = "data";
  formatted.value = false;
  void loadData(path);
}

async function copyAddress(): Promise<void> {
  if (!store.snapshot) {
    return;
  }
  const address = `${store.snapshot.host}:${store.snapshot.port}`;
  try {
    await navigator.clipboard.writeText(address);
    toast.value = t("shell.copied", { address });
  } catch {
    toast.value = t("shell.copyFailed");
  }
  menuOpen.value = false;
}

function jump(): void {
  const raw = jumpPath.value.trim();
  if (!raw) {
    return;
  }
  const normalized = raw.startsWith("/") ? raw : `/${raw}`;
  const path = normalized.length > 1 ? normalized.replace(/\/+$/, "") : "/";
  select(path);
}

function refresh(): void {
  treeKey.value += 1;
  if (selectedPath.value) {
    void loadData(selectedPath.value);
  }
}

watch(connectionId, () => {
  treeKey.value += 1;
  selectedPath.value = "/";
  data.value = null;
  void loadData("/");
});

watch(
  () => store.snapshot?.generation,
  () => {
    treeKey.value += 1;
    if (selectedPath.value) {
      void loadData(selectedPath.value);
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
  void loadData("/");
});

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

function onSaved(stat: NodeStat): void {
  editorOpen.value = false;
  toast.value = t("editor.saved", { version: stat.version });
  if (selectedPath.value) {
    void loadData(selectedPath.value);
  }
}

function onCreated(actualPath: string): void {
  createOpen.value = false;
  toast.value = t("create.created", { path: actualPath });
  treeKey.value += 1;
  select(actualPath);
}

function onDeleted(outcome: DeleteOutcome): void {
  deleteOpen.value = false;
  toast.value = outcome.failed_path
    ? t("delete.failed", { path: outcome.failed_path, done: outcome.deleted })
    : t("delete.done", { done: outcome.deleted });
  treeKey.value += 1;
  const parent = selectedPath.value?.replace(/\/[^/]+$/, "") || "/";
  select(parent || "/");
}
</script>

<template>
  <div class="shell">
    <header class="topbar">
      <div class="conn">
        <button class="chip" type="button" @click="menuOpen = !menuOpen">
          <span class="dot" :data-state="store.snapshot?.state ?? 'none'" />
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
            type="button"
            class="menu-item"
            :class="{ active: item.connection_id === store.snapshot?.connection_id }"
            @click="switchTo(item)"
          >
            <span class="dot" :data-state="item.state" />
            <span>{{ item.display_name }}</span>
            <span class="muted">{{ item.host }}:{{ item.port }}</span>
          </button>
          <div class="sep" />
          <button
            v-if="store.snapshot"
            type="button"
            class="menu-item"
            @click="copyAddress"
          >
            {{ t("shell.copyAddress") }}
          </button>
          <button type="button" class="menu-item" @click="dialogOpen = true; menuOpen = false">
            {{ t("shell.newConnection") }}
          </button>
          <button type="button" class="menu-item" @click="router.push('/')">
            {{ t("shell.home") }}
          </button>
          <button type="button" class="menu-item danger" @click="disconnect">
            {{ t("shell.disconnect") }}
          </button>
        </div>
      </div>

      <div class="spacer" />
      <button type="button" class="ghost" @click="router.push('/settings')">
        {{ t("home.settings") }}
      </button>
    </header>

    <div class="body">
      <aside class="tree">
        <div class="aside-head">
          <span>{{ t("shell.tree") }}</span>
          <button type="button" class="tiny" @click="refresh">{{ t("shell.refresh") }}</button>
        </div>
        <form class="jump" @submit.prevent="jump">
          <input v-model="jumpPath" :placeholder="t('shell.jumpPlaceholder')" />
          <button type="submit" class="tiny">{{ t("shell.go") }}</button>
        </form>
        <div v-if="connectionId" class="tree-scroll">
          <TreeNodeItem
            :key="`${connectionId}-${store.snapshot?.generation ?? 0}-${treeKey}`"
            :connection-id="connectionId"
            name="/"
            path="/"
            :depth="0"
            @select="select"
            @error="dataError = $event"
          />
        </div>
        <p v-else class="muted">{{ t("shell.noConnection") }}</p>
      </aside>

      <section class="content">
        <div v-if="!selectedPath" class="placeholder">
          <h2>{{ t("shell.contentTitle") }}</h2>
          <p class="muted">{{ t("shell.contentHint") }}</p>
        </div>

        <div v-else class="detail">
          <div class="detail-head">
            <h2 class="path">{{ selectedPath }}</h2>
            <div class="toolbar">
              <button type="button" class="tiny" @click="refresh">{{ t("data.refresh") }}</button>
              <button type="button" class="tiny" @click="createOpen = true">
                {{ t("data.create") }}
              </button>
              <button
                type="button"
                class="tiny"
                :disabled="!data || !data.is_text"
                :title="data && !data.is_text ? t('data.notEditable') : ''"
                @click="editorOpen = true"
              >
                {{ t("data.edit") }}
              </button>
              <button type="button" class="tiny danger" @click="deleteOpen = true">
                {{ t("data.delete") }}
              </button>
            </div>
          </div>

          <div class="tabs">
            <button
              type="button"
              :class="{ active: activeTab === 'data' }"
              @click="activeTab = 'data'"
            >
              {{ t("data.tab") }}
            </button>
            <button
              type="button"
              :class="{ active: activeTab === 'stat' }"
              @click="activeTab = 'stat'"
            >
              {{ t("data.statTab") }}
            </button>
          </div>

          <p v-if="dataError" class="error">{{ t(dataError.message_key) }}</p>
          <p v-else-if="loadingData" class="muted">{{ t("delete.loading") }}</p>

          <template v-else-if="data">
            <div v-if="activeTab === 'data'" class="data-pane">
              <div v-if="!data.is_text" class="binary">
                <p class="warn">{{ t("data.binaryHint") }}</p>
                <p class="muted">{{ t("data.bytes") }}: {{ data.byte_len }}</p>
                <pre class="hex">{{ hexDump }}</pre>
              </div>
              <template v-else>
                <div class="data-toolbar">
                  <span class="muted">{{ t("data.bytes") }}: {{ data.byte_len }}</span>
                  <button
                    v-if="pretty"
                    type="button"
                    class="tiny"
                    @click="formatted = !formatted"
                  >
                    {{ formatted ? t("data.raw") : t("data.formatted") }}
                  </button>
                </div>
                <pre class="text">{{ displayText || t("data.empty") }}</pre>
              </template>
            </div>

            <dl v-else class="stat-grid">
              <div><dt>{{ t("shell.version") }}</dt><dd>{{ stat?.version }}</dd></div>
              <div><dt>{{ t("shell.cversion") }}</dt><dd>{{ stat?.cversion }}</dd></div>
              <div><dt>{{ t("shell.children") }}</dt><dd>{{ stat?.num_children }}</dd></div>
              <div><dt>{{ t("shell.dataLength") }}</dt><dd>{{ stat?.data_length }}</dd></div>
              <div>
                <dt>{{ t("shell.ephemeral") }}</dt>
                <dd>
                  {{ stat?.ephemeral_owner === 0 ? t("shell.persistent") : stat?.ephemeral_owner }}
                </dd>
              </div>
              <div><dt>{{ t("shell.mtime") }}</dt><dd>{{ stat ? new Date(stat.mtime).toLocaleString() : "" }}</dd></div>
            </dl>
          </template>
        </div>
      </section>
    </div>

    <footer class="statusbar">
      <span>{{ t("shell.state") }}: {{ stateLabel }}</span>
      <span>generation: {{ store.snapshot?.generation ?? "-" }}</span>
      <span v-if="store.snapshot">
        {{ t("shell.session") }}: {{ Math.round(store.snapshot.session_timeout_ms / 1000) }}s
      </span>
      <div class="spacer" />
      <span>{{ store.snapshot?.connection_id ?? "" }}</span>
    </footer>

    <p v-if="toast" class="toast">{{ toast }}</p>

    <ConnectionDialog :open="dialogOpen" @close="dialogOpen = false" @connected="onConnected" />
    <NodeEditor
      v-if="data"
      :open="editorOpen"
      :connection-id="connectionId"
      :path="selectedPath ?? '/'"
      :original-text="text"
      :version="data.stat.version"
      @close="editorOpen = false"
      @saved="onSaved"
      @reload="editorOpen = false; loadData(selectedPath ?? '/')"
    />
    <NodeCreateDialog
      :open="createOpen"
      :connection-id="connectionId"
      :default-path="defaultChildPath"
      @close="createOpen = false"
      @created="onCreated"
    />
    <NodeDeleteDialog
      v-if="selectedPath"
      :open="deleteOpen"
      :connection-id="connectionId"
      :path="selectedPath"
      :has-children="(stat?.num_children ?? 0) > 0"
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
.body {
  display: flex;
  flex: 1;
  min-height: 0;
}
.tree {
  width: 280px;
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
.tree-scroll {
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
.path {
  margin: 0;
  font-family: ui-monospace, monospace;
  font-size: 17px;
  word-break: break-all;
}
.toolbar {
  display: flex;
  gap: 6px;
  flex: none;
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
.data-toolbar {
  display: flex;
  align-items: center;
  gap: 12px;
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
  bottom: 40px;
  left: 50%;
  transform: translateX(-50%);
  padding: 8px 14px;
  border-radius: 6px;
  background: #1d242c;
  border: 1px solid #2a323c;
  font-size: 13px;
  z-index: 50;
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
