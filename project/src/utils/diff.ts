//! 行级 diff：用于保存前的变更预览。

export type DiffKind = "same" | "add" | "remove";

export interface DiffLine {
  kind: DiffKind;
  text: string;
}

/** 基于 LCS 的行 diff；输入超过 2000 行时退化为整段替换。 */
export function diffLines(before: string, after: string): DiffLine[] {
  const oldLines = before.split("\n");
  const newLines = after.split("\n");
  if (oldLines.length > 2000 || newLines.length > 2000) {
    return [
      ...oldLines.map((text) => ({ kind: "remove" as const, text })),
      ...newLines.map((text) => ({ kind: "add" as const, text })),
    ];
  }

  const rows = oldLines.length + 1;
  const cols = newLines.length + 1;
  const lcs: number[][] = Array.from({ length: rows }, () => new Array<number>(cols).fill(0));
  for (let i = 1; i < rows; i += 1) {
    for (let j = 1; j < cols; j += 1) {
      lcs[i][j] =
        oldLines[i - 1] === newLines[j - 1]
          ? lcs[i - 1][j - 1] + 1
          : Math.max(lcs[i - 1][j], lcs[i][j - 1]);
    }
  }

  const result: DiffLine[] = [];
  let i = oldLines.length;
  let j = newLines.length;
  while (i > 0 || j > 0) {
    if (i > 0 && j > 0 && oldLines[i - 1] === newLines[j - 1]) {
      result.push({ kind: "same", text: oldLines[i - 1] });
      i -= 1;
      j -= 1;
    } else if (j > 0 && (i === 0 || lcs[i][j - 1] >= lcs[i - 1][j])) {
      result.push({ kind: "add", text: newLines[j - 1] });
      j -= 1;
    } else {
      result.push({ kind: "remove", text: oldLines[i - 1] });
      i -= 1;
    }
  }
  return result.reverse();
}
