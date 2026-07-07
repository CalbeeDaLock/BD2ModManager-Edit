<script setup lang="ts">
import { Grid3X3, List, RefreshCcw } from 'lucide-vue-next';
import { computed, reactive, ref, watch, type ComponentPublicInstance } from 'vue';
import { useLocalStorage } from '@vueuse/core';
import { useResizeObserver } from '@vueuse/core';
import { useVirtualizer } from '@tanstack/vue-virtual';
import { useI18n } from 'vue-i18n';

import { BD2Mod, BD2ModExtended, useModsStore } from '../../stores/mods';
import { useHeader } from '../../composables/useHeader';
import { useSaveScroll } from '../../composables/useSaveScroll';

import NpcGridItem from './NpcGridItem.vue';
import NpcModal from './modals/NpcModal.vue';
import Button from '../../components/common/Button.vue';
import Input from '../../components/common/Input.vue';
import Select from '../../components/common/Select.vue';
import Image from '../../components/common/Image.vue';
import { getNpcIcon } from './npcIcons';
import type { NpcEntry } from './types';

const { t } = useI18n();
const modsStore = useModsStore();

const viewMode = useLocalStorage('npc-view-mode', 'grid'); // 'grid' or 'list'

const userFilters = reactive({
    searchQuery: '',
    sortBy: 'id-desc'
});

const sortOptions = computed(() => [
    { label: t('npcTab.filters.sortBy.options.idDesc'), value: 'id-desc' },
    { label: t('npcTab.filters.sortBy.options.idAsc'), value: 'id-asc' },
    { label: t('npcTab.filters.sortBy.options.nameAsc'), value: 'a-z' },
    { label: t('npcTab.filters.sortBy.options.nameDesc'), value: 'z-a' },
    { label: t('npcTab.filters.sortBy.options.modsAsc'), value: 'mods-asc' },
    { label: t('npcTab.filters.sortBy.options.modsDesc'), value: 'mods-desc' },
]);

// npc300501 is not an NPC — she is the character "Loen" (003201) whose
// standing id happens to start with "npc". Exclude her from the NPC list;
// she is still reachable from the Characters page via her npc_id.
const EXCLUDED_NPC_IDS = new Set(['300501']);

// Manual display-name overrides for NPCs that are known but not resolved
// from characters.json. Keyed by the numeric NPC id (no "npc" prefix).
const NPC_NAME_OVERRIDES: Record<string, string> = {
    '000001': 'Shop Girl',
    '000005': 'Eleanor',
};

// Derive the NPC list from NPC-typed mods in the mods store, grouped by
// the NPC id reported on each mod's modType. Most NPCs are not present in
// characters.json (only 1 of 181 entries has npc_id), so we cannot rely on
// the characters store here — the mod list is the source of truth.
const npcs = computed<NpcEntry[]>(() => {
    const groups = new Map<string, NpcEntry>();

    for (const mod of modsStore.mods as readonly BD2ModExtended[]) {
        if (mod.modType?.type !== 'NPC') continue;
        if (!('id' in mod.modType)) continue;
        const id = mod.modType.id;

        // Skip ids that aren't real NPCs (e.g. characters whose standing id
        // starts with "npc").
        if (EXCLUDED_NPC_IDS.has(id)) continue;

        // Prefer a manual override, then the resolved character name from the
        // store (extendedMods sets mod.character via getCharacterByNpcId), and
        // finally fall back to "NPC <id>" when the NPC isn't otherwise known.
        const characterName = NPC_NAME_OVERRIDES[id] || mod.character?.character;

        let entry = groups.get(id);
        if (!entry) {
            entry = {
                id,
                name: characterName || `NPC ${id}`,
                modsCount: 0,
                enabledCount: 0,
                hasStanding: false
            };
            groups.set(id, entry);
        }
        entry.modsCount += 1;
        if (mod.enabled && !mod.errors.length) {
            entry.enabledCount += 1;
            entry.hasStanding = true;
        }
    }

    return Array.from(groups.values());
});

// Header counts must match what's actually shown on screen: sum over the
// derived npc list (which already excludes non-NPC ids), not every NPC-typed
// mod in the store.
const totalModsCount = computed(() =>
    npcs.value.reduce((sum, npc) => sum + npc.modsCount, 0)
);
const enabledModsCount = computed(() =>
    npcs.value.reduce((sum, npc) => sum + npc.enabledCount, 0)
);

const filteredNpcs = computed(() => {
    const search = userFilters.searchQuery.toLowerCase();
    return npcs.value.filter(npc => {
        if (search && !npc.name.toLowerCase().includes(search) && !npc.id.toLowerCase().includes(search)) {
            return false;
        }
        return true;
    });
});

const sortedNpcs = computed(() => {
    const arr = [...filteredNpcs.value];
    arr.sort((a, b) => {
        switch (userFilters.sortBy) {
            case 'id-asc':
                return a.id.localeCompare(b.id, undefined, { numeric: true });
            case 'id-desc':
                return b.id.localeCompare(a.id, undefined, { numeric: true });
            case 'a-z':
                return a.name.localeCompare(b.name, undefined, { numeric: true }) || a.id.localeCompare(b.id);
            case 'z-a':
                return b.name.localeCompare(a.name, undefined, { numeric: true }) || b.id.localeCompare(a.id);
            case 'mods-desc':
                return b.modsCount - a.modsCount;
            case 'mods-asc':
                return a.modsCount - b.modsCount;
            default:
                // Default: largest id number first.
                return b.id.localeCompare(a.id, undefined, { numeric: true });
        }
    });
    return arr;
});

// Grid layout: same responsive column-count as CharacterGrid.vue.
const parentRef = ref<HTMLElement | null>(null);
const columnCount = ref(4);

function getColumnCount(width: number) {
    if (width >= 940) return 5;
    if (width >= 740) return 4;
    if (width >= 480) return 3;
    if (width >= 440) return 2;
    return 1;
}

useResizeObserver(parentRef, ([entry]) => {
    columnCount.value = getColumnCount(entry.contentRect.width);
});

const rows = computed(() => {
    const result: NpcEntry[][] = [];
    for (let i = 0; i < sortedNpcs.value.length; i += columnCount.value) {
        result.push(sortedNpcs.value.slice(i, i + columnCount.value));
    }
    return result;
});

const rowVirtualizer = useVirtualizer({
    get count() { return rows.value.length; },
    getScrollElement: () => parentRef.value,
    estimateSize: () => 220,
    overscan: 4,
    measureElement: (element, _entry, instance) => {
        const index = Number(element.getAttribute('data-index'));
        const cached = instance.measurementsCache[index]?.size;
        if (cached !== undefined) return cached;
        return element.getBoundingClientRect().height;
    },
});

watch(columnCount, () => {
    rowVirtualizer.value.measure();
});

const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems());
const totalSize = computed(() => rowVirtualizer.value.getTotalSize());

const measureElement = (el: Element | ComponentPublicInstance | null) => {
    if (!el || !(el instanceof Element)) return;
    rowVirtualizer.value.measureElement(el);
};

useSaveScroll(rowVirtualizer);

const showNpcModal = ref(false);
const selectedNpc = ref<NpcEntry | null>(null);

function openNpcDetails(npc: NpcEntry) {
    selectedNpc.value = npc;
    showNpcModal.value = true;
}

function toggleMod(mod: BD2Mod) {
    if (mod.enabled) {
        modsStore.disableMods([mod.name]);
    } else {
        modsStore.enableMods([mod.name]);
    }
}

function refreshData() {
    modsStore.discoverMods();
}

useHeader({
    title: t('npcTab.title'),
    subtitle: computed(() =>
        t('npcTab.subtitle', {
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
            <div v-if="sortedNpcs.length === 0" class="text-center py-12 text-text-secondary">
                <p class="text-lg">{{ t('npcTab.npcsNotFound') }}</p>
            </div>
            <div v-else-if="viewMode === 'grid'" ref="parentRef" class="h-full overflow-y-auto bg-surface-app" style="contain: strict">
                <div :style="{ height: `${totalSize}px`, width: '100%', position: 'relative' }">
                    <div :style="{
                        position: 'absolute',
                        top: 0,
                        left: 0,
                        width: '100%',
                        transform: `translateY(${virtualRows[0]?.start ?? 0}px)`,
                    }">
                        <div
                            v-for="virtualRow in virtualRows"
                            :key="String(virtualRow.key)"
                            :ref="measureElement"
                            :data-index="virtualRow.index"
                            class="grid gap-4 pb-4"
                            :style="{ gridTemplateColumns: `repeat(${columnCount}, minmax(0, 1fr))` }"
                        >
                            <NpcGridItem
                                v-for="npc in rows[virtualRow.index]"
                                :key="npc.id"
                                :npc="npc"
                                @open-mod-details="openNpcDetails"
                            />
                        </div>
                    </div>
                </div>
            </div>
            <!-- List view: simple non-virtualized list, one row per NPC -->
            <div v-else class="h-full overflow-y-auto bg-surface-app">
                <div v-for="npc in sortedNpcs" :key="npc.id"
                    @click="openNpcDetails(npc)"
                    class="flex bg-surface-card rounded-lg overflow-hidden cursor-pointer hover:bg-state-hover transition-colors mx-2 mb-2">
                    <div class="shrink-0">
                        <Image :src="getNpcIcon(npc.id) ?? 'characters/standing/placeholder_character.png'" :alt="npc.name"
                            class="w-42 h-42 object-cover object-top rounded-t-md aspect-square"
                            error-src="characters/standing/placeholder_character.png" error-class="bg-text-primary" skeleton />
                    </div>
                    <div class="flex flex-col flex-1 p-2 min-w-0">
                        <div class="text-lg font-medium flex gap-2 items-center flex-wrap">
                            <span class="truncate">{{ npc.name }}</span>
                            <div v-if="npc.modsCount > 0"
                                class="flex bg-accent/75 text-text-on-accent text-xs px-2 py-1 rounded-full font-medium whitespace-nowrap">
                                {{ t('npcTab.tags.modsCount', { count: npc.modsCount }) }}
                            </div>
                        </div>
                        <span class="text-text-secondary font-mono text-sm">{{ t('npcTab.id', { id: npc.id }) }}</span>
                        <div class="flex flex-1 items-end gap-8 md:gap-12 mr-4 md:mr-8 mt-2">
                            <div class="flex flex-col items-center">
                                <span class="font-semibold text-sm md:text-base">{{ t('charactersTab.modTypes.standing') }}</span>
                                <span class="font-mono text-xs md:text-sm" :class="{
                                    'text-success': npc.hasStanding,
                                    'text-error': !npc.hasStanding
                                }">
                                    {{ npc.hasStanding ? t('charactersTab.modTypes.states.enabled', 'Enabled') : t('charactersTab.modTypes.states.disabled', 'Disabled') }}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div
            class="flex overflow-y-auto flex-col gap-2.5 items-start justify-start w-full max-w-[20%] min-w-50 p-2 bg-surface-panel rounded-md border border-border-default">
            <h1 class="text-lg font-bold text-text-primary flex flex-row justify-between items-center w-full">
                {{ t('npcTab.filters.title') }}
                <div class="flex gap-1">
                    <Button @click="viewMode = 'grid'" :icon="Grid3X3" :icon-class="{
                        'text-accent': viewMode === 'grid',
                        'text-text-primary': viewMode !== 'grid',
                        'text-xs': true
                    }" />
                    <Button @click="viewMode = 'list'" :icon="List" :icon-class="{
                        'text-accent': viewMode === 'list',
                        'text-text-primary': viewMode !== 'list',
                        'text-xs': true
                    }" />
                </div>
            </h1>

            <div class="flex flex-col gap-4 w-full">
                <div class="min-h-10 w-full">
                    <Input v-model="userFilters.searchQuery" :placeholder="t('npcTab.filters.searchPlaceholder')" class="w-64" />
                </div>

                <div class="flex flex-col gap-1.5">
                    <label class="text-sm font-semibold text-text-primary">{{ t('npcTab.filters.sortBy.title') }}</label>
                    <Select v-model="userFilters.sortBy" :options="sortOptions" />
                </div>
            </div>
        </div>

        <NpcModal v-model:show="showNpcModal" :selectedNpc="selectedNpc" :toggleMod="toggleMod" />
    </div>
</template>

<style scoped></style>
