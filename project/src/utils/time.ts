//! 时间格式化：统一使用 RFC3339（UTC）。

export function toRfc3339(value: number | null | undefined): string {
  if (value === null || value === undefined) {
    return "";
  }
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? "" : date.toISOString();
}
