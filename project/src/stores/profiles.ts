//! 连接档案状态：本地持久化，重启后仍可列出。

import {defineStore} from "pinia";
import {type Ref, ref} from "vue";

import {type InvokeRejection, toAppError} from "@/api/client";
import {profileDelete, profileList, profileSave} from "@/api/profiles";
import type {AppError} from "@/api/types/AppError";
import type {ConnectionProfile} from "@/api/types/ConnectionProfile";
import type {ProfileInput} from "@/api/types/ProfileInput";

export interface ProfilesStoreSetup {
    profiles: Ref<ConnectionProfile[]>;
    error: Ref<AppError | null>;
    loading: Ref<boolean>;
    load: () => Promise<void>;
    save: (input: ProfileInput) => Promise<ConnectionProfile | null>;
    remove: (id: string) => Promise<void>;
}

export const useProfilesStore = defineStore("profiles", (): ProfilesStoreSetup => {
    const profiles = ref<ConnectionProfile[]>([]);
    const error = ref<AppError | null>(null);
    const loading = ref(false);

    async function load(): Promise<void> {
        try {
            profiles.value = await profileList();
        } catch (raw) {
            error.value = toAppError(raw as InvokeRejection);
        }
    }

    async function save(input: ProfileInput): Promise<ConnectionProfile | null> {
        loading.value = true;
        error.value = null;
        try {
            const saved = await profileSave(input);
            const index = profiles.value.findIndex((item) => item.id === saved.id);
            if (index >= 0) {
                profiles.value[index] = saved;
            } else {
                profiles.value.push(saved);
            }
            return saved;
        } catch (raw) {
            error.value = toAppError(raw as InvokeRejection);
            return null;
        } finally {
            loading.value = false;
        }
    }

    async function remove(id: string): Promise<void> {
        try {
            await profileDelete(id);
            profiles.value = profiles.value.filter((item) => item.id !== id);
        } catch (raw) {
            error.value = toAppError(raw as InvokeRejection);
        }
    }

    return {profiles, error, loading, load, save, remove};
});
