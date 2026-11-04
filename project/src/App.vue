<script setup lang="ts">
import { computed, onMounted, onUnmounted } from "vue";

import { listenConnectionState } from "@/api/events";
import { useConnectionStore } from "@/stores/connection";
import { useSettingsStore } from "@/stores/settings";

const settings = useSettingsStore();
const connections = useConnectionStore();
let unlisten: (() => void) | undefined;

onMounted(async () => {
  settings.load();
  unlisten = await listenConnectionState((event) => connections.applyEvent(event));
});

onUnmounted(() => unlisten?.());

const background = computed(
  () => settings.settings?.appearance.background ?? "linear-gradient(135deg, #7c3aed, #2563eb)",
);
</script>

<template>
  <main class="app" :style="{ background }">
    <router-view />
  </main>
</template>
