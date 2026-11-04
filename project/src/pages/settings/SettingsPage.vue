<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { appPaths } from "@/api/app";
import { auditClear, auditExport, auditList, auditPrune, auditVerify } from "@/api/audit";
import { type InvokeRejection, toAppError } from "@/api/client";
import { metricsSnapshot } from "@/api/metrics";
import { otelExportNow, otelStatus } from "@/api/otel";
import { proxyPasswordClear, proxyPasswordSet, proxyTest } from "@/api/proxy";
import type { AppPaths } from "@/api/types/AppPaths";
import type { AuditRecord } from "@/api/types/AuditRecord";
import type { MetricsSnapshot } from "@/api/types/MetricsSnapshot";
import type { OtelExportStatus } from "@/api/types/OtelExportStatus";
import type { ProxyProfile } from "@/api/types/ProxyProfile";
import { LOCALES } from "@/i18n/locale";
import { NOTIFICATION_LEVELS, PROXY_KINDS } from "@/settings/constants";
import { useSettingsStore } from "@/stores/settings";

const { t } = useI18n();
const router = useRouter();
const store = useSettingsStore();

const noProxyText = computed({
  get: () => store.settings?.proxy.no_proxy.join("\n") ?? "",
  set: (value: string) => {
    if (!store.settings) {
      return;
    }
    store.settings.proxy.no_proxy = value
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
  },
});

const paths = ref<AppPaths | null>(null);
const metrics = ref<MetricsSnapshot | null>(null);
const auditRows = ref<AuditRecord[]>([]);
const auditMessage = ref("");
const exportPath = ref("");
const otel = ref<OtelExportStatus | null>(null);
const otelMessage = ref("");
const passwords = ref<Record<number, string>>({});
const testTarget = ref("127.0.0.1:2181");
const testMessage = ref("");

async function loadMetrics(): Promise<void> {
  metrics.value = await metricsSnapshot();
}

async function loadOtel(): Promise<void> {
  otel.value = await otelStatus();
}

async function exportOtel(): Promise<void> {
  try {
    await otelExportNow();
    otelMessage.value = t("settings.observability.otelExported");
  } catch (raw) {
    const error = raw as { message_key?: string };
    otelMessage.value = error.message_key ? t(error.message_key) : String(raw);
  }
  await loadOtel();
}

async function loadAudit(): Promise<void> {
  auditRows.value = await auditList(100, 0, null);
}

async function verifyAudit(): Promise<void> {
  const report = await auditVerify();
  auditMessage.value = report.ok
    ? t("settings.audit.verified", { count: report.checked })
    : t("settings.audit.broken", { id: report.broken_at ?? "-" });
}

async function clearAudit(): Promise<void> {
  if (!window.confirm(t("settings.audit.confirmClear"))) {
    return;
  }
  const cleared = await auditClear();
  auditMessage.value = t("settings.audit.cleared", { count: cleared });
  await loadAudit();
}

async function pruneAudit(): Promise<void> {
  const days = store.settings?.audit.retention_days ?? 90;
  const removed = await auditPrune(days);
  auditMessage.value = t("settings.audit.pruned", { count: removed });
  await loadAudit();
}

async function exportAudit(): Promise<void> {
  if (!exportPath.value.trim()) {
    return;
  }
  await auditExport(exportPath.value.trim());
  auditMessage.value = t("settings.audit.exported");
}

onMounted(async () => {
  await store.load();
  paths.value = await appPaths();
  await loadMetrics();
  await loadAudit();
  await loadOtel();
});

function moveProxy(index: number, delta: number): void {
  const profiles = store.settings?.proxy.profiles;
  if (!profiles) {
    return;
  }
  const target = index + delta;
  if (target < 0 || target >= profiles.length) {
    return;
  }
  const [item] = profiles.splice(index, 1);
  profiles.splice(target, 0, item);
}

async function setPassword(index: number, profile: ProxyProfile): Promise<void> {
  const password = passwords.value[index];
  if (!password) {
    return;
  }
  await proxyPasswordSet(profile.name, password);
  passwords.value[index] = "";
}

async function clearPassword(profile: ProxyProfile): Promise<void> {
  await proxyPasswordClear(profile.name);
}

async function resetSettings(): Promise<void> {
  if (!window.confirm(t("settings.resetConfirm"))) {
    return;
  }
  await store.reset();
}

async function testProxy(profile: ProxyProfile): Promise<void> {
  try {
    await proxyTest(profile, testTarget.value);
    testMessage.value = t("settings.proxy.testOk");
  } catch (raw) {
    const error = toAppError(raw as InvokeRejection);
    testMessage.value = t(error.message_key);
  }
}

function addProxy(): void {
  const profile: ProxyProfile = {
    name: `proxy-${store.settings?.proxy.profiles.length ?? 0 + 1}`,
    kind: "socks5",
    host: "",
    port: 1080,
    username: null,
  };
  store.settings?.proxy.profiles.push(profile);
}

function removeProxy(index: number): void {
  store.settings?.proxy.profiles.splice(index, 1);
}
</script>

<template>
  <section class="settings">
    <header class="header">
      <h1>{{ t("settings.title") }}</h1>
      <button type="button" @click="router.push('/')">{{ t("settings.back") }}</button>
    </header>

    <p v-if="store.error" class="error">{{ t(store.error.message_key) }}</p>
    <p v-if="!store.settings">{{ t("settings.loading") }}</p>

    <form v-if="store.settings" class="sections" @submit.prevent="store.save()">
      <fieldset>
        <legend>{{ t("settings.general.title") }}</legend>
        <label>
          {{ t("settings.general.locale") }}
          <select v-model="store.settings.general.locale">
            <option v-for="locale in LOCALES" :key="locale" :value="locale">
              {{ locale }}
            </option>
          </select>
        </label>
        <label>
          {{ t("settings.general.theme") }}
          <input v-model="store.settings.general.theme" />
        </label>
        <label class="inline">
          <input v-model="store.settings.general.restore_last_connection" type="checkbox" />
          {{ t("settings.general.restore") }}
        </label>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.appearance.title") }}</legend>
        <label>
          {{ t("settings.appearance.background") }}
          <input v-model="store.settings.appearance.background" />
        </label>
        <div class="preview" :style="{ background: store.settings.appearance.background }" />
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.notifications.title") }}</legend>
        <label>
          {{ t("settings.notifications.desktop") }}
          <select v-model="store.settings.notifications.desktop">
            <option v-for="level in NOTIFICATION_LEVELS" :key="level" :value="level">
              {{ level }}
            </option>
          </select>
        </label>
        <label class="inline">
          <input v-model="store.settings.notifications.command_finished" type="checkbox" />
          {{ t("settings.notifications.commandFinished") }}
        </label>
        <label class="inline">
          <input v-model="store.settings.notifications.sound" type="checkbox" />
          {{ t("settings.notifications.sound") }}
        </label>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.retry.title") }}</legend>
        <label class="inline">
          <input v-model="store.settings.retry.auto_reconnect" type="checkbox" />
          {{ t("settings.retry.autoReconnect") }}
        </label>
        <label>
          {{ t("settings.retry.maxAttempts") }}
          <input v-model.number="store.settings.retry.max_attempts" min="1" type="number" />
        </label>
        <label>
          {{ t("settings.retry.initial") }}
          <input v-model.number="store.settings.retry.initial_interval_ms" min="100" type="number" />
        </label>
        <label>
          {{ t("settings.retry.max") }}
          <input v-model.number="store.settings.retry.max_interval_ms" min="100" type="number" />
        </label>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.task.title") }}</legend>
        <label class="inline">
          <input v-model="store.settings.task.continue_on_error" type="checkbox" />
          {{ t("settings.task.continueOnError") }}
        </label>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.proxy.title") }}</legend>
        <label class="inline">
          <input v-model="store.settings.proxy.enabled" type="checkbox" />
          {{ t("settings.proxy.enabled") }}
        </label>
        <label>
          {{ t("settings.proxy.noProxy") }}
          <textarea v-model="noProxyText" rows="3" />
        </label>
        <div v-for="(profile, index) in store.settings.proxy.profiles" :key="index" class="proxy-row">
          <input v-model="profile.name" :placeholder="t('settings.proxy.name')" />
          <select v-model="profile.kind">
            <option v-for="kind in PROXY_KINDS" :key="kind" :value="kind">
              {{ kind }}
            </option>
          </select>
          <input v-model="profile.host" :placeholder="t('settings.proxy.host')" />
          <input v-model.number="profile.port" :placeholder="t('settings.proxy.port')" type="number" />
          <input v-model="profile.username" :placeholder="t('settings.proxy.username')" />
          <input
            v-model="passwords[index]"
            :placeholder="t('settings.proxy.password')"
            type="password"
          />
          <div class="row-actions">
            <button type="button" @click="moveProxy(index, -1)">↑</button>
            <button type="button" @click="moveProxy(index, 1)">↓</button>
            <button type="button" @click="setPassword(index, profile)">
              {{ t("settings.proxy.setPassword") }}
            </button>
            <button type="button" @click="clearPassword(profile)">
              {{ t("settings.proxy.clearPassword") }}
            </button>
            <button type="button" @click="testProxy(profile)">{{ t("settings.proxy.test") }}</button>
            <button type="button" @click="removeProxy(index)">{{ t("settings.proxy.remove") }}</button>
          </div>
        </div>
        <button type="button" @click="addProxy">{{ t("settings.proxy.add") }}</button>
        <label>
          {{ t("settings.proxy.testTarget") }}
          <input v-model="testTarget" />
        </label>
        <p v-if="testMessage" class="hint">{{ testMessage }}</p>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.audit.title") }}</legend>
        <label>
          {{ t("settings.audit.retention") }}
          <input v-model.number="store.settings.audit.retention_days" min="1" type="number" />
        </label>
        <div class="audit-actions">
          <button type="button" @click="loadAudit">{{ t("settings.audit.refresh") }}</button>
          <button type="button" @click="verifyAudit">{{ t("settings.audit.verify") }}</button>
          <button type="button" @click="pruneAudit">{{ t("settings.audit.prune") }}</button>
          <button type="button" class="danger" @click="clearAudit">
            {{ t("settings.audit.clear") }}
          </button>
        </div>
        <p v-if="auditMessage" class="hint">{{ auditMessage }}</p>

        <div class="audit-export">
          <input v-model="exportPath" :placeholder="t('settings.audit.exportPath')" />
          <button type="button" @click="exportAudit">{{ t("settings.audit.export") }}</button>
        </div>

        <table v-if="auditRows.length" class="audit-table">
          <thead>
            <tr>
              <th>{{ t("settings.audit.time") }}</th>
              <th>{{ t("settings.audit.op") }}</th>
              <th>{{ t("settings.audit.path") }}</th>
              <th>{{ t("settings.audit.result") }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in auditRows" :key="row.id">
              <td>{{ new Date(row.ts).toLocaleString() }}</td>
              <td>{{ row.op }}</td>
              <td class="mono">{{ row.path ?? "-" }}</td>
              <td>
                {{ row.result }}
                <span v-if="row.error_kind" class="muted">({{ row.error_kind }})</span>
              </td>
            </tr>
          </tbody>
        </table>
        <p v-else class="hint">{{ t("settings.audit.empty") }}</p>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.observability.title") }}</legend>
        <label class="inline">
          <input v-model="store.settings.observability.otel.enabled" type="checkbox" />
          {{ t("settings.observability.otlp") }}
        </label>
        <label>
          {{ t("settings.observability.endpoint") }}
          <input v-model="store.settings.observability.otel.endpoint" />
        </label>
        <label>
          {{ t("settings.observability.sampleRatio") }}
          <input
            v-model.number="store.settings.observability.otel.sample_ratio"
            max="1"
            min="0"
            step="0.1"
            type="number"
          />
        </label>
        <p class="hint">{{ t("settings.observability.hint") }}</p>

        <div class="audit-actions">
          <button type="button" @click="exportOtel">
            {{ t("settings.observability.otelExportNow") }}
          </button>
          <button type="button" @click="loadOtel">
            {{ t("settings.observability.refresh") }}
          </button>
          <span v-if="otel" class="hint">
            {{ otel.enabled ? t("settings.observability.otelEnabled") : t("settings.observability.otelDisabled") }}
            <template v-if="otel.last_export_ts">
              · {{ t("settings.observability.otelLast") }}:
              {{ new Date(otel.last_export_ts).toLocaleString() }}
            </template>
            <template v-if="otel.last_error">
              · {{ t("settings.observability.otelError") }}: {{ otel.last_error }}
            </template>
          </span>
        </div>
        <p v-if="otelMessage" class="hint">{{ otelMessage }}</p>

        <div class="metrics-head">
          <span>{{ t("settings.observability.metrics") }}</span>
          <button type="button" @click="loadMetrics">
            {{ t("settings.observability.refresh") }}
          </button>
        </div>
        <dl v-if="metrics" class="metrics">
          <div><dt>{{ t("settings.observability.connects") }}</dt><dd>{{ metrics.connect_attempts }}</dd></div>
          <div><dt>{{ t("settings.observability.failures") }}</dt><dd>{{ metrics.connect_failures }}</dd></div>
          <div><dt>{{ t("settings.observability.reconnects") }}</dt><dd>{{ metrics.reconnects }}</dd></div>
          <div><dt>{{ t("settings.observability.reads") }}</dt><dd>{{ metrics.node_reads }}</dd></div>
          <div><dt>{{ t("settings.observability.writes") }}</dt><dd>{{ metrics.node_writes }}</dd></div>
          <div><dt>{{ t("settings.observability.creates") }}</dt><dd>{{ metrics.node_creates }}</dd></div>
          <div><dt>{{ t("settings.observability.deletes") }}</dt><dd>{{ metrics.node_deletes }}</dd></div>
          <div><dt>{{ t("settings.observability.readP50") }}</dt><dd>{{ metrics.read_p50_ms }} ms</dd></div>
          <div><dt>{{ t("settings.observability.readP95") }}</dt><dd>{{ metrics.read_p95_ms }} ms</dd></div>
          <div><dt>{{ t("settings.observability.writeP50") }}</dt><dd>{{ metrics.write_p50_ms }} ms</dd></div>
          <div><dt>{{ t("settings.observability.writeP95") }}</dt><dd>{{ metrics.write_p95_ms }} ms</dd></div>
        </dl>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.logs.title") }}</legend>
        <p class="hint">{{ t("settings.logs.hint") }}</p>
        <p v-if="paths" class="hint">{{ t("settings.logs.config") }}: {{ paths.config_path }}</p>
        <p v-if="paths" class="hint">{{ t("settings.logs.dir") }}: {{ paths.log_dir }}</p>
      </fieldset>

      <div class="actions">
        <button :disabled="store.saving" type="submit">{{ t("settings.save") }}</button>
        <button type="button" @click="resetSettings">{{ t("settings.resetConfig") }}</button>
      </div>
    </form>
  </section>
</template>


<style scoped>
.settings {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.sections {
  display: grid;
  gap: 16px;
  max-width: 900px;
}
fieldset {
  border: 1px solid #2a323c;
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px 16px;
}
legend {
  padding: 0 6px;
}
label {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
label.inline {
  flex-direction: row;
  align-items: center;
  gap: 8px;
}
.proxy-row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 90px 1fr 1fr;
  align-items: start;
  gap: 6px;
}
.row-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}
.actions {
  display: flex;
  gap: 12px;
}
.audit-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.audit-export {
  display: flex;
  gap: 8px;
}
.audit-export input {
  flex: 1;
  padding: 7px 9px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
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
}
.audit-table .mono {
  font-family: ui-monospace, monospace;
}
.danger {
  border-color: rgb(248 113 113 / 45%);
  color: #f87171;
}
.metrics-head {
  display: flex;
  align-items: center;
  gap: 10px;
}
.metrics {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 8px 16px;
  margin: 0;
}
.metrics div {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  padding: 4px 0;
  border-bottom: 1px solid rgb(255 255 255 / 6%);
}
.metrics dt {
  color: #9aa4b2;
  font-size: 12px;
}
.metrics dd {
  margin: 0;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}
.preview {
  height: 48px;
  border: 1px solid #2a323c;
  border-radius: 6px;
}
.hint {
  color: #9aa4b2;
  margin: 0;
}
.error {
  color: #ef4444;
}
</style>
