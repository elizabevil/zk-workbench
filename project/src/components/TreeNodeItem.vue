<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";

import { type InvokeRejection, toAppError } from "@/api/client";
import { nodeChildren } from "@/api/node";
import type { AppError } from "@/api/types/AppError";

const props = defineProps<{
  connectionId: string;
  name: string;
  path: string;
  depth: number;
}>();

const { t } = useI18n();
const localError = ref(false);
const emit = defineEmits<{
  select: [path: string];
  error: [error: AppError];
}>();

const expanded = ref(false);
const loading = ref(false);
const children = ref<string[] | null>(null);

async function toggle(): Promise<void> {
  if (expanded.value) {
    expanded.value = false;
    return;
  }
  expanded.value = true;
  if (children.value !== null) {
    return;
  }
  loading.value = true;
  localError.value = false;
  try {
    children.value = await nodeChildren(props.connectionId, props.path);
  } catch (raw) {
    localError.value = true;
    const error = toAppError(raw as InvokeRejection);
    emit("error", error);
  } finally {
    loading.value = false;
  }
}

function childPath(name: string): string {
  return props.path === "/" ? `/${name}` : `${props.path}/${name}`;
}
</script>

<template>
  <div class="node">
    <div class="row" :style="{ paddingLeft: `${depth * 14}px` }">
      <button class="twist" type="button" @click="toggle">
        {{ expanded ? "▾" : "▸" }}
      </button>
      <button class="label" type="button" @click="emit('select', path)">
        {{ name }}
      </button>
      <span v-if="loading" class="muted">…</span>
      <button v-else-if="localError" class="retry" type="button" @click="toggle">
        {{ t("shell.retry") }}
      </button>
    </div>

    <div v-if="expanded">
      <TreeNodeItem
        v-for="child in children ?? []"
        :key="child"
        :connection-id="connectionId"
        :name="child"
        :path="childPath(child)"
        :depth="depth + 1"
        @select="emit('select', $event)"
        @error="emit('error', $event)"
      />
      <div
        v-if="children && children.length === 0"
        class="muted empty"
        :style="{ paddingLeft: `${(depth + 1) * 14 + 20}px` }"
      >
        —
      </div>
    </div>
  </div>
</template>

<style scoped>
.row {
  display: flex;
  align-items: center;
  gap: 4px;
  min-height: 26px;
}
.twist,
.label {
  background: transparent;
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 2px 4px;
}
.twist {
  width: 18px;
  color: #9aa4b2;
}
.label {
  font-family: ui-monospace, monospace;
  font-size: 13px;
  text-align: left;
}
.label:hover {
  color: #93c5fd;
}
.retry {
  border: none;
  background: transparent;
  color: #f87171;
  cursor: pointer;
  font-size: 11px;
}
.muted {
  color: #9aa4b2;
  font-size: 12px;
}
.empty {
  min-height: 22px;
  display: flex;
  align-items: center;
}
</style>
