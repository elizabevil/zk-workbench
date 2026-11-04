//! 连接档案命令。

import { call } from "./client";
import type { ConnectionProfile } from "./types/ConnectionProfile";
import type { ProfileInput } from "./types/ProfileInput";

export function profileList(): Promise<ConnectionProfile[]> {
  return call<ConnectionProfile[]>("profile_list");
}

export function profileSave(input: ProfileInput): Promise<ConnectionProfile> {
  return call<ConnectionProfile>("profile_save", { input });
}

export function profileDelete(id: string): Promise<void> {
  return call<void>("profile_delete", { id });
}
