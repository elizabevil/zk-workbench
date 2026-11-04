<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { auditClear, auditExport, auditList, auditPrune, auditVerify } from "@/api/audit";
import { type InvokeRejection, toAppError } from "@/api/client";
import type { AuditRecord } from "@/api/types/AuditRecord";
import { useSettingsStore } from "@/stores/settings";
import { toRfc3339 } from "@/utils/time";

const { t } = useI18n();
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

async function load(): Promise<void> {
  rows.value = await auditList(200, 0, null);
}

async function verify(): Promise<void> {
  const report = await auditVerify();
  message.value = report.ok
    ? t("settings.audit.verified", { count: report.checked })
    : t("settings.audit.broken", { id: report.broken_at });
}

async function clear(): Promise<void> {
  if (!window.confirm(t("settings.audit.confirmClear"))) {
    return;
  }
  const cleared = await auditClear();
  message.value = t("settings.audit.cleared", { count: cleared });
  await load();
}

async function prune(): Promise<void> {
  const days = store.settings?.audit.retention_days ?? 90;
  const removed = await auditPrune(days);
  message.value = t("settings.audit.pruned", { count: removed });
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

function onEscape(event: KeyboardEvent): void {
  if (event.key === "Escape") {
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
  <main class="page">
    <header class="head">
      <div>
        <h1>{{ t("settings.audit.title") }}</h1>
        <p class="hint">{{ t("settings.audit.hint") }}</p>
      </div>
      <button type="button" @click="goBack">{{ t("settings.back") }}</button>
    </header>

    <section v-if="store.settings" class="bar">
      <label>
        {{ t("settings.audit.retention") }}
        <input v-model.number="store.settings.audit.retention_days" min="1" type="number" />
      </label>
      <button type="button" @click="saveRetention">{{ t("settings.save") }}</button>
      <button type="button" @click="load">{{ t("settings.audit.refresh") }}</button>
      <button type="button" @click="verify">{{ t("settings.audit.verify") }}</button>
      <button type="button" @click="prune">{{ t("settings.audit.prune") }}</button>
      <button type="button" @click="clear">{{ t("settings.audit.clear") }}</button>
    </section>

    <section class="bar">
      <input v-model="exportPath" :placeholder="t('settings.audit.exportPath')" />
      <button type="button" @click="exportLog">{{ t("settings.audit.export") }}</button>
      <span v-if="message" class="hint">{{ message }}</span>
    </section>

    <div class="table-wrap">
      <table class="audit-table">
        <thead>
          <tr>
            <th>{{ t("settings.audit.time") }}</th>
            <th>{{ t("settings.audit.op") }}</th>
            <th>{{ t("settings.audit.path") }}</th>
            <th>{{ t("settings.audit.result") }}</th>
            <th>{{ t("settings.audit.actor") }}</th>
            <th>{{ t("settings.audit.detail") }}</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="row in rows" :key="row.id">
            <td class="mono">{{ toRfc3339(row.ts) }}</td>
            <td>{{ row.op }}</td>
            <td class="mono">{{ row.path }}</td>
            <td>{{ row.result }}</td>
            <td>{{ row.actor }}</td>
            <td>{{ row.detail }}</td>
          </tr>
        </tbody>
      </table>
      <p v-if="!rows.length" class="hint">{{ t("settings.audit.empty") }}</p>
    </div>
  </main>
</template>

<style scoped>
.page {
  height: 100%;
  overflow-y: auto;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
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
.bar {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
}
.bar input {
  min-width: 6rem;
}
.table-wrap {
  flex: 1;
  overflow: auto;
}
.audit-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}
.audit-table th,
.audit-table td {
  text-align: left;
  padding: 6px 8px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
  white-space: nowrap;
}
.mono {
  font-family: ui-monospace, monospace;
}
</style>
