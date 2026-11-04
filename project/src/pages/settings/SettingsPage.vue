<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { appPaths } from "@/api/app";
import { type InvokeRejection, toAppError } from "@/api/client";
import { proxyPasswordClear, proxyPasswordSet, proxyTest } from "@/api/proxy";
import type { AppPaths } from "@/api/types/AppPaths";
import type { ProxyProfile } from "@/api/types/ProxyProfile";
import { LOCALES } from "@/i18n/locale";
import { NOTIFICATION_LEVELS, PROXY_KINDS } from "@/settings/constants";
import { useSettingsStore } from "@/stores/settings";
import {
  DEFAULT_SHORTCUTS,
  eventToBinding,
  loadShortcuts,
  type ShortcutAction,
  type ShortcutMap,
  saveShortcuts,
} from "@/utils/shortcuts";

const { t } = useI18n();
const router = useRouter();
const store = useSettingsStore();

const noProxyText = computed({
  get: () => store.settings?.proxy.no_proxy.join("\n") ?? "",
  set: (value: string) => {
    if (!store.settings) {
      return;
    }
    store.settings.proxy.no_proxy = value
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length > 0);
  },
});

const paths = ref<AppPaths | null>(null);

const sections = [
  "general",
  "appearance",
  "notifications",
  "retry",
  "task",
  "proxy",
  "shortcuts",
  "logs",
] as const;
const activeSection = ref<string>("general");
let observer: IntersectionObserver | undefined;

function scrollToSection(id: string): void {
  document.getElementById(`settings-${id}`)?.scrollIntoView({ behavior: "smooth", block: "start" });
}
const passwords = ref<Record<number, string>>({});
const testTarget = ref("127.0.0.1:2181");
const testMessage = ref("");
const dragIndex = ref<number | null>(null);
const shortcuts = ref<ShortcutMap>(loadShortcuts());
const SHORTCUT_ACTIONS: ShortcutAction[] = ["edit", "create", "delete", "refresh", "jump"];

function captureShortcut(action: ShortcutAction, event: KeyboardEvent): void {
  const binding = eventToBinding(event);
  if (!binding) {
    return;
  }
  shortcuts.value[action] = binding;
  saveShortcuts(shortcuts.value);
}

function resetShortcuts(): void {
  shortcuts.value = { ...DEFAULT_SHORTCUTS };
  saveShortcuts(shortcuts.value);
}

onMounted(async () => {
  await store.load();
  paths.value = await appPaths();
  await nextTick();
  const container = document.querySelector(".settings");
  const fieldsets = Array.from(container?.querySelectorAll("fieldset") ?? []);
  fieldsets.forEach((element, index) => {
    element.id = `settings-${sections[index] ?? index}`;
  });
  observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (entry.isIntersecting) {
          activeSection.value = (entry.target.id || "").replace("settings-", "");
        }
      }
    },
    { root: container, rootMargin: "-10% 0px -70% 0px", threshold: 0 },
  );
  for (const fieldset of fieldsets) {
    observer.observe(fieldset);
  }
});

onUnmounted(() => observer?.disconnect());

async function clearPassword(profile: ProxyProfile): Promise<void> {
  await proxyPasswordClear(profile.name);
}

async function resetSettings(): Promise<void> {
  if (!window.confirm(t("settings.resetConfirm"))) {
    return;
  }
  await store.reset();
}

async function testProxy(profile: ProxyProfile): Promise<void> {
  try {
    await proxyTest(profile, testTarget.value);
    testMessage.value = t("settings.proxy.testOk");
  } catch (raw) {
    const error = toAppError(raw as InvokeRejection);
    testMessage.value = t(error.message_key);
  }
}

function addProxy(): void {
  const profile: ProxyProfile = {
    name: "socks5://",
    kind: "socks5",
    host: "",
    port: 1080,
    username: null,
  };
  store.settings?.proxy.profiles.push(profile);
}

function dragStart(index: number): void {
  dragIndex.value = index;
}

function drop(index: number): void {
  const profiles = store.settings?.proxy.profiles;
  const from = dragIndex.value;
  dragIndex.value = null;
  if (!profiles || from === null || from === index) {
    return;
  }
  const [item] = profiles.splice(from, 1);
  profiles.splice(index, 0, item);
}

async function saveSettings(): Promise<void> {
  if (!store.settings) {
    return;
  }
  for (const profile of store.settings.proxy.profiles) {
    profile.name = `${profile.kind}://${profile.host}:${profile.port}`;
  }
  for (const [index, password] of Object.entries(passwords.value)) {
    const profile = store.settings.proxy.profiles[Number(index)];
    if (profile && password) {
      await proxyPasswordSet(profile.name, password);
      passwords.value[Number(index)] = "";
    }
  }
  await store.save();
}

function removeProxy(index: number): void {
  store.settings?.proxy.profiles.splice(index, 1);
}
</script>

<template>
  <section class="settings">
    <header class="header">
      <h1>{{ t("settings.title") }}</h1>
      <button type="button" @click="router.push('/')">{{ t("settings.back") }}</button>
    </header>

    <p v-if="store.error" class="error">{{ t(store.error.message_key) }}</p>
    <p v-if="!store.settings">{{ t("settings.loading") }}</p>

    <div class="settings-layout">
      <nav class="settings-nav">
        <button
          v-for="section in sections"
          :key="section"
          type="button"
          :class="{ active: activeSection === section }"
          @click="scrollToSection(section)"
        >
          {{ t(`settings.${section}.title`) }}
        </button>
      </nav>

      <form v-if="store.settings" class="sections settings-form" @submit.prevent="saveSettings">
      <fieldset>
        <legend>{{ t("settings.general.title") }}</legend>
        <label>
          {{ t("settings.general.locale") }}
          <select v-model="store.settings.general.locale">
            <option v-for="locale in LOCALES" :key="locale" :value="locale">
              {{ locale }}
            </option>
          </select>
        </label>
        <label>
          {{ t("settings.general.theme") }}
          <input v-model="store.settings.general.theme" />
        </label>
        <label class="inline">
          <input v-model="store.settings.general.restore_last_connection" type="checkbox" />
          {{ t("settings.general.restore") }}
        </label>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.appearance.title") }}</legend>
        <label>
          {{ t("settings.appearance.background") }}
          <input v-model="store.settings.appearance.background" />
        </label>
        <div class="preview" :style="{ background: store.settings.appearance.background }" />
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.notifications.title") }}</legend>
        <label>
          {{ t("settings.notifications.desktop") }}
          <select v-model="store.settings.notifications.desktop">
            <option v-for="level in NOTIFICATION_LEVELS" :key="level" :value="level">
              {{ level }}
            </option>
          </select>
        </label>
        <label class="inline">
          <input v-model="store.settings.notifications.command_finished" type="checkbox" />
          {{ t("settings.notifications.commandFinished") }}
        </label>
        <label class="inline">
          <input v-model="store.settings.notifications.sound" type="checkbox" />
          {{ t("settings.notifications.sound") }}
        </label>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.retry.title") }}</legend>
        <label class="inline">
          <input v-model="store.settings.retry.auto_reconnect" type="checkbox" />
          {{ t("settings.retry.autoReconnect") }}
        </label>
        <label>
          {{ t("settings.retry.maxAttempts") }}
          <input v-model.number="store.settings.retry.max_attempts" min="1" type="number" />
        </label>
        <label>
          {{ t("settings.retry.initial") }}
          <input v-model.number="store.settings.retry.initial_interval_ms" min="100" type="number" />
        </label>
        <label>
          {{ t("settings.retry.max") }}
          <input v-model.number="store.settings.retry.max_interval_ms" min="100" type="number" />
        </label>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.task.title") }}</legend>
        <label class="inline">
          <input v-model="store.settings.task.continue_on_error" type="checkbox" />
          {{ t("settings.task.continueOnError") }}
        </label>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.proxy.title") }}</legend>
        <label class="inline">
          <input v-model="store.settings.proxy.enabled" type="checkbox" />
          {{ t("settings.proxy.enabled") }}
        </label>
        <label>
          {{ t("settings.proxy.noProxy") }}
          <textarea v-model="noProxyText" rows="3" />
        </label>
        <ul class="proxy-list">
        <li
          v-for="(profile, index) in store.settings.proxy.profiles"
          :key="index"
          class="proxy-row"
          @dragover.prevent
          @drop="drop(index)"
        >
          <button
            class="proxy-handle"
            type="button"
            draggable="true"
            :title="t('settings.proxy.drag')"
            @dragstart="dragStart(index)"
          >
            ⠿
          </button>
          <select v-model="profile.kind">
            <option v-for="kind in PROXY_KINDS" :key="kind" :value="kind">
              {{ kind }}
            </option>
          </select>
          <input v-model="profile.host" :placeholder="t('settings.proxy.host')" />
          <input v-model.number="profile.port" :placeholder="t('settings.proxy.port')" type="number" />
          <input v-model="profile.username" :placeholder="t('settings.proxy.username')" />
          <input
            v-model="passwords[index]"
            :placeholder="t('settings.proxy.password')"
            type="password"
          />
          <div class="row-actions">
            <button type="button" @click="testProxy(profile)">{{ t("settings.proxy.test") }}</button>
            <button type="button" @click="clearPassword(profile)">
              {{ t("settings.proxy.clearPassword") }}
            </button>
            <button type="button" @click="removeProxy(index)">{{ t("settings.proxy.remove") }}</button>
          </div>
        </li>
        </ul>
        <button type="button" @click="addProxy">{{ t("settings.proxy.add") }}</button>
        <label>
          {{ t("settings.proxy.testTarget") }}
          <input v-model="testTarget" />
        </label>
        <p v-if="testMessage" class="hint">{{ testMessage }}</p>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.shortcuts.title") }}</legend>
        <p class="hint">{{ t("settings.shortcuts.hint") }}</p>
        <div v-for="action in SHORTCUT_ACTIONS" :key="action" class="shortcut-row">
          <span>{{ t(`settings.shortcuts.${action}`) }}</span>
          <input
            :value="shortcuts[action]"
            readonly
            @keydown.prevent="captureShortcut(action, $event)"
          />
        </div>
        <button type="button" @click="resetShortcuts">{{ t("settings.shortcuts.reset") }}</button>
      </fieldset>

      <fieldset>
        <legend>{{ t("settings.logs.title") }}</legend>
        <p class="hint">{{ t("settings.logs.hint") }}</p>
        <label>
          {{ t("settings.logs.directory") }}
          <input v-model="store.settings.logs.directory" :placeholder="t('settings.logs.directoryPlaceholder')" />
        </label>
        <p class="hint">{{ t("settings.logs.restartHint") }}</p>
        <label>
          {{ t("settings.logs.maxSize") }}
          <input v-model.number="store.settings.logs.max_size_bytes" min="4096" step="1024" type="number" />
        </label>
        <label>
          {{ t("settings.logs.retention") }}
          <input v-model.number="store.settings.logs.retention_days" min="0" type="number" />
        </label>
        <p v-if="paths" class="hint">{{ t("settings.logs.config") }}: {{ paths.config_path }}</p>
        <p v-if="paths" class="hint">{{ t("settings.logs.dir") }}: {{ paths.log_dir }}</p>
      </fieldset>

      <div class="actions">
        <button :disabled="store.saving" type="button" @click="saveSettings">{{ t("settings.save") }}</button>
        <button type="button" @click="resetSettings">{{ t("settings.resetConfig") }}</button>
      </div>
      </form>
    </div>
  </section>
</template>


<style scoped>
.settings {
  height: 100%;
  overflow-y: auto;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  width: 100%;
  max-width: 1180px;
  margin: 0 auto;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.settings-layout {
  display: flex;
  gap: 2%;
  min-height: 0;
  flex: 1;
}
.settings-nav {
  position: sticky;
  top: 0;
  align-self: flex-start;
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 0 0 18%;
}
.settings-nav button {
  text-align: left;
  padding: 8px 10px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: #9aa4b2;
  cursor: pointer;
}
.settings-nav button.active {
  background: rgb(37 99 235 / 22%);
  color: #fff;
}
.settings-form {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.sections {
  display: grid;
  gap: 16px;
  max-width: 900px;
}
fieldset {
  border: 1px solid #2a323c;
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px 16px;
}
legend {
  padding: 0 6px;
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
.shortcut-row {
  display: grid;
  grid-template-columns: 40% 1fr;
  align-items: center;
  gap: 12px;
}
.shortcut-row input {
  font-family: ui-monospace, monospace;
}
.proxy-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin: 0;
  padding: 0;
  list-style: none;
}
.proxy-row {
  display: grid;
  grid-template-columns: 2% 1fr 1fr 0.6fr 1fr 1fr auto;
  align-items: center;
  gap: 8px;
}
.proxy-handle {
  cursor: grab;
  color: #9aa4b2;
  text-align: center;
}
.proxy-handle:active {
  cursor: grabbing;
}
.row-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}
.actions {
  display: flex;
  gap: 12px;
}
.audit-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
.audit-export {
  display: flex;
  gap: 8px;
}
.audit-export input {
  flex: 1;
  padding: 7px 9px;
  border-radius: 6px;
  border: 1px solid #2a323c;
  background: #101418;
  color: inherit;
}
.audit-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 12px;
}
.audit-table th,
.audit-table td {
  text-align: left;
  padding: 6px 8px;
  border-bottom: 1px solid rgb(255 255 255 / 8%);
}
.audit-table .mono {
  font-family: ui-monospace, monospace;
}
.danger {
  border-color: rgb(248 113 113 / 45%);
  color: #f87171;
}
.metrics-head {
  display: flex;
  align-items: center;
  gap: 10px;
}
.metrics {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
  gap: 8px 16px;
  margin: 0;
}
.metrics div {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  padding: 4px 0;
  border-bottom: 1px solid rgb(255 255 255 / 6%);
}
.metrics dt {
  color: #9aa4b2;
  font-size: 12px;
}
.metrics dd {
  margin: 0;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}
.preview {
  height: 48px;
  border: 1px solid #2a323c;
  border-radius: 6px;
}
.hint {
  color: #9aa4b2;
  margin: 0;
}
.error {
  color: #ef4444;
}
</style>
