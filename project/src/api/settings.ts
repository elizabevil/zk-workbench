//! 设置方向命令。

import {call} from "./client";
import type {Settings} from "./types/Settings";

export function settingsGet(): Promise<Settings> {
    return call<Settings>("settings_get");
}

export function settingsSave(settings: Settings): Promise<Settings> {
    return call<Settings>("settings_save", {settings});
}

export function settingsReset(): Promise<Settings> {
    return call<Settings>("settings_reset");
}
