<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import { type InvokeRejection, toAppError } from "@/api/client";
import { nodeCreate } from "@/api/node";
import type { AppError } from "@/api/types/AppError";
import type { NodeCreateMode } from "@/api/types/NodeCreateMode";
import { encodeUtf8Base64 } from "@/utils/encoding";

const props = defineProps<{
  open: boolean;
  connectionId: string;
  defaultPath: string;
}>();

const emit = defineEmits<{
  close: [];
  created: [actualPath: string];
}>();

const { t } = useI18n();
const path = ref("/");
const mode = ref<NodeCreateMode>("persistent");
const data = ref("");
const createParents = ref(false);
const error = ref<AppError | null>(null);
const saving = ref(false);

watch(
  () => props.open,
  (open) => {
    if (open) {
      path.value = props.defaultPath;
      mode.value = "persistent";
      data.value = "";
      createParents.value = false;
      error.value = null;
    }
  },
);

const modes: NodeCreateMode[] = [
  "persistent",
  "ephemeral",
  "persistent_sequential",
  "ephemeral_sequential",
];

async function submit(): Promise<void> {
  saving.value = true;
  error.value = null;
  try {
    const actual = await nodeCreate(
      props.connectionId,
      path.value,
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
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="overlay">
      <form class="dialog" @submit.prevent="submit">
        <h2>{{ t("create.title") }}</h2>
        <label>
          {{ t("create.path") }}
          <input v-model="path" />
        </label>
        <label>
          {{ t("create.modeLabel") }}
          <select v-model="mode">
            <option v-for="item in modes" :key="item" :value="item">
              {{ t(`create.mode.${item}`) }}
            </option>
          </select>
        </label>
        <label>
          {{ t("create.data") }}
          <textarea v-model="data" rows="5" />
        </label>
        <label class="inline">
          <input v-model="createParents" type="checkbox" />
          {{ t("create.parents") }}
        </label>
        <p v-if="mode === 'ephemeral' || mode === 'ephemeral_sequential'" class="warn">
          {{ t("create.ephemeralHint") }}
        </p>
        <p v-if="error" class="error">{{ t(error.message_key) }}</p>
        <div class="actions">
          <button type="button" @click="emit('close')">{{ t("create.cancel") }}</button>
          <button class="primary" :disabled="saving" type="submit">
            {{ saving ? t("create.saving") : t("create.submit") }}
          </button>
        </div>
      </form>
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
input,
select,
textarea {
  padding: 7px 9px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
  font-family: inherit;
}
textarea {
  font-family: ui-monospace, monospace;
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
.primary {
  background: #2563eb;
  border-color: #2563eb;
}
.error {
  color: #ef4444;
  margin: 0;
  font-size: 13px;
}
.warn {
  color: #fbbf24;
  margin: 0;
  font-size: 12px;
}
</style>
