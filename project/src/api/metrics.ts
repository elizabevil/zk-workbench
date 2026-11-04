//! 进程内指标。

import { call } from "./client";
import type { MetricsSnapshot } from "./types/MetricsSnapshot";

export function metricsSnapshot(): Promise<MetricsSnapshot> {
  return call<MetricsSnapshot>("metrics_snapshot");
}
