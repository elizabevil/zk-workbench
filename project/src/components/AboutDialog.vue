<script lang="ts" setup>
import {invoke} from "@tauri-apps/api/core";
import {NButton, NModal, NTag} from "naive-ui";
import {computed, onMounted, onUnmounted, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {appInfo} from "@/api/app";
import type {AppInfo} from "@/api/types/AppInfo";
import {ABOUT_LINKS} from "@/constants/about";
import {KEY_ESCAPE} from "@/constants/keyboard";
import {useUiStore} from "@/stores/ui";

const {t} = useI18n();
const ui = useUiStore();
const info = ref<AppInfo | null>(null);
const copied = ref(false);
const linkError = ref("");
let copiedTimer = 0;

const versionLabel = computed(() => {
  if (!info.value) {
    return "";
  }
  return `${info.value.name} Version ${info.value.version}`;
});

async function load(): Promise<void> {
  if (info.value) {
    return;
  }
  try {
    info.value = await appInfo();
  } catch {
    info.value = {name: "zkui", version: "0.1.0"};
  }
}

function close(): void {
  ui.closeAbout();
}

function onShow(value: boolean): void {
  if (!value) {
    close();
  }
}

async function openLink(url: string): Promise<void> {
  if (!url) {
    return;
  }
  linkError.value = "";
  try {
    await invoke("open_external", {url});
  } catch {
    linkError.value = t("about.openFailed");
  }
}

async function copyVersion(): Promise<void> {
  if (!versionLabel.value) {
    return;
  }
  try {
    await navigator.clipboard.writeText(versionLabel.value);
    copied.value = true;
    window.clearTimeout(copiedTimer);
    copiedTimer = window.setTimeout(() => {
      copied.value = false;
    }, 1600);
  } catch {
    // 剪贴板不可用时保持弹框原样。
  }
}

function onKeydown(event: KeyboardEvent): void {
  if (ui.aboutOpen && event.key === KEY_ESCAPE) {
    event.preventDefault();
    event.stopImmediatePropagation();
    close();
  }
}

watch(
  () => ui.aboutOpen,
  (open) => {
    if (open) {
      copied.value = false;
      linkError.value = "";
      void load();
    }
  },
  {immediate: true},
);

onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => {
  window.removeEventListener("keydown", onKeydown);
  window.clearTimeout(copiedTimer);
});
</script>

<template>
  <NModal
    :close-on-esc="false"
    :show="ui.aboutOpen"
    :style="{width: '460px'}"
    :title="t('shell.about')"
    preset="card"
    @update:show="onShow"
  >
    <div class="content">
      <div aria-hidden="true" class="logo">ZK</div>
      <h2>ZK Workbench</h2>
      <p class="version">{{ versionLabel || "zkui Version 0.1.0" }}</p>
      <p class="description">{{ t("about.description") }}</p>

      <section class="block">
        <h3>{{ t("about.techStack") }}</h3>
        <div class="chips">
          <NTag size="small">Tauri 2</NTag>
          <NTag size="small">Vue 3</NTag>
          <NTag size="small">Rust</NTag>
        </div>
      </section>

      <section class="links">
        <NButton
          v-for="item in ABOUT_LINKS"
          :key="item.key"
          :disabled="!item.url"
          :title="item.url || t('about.comingSoon')"
          size="tiny"
          @click="openLink(item.url)"
        >
          {{ t(`about.${item.key}`) }}
        </NButton>
      </section>

      <p v-if="linkError" class="link-error">{{ linkError }}</p>

      <p class="developer">{{ t("about.developedBy", {name: "Debi"}) }}</p>
      <p class="legal">© 2026 ZK Workbench · License</p>
    </div>

    <template #footer>
      <div class="actions">
        <NButton @click="copyVersion">
          {{ copied ? t("about.copied") : t("about.copyVersion") }}
        </NButton>
        <NButton type="primary" @click="close">{{ t("about.close") }}</NButton>
      </div>
    </template>
  </NModal>
</template>

<style scoped>
.content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
}

.logo {
  width: 58px;
  height: 58px;
  display: grid;
  place-items: center;
  border-radius: 16px;
  background: linear-gradient(135deg, #2563eb, #7c3aed);
  color: #fff;
  font-size: 20px;
  font-weight: 700;
  letter-spacing: 0.5px;
}

h2 {
  margin: 2px 0 0;
  font-size: 20px;
}

.version {
  margin: 0;
  color: #9aa4b2;
  font-family: ui-monospace, monospace;
  font-size: 12px;
}

.description {
  margin: 2px 0 4px;
  color: #cbd5e1;
  font-size: 13px;
  text-align: center;
}

.block {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  padding: 10px 0;
  border-top: 1px solid rgb(255 255 255 / 8%);
}

.block h3 {
  margin: 0;
  color: #9aa4b2;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.8px;
}

.chips {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 6px;
}

.links {
  width: 100%;
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 6px;
  padding: 10px 0;
  border-top: 1px solid rgb(255 255 255 / 8%);
}

.link-error {
  margin: 0;
  color: #f87171;
  font-size: 11px;
}

.developer,
.legal {
  margin: 0;
  color: #9aa4b2;
  font-size: 11px;
}

.legal {
  color: #6b7280;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
