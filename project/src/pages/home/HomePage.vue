<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import type { ConnectionSnapshot } from "@/api/types/ConnectionSnapshot";
import ConnectionDialog from "@/components/ConnectionDialog.vue";
import { useConnectionStore } from "@/stores/connection";

const { t } = useI18n();
const router = useRouter();
const store = useConnectionStore();
const dialogOpen = ref(false);

const hasConnections = computed(() => store.connections.length > 0);

onMounted(() => store.refresh());

function open(snapshot: ConnectionSnapshot): void {
  store.activate(snapshot.connection_id);
  router.push("/workbench");
}

function connected(snapshot: ConnectionSnapshot): void {
  dialogOpen.value = false;
  open(snapshot);
}

async function disconnect(snapshot: ConnectionSnapshot): Promise<void> {
  await store.close(snapshot.connection_id);
}
</script>

<template>
  <div class="home">
    <header class="topbar">
      <div class="brand">ZK Workbench</div>
      <div class="spacer" />
      <button type="button" class="ghost" @click="router.push('/settings')">
        {{ t("home.settings") }}
      </button>
    </header>

    <main class="content">
      <div class="section-head">
        <div>
          <h1>{{ t("home.title") }}</h1>
          <p class="muted">{{ t("home.subtitle") }}</p>
        </div>
        <button type="button" class="primary" @click="dialogOpen = true">
          {{ t("home.new") }}
        </button>
      </div>

      <div v-if="hasConnections" class="grid">
        <article v-for="item in store.connections" :key="item.connection_id" class="card">
          <div class="card-head">
            <span class="dot" :data-state="item.state" />
            <strong>{{ item.display_name }}</strong>
            <span class="muted small">{{ item.connection_id }}</span>
          </div>
          <div class="endpoint">{{ item.host }}:{{ item.port }}</div>
          <div class="muted small">{{ t(`connection.state.${item.state}`) }}</div>
          <div class="card-actions">
            <button type="button" class="primary" @click="open(item)">{{ t("home.open") }}</button>
            <button type="button" class="ghost" @click="disconnect(item)">
              {{ t("home.disconnect") }}
            </button>
          </div>
        </article>
      </div>

      <div v-else class="empty">
        <h2>{{ t("home.empty") }}</h2>
        <p class="muted">{{ t("home.emptyHint") }}</p>
        <button type="button" class="primary" @click="dialogOpen = true">
          {{ t("home.new") }}
        </button>
      </div>
    </main>

    <ConnectionDialog
      :open="dialogOpen"
      @close="dialogOpen = false"
      @connected="connected"
    />
  </div>
</template>

<style scoped>
.home {
  min-height: 100%;
  color: #e6e6e6;
}
.topbar {
  display: flex;
  align-items: center;
  gap: 12px;
  height: 48px;
  padding: 0 16px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 80%);
  backdrop-filter: blur(6px);
}
.brand {
  font-weight: 600;
  letter-spacing: 0.2px;
}
.spacer {
  flex: 1;
}
.content {
  max-width: 1080px;
  margin: 0 auto;
  padding: 32px 24px;
}
.section-head {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 20px;
}
h1 {
  margin: 0;
  font-size: 22px;
}
.muted {
  color: #9aa4b2;
  margin: 4px 0 0;
}
.small {
  font-size: 12px;
}
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 14px;
}
.card {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 16px;
  border-radius: 10px;
  border: 1px solid rgb(255 255 255 / 10%);
  background: rgb(16 20 24 / 70%);
}
.card-head {
  display: flex;
  align-items: center;
  gap: 8px;
}
.endpoint {
  font-family: ui-monospace, monospace;
  font-size: 13px;
}
.card-actions {
  display: flex;
  gap: 8px;
  margin-top: 6px;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #6b7280;
}
.dot[data-state="connected"] {
  background: #22c55e;
}
.dot[data-state="disconnected"],
.dot[data-state="expired"] {
  background: #ef4444;
}
.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 72px 0;
  border: 1px dashed rgb(255 255 255 / 15%);
  border-radius: 12px;
  background: rgb(16 20 24 / 45%);
}
.empty h2 {
  margin: 0;
  font-size: 18px;
}
button {
  cursor: pointer;
  border-radius: 6px;
  padding: 8px 14px;
  border: 1px solid transparent;
  color: inherit;
}
.primary {
  background: #2563eb;
}
.ghost {
  background: transparent;
  border-color: rgb(255 255 255 / 18%);
}
</style>
