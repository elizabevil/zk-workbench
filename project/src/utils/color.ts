export interface RgbColor {
  r: number;
  g: number;
  b: number;
}

const HEX_COLOR = /^#?([0-9a-f]{3}|[0-9a-f]{6})$/i;

export function parseHexColor(value: string): RgbColor | null {
  const match = HEX_COLOR.exec(value.trim());
  if (!match) {
    return null;
  }
  const hex = match[1];
  const full = hex.length === 3
    ? hex.split("").map((part) => `${part}${part}`).join("")
    : hex;
  return {
    r: Number.parseInt(full.slice(0, 2), 16),
    g: Number.parseInt(full.slice(2, 4), 16),
    b: Number.parseInt(full.slice(4, 6), 16),
  };
}

export function rgbaColor(value: string, opacity: number): string {
  const color = parseHexColor(value) ?? {r: 230, g: 230, b: 230};
  const alpha = Math.min(1, Math.max(0, opacity));
  return `rgba(${color.r}, ${color.g}, ${color.b}, ${alpha})`;
}
