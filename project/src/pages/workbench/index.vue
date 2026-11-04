<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useI18n } from "vue-i18n";

import { listenConnectionState } from "../../api/events";
import { useConnectionStore } from "../../stores/connection";

const { t } = useI18n();
const store = useConnectionStore();

const endpoint = ref("127.0.0.1:2181");
const displayName = ref("local");
let unlisten: (() => void) | undefined;

const stateLabel = computed(() =>
  store.snapshot ? t(`connection.state.${store.snapshot.state}`) : t("workbench.empty"),
);

const errorLabel = computed(() => (store.error ? t(store.error.message_key) : ""));

onMounted(async () => {
  unlisten = await listenConnectionState((event) => store.applyEvent(event));
});

onUnmounted(() => {
  unlisten?.();
});

async function connect(): Promise<void> {
  await store.connect(endpoint.value, displayName.value);
}
</script>

<template>
  <section class="workbench">
    <h1>{{ t("app.title") }}</h1>

    <form class="form" @submit.prevent="connect">
      <label>
        {{ t("workbench.endpoint") }}
        <input v-model="endpoint" :placeholder="t('workbench.endpointPlaceholder')" />
      </label>
      <label>
        {{ t("workbench.displayName") }}
        <input v-model="displayName" />
      </label>
      <button type="submit" :disabled="store.loading || store.snapshot !== null">
        {{ store.loading ? t("workbench.connecting") : t("workbench.connect") }}
      </button>
    </form>

    <div class="status">
      <span class="state" :data-state="store.snapshot?.state ?? 'none'">{{ stateLabel }}</span>
      <span v-if="store.snapshot">
        {{ t("workbench.generation") }}: {{ store.snapshot.generation }}
      </span>
      <button v-if="store.snapshot" type="button" @click="store.close()">
        {{ t("workbench.close") }}
      </button>
    </div>

    <p v-if="errorLabel" class="error">{{ t("error.title") }}: {{ errorLabel }}</p>
  </section>
</template>

<style scoped>
.workbench {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 24px;
}
.form {
  display: flex;
  gap: 12px;
  align-items: flex-end;
}
.form label {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.status {
  display: flex;
  gap: 12px;
  align-items: center;
}
.state {
  padding: 2px 8px;
  border-radius: 4px;
  background: #2a323c;
  color: #e6e6e6;
}
.state[data-state="connected"] {
  background: #22c55e;
  color: #0b1f10;
}
.state[data-state="disconnected"],
.state[data-state="expired"] {
  background: #ef4444;
  color: #2a0b0b;
}
.error {
  color: #ef4444;
}
</style>
