<script lang="ts" setup>
import {NAlert, NButton, NCheckbox, NModal, NScrollbar, NSpin} from "naive-ui";
import {computed, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {type InvokeRejection, toAppError} from "@/api/client";
import {nodeDelete, nodeDeletePreview, nodeDeletePreviewCancel} from "@/api/node";
import type {AppError} from "@/api/types/AppError";
import type {DeleteOutcome} from "@/api/types/DeleteOutcome";
import type {DeletePreview} from "@/api/types/DeletePreview";

const props = defineProps<{
  open: boolean;
  connectionId: string;
  path: string;
  hasChildren: boolean;
  initialRecursive?: boolean;
}>();

const emit = defineEmits<{
  close: [];
  deleted: [outcome: DeleteOutcome];
}>();

const {t} = useI18n();
const recursive = ref(false);
const preview = ref<DeletePreview>();
const loading = ref(false);
const error = ref<AppError | null>(null);
const requestId = ref("");
const cancelled = ref(false);

watch(
  () => [props.open, recursive.value] as const,
  async ([open]) => {
    if (!open) {
      return;
    }
    preview.value = undefined;
    error.value = null;
    cancelled.value = false;
    loading.value = true;
    requestId.value = crypto.randomUUID();
    try {
      preview.value = await nodeDeletePreview(
        props.connectionId,
        props.path,
        recursive.value,
        requestId.value,
      );
    } catch (raw) {
      const failure = toAppError(raw as InvokeRejection);
      if (failure.kind === "cancelled") {
        cancelled.value = true;
      } else {
        error.value = failure;
      }
    } finally {
      loading.value = false;
    }
  },
  {immediate: true},
);

watch(
  () => props.open,
  (open) => {
    if (open) {
      recursive.value = props.initialRecursive ?? false;
    }
  },
);

const canConfirm = computed(() => !props.hasChildren || recursive.value);

async function cancelPreview(): Promise<void> {
  if (requestId.value) {
    await nodeDeletePreviewCancel(requestId.value);
  }
}

const totalLabel = computed(() => {
  if (!preview.value) {
    return "-";
  }
  return preview.value.truncated ? "≥5000" : String(preview.value.total);
});

async function confirm(): Promise<void> {
  error.value = null;
  try {
    const outcome = await nodeDelete(props.connectionId, props.path, recursive.value);
    emit("deleted", outcome);
  } catch (raw) {
    error.value = toAppError(raw as InvokeRejection);
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
    :style="{width: '600px'}"
    :title="t('delete.title')"
    preset="card"
    @update:show="onShow"
  >
    <p class="path">{{ path }}</p>

    <NCheckbox v-if="hasChildren" v-model:checked="recursive">
      {{ t("delete.recursive") }}
    </NCheckbox>

    <div class="preview">
      <p class="muted">
        {{ t("delete.impact") }}: <strong>{{ totalLabel }}</strong>
      </p>
      <div v-if="loading" class="preview-status">
        <NSpin size="small" />
        <span class="muted">{{ t("delete.loading") }}</span>
        <NButton size="tiny" @click="cancelPreview">{{ t("delete.cancelPreview") }}</NButton>
      </div>
      <p v-else-if="cancelled" class="muted">{{ t("delete.cancelled") }}</p>
      <NScrollbar v-else-if="preview" style="max-height: 280px">
        <ul class="paths">
          <li v-for="item in preview.paths" :key="item">{{ item }}</li>
        </ul>
      </NScrollbar>
    </div>

    <NAlert v-if="error" :bordered="false" type="error">
      {{ t(error.message_key) }}
    </NAlert>

    <template #footer>
      <div class="actions">
        <NButton @click="emit('close')">{{ t("delete.cancel") }}</NButton>
        <NButton :disabled="!canConfirm" type="error" @click="confirm">
          {{ t("delete.confirm") }}
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

.preview {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-top: 10px;
  padding: 10px;
  border: 1px solid #2a323c;
  border-radius: 6px;
  background: #101418;
}

.preview-status {
  display: flex;
  align-items: center;
  gap: 8px;
}

.paths {
  margin: 0;
  padding-left: 18px;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.muted {
  color: #9aa4b2;
  margin: 0;
  font-size: 13px;
}
</style>
