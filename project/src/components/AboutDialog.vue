<script lang="ts" setup>
import {computed, onMounted, onUnmounted, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {appInfo} from "@/api/app";
import type {AppInfo} from "@/api/types/AppInfo";

const props = defineProps<{ open: boolean }>();
const emit = defineEmits<{ close: [] }>();

const {t} = useI18n();
const info = ref<AppInfo | null>(null);
const copied = ref(false);
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
  emit("close");
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
  if (props.open && event.key === "Escape") {
    event.preventDefault();
    event.stopImmediatePropagation();
    close();
  }
}

watch(
    () => props.open,
    (open) => {
      if (open) {
        copied.value = false;
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
  <Teleport to="body">
    <div v-if="open" class="overlay" @mousedown.self="close">
      <section
          aria-labelledby="about-title"
          aria-modal="true"
          class="dialog"
          role="dialog"
      >
        <div aria-hidden="true" class="logo">ZK</div>
        <h2 id="about-title">ZK Workbench</h2>
        <p class="version">{{ versionLabel || "zkui Version 0.1.0" }}</p>
        <p class="description">{{ t("about.description") }}</p>

        <section class="block">
          <h3>{{ t("about.techStack") }}</h3>
          <div class="chips">
            <span>Tauri 2</span>
            <span>Vue 3</span>
            <span>Rust</span>
          </div>
        </section>

        <section class="links">
          <button
              v-for="item in ['website', 'github', 'releaseNotes', 'support']"
              :key="item"
              :title="t('about.comingSoon')"
              class="placeholder"
              disabled
              type="button"
          >
            {{ t(`about.${item}`) }}
          </button>
        </section>

        <p class="developer">{{ t("about.developedBy", {name: "Debi"}) }}</p>
        <p class="legal">© 2026 ZK Workbench · License</p>

        <footer class="actions">
          <button type="button" @click="copyVersion">
            {{ copied ? t("about.copied") : t("about.copyVersion") }}
          </button>
          <button class="primary" type="button" @click="close">
            {{ t("about.close") }}
          </button>
        </footer>
      </section>
    </div>
  </Teleport>
</template>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  display: grid;
  place-items: center;
  background: rgb(0 0 0 / 55%);
  z-index: 100;
}

.dialog {
  width: min(460px, 92vw);
  max-height: 88vh;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 24px 26px 20px;
  border: 1px solid #2a323c;
  border-radius: 12px;
  background: #171c22;
  box-shadow: 0 18px 50px rgb(0 0 0 / 45%);
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

.chips span {
  padding: 3px 9px;
  border: 1px solid rgb(255 255 255 / 12%);
  border-radius: 999px;
  background: rgb(255 255 255 / 5%);
  color: #cbd5e1;
  font-size: 12px;
}

.links {
  width: 100%;
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 6px;
  padding: 10px 0;
  border-top: 1px solid rgb(255 255 255 / 8%);
}

.placeholder {
  padding: 6px 4px;
  border: 1px dashed rgb(255 255 255 / 12%);
  border-radius: 6px;
  background: transparent;
  color: #6b7280;
  cursor: not-allowed;
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
  width: 100%;
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 8px;
}

button {
  padding: 7px 14px;
  border: 1px solid #2a323c;
  border-radius: 6px;
  background: #1d242c;
  color: inherit;
  cursor: pointer;
}

button:hover:not(:disabled) {
  background: #252d36;
}

.primary {
  border-color: #2563eb;
  background: #2563eb;
}

.primary:hover {
  background: #1d4ed8;
}
</style>
