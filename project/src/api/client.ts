//! IPC 调用封装：只做两件事——调用命令、把失败收敛为 AppError。
//!
//! 失败值在边界上只有两种非契约形态：字符串或 Error；其余按 AppError 契约处理。
//! 形状校验放在后端与类型生成层，前端不在运行时逐字段判断。

import { invoke } from "@tauri-apps/api/core";

import type { AppError } from "./types/AppError";
import type { ErrorFields } from "./types/ErrorFields";

/** invoke 的失败值：后端 AppError，或 Tauri 自身的字符串/异常。 */
export type InvokeRejection = AppError | string | Error;

const EMPTY_FIELDS: ErrorFields = {
  path: "",
  version: -1,
  connection_id: "",
  generation: 0,
  detail: "",
};

function ipcError(messageKey: string, detail: string): AppError {
  return {
    kind: "unknown",
    scope: "ipc",
    code: 3000,
    message_key: messageKey,
    fields: { ...EMPTY_FIELDS, detail },
  };
}

/** 把失败值收敛为 AppError；非契约形态只保留 detail。 */
export function toAppError(raw: InvokeRejection): AppError {
  if (typeof raw === "string") {
    return ipcError("error.ipc.unknown", raw);
  }
  if (raw instanceof Error) {
    return ipcError("error.ipc.unknown", raw.message);
  }
  return raw;
}

const NOT_TAURI = ipcError("error.ipc.not_tauri", "window.__TAURI_INTERNALS__ is missing");

export async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) {
    throw NOT_TAURI;
  }
  try {
    return await invoke<T>(command, args);
  } catch (raw) {
    // Tauri 只会以 AppError、字符串或 Error 拒绝；此处做一次边界断言。
    throw toAppError(raw as InvokeRejection);
  }
}
