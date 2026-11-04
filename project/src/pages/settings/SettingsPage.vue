<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";

import { appPaths } from "@/api/app";
import { backgroundImageLoad, backgroundImageStore } from "@/api/appearance";
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
let suppressObserverUntil = 0;

function scrollToSection(id: string): void {
  activeSection.value = id;
  suppressObserverUntil = Date.now() + 700;
  const container = document.querySelector(".settings-form") as HTMLElement | null;
  const target = document.getElementById(`settings-${id}`) as HTMLElement | null;
  if (!container || !target) {
    return;
  }
  const top = target.getBoundingClientRect().top - container.getBoundingClientRect().top + container.scrollTop;
  container.scrollTo({ top: top - 8, behavior: "smooth" });
}
const passwords = ref<Record<number, string>>({});
const testTarget = ref("127.0.0.1:2181");
const testMessage = ref("");
const dragIndex = ref<number | null>(null);
const proxySaved = ref("");
let autoSaveTimer = 0;
const shortcutSaved = ref("");
const previewImage = ref("");

watch(
  () => [store.settings?.appearance.mode, store.settings?.appearance.image_path] as const,
  async ([mode, path]) => {
    if (mode === "image" && path) {
      try {
        previewImage.value = await backgroundImageLoad(path);
      } catch {
        previewImage.value = "";
      }
    } else {
      previewImage.value = "";
    }
  },
  { immediate: true },
);

const appearancePreview = computed(() => {
  const appearance = store.settings?.appearance;
  if (!appearance) {
    return {};
  }
  if (appearance.mode === "solid") {
    return { background: appearance.color };
  }
  if (appearance.mode === "image" && previewImage.value) {
    return { background: `url("${previewImage.value}") center / cover` };
  }
  return {
    background: `linear-gradient(${appearance.gradient_angle}deg, ${appearance.gradient_from}, ${appearance.gradient_to})`,
  };
});

async function pickBackground(event: Event): Promise<void> {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file || !store.settings) {
    return;
  }
  const dataUrl = await new Promise<string>((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(String(reader.result));
    reader.onerror = () => reject(reader.error);
    reader.readAsDataURL(file);
  });
  const base64 = dataUrl.slice(dataUrl.indexOf(",") + 1);
  const extension = file.name.split(".").pop()?.toLowerCase() ?? "png";
  const path = await backgroundImageStore(base64, extension);
  store.settings.appearance.image_path = path;
  store.settings.appearance.mode = "image";
}

function clearBackground(): void {
  if (store.settings) {
    store.settings.appearance.image_path = "";
  }
}
const autoSaved = ref("");
let lastProxyJson = "";
let settingsLoaded = false;
const shortcuts = ref<ShortcutMap>(loadShortcuts());
const SHORTCUT_ACTIONS: ShortcutAction[] = ["edit", "create", "delete", "refresh", "jump"];

function captureShortcut(action: ShortcutAction, event: KeyboardEvent): void {
  const binding = eventToBinding(event);
  if (!binding) {
    return;
  }
  shortcuts.value[action] = binding;
  saveShortcuts(shortcuts.value);
  shortcutSaved.value = t("settings.shortcuts.saved");
  window.setTimeout(() => {
    shortcutSaved.value = "";
  }, 2000);
}

function resetShortcuts(): void {
  shortcuts.value = { ...DEFAULT_SHORTCUTS };
  saveShortcuts(shortcuts.value);
}

function goBack(): void {
  if (window.history.length > 1) {
    router.back();
  } else {
    void router.push("/");
  }
}

function onEscape(event: KeyboardEvent): void {
  if (event.key === "Escape") {
    goBack();
  }
}

watch(
  () => store.settings,
  (value) => {
    if (!value || !settingsLoaded) {
      return;
    }
    const proxyJson = JSON.stringify(value.proxy);
    if (proxyJson !== lastProxyJson) {
      lastProxyJson = proxyJson;
      return;
    }
    window.clearTimeout(autoSaveTimer);
    autoSaveTimer = window.setTimeout(async () => {
      await store.save();
      autoSaved.value = t("settings.autoSaved");
      window.setTimeout(() => {
        autoSaved.value = "";
      }, 1500);
    }, 600);
  },
  { deep: true },
);

onMounted(async () => {
  await store.load();
  paths.value = await appPaths();
  await nextTick();
  const container = document.querySelector(".settings-form");
  const fieldsets = Array.from(container?.querySelectorAll("fieldset") ?? []);
  fieldsets.forEach((element, index) => {
    element.id = `settings-${sections[index] ?? index}`;
  });
  observer = new IntersectionObserver(
    (entries) => {
      if (Date.now() < suppressObserverUntil) {
        return;
      }
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
  lastProxyJson = JSON.stringify(store.settings?.proxy);
  settingsLoaded = true;
  window.addEventListener("keydown", onEscape);
});

onUnmounted(() => {
  observer?.disconnect();
  window.removeEventListener("keydown", onEscape);
});

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

async function applyProxy(): Promise<void> {
  await saveSettings();
  const profiles = store.settings?.proxy.profiles ?? [];
  const active = profiles.length ? `${profiles[0].kind}://${profiles[0].host}:${profiles[0].port}` : t("settings.proxy.direct");
  proxySaved.value = t("settings.proxy.saved", { target: active });
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
          {{ t("settings.appearance.mode") }}
          <select v-model="store.settings.appearance.mode">
            <option value="solid">{{ t("settings.appearance.solid") }}</option>
            <option value="gradient">{{ t("settings.appearance.gradient") }}</option>
            <option value="image">{{ t("settings.appearance.image") }}</option>
          </select>
        </label>
        <label v-if="store.settings.appearance.mode === 'solid'">
          {{ t("settings.appearance.color") }}
          <input v-model="store.settings.appearance.color" type="color" />
        </label>
        <template v-if="store.settings.appearance.mode === 'gradient'">
          <label>
            {{ t("settings.appearance.from") }}
            <input v-model="store.settings.appearance.gradient_from" type="color" />
          </label>
          <label>
            {{ t("settings.appearance.to") }}
            <input v-model="store.settings.appearance.gradient_to" type="color" />
          </label>
          <label>
            {{ t("settings.appearance.angle") }}
            <input v-model.number="store.settings.appearance.gradient_angle" max="360" min="0" type="number" />
          </label>
        </template>
        <template v-if="store.settings.appearance.mode === 'image'">
          <label>
            {{ t("settings.appearance.imageFile") }}
            <input accept="image/png,image/jpeg,image/webp,image/gif" type="file" @change="pickBackground" />
          </label>
          <p class="hint">{{ store.settings.appearance.image_path || t("settings.appearance.noImage") }}</p>
          <button type="button" @click="clearBackground">{{ t("settings.appearance.clearImage") }}</button>
        </template>
        <label>
          {{ t("settings.appearance.panelOpacity") }}
          <input v-model.number="store.settings.appearance.panel_opacity" max="100" min="0" type="range" />
        </label>
        <label>
          {{ t("settings.appearance.backgroundBlur") }}
          <input v-model.number="store.settings.appearance.background_blur" max="30" min="0" type="range" />
        </label>
        <label>
          {{ t("settings.appearance.fontSize") }}
          <input v-model.number="store.settings.appearance.font_size" max="22" min="10" type="number" />
        </label>
        <div class="preview" :style="appearancePreview" />
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
        <div class="row-actions">
          <button type="button" @click="addProxy">{{ t("settings.proxy.add") }}</button>
          <button type="button" @click="applyProxy">{{ t("settings.proxy.apply") }}</button>
        </div>
        <p v-if="proxySaved" class="hint">{{ proxySaved }}</p>
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
            :placeholder="t('settings.shortcuts.press')"
            readonly
            @keydown.prevent="captureShortcut(action, $event)"
            @focus="shortcutSaved = ''"
          />
        </div>
        <div class="row-actions">
          <button type="button" @click="resetShortcuts">{{ t("settings.shortcuts.reset") }}</button>
          <span v-if="shortcutSaved" class="hint">{{ shortcutSaved }}</span>
        </div>
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
          <input v-model.number="store.settings.logs.max_size_mb" min="1" step="1" type="number" />
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
  overflow: hidden;
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
  overflow: hidden;
}
.settings-nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 0 0 18%;
  overflow-y: auto;
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
.saved-tip {
  margin-left: 10px;
  font-size: 12px;
  font-weight: 400;
  color: #34d399;
}
.settings-form {
  flex: 1;
  min-width: 0;
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-right: 4px;
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
