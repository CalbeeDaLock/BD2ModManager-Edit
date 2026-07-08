<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { open } from '@tauri-apps/plugin-dialog';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { FlaskConical, RefreshCcw, Check, Link2, FolderInput, Search } from 'lucide-vue-next';

import Modal from '../../../components/common/Modal.vue';
import Button from '../../../components/common/Button.vue';
import Input from '../../../components/common/Input.vue';
import Select from '../../../components/common/Select.vue';
import Checkbox from '../../../components/common/Checkbox.vue';

import { MatchedMod, OrphanMod, useModsStore } from '../../../stores/mods';
import { useProfilesStore } from '../../../stores/profiles';
import { useSettingsStore } from '../../../stores/settings';
import { useLoggingStore } from '../../../stores/logging';
import { useNotificationStore } from '../../../stores/notification';
import { useConfirm } from '../../../plugins/ConfirmService';
import { getErrorMessage } from '../../../utils/errors';

const { t } = useI18n();
const modsStore = useModsStore();
const profilesStore = useProfilesStore();
const settingsStore = useSettingsStore();
const loggingStore = useLoggingStore();
const notificationStore = useNotificationStore();
const confirm = useConfirm();

const show = ref(false);

// An orphan with the destination the user picked (or resolved to a new import).
interface OrphanRow extends OrphanMod {
    // staging name once imported (empty until imported)
    stagingName: string;
    // whether the user chose to import this orphan
    selected: boolean;
    // resolved after a successful import
    imported: boolean;
}

const isScanning = ref(false);
const isApplying = ref(false);
const hasScanned = ref(false);

// Live scan progress emitted by the backend.
const scanScanned = ref(0);
const scanTotal = ref(0);

const matched = ref<MatchedMod[]>([]);
const orphans = ref<OrphanRow[]>([]);

// Matches that the user wants mapped into their active profile (checked = enable).
const matchSelected = ref<Set<string>>(new Set());

// Search filter over both sections (matches active name, staging name, path).
const search = ref('');

// Cache the scan result at module scope so re-opening the modal doesn't rescan.
// Cleared only by an explicit rescan.
interface ScanCache {
    matched: MatchedMod[];
    orphans: OrphanRow[];
    matchSelected: string[];
}
let scanCache: ScanCache | null = null;

function snapshotToCache() {
    scanCache = {
        matched: matched.value.map(m => ({ ...m })),
        orphans: orphans.value.map(o => ({ ...o })),
        matchSelected: [...matchSelected.value]
    };
}

function restoreFromCache(cache: ScanCache) {
    matched.value = cache.matched.map(m => ({ ...m }));
    orphans.value = cache.orphans.map(o => ({ ...o }));
    matchSelected.value = new Set(cache.matchSelected);
    hasScanned.value = true;
}

let unlistenProgress: UnlistenFn | null = null;

async function scan() {
    isScanning.value = true;
    scanScanned.value = 0;
    scanTotal.value = 0;

    // Listen for backend progress so we can show how many mods were scanned.
    unlistenProgress = await listen<{ scanned: number; total: number }>(
        'recognize-progress',
        (event) => {
            scanScanned.value = event.payload.scanned;
            scanTotal.value = event.payload.total;
        }
    );

    try {
        const result = await modsStore.scanActiveMods();
        matched.value = result.matched;
        orphans.value = result.orphans.map(o => ({
            ...o,
            stagingName: '',
            selected: true,
            imported: false
        }));
        // default: map every matched mod into the profile. Keyed by the unique
        // activePath (not stagingName) so multiple active mods that resolve to
        // the same staging source each keep their own tick.
        matchSelected.value = new Set(result.matched.map(m => m.activePath));
        hasScanned.value = true;
        snapshotToCache();
    } catch (error) {
        loggingStore.logError('Recognize scan failed:', error);
        notificationStore.add({
            severity: 'error',
            closable: true,
            title: t('modsTab.recognizeMods.notifications.scanFailed.title'),
            message: getErrorMessage(t, error),
            duration: 6000
        });
    } finally {
        isScanning.value = false;
        unlistenProgress?.();
        unlistenProgress = null;
    }
}

// Explicit rescan: throw away the cache and scan fresh.
function rescan() {
    scanCache = null;
    reset();
    scan();
}

function toggleMatch(name: string) {
    const set = new Set(matchSelected.value);
    if (set.has(name)) set.delete(name);
    else set.add(name);
    matchSelected.value = set;
}

function reset() {
    hasScanned.value = false;
    matched.value = [];
    orphans.value = [];
    matchSelected.value = new Set();
    search.value = '';
}

// --- Search filtering ------------------------------------------------------

const filteredMatched = computed(() => {
    const q = search.value.trim().toLowerCase();
    if (!q) return matched.value;
    return matched.value.filter(m =>
        m.activeName.toLowerCase().includes(q) ||
        m.stagingName.toLowerCase().includes(q)
    );
});

const filteredOrphans = computed(() => {
    const q = search.value.trim().toLowerCase();
    if (!q) return orphans.value;
    return orphans.value.filter(o =>
        o.activeName.toLowerCase().includes(q) ||
        o.activePath.toLowerCase().includes(q)
    );
});

const selectedOrphanCount = computed(() => orphans.value.filter(o => o.selected && !o.imported).length);
const selectedMatchCount = computed(() => matchSelected.value.size);

// Apply: map matched mods into the active profile, then (optionally) import the
// selected orphans. All imports go into a single destination folder the user
// picks once — an include/exclude choice gates that step.
async function apply() {
    isApplying.value = true;
    try {
        // 1. Map matched mods into the active profile by enabling them. Selection
        // is keyed by activePath; dedupe the resulting staging names to enable.
        const toEnable = [...new Set(
            matched.value
                .filter(m => matchSelected.value.has(m.activePath))
                .map(m => m.stagingName)
        )];
        if (toEnable.length > 0) {
            await modsStore.enableMods(toEnable);
        }

        // 2. Import the selected orphans, if the user opts in.
        const importedCount = await maybeImportOrphans();

        if (toEnable.length > 0 || importedCount > 0) {
            notificationStore.add({
                severity: 'success',
                closable: true,
                title: t('modsTab.recognizeMods.notifications.applySuccess.title'),
                message: t('modsTab.recognizeMods.notifications.applySuccess.message', {
                    mapped: toEnable.length,
                    imported: importedCount
                }),
                duration: 5000
            });
        }

        snapshotToCache();

        // 3. Offer to clean up the original loose copies in the game folder.
        await maybeDeleteOriginals();
    } finally {
        isApplying.value = false;
    }
}

// Ask whether to include the orphan imports. On "include", the user picks ONE
// destination folder and every selected orphan is copied there, then picks
// which profile to enable them in. Returns the number of orphans imported.
async function maybeImportOrphans(): Promise<number> {
    const toImport = orphans.value.filter(o => o.selected && !o.imported);
    if (toImport.length === 0) return 0;

    // Single popup: include vs exclude. The message reminds the user that all
    // selected orphans will be imported into one shared destination folder.
    const choice = await confirm.confirm({
        title: t('modsTab.recognizeMods.importOrphans.title'),
        message: t('modsTab.recognizeMods.importOrphans.message', { count: toImport.length }),
        acceptButton: { label: t('modsTab.recognizeMods.importOrphans.actions.include') },
        rejectButton: { label: t('modsTab.recognizeMods.importOrphans.actions.exclude') }
    });

    if (!choice.confirmed) return 0; // exclude → skip importing

    // Pick the one destination folder for every selected orphan.
    const destination = await open({
        multiple: false,
        directory: true,
        defaultPath: settingsStore.settings.stagingDirectory ?? undefined,
        title: t('modsTab.recognizeMods.importOrphans.chooseDestinationTitle')
    });
    if (!destination || typeof destination !== 'string') return 0; // cancelled

    // Ask which profile the imported mods should be enabled in.
    const targetProfileId = await pickTargetProfile();
    if (targetProfileId === null) return 0; // cancelled the profile choice

    let importedCount = 0;
    const importedNames: string[] = [];
    for (const orphan of toImport) {
        try {
            const stagingName = await modsStore.importOrphanMod(orphan.activePath, destination);
            orphan.stagingName = stagingName;
            orphan.imported = true;
            importedNames.push(stagingName);
            importedCount += 1;
        } catch (error) {
            loggingStore.logError(`Failed to import orphan "${orphan.activeName}":`, error);
            notificationStore.add({
                severity: 'error',
                closable: true,
                title: t('modsTab.recognizeMods.notifications.importFailed.title'),
                message: getErrorMessage(t, error),
                duration: 5000
            });
        }
    }

    // Enable the freshly imported orphans in the chosen profile so their profile
    // address points at the new staging source.
    if (importedNames.length > 0) {
        try {
            await modsStore.enableModsInProfile(targetProfileId, importedNames);
        } catch (error) {
            loggingStore.logError('Failed to enable imported mods in profile:', error);
        }
    }

    return importedCount;
}

// --- Target-profile picker -------------------------------------------------

const showProfilePicker = ref(false);
const profileChoice = ref<string>('');
let resolveProfileChoice: ((id: string | null) => void) | null = null;

const profileOptions = computed(() =>
    profilesStore.sortedProfiles.map(p => ({ label: p.name, value: p.id }))
);

// Opens the in-modal profile picker and resolves with the chosen id (or null on
// cancel). Defaults to the active profile.
function pickTargetProfile(): Promise<string | null> {
    profileChoice.value = profilesStore.activeProfileId ?? 'default';
    showProfilePicker.value = true;
    return new Promise((resolve) => {
        resolveProfileChoice = resolve;
    });
}

function confirmProfileChoice() {
    showProfilePicker.value = false;
    resolveProfileChoice?.(profileChoice.value || null);
    resolveProfileChoice = null;
}

function cancelProfileChoice() {
    showProfilePicker.value = false;
    resolveProfileChoice?.(null);
    resolveProfileChoice = null;
}

// After a successful recognize/import, all originals that were resolved (matched
// mods + imported orphans) are candidates for deletion from the active folder.
async function maybeDeleteOriginals() {
    const candidates: { path: string; name: string }[] = [
        ...matched.value
            .filter(m => matchSelected.value.has(m.activePath))
            .map(m => ({ path: m.activePath, name: m.activeName })),
        ...orphans.value
            .filter(o => o.imported)
            .map(o => ({ path: o.activePath, name: o.activeName }))
    ];

    if (candidates.length === 0) {
        onClose();
        return;
    }

    // First prompt: do you want to remove the originals at all?
    const first = await confirm.confirm({
        title: t('modsTab.recognizeMods.deleteOriginals.title'),
        message: t('modsTab.recognizeMods.deleteOriginals.message', { count: candidates.length }),
        acceptButton: { label: t('modsTab.recognizeMods.deleteOriginals.actions.review') },
        rejectButton: { label: t('modsTab.recognizeMods.deleteOriginals.actions.keep') }
    });

    if (!first.confirmed) {
        onClose();
        return;
    }

    // Second confirmation (destructive): explicit, spelled-out warning.
    const second = await confirm.confirm({
        title: t('modsTab.recognizeMods.deleteOriginals.confirmTitle'),
        message: t('modsTab.recognizeMods.deleteOriginals.confirmMessage', { count: candidates.length }),
        acceptButton: { label: t('modsTab.recognizeMods.deleteOriginals.actions.delete') },
        rejectButton: { label: t('common.actions.cancel') }
    });

    if (!second.confirmed) {
        onClose();
        return;
    }

    try {
        await modsStore.deleteActiveMods(candidates.map(c => c.path));
        // Remove the deleted entries from the cache so a reopen reflects reality.
        scanCache = null;
        notificationStore.add({
            severity: 'success',
            closable: true,
            title: t('modsTab.recognizeMods.notifications.deleteSuccess.title'),
            message: t('modsTab.recognizeMods.notifications.deleteSuccess.message', { count: candidates.length }),
            duration: 4000
        });
    } catch (error) {
        loggingStore.logError('Failed to delete original active mods:', error);
        notificationStore.add({
            severity: 'error',
            closable: true,
            title: t('modsTab.recognizeMods.notifications.deleteFailed.title'),
            message: getErrorMessage(t, error),
            duration: 5000
        });
    }

    onClose();
}

function onClose() {
    show.value = false;
    search.value = '';
    unlistenProgress?.();
    unlistenProgress = null;
}

defineExpose({
    open() {
        show.value = true;
        search.value = '';
        // Reuse the previous scan result if we have one; otherwise scan fresh.
        if (scanCache) {
            restoreFromCache(scanCache);
        } else {
            reset();
            scan();
        }
    }
});
</script>

<template>
    <Modal v-model:show="show" class="w-[54vw] max-h-[85vh]" @close="onClose">
        <template #header></template>
        <template #footer>
            <div class="flex p-3 justify-between items-center w-full border-t border-border-default">
                <span class="text-xs text-text-secondary">
                    {{ t('modsTab.recognizeMods.summary', {
                        matched: selectedMatchCount,
                        orphans: selectedOrphanCount
                    }) }}
                </span>
                <div class="flex gap-2">
                    <Button variant="default" :label="t('common.actions.close')" @click="onClose" />
                    <Button variant="primary" :icon="FolderInput"
                        :label="t('modsTab.recognizeMods.actions.apply')"
                        :disabled="isApplying || isScanning || (selectedMatchCount === 0 && selectedOrphanCount === 0)"
                        @click="apply" />
                </div>
            </div>
        </template>

        <div class="flex flex-col min-h-0 text-text-primary overflow-hidden">
            <div class="flex items-center gap-2 px-4 py-3 border-b border-border-default shrink-0">
                <FlaskConical class="w-5 h-5 text-warning" />
                <div class="flex-1 min-w-0">
                    <h3 class="font-semibold text-base">{{ t('modsTab.recognizeMods.title') }}</h3>
                    <p class="text-xs text-text-secondary">{{ t('modsTab.recognizeMods.description') }}</p>
                </div>
                <Button :icon="RefreshCcw" :label="t('modsTab.recognizeMods.actions.rescan')" :disabled="isScanning"
                    @click="rescan" />
            </div>

            <!-- Search bar -->
            <div class="px-4 py-2 border-b border-border-default shrink-0">
                <Input v-model="search" :icon-left="Search" clearable
                    :placeholder="t('modsTab.recognizeMods.searchPlaceholder')"
                    class="w-full" :disabled="isScanning || !hasScanned" />
            </div>

            <div class="overflow-y-auto h-[24rem]">
                <div v-if="isScanning" class="text-center py-12 px-4 text-text-secondary">
                    <p class="text-sm">
                        {{ scanTotal > 0
                            ? t('modsTab.recognizeMods.scanningCount', { scanned: scanScanned, total: scanTotal })
                            : t('modsTab.recognizeMods.scanning') }}
                    </p>
                    <p class="text-xs mt-1 opacity-75">{{ t('modsTab.recognizeMods.scanPerfNote') }}</p>
                </div>

                <template v-else-if="hasScanned">
                    <div v-if="matched.length === 0 && orphans.length === 0"
                        class="text-center py-12 px-4 text-text-secondary">
                        <p class="text-sm font-medium mb-1">{{ t('modsTab.recognizeMods.empty.title') }}</p>
                        <p class="text-xs">{{ t('modsTab.recognizeMods.empty.description') }}</p>
                    </div>

                    <div v-else-if="filteredMatched.length === 0 && filteredOrphans.length === 0"
                        class="text-center py-12 px-4 text-text-secondary">
                        <p class="text-sm">{{ t('modsTab.recognizeMods.noResults') }}</p>
                    </div>

                    <!-- Matched section -->
                    <template v-if="filteredMatched.length > 0">
                        <div
                            class="flex items-center gap-2 px-4 py-2 bg-surface-dialog border-b border-border-default sticky top-0 z-10">
                            <Link2 class="w-4 h-4 text-success" />
                            <span class="text-xs font-medium text-text-secondary uppercase tracking-wide">
                                {{ t('modsTab.recognizeMods.matchedHeader') }}
                            </span>
                            <span class="text-xs text-text-secondary">{{ selectedMatchCount }}/{{ matched.length }}</span>
                        </div>
                        <div v-for="m in filteredMatched" :key="m.activePath"
                            class="flex items-center gap-3 px-4 py-2.5 border-b border-border-default cursor-pointer hover:bg-state-hover transition-colors"
                            @click="toggleMatch(m.activePath)">
                            <Checkbox :model-value="matchSelected.has(m.activePath)" class="shrink-0 pointer-events-none" />
                            <div class="flex-1 min-w-0">
                                <p class="text-sm truncate">{{ m.activeName }}</p>
                                <p class="text-xs text-text-secondary font-mono truncate flex items-center gap-1">
                                    <Link2 class="w-3 h-3 shrink-0" /> {{ m.stagingName }}
                                </p>
                            </div>
                            <Check class="w-4 h-4 text-success shrink-0" />
                        </div>
                    </template>

                    <!-- Orphans section -->
                    <template v-if="filteredOrphans.length > 0">
                        <div
                            class="flex items-center gap-2 px-4 py-2 bg-surface-dialog border-b border-border-default sticky top-0 z-10">
                            <FolderInput class="w-4 h-4 text-warning" />
                            <span class="text-xs font-medium text-text-secondary uppercase tracking-wide">
                                {{ t('modsTab.recognizeMods.orphansHeader') }}
                            </span>
                            <span class="text-xs text-text-secondary">{{ selectedOrphanCount }}/{{ orphans.length }}</span>
                        </div>
                        <div v-for="o in filteredOrphans" :key="o.activePath"
                            class="flex items-center gap-3 px-4 py-2.5 border-b border-border-default transition-colors"
                            :class="o.imported ? 'opacity-60' : 'cursor-pointer hover:bg-state-hover'"
                            @click="!o.imported && (o.selected = !o.selected)">
                            <Checkbox :model-value="o.selected" :disabled="o.imported" class="shrink-0 pointer-events-none" />
                            <div class="flex-1 min-w-0">
                                <p class="text-sm truncate">{{ o.activeName }}</p>
                                <p class="text-xs text-text-secondary font-mono truncate">{{ o.activePath }}</p>
                            </div>
                            <span v-if="o.imported"
                                class="text-xs text-success shrink-0 flex items-center gap-1">
                                <Check class="w-3 h-3" /> {{ t('modsTab.recognizeMods.importedTag') }}
                            </span>
                            <span v-else class="text-xs text-warning shrink-0">
                                {{ t('modsTab.recognizeMods.newTag') }}
                            </span>
                        </div>
                    </template>
                </template>
            </div>
        </div>

        <!-- Target-profile picker for imported orphans -->
        <div v-if="showProfilePicker"
            class="absolute inset-0 z-20 flex items-center justify-center bg-black/40">
            <div class="bg-surface-dialog border border-border-default rounded-md shadow-lg p-4 w-80">
                <h4 class="font-semibold text-sm mb-1">{{ t('modsTab.recognizeMods.chooseProfile.title') }}</h4>
                <p class="text-xs text-text-secondary mb-3">{{ t('modsTab.recognizeMods.chooseProfile.description') }}</p>
                <Select v-model="profileChoice" :options="profileOptions" class="w-full" />
                <div class="flex justify-end gap-2 mt-4">
                    <Button variant="default" :label="t('common.actions.cancel')" @click="cancelProfileChoice" />
                    <Button variant="primary" :label="t('modsTab.recognizeMods.chooseProfile.confirm')"
                        @click="confirmProfileChoice" />
                </div>
            </div>
        </div>
    </Modal>
</template>

<style scoped></style>
