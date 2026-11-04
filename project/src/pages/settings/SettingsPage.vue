<script lang="ts" setup>
import {
  NAlert,
  NButton,
  NCheckbox,
  NColorPicker,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NSelect,
  NSlider,
} from "naive-ui";
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import {useI18n} from "vue-i18n";
import {useRouter} from "vue-router";

import {appPaths} from "@/api/app";
import {backgroundImageLoad, backgroundImageStore} from "@/api/appearance";
import {type InvokeRejection, toAppError} from "@/api/client";
import {proxyPasswordClear, proxyPasswordSet, proxyTest} from "@/api/proxy";
import type {AppPaths} from "@/api/types/AppPaths";
import type {Locale} from "@/api/types/Locale";
import type {NotificationLevel} from "@/api/types/NotificationLevel";
import type {ProxyProfile} from "@/api/types/ProxyProfile";
import {KEY_ESCAPE} from "@/constants/keyboard";
import {LOCALES} from "@/i18n/locale";
import {NOTIFICATION_LEVELS, PROXY_KINDS} from "@/settings/constants";
import {useSettingsStore} from "@/stores/settings";
import {useUiStore} from "@/stores/ui";
import {applyNumber} from "@/utils/number";
import {
  DEFAULT_SHORTCUTS,
  eventToBinding,
  loadShortcuts,
  saveShortcuts,
  type ShortcutAction,
  type ShortcutMap,
} from "@/utils/shortcuts";

const {t} = useI18n();
const router = useRouter();
const store = useSettingsStore();
const ui = useUiStore();

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

const localeOptions = LOCALES.map((locale) => ({label: locale, value: locale}));
const proxyKindOptions = PROXY_KINDS.map((kind) => ({label: kind, value: kind}));
const appearanceModeOptions = computed(() => [
  {label: t("settings.appearance.solid"), value: "solid"},
  {label: t("settings.appearance.gradient"), value: "gradient"},
  {label: t("settings.appearance.image"), value: "image"},
]);
const notificationOptions = computed(() =>
  NOTIFICATION_LEVELS.map((level) => ({label: level, value: level})),
);

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
  container.scrollTo({top: top - 8, behavior: "smooth"});
}

const passwords = ref<Record<number, string>>({});
const testTarget = ref("127.0.0.1:2181");
const testMessage = ref("");
const dragIndex = ref<number>();
const proxySaved = ref("");
let autoSaveTimer = 0;
let proxyTipTimer = 0;
let testTipTimer = 0;
const shortcutSaved = ref("");
const previewImage = ref("");
const selectedImageName = ref("");
const backgroundFileInput = ref<HTMLInputElement>();

const imageName = computed(() => {
  const path = store.settings?.appearance.image_path ?? "";
  if (!path) {
    return "";
  }
  return selectedImageName.value || path.split(/[\\/]/).pop() || "";
});

function setProxySaved(message: string): void {
  proxySaved.value = message;
  window.clearTimeout(proxyTipTimer);
  proxyTipTimer = window.setTimeout(() => {
    proxySaved.value = "";
  }, 4000);
}

function setTestMessage(message: string): void {
  testMessage.value = message;
  window.clearTimeout(testTipTimer);
  testTipTimer = window.setTimeout(() => {
    testMessage.value = "";
  }, 4000);
}

function setLocale(value: unknown): void {
  if (!store.settings) {
    return;
  }
  if (value === "zh-CN" || value === "en-US") {
    store.settings.general.locale = value as Locale;
  }
}

// 后端把 u64 序列化为 JSON number，ts-rs 生成的类型是 bigint；这里只在类型层转换。
function toU64(value: number): bigint {
  return value as unknown as bigint;
}

function fromU64(value: bigint | number): number {
  return Number(value);
}

function setAppearanceMode(value: unknown): void {
  if (!store.settings) {
    return;
  }
  if (value === "solid" || value === "gradient" || value === "image") {
    store.settings.appearance.mode = value;
  }
}

function setNotificationLevel(value: unknown): void {
  if (!store.settings) {
    return;
  }
  if (typeof value === "string" && (NOTIFICATION_LEVELS as readonly string[]).includes(value)) {
    store.settings.notifications.desktop = value as NotificationLevel;
  }
}

function setProxyKind(profile: ProxyProfile, value: unknown): void {
  if (value === "socks5" || value === "http_connect") {
    profile.kind = value;
  }
}

function setGradientAngle(value: number): void {
  if (store.settings) {
    store.settings.appearance.gradient_angle = value;
  }
}

function setPanelOpacity(value: number): void {
  if (store.settings) {
    store.settings.appearance.panel_opacity = value;
  }
}

function setBackgroundBlur(value: number): void {
  if (store.settings) {
    store.settings.appearance.background_blur = value;
  }
}

function setFontSize(value: number): void {
  if (store.settings) {
    store.settings.appearance.font_size = value;
  }
}

function setMaxAttempts(value: number): void {
  if (store.settings) {
    store.settings.retry.max_attempts = value;
  }
}

function setInitialInterval(value: number): void {
  if (store.settings) {
    store.settings.retry.initial_interval_ms = toU64(value);
  }
}

function setMaxInterval(value: number): void {
  if (store.settings) {
    store.settings.retry.max_interval_ms = toU64(value);
  }
}

function setLogMaxSize(value: number): void {
  if (store.settings) {
    store.settings.logs.max_size_mb = toU64(value);
  }
}

function setLogRetention(value: number): void {
  if (store.settings) {
    store.settings.logs.retention_days = value;
  }
}

function setProxyPort(profile: ProxyProfile, value: number): void {
  profile.port = value;
}

function applyProxyPort(profile: ProxyProfile, value: unknown): void {
  applyNumber(value, (port) => setProxyPort(profile, port));
}

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
  {immediate: true},
);

const appearancePreview = computed(() => {
  const appearance = store.settings?.appearance;
  if (!appearance) {
    return {};
  }
  if (appearance.mode === "solid") {
    return {background: appearance.color};
  }
  if (appearance.mode === "image" && previewImage.value) {
    return {background: `url("${previewImage.value}") center / cover`};
  }
  return {
    background: `linear-gradient(${appearance.gradient_angle}deg, ${appearance.gradient_from}, ${appearance.gradient_to})`,
  };
});

function chooseBackground(): void {
  backgroundFileInput.value?.click();
}

async function pickBackground(event: Event): Promise<void> {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (!file || !store.settings) {
    return;
  }
  selectedImageName.value = file.name;
  input.value = "";
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
    selectedImageName.value = "";
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
  shortcuts.value = {...DEFAULT_SHORTCUTS};
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
  if (event.key === KEY_ESCAPE) {
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
  {deep: true},
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
    {root: container, rootMargin: "-10% 0px -40% 0px", threshold: 0},
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
  window.clearTimeout(proxyTipTimer);
  window.clearTimeout(testTipTimer);
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
    setTestMessage(t("settings.proxy.testOk"));
  } catch (raw) {
    const error = toAppError(raw as InvokeRejection);
    setTestMessage(t(error.message_key));
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
  dragIndex.value = undefined;
  if (!profiles || from === undefined || from === index) {
    return;
  }
  const [item] = profiles.splice(from, 1);
  profiles.splice(index, 0, item);
}

async function applyProxy(): Promise<void> {
  await saveSettings();
  const profiles = store.settings?.proxy.profiles ?? [];
  const active = profiles.length
    ? `${profiles[0].kind}://${profiles[0].host}:${profiles[0].port}`
    : t("settings.proxy.direct");
  setProxySaved(t("settings.proxy.saved", {target: active}));
}

function submitSettings(event: Event): void {
  event.preventDefault();
  void saveSettings();
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
      <div class="header-actions">
        <span :title="t('settings.zoom')" class="zoom-badge">{{ ui.zoomPercent }}%</span>
        <button :title="t('settings.zoomReset')" type="button" @click="ui.resetZoom()">
          {{ t("settings.zoomReset") }}
        </button>
        <button type="button" @click="router.push('/')">{{ t("settings.back") }}</button>
      </div>
    </header>

    <NAlert v-if="store.error" :bordered="false" type="error">
      {{ t(store.error.message_key) }}
    </NAlert>
    <p v-if="!store.settings">{{ t("settings.loading") }}</p>

    <div class="settings-layout">
      <nav class="settings-nav">
        <button
          v-for="section in sections"
          :key="section"
          :class="{ active: activeSection === section }"
          type="button"
          @click="scrollToSection(section)"
        >
          {{ t(`settings.${section}.title`) }}
        </button>
      </nav>

      <NForm
        v-if="store.settings"
        :on-submit="submitSettings"
        :show-feedback="false"
        class="settings-form"
        label-align="left"
        label-placement="left"
        label-width="180"
        size="small"
      >
        <fieldset>
          <legend>{{ t("settings.general.title") }}</legend>
          <NFormItem :label="t('settings.general.locale')">
            <NSelect
              :options="localeOptions"
              :value="store.settings.general.locale"
              class="control-sm"
              @update:value="setLocale"
            />
          </NFormItem>
          <NFormItem :label="t('settings.general.theme')">
            <NInput v-model:value="store.settings.general.theme" class="control-md" />
          </NFormItem>
          <NFormItem :label="t('settings.general.restore')">
            <NCheckbox v-model:checked="store.settings.general.restore_last_connection" />
          </NFormItem>
        </fieldset>

        <fieldset>
          <legend>{{ t("settings.appearance.title") }}</legend>
          <NFormItem :label="t('settings.appearance.mode')">
            <NSelect
              :options="appearanceModeOptions"
              :value="store.settings.appearance.mode"
              class="control-sm"
              @update:value="setAppearanceMode"
            />
          </NFormItem>
          <NFormItem v-if="store.settings.appearance.mode === 'solid'" :label="t('settings.appearance.color')">
            <NColorPicker
              v-model:value="store.settings.appearance.color"
              :modes="['hex']"
              :show-alpha="false"
              class="control-color"
            />
          </NFormItem>
          <template v-if="store.settings.appearance.mode === 'gradient'">
            <NFormItem :label="t('settings.appearance.from')">
              <NColorPicker
                v-model:value="store.settings.appearance.gradient_from"
                :modes="['hex']"
                :show-alpha="false"
                class="control-color"
              />
            </NFormItem>
            <NFormItem :label="t('settings.appearance.to')">
              <NColorPicker
                v-model:value="store.settings.appearance.gradient_to"
                :modes="['hex']"
                :show-alpha="false"
                class="control-color"
              />
            </NFormItem>
            <NFormItem :label="t('settings.appearance.angle')">
              <NInputNumber
                :max="360"
                :min="0"
                :show-button="false"
                :value="store.settings.appearance.gradient_angle"
                class="control-xs"
                @update:value="(value) => applyNumber(value, setGradientAngle)"
              />
            </NFormItem>
          </template>
          <template v-if="store.settings.appearance.mode === 'image'">
            <NFormItem :label="t('settings.appearance.imageFile')">
              <div class="file-row">
                <NButton size="small" @click="chooseBackground">
                  {{ t("settings.appearance.imageFile") }}
                </NButton>
                <span v-if="imageName" :title="store.settings.appearance.image_path" class="file-name">
                  {{ imageName }}
                </span>
                <span v-else class="hint">{{ t("settings.appearance.noImage") }}</span>
                <input
                  ref="backgroundFileInput"
                  accept="image/png,image/jpeg,image/webp,image/gif"
                  class="file-input-hidden"
                  type="file"
                  @change="pickBackground"
                />
              </div>
            </NFormItem>
            <div class="actions">
              <NButton :disabled="!store.settings.appearance.image_path" size="small" @click="clearBackground">
                {{ t("settings.appearance.clearImage") }}
              </NButton>
            </div>
          </template>
          <NFormItem :label="t('settings.appearance.panelOpacity')">
            <div class="slider-control">
              <NSlider
                :max="100"
                :min="0"
                :value="store.settings.appearance.panel_opacity"
                @update:value="(value) => applyNumber(value, setPanelOpacity)"
              />
              <span class="slider-value">{{ store.settings.appearance.panel_opacity }}</span>
            </div>
          </NFormItem>
          <NFormItem :label="t('settings.appearance.backgroundBlur')">
            <div class="slider-control">
              <NSlider
                :max="30"
                :min="0"
                :value="store.settings.appearance.background_blur"
                @update:value="(value) => applyNumber(value, setBackgroundBlur)"
              />
              <span class="slider-value">{{ store.settings.appearance.background_blur }}</span>
            </div>
          </NFormItem>
          <NFormItem :label="t('settings.appearance.fontSize')">
            <NInputNumber
              :max="22"
              :min="10"
              :show-button="false"
              :value="store.settings.appearance.font_size"
              class="control-xs"
              @update:value="(value) => applyNumber(value, setFontSize)"
            />
          </NFormItem>
          <div :style="appearancePreview" class="preview">
            <span class="preview-sample" :style="{fontSize: `${store.settings.appearance.font_size}px`}">
              {{ t("settings.appearance.previewText") }}
            </span>
            <span class="preview-meta">{{ store.settings.appearance.font_size }} px</span>
          </div>
        </fieldset>

        <fieldset>
          <legend>{{ t("settings.notifications.title") }}</legend>
          <NFormItem :label="t('settings.notifications.desktop')">
            <NSelect
              :options="notificationOptions"
              :value="store.settings.notifications.desktop"
              class="control-sm"
              @update:value="setNotificationLevel"
            />
          </NFormItem>
          <NFormItem :label="t('settings.notifications.commandFinished')">
            <NCheckbox v-model:checked="store.settings.notifications.command_finished" />
          </NFormItem>
          <NFormItem :label="t('settings.notifications.sound')">
            <NCheckbox v-model:checked="store.settings.notifications.sound" />
          </NFormItem>
        </fieldset>

        <fieldset>
          <legend>{{ t("settings.retry.title") }}</legend>
          <NFormItem :label="t('settings.retry.autoReconnect')">
            <NCheckbox v-model:checked="store.settings.retry.auto_reconnect" />
          </NFormItem>
          <NFormItem :label="t('settings.retry.maxAttempts')">
            <NInputNumber
              :min="1"
              :show-button="false"
              :value="store.settings.retry.max_attempts"
              class="control-xs"
              @update:value="(value) => applyNumber(value, setMaxAttempts)"
            />
          </NFormItem>
          <NFormItem :label="t('settings.retry.initial')">
            <NInputNumber
              :min="100"
              :show-button="false"
              :value="fromU64(store.settings.retry.initial_interval_ms)"
              class="control-sm"
              @update:value="(value) => applyNumber(value, setInitialInterval)"
            />
          </NFormItem>
          <NFormItem :label="t('settings.retry.max')">
            <NInputNumber
              :min="100"
              :show-button="false"
              :value="fromU64(store.settings.retry.max_interval_ms)"
              class="control-sm"
              @update:value="(value) => applyNumber(value, setMaxInterval)"
            />
          </NFormItem>
        </fieldset>

        <fieldset>
          <legend>{{ t("settings.task.title") }}</legend>
          <NFormItem :label="t('settings.task.continueOnError')">
            <NCheckbox v-model:checked="store.settings.task.continue_on_error" />
          </NFormItem>
        </fieldset>

        <fieldset>
          <legend>{{ t("settings.proxy.title") }}</legend>
          <NFormItem :label="t('settings.proxy.enabled')">
            <NCheckbox v-model:checked="store.settings.proxy.enabled" />
          </NFormItem>
          <NFormItem :label="t('settings.proxy.noProxy')">
            <NInput
              v-model:value="noProxyText"
              :autosize="{minRows: 3, maxRows: 6}"
              class="control-wide"
              type="textarea"
            />
          </NFormItem>

          <div class="proxy-block">
            <ul class="proxy-list">
              <li
                v-for="(profile, index) in store.settings.proxy.profiles"
                :key="index"
                class="proxy-row"
                @dragover.prevent
                @drop="drop(index)"
              >
                <button
                  :title="t('settings.proxy.drag')"
                  class="proxy-handle"
                  draggable="true"
                  type="button"
                  @dragstart="dragStart(index)"
                >
                  ⠿
                </button>
                <NSelect
                  :options="proxyKindOptions"
                  :value="profile.kind"
                  size="small"
                  @update:value="(value) => setProxyKind(profile, value)"
                />
                <NInput v-model:value="profile.host" :placeholder="t('settings.proxy.host')" size="small" />
                <NInputNumber
                  :max="65535"
                  :min="1"
                  :show-button="false"
                  :value="profile.port"
                  size="small"
                  @update:value="(value) => applyProxyPort(profile, value)"
                />
                <NInput
                  v-model:value="profile.username"
                  :placeholder="t('settings.proxy.username')"
                  size="small"
                />
                <NInput
                  v-model:value="passwords[index]"
                  :placeholder="t('settings.proxy.password')"
                  show-password-on="click"
                  size="small"
                  type="password"
                />
                <div class="row-actions">
                  <NButton size="tiny" @click="testProxy(profile)">{{ t("settings.proxy.test") }}</NButton>
                  <NButton size="tiny" @click="clearPassword(profile)">
                    {{ t("settings.proxy.clearPassword") }}
                  </NButton>
                  <NButton size="tiny" @click="removeProxy(index)">{{ t("settings.proxy.remove") }}</NButton>
                </div>
              </li>
            </ul>
            <div class="row-actions">
              <NButton size="small" @click="addProxy">{{ t("settings.proxy.add") }}</NButton>
              <NButton size="small" @click="applyProxy">{{ t("settings.proxy.apply") }}</NButton>
            </div>
            <p v-if="proxySaved" class="hint">{{ proxySaved }}</p>
          </div>

          <NFormItem :label="t('settings.proxy.testTarget')">
            <NInput v-model:value="testTarget" class="control-md" />
          </NFormItem>
          <p v-if="testMessage" class="hint">{{ testMessage }}</p>
        </fieldset>

        <fieldset>
          <legend>{{ t("settings.shortcuts.title") }}</legend>
          <p class="hint">{{ t("settings.shortcuts.hint") }}</p>
          <NFormItem
            v-for="action in SHORTCUT_ACTIONS"
            :key="action"
            :label="t(`settings.shortcuts.${action}`)"
          >
            <input
              :value="shortcuts[action]"
              class="shortcut-input"
              readonly
              @focus="shortcutSaved = ''"
              @keydown.prevent="captureShortcut(action, $event)"
            />
          </NFormItem>
          <div class="actions">
            <NButton size="small" @click="resetShortcuts">{{ t("settings.shortcuts.reset") }}</NButton>
            <span v-if="shortcutSaved" class="hint">{{ shortcutSaved }}</span>
          </div>
        </fieldset>

        <fieldset>
          <legend>{{ t("settings.logs.title") }}</legend>
          <p class="hint">{{ t("settings.logs.hint") }}</p>
          <NFormItem :label="t('settings.logs.directory')">
            <NInput
              v-model:value="store.settings.logs.directory"
              :placeholder="t('settings.logs.directoryPlaceholder')"
              class="control-lg"
            />
          </NFormItem>
          <p class="hint">{{ t("settings.logs.restartHint") }}</p>
          <NFormItem :label="t('settings.logs.maxSize')">
            <NInputNumber
              :min="1"
              :show-button="false"
              :step="1"
              :value="fromU64(store.settings.logs.max_size_mb)"
              class="control-xs"
              @update:value="(value) => applyNumber(value, setLogMaxSize)"
            />
          </NFormItem>
          <NFormItem :label="t('settings.logs.retention')">
            <NInputNumber
              :min="0"
              :show-button="false"
              :value="store.settings.logs.retention_days"
              class="control-xs"
              @update:value="(value) => applyNumber(value, setLogRetention)"
            />
          </NFormItem>
          <p v-if="paths" class="hint">{{ t("settings.logs.config") }}: {{ paths.config_path }}</p>
          <p v-if="paths" class="hint">{{ t("settings.logs.dir") }}: {{ paths.log_dir }}</p>
        </fieldset>

        <div class="actions">
          <NButton :loading="store.saving" type="primary" @click="saveSettings">
            {{ t("settings.save") }}
          </NButton>
          <NButton @click="resetSettings">{{ t("settings.resetConfig") }}</NButton>
          <span v-if="autoSaved" class="saved-tip">{{ autoSaved }}</span>
        </div>
      </NForm>
    </div>
  </section>
</template>

<style scoped>
.settings {
  box-sizing: border-box;
  height: 100%;
  overflow: hidden;
  padding: 24px 32px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  width: 100%;
  max-width: none;
  margin: 0;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  width: 100%;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.zoom-badge {
  padding: 3px 9px;
  border: 1px solid rgb(255 255 255 / 12%);
  border-radius: 999px;
  color: #9aa4b2;
  font-size: 12px;
  font-variant-numeric: tabular-nums;
}

.header-actions button {
  padding: 6px 12px;
  border: 1px solid rgb(255 255 255 / 15%);
  border-radius: 6px;
  background: rgb(255 255 255 / 8%);
  color: #e6e6e6;
  cursor: pointer;
}

.header-actions button:hover {
  background: rgb(255 255 255 / 14%);
}

.settings-layout {
  display: flex;
  gap: 32px;
  width: 100%;
  max-width: 1180px;
  margin: 0 auto;
  min-height: 0;
  flex: 1;
  overflow: hidden;
}

.settings-nav {
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  gap: 6px;
  flex: 0 0 200px;
  padding: 0 0 8px;
  overflow-y: auto;
}

.settings-nav button {
  width: 100%;
  text-align: center;
  padding: 10px 12px;
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
  color: #34d399;
}

.settings-form {
  display: flex;
  flex: 1;
  flex-direction: column;
  gap: 16px;
  min-width: 0;
  min-height: 0;
  margin: 0;
  padding-right: 4px;
  overflow-y: auto;
}

fieldset {
  border: 1px solid #2a323c;
  border-radius: 6px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 14px 18px;
  background: var(--panel-bg, rgb(0 0 0 / 35%));
}

legend {
  padding: 0 6px;
}

.settings-form :deep(.n-form-item) {
  margin-bottom: 6px;
  align-items: center;
}

.settings-form :deep(.n-form-item-label) {
  padding-right: 20px;
  color: #cbd5e1;
  font-size: 13px;
  text-align: left;
}

.settings-form :deep(.n-form-item-blank) {
  justify-content: flex-end;
}

.settings-form :deep(.n-form-item-blank) > * {
  flex: none;
}

.control-xs {
  width: 120px;
}

.control-sm {
  width: 180px;
}

.control-md {
  width: 320px;
}

.control-lg {
  width: 420px;
}

.control-wide {
  width: 100%;
}

.control-color {
  width: 64px;
}

.settings-form :deep(.control-xs) {
  width: 120px;
}

.settings-form :deep(.control-sm) {
  width: 180px;
}

.settings-form :deep(.control-md) {
  width: 320px;
  max-width: 100%;
}

.settings-form :deep(.control-lg) {
  width: 420px;
  max-width: 100%;
}

.settings-form :deep(.control-wide) {
  width: 100%;
}

.settings-form :deep(.control-color) {
  width: 64px;
}

.slider-control {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 360px;
  max-width: 100%;
}

.slider-control :deep(.n-slider) {
  flex: 1;
}

.slider-value {
  width: 2.5rem;
  color: #9aa4b2;
  font-size: 12px;
  text-align: right;
}

.file-row {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
  max-width: 420px;
}

.file-name {
  min-width: 0;
  overflow: hidden;
  color: #cbd5e1;
  font-family: ui-monospace, monospace;
  font-size: 12px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-input-hidden {
  display: none;
}

.shortcut-input {
  width: 260px;
  max-width: 100%;
  padding: 5px 8px;
  border: 1px solid #2a323c;
  border-radius: 5px;
  background: #101418;
  color: #e6e6e6;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}

.proxy-block {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
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
  grid-template-columns: 1.5rem minmax(0, 0.8fr) minmax(0, 1.2fr) 6rem minmax(0, 1fr) minmax(0, 1fr) auto;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.proxy-row :deep(.n-input),
.proxy-row :deep(.n-input-number),
.proxy-row :deep(.n-base-selection) {
  width: 100%;
}

.proxy-handle {
  padding: 0;
  border: none;
  background: transparent;
  color: #9aa4b2;
  cursor: grab;
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
  align-items: center;
  justify-content: flex-end;
  flex-wrap: wrap;
  gap: 12px;
}

.settings-form > .actions {
  padding-right: 30px;
}

.preview {
  box-sizing: border-box;
  min-height: 64px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 10px 14px;
  overflow: hidden;
  border: 1px solid #2a323c;
  border-radius: 6px;
  background-position: center;
  background-size: cover;
}

.preview-sample {
  min-width: 0;
  overflow: hidden;
  color: #fff;
  line-height: 1.2;
  text-overflow: ellipsis;
  text-shadow: 0 1px 3px rgb(0 0 0 / 85%);
  white-space: nowrap;
}

.preview-meta {
  flex: none;
  color: rgb(255 255 255 / 75%);
  font-family: ui-monospace, monospace;
  font-size: 12px;
  text-shadow: 0 1px 3px rgb(0 0 0 / 85%);
}

.hint {
  color: #9aa4b2;
  margin: 0;
  font-size: 12px;
}
</style>
