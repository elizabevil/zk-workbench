<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import { useRouter } from "vue-router";

import { appInfo } from "@/api/app";
import { backgroundImageLoad } from "@/api/appearance";
import { listenConnectionState, MENU_ACTION, MENU_ACTION_ABOUT, MENU_ACTION_SETTINGS } from "@/api/events";
import { useConnectionStore } from "@/stores/connection";
import { useSettingsStore } from "@/stores/settings";

const router = useRouter();
const info = ref<{ name: string; version: string } | null>(null);
const imageData = ref("");
const settings = useSettingsStore();
const connections = useConnectionStore();
let unlisten: (() => void) | undefined;
let unlistenMenu: (() => void) | undefined;

onMounted(async () => {
  settings.load();
  unlisten = await listenConnectionState((event) => connections.applyEvent(event));
  info.value = await appInfo();
  unlistenMenu = await listen<string>(MENU_ACTION, (event) => {
    if (event.payload === MENU_ACTION_SETTINGS) {
      void router.push("/settings");
    } else if (event.payload === MENU_ACTION_ABOUT) {
      const name = info.value?.name ?? "";
      const version = info.value?.version ?? "";
      alert(`${name} ${version}`.trim());
    }
  });
});

onUnmounted(() => {
  unlisten?.();
  unlistenMenu?.();
});

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
  const fontSize = settings.settings?.appearance.font_size ?? 14;
  return {
    "--panel-bg": `rgb(0 0 0 / ${opacity})`,
    "--panel-hover-bg": `rgb(0 0 0 / ${Math.min(0.9, opacity + 0.12)})`,
    zoom: String(fontSize / 14),
  };
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
  { immediate: true },
);
</script>

<template>
  <main class="app" :style="appStyle">
    <div class="app-bg" :style="backgroundStyle" />
    <router-view />
  </main>
</template>
