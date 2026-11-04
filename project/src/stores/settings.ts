//! 设置状态。

import { defineStore } from "pinia";
import { ref } from "vue";

import { settingsGet, settingsReset, settingsSave } from "../api/settings";
import type { AppError } from "../api/types/AppError";
import type { Settings } from "../api/types/Settings";
import { i18n } from "../i18n";
import { normalizeLocale } from "../i18n/locale";

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<Settings | null>(null);
  const error = ref<AppError | null>(null);
  const saving = ref(false);

  function applyLocale(settings: Settings): void {
    i18n.global.locale.value = normalizeLocale(settings.general.locale);
  }

  async function load(): Promise<void> {
    try {
      settings.value = await settingsGet();
      applyLocale(settings.value);
    } catch (raw) {
      error.value = raw as AppError;
    }
  }

  async function save(): Promise<void> {
    if (!settings.value) {
      return;
    }
    saving.value = true;
    try {
      settings.value = await settingsSave(settings.value);
      applyLocale(settings.value);
    } catch (raw) {
      error.value = raw as AppError;
    } finally {
      saving.value = false;
    }
  }

  async function reset(): Promise<void> {
    try {
      settings.value = await settingsReset();
      applyLocale(settings.value);
    } catch (raw) {
      error.value = raw as AppError;
    }
  }

  return { settings, error, saving, load, save, reset };
});
