//! 节点读写与删除。

import { call } from "./client";
import type { DeleteOutcome } from "./types/DeleteOutcome";
import type { DeletePreview } from "./types/DeletePreview";
import type { NodeCreateMode } from "./types/NodeCreateMode";
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

export function nodeSet(
  connectionId: string,
  path: string,
  dataB64: string,
  expectedVersion: number,
): Promise<NodeStat> {
  return call<NodeStat>("node_set", { connectionId, path, dataB64, expectedVersion });
}

export function nodeCreate(
  connectionId: string,
  path: string,
  dataB64: string,
  mode: NodeCreateMode,
  createParents: boolean,
): Promise<string> {
  return call<string>("node_create", { connectionId, path, dataB64, mode, createParents });
}

export function nodeDeletePreview(
  connectionId: string,
  path: string,
  recursive: boolean,
): Promise<DeletePreview> {
  return call<DeletePreview>("node_delete_preview", { connectionId, path, recursive });
}

export function nodeDelete(
  connectionId: string,
  path: string,
  recursive: boolean,
): Promise<DeleteOutcome> {
  return call<DeleteOutcome>("node_delete", { connectionId, path, recursive });
}
