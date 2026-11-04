//! 可配置快捷键：前端持久化，默认参考常用编辑器。

export type ShortcutAction = "edit" | "create" | "delete" | "refresh" | "jump";

export type ShortcutMap = Record<ShortcutAction, string>;

export const DEFAULT_SHORTCUTS: ShortcutMap = {
  edit: "Mod+E",
  create: "Mod+N",
  delete: "Delete",
  refresh: "F5",
  jump: "Mod+K",
};

const STORAGE_KEY = "zkui.shortcuts";

export function loadShortcuts(): ShortcutMap {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return { ...DEFAULT_SHORTCUTS };
    }
    const parsed = JSON.parse(raw) as Partial<ShortcutMap>;
    return { ...DEFAULT_SHORTCUTS, ...parsed };
  } catch {
    return { ...DEFAULT_SHORTCUTS };
  }
}

export function saveShortcuts(shortcuts: ShortcutMap): void {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(shortcuts));
}

export function eventToBinding(event: KeyboardEvent): string | null {
  if (["Control", "Meta", "Shift", "Alt"].includes(event.key)) {
    return null;
  }
  const parts: string[] = [];
  if (event.ctrlKey || event.metaKey) {
    parts.push("Mod");
  }
  if (event.altKey) {
    parts.push("Alt");
  }
  if (event.shiftKey) {
    parts.push("Shift");
  }
  const key = event.key.length === 1 ? event.key.toUpperCase() : event.key;
  parts.push(key);
  return parts.join("+");
}

export function matchesShortcut(event: KeyboardEvent, binding: string): boolean {
  return eventToBinding(event) === binding;
}
