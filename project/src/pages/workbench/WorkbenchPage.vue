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

const menuOpen = ref(false);
const dialogOpen = ref(false);

const stateLabel = computed(() =>
  store.snapshot ? t(`connection.state.${store.snapshot.state}`) : t("shell.noConnection"),
);

onMounted(async () => {
  await store.refresh();
  if (!store.snapshot) {
    const first = store.connections[0];
    if (first) {
      store.activate(first.connection_id);
    } else {
      await router.replace("/");
    }
  }
});

function switchTo(snapshot: ConnectionSnapshot): void {
  store.activate(snapshot.connection_id);
  menuOpen.value = false;
}

function onConnected(snapshot: ConnectionSnapshot): void {
  dialogOpen.value = false;
  store.activate(snapshot.connection_id);
}

async function disconnect(): Promise<void> {
  menuOpen.value = false;
  await store.close();
  if (store.connections.length === 0) {
    await router.replace("/");
  }
}
</script>

<template>
  <div class="shell">
    <header class="topbar">
      <div class="conn">
        <button class="chip" type="button" @click="menuOpen = !menuOpen">
          <span class="dot" :data-state="store.snapshot?.state ?? 'none'" />
          <span class="name">{{ store.snapshot?.display_name ?? t("shell.noConnection") }}</span>
          <span v-if="store.snapshot" class="endpoint">
            {{ store.snapshot.host }}:{{ store.snapshot.port }}
          </span>
          <span class="caret">▾</span>
        </button>

        <div v-if="menuOpen" class="menu">
          <button
            v-for="item in store.connections"
            :key="item.connection_id"
            type="button"
            class="menu-item"
            :class="{ active: item.connection_id === store.snapshot?.connection_id }"
            @click="switchTo(item)"
          >
            <span class="dot" :data-state="item.state" />
            <span>{{ item.display_name }}</span>
            <span class="muted">{{ item.host }}:{{ item.port }}</span>
          </button>
          <div class="sep" />
          <button type="button" class="menu-item" @click="dialogOpen = true; menuOpen = false">
            {{ t("shell.newConnection") }}
          </button>
          <button type="button" class="menu-item" @click="router.push('/')">
            {{ t("shell.home") }}
          </button>
          <button type="button" class="menu-item danger" @click="disconnect">
            {{ t("shell.disconnect") }}
          </button>
        </div>
      </div>

      <div class="spacer" />
      <button type="button" class="ghost" @click="router.push('/settings')">
        {{ t("home.settings") }}
      </button>
    </header>

    <div class="body">
      <aside class="tree">
        <div class="aside-head">{{ t("shell.tree") }}</div>
        <p class="muted">{{ t("shell.treeHint") }}</p>
      </aside>

      <section class="content">
        <div class="placeholder">
          <h2>{{ t("shell.contentTitle") }}</h2>
          <p class="muted">{{ t("shell.contentHint") }}</p>
        </div>
      </section>
    </div>

    <footer class="statusbar">
      <span>{{ t("shell.state") }}: {{ stateLabel }}</span>
      <span>generation: {{ store.snapshot?.generation ?? "-" }}</span>
      <span v-if="store.snapshot">
        {{ t("shell.session") }}: {{ Math.round(store.snapshot.session_timeout_ms / 1000) }}s
      </span>
      <div class="spacer" />
      <span>{{ store.snapshot?.connection_id ?? "" }}</span>
    </footer>

    <ConnectionDialog
      :open="dialogOpen"
      @close="dialogOpen = false"
      @connected="onConnected"
    />
  </div>
</template>

<style scoped>
.shell {
  display: flex;
  flex-direction: column;
  height: 100%;
  color: #e6e6e6;
}
.topbar {
  position: relative;
  display: flex;
  align-items: center;
  height: 48px;
  padding: 0 12px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 80%);
  backdrop-filter: blur(6px);
}
.conn {
  position: relative;
}
.chip {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 12%);
  background: rgb(255 255 255 / 6%);
  color: inherit;
  cursor: pointer;
}
.chip:hover {
  background: rgb(255 255 255 / 10%);
}
.name {
  font-weight: 600;
}
.endpoint {
  color: #9aa4b2;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}
.caret {
  color: #9aa4b2;
  font-size: 11px;
}
.menu {
  position: absolute;
  top: calc(100% + 6px);
  left: 0;
  width: 320px;
  display: flex;
  flex-direction: column;
  padding: 6px;
  border-radius: 8px;
  border: 1px solid #2a323c;
  background: #171c22;
  box-shadow: 0 18px 40px rgb(0 0 0 / 45%);
  z-index: 20;
}
.menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: inherit;
  cursor: pointer;
  text-align: left;
}
.menu-item:hover,
.menu-item.active {
  background: rgb(255 255 255 / 8%);
}
.menu-item.danger {
  color: #f87171;
}
.sep {
  height: 1px;
  margin: 4px 2px;
  background: rgb(255 255 255 / 10%);
}
.spacer {
  flex: 1;
}
.body {
  display: flex;
  flex: 1;
  min-height: 0;
}
.tree {
  width: 260px;
  padding: 16px;
  border-right: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 45%);
}
.aside-head {
  font-weight: 600;
  margin-bottom: 8px;
}
.content {
  flex: 1;
  display: grid;
  place-items: center;
}
.placeholder {
  text-align: center;
}
.placeholder h2 {
  margin: 0 0 6px;
  font-size: 18px;
}
.statusbar {
  display: flex;
  align-items: center;
  gap: 16px;
  height: 28px;
  padding: 0 12px;
  font-size: 12px;
  color: #9aa4b2;
  border-top: 1px solid rgb(255 255 255 / 8%);
  background: rgb(16 20 24 / 80%);
}
.muted {
  color: #9aa4b2;
  font-size: 12px;
}
.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #6b7280;
  flex: none;
}
.dot[data-state="connected"] {
  background: #22c55e;
}
.dot[data-state="disconnected"],
.dot[data-state="expired"] {
  background: #ef4444;
}
.ghost {
  padding: 6px 12px;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 18%);
  background: transparent;
  color: inherit;
  cursor: pointer;
}
</style>
