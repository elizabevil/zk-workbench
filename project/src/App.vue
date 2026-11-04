<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useRouter } from "vue-router";

import { appInfo } from "@/api/app";
import { listenConnectionState, MENU_ACTION, MENU_ACTION_ABOUT, MENU_ACTION_SETTINGS } from "@/api/events";
import { useConnectionStore } from "@/stores/connection";
import { useSettingsStore } from "@/stores/settings";

const router = useRouter();
const info = ref<{ name: string; version: string } | null>(null);
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

const background = computed(
  () => settings.settings?.appearance.background ?? "linear-gradient(135deg, #7c3aed, #2563eb)",
);
</script>

<template>
  <main class="app" :style="{ background }">
    <router-view />
  </main>
</template>
