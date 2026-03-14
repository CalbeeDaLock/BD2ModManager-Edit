import { invoke } from "@tauri-apps/api/core";

import { defineStore } from "pinia";
import { ref, computed } from "vue";

export interface Profile {
    id: string,
    name: string,
    description: string,
    createdAt: string,
    active: boolean,
    enabledMods: Array<string>
}

interface ProfilesState {
    activeProfile: Profile | null,
    profiles: Array<Profile>
}

// TODO: use events?

export const useProfilesStore = defineStore("profiles", () => {
    const activeProfile = ref<Profile | null>(null);
    const activeProfileId = ref<string | null>(null);
    const profiles = ref<Profile[]>([]);

    const sortedProfiles = computed(() => {
        return [...profiles.value].sort((a, b) => {
            if (a.id === "default") return -1;
            if (b.id === "default") return 1;
            return new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime();
        });
    });

    async function loadProfiles() {
        await invoke<ProfilesState>("get_profiles").then(result => {
            activeProfile.value = result.activeProfile;
            activeProfileId.value = result.activeProfile?.id || null;
            profiles.value = result.profiles;
            console.log(profiles.value);
        });
    }

    async function switchProfile(id: string) {
        await invoke("switch_profile", { id }).then(() => {
            activeProfile.value = getProfileById(id);
            activeProfileId.value = id;
        });
    }

    async function createProfile(name: string, description: string | null, templateId: string | null) {
        await invoke("create_profile", { name, description, templateId }).then(result => {
            console.log("create profile", result);
        });
        await loadProfiles();
    }

    async function deleteProfile(id: string) {
        await invoke("delete_profile", { id }).then(result => {
            console.log("delete profile", result);
        });
        await loadProfiles();
    }

    function getProfileById(id: string): Profile | null {
        const profile = profiles.value.find(p => p.id === id);
        return profile || null;
    }

    async function editProfile(id: string, name: string, description: string | null) {
        await invoke("edit_profile", { id, name, description }).then(async (result) => {
            console.log("edit profile", result);
        });
        await loadProfiles();
    }

    return {
        activeProfile,
        activeProfileId,

        profiles,
        sortedProfiles,

        loadProfiles,
        switchProfile,
        createProfile,
        deleteProfile,
        getProfileById,
        editProfile
    };
});