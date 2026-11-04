//! 节点读取。

import { call } from "./client";
import type { NodeStat } from "./types/NodeStat";

export function nodeChildren(connectionId: string, path: string): Promise<string[]> {
  return call<string[]>("node_children", { connectionId, path });
}

export function nodeStat(connectionId: string, path: string): Promise<NodeStat> {
  return call<NodeStat>("node_stat", { connectionId, path });
}
