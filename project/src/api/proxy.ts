//! 代理连通性测试与密码管理。

import { call } from "./client";
import type { ProxyProfile } from "./types/ProxyProfile";

export function proxyTest(profile: ProxyProfile, target: string): Promise<void> {
  return call<void>("proxy_test", { profile, target });
}

export function proxyPasswordSet(profile: string, password: string): Promise<void> {
  return call<void>("proxy_password_set", { profile, password });
}

export function proxyPasswordClear(profile: string): Promise<void> {
  return call<void>("proxy_password_clear", { profile });
}

export function proxyPasswordHas(profile: string): Promise<boolean> {
  return call<boolean>("proxy_password_has", { profile });
}
