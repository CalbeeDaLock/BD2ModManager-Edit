<script setup lang="ts">
import { ref, watch } from 'vue';
import { storeToRefs } from 'pinia';

import { AlertOctagon, X } from 'lucide-vue-next';

import { open } from '@tauri-apps/plugin-dialog';
import { dirname } from "@tauri-apps/api/path"

import { useLoggingStore } from '../../stores/logging';
import { useSettingsStore } from '../../stores/settings';
import { useI18n } from 'vue-i18n';

import Modal from '../common/Modal.vue';
import Input from '../common/Input.vue';
import Button from '../common/Button.vue';

const loggingStore = useLoggingStore()

const visible = defineModel('visible', {
    type: Boolean,
    required: true
})

const settingsStore = useSettingsStore()

const {
    locateGamePath,
    validateGamePath,
    saveSettings
} = settingsStore

const { settings } = storeToRefs(settingsStore)

const errorMessage = ref(null as string | null)
const gamePathsFound = ref<string[]>([])

const { t } = useI18n()

async function handlePathSelected(path: string) {
    try {
        const isValid = await validateGamePath(path)

        if (!isValid) {
            errorMessage.value = t("modals.selectGameDirectory.errors.invalidGameDirectory", {
                path: path
            })
            return
        }

        // await setGameDirectory(path)
        await saveSettings({
            gameDirectory: path
        })

        visible.value = false
        errorMessage.value = null
    } catch (err) {
        errorMessage.value = t("modals.selectGameDirectory.errors.unknownError", {
            error: err
        })
    }
}

async function handleDirectoryBrowse() {
    const file = await open({
        filters: [
            {
                name: "Executable Files",
                extensions: ["exe"],
            },
        ],
        multiple: false,
        directory: false,
    })

    if (typeof file === 'string') {
        if (!file.toLowerCase().endsWith('browndust ii.exe')) {
            errorMessage.value = t('modals.selectGameDirectory.errors.invalidGameExecutable')
            return
        }

        const folder = await dirname(file) //file.substring(0, file.lastIndexOf(file.includes('/') ? '/' : '\\'))

        handlePathSelected(folder)
    }
}

function handleClose() {
    visible.value = false
}

watch(visible, async (isVisible) => {
    if (isVisible) {
        loggingStore.logDebug('SelectGameDirectory modal opened, attempting to locate game path.')

        let detectedGamePath = await locateGamePath()
        if (detectedGamePath) {
            gamePathsFound.value = detectedGamePath
        }



    }
})
</script>

<template>
    <Modal v-model:show="visible" class="w-[80%] max-w-200 max-h-[80%]" @close="handleClose"
        :closeOnClickOutside="false">
        <div class="flex flex-col gap-4 p-4 bg-bg-surface relative">
            <div class="flex items-center justify-between">
                <h2 class="text-primary text-xl font-bold">
                    <span>{{ $t('modals.selectGameDirectory.title') }}</span>
                    <!-- Select Brown Dust II Directory -->
                </h2>
                <X class="text-primary w-[1.75em] h-[1.75em] top-2.5 right-2.5 cursor-pointer hover:text-accent-primary! transition-colors"
                    @click="handleClose" />
            </div>


            <div class="text-center">
                <p class="text-secondary text-sm">
                    {{ $t('modals.selectGameDirectory.description') }}
                    <!-- Please select the "Brown Dust II.exe" -->
                </p>
            </div>

            <div class="flex gap-2 items-stretch h-10">
                <Input readonly :model-value="settings.gameDirectory ?? undefined"
                    :placeholder="$t('modals.selectGameDirectory.placeholders.gamePath')" />
                <Button :label="$t('modals.selectGameDirectory.actions.browse')" @click="handleDirectoryBrowse" />
            </div>

            <div v-if="gamePathsFound.length > 0">
                <p class="text-sm text-secondary mb-2">
                    {{ $t('modals.selectGameDirectory.labels.foundPaths') }}
                </p>
                <div
                    class="w-full border rounded border-border bg-bg-surface flex flex-col max-h-50 overflow-y-auto">
                    <div v-for="path in gamePathsFound" :key="path"
                        class="p-3 py-2 bg-bg-surface hover:bg-interactive-bg-hover! cursor-pointer items-center rounded border border-transparent transition-colors"
                        @click="handlePathSelected(path)">
                        <span class="text-primary">{{ path }}</span>
                    </div>
                </div>

            </div>

            <div class="flex items-center justify-between">
                <div v-if="errorMessage" class="flex items-center gap-2">
                    <AlertOctagon class="text-red-400 w-4 h-4" />
                    <span class="text-red-400 text-sm">{{ errorMessage }}</span>
                </div>
                <span v-else></span>
                <Button :label="$t('modals.selectGameDirectory.actions.skip')" @click="handleClose" variant="text" />
            </div>
        </div>
    </Modal>

</template>