<script setup lang="ts">
import { RefreshCcw, Github, Upload, Trash2, AlertTriangle, TriangleAlert } from 'lucide-vue-next';

import { computed, nextTick, onBeforeUnmount, onMounted, ref, useTemplateRef, watch } from 'vue';
import { useI18n } from 'vue-i18n';

import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { listen } from '@tauri-apps/api/event';

import { useSettingsStore } from '../../stores/settings';
import { useBdxLogsStore } from '../../stores/BDXLogs';

import { useHeader } from '../../composables/useHeader';
import { useConfirm } from '../../plugins/ConfirmService';

import BepInEx from './modals/BepInEx.vue';

import Button from '../../components/common/Button.vue';
import { useLoggingStore } from '../../stores/logging';

enum BepInExStatus { INSTALLED = "INSTALLED", NOT_INSTALLED = "NOT_INSTALLED" }
enum BrownDustXStatus { INSTALLED = "INSTALLED", NOT_INSTALLED = "NOT_INSTALLED", BEPINEX_MISSING = "BEPINEX_MISSING" }
enum ConfigurationManagerStatus { INSTALLED = "INSTALLED", NOT_INSTALLED = "NOT_INSTALLED", BEPINEX_MISSING = "BEPINEX_MISSING" }

const { t } = useI18n()
const confirm = useConfirm()

const settingsStore = useSettingsStore()

const {
    logs
} = useBdxLogsStore()

const loggingStore = useLoggingStore()

const bepInExState = ref<{
    status: BepInExStatus;
    version: string | null
    can_configure?: boolean,
    can_remove?: boolean
}>({ status: BepInExStatus.NOT_INSTALLED, version: null, can_configure: false, can_remove: false })
const brownDustXState = ref<{
    status: BrownDustXStatus; version: string | null,
    can_configure?: boolean,
    can_remove?: boolean
}>({ status: BrownDustXStatus.NOT_INSTALLED, version: null, can_configure: false, can_remove: false })
const configurationManagerState = ref<{
    status: ConfigurationManagerStatus; version: string | null,
    can_remove?: boolean
    can_configure?: boolean
}>({ status: ConfigurationManagerStatus.NOT_INSTALLED, version: null, can_configure: false, can_remove: false })

const isBepInExDialogOpen = ref(false)

function openBepInExDialog() {
    isBepInExDialogOpen.value = true
}

async function getBDXState(log: boolean = true) {
    try {
        let result = await invoke<{
            status: BrownDustXStatus,
            version: string | null,
            can_configure?: boolean,
            can_remove?: boolean
        }>('get_browndustx_version')

        brownDustXState.value = result

        if (log) {
            logs.push({
                level: 'info',
                scope: 'BrownDustX',
                message: result.status === BrownDustXStatus.INSTALLED
                    ? `BrownDustX version ${result.version} is installed.`
                    : 'BrownDustX is not installed.',
                timestamp: new Date().toISOString(),
            })
        }

        // add reason why it was not installed?
    } catch (error) {
        loggingStore.logError('Error fetching BrownDustX version:', error)

        brownDustXState.value = { status: BrownDustXStatus.NOT_INSTALLED, version: null }

        logs.push({
            level: 'error',
            scope: 'BrownDustX',
            message: `Error fetching BrownDustX version: ${error}.`,
            timestamp: new Date().toISOString(),
        })
    }
}

async function getBepInExState(log: boolean = true) {
    try {
        let result = await invoke<{
            status: BepInExStatus,
            version: string | null
        }>('get_bepinex_version')

        bepInExState.value = result

        if (log) {
            logs.push({
                level: 'info',
                scope: 'BepInEx',
                message: result.status === BepInExStatus.INSTALLED
                    ? `BepInEx version ${result.version} is installed.`
                    : 'BepInEx is not installed.',
                timestamp: new Date().toISOString(),
            })

        }
    } catch (error) {
        console.error('Error fetching BepInEx version:', error)

        bepInExState.value = { status: BepInExStatus.NOT_INSTALLED, version: null }

        logs.push({
            level: 'error',
            scope: 'BepInEx',
            message: `Error fetching BepInEx version: ${error}.`,
            timestamp: new Date().toISOString(),
        })
    }
}

async function getConfigManagerState(log: boolean = true) {
    try {
        let result = await invoke<{
            status: ConfigurationManagerStatus,
            version: string | null
        }>('get_configmanager_version')

        configurationManagerState.value = result

        if (log) {
            logs.push({
                level: 'info',
                scope: 'Configuration Manager',
                message: result.status === ConfigurationManagerStatus.INSTALLED
                    ? `Configuration Manager version ${result.version} is installed.`
                    : 'Configuration Manager is not installed.',
                timestamp: new Date().toISOString(),
            })
        }
    } catch (error) {
        console.error('Error fetching Configuration Manager status:', error)

        configurationManagerState.value = { status: ConfigurationManagerStatus.NOT_INSTALLED, version: null }

        logs.push({
            level: 'error',
            scope: 'Configuration Manager',
            message: `Error fetching Configuration Manager status: ${error}.`,
            timestamp: new Date().toISOString(),
        })
    }
}

async function installBepInExFromUrl(url: string) {
    try {
        logs.push({
            level: 'info',
            scope: 'BepInEx',
            message: `Starting BepInEx installation from URL: ${url}`,
            timestamp: new Date().toISOString(),
        })

        await invoke('install_bepinex_from_url', { url })

        logs.push({
            level: 'success',
            scope: 'BepInEx',
            message: `BepInEx installed successfully from URL.`,
            timestamp: new Date().toISOString(),
        })

        await initialize()
    } catch (error) {
        const errorStr = String(error)

        let errorMessage = 'Unknown error'

        if (errorStr.includes('GamePathNotSet')) {
            errorMessage = 'Game path not set'
        } else if (errorStr.includes('InvalidBepInExArchive')) {
            errorMessage = 'Invalid BepInEx archive'
        } else if (errorStr.includes('BepInExAlreadyInstalled')) {
            errorMessage = 'BepInEx is already installed'
        } else if (errorStr.includes('DownloadFailed')) {
            errorMessage = `Download failed: ${errorStr}`
        } else if (errorStr.includes('ExtractionFailed')) {
            errorMessage = `Extraction failed: ${errorStr}`
        }

        logs.push({
            level: 'error',
            scope: 'BepInEx',
            message: `Error installing BepInEx: ${errorMessage}`,
            timestamp: new Date().toISOString(),
        })
    }
}

async function installBepInExFromArchive(filePath: string) {
    console.log('Installing BepInEx from archive:', filePath)
    try {
        logs.push({
            level: 'info',
            scope: 'BepInEx',
            message: `Starting BepInEx installation from archive: ${filePath}`,
            timestamp: new Date().toISOString(),
        })

        await invoke('install_bepinex_from_archive', { archivePath: filePath })

        logs.push({
            level: 'success',
            scope: 'BepInEx',
            message: `BepInEx installed successfully from archive.`,
            timestamp: new Date().toISOString(),
        })

        await initialize()
    } catch (error) {
        const errorStr = String(error)

        console.error('Error installing BepInEx from archive:', error)

        let errorMessage = 'Unknown error'

        if (errorStr.includes('GamePathNotSet')) {
            errorMessage = 'Game path not set'
        } else if (errorStr.includes('InvalidBepInExArchive')) {
            errorMessage = 'Invalid BepInEx archive'
        } else if (errorStr.includes('BepInExAlreadyInstalled')) {
            errorMessage = 'BepInEx is already installed'
        } else if (errorStr.includes('DownloadFailed')) {
            errorMessage = `Download failed: ${errorStr}`
        } else if (errorStr.includes('ExtractionFailed')) {
            errorMessage = `Extraction failed: ${errorStr}`
        }

        logs.push({
            level: 'error',
            scope: 'BepInEx',
            message: `Error installing BepInEx: ${errorMessage}`,
            timestamp: new Date().toISOString(),
        })
    }
}

async function promptForBepInExArchive() {
    const selected = await open({
        title: "Select BepInEx Archive File",
        multiple: false,
        filters: [
            { name: 'Archive Files', extensions: ['zip', 'rar', '7z'] }
        ]
    })

    if (selected && typeof selected === 'string') {
        await installBepInExFromArchive(selected)
    }
}

async function promptForBrownDustXArchive() {
    const selected = await open({
        title: "Select BrownDustX Archive File",
        multiple: false,
        filters: [
            { name: 'Archive Files', extensions: ['zip', 'rar', '7z'] }
        ]
    })

    if (selected && typeof selected === 'string') {
        await installBrownDustXFromArchive(selected)
    }
}

async function promptForConfigurationManagerArchive() {
    const selected = await open({
        title: "Select Configuration Manager Archive File",
        multiple: false,
        filters: [
            { name: 'Archive Files', extensions: ['zip', 'rar', '7z'] }
        ]
    })

    if (selected && typeof selected === 'string') {
        await installConfigurationManagerFromArchive(selected)
    }
}

async function installBrownDustXFromArchive(filePath: string) {
    console.log('Installing BrownDustX from archive:', filePath)
    try {
        logs.push({
            level: 'info',
            scope: 'BrownDustX',
            message: `Starting BrownDustX installation from archive: ${filePath}`,
            timestamp: new Date().toISOString(),
        })

        // maybe return version?
        await invoke('install_browndustx_from_archive', { path: filePath })

        logs.push({
            level: 'success',
            scope: 'BrownDustX',
            message: `BrownDustX installed successfully from archive.`,
            timestamp: new Date().toISOString(),
        })

        await initialize()
    } catch (error) {
        console.error('Error installing BrownDustX from archive:', error)
        let errorStr = String(error)
        // pub enum InstallBrownDustXError {
        //     GamePathNotSet,
        //     InvalidBrownDustXArchive,
        //     BepInExNotInstalled,
        //     BrownDustXAlreadyInstalled,
        //     ExtractionFailed(String),
        //     ArchiveNotFound(String),
        // }

        if (errorStr.includes('GamePathNotSet')) {
            errorStr = 'Game path not set'
        } else if (errorStr.includes('InvalidBrownDustXArchive')) {
            errorStr = 'Invalid BrownDustX archive'
        } else if (errorStr.includes('BepInExNotInstalled')) {
            errorStr = 'BepInEx is not installed. Please install BepInEx first.'
        } else if (errorStr.includes('BrownDustXAlreadyInstalled')) {
            errorStr = 'BrownDustX is already installed'
        } else if (errorStr.includes('ExtractionFailed')) {
            errorStr = `Extraction failed: ${errorStr}`
        } else if (errorStr.includes('ArchiveNotFound')) {
            errorStr = `Archive not found: ${errorStr}`
        }

        logs.push({
            level: 'error',
            scope: 'BrownDustX',
            message: `Error installing BrownDustX from archive: ${errorStr}.`,
            timestamp: new Date().toISOString(),
        })
    }
}

async function installConfigurationManagerFromArchive(filePath: string) {

    []
    console.log('Installing Configuration Manager from archive:', filePath)
    try {
        logs.push({
            level: 'info',
            scope: 'Configuration Manager',
            message: `Starting Configuration Manager installation from archive: ${filePath}`,
            timestamp: new Date().toISOString(),
        })

        // maybe return version?
        await invoke('install_configmanager_from_archive', { path: filePath })

        logs.push({
            level: 'success',
            scope: 'Configuration Manager',
            message: `Configuration Manager installed successfully from archive.`,
            timestamp: new Date().toISOString(),
        })

        await initialize()
    } catch (error) {
        console.error('Error installing Configuration Manager from archive:', error)
        let errorStr = String(error)

        if (errorStr.includes('GamePathNotSet')) {
            errorStr = 'Game path not set'
        } else if (errorStr.includes('InvalidConfigManagerArchive')) {
            errorStr = 'Invalid Configuration Manager archive'
        } else if (errorStr.includes('ExtractionFailed')) {
            errorStr = `Extraction failed: ${errorStr}`
        } else if (errorStr.includes('ArchiveNotFound')) {
            errorStr = `Archive not found: ${errorStr}`
        }

        logs.push({
            level: 'error',
            scope: 'Configuration Manager',
            message: `Error installing Configuration Manager from archive: ${errorStr}.`,
            timestamp: new Date().toISOString(),
        })
    }
}

async function uninstallBrownDustX() {
    // confirmation fist
    const result = await confirm.confirm({
        title: t('browndustxTab.confirmations.removeBrownDustX.title'),
        message: t('browndustxTab.confirmations.removeBrownDustX.message'),
        acceptButton: { label: t('browndustxTab.confirmations.removeBrownDustX.actions.removeBrownDustX') },
        rejectButton: { label: t('common.actions.cancel') },
    })

    if (result.confirmed) {
        await invoke('uninstall_browndustx')
        await initialize()
    }
}

async function uninstallBepInEx() {
    const result = await confirm.confirm({
        title: t('browndustxTab.confirmations.removeBepInEx.title'),
        message: t('browndustxTab.confirmations.removeBepInEx.message'),
        acceptButton: { label: t('browndustxTab.confirmations.removeBepInEx.actions.removeBepInEx') },
        rejectButton: { label: t('common.actions.cancel') },
    })
    if (result.confirmed) {
        await invoke('uninstall_bepinex')
        await initialize()
    }
}

async function uninstallConfigManager() {
    const result = await confirm.confirm({
        title: t('browndustxTab.confirmations.removeConfigManager.title'),
        message: t('browndustxTab.confirmations.removeConfigManager.message'),
        acceptButton: { label: t('browndustxTab.confirmations.removeConfigManager.actions.removeConfigManager') },
        rejectButton: { label: t('common.actions.cancel') },
    })

    if (result.confirmed) {
        await invoke('uninstall_configmanager')
        await initialize()
    }
}

async function initialize(log: boolean = true) {
    await Promise.all([getBDXState(log), getBepInExState(log), getConfigManagerState(log)])
}

onMounted(async () => {
    if (settingsStore.settings.gameDirectory) {
        await initialize(false)
    }

    if (!unlistenDragDrop.value) {
        await setupDragAndDrop()
    }
})

watch(() => settingsStore.settings.gameDirectory, async (newPath) => {
    if (newPath) {
        logs.push({
            level: 'info',
            scope: 'All',
            message: `Game directory set to ${newPath}. Rechecking installations...`,
            timestamp: new Date().toISOString(),
        })

        await initialize()
    } else {
        logs.push({
            level: 'warn',
            scope: 'All',
            message: `Game directory unset. Please set the game directory to manage BepInEx and BrownDustX installations.`,
            timestamp: new Date().toISOString(),
        })

        bepInExState.value = { status: BepInExStatus.NOT_INSTALLED, version: null }
        brownDustXState.value = { status: BrownDustXStatus.NOT_INSTALLED, version: null }
        configurationManagerState.value = { status: ConfigurationManagerStatus.NOT_INSTALLED, version: null }
    }
})

useHeader({
    title: computed(() => t('browndustxTab.title')),
    buttons: [
        {
            label: computed(() => t('common.actions.refresh')),
            icon: RefreshCcw,
            class: "flex justify-center items-center gap-1 mt-1  px-4 py-1 rounded-sm transition-colors duration-200 border border-interactive-border cursor-pointer",
            action: async () => {
                await initialize(true)
            }
        }
    ]
})

const unlistenDragDrop = ref<(() => void) | null>(null)

async function setupDragAndDrop() {
    unlistenDragDrop.value = await listen<{
        paths: string[]
    }>("tauri://drag-drop", async (event) => {
        const paths = event.payload?.paths as string[]

        for (const path of paths) {
            if (path.toLowerCase().endsWith('.zip') || path.toLowerCase().endsWith('.rar') || path.toLowerCase().endsWith('.7z')) {
                if (path.toLowerCase().includes('bepinex')) {
                    await installBepInExFromArchive(path)
                    continue
                } else if (path.toLowerCase().includes('browndustx') || path.toLowerCase().includes('bdx')) {
                    await installBrownDustXFromArchive(path)
                    continue
                } else if (path.toLowerCase().includes('configurationmanager') || path.toLowerCase().includes('configmanager')) {
                    await installConfigurationManagerFromArchive(path)
                    continue
                }

                const result = await invoke<'BEPINEX' | 'BROWNDUSTX' | 'CONFIGMANAGER' | null>('determine_archive_type', { path: path })

                if (result === 'BEPINEX') {
                    await installBepInExFromArchive(path)
                } else if (result === 'BROWNDUSTX') {
                    await installBrownDustXFromArchive(path)
                } else if (result === 'CONFIGMANAGER') {
                    await installConfigurationManagerFromArchive(path)
                } else {
                    console.warn('Could not determine archive type for dropped file:', path)
                    logs.push({
                        level: 'warn',
                        scope: 'All',
                        message: `Could not determine if the archive is for BepInEx or BrownDustX: ${path}. Please rename the file to include 'BepInEx' or 'BrownDustX' in its name.`,
                        timestamp: new Date().toISOString(),
                    })
                }
            } else {
                console.warn('Unsupported file type dropped:', path)
                logs.push({
                    level: 'warn',
                    scope: 'All',
                    message: `Unsupported file type dropped: ${path}`,
                    timestamp: new Date().toISOString(),
                })
            }
        }
    })
}

onBeforeUnmount(() => {
    if (unlistenDragDrop.value) {
        unlistenDragDrop.value()
        unlistenDragDrop.value = null
    }
})

const logsContainer = useTemplateRef<HTMLElement | null>('logsContainer')

watch(logs, async () => {
    await nextTick()
    if (logsContainer.value) {
        logsContainer.value.scrollTop = logsContainer.value.scrollHeight
    }
}, { deep: true, flush: 'post' })

function getStatusMessage(status: BepInExStatus | BrownDustXStatus | ConfigurationManagerStatus) {
    switch (status) {
        case BepInExStatus.INSTALLED:
        case BrownDustXStatus.INSTALLED:
        case ConfigurationManagerStatus.INSTALLED:
            return t("browndustxTab.status.installed")
        case BepInExStatus.NOT_INSTALLED:
        case BrownDustXStatus.NOT_INSTALLED:
        case ConfigurationManagerStatus.NOT_INSTALLED:
            return t("browndustxTab.status.notInstalled")
        case BrownDustXStatus.BEPINEX_MISSING:
        case ConfigurationManagerStatus.BEPINEX_MISSING:
            return t("browndustxTab.status.bepinexMissing")
        default:
            return status
    }
}

function isInstalled(status: BepInExStatus | BrownDustXStatus | ConfigurationManagerStatus) {
    return status === BepInExStatus.INSTALLED || status === BrownDustXStatus.INSTALLED || status === ConfigurationManagerStatus.INSTALLED
}
</script>

<template>
    <div class="text-primary h-full min-h-0 flex flex-col overflow-x-auto">
        <BepInEx v-model:show="isBepInExDialogOpen" @close="isBepInExDialogOpen = false"
            @version-selected="installBepInExFromUrl" />

        <div class="flex flex-col flex-1 p-4 min-h-0 gap-2">
            <div class="flex gap-2.5 p-3 rounded bg-danger/10 text-sm">
                <TriangleAlert class="w-4 h-4 shrink-0 mt-0.5 text-danger" />
                <div class="flex flex-col gap-0.5">
                    <p class="font-medium text-danger">{{ $t('browndustxTab.alerts.danger') }}</p>
                    <p class="text-secondary">{{ $t('browndustxTab.alerts.dangerMessage') }}</p>
                </div>
            </div>
            <div class="flex gap-2.5 p-3 rounded bg-warning/10 text-sm">
                <TriangleAlert class="w-4 h-4 shrink-0 mt-0.5 text-warning" />
                <div class="flex flex-col gap-0.5">
                    <p class="font-medium text-warning">{{ $t('browndustxTab.alerts.caution') }}</p>
                    <p class="text-secondary">{{ $t('browndustxTab.alerts.cautionMessage') }}</p>
                </div>
            </div>

            <div>
                <h3 class="text-secondary uppercase text-xs font-semibold tracking-wider mb-1">
                    {{ $t('browndustxTab.labels.frameworkAndPlugins') }}
                </h3>

                <section class="mb-2">
                    <p v-if="!settingsStore.settings.gameDirectory" class="text-error text-sm">
                        {{ t("browndustx.messages.gameDirectoryNotSet") }}
                    </p>
                    <p v-else class="text-secondary text-xs flex items-center gap-1.5">
                        <span>{{ t("browndustxTab.labels.gameDirectory") }}</span>
                        <span class="text-primary font-mono">{{ settingsStore.settings.gameDirectory }}</span>
                    </p>
                </section>

                <div class="flex flex-col">
                    <!-- BepInEx -->
                    <div
                        class="grid grid-cols-[1fr_auto] lg:grid-cols-[2fr_100px_120px_1fr] gap-2 lg:gap-4 items-center px-3 py-2.5 border-t border-border">
                        <div class="min-w-0">
                            <div class="flex gap-2">
                                <span class="text-sm text-primary">BepInEx</span>
                                <span v-if="bepInExState.version" class="inline hidden:inline text-sm"
                                    :class="bepInExState.version ? 'text-secondary' : 'text-secondary'">v{{
                                        bepInExState.version || '-' }}</span>
                            </div>
                            <p class="text-xs text-secondary truncate"
                                :title="$t('browndustxTab.descriptions.bepinex')">{{
                                    $t('browndustxTab.descriptions.bepinex') }}</p>
                        </div>
                        <span class="hidden lg:inline text-sm"
                            :class="bepInExState.version ? 'text-primary' : 'text-secondary'">{{ bepInExState.version ||
                                '-' }}</span>
                        <div class="hidden lg:flex items-center gap-1">
                            <span class="text-xs"
                                :class="isInstalled(bepInExState.status) ? 'text-success' : 'text-secondary'">{{
                                    getStatusMessage(bepInExState.status) }}</span>
                        </div>
                        <div class="flex gap-1 justify-end">
                            <Button v-if="bepInExState.status === BepInExStatus.NOT_INSTALLED" variant="text"
                                :label="$t('browndustxTab.actions.installFromGithub')" :icon="Github"
                                label-class="hidden lg:inline" :disabled="!settingsStore.settings.gameDirectory"
                                @click="openBepInExDialog" />
                            <Button v-if="bepInExState.status === BepInExStatus.NOT_INSTALLED" variant="text"
                                :label="$t('browndustxTab.actions.selectFile')" :icon="Upload"
                                label-class="hidden lg:inline" :disabled="!settingsStore.settings.gameDirectory"
                                @click="promptForBepInExArchive" />
                            <Button v-if="bepInExState.status === BepInExStatus.INSTALLED && bepInExState.can_remove"
                                variant="text" :label="$t('browndustxTab.actions.remove')" :icon="Trash2"
                                label-class="hidden lg:inline" @click="uninstallBepInEx" />
                        </div>
                    </div>

                    <!-- BrownDustX -->
                    <div
                        class="grid grid-cols-[1fr_auto] lg:grid-cols-[2fr_100px_120px_1fr] gap-2 lg:gap-4 items-center px-3 py-2.5 border-t border-border">
                        <div class="min-w-0">
                            <div class="flex items-center gap-1.5">
                                <TriangleAlert v-if="brownDustXState.status === BrownDustXStatus.BEPINEX_MISSING"
                                    class="w-4 h-4 text-warning" />
                                <span class="text-sm text-primary">BrownDustX</span>
                                <span v-if="brownDustXState.version" class="inline lg:hidden text-sm"
                                    :class="brownDustXState.version ? 'text-secondary' : 'text-secondary'">v{{
                                        brownDustXState.version || '-' }}</span>
                            </div>
                            <p class="text-xs text-secondary truncate"
                                :title="$t('browndustxTab.descriptions.browndustx')">{{
                                    $t('browndustxTab.descriptions.browndustx') }}</p>
                        </div>
                        <span class="hidden lg:inline text-sm"
                            :class="brownDustXState.version ? 'text-primary' : 'text-secondary'">{{
                                brownDustXState.version || '-' }}</span>
                        <div class="hidden lg:flex items-center gap-1">
                            <span class="text-xs" :class="{
                                'text-success': isInstalled(brownDustXState.status),
                                'text-warning': brownDustXState.status === BrownDustXStatus.BEPINEX_MISSING,
                                'text-secondary': brownDustXState.status === BrownDustXStatus.NOT_INSTALLED
                            }">{{ getStatusMessage(brownDustXState.status) }}</span>
                        </div>
                        <div class="flex gap-1 justify-end">
                            <Button
                                v-if="brownDustXState.status !== BrownDustXStatus.INSTALLED && bepInExState.status === BepInExStatus.INSTALLED"
                                variant="text" :label="$t('browndustxTab.actions.selectFile')" :icon="Upload"
                                label-class="hidden lg:inline"
                                :disabled="!settingsStore.settings.gameDirectory || brownDustXState.status === BrownDustXStatus.BEPINEX_MISSING"
                                @click="promptForBrownDustXArchive" />
                            <Button
                                v-if="brownDustXState.status === BrownDustXStatus.INSTALLED && brownDustXState.can_remove"
                                variant="text" :label="$t('browndustxTab.actions.remove')" :icon="Trash2"
                                label-class="hidden lg:inline" :disabled="!settingsStore.settings.gameDirectory"
                                @click="uninstallBrownDustX" />
                        </div>
                    </div>

                    <!-- Configuration Manager -->
                    <div
                        class="grid grid-cols-[1fr_auto] lg:grid-cols-[2fr_100px_120px_1fr] gap-2 lg:gap-4 items-center px-3 py-2.5 border-t border-border">
                        <div class="min-w-0">
                            <div class="flex items-center gap-1.5">
                                <TriangleAlert
                                    v-if="configurationManagerState.status === ConfigurationManagerStatus.BEPINEX_MISSING"
                                    class="w-3.5 h-3.5 text-warning" />
                                <span class="text-sm text-primary">Configuration Manager</span>
                                <span v-if="configurationManagerState.version" class="inline lg:hidden text-sm"
                                    :class="configurationManagerState.version ? 'text-secondary' : 'text-secondary'">v{{
                                        configurationManagerState.version || '-' }}
                                </span>
                            </div>
                            <p class="text-xs text-secondary truncate"
                                :title="$t('browndustxTab.descriptions.configurationManager')">{{
                                    $t('browndustxTab.descriptions.configurationManager') }}</p>
                        </div>
                        <span class="hidden lg:inline text-sm"
                            :class="configurationManagerState.version ? 'text-primary' : 'text-secondary'">{{
                                configurationManagerState.version || '-' }}</span>
                        <div class="hidden lg:flex items-center gap-1">
                            <span class="text-xs" :class="{
                                'text-success': isInstalled(configurationManagerState.status),
                                'text-warning': configurationManagerState.status === ConfigurationManagerStatus.BEPINEX_MISSING,
                                'text-secondary': configurationManagerState.status === ConfigurationManagerStatus.NOT_INSTALLED
                            }">{{ getStatusMessage(configurationManagerState.status) }}</span>
                        </div>
                        <div class="flex gap-1 justify-end">
                            <Button
                                v-if="configurationManagerState.status !== ConfigurationManagerStatus.INSTALLED && bepInExState.status === BepInExStatus.INSTALLED"
                                variant="text" :label="$t('browndustxTab.actions.selectFile')" :icon="Upload"
                                label-class="hidden lg:inline"
                                :disabled="!settingsStore.settings.gameDirectory || configurationManagerState.status === ConfigurationManagerStatus.BEPINEX_MISSING"
                                @click="promptForConfigurationManagerArchive" />
                            <Button
                                v-if="configurationManagerState.status === ConfigurationManagerStatus.INSTALLED && configurationManagerState.can_remove"
                                variant="text" :label="$t('browndustxTab.actions.remove')" :icon="Trash2"
                                label-class="hidden lg:inline" :disabled="!settingsStore.settings.gameDirectory"
                                @click="uninstallConfigManager" />
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex flex-col gap-1">
                <p v-if="bepInExState.status === BepInExStatus.INSTALLED && brownDustXState.status === BrownDustXStatus.NOT_INSTALLED"
                    class="text-warning flex gap-1.5 items-center text-xs">
                    <AlertTriangle class="inline w-4 h-4 shrink-0" />
                    {{ $t("browndustxTab.messages.brownDustXMissing") }}
                </p>
                <p v-if="configurationManagerState.status === ConfigurationManagerStatus.NOT_INSTALLED && brownDustXState.status === BrownDustXStatus.INSTALLED"
                    class="text-warning flex gap-1.5 items-center text-xs">
                    <AlertTriangle class="inline w-4 h-4 shrink-0" />
                    {{ $t("browndustxTab.messages.configurationManagerMissing") }}
                </p>
                <p v-if="brownDustXState.status === BrownDustXStatus.BEPINEX_MISSING"
                    class="text-warning flex gap-1.5 items-center text-xs">
                    <AlertTriangle class="inline w-4 h-4 shrink-0" />
                    {{ $t("browndustxTab.messages.bepinexMissing") }}
                </p>
            </div>

            <div v-if="settingsStore.settings.gameDirectory"
                class="p-4 rounded border-2 border-dashed border-border text-center text-sm text-secondary">
                {{ $t("browndustxTab.messages.dragAndDrop") }}
            </div>

            <div class="flex-1 flex flex-col min-h-0">
                <h3 class="text-secondary uppercase text-xs tracking-wider mb-2">{{ $t('browndustxTab.labels.logs') }}
                </h3>
                <div ref="logsContainer"
                    class="flex-1 flex flex-col text-secondary overflow-y-auto bg-bg-surface p-2 border border-border rounded min-h-0"
                    :class="{ 'items-center justify-center': logs.length === 0 }">

                    <template v-if="logs.length === 0">
                        <div class="text-xs font-mono text-secondary italic text-center py-2">
                            {{ $t('browndustxTab.messages.noLogsYet') }}
                        </div>
                    </template>

                    <template v-else>
                        <div v-for="(log, index) in logs" :key="index" class="text-xs  font-mono flex gap-1.5">
                            <span class="text-secondary/60 shrink-0">{{ new Date(log.timestamp).toLocaleTimeString()
                                }}</span>
                            <span class="shrink-0" :class="{
                                'text-success': log.level === 'success',
                                'text-info': log.level === 'info',
                                'text-warning': log.level === 'warn',
                                'text-danger': log.level === 'error',
                            }">
                                {{ log.level.toUpperCase() }}
                            </span>
                            <span class="text-secondary/80 shrink-0">[{{ log.scope }}]</span>
                            <span>{{ log.message }}</span>
                        </div>
                    </template>
                </div>
            </div>
        </div>
    </div>
</template>
