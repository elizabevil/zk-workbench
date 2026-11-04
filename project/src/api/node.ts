//! 节点读取。

import { call } from "./client";
import type { NodeData } from "./types/NodeData";
import type { NodeStat } from "./types/NodeStat";

export function nodeChildren(connectionId: string, path: string): Promise<string[]> {
  return call<string[]>("node_children", { connectionId, path });
}

export function nodeStat(connectionId: string, path: string): Promise<NodeStat> {
  return call<NodeStat>("node_stat", { connectionId, path });
}

export function nodeData(connectionId: string, path: string): Promise<NodeData> {
  return call<NodeData>("node_data", { connectionId, path });
}
