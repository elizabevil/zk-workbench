/** 过滤 Naive UI 数值输入可能产生的 null，业务 setter 只接收有效数字。 */
export function applyNumber(value: unknown, setter: (value: number) => void): void {
  if (typeof value === "number") {
    setter(value);
  }
}
