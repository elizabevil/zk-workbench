<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import { type InvokeRejection, toAppError } from "@/api/client";
import { nodeSet } from "@/api/node";
import type { AppError } from "@/api/types/AppError";
import type { NodeStat } from "@/api/types/NodeStat";
import { decodeBase64Bytes, encodeBytesBase64 } from "@/utils/encoding";

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

const { t } = useI18n();
const hex = ref("");
const error = ref<AppError | null>(null);
const localError = ref("");
const saving = ref(false);
const fileName = ref("");

watch(
  () => props.open,
  (open) => {
    if (open) {
      hex.value = "";
      error.value = null;
      localError.value = "";
      fileName.value = "";
    }
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
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="overlay">
      <div class="dialog">
        <h2>{{ t("binary.title") }}</h2>
        <p class="path">{{ path }}</p>

        <label class="file">
          {{ t("binary.file") }}
          <input type="file" @change="pickFile" />
          <span v-if="fileName" class="muted">{{ fileName }}</span>
        </label>

        <label>
          {{ t("binary.hex") }}
          <textarea v-model="hex" class="hex" spellcheck="false" />
        </label>

        <p class="muted">
          {{ t("binary.bytes") }}: {{ byteLen }} → {{ newBytes?.length ?? "-" }}
          · {{ t("binary.delta") }}: {{ delta >= 0 ? `+${delta}` : delta }}
          · {{ t("binary.changed") }}: {{ changedBytes }}
        </p>

        <p v-if="localError" class="error">{{ localError }}</p>
        <p v-if="error" class="error">{{ t(error.message_key) }}</p>

        <div class="actions">
          <button type="button" @click="emit('close')">{{ t("binary.cancel") }}</button>
          <button class="danger" :disabled="saving" type="button" @click="save">
            {{ saving ? t("binary.saving") : t("binary.save") }}
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
  z-index: 95;
}
.dialog {
  width: min(720px, 94vw);
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
label {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 13px;
}
.hex {
  min-height: 180px;
  padding: 10px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
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
.muted {
  color: #9aa4b2;
  font-size: 12px;
}
</style>
