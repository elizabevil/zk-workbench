<script lang="ts" setup>
import {NButton, NDataTable, NInput, NInputNumber, type DataTableColumns} from "naive-ui";
import {computed, h, onMounted, onUnmounted, ref} from "vue";
import {useI18n} from "vue-i18n";
import {useRouter} from "vue-router";

import {auditClear, auditExport, auditList, auditPrune, auditVerify} from "@/api/audit";
import {type InvokeRejection, toAppError} from "@/api/client";
import type {AuditRecord} from "@/api/types/AuditRecord";
import {KEY_ESCAPE} from "@/constants/keyboard";
import {useSettingsStore} from "@/stores/settings";
import {applyNumber} from "@/utils/number";
import {toRfc3339} from "@/utils/time";

const {t} = useI18n();
const router = useRouter();

function goBack(): void {
  if (window.history.length > 1) {
    router.back();
  } else {
    void router.push("/");
  }
}

const store = useSettingsStore();
const rows = ref<AuditRecord[]>([]);
const message = ref("");
const exportPath = ref("");

const columns = computed<DataTableColumns<AuditRecord>>(() => [
  {
    title: t("settings.audit.time"),
    key: "ts",
    width: 190,
    render: (row) => h("span", {class: "mono"}, toRfc3339(row.ts)),
  },
  {title: t("settings.audit.op"), key: "op", width: 90},
  {
    title: t("settings.audit.path"),
    key: "path",
    minWidth: 180,
    ellipsis: {tooltip: true},
    render: (row) => h("span", {class: "mono"}, row.path),
  },
  {title: t("settings.audit.result"), key: "result", width: 90},
  {title: t("settings.audit.actor"), key: "actor", width: 130, ellipsis: {tooltip: true}},
  {
    title: t("settings.audit.detail"),
    key: "detail",
    minWidth: 180,
    ellipsis: {tooltip: true},
  },
]);

async function load(): Promise<void> {
  rows.value = await auditList(200, 0, null);
}

async function verify(): Promise<void> {
  const report = await auditVerify();
  message.value = report.ok
    ? t("settings.audit.verified", {count: report.checked})
    : t("settings.audit.broken", {id: report.broken_at});
}

async function clear(): Promise<void> {
  if (!window.confirm(t("settings.audit.confirmClear"))) {
    return;
  }
  const cleared = await auditClear();
  message.value = t("settings.audit.cleared", {count: cleared});
  await load();
}

async function prune(): Promise<void> {
  const days = store.settings?.audit.retention_days ?? 90;
  const removed = await auditPrune(days);
  message.value = t("settings.audit.pruned", {count: removed});
  await load();
}

async function exportLog(): Promise<void> {
  try {
    await auditExport(exportPath.value.trim());
    message.value = t("settings.audit.exported");
  } catch (raw) {
    message.value = t(toAppError(raw as InvokeRejection).message_key);
  }
}

async function saveRetention(): Promise<void> {
  await store.save();
  message.value = t("settings.saved");
}

function setRetention(value: number): void {
  if (store.settings) {
    store.settings.audit.retention_days = value;
  }
}

function onEscape(event: KeyboardEvent): void {
  if (event.key === KEY_ESCAPE) {
    goBack();
  }
}

onMounted(async () => {
  await store.load();
  await load();
  window.addEventListener("keydown", onEscape);
});

onUnmounted(() => window.removeEventListener("keydown", onEscape));
</script>

<template>
  <main class="page audit-page">
    <header class="head">
      <div>
        <h1>{{ t("settings.audit.title") }}</h1>
        <p class="hint">{{ t("settings.audit.hint") }}</p>
      </div>
      <NButton size="small" @click="goBack">{{ t("settings.back") }}</NButton>
    </header>

    <section v-if="store.settings" class="bar">
      <label class="retention">
        <span>{{ t("settings.audit.retention") }}</span>
        <NInputNumber
          :min="1"
          :show-button="false"
          :value="store.settings.audit.retention_days"
          class="retention-input"
          size="small"
          @update:value="(value) => applyNumber(value, setRetention)"
        />
      </label>
      <NButton size="small" @click="saveRetention">{{ t("settings.save") }}</NButton>
      <NButton size="small" @click="load">{{ t("settings.audit.refresh") }}</NButton>
      <NButton size="small" @click="verify">{{ t("settings.audit.verify") }}</NButton>
      <NButton size="small" @click="prune">{{ t("settings.audit.prune") }}</NButton>
      <NButton size="small" type="error" @click="clear">{{ t("settings.audit.clear") }}</NButton>
    </section>

    <section class="bar">
      <NInput
        v-model:value="exportPath"
        :placeholder="t('settings.audit.exportPath')"
        class="export-input"
        size="small"
      />
      <NButton size="small" @click="exportLog">{{ t("settings.audit.export") }}</NButton>
      <span v-if="message" class="hint">{{ message }}</span>
    </section>

    <div class="table-wrap">
      <NDataTable
        :columns="columns"
        :data="rows"
        :row-key="(row: AuditRecord) => row.id"
        flex-height
        size="small"
      >
        <template #empty>
          <p class="hint">{{ t("settings.audit.empty") }}</p>
        </template>
      </NDataTable>
    </div>
  </main>
</template>

<style scoped>
.page {
  box-sizing: border-box;
  height: 100%;
  overflow-y: auto;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  background: var(--panel-bg, rgb(0 0 0 / 50%));
  backdrop-filter: blur(10px) saturate(120%);
}

.head {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
}

h1 {
  margin: 0 0 4px;
  font-size: 20px;
}

.hint {
  color: #9aa4b2;
  margin: 0;
  font-size: 12px;
}

.bar {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}

.retention {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #cbd5e1;
  font-size: 13px;
}

.retention-input {
  width: 110px;
}

.export-input {
  width: 280px;
  max-width: 100%;
}

.table-wrap {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.table-wrap :deep(.n-data-table) {
  height: 100%;
}

.table-wrap :deep(.n-data-table-th) {
  background: rgb(255 255 255 / 4%);
}

.mono {
  font-family: ui-monospace, monospace;
}
</style>
