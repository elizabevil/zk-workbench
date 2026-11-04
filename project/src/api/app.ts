//! 应用路径等通用信息。

import {call} from "./client";
import type {AppInfo} from "./types/AppInfo";
import type {AppPaths} from "./types/AppPaths";

export function appPaths(): Promise<AppPaths> {
    return call<AppPaths>("app_paths");
}

export function appInfo(): Promise<AppInfo> {
    return call<AppInfo>("app_info");
}
