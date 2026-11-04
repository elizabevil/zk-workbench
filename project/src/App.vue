<script lang="ts" setup>
import {listen} from "@tauri-apps/api/event";
import {getCurrentWebview} from "@tauri-apps/api/webview";
import {NConfigProvider, darkTheme, enUS, zhCN} from "naive-ui";
import {computed, onMounted, onUnmounted, ref, watch} from "vue";
import {useRouter} from "vue-router";

import {backgroundImageLoad} from "@/api/appearance";
import AboutDialog from "@/components/AboutDialog.vue";
import {listenConnectionState, MENU_ACTION, MENU_ACTION_SETTINGS} from "@/api/events";
import {useConnectionStore} from "@/stores/connection";
import {useSettingsStore} from "@/stores/settings";
import {useUiStore} from "@/stores/ui";
import {createNaiveThemeOverrides} from "@/theme/naive";
import {rgbaColor} from "@/utils/color";
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
    ui.resetZoom();
  }
}

watch(
  () => ui.zoomResetRevision,
  () => {
    const fontSize = settings.settings?.appearance.font_size ?? BASE_FONT_SIZE;
    uiZoom.value = BASE_FONT_SIZE / fontSize;
    saveZoom(uiZoom.value);
  },
);

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
  const appearance = settings.settings?.appearance;
  const opacity = (appearance?.panel_opacity ?? 50) / 100;
  const fontOpacity = (appearance?.font_opacity ?? 100) / 100;
  const fontColor = appearance?.font_color ?? "#e6e6e6";
  const workspaceScale = (appearance?.workspace_font_size ?? 14) / 14;
  return {
    "--panel-bg": `rgb(0 0 0 / ${opacity})`,
    "--panel-hover-bg": `rgb(0 0 0 / ${Math.min(0.9, opacity + 0.12)})`,
    "--app-text": rgbaColor(fontColor, fontOpacity),
    "--app-text-soft": rgbaColor(fontColor, fontOpacity * 0.85),
    "--app-text-muted": rgbaColor(fontColor, fontOpacity * 0.65),
    "--app-text-dim": rgbaColor(fontColor, fontOpacity * 0.45),
    "--workspace-font-scale": String(workspaceScale),
  };
});

const naiveLocale = computed(() =>
  settings.settings?.general.locale === "en-US" ? enUS : zhCN,
);

// 弹框会 Teleport 到 body，CSS 变量需要同步到根节点才能继承。
watch(
  appStyle,
  (style) => {
    for (const [name, value] of Object.entries(style)) {
      document.documentElement.style.setProperty(name, String(value));
    }
  },
  {immediate: true},
);

const naiveThemeOverrides = computed(() => {
  const appearance = settings.settings?.appearance;
  return createNaiveThemeOverrides(
    appearance?.font_color ?? "#e6e6e6",
    (appearance?.font_opacity ?? 100) / 100,
  );
});

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
      <AboutDialog/>
    </NConfigProvider>
  </main>
</template>
