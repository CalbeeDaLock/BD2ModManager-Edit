<script setup lang="ts">
import { Minus, Square, SquareStop, X, Check, AlertTriangle, RotateCw, ScrollText, Heart, Puzzle, Database, Sparkles } from 'lucide-vue-next';

import { computed, onMounted, ref, watch, onUnmounted } from 'vue';
import { refThrottled } from '@vueuse/core';

import { getCurrentWindow } from '@tauri-apps/api/window';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { openUrl } from '@tauri-apps/plugin-opener';
import { SyncStatus, useSyncStateStore } from '../stores/syncState';
import { SyncError, SyncType } from '../composables/useSyncEvents';
import RefinedGithub from '../assets/icons/github.svg';
import SyncModal from './modals/SyncModal.vue';
import LogsModal from './modals/LogsModal.vue';
import { getVersion } from '@tauri-apps/api/app';
import { useI18n } from 'vue-i18n';
import { useLoggingStore } from '../stores/logging';

const isMaximized = ref(false)
const appWindow = getCurrentWindow()

function closeWindow() { appWindow.close() }
function minimizeWindow() { appWindow.minimize() }
function toggleMaximizeWindow() {
    isMaximized.value ? appWindow.unmaximize() : appWindow.maximize()
}

type UpdateStatus = 'checking' | 'available' | 'downloading' | 'downloaded' | 'updated' | 'error' | null

const appUpdate = ref<{
    show: boolean
    status: UpdateStatus
    newVersion: string | null
    url?: string | null
    error: string | null
}>({ show: false, status: null, newVersion: null, error: null, url: null })

const modPreviewUpdate = ref<{
    show: boolean
    status: UpdateStatus
    version: string | null
    progress: number
    error: string | null
}>({ show: false, status: null, version: null, progress: 0, error: null })

const gameDataUpdate = ref<{
    show: boolean
    status: UpdateStatus
    progress: number
    label: string | null
    error: string | null
}>({ show: false, status: null, progress: 0, label: null, error: null })


const rawmodPreviewProgress = computed(() => modPreviewUpdate.value.progress)
const rawGameDataProgress = computed(() => gameDataUpdate.value.progress)
const modPreviewProgress = refThrottled(rawmodPreviewProgress, 50, false, true)
const gameDataProgress = refThrottled(rawGameDataProgress, 50, false, true)

async function handleAppUpdateClick() {
    await openUrl(appUpdate.value.url!)
}

const unlistenFunctions = ref<UnlistenFn[]>([])
const syncStateStore = useSyncStateStore()
const showSyncBar = ref(false)

watch(() => syncStateStore.status, () => {
    if (!showSyncBar.value) showSyncBar.value = true
})

const rawSyncProgress = computed(() =>
    syncStateStore.progress.total > 0
        ? Math.round((syncStateStore.progress.current / syncStateStore.progress.total) * 100)
        : 0
)
const syncProgress = refThrottled(rawSyncProgress, 50, false, true)

async function handleSyncClick() { isSyncModalVisible.value = true }
async function handleGithubClick() { await openUrl("https://github.com/bruhnn/BD2ModManager") }
async function handleLogsClick() { isLogsModalVisible.value = true }

const isSyncModalVisible = ref(false)
const isLogsModalVisible = ref(false)
const appVersion = ref('0.0.0')
const { t } = useI18n()

function getErrorMessage(t: (key: string, params?: any) => string, error: SyncError | null | undefined): string {
    if (!error) return t('errors.unknownError')

    switch (error.type) {
        case 'SymlinkAdminRequired': return t('errors.symlinkAdminRequired')
        case 'PermissionDenied': return t('errors.permissionDenied')
        case 'DiskFull': return t('errors.diskFull')
        case 'ModPathNotFound': return t('errors.modPathNotFound', { path: error.details })
        case 'CopyFailed': return t('errors.copyFailed', { error: error.details })
        case 'SymlinkFailed': return t('errors.symlinkFailed', { error: error.details })
        case 'HardlinkFailed': return t('errors.hardlinkFailed', { error: error.details })
        case 'DirectoryCreationFailed': return t('errors.directoryCreationFailed', { error: error.details })
        case 'RemovalFailed': return t('errors.removalFailed', { error: error.details })
        default: return t('errors.unknownError', { error: JSON.stringify(error) })
    }
}
async function openGithubUser() { await openUrl("https://github.com/bruhnn") }

const loggingStore = useLoggingStore()

onMounted(async () => {
    isMaximized.value = await appWindow.isMaximized()
    const unlistenResize = await listen("tauri://resize", async () => {
        isMaximized.value = await appWindow.isMaximized()
    })
    unlistenFunctions.value.push(unlistenResize)

    appVersion.value = await getVersion()

    const appUpdateCheckStart = ref<number | null>(null)

    unlistenFunctions.value.push(await listen('update:app:checking', () => {
        loggingStore.logInfo('Checking for app updates...')
        appUpdateCheckStart.value = Date.now()  
        appUpdate.value = { show: true, status: 'checking', newVersion: null, error: null }
    }))

    unlistenFunctions.value.push(await listen('update:app:uptodate', () => {
        loggingStore.logInfo('No updates available, app is up to date.')
        const elapsed = Date.now() - (appUpdateCheckStart.value ?? Date.now())
        const remaining = Math.max(0, 1000 - elapsed)
        setTimeout(() => { appUpdate.value.show = false }, remaining) 
    }))

    unlistenFunctions.value.push(await listen('update:app:error', (e: any) => {
        const elapsed = Date.now() - (appUpdateCheckStart.value ?? Date.now())
        const remaining = Math.max(0, 1000 - elapsed)
        setTimeout(() => {
            appUpdate.value.show = false
            loggingStore.logError('Failed to check for updates: ' + e.payload.message)
        }, remaining)
    }))

    unlistenFunctions.value.push(await listen('update:modPreview:checking', () => {
        loggingStore.logInfo('Checking for Mod Preview updates...')
        modPreviewUpdate.value = { show: true, status: 'checking', version: null, progress: 0, error: null }
    }))
    unlistenFunctions.value.push(await listen('update:modPreview:downloading', (e: any) => {
        loggingStore.logInfo(`Downloading Mod Preview v${e.payload.version}... Progress: ${e.payload.progress}%`)
        modPreviewUpdate.value = { show: true, status: 'downloading', version: e.payload.version, progress: e.payload.progress, error: null }
    }))
    unlistenFunctions.value.push(await listen('update:modPreview:downloaded', (e: any) => {
        loggingStore.logInfo(`Mod Preview v${e.payload.version} updated successfully!`)
        modPreviewUpdate.value.status = 'downloaded'; modPreviewUpdate.value.version = e.payload
        setTimeout(() => { modPreviewUpdate.value.show = false }, 4000)
    }))
    unlistenFunctions.value.push(await listen('update:modPreview:error', (e: any) => {
        loggingStore.logError('Failed to update Mod Preview: ' + e.payload.message)
        modPreviewUpdate.value.status = 'error'; modPreviewUpdate.value.error = e.payload.message
    }))

    unlistenFunctions.value.push(await listen('update:gameData:checking', (e: any) => {
        loggingStore.logInfo(`Checking for game data updates... (${e.payload})`)
        gameDataUpdate.value = { show: true, status: 'checking', progress: 0, label: e.payload, error: null }
    }))
    unlistenFunctions.value.push(await listen('update:gameData:updating', (e: any) => {
        loggingStore.logInfo(`Downloading game data... (${e.payload.label}) Progress: ${e.payload.progress}%`)
        gameDataUpdate.value = { show: true, status: 'downloading', progress: e.payload.progress, label: e.payload.label, error: null }
    }))
    unlistenFunctions.value.push(await listen('update:gameData:updated', () => {
        loggingStore.logInfo('Game data updated successfully!')
        gameDataUpdate.value.status = 'updated'
        gameDataUpdate.value.label = 'Game data up to date'
        setTimeout(() => { gameDataUpdate.value.show = false }, 4000)
    }))
    unlistenFunctions.value.push(await listen('update:gameData:error', (e: any) => {
        loggingStore.logError('Failed to update game data: ' + e.payload.message)
        gameDataUpdate.value.status = 'error'
        gameDataUpdate.value.error = e.payload.message
    }))
})

onUnmounted(() => {
    unlistenFunctions.value.forEach(fn => fn())
})
</script>

<template>
    <div class="flex select-none bg-bg-deep border-b border-bg-surface overflow-hidden min-h-10 h-10 shrink-0 sticky text-primary justify-between transition-[max-height] duration-300 ease-out"
        data-tauri-drag-region>

        <div class="flex min-w-0 items-center gap-2.5 px-2 py-1 justify-center shrink-0" data-tauri-drag-region>
            <span class="font-mono truncate font-bold text-lg text-primary select-none" data-tauri-drag-region>
                Mod Manager
            </span>
            <span class="text-xs font-semibold text-primary flex gap-2 items-center justify-center">
                <Heart class="inline w-3.5 h-3.5 text-accent-primary" />
                v{{ appVersion }} by
                <span class="font-mono cursor-pointer
                    bg-linear-to-r from-accent-primary via-accent-primary/60 to-accent-primary
                    bg-size-[200%_100%] bg-clip-text text-transparent
                    animate-sweep hover:text-accent-primary transition-colors" @click="openGithubUser">
                    @bruhnn
                </span>
            </span>
            <transition name="slide-fade">
                <div v-if="appUpdate.show" class="group relative flex items-center gap-1.5 rounded-md shrink-0"
                    :class="appUpdate.status === 'available' ? 'cursor-pointer' : ''"
                    @click="appUpdate.status === 'available' ? handleAppUpdateClick() : undefined">

                    <div v-if="appUpdate.status === 'available'"
                        class="w-full h-full absolute inset-0 z-10 rounded-md group-hover:underline transition-colors pointer-events-none" />

                    <div class="flex items-center gap-1.5 relative z-10 min-w-0">
                        <RotateCw v-if="appUpdate.status === 'checking'"
                            class="w-3.5 h-3.5 shrink-0 animate-spin text-primary/40" />
                        <Sparkles v-else-if="appUpdate.status === 'available'"
                            class="w-3.5 h-3.5 shrink-0 text-accent-primary " />

                        <span class="text-xs font-mono truncate">
                            <span v-if="appUpdate.status === 'checking'" class="text-secondary hidden md:inline">
                                {{ $t('titlebar.appUpdate.checking') }}
                            </span>
                            <span v-else-if="appUpdate.status === 'available'"
                                class="text-accent-primary font-semibold group-hover:underline">
                                {{ $t('titlebar.appUpdate.available', { version: appUpdate.newVersion }) }}
                            </span>
                        </span>
                    </div>
                </div>
            </transition>
        </div>

        <div class="flex gap-1 md:gap-2 min-w-0 overflow-hidden">
            <transition name="slide-fade">
                <div v-if="modPreviewUpdate.show"
                    class="relative flex items-center gap-1.5 px-2 my-1.5 rounded-md shrink-0">
                    <div class="flex items-center gap-1.5 relative z-10 min-w-0">
                        <Puzzle v-if="modPreviewUpdate.status === 'downloading'"
                            class="w-4 h-4 shrink-0 text-accent-primary animate-pulse" />
                        <Check v-else-if="modPreviewUpdate.status === 'downloaded'"
                            class="w-4 h-4 shrink-0 text-success" />

                        <span class="text-xs font-mono truncate max-w-60">
                            <span v-if="modPreviewUpdate.status === 'downloading'">
                                <span class="hidden sm:inline">
                                    {{ $t('titlebar.modPreview.downloading', { version: modPreviewUpdate.version }) }}
                                </span>
                                <span class="sm:hidden">
                                    {{ $t('titlebar.modPreview.downloadingShort') }}
                                </span>
                            </span>
                            <span v-else-if="modPreviewUpdate.status === 'error'" class="text-danger">
                                <span class="hidden sm:inline">
                                    {{ $t('titlebar.modPreview.updateFailed') }}
                                </span>
                                <span class="sm:hidden">
                                    {{ $t('titlebar.modPreview.updateFailedShort') }}
                                </span>
                            </span>
                            <span v-else-if="modPreviewUpdate.status === 'downloaded'" class="text-primary">
                                <span class="hidden sm:inline">
                                    {{ $t('titlebar.modPreview.updated', { version: modPreviewUpdate.version }) }}
                                </span>
                                <span class="sm:hidden">
                                    {{ $t('titlebar.modPreview.updatedShort') }}
                                </span>
                            </span>
                        </span>

                        <div v-if="modPreviewUpdate.status === 'downloading'"
                            class="hidden sm:block w-14 md:w-20 h-2 bg-interactive-bg rounded-full overflow-hidden shrink-0">
                            <div class="h-full bg-accent-primary rounded-full"
                                :class="modPreviewProgress === 0 ? 'transition-none' : 'transition-all duration-150 ease-out'"
                                :style="{ width: `${modPreviewProgress}%` }" />
                        </div>
                        <span v-if="modPreviewUpdate.status === 'downloading'"
                            class="text-xs font-mono text-primary/40 shrink-0 hidden md:inline tabular-nums">
                            {{ Math.round(modPreviewProgress) }}%
                        </span>
                    </div>
                </div>
            </transition>

            <div v-if="modPreviewUpdate.show && gameDataUpdate.show" class="w-px bg-border my-2.5 shrink-0" />

            <transition name="slide-fade">
                <div v-if="gameDataUpdate.show"
                    class="relative flex items-center gap-1.5 px-2 my-1.5 rounded-md shrink-0">

                    <div class="flex items-center gap-1.5 relative z-10 min-w-0">
                        <Database v-if="gameDataUpdate.status === 'downloading'"
                            class="w-4 h-4 shrink-0 text-accent-primary animate-pulse" />
                        <RotateCw v-else-if="gameDataUpdate.status === 'checking'"
                            class="w-4 h-4 shrink-0 text-primary/40 animate-spin" />
                        <Check v-else-if="gameDataUpdate.status === 'updated'" class="w-4 h-4 shrink-0 text-success" />
                        <AlertTriangle v-else-if="gameDataUpdate.status === 'error'"
                            class="w-4 h-4 shrink-0 text-danger" />

                        <span class="text-xs font-mono truncate max-w-40">
                            <span v-if="gameDataUpdate.status === 'downloading'">
                                <span class="hidden sm:inline">
                                    {{ gameDataUpdate.label ?? $t('titlebar.gameData.downloading') }}
                                </span>
                                <span class="sm:hidden">
                                    {{ $t('titlebar.gameData.downloadingShort') }}
                                </span>
                            </span>
                            <span v-else-if="gameDataUpdate.status === 'checking'">
                                <span class="hidden sm:inline">
                                    {{ gameDataUpdate.label ?? $t('titlebar.gameData.checking') }}
                                </span>
                                <span class="sm:hidden">
                                    {{ $t('titlebar.gameData.downloadingShort') }}
                                </span>
                            </span>
                            <span v-else-if="gameDataUpdate.status === 'updated'" class="text-primary">
                                <span class="hidden sm:inline">{{ gameDataUpdate.label }}</span>
                                <span class="sm:hidden">
                                    {{ $t('titlebar.gameData.updatedShort') }}
                                </span>
                            </span>
                            <span v-else-if="gameDataUpdate.status === 'error'" class="text-danger">
                                <span class="hidden sm:inline">
                                    {{ gameDataUpdate.error ?? $t('titlebar.gameData.error') }}
                                </span>
                                <span class="sm:hidden">
                                    {{ $t('titlebar.gameData.errorShort') }}
                                </span>
                            </span>
                        </span>

                        <div v-if="gameDataUpdate.status === 'downloading'"
                            class="hidden sm:block w-14 md:w-20 h-2 bg-interactive-bg rounded-full overflow-hidden shrink-0">
                            <div class="h-full bg-accent-primary rounded-full"
                                :class="gameDataProgress === 0 ? 'transition-none' : 'transition-all duration-150 ease-out'"
                                :style="{ width: `${gameDataProgress}%` }" />
                        </div>
                        <span v-if="gameDataUpdate.status === 'downloading'"
                            class="text-xs font-mono text-primary/40 shrink-0 hidden md:inline tabular-nums">
                            {{ Math.round(gameDataProgress) }}%
                        </span>
                    </div>
                </div>
            </transition>

            <div v-if="(modPreviewUpdate.show || gameDataUpdate.show) && showSyncBar"
                class="w-px bg-border my-2.5 shrink-0" />

            <div v-show="showSyncBar"
                class="group relative flex items-center justify-between gap-2 md:gap-3 mr-1 md:mr-2 py-0 px-2 my-1.5 transition-all rounded-md cursor-pointer shrink-0"
                @click="handleSyncClick">
                <div
                    class="w-full h-full absolute inset-0 z-10 rounded-md group-hover:bg-interactive-bg-hover transition-colors pointer-events-none" />

                <div class="flex items-center gap-2 relative z-10 flex-1 min-w-0">
                    <RotateCw v-if="syncStateStore.status === SyncStatus.SYNCING"
                        class="w-4 h-4 shrink-0 animate-spin text-accent-primary" />
                    <Check v-else-if="syncStateStore.status === SyncStatus.COMPLETED"
                        class="w-4 h-4 shrink-0 text-success" />
                    <AlertTriangle v-else-if="syncStateStore.status === SyncStatus.FAILED"
                        class="w-4 h-4 shrink-0 text-danger" />

                    <span v-if="syncStateStore.status === SyncStatus.FAILED"
                        class="flex-1 min-w-0 text-sm text-danger truncate">
                        {{ getErrorMessage(t, syncStateStore.error) }}
                    </span>
                    <span v-else-if="syncStateStore.status === SyncStatus.SYNCING"
                        class="text-sm font-mono truncate max-w-30 md:max-w-50">
                        {{ $t('titlebar.sync.applying', { modName: syncStateStore.lastSyncedMod?.modName }) }}
                    </span>
                    <span v-else class="text-sm font-mono truncate max-w-25 md:max-w-50">
                        <span v-if="syncStateStore.type === SyncType.Sync">
                            {{ $t('titlebar.sync.syncSuccess') }}
                        </span>
                        <span v-else>
                            {{ $t('titlebar.sync.unsyncSuccess') }}
                        </span>
                    </span>
                </div>

                <div v-if="syncStateStore.status == SyncStatus.SYNCING"
                    class="hidden sm:block w-16 md:w-24 h-2.5 bg-interactive-bg rounded-full overflow-hidden relative z-10 shrink-0">
                    <div class="h-full bg-accent-primary rounded-full"
                        :class="syncProgress === 0 ? 'transition-none' : 'transition-all duration-150 ease-out'"
                        :style="{ width: `${syncProgress}%` }" />
                </div>

                <button
                    v-if="syncStateStore.status === SyncStatus.COMPLETED || syncStateStore.status === SyncStatus.FAILED"
                    @click.stop="showSyncBar = false"
                    class="text-interactive-primary hover:text-accent-primary relative z-20 transition-all flex items-center justify-center shrink-0">
                    <X class="w-[1.25em] h-[1.25em] cursor-pointer" />
                </button>
            </div>

            <div class="flex items-center shrink-0 gap-2 md:gap-4">
                <button class="flex items-center gap-1 md:gap-2 text-primary fill-primary cursor-pointer group"
                    @click="handleGithubClick">
                    <RefinedGithub class="w-[1.25em] h-[1.25em] group-hover:fill-accent-primary transition-colors" />
                    <span
                        class="font-mono hidden lg:inline font-bold text-sm group-hover:text-accent-primary transition-colors">
                        {{ $t('titlebar.actions.github') }}
                    </span>
                </button>
                <button class="flex items-center gap-1 md:gap-2 text-primary fill-primary cursor-pointer group"
                    @click="handleLogsClick">
                    <ScrollText class="w-[1.25em] h-[1.25em] group-hover:text-accent-primary transition-colors" />
                    <span
                        class="font-mono hidden lg:inline font-bold text-sm group-hover:text-accent-primary transition-colors">
                        {{ $t('titlebar.actions.logs') }}
                    </span>
                </button>
            </div>

            <div class="flex shrink-0 ml-1 md:ml-1">
                <button @click="minimizeWindow"
                    class="flex items-center justify-center px-3 md:px-4 hover:bg-interactive-bg-hover cursor-pointer transition-colors">
                    <Minus class="w-[1.25em] h-[1.25em] font-bold" />
                </button>
                <button @click="toggleMaximizeWindow"
                    class="flex items-center justify-center px-3 md:px-4 hover:bg-interactive-bg-hover cursor-pointer transition-colors">
                    <SquareStop v-if="isMaximized" class="w-[1.25em] h-[1.25em]" />
                    <Square v-else class="w-[1.25em] h-[1.25em]" />
                </button>
                <button @click="closeWindow"
                    class="flex items-center justify-center px-3 md:px-4 hover:bg-danger cursor-pointer transition-colors">
                    <X class="w-[1.5em] h-[1.5em]" />
                </button>
            </div>
        </div>
    </div>

    <Teleport to="body">
        <SyncModal :visible="isSyncModalVisible" @close="isSyncModalVisible = false" />
        <LogsModal :visible="isLogsModalVisible" @close="isLogsModalVisible = false" />
    </Teleport>
</template>

<style scoped>
.slide-fade-enter-active {
    transition: all 0.2s ease-out;
}

.slide-fade-leave-active {
    transition: all 0.15s ease-in;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
    opacity: 0;
    transform: translateY(-4px);
}
</style>