<script lang="ts" setup>
import {
  NButton,
  NCard,
  NCheckbox,
  NGrid,
  NGridItem,
  NInput,
  NInputNumber,
  NStatistic,
} from "naive-ui";
import {onMounted, onUnmounted, ref} from "vue";
import {useI18n} from "vue-i18n";
import {useRouter} from "vue-router";

import {type InvokeRejection, toAppError} from "@/api/client";
import {metricsSnapshot} from "@/api/metrics";
import {otelExportNow, otelStatus} from "@/api/otel";
import type {MetricsSnapshot} from "@/api/types/MetricsSnapshot";
import type {OtelExportStatus} from "@/api/types/OtelExportStatus";
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

function setSampleRatio(value: number): void {
  if (store.settings) {
    store.settings.observability.otel.sample_ratio = value;
  }
}

function onEscape(event: KeyboardEvent): void {
  if (event.key === KEY_ESCAPE) {
    goBack();
  }
}

onMounted(async () => {
  await store.load();
  await refresh();
  window.addEventListener("keydown", onEscape);
});

onUnmounted(() => window.removeEventListener("keydown", onEscape));
</script>

<template>
  <main class="page observability-page">
    <header class="head">
      <div>
        <h1>{{ t("settings.observability.title") }}</h1>
        <p class="hint">{{ t("settings.observability.hint") }}</p>
      </div>
      <NButton size="small" @click="goBack">{{ t("settings.back") }}</NButton>
    </header>

    <NCard v-if="store.settings" size="small">
      <div class="settings">
        <NCheckbox v-model:checked="store.settings.observability.otel.enabled">
          {{ t("settings.observability.otlp") }}
        </NCheckbox>
        <div class="field">
          <span class="field-label">{{ t("settings.observability.endpoint") }}</span>
          <NInput
            v-model:value="store.settings.observability.otel.endpoint"
            class="endpoint"
            size="small"
          />
        </div>
        <div class="field">
          <span class="field-label">{{ t("settings.observability.sampleRatio") }}</span>
          <NInputNumber
            :max="1"
            :min="0"
            :show-button="false"
            :step="0.1"
            :value="store.settings.observability.otel.sample_ratio"
            class="ratio"
            size="small"
            @update:value="(value) => applyNumber(value, setSampleRatio)"
          />
        </div>
        <div class="actions">
          <NButton size="small" type="primary" @click="save">{{ t("settings.save") }}</NButton>
          <NButton size="small" @click="exportNow">
            {{ t("settings.observability.otelExportNow") }}
          </NButton>
          <NButton size="small" @click="refresh">{{ t("settings.observability.refresh") }}</NButton>
        </div>
        <p v-if="message" class="hint">{{ message }}</p>
        <p v-if="otel" class="hint">
          {{ otel.enabled ? t("settings.observability.otelEnabled") : t("settings.observability.otelDisabled") }}
          · {{ otel.endpoint }}
          <span v-if="otel.last_export_ts">
            · {{ t("settings.observability.otelLast") }}: {{ toRfc3339(otel.last_export_ts) }}
          </span>
        </p>
      </div>
    </NCard>

    <NGrid v-if="metrics" :cols="4" :x-gap="8" :y-gap="8" responsive="screen">
      <NGridItem v-for="(value, key) in metrics" :key="key">
        <NCard size="small">
          <NStatistic :label="key" :value="value" />
        </NCard>
      </NGridItem>
    </NGrid>
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

.settings {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.field {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.field-label {
  color: #cbd5e1;
  font-size: 13px;
}

.endpoint {
  width: 360px;
  max-width: 100%;
}

.ratio {
  width: 120px;
}

.actions {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
}
</style>
