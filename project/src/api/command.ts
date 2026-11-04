//! 命令区。

import { call } from "./client";

export function commandPreview(line: string): Promise<string | null> {
  return call<string | null>("command_preview", { line });
}

export function commandExecute(connectionId: string, line: string): Promise<string> {
  return call<string>("command_execute", { connectionId, line });
}
