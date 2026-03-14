<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';
import { AlertOctagon, Check } from 'lucide-vue-next';
import { open } from '@tauri-apps/plugin-dialog';
import { dirname } from '@tauri-apps/api/path';
import { useLoggingStore } from '../../stores/logging';
import { useSettingsStore } from '../../stores/settings';
import { useI18n } from 'vue-i18n';

import Button from '../common/Button.vue';
import Input from '../common/Input.vue';
import Modal from '../common/Modal.vue';

const loggingStore = useLoggingStore()
const settingsStore = useSettingsStore()
const { locateGamePath, validateGamePath, saveSettings } = settingsStore
const { settings } = storeToRefs(settingsStore)
const { t } = useI18n()

const visible = defineModel('visible', {
    type: Boolean,
    required: true,
    default: false
})

const errorMessage = ref<string | null>(null)
const gamePathsFound = ref<string[]>([])

async function handlePathSelected(path: string) {
    try {
        const isValid = await validateGamePath(path)

        if (!isValid) {
            errorMessage.value = t('modals.selectGameDirectory.errors.invalidGameDirectory', { path })
            return
        }

        await saveSettings({ gameDirectory: path })
        errorMessage.value = null
    } catch (err) {
        errorMessage.value = t('modals.welcome.selectGameDirectory.errors.unknownError', { error: err })
    }
}

async function handleDirectoryBrowse() {
    const file = await open({
        filters: [{ name: 'Executable Files', extensions: ['exe'] }],
        multiple: false,
        directory: false,
    })

    if (typeof file === 'string') {
        if (!file.toLowerCase().endsWith('browndust ii.exe')) {
            errorMessage.value = t('modals.welcome.selectGameDirectory.errors.invalidGameExecutable')
            return
        }

        const folder = await dirname(file)
        handlePathSelected(folder)
    }
}

function onClose() {
    visible.value = false
}

async function findGamePaths() {
    loggingStore.logDebug('WelcomeModal opened, attempting to locate game path.')
    const detectedGamePaths = await locateGamePath()
    if (detectedGamePaths && detectedGamePaths.length > 0) {
        gamePathsFound.value = [...detectedGamePaths]
    } else {
        loggingStore.logDebug('No game paths found by locateGamePath.')
        gamePathsFound.value = []
    }
}

onMounted(() => {
    loggingStore.logDebug('WelcomeModal mounted.')
    if (visible.value) {
        findGamePaths()
    }
})

watch(visible, async (isVisible) => {
    if (isVisible) {
        await findGamePaths()
    }
})

import RefinedGithub from '../../assets/icons/github.svg';
import { openUrl } from '@tauri-apps/plugin-opener';
import Discord from '../icons/Discord.vue';

const GITHUB_URL = 'https://github.com/bruhnn/BD2ModManager'
const DISCORD_URL = 'https://discord.gg/B3Aqz6tDG2'

</script>

<template>
    <Modal v-model:show="visible" class="w-[80%] max-w-200 max-h-[80%]" @close="onClose" :close-on-escape="false"
        :title="$t('modals.welcome.title')" :subtitle="$t('modals.welcome.subtitle')">
        <div class="w-full h-full flex flex-col gap-5 p-4">
            <div class="flex items-start gap-3 p-3 rounded-md border border-red-500/30 bg-red-500/10">
                <AlertOctagon class="w-4 h-4 text-red-400 shrink-0 mt-0.5" />
                <p class="text-xs text-red-200">
                    This is a <span class="font-semibold">beta version</span> of the mod manager.
                    Bugs or unexpected behavior may occur. If you find any issues, please report them on
                    <span class="underline cursor-pointer hover:text-red-100" @click="openUrl(GITHUB_URL)">GitHub
                        Issues</span>
                </p>
            </div>
            <div class="flex items-center justify-ceter gap-2">
                <span @click="openUrl(GITHUB_URL)"
                    class="text-sm flex items-center gap-1.5 text-secondary bg-bg-surface border border-border rounded-full px-3 py-1 hover:text-primary! hover:bg-interactive-bg-hover! cursor-pointer transition-colors">
                    <RefinedGithub class="w-4 h-4 fill-secondary" />
                    {{ $t('modals.welcome.github') }}
                </span>
                <span @click="openUrl(DISCORD_URL)"
                    class="text-sm flex items-center gap-1.5 text-secondary bg-bg-surface border border-border rounded-full px-3 py-1 hover:text-primary! hover:bg-interactive-bg-hover! cursor-pointer transition-colors">
                    <Discord class="w-4 h-4 fill-secondary" />
                    {{ $t('modals.welcome.discord') }}
                </span>
            </div>

            <div class="flex flex-col gap-3">
                <div class="flex flex-col gap-1">
                    <p class="text-sm font-medium text-primary">{{ $t('modals.welcome.selectGameDirectory.title') }}</p>
                    <p class="text-xs text-secondary">
                        {{ $t('modals.welcome.selectGameDirectory.description') }}
                    </p>
                </div>

                <div class="flex gap-2 items-stretch h-10">
                    <Input readonly :model-value="settings.gameDirectory ?? ''"
                        :placeholder="$t('modals.welcome.selectGameDirectory.placeholders.gamePath')" />
                    <Button :label="$t('common.actions.browse')" @click="handleDirectoryBrowse" />
                </div>

                <div v-if="settings.gameDirectory && !errorMessage" class="flex items-center gap-2">
                    <Check class="text-success green-400 w-4 h-4 shrink-0" />
                    <span class="text-success text-xs">
                        {{ $t('modals.welcome.selectGameDirectory.validPath') }}
                    </span>
                </div>

                <div v-if="errorMessage" class="flex items-center gap-2">
                    <AlertOctagon class="text-error w-4 h-4 shrink-0" />
                    <span class="text-error text-xs">{{ errorMessage }}</span>
                </div>

                <div v-if="gamePathsFound.length > 0" class="flex flex-col gap-2">
                    <p class="text-xs text-secondary uppercase tracking-wide">
                        {{ $t('modals.welcome.selectGameDirectory.labels.foundPaths') }}
                    </p>
                    <div
                        class="w-full border rounded border-border bg-bg-surface flex flex-col max-h-48 overflow-y-auto">
                        <div v-for="path in gamePathsFound" :key="path"
                            class="px-3 py-2 hover:bg-interactive-bg-hover! cursor-pointer flex items-center gap-2 rounded border border-transparent transition-colors"
                            @click="handlePathSelected(path)">
                            <Check v-if="path === settings.gameDirectory"
                                class="text-accent-primary w-4 h-4 shrink-0" />
                            <!-- <div v-else class="w-4 h-4 shrink-0" /> -->
                            <span class="text-primary text-sm font-mono truncate">{{ path }}</span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex justify-end mt-auto">
                <Button @click="onClose" :label="$t('common.actions.continue')" />
            </div>

        </div>
    </Modal>
</template>

<style scoped></style>