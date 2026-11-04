//! 审计查询与维护。

import { call } from "./client";
import type { AuditRecord } from "./types/AuditRecord";
import type { AuditVerifyReport } from "./types/AuditVerifyReport";

export function auditList(
  limit: number,
  offset: number,
  pathFilter?: string | null,
): Promise<AuditRecord[]> {
  return call<AuditRecord[]>("audit_list", { limit, offset, pathFilter: pathFilter ?? null });
}

export function auditVerify(): Promise<AuditVerifyReport> {
  return call<AuditVerifyReport>("audit_verify");
}

export function auditClear(): Promise<number> {
  return call<number>("audit_clear");
}

export function auditPrune(retentionDays: number): Promise<number> {
  return call<number>("audit_prune", { retentionDays });
}

export function auditExport(target: string): Promise<void> {
  return call<void>("audit_export", { target });
}
