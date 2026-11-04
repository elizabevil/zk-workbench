<script setup lang="ts">
import { nextTick, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { ConnectionSnapshot } from "@/api/types/ConnectionSnapshot";
import { useConnectionStore } from "@/stores/connection";

const props = defineProps<{ open: boolean }>();
const emit = defineEmits<{
  close: [];
  connected: [snapshot: ConnectionSnapshot];
}>();

const { t } = useI18n();
const store = useConnectionStore();

const host = ref("127.0.0.1");
const port = ref(2181);
const displayName = ref("local");
const localError = ref("");
const nameInput = ref<HTMLInputElement | null>(null);

watch(
  () => props.open,
  (open) => {
    if (open) {
      localError.value = "";
      void nextTick(() => nameInput.value?.focus());
    }
  },
);

async function submit(): Promise<void> {
  if (!host.value.trim()) {
    localError.value = t("home.hostRequired");
    return;
  }
  if (!Number.isInteger(port.value) || port.value < 1 || port.value > 65535) {
    localError.value = t("home.portInvalid");
    return;
  }
  localError.value = "";
  const created = await store.connect({
    host: host.value.trim(),
    port: port.value,
    displayName: displayName.value.trim() || `${host.value}:${port.value}`,
  });
  if (created) {
    emit("connected", created);
  }
}
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="overlay">
      <form class="dialog" @submit.prevent="submit">
        <h2>{{ t("home.new") }}</h2>

        <label>
          {{ t("home.name") }}
          <input ref="nameInput" v-model="displayName" />
        </label>
        <label>
          {{ t("home.host") }}
          <input v-model="host" placeholder="127.0.0.1" />
        </label>
        <label>
          {{ t("home.port") }}
          <input v-model.number="port" min="1" type="number" />
        </label>

        <p v-if="localError" class="error">{{ localError }}</p>
        <p v-else-if="store.error" class="error">
          {{ t(store.error.message_key) }}
          <span v-if="store.error.fields.detail">（{{ store.error.fields.detail }}）</span>
        </p>

        <div class="actions">
          <button type="button" @click="emit('close')">{{ t("home.cancel") }}</button>
          <button :disabled="store.loading" type="submit">
            {{ store.loading ? t("home.connecting") : t("home.connect") }}
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
  z-index: 100;
}
.dialog {
  width: min(420px, 90vw);
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 20px 24px;
  border-radius: 10px;
  background: #171c22;
  border: 1px solid #2a323c;
  box-shadow: 0 18px 50px rgb(0 0 0 / 45%);
}
h2 {
  margin: 0 0 4px;
  font-size: 18px;
}
label {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
input {
  padding: 8px 10px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
}
.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 4px;
}
button {
  padding: 8px 14px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #1d242c;
  color: inherit;
  cursor: pointer;
}
button[type="submit"] {
  background: #2563eb;
  border-color: #2563eb;
}
.error {
  margin: 0;
  color: #ef4444;
  font-size: 13px;
}
</style>
