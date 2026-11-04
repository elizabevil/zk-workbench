import { createI18n } from "vue-i18n";

import enUS from "./en-US";
import { DEFAULT_LOCALE } from "./locale";
import zhCN from "./zh-CN";

export type Locale = "zh-CN" | "en-US";

export const i18n = createI18n({
  legacy: false,
  locale: DEFAULT_LOCALE,
  fallbackLocale: "en-US",
  messages: { "zh-CN": zhCN, "en-US": enUS },
});
