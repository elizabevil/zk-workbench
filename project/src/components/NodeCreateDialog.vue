<script lang="ts" setup>
import {NAlert, NButton, NCheckbox, NForm, NFormItem, NInput, NModal, NRadio, NRadioGroup} from "naive-ui";
import {ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {type InvokeRejection, toAppError} from "@/api/client";
import {nodeCreate} from "@/api/node";
import type {AppError} from "@/api/types/AppError";
import type {NodeCreateMode} from "@/api/types/NodeCreateMode";
import {encodeUtf8Base64} from "@/utils/encoding";
import {normalizePath} from "@/utils/path";

const props = defineProps<{
  open: boolean;
  connectionId: string;
  defaultPath: string;
}>();

const emit = defineEmits<{
  close: [];
  created: [actualPath: string];
}>();

const {t} = useI18n();
const path = ref("/");
const NODE_CREATE_MODES = [
  "persistent",
  "ephemeral",
  "persistent_sequential",
  "ephemeral_sequential",
] as const satisfies readonly NodeCreateMode[];
const DEFAULT_CREATE_MODE = NODE_CREATE_MODES[0];
const mode = ref<NodeCreateMode>(DEFAULT_CREATE_MODE);
const data = ref("");
const createParents = ref(false);
const error = ref<AppError | null>(null);
const saving = ref(false);

watch(
  () => props.open,
  (open) => {
    if (!open) {
      return;
    }
    path.value = props.defaultPath;
    mode.value = DEFAULT_CREATE_MODE;
    data.value = "";
    createParents.value = false;
    error.value = null;
  },
);

function isNodeCreateMode(value: unknown): value is NodeCreateMode {
  return typeof value === "string" && (NODE_CREATE_MODES as readonly string[]).includes(value);
}

function setMode(value: unknown): void {
  if (isNodeCreateMode(value)) {
    mode.value = value;
  }
}

async function submit(event?: Event): Promise<void> {
  event?.preventDefault();
  saving.value = true;
  error.value = null;
  try {
    const actual = await nodeCreate(
      props.connectionId,
      normalizePath(path.value),
      encodeUtf8Base64(data.value),
      mode.value,
      createParents.value,
    );
    emit("created", actual);
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
    :style="{width: '560px'}"
    :title="t('create.title')"
    preset="card"
    @update:show="onShow"
  >
    <NForm :on-submit="submit" :show-feedback="false" label-placement="top" size="small">
      <NFormItem :label="t('create.path')">
        <NInput v-model:value="path" />
      </NFormItem>
      <NFormItem :label="t('create.modeLabel')">
        <NRadioGroup :value="mode" @update:value="setMode">
          <div class="mode-group">
            <NRadio v-for="item in NODE_CREATE_MODES" :key="item" :value="item">
              {{ t(`create.mode.${item}`) }}
            </NRadio>
          </div>
        </NRadioGroup>
      </NFormItem>
      <NFormItem :label="t('create.data')">
        <NInput
          v-model:value="data"
          :autosize="{minRows: 4, maxRows: 8}"
          class="data-input"
          type="textarea"
        />
      </NFormItem>
      <NFormItem>
        <NCheckbox v-model:checked="createParents">{{ t("create.parents") }}</NCheckbox>
      </NFormItem>
    </NForm>

    <NAlert
      v-if="mode === 'ephemeral' || mode === 'ephemeral_sequential'"
      :bordered="false"
      type="warning"
    >
      {{ t("create.ephemeralHint") }}
    </NAlert>
    <NAlert v-if="error" :bordered="false" type="error">
      {{ t(error.message_key) }}
    </NAlert>

    <template #footer>
      <div class="actions">
        <NButton @click="emit('close')">{{ t("create.cancel") }}</NButton>
        <NButton :loading="saving" type="primary" @click="submit()">
          {{ saving ? t("create.saving") : t("create.submit") }}
        </NButton>
      </div>
    </template>
  </NModal>
</template>

<style scoped>
.mode-group {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.data-input :deep(.n-input__textarea-el) {
  font-family: ui-monospace, monospace;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
