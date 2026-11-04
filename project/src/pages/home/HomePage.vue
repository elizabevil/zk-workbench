<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import type { ConnectionProfile } from "@/api/types/ConnectionProfile";
import type { ConnectionSnapshot } from "@/api/types/ConnectionSnapshot";
import ConnectionDialog from "@/components/ConnectionDialog.vue";
import { useConnectionStore } from "@/stores/connection";
import { useProfilesStore } from "@/stores/profiles";

const { t } = useI18n();
const router = useRouter();
const connections = useConnectionStore();
const profiles = useProfilesStore();
const dialogOpen = ref(false);
const editTarget = ref<ConnectionProfile | null>(null);

const search = ref("");
const hasProfiles = computed(() => profiles.profiles.length > 0);
const filtered = computed(() => {
  const keyword = search.value.trim().toLowerCase();
  if (!keyword) {
    return profiles.profiles;
  }
  return profiles.profiles.filter((profile) =>
    [profile.name, profile.host, `${profile.host}:${profile.port}`, profile.group ?? ""]
      .join(" ")
      .toLowerCase()
      .includes(keyword),
  );
});
const groups = computed(() => {
  const map = new Map<string, ConnectionProfile[]>();
  for (const profile of filtered.value) {
    const key = profile.group?.trim() || t("home.ungrouped");
    const list = map.get(key) ?? [];
    list.push(profile);
    map.set(key, list);
  }
  return [...map.entries()].sort(([a], [b]) => a.localeCompare(b));
});

onMounted(async () => {
  await Promise.all([profiles.load(), connections.refresh()]);
});

function sessionOf(profile: ConnectionProfile): ConnectionSnapshot | undefined {
  return connections.connections.find(
    (item) => item.host === profile.host && item.port === profile.port && item.state === "connected",
  );
}

function create(): void {
  editTarget.value = null;
  dialogOpen.value = true;
}

function edit(profile: ConnectionProfile): void {
  editTarget.value = profile;
  dialogOpen.value = true;
}

async function saved(): Promise<void> {
  dialogOpen.value = false;
  editTarget.value = null;
  await profiles.load();
}

async function open(profile: ConnectionProfile): Promise<void> {
  const existing = sessionOf(profile);
  if (existing) {
    connections.activate(existing.connection_id);
    await router.push("/workspace");
    return;
  }
  const created = await connections.connect({
    host: profile.host,
    port: profile.port,
    displayName: profile.name,
  });
  if (created) {
    await router.push("/workspace");
  }
}

async function remove(profile: ConnectionProfile): Promise<void> {
  if (!window.confirm(t("home.confirmDelete", { name: profile.name }))) {
    return;
  }
  await profiles.remove(profile.id);
}

async function connected(): Promise<void> {
  dialogOpen.value = false;
  await router.push("/workspace");
}
</script>

<template>
  <div class="home">
    <header class="topbar">
      <div class="brand">ZK Workbench</div>
      <div class="spacer" />
      <button type="button" class="ghost" @click="router.push('/audit')">
        {{ t("home.audit") }}
      </button>
      <button type="button" class="ghost" @click="router.push('/observability')">
        {{ t("home.observability") }}
      </button>
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
        <div class="head-actions">
          <input v-model="search" class="search" :placeholder="t('home.searchPlaceholder')" />
          <button type="button" class="primary" @click="create">
            {{ t("home.new") }}
          </button>
        </div>
      </div>

      <div v-if="hasProfiles" class="groups">
        <section v-for="[groupName, items] in groups" :key="groupName" class="group">
          <h2 class="group-title">{{ groupName }}</h2>
          <div class="grid">
            <article v-for="profile in items" :key="profile.id" class="card">
          <div class="card-head">
            <span class="dot" :data-state="sessionOf(profile)?.state ?? 'idle'" />
            <strong>{{ profile.name }}</strong>
          </div>
          <div class="endpoint">{{ profile.host }}:{{ profile.port }}</div>
          <div class="muted small">
            {{ sessionOf(profile) ? t("home.connected") : t("home.saved") }}
          </div>
          <div class="card-actions">
            <button type="button" class="primary" @click="open(profile)">
              {{ sessionOf(profile) ? t("home.open") : t("home.connect") }}
            </button>
            <button type="button" class="ghost" @click="edit(profile)">
              {{ t("home.edit") }}
            </button>
            <button type="button" class="ghost" @click="remove(profile)">
              {{ t("home.delete") }}
            </button>
          </div>
            </article>
          </div>
        </section>
      </div>

      <div v-else class="empty">
        <h2>{{ t("home.empty") }}</h2>
        <p class="muted">{{ t("home.emptyHint") }}</p>
        <button type="button" class="primary" @click="create">
          {{ t("home.new") }}
        </button>
      </div>

      <p v-if="profiles.error" class="error">{{ t(profiles.error.message_key) }}</p>
    </main>

    <ConnectionDialog
      :open="dialogOpen"
      :profile="editTarget"
      @close="dialogOpen = false"
      @connected="connected"
      @saved="saved"
    />
  </div>
</template>

<style scoped>
.home {
  height: 100%;
  overflow-y: auto;
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
.head-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}
.search {
  width: 220px;
  padding: 8px 10px;
  border-radius: 6px;
  border: 1px solid rgb(255 255 255 / 12%);
  background: rgb(16 20 24 / 70%);
  color: inherit;
}
.group {
  margin-bottom: 22px;
}
.group-title {
  margin: 0 0 10px;
  font-size: 14px;
  font-weight: 600;
  color: #cbd5e1;
}
.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 14px;
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
.error {
  margin-top: 16px;
  color: #ef4444;
}
</style>
