<script lang="ts" setup>
import {NAlert, NButton, NForm, NFormItem, NInput, NInputNumber, NModal, type InputInst} from "naive-ui";
import {computed, nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import {useI18n} from "vue-i18n";

import type {ConnectionProfile} from "@/api/types/ConnectionProfile";
import type {ConnectionSnapshot} from "@/api/types/ConnectionSnapshot";
import {KEY_ESCAPE} from "@/constants/keyboard";
import {useConnectionStore} from "@/stores/connection";
import {useProfilesStore} from "@/stores/profiles";
import {applyNumber} from "@/utils/number";

const props = defineProps<{ open: boolean; profile?: ConnectionProfile | null }>();
const emit = defineEmits<{
  close: [];
  connected: [snapshot: ConnectionSnapshot];
  saved: [];
}>();

const {t} = useI18n();
const store = useConnectionStore();
const profiles = useProfilesStore();

const DEFAULT_HOST = "127.0.0.1";
const DEFAULT_PORT = 2181;

const host = ref(DEFAULT_HOST);
const port = ref(DEFAULT_PORT);
const displayName = ref("local");
const group = ref("");
const localError = ref("");
const nameInput = ref<InputInst | null>(null);

const dialogError = computed(() => profiles.error ?? store.error);
const title = computed(() => (props.profile ? t("home.editTitle") : t("home.new")));

watch(
  () => props.open,
  async (open) => {
    if (!open) {
      return;
    }
    localError.value = "";
    if (props.profile) {
      displayName.value = props.profile.name;
      host.value = props.profile.host;
      port.value = props.profile.port;
      group.value = props.profile.group ?? "";
    } else {
      host.value = DEFAULT_HOST;
      port.value = DEFAULT_PORT;
      displayName.value = "local";
      group.value = "";
    }
    await nextTick();
    nameInput.value?.focus();
  },
);

function setPort(value: number): void {
  port.value = value;
}

async function submit(event?: Event): Promise<void> {
  event?.preventDefault();
  if (!host.value.trim()) {
    localError.value = t("home.hostRequired");
    return;
  }
  if (!Number.isInteger(port.value) || port.value < 1 || port.value > 65535) {
    localError.value = t("home.portInvalid");
    return;
  }
  localError.value = "";
  const name = displayName.value.trim() || `${host.value}:${port.value}`;
  const saved = await profiles.save({
    id: props.profile?.id ?? null,
    name,
    host: host.value.trim(),
    port: port.value,
    group: group.value.trim() ? group.value.trim() : null,
  });
  if (!saved) {
    return;
  }
  if (props.profile) {
    emit("saved");
    return;
  }
  const created = await store.connect({
    host: host.value.trim(),
    port: port.value,
    displayName: name,
  });
  if (created) {
    emit("connected", created);
  }
}

function onShow(value: boolean): void {
  if (!value) {
    emit("close");
  }
}

function onKeydown(event: KeyboardEvent): void {
  if (props.open && event.key === KEY_ESCAPE) {
    event.preventDefault();
    event.stopImmediatePropagation();
    emit("close");
  }
}

onMounted(() => window.addEventListener("keydown", onKeydown));
onUnmounted(() => window.removeEventListener("keydown", onKeydown));
</script>

<template>
  <NModal
    :close-on-esc="false"
    :content-style="{padding: '18px 22px 8px'}"
    :footer-style="{padding: '10px 22px 18px'}"
    :header-style="{padding: '16px 22px 8px'}"
    :show="open"
    :style="{width: '440px'}"
    :title="title"
    preset="card"
    @update:show="onShow"
  >
    <NForm
      :on-submit="submit"
      :show-feedback="false"
      class="connection-form"
      label-placement="top"
      size="small"
    >
      <NFormItem :label="t('home.name')">
        <NInput ref="nameInput" v-model:value="displayName" />
      </NFormItem>
      <NFormItem :label="t('home.host')">
        <NInput v-model:value="host" :placeholder="DEFAULT_HOST" />
      </NFormItem>
      <NFormItem :label="t('home.port')">
        <NInputNumber
          :max="65535"
          :min="1"
          :show-button="false"
          :value="port"
          class="port-input"
          @update:value="(value) => applyNumber(value, setPort)"
        />
      </NFormItem>
      <NFormItem :label="t('home.group')">
        <NInput v-model:value="group" :placeholder="t('home.groupPlaceholder')" />
      </NFormItem>
    </NForm>

    <NAlert v-if="localError" :bordered="false" class="dialog-alert" type="error">
      {{ localError }}
    </NAlert>
    <NAlert v-else-if="dialogError" :bordered="false" class="dialog-alert" type="error">
      {{ t(dialogError.message_key) }}
      <span v-if="dialogError.fields.detail">（{{ dialogError.fields.detail }}）</span>
    </NAlert>

    <template #footer>
      <div class="actions">
        <NButton @click="emit('close')">{{ t("home.cancel") }}</NButton>
        <NButton :loading="store.loading" type="primary" @click="submit()">
          {{ store.loading ? t("home.connecting") : profile ? t("home.save") : t("home.connect") }}
        </NButton>
      </div>
    </template>
  </NModal>
</template>

<style scoped>
.connection-form :deep(.n-form-item) {
  margin-bottom: 10px;
}

.connection-form :deep(.n-form-item:last-child) {
  margin-bottom: 0;
}

.connection-form :deep(.n-form-item-label) {
  padding-bottom: 6px;
  color: var(--app-text-muted, #9aa4b2);
  font-size: 12px;
}

.connection-form :deep(.n-input) {
  width: 100%;
}

.port-input {
  width: 130px;
}

.dialog-alert {
  margin-top: 6px;
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
