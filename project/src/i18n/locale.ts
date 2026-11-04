//! 语言常量。类型来自 ts-rs 生成的 Locale，运行时候选集中在此。

import type { Locale } from "../api/types/Locale";

export const LOCALES: readonly Locale[] = ["zh-CN", "en-US"];

export const DEFAULT_LOCALE: Locale = "zh-CN";

export function normalizeLocale(value: string): Locale {
  return value === "en-US" ? "en-US" : "zh-CN";
}
