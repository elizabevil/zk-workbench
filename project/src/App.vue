<script setup lang="ts">
import { onMounted } from "vue";

import { useSettingsStore } from "./stores/settings";

const settings = useSettingsStore();
onMounted(() => settings.load());
</script>

<template>
  <main class="app">
    <router-view />
  </main>
</template>
