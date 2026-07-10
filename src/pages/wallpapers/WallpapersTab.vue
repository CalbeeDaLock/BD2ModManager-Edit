<script setup lang="ts">
import { RefreshCcw, Pencil, Check, X, Eye } from 'lucide-vue-next';
import { computed, reactive, ref } from 'vue';
import { useI18n } from 'vue-i18n';

import { BD2Mod, BD2ModExtended, useModsStore } from '../../stores/mods';
import { usePreferencesStore } from '../../stores/preferences';
import { useLoggingStore } from '../../stores/logging';
import { useNotificationStore } from '../../stores/notification';
import { getErrorMessage } from '../../utils/errors';

import { useHeader } from '../../composables/useHeader';

import Button from '../../components/common/Button.vue';
import Input from '../../components/common/Input.vue';
import Select from '../../components/common/Select.vue';
import Checkbox from '../../components/common/Checkbox.vue';

import type { WallpaperEntry } from './types';

const { t } = useI18n();
const modsStore = useModsStore();
const preferencesStore = usePreferencesStore();
const loggingStore = useLoggingStore();
const notificationStore = useNotificationStore();

const userFilters = reactive({
    searchQuery: '',
    sortBy: 'id-asc'
});

const sortOptions = computed(() => [
    { label: t('wallpapersTab.filters.sortBy.options.idAsc'), value: 'id-asc' },
    { label: t('wallpapersTab.filters.sortBy.options.idDesc'), value: 'id-desc' },
    { label: t('wallpapersTab.filters.sortBy.options.nameAsc'), value: 'a-z' },
    { label: t('wallpapersTab.filters.sortBy.options.nameDesc'), value: 'z-a' },
    { label: t('wallpapersTab.filters.sortBy.options.modsAsc'), value: 'mods-asc' },
    { label: t('wallpapersTab.filters.sortBy.options.modsDesc'), value: 'mods-desc' },
]);

// Group wallpaper-typed mods by their stable id (the .modfile filename).
const wallpapers = computed<WallpaperEntry[]>(() => {
    const groups = new Map<string, WallpaperEntry>();

    for (const mod of modsStore.mods as readonly BD2ModExtended[]) {
        if (mod.modType?.type !== 'Wallpaper') continue;
        if (!('id' in mod.modType)) continue;
        const id = mod.modType.id;

        let entry = groups.get(id);
        if (!entry) {
            entry = {
                id,
                nickname: preferencesStore.wallpaperNicknames[id] || '',
                modsCount: 0,
                enabledCount: 0
            };
            groups.set(id, entry);
        }
        entry.modsCount += 1;
        if (mod.enabled && !mod.errors.length) entry.enabledCount += 1;
    }

    return Array.from(groups.values());
});

const totalModsCount = computed(() =>
    wallpapers.value.reduce((sum, w) => sum + w.modsCount, 0)
);
const enabledModsCount = computed(() =>
    wallpapers.value.reduce((sum, w) => sum + w.enabledCount, 0)
);

const filteredWallpapers = computed(() => {
    const search = userFilters.searchQuery.toLowerCase();
    return wallpapers.value.filter(w => {
        if (search &&
            !w.id.toLowerCase().includes(search) &&
            !w.nickname.toLowerCase().includes(search)) {
            return false;
        }
        return true;
    });
});

const sortedWallpapers = computed(() => {
    const arr = [...filteredWallpapers.value];
    arr.sort((a, b) => {
        const nameA = a.nickname || a.id;
        const nameB = b.nickname || b.id;
        switch (userFilters.sortBy) {
            case 'id-asc':
                return a.id.localeCompare(b.id, undefined, { numeric: true });
            case 'id-desc':
                return b.id.localeCompare(a.id, undefined, { numeric: true });
            case 'a-z':
                return nameA.localeCompare(nameB, undefined, { numeric: true });
            case 'z-a':
                return nameB.localeCompare(nameA, undefined, { numeric: true });
            case 'mods-asc':
                return a.modsCount - b.modsCount;
            case 'mods-desc':
                return b.modsCount - a.modsCount;
            default:
                return a.id.localeCompare(b.id, undefined, { numeric: true });
        }
    });
    return arr;
});

// Inline nickname editing.
const editingId = ref<string | null>(null);
const editingValue = ref('');

function startEditing(entry: WallpaperEntry) {
    editingId.value = entry.id;
    // Pre-fill with the current nickname, or the id when unnamed so editing
    // starts from the shown name instead of an empty field.
    editingValue.value = entry.nickname || entry.id;
}

function cancelEditing() {
    editingId.value = null;
    editingValue.value = '';
}

function saveEditing(entry: WallpaperEntry) {
    const value = editingValue.value.trim();
    const map = { ...preferencesStore.wallpaperNicknames };
    if (value) {
        map[entry.id] = value;
    } else {
        delete map[entry.id];
    }
    preferencesStore.wallpaperNicknames = map;
    cancelEditing();
}

// Mods belonging to a wallpaper id, for enabling/previewing.
function modsForWallpaper(id: string): BD2Mod[] {
    return (modsStore.mods as readonly BD2ModExtended[]).filter(mod =>
        mod.modType?.type === 'Wallpaper' && 'id' in mod.modType && mod.modType.id === id
    );
}

function toggleMod(mod: BD2Mod) {
    if (mod.enabled) {
        modsStore.disableMods([mod.name]);
    } else {
        modsStore.enableMods([mod.name]);
    }
}

async function previewMod(mod: BD2Mod) {
    modsStore.previewMod(mod.name).catch((error) => {
        notificationStore.add({
            severity: 'error',
            closable: true,
            title: t('modsTab.errors.modPreview.title'),
            message: getErrorMessage(t, error),
            duration: 5000
        });
        loggingStore.logError('Error previewing wallpaper mod:', error);
    });
}

// Which wallpaper card is expanded to show its individual mods.
const expandedId = ref<string | null>(null);
function toggleExpanded(id: string) {
    expandedId.value = expandedId.value === id ? null : id;
}

function refreshData() {
    modsStore.discoverMods();
}

useHeader({
    title: t('wallpapersTab.title'),
    subtitle: computed(() =>
        t('wallpapersTab.subtitle', {
            enabledModsCount: enabledModsCount.value,
            totalModsCount: totalModsCount.value
        })
    ),
    buttons: [
        { icon: RefreshCcw, label: t('common.actions.refreshMods'), action: refreshData },
    ]
});
</script>

<template>
    <div class="flex flex-row w-full h-full p-4 py-2 gap-4 bg-surface-app overflow-hidden">
        <div class="flex-1 min-h-0 overflow-hidden">
            <div v-if="sortedWallpapers.length === 0" class="text-center py-12 text-text-secondary">
                <p class="text-lg">{{ t('wallpapersTab.wallpapersNotFound') }}</p>
            </div>
            <div v-else class="h-full overflow-y-auto bg-surface-app">
                <div v-for="wallpaper in sortedWallpapers" :key="wallpaper.id"
                    class="bg-surface-card rounded-lg overflow-hidden mx-2 mb-2">
                    <div class="flex items-center gap-3 p-3">
                        <div
                            class="w-10 h-10 shrink-0 rounded-md bg-mod-wallpaper-bg flex items-center justify-center">
                            <div class="w-5 h-5 bg-mod-wallpaper"
                                :style="{ maskImage: 'url(/icons/wallpaper.png)', maskSize: 'contain', maskRepeat: 'no-repeat', maskPosition: 'center', WebkitMaskImage: 'url(/icons/wallpaper.png)', WebkitMaskSize: 'contain', WebkitMaskRepeat: 'no-repeat', WebkitMaskPosition: 'center' }">
                            </div>
                        </div>

                        <div class="flex-1 min-w-0">
                            <!-- Inline nickname editor -->
                            <div v-if="editingId === wallpaper.id" class="flex items-center gap-2">
                                <Input v-model="editingValue" class="w-full"
                                    :placeholder="t('wallpapersTab.nicknamePlaceholder')"
                                    @keyup.enter="saveEditing(wallpaper)" @keyup.esc="cancelEditing" />
                                <Button :icon="Check" @click="saveEditing(wallpaper)" />
                                <Button :icon="X" @click="cancelEditing" />
                            </div>
                            <div v-else class="flex items-center gap-2 min-w-0">
                                <span class="font-medium truncate"
                                    :class="wallpaper.nickname ? 'text-text-primary' : 'text-text-secondary'">
                                    {{ wallpaper.nickname || wallpaper.id }}
                                </span>
                                <button
                                    class="text-text-secondary hover:text-text-primary transition-colors cursor-pointer shrink-0"
                                    :aria-label="t('wallpapersTab.editName')" @click="startEditing(wallpaper)">
                                    <Pencil class="w-4 h-4" />
                                </button>
                            </div>
                            <div class="text-text-secondary font-mono text-xs truncate">
                                {{ t('wallpapersTab.id', { id: wallpaper.id }) }}
                            </div>
                        </div>

                        <div class="shrink-0 flex items-center gap-3">
                            <button v-if="wallpaper.modsCount > 0"
                                class="bg-accent/75 text-text-on-accent text-xs px-2 py-1 rounded-full font-medium whitespace-nowrap cursor-pointer hover:bg-accent transition-colors"
                                @click="toggleExpanded(wallpaper.id)">
                                {{ t('wallpapersTab.tags.modsCount', { count: wallpaper.modsCount }) }}
                            </button>
                            <Button variant="text" size="sm" :label="t('wallpapersTab.showMods')"
                                @click="toggleExpanded(wallpaper.id)" />
                        </div>
                    </div>

                    <!-- Expanded per-mod list -->
                    <div v-if="expandedId === wallpaper.id" class="border-t border-border-default">
                        <label v-for="mod in modsForWallpaper(wallpaper.id)" :key="mod.name"
                            class="flex items-center gap-3 px-4 py-2.5 border-b border-border-default last:border-b-0 cursor-pointer hover:bg-state-hover transition-colors"
                            :class="{ 'bg-surface-dialog': !mod.enabled }">
                            <Checkbox :model-value="mod.enabled" @update:model-value="toggleMod(mod)"
                                class="shrink-0" />
                            <button @click.stop.prevent="previewMod(mod)"
                                :aria-label="t('charactersTab.characterModal.previewMod')">
                                <Eye
                                    class="w-5 h-5 cursor-pointer hover:text-text-primary! transition-colors active:scale-95 text-text-secondary" />
                            </button>
                            <div class="flex-1 min-w-0">
                                <p class="text-sm truncate"
                                    :class="mod.enabled ? 'text-text-primary' : 'text-text-secondary'">
                                    {{ mod.name }}
                                </p>
                                <p v-if="mod.author" class="text-xs text-text-secondary mt-0.5">{{ mod.author }}</p>
                            </div>
                        </label>
                    </div>
                </div>
            </div>
        </div>

        <div
            class="flex overflow-y-auto flex-col gap-2.5 items-start justify-start w-full max-w-[20%] min-w-50 p-2 bg-surface-panel rounded-md border border-border-default">
            <h1 class="text-lg font-bold text-text-primary w-full">
                {{ t('wallpapersTab.filters.title') }}
            </h1>

            <div class="flex flex-col gap-4 w-full">
                <div class="min-h-10 w-full">
                    <Input v-model="userFilters.searchQuery"
                        :placeholder="t('wallpapersTab.filters.searchPlaceholder')" class="w-64" />
                </div>

                <div class="flex flex-col gap-1.5">
                    <label class="text-sm font-semibold text-text-primary">{{ t('wallpapersTab.filters.sortBy.title')
                        }}</label>
                    <Select v-model="userFilters.sortBy" :options="sortOptions" />
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped></style>
