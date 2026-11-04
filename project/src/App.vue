<script lang="ts" setup>
import {listen} from "@tauri-apps/api/event";
import {getCurrentWebview} from "@tauri-apps/api/webview";
import {NConfigProvider, darkTheme, enUS, zhCN} from "naive-ui";
import {computed, onMounted, onUnmounted, ref, watch} from "vue";
import {useRouter} from "vue-router";

import {backgroundImageLoad} from "@/api/appearance";
import {listenConnectionState, MENU_ACTION, MENU_ACTION_SETTINGS} from "@/api/events";
import {useConnectionStore} from "@/stores/connection";
import {useSettingsStore} from "@/stores/settings";
import {useUiStore} from "@/stores/ui";
import {naiveThemeOverrides} from "@/theme/naive";
import {BASE_FONT_SIZE, effectiveZoom, loadZoom, nextZoom, saveZoom} from "@/utils/zoom";

const router = useRouter();
const imageData = ref("");
const settings = useSettingsStore();
const connections = useConnectionStore();
const ui = useUiStore();
let unlisten: (() => void) | undefined;
let unlistenMenu: (() => void) | undefined;
const ready = ref(false);
const uiZoom = ref(loadZoom());

async function applyZoom(): Promise<void> {
  const fontSize = settings.settings?.appearance.font_size ?? BASE_FONT_SIZE;
  const scale = effectiveZoom(fontSize, uiZoom.value);
  ui.setZoomPercent(scale * 100);
  if ("__TAURI_INTERNALS__" in window) {
    try {
      await getCurrentWebview().setZoom(scale);
      document.documentElement.style.removeProperty("zoom");
      return;
    } catch {
      // 权限或平台不支持时退回 CSS 缩放。
    }
  }
  document.documentElement.style.setProperty("zoom", String(scale));
}

function onWheel(event: WheelEvent): void {
  if (!event.ctrlKey && !event.metaKey) {
    return;
  }
  event.preventDefault();
  uiZoom.value = nextZoom(uiZoom.value, event.deltaY);
  saveZoom(uiZoom.value);
}

function onZoomKey(event: KeyboardEvent): void {
  if ((event.ctrlKey || event.metaKey) && event.key === "0") {
    event.preventDefault();
    uiZoom.value = 1;
    saveZoom(1);
  }
}

onMounted(async () => {
  settings.load();
  ready.value = true;
  void applyZoom();
  window.addEventListener("wheel", onWheel, {passive: false});
  window.addEventListener("keydown", onZoomKey);
  unlisten = await listenConnectionState((event) => connections.applyEvent(event));
  unlistenMenu = await listen<string>(MENU_ACTION, (event) => {
    if (event.payload === MENU_ACTION_SETTINGS) {
      void router.push("/settings");
    }
  });
});

onUnmounted(() => {
  unlisten?.();
  unlistenMenu?.();
  window.removeEventListener("wheel", onWheel);
  window.removeEventListener("keydown", onZoomKey);
});

watch(
    [() => settings.settings?.appearance.font_size, uiZoom],
    () => {
      if (ready.value) {
        void applyZoom();
      }
    },
);

const background = computed(() => {
  const appearance = settings.settings?.appearance;
  if (!appearance) {
    return "linear-gradient(135deg, #7c3aed, #2563eb)";
  }
  if (appearance.mode === "solid") {
    return appearance.color;
  }
  if (appearance.mode === "image" && imageData.value) {
    return `linear-gradient(rgb(0 0 0 / 25%), rgb(0 0 0 / 25%)), url("${imageData.value}") center / cover no-repeat`;
  }
  return `linear-gradient(${appearance.gradient_angle}deg, ${appearance.gradient_from}, ${appearance.gradient_to})`;
});

const appStyle = computed(() => {
  const opacity = (settings.settings?.appearance.panel_opacity ?? 50) / 100;
  return {
    "--panel-bg": `rgb(0 0 0 / ${opacity})`,
    "--panel-hover-bg": `rgb(0 0 0 / ${Math.min(0.9, opacity + 0.12)})`,
  };
});

const naiveLocale = computed(() =>
  settings.settings?.general.locale === "en-US" ? enUS : zhCN,
);

const backgroundStyle = computed(() => {
  const blur = settings.settings?.appearance.background_blur ?? 0;
  return {
    background: background.value,
    filter: blur > 0 ? `blur(${blur}px)` : "none",
    transform: blur > 0 ? "scale(1.06)" : "none",
  };
});

watch(
    () => [settings.settings?.appearance.mode, settings.settings?.appearance.image_path],
    async ([mode, path]) => {
      if (mode === "image" && path) {
        try {
          imageData.value = await backgroundImageLoad(path);
        } catch {
          imageData.value = "";
        }
      } else {
        imageData.value = "";
      }
    },
    {immediate: true},
);
</script>

<template>
  <main :style="appStyle" class="app">
    <div :style="backgroundStyle" class="app-bg"/>
    <NConfigProvider abstract :locale="naiveLocale" :theme="darkTheme" :theme-overrides="naiveThemeOverrides">
      <router-view/>
    </NConfigProvider>
  </main>
</template>
