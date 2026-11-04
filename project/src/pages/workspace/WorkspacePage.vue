<script setup lang="ts">
import hljs from "highlight.js/lib/core";
import jsonLanguage from "highlight.js/lib/languages/json";
import yamlLanguage from "highlight.js/lib/languages/yaml";
import { computed, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import { isMap, isSeq, parseDocument } from "yaml";
import { auditList } from "@/api/audit";
import { type InvokeRejection, toAppError } from "@/api/client";
import { nodeAcl, nodeData, nodeStat } from "@/api/node";
import type { AclEntry } from "@/api/types/AclEntry";
import type { AppError } from "@/api/types/AppError";
import type { AuditRecord } from "@/api/types/AuditRecord";
import type { ConnectionSnapshot } from "@/api/types/ConnectionSnapshot";
import type { DeleteOutcome } from "@/api/types/DeleteOutcome";
import type { NodeData } from "@/api/types/NodeData";
import type { NodeStat } from "@/api/types/NodeStat";
import CommandPanel from "@/components/CommandPanel.vue";
import ConnectionDialog from "@/components/ConnectionDialog.vue";
import NodeBinaryEditor from "@/components/NodeBinaryEditor.vue";
import NodeCreateDialog from "@/components/NodeCreateDialog.vue";
import NodeDeleteDialog from "@/components/NodeDeleteDialog.vue";
import NodeEditor from "@/components/NodeEditor.vue";
import TreeNodeItem from "@/components/TreeNodeItem.vue";
import { useConnectionStore } from "@/stores/connection";
import { useTabsStore } from "@/stores/tabs";
import { decodeUtf8Base64, hexPreview } from "@/utils/encoding";
import { toRfc3339 } from "@/utils/time";

hljs.registerLanguage("json", jsonLanguage);
hljs.registerLanguage("yaml", yamlLanguage);

const { t } = useI18n();
const router = useRouter();
const store = useConnectionStore();
const tabs = useTabsStore();

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
const jumpPath = ref("");
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
    return document.toString({ indent: 2 });
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
  return hljs.highlight(displayText.value, { language: format.value }).value;
});
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

function select(path: string): void {
  selectedPath.value = path;
  activeTab.value = "data";
  format.value = "raw";
  acls.value = null;
  aclError.value = null;
  if (connectionId.value) {
    tabs.open(connectionId.value, path);
  }
  void loadStat(path);
  void loadTimeline(path);
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

function loadFull(): void {
  if (selectedPath.value) {
    void loadData(selectedPath.value, true);
  }
}

function refresh(): void {
  treeKey.value += 1;
  if (selectedPath.value) {
    void loadStat(selectedPath.value);
  }
}

watch(connectionId, () => {
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
  retryPath.value = outcome.failed_path ?? null;
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
                :disabled="!data?.is_text || data.truncated"
                :title="data?.is_text === false ? t('data.notEditable') : ''"
                @click="editorOpen = true"
              >
                {{ t("data.edit") }}
              </button>
              <button type="button" class="tiny danger" @click="openDelete">
                {{ t("data.delete") }}
              </button>
            </div>
          </div>

          <div class="node-tabs">
            <button
              v-for="tab in tabs.tabsFor(connectionId)"
              :key="tab"
              type="button"
              class="node-tab"
              :class="{ active: tab === selectedPath }"
              @click="select(tab)"
            >
              {{ tab }}
              <button
                class="close"
                type="button"
                :aria-label="t('shell.closeTab')"
                @click.stop="closeTab(tab)"
              >
                ×
              </button>
            </button>
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
            <button
              type="button"
              :class="{ active: activeTab === 'acl' }"
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
                <p class="warn">{{ t("data.largeHint", { bytes: stat?.data_length }) }}</p>
                <button type="button" class="tiny" @click="loadData(selectedPath ?? '/')">
                  {{ t("data.loadData") }}
                </button>
              </div>
              <template v-else>
              <div v-if="!data.is_text" class="binary">
                <p class="warn">{{ t("data.binaryHint") }}</p>
                <p class="muted">{{ t("data.bytes") }}: {{ data.byte_len }}</p>
                <div>
                  <button type="button" class="tiny" @click="openBinaryEditor">
                    {{ t("data.replaceBinary") }}
                  </button>
                </div>
                <pre class="hex">{{ hexDump }}</pre>
              </div>
              <template v-else>
                <div class="data-toolbar">
                  <span class="muted">{{ t("data.bytes") }}: {{ data.byte_len }}</span>
                  <select v-model="format" class="format-select">
                    <option value="raw">{{ t("data.formatRaw") }}</option>
                    <option value="json">JSON</option>
                    <option value="yaml">YAML</option>
                  </select>
                  <template v-if="data.truncated">
                    <span class="warn">{{ t("data.truncated") }}</span>
                    <button type="button" class="tiny" @click="loadFull">
                      {{ t("data.loadFull") }}
                    </button>
                  </template>
                </div>
                <pre v-if="format === 'raw'" class="text">{{ displayText || t("data.empty") }}</pre>
                <pre v-else class="text hljs" v-html="highlighted" />
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
              <div><dt>{{ t("shell.mtime") }}</dt><dd>{{ stat ? toRfc3339(stat.mtime) : "" }}</dd></div>
            </dl>
          </template>
        </div>
      </section>

      <aside class="inspector">
        <div class="aside-head">{{ t("shell.inspector") }}</div>
        <dl v-if="stat" class="inspector-grid">
          <div><dt>{{ t("shell.path") }}</dt><dd class="mono">{{ stat.path }}</dd></div>
          <div><dt>{{ t("shell.version") }}</dt><dd>{{ stat.version }}</dd></div>
          <div><dt>{{ t("shell.cversion") }}</dt><dd>{{ stat.cversion }}</dd></div>
          <div><dt>{{ t("shell.children") }}</dt><dd>{{ stat.num_children }}</dd></div>
          <div><dt>{{ t("shell.dataLength") }}</dt><dd>{{ stat.data_length }}</dd></div>
          <div><dt>{{ t("shell.ephemeral") }}</dt><dd>{{ stat.ephemeral_owner === 0 ? t("shell.persistent") : stat.ephemeral_owner }}</dd></div>
          <div><dt>{{ t("shell.mtime") }}</dt><dd>{{ toRfc3339(stat.mtime) }}</dd></div>
        </dl>
        <div class="aside-head timeline-head">{{ t("shell.timeline") }}</div>
        <ul v-if="timeline.length" class="timeline">
          <li v-for="entry in timeline" :key="entry.id">
            <span class="mono">{{ entry.op }}</span>
            <span class="muted">{{ toRfc3339(entry.ts) }}</span>
          </li>
        </ul>
        <p v-else class="muted">{{ t("shell.timelineEmpty") }}</p>
      </aside>
    </div>

    <CommandPanel
      v-if="connectionId"
      :connection-id="connectionId"
      :connection-name="store.snapshot?.display_name ?? ''"
      @refresh="refresh"
    />

    <footer class="statusbar">
      <span class="state">
        <span class="dot" :data-state="store.snapshot?.state ?? 'none'" />
        {{ stateLabel }}
      </span>
      <span>{{ t("shell.connection") }}: {{ store.snapshot?.display_name ?? "-" }}</span>
      <span v-if="(store.snapshot?.generation ?? 0) > 0">
        {{ t("shell.generation") }}: {{ store.snapshot?.generation }}
      </span>
      <div class="spacer" />
      <span class="muted">{{ store.snapshot?.connection_id ?? "" }}</span>
    </footer>

    <p v-if="toast" class="toast">
      {{ toast }}
      <button v-if="retryPath" type="button" class="tiny" @click="retryFailedSubtree">
        {{ t("delete.retryFailed") }}
      </button>
    </p>

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
    <NodeBinaryEditor
      v-if="data && !data.is_text"
      :open="binaryOpen"
      :connection-id="connectionId"
      :path="selectedPath ?? '/'"
      :version="data.stat.version"
      :original-b64="data.data_b64"
      :byte-len="data.byte_len"
      @close="binaryOpen = false"
      @saved="binaryOpen = false; onSaved($event)"
    />
    <NodeCreateDialog
      :open="createOpen"
      :connection-id="connectionId"
      :default-path="defaultChildPath"
      @close="createOpen = false"
      @created="onCreated"
    />
    <NodeDeleteDialog
      v-if="deleteTarget"
      :open="deleteOpen"
      :connection-id="connectionId"
      :path="deleteTarget"
      :has-children="(stat?.num_children ?? 0) > 0 || retryPath !== null"
      :initial-recursive="retryPath !== null"
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
  width: 260px;
  padding: 12px;
  overflow: auto;
  border-left: 1px solid rgb(255 255 255 / 8%);
  background: rgb(10 13 16 / 70%);
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
  gap: 8px;
  font-size: 12px;
  padding: 3px 0;
  border-bottom: 1px solid rgb(255 255 255 / 5%);
}
.inspector-grid dt {
  color: #9aa4b2;
}
.inspector-grid dd {
  margin: 0;
}
.mono {
  font-family: ui-monospace, monospace;
}
.timeline-head {
  margin-top: 16px;
}
.timeline {
  margin: 0;
  padding: 0;
  list-style: none;
  font-size: 12px;
}
.timeline li {
  display: flex;
  justify-content: space-between;
  padding: 3px 0;
  border-bottom: 1px solid rgb(255 255 255 / 5%);
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
