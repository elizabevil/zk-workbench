<script lang="ts" setup>
import {NAlert, NButton, NForm, NFormItem, NInput, NModal} from "naive-ui";
import {computed, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {type InvokeRejection, toAppError} from "@/api/client";
import {nodeSet} from "@/api/node";
import type {AppError} from "@/api/types/AppError";
import type {NodeStat} from "@/api/types/NodeStat";
import {decodeBase64Bytes, encodeBytesBase64} from "@/utils/encoding";

const props = defineProps<{
  open: boolean;
  connectionId: string;
  path: string;
  version: number;
  originalB64: string;
  byteLen: number;
}>();

const emit = defineEmits<{
  close: [];
  saved: [stat: NodeStat];
}>();

const {t} = useI18n();
const hex = ref("");
const error = ref<AppError | null>(null);
const localError = ref("");
const saving = ref(false);
const fileName = ref("");

watch(
  () => props.open,
  (open) => {
    if (!open) {
      return;
    }
    hex.value = "";
    error.value = null;
    localError.value = "";
    fileName.value = "";
  },
);

const newBytes = computed<Uint8Array | null>(() => {
  const cleaned = hex.value.replace(/[^0-9a-fA-F]/g, "");
  if (cleaned.length === 0) {
    return new Uint8Array();
  }
  if (cleaned.length % 2 !== 0) {
    return null;
  }
  const bytes = new Uint8Array(cleaned.length / 2);
  for (let index = 0; index < bytes.length; index += 1) {
    bytes[index] = Number.parseInt(cleaned.slice(index * 2, index * 2 + 2), 16);
  }
  return bytes;
});

const originalBytes = computed(() => decodeBase64Bytes(props.originalB64));
const delta = computed(() => (newBytes.value ? newBytes.value.length - props.byteLen : 0));
const changedBytes = computed(() => {
  if (!newBytes.value) {
    return 0;
  }
  const before = originalBytes.value;
  const after = newBytes.value;
  let changed = Math.abs(before.length - after.length);
  const common = Math.min(before.length, after.length);
  for (let index = 0; index < common; index += 1) {
    if (before[index] !== after[index]) {
      changed += 1;
    }
  }
  return changed;
});

async function pickFile(event: Event): Promise<void> {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file) {
    return;
  }
  const bytes = new Uint8Array(await file.arrayBuffer());
  fileName.value = file.name;
  hex.value = Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join(" ");
}

async function save(): Promise<void> {
  if (!newBytes.value) {
    localError.value = t("binary.invalidHex");
    return;
  }
  saving.value = true;
  error.value = null;
  try {
    const stat = await nodeSet(
      props.connectionId,
      props.path,
      encodeBytesBase64(newBytes.value),
      props.version,
    );
    emit("saved", stat);
  } catch (raw) {
    error.value = toAppError(raw as InvokeRejection);
  } finally {
    saving.value = false;
  }
}

function onShow(value: boolean): void {
  if (!value) {
    emit("close");
  }
}
</script>

<template>
  <NModal
    :close-on-esc="false"
    :show="open"
    :style="{width: '720px'}"
    :title="t('binary.title')"
    preset="card"
    @update:show="onShow"
  >
    <p class="path">{{ path }}</p>

    <NForm :show-feedback="false" label-placement="top" size="small">
      <NFormItem :label="t('binary.file')">
        <div class="file-row">
          <input accept="*/*" class="file-input" type="file" @change="pickFile" />
          <span v-if="fileName" class="muted">{{ fileName }}</span>
        </div>
      </NFormItem>
      <NFormItem :label="t('binary.hex')">
        <NInput
          v-model:value="hex"
          :autosize="{minRows: 8, maxRows: 16}"
          class="hex-input"
          type="textarea"
        />
      </NFormItem>
    </NForm>

    <p class="muted">
      {{ t("binary.bytes") }}: {{ byteLen }} → {{ newBytes?.length ?? "-" }}
      · {{ t("binary.delta") }}: {{ delta >= 0 ? `+${delta}` : delta }}
      · {{ t("binary.changed") }}: {{ changedBytes }}
    </p>

    <NAlert v-if="localError" :bordered="false" type="error">
      {{ localError }}
    </NAlert>
    <NAlert v-if="error" :bordered="false" type="error">
      {{ t(error.message_key) }}
    </NAlert>

    <template #footer>
      <div class="actions">
        <NButton @click="emit('close')">{{ t("binary.cancel") }}</NButton>
        <NButton :loading="saving" type="error" @click="save">
          {{ saving ? t("binary.saving") : t("binary.save") }}
        </NButton>
      </div>
    </template>
  </NModal>
</template>

<style scoped>
.path {
  margin: 0 0 10px;
  font-family: ui-monospace, monospace;
  word-break: break-all;
}

.file-row {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
}

.file-input {
  max-width: 100%;
  color: var(--app-text-muted, #9aa4b2);
  font: inherit;
}

.file-input::file-selector-button {
  margin-right: 8px;
  padding: 4px 8px;
  border: 1px solid #2a323c;
  border-radius: 4px;
  background: #1d242c;
  color: var(--app-text, #e6e6e6);
  cursor: pointer;
}

.hex-input :deep(.n-input__textarea-el) {
  font-family: ui-monospace, monospace;
  font-size: 12px;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.muted {
  color: var(--app-text-muted, #9aa4b2);
  font-size: 12px;
}
</style>
