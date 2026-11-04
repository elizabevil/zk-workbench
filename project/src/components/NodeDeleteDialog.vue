<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import { type InvokeRejection, toAppError } from "@/api/client";
import { nodeDelete, nodeDeletePreview, nodeDeletePreviewCancel } from "@/api/node";
import type { AppError } from "@/api/types/AppError";
import type { DeleteOutcome } from "@/api/types/DeleteOutcome";
import type { DeletePreview } from "@/api/types/DeletePreview";

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

const { t } = useI18n();
const recursive = ref(false);
const preview = ref<DeletePreview | null>(null);
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
    preview.value = null;
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
  { immediate: true },
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
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="overlay">
      <div class="dialog">
        <h2>{{ t("delete.title") }}</h2>
        <p class="path">{{ path }}</p>

        <label v-if="hasChildren" class="inline">
          <input v-model="recursive" type="checkbox" />
          {{ t("delete.recursive") }}
        </label>

        <div class="preview">
          <p class="muted">
            {{ t("delete.impact") }}: <strong>{{ totalLabel }}</strong>
          </p>
          <p v-if="loading" class="muted">
            {{ t("delete.loading") }}
            <button type="button" class="tiny" @click="cancelPreview">
              {{ t("delete.cancelPreview") }}
            </button>
          </p>
          <p v-else-if="cancelled" class="muted">{{ t("delete.cancelled") }}</p>
          <ul v-else-if="preview" class="paths">
            <li v-for="item in preview.paths" :key="item">{{ item }}</li>
          </ul>
        </div>

        <p v-if="error" class="error">{{ t(error.message_key) }}</p>

        <div class="actions">
          <button type="button" @click="emit('close')">{{ t("delete.cancel") }}</button>
          <button class="danger" :disabled="!canConfirm" type="button" @click="confirm">
            {{ t("delete.confirm") }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  display: grid;
  place-items: center;
  background: rgb(0 0 0 / 55%);
  z-index: 90;
}
.dialog {
  width: min(560px, 92vw);
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 18px 20px;
  border-radius: 10px;
  background: #171c22;
  border: 1px solid #2a323c;
}
h2 {
  margin: 0;
  font-size: 17px;
}
.path {
  margin: 0;
  font-family: ui-monospace, monospace;
  word-break: break-all;
}
label.inline {
  display: flex;
  align-items: center;
  gap: 8px;
}
.preview {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-height: 320px;
  overflow: auto;
  border: 1px solid #2a323c;
  border-radius: 6px;
  padding: 10px;
  background: #101418;
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
button {
  padding: 7px 13px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #1d242c;
  color: inherit;
  cursor: pointer;
}
.danger {
  background: #b91c1c;
  border-color: #b91c1c;
}
.error {
  color: #ef4444;
  margin: 0;
  font-size: 13px;
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
.muted {
  color: #9aa4b2;
  margin: 0;
  font-size: 13px;
}
</style>
