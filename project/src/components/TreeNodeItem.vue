<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";

import { type InvokeRejection, toAppError } from "@/api/client";
import { nodeChildren } from "@/api/node";
import type { AppError } from "@/api/types/AppError";

const props = defineProps<{
  connectionId: string;
  name: string;
  path: string;
  depth: number;
  prefix: string;
  isLast: boolean;
  selectedPath: string | null;
  expandedPaths: Set<string>;
}>();

const { t } = useI18n();
const localError = ref(false);
const emit = defineEmits<{
  select: [path: string];
  dblclick: [path: string];
  toggle: [path: string];
  create: [path: string];
  edit: [path: string];
  delete: [path: string];
  error: [error: AppError];
}>();

const loading = ref(false);
const children = ref<string[] | null>(null);
const expanded = computed(() => props.expandedPaths.has(props.path));

async function load(): Promise<void> {
  if (loading.value || children.value !== null) {
    return;
  }
  loading.value = true;
  localError.value = false;
  try {
    children.value = await nodeChildren(props.connectionId, props.path);
  } catch (raw) {
    localError.value = true;
    emit("error", toAppError(raw as InvokeRejection));
  } finally {
    loading.value = false;
  }
}

watch(
  expanded,
  (open) => {
    if (open) {
      void load();
    }
  },
  { immediate: true },
);

function toggle(): void {
  emit("toggle", props.path);
}

function childPath(name: string): string {
  return props.path === "/" ? `/${name}` : `${props.path}/${name}`;
}
</script>

<template>
  <div class="node">
    <div
      class="row"
      :class="{ selected: path === selectedPath }"
      role="treeitem"
      tabindex="0"
      :aria-selected="path === selectedPath"
      @click="emit('select', path)"
      @keydown.enter.prevent="emit('select', path)"
      @keydown.space.prevent="emit('select', path)"
    >
      <span v-if="depth === 0" class="prefix mono" />
      <span v-else class="prefix mono">{{ prefix }}{{ isLast ? "└─" : "├─" }}</span>
      <button
        class="twist"
        type="button"
        :aria-expanded="expanded"
        @click.stop="toggle"
      >
        {{ expanded ? "▾" : "▸" }}
      </button>
      <button
        class="label"
        type="button"
        @click="emit('select', path)"
        @dblclick="emit('dblclick', path)"
      >
        {{ name }}
      </button>
      <span v-if="loading" class="muted">…</span>
      <button v-else-if="localError" class="retry" type="button" @click="load">
        {{ t("shell.retry") }}
      </button>
      <div class="row-actions">
        <button
          type="button"
          class="action action-top"
          :title="t('shell.newChild')"
          @click.stop="emit('create', path)"
        >
          +
        </button>
        <button
          v-if="path !== '/'"
          type="button"
          class="action action-right"
          :title="t('shell.editNode')"
          @click.stop="emit('edit', path)"
        >
          ✎
        </button>
        <button
          v-if="path !== '/'"
          type="button"
          class="action action-bottom"
          :title="t('shell.deleteNode')"
          @click.stop="emit('delete', path)"
        >
          ×
        </button>
      </div>
    </div>

    <div v-if="expanded">
      <TreeNodeItem
        v-for="(child, index) in children ?? []"
        :key="child"
        :connection-id="connectionId"
        :name="child"
        :path="childPath(child)"
        :depth="depth + 1"
        :prefix="prefix + (isLast ? '   ' : '│  ')"
        :is-last="index === (children?.length ?? 0) - 1"
        :selected-path="selectedPath"
        :expanded-paths="expandedPaths"
        @select="emit('select', $event)"
        @dblclick="emit('dblclick', $event)"
        @toggle="emit('toggle', $event)"
        @create="emit('create', $event)"
        @edit="emit('edit', $event)"
        @delete="emit('delete', $event)"
        @error="emit('error', $event)"
      />
      <div
        v-if="children && children.length === 0"
        class="muted empty"
      >
        {{ prefix }}{{ isLast ? "   " : "│  " }}└─ —
      </div>
    </div>
  </div>
</template>

<style scoped>
.node {
  font-size: 12px;
}
.row {
  position: relative;
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 1px 4px;
  border-radius: 6px;
  cursor: pointer;
  white-space: nowrap;
}
.row:hover {
  background: rgb(255 255 255 / 5%);
}
.row.selected {
  background: rgb(34 211 238 / 8%);
  outline: 1px solid #22d3ee;
  outline-offset: -1px;
}
.prefix {
  color: #303845;
  font-family: ui-monospace, monospace;
  font-size: 11px;
  user-select: none;
}
.twist {
  padding: 0 2px;
  border: none;
  background: transparent;
  color: #9aa4b2;
  cursor: pointer;
  font-size: 10px;
}
.label {
  padding: 1px 2px;
  border: none;
  background: transparent;
  color: #d7dee8;
  cursor: pointer;
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
}
.row.selected .label {
  color: #fff;
  font-weight: 600;
}
.muted {
  color: #9aa4b2;
  font-size: 11px;
}
.retry {
  border: none;
  background: transparent;
  color: #f87171;
  cursor: pointer;
  font-size: 11px;
}
.empty {
  padding-left: 1.4rem;
}
.row-actions {
  position: absolute;
  right: 4px;
  top: 50%;
  display: inline-flex;
  gap: 2px;
  padding: 1px 3px;
  border-radius: 999px;
  background: rgb(0 0 0 / 45%);
  opacity: 0;
  visibility: hidden;
  pointer-events: none;
  transform: translateY(-50%) scale(0.85);
  transition:
    opacity 100ms ease,
    transform 100ms ease;
}
.row:hover .row-actions,
.row:focus-within .row-actions {
  opacity: 1;
  visibility: visible;
  pointer-events: auto;
  transform: translateY(-50%) scale(1);
}
.action {
  width: 1rem;
  height: 1rem;
  padding: 0;
  border: none;
  border-radius: 3px;
  background: transparent;
  color: rgb(203 213 225 / 55%);
  font-size: 0.7rem;
  line-height: 1;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}
.action-top {
  color: rgb(52 211 153 / 60%);
}
.action-right {
  color: rgb(96 165 250 / 60%);
}
.action-bottom {
  color: rgb(248 113 113 / 60%);
}
.action:hover {
  background: rgb(255 255 255 / 10%);
  color: #fff;
}
</style>
