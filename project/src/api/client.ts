//! IPC 调用基础封装：统一错误形状，前端不解析后端的原始文案。

import { invoke } from "@tauri-apps/api/core";
import type { AppError } from "./types/AppError";

function isAppError(value: unknown): value is AppError {
  return (
    typeof value === "object" &&
    value !== null &&
    "kind" in value &&
    "message_key" in value &&
    "scope" in value
  );
}

export function normalizeError(raw: unknown): AppError {
  if (isAppError(raw)) {
    return raw;
  }
  return {
    kind: "ipc_invalid_argument",
    scope: "ipc",
    code: 3001,
    message_key: "error.ipc.invalid_argument",
    fields: {
      path: null,
      version: null,
      connection_id: null,
      generation: null,
      detail: typeof raw === "string" ? raw : JSON.stringify(raw),
    },
  };
}

export async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (raw) {
    throw normalizeError(raw);
  }
}
