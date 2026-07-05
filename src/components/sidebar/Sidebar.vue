<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue';

import NavigationButton from './NavigationButton.vue';
import NavigationSection from './NavigationSection.vue'
import { Bolt, Component, Play, Puzzle, Settings, Users } from 'lucide-vue-next';
import ProfilesListbox from './ProfilesListbox.vue';
import { useModsStore } from '../../stores/mods';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from 'primevue/usetoast';
import { useSettingsStore } from '../../stores/settings';
import { useI18n } from 'vue-i18n';
import { useConfirm } from '../../plugins/ConfirmService';
import { useLoggingStore } from '../../stores/logging';

const { t } = useI18n()
const toast = useToast()
const confirm = useConfirm()
const loggingStore = useLoggingStore()

const settingsStore = useSettingsStore()
// const profilesStore = useProfilesStore()

const gameVersion = ref<string | null>(null)

const {
    isSyncNeeded
} = useModsStore()


const { getGameVersion } = useSettingsStore()

onMounted(async () => {
    gameVersion.value = await getGameVersion()
})

async function launchGame() {
    if (await isSyncNeeded()) {
        const result = await confirm.confirm({
            title: t('sidebar.confirmations.unsyncedMods.title'),
            message: t('sidebar.confirmations.unsyncedMods.message'),
            acceptButton: {
                label: t('sidebar.confirmations.unsyncedMods.actions.launchAnyway')
            },
            rejectButton: {
                label: t('common.actions.cancel')
            }
        })
        if (result.confirmed) {
            await invoke("launch_game")
        }
    } else {
        await invoke("launch_game").then(() => {
            toast.add({ severity: 'success', summary: t('sidebar.notifications.gameLaunched.title'), detail: t('sidebar.notifications.gameLaunched.description'), life: 3000 })
        }).catch((error) => {
            loggingStore.logError('Faile d to launch game', error)
            toast.add({ severity: 'error', summary: t('sidebar.notifications.gameLaunchError.title'), detail: t('sidebar.notifications.gameLaunchError.description'), life: 5000 })
        })
    }
}

const headerImage = computed(() => {
    if (import.meta.env.DEV) {
        return `headers/header7.png`
    }
    return `headers/header${Math.floor(Math.random() * 7) + 1}.png`
})

watch(() => settingsStore.settings.gameDirectory, (gameDir) => {
    if (gameDir) {
        getGameVersion().then(version => {
            gameVersion.value = version
        })
    }
})

</script>
<template>
    <div
        class="flex flex-col min-h-0 h-full bg-bg-surface gap-0 overflow-y-auto min-w-70 max-w-70 border-r border-border">
        <!-- header -->
        <div class="h-32 flex items-center flex-col justify-center gap-1 relative overflow-hidden">
            <span class="font-cinzel font-bold text-xl text-primary">
                BROWNDUST II
            </span>

            <span v-if="gameVersion" class="text-xs font-semibold text-primary flex gap-2 items-center justify-center">
                Game v{{ gameVersion }}
            </span>

            <img :src="headerImage" class="absolute inset-0 w-full h-full
         object-cover
         opacity-35
         mask-[linear-gradient(to_bottom,black_50%,transparent_100%)]
         pointer-events-none select-none" />
        </div>

        <!-- navigation -->
        <div>
            <!-- section: navigation -->
            <NavigationSection :title="$t('sidebar.sections.navigation')">
                <NavigationButton :icon="Component" :label="$t('sidebar.tabs.mods')" value="mods" />
                <NavigationButton :icon="Users" :label="$t('sidebar.tabs.characters')" value="characters" />
                <!-- <NavigationButton :icon="GalleryVertical" :label="$t('sidebar.tabs.scenes')" value="scenes" /> -->
                <NavigationButton icon-src="/icons/npc.png" :label="$t('sidebar.tabs.npcs')" value="npcs" />
            </NavigationSection>
            <!-- section: settings -->
            <NavigationSection :title="$t('sidebar.sections.settings')">
                <NavigationButton :icon="Bolt" :label="$t('sidebar.tabs.profiles')" value="profiles" />
                <NavigationButton :icon="Puzzle" :label="$t('sidebar.tabs.browndustx')" value="bdx" />
                <NavigationButton :icon="Settings" :label="$t('sidebar.tabs.settings')" value="settings" />
            </NavigationSection>
        </div>

        <!-- profile section -->
        <div>
            <NavigationSection :title="$t('sidebar.sections.currentProfile')">
                <ProfilesListbox />
            </NavigationSection>
        </div>

        <!-- // spacer -->
        <span class="flex-1"></span>

        <!-- launch game button -->
        <div class="m-4">
            <button @click="launchGame" class="
                w-full p-4 gap-2 rounded-md
                cursor-pointer 
                group 
                transition-all 
                transform hover:scale-100 active:scale-99
                text-primary font-bold
                text-sm
                flex items-center align-center justify-center 
                bg-interactive-bg
                border-2 border-border hover:bg-accent-primary-hover! hover:border-accent-primary-hover">
                <Play class="w-[1.5em] h-[1.5em] group-hover:text-purple-200 transition-color duration-300" />
                {{ $t('sidebar.actions.launchGame') }}
            </button>
        </div>
    </div>
</template>
<style scoped>
</style>