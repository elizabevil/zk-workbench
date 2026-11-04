<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { type InvokeRejection, toAppError } from "@/api/client";
import { metricsSnapshot } from "@/api/metrics";
import { otelExportNow, otelStatus } from "@/api/otel";
import type { MetricsSnapshot } from "@/api/types/MetricsSnapshot";
import type { OtelExportStatus } from "@/api/types/OtelExportStatus";
import { useSettingsStore } from "@/stores/settings";
import { toRfc3339 } from "@/utils/time";

const { t } = useI18n();
const router = useRouter();
const store = useSettingsStore();
const metrics = ref<MetricsSnapshot | null>(null);
const otel = ref<OtelExportStatus | null>(null);
const message = ref("");

async function refresh(): Promise<void> {
  metrics.value = await metricsSnapshot();
  otel.value = await otelStatus();
}

async function exportNow(): Promise<void> {
  try {
    await otelExportNow();
    message.value = t("settings.observability.otelExported");
    await refresh();
  } catch (raw) {
    message.value = t(toAppError(raw as InvokeRejection).message_key);
  }
}

async function save(): Promise<void> {
  await store.save();
  message.value = t("settings.saved");
}

onMounted(async () => {
  await store.load();
  await refresh();
});
</script>

<template>
  <main class="page">
    <header class="head">
      <div>
        <h1>{{ t("settings.observability.title") }}</h1>
        <p class="hint">{{ t("settings.observability.hint") }}</p>
      </div>
      <button type="button" @click="router.push('/')">{{ t("settings.back") }}</button>
    </header>

    <section v-if="store.settings" class="card">
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
        <input v-model.number="store.settings.observability.otel.sample_ratio" max="1" min="0" step="0.1" type="number" />
      </label>
      <div class="actions">
        <button type="button" @click="save">{{ t("settings.save") }}</button>
        <button type="button" @click="exportNow">{{ t("settings.observability.otelExportNow") }}</button>
        <button type="button" @click="refresh">{{ t("settings.observability.refresh") }}</button>
      </div>
      <p v-if="message" class="hint">{{ message }}</p>
      <p v-if="otel" class="hint">
        {{ otel.enabled ? t("settings.observability.otelEnabled") : t("settings.observability.otelDisabled") }}
        · {{ otel.endpoint }}
        <span v-if="otel.last_export_ts">
          · {{ t("settings.observability.otelLast") }}: {{ toRfc3339(otel.last_export_ts) }}
        </span>
      </p>
    </section>

    <section v-if="metrics" class="metrics">
      <div v-for="(value, key) in metrics" :key="key" class="metric">
        <span class="label">{{ key }}</span>
        <span class="value">{{ value }}</span>
      </div>
    </section>
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
.card {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 16px;
  border: 1px solid rgb(255 255 255 / 8%);
  border-radius: 8px;
}
.inline {
  flex-direction: row;
  align-items: center;
  gap: 8px;
}
.actions {
  display: flex;
  gap: 8px;
}
.metrics {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(9rem, 1fr));
  gap: 8px;
}
.metric {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 10px;
  border: 1px solid rgb(255 255 255 / 8%);
  border-radius: 6px;
}
.label {
  color: #9aa4b2;
  font-size: 11px;
}
.value {
  font-family: ui-monospace, monospace;
  font-size: 18px;
}
</style>
