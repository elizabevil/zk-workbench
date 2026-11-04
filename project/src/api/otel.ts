//! OpenTelemetry 导出。

import {call} from "./client";
import type {OtelExportStatus} from "./types/OtelExportStatus";

export function otelExportNow(): Promise<void> {
    return call<void>("otel_export_now");
}

export function otelStatus(): Promise<OtelExportStatus> {
    return call<OtelExportStatus>("otel_status");
}
