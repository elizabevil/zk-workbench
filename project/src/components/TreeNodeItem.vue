<script lang="ts" setup>
import {computed, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import {type InvokeRejection, toAppError} from "@/api/client";
import {nodeChildren} from "@/api/node";
import type {AppError} from "@/api/types/AppError";

const props = defineProps<{
  connectionId: string;
  name: string;
  path: string;
  depth: number;
  selectedPath: string | null;
  expandedPaths: Set<string>;
}>();

const {t} = useI18n();
const localError = ref<AppError | null>(null);
const emit = defineEmits<{
  select: [path: string];
  dblclick: [path: string];
  toggle: [path: string];
  create: [path: string];
  edit: [path: string];
  delete: [path: string];
}>();

const loading = ref(false);
const children = ref<string[] | null>(null);
const expanded = computed(() => props.expandedPaths.has(props.path));

async function load(): Promise<void> {
  if (loading.value || children.value !== null) {
    return;
  }
  loading.value = true;
  localError.value = null;
  try {
    children.value = await nodeChildren(props.connectionId, props.path);
  } catch (raw) {
    localError.value = toAppError(raw as InvokeRejection);
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
    {immediate: true},
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
        :aria-level="depth + 1"
        :aria-selected="path === selectedPath"
        :class="{ selected: path === selectedPath }"
        class="row"
        role="treeitem"
        tabindex="0"
        @click="emit('select', path)"
        @keydown.enter.prevent="emit('select', path)"
        @keydown.space.prevent="emit('select', path)"
    >
      <span v-if="depth > 0" aria-hidden="true" class="dash">–</span>
      <button :aria-expanded="expanded" class="twist" type="button" @click.stop="toggle">
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
      <button
          v-else-if="localError"
          :title="t(localError.message_key)"
          class="retry"
          type="button"
          @click="load"
      >
        {{ t("shell.retry") }}
      </button>
      <div class="row-actions">
        <button
            :title="t('shell.newChild')"
            class="action action-top"
            type="button"
            @click.stop="emit('create', path)"
        >
          +
        </button>
        <button
            v-if="path !== '/'"
            :title="t('shell.editNode')"
            class="action action-right"
            type="button"
            @click.stop="emit('edit', path)"
        >
          ✎
        </button>
        <button
            v-if="path !== '/'"
            :title="t('shell.deleteNode')"
            class="action action-bottom"
            type="button"
            @click.stop="emit('delete', path)"
        >
          ×
        </button>
      </div>
    </div>

    <div v-if="expanded" class="children">
      <TreeNodeItem
          v-for="child in children ?? []"
          :key="child"
          :connection-id="connectionId"
          :depth="depth + 1"
          :expanded-paths="expandedPaths"
          :name="child"
          :path="childPath(child)"
          :selected-path="selectedPath"
          @create="emit('create', $event)"
          @dblclick="emit('dblclick', $event)"
          @delete="emit('delete', $event)"
          @edit="emit('edit', $event)"
          @select="emit('select', $event)"
          @toggle="emit('toggle', $event)"
      />
      <div v-if="children && children.length === 0" class="empty">—</div>
    </div>
  </div>
</template>

<style scoped>
.node {
  font-size: 12.5px;
}

.row {
  display: flex;
  align-items: center;
  gap: 4px;
  width: fit-content;
  max-width: 100%;
  padding: 3px 6px;
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

.dash {
  flex: none;
  color: rgb(148 163 184 / 42%);
  font-family: ui-monospace, monospace;
  user-select: none;
}

.twist {
  flex: none;
  width: 1.1rem;
  height: 1.1rem;
  padding: 0;
  border: none;
  background: transparent;
  color: #9aa4b2;
  cursor: pointer;
  font-size: 10px;
  line-height: 1;
}

.label {
  min-width: 0;
  max-width: 100%;
  padding: 0 2px;
  border: none;
  background: transparent;
  color: #d7dee8;
  cursor: pointer;
  font-size: 12.5px;
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

.children {
  margin-left: 10px;
  padding-left: 9px;
  border-left: 1px solid rgb(148 163 184 / 22%);
}

.empty {
  padding: 2px 6px;
  color: #6b7280;
  font-size: 11px;
  user-select: none;
}

.row-actions {
  display: inline-flex;
  align-items: center;
  gap: 2px;
  max-width: 0;
  overflow: hidden;
  opacity: 0;
  visibility: hidden;
  transition: max-width 120ms ease,
  opacity 120ms ease;
}

.row:hover .row-actions,
.row:focus-within .row-actions,
.row.selected .row-actions {
  max-width: 7rem;
  opacity: 1;
  visibility: visible;
}

.action {
  flex: none;
  width: 1.1rem;
  height: 1.1rem;
  padding: 0;
  border: none;
  border-radius: 3px;
  background: transparent;
  color: rgb(203 213 225 / 65%);
  font-size: 0.75rem;
  line-height: 1;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.action-top {
  color: rgb(52 211 153 / 70%);
}

.action-right {
  color: rgb(96 165 250 / 70%);
}

.action-bottom {
  color: rgb(248 113 113 / 70%);
}

.action:hover {
  background: rgb(255 255 255 / 10%);
  color: #fff;
}
</style>
