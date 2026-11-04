//! 应用路径等通用信息。

import { call } from "./client";
import type { AppPaths } from "./types/AppPaths";

export function appPaths(): Promise<AppPaths> {
  return call<AppPaths>("app_paths");
}
