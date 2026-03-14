<script setup lang="ts">
import { Grid3X3, List, RefreshCcw, FolderPlus } from 'lucide-vue-next';
import { computed, h, nextTick, onActivated, onDeactivated, onMounted, reactive, ref, watch } from 'vue';
import { Character, useCharactersStore } from '../../stores/characters';
import { BD2Mod, useModsStore } from '../../stores/mods';
import { useHeader } from '../../composables/useHeader';
import { useI18n } from 'vue-i18n';
import { useLocalStorage } from '@vueuse/core';
import { useRoute } from 'vue-router';
import Menu from '../../components/common/Menu.vue';
import Button from '../../components/common/Button.vue';
import Input from '../../components/common/Input.vue';
import CharacterModal from './modals/CharacterModal.vue';
import Select from '../../components/common/Select.vue';
import Checkbox from '../../components/common/Checkbox.vue';
import CharacterList from './CharacterList.vue';
import CharacterGrid from './CharacterGrid.vue';
import { useLoggingStore } from '../../stores/logging';
import { useToast } from 'primevue/usetoast';
import { open } from '@tauri-apps/plugin-dialog';

const { t } = useI18n()

const charactersStore = useCharactersStore()
const modsStore = useModsStore()

const viewMode = useLocalStorage('characters-view-mode', 'grid') // 'grid' or 'list'

const modStatusOptions = computed(() => [
    { label: t('characters.modStatus.any'), value: 'any' },
    { label: t('characters.modStatus.enabled'), value: 'enabled' },
    { label: t('characters.modStatus.disabled'), value: 'disabled' }
]);

const userFilters = reactive({
    searchQuery: '',
    sortBy: 'a-z',
    cutscene: 'any',
    standing: 'any',
    dating: 'any',
    hideCollabCharacters: false,
    hideMenCharacters: false,
    hideWomenCharacters: false,
    releasePeriod: 'all',
    onlyWithMods: false,
    onlyWithoutMods: false
})

const showCostumeModal = ref(false);
const selectedCostume = ref<Character | null>(null);

const modIndex = computed(() => {
    type Mod = typeof modsStore.mods[number];
    const index = new Map<string, { enabledTypes: Set<string>; mods: Mod[] }>();
    for (const mod of modsStore.mods) {
        if (!mod.modType || !('id' in mod.modType)) continue;
        const id = mod.modType.id;
        let entry = index.get(id);
        if (!entry) {
            entry = { enabledTypes: new Set(), mods: [] };
            index.set(id, entry);
        }
        if (['Cutscene', 'Standing', 'Dating'].includes(mod.modType.type)) {
            entry.mods.push(mod);
        }
        if (mod.enabled) {
            entry.enabledTypes.add(mod.modType.type);
        }
    }
    return index;
});

function hasCutsceneInstalled(id: string) {
    return modIndex.value.get(id)?.enabledTypes.has('Cutscene') ?? false;
}

function hasStandingInstalled(id: string) {
    return modIndex.value.get(id)?.enabledTypes.has('Standing') ?? false;
}

function hasDatingInstalled(id: string) {
    return modIndex.value.get(id)?.enabledTypes.has('Dating') ?? false;
}

function getInstalledMods(costumeId: string) {
    return modIndex.value.get(costumeId)?.mods ?? [];
}

function getInstalledModsCount(costumeId: string) {
    return modIndex.value.get(costumeId)?.mods.length ?? 0;
}

function toggleMod(mod: BD2Mod) {
    if (mod.enabled) {
        modsStore.disableMods([mod.name])
    } else {
        modsStore.enableMods([mod.name])
    }
}

const filteredCharacters = computed(() => {
    const search = userFilters.searchQuery.toLowerCase();

    return charactersStore.characters.filter((char) => {
        if (
            search &&
            !char.character.toLowerCase().includes(search) &&
            !char.costume.toLowerCase().includes(search)
        ) return false;

        const cutsceneEnabled = hasCutsceneInstalled(char.id);
        const standingEnabled = hasStandingInstalled(char.id);
        const datingEnabled = hasDatingInstalled(char.id);

        if (userFilters.cutscene === 'enabled' && !cutsceneEnabled) return false;
        if (userFilters.cutscene === 'disabled' && cutsceneEnabled) return false;

        if (userFilters.standing === 'enabled' && !standingEnabled) return false;
        if (userFilters.standing === 'disabled' && standingEnabled) return false;


        // if filtkering by dating, also exclude characters that don't have dating at all. we can check dating_id
        if (userFilters.dating !== 'any' && !char.dating_id) return false;
        if (userFilters.dating === 'enabled' && !datingEnabled) return false;
        if (userFilters.dating === 'disabled' && datingEnabled) return false;

        if (userFilters.hideMenCharacters && char?.gender === 'male') return false;
        if (userFilters.hideWomenCharacters && char?.gender === 'female') return false;

        if (userFilters.onlyWithMods && getInstalledModsCount(char.id) === 0) return false;
        if (userFilters.onlyWithoutMods && getInstalledModsCount(char.id) > 0) return false;

        switch (userFilters.releasePeriod) {
            case 'lastWeek':
                if (!char.release_date || new Date(char.release_date) < new Date(Date.now() - 7 * 86400000)) return false;
                break;
            case 'lastTwoWeeks':
                if (!char.release_date || new Date(char.release_date) < new Date(Date.now() - 14 * 86400000)) return false;
                break;
            case 'lastMonth':
                if (!char.release_date || new Date(char.release_date) < new Date(Date.now() - 30 * 86400000)) return false;
                break;
            case 'lastThreeMonths':
                if (!char.release_date || new Date(char.release_date) < new Date(Date.now() - 90 * 86400000)) return false;
                break;
            case 'lastSixMonths':
                if (!char.release_date || new Date(char.release_date) < new Date(Date.now() - 180 * 86400000)) return false;
                break;
            case 'lastYear':
                if (!char.release_date || new Date(char.release_date) < new Date(Date.now() - 365 * 86400000)) return false;
                break;
        }

        return true;
    });
});

const sortedCharacters = computed(() => {
    const arr = [...filteredCharacters.value];

    arr.sort((a, b) => {
        switch (userFilters.sortBy) {
            case 'a-z':
                return a.character.localeCompare(b.character) || a.costume.localeCompare(b.costume);

            case 'z-a':
                return b.character.localeCompare(a.character) || b.costume.localeCompare(a.costume);

            case 'newest':
                return new Date(b.release_date || 0).getTime() - new Date(a.release_date || 0).getTime();

            case 'oldest':
                return new Date(a.release_date || 0).getTime() - new Date(b.release_date || 0).getTime();

            case 'mods-desc':
                return getInstalledModsCount(b.id) - getInstalledModsCount(a.id);

            case 'mods-asc':
                return getInstalledModsCount(a.id) - getInstalledModsCount(b.id);

            default:
                return 0;
        }
    });

    return arr;
});

const charactersGrid = computed(() =>
    sortedCharacters.value.map((costume) => ({
        ...costume,
        hasCutscene: hasCutsceneInstalled(costume.id),
        hasStanding: hasStandingInstalled(costume.id),
        hasDating: hasDatingInstalled(costume.id),
        modsCount: getInstalledModsCount(costume.id)
    }))
);

const charactersList = computed(() => {
    const groups = new Map<string, Character[]>();

    for (const char of sortedCharacters.value) {
        if (!groups.has(char.character)) {
            groups.set(char.character, []);
        }
        groups.get(char.character)!.push(char);
    }

    return Array.from(groups.entries()).map(([name, costumes]) => ({
        name,
        costumes: costumes.map(costume => ({
            ...costume,
            hasCutscene: hasCutsceneInstalled(costume.id),
            hasStanding: hasStandingInstalled(costume.id),
            hasDating: hasDatingInstalled(costume.id),
            modsCount: getInstalledModsCount(costume.id)
        }))
    }));
});



function refreshData() {
    charactersStore.loadCharacters()
    modsStore.discoverMods()
}

const openCostumeDetails = (costume: Character) => {
    selectedCostume.value = costume;
    showCostumeModal.value = true;
};

const totalModsCount = computed(() => modsStore.mods.length)
const enabledModsCount = computed(() => modsStore.mods.filter(mod => mod.enabled).length)
const route = useRoute()

function getErrorMessage(error: any) {
    if (!error) return t('errors.unknownError')

    if (error === "GameDirectoryNotSet") {
        return t('errors.gameDirectoryNotSet')
    }

    if (error.SyncMethodInvalid) {
        return t('errors.syncMethodInvalid', { method: error.SyncMethodInvalid })
    }

    if (error.SyncFailed) {
        const sync = error.SyncFailed

        switch (sync.type) {
            case "SymlinkAdminRequired":
                return t('errors.symlinkAdminRequired')

            case "ModPathNotFound":
                return t('errors.modPathNotFound', { path: sync.details })

            case "CopyFailed":
                return t('errors.copyFailed', { error: sync.details })

            case "SymlinkFailed":
                return t('errors.symlinkFailed', { error: sync.details })

            case "HardlinkFailed":
                return t('errors.hardlinkFailed', { error: sync.details })

            case "DirectoryCreationFailed":
                return t('errors.directoryCreationFailed', { error: sync.details })

            case "RemovalFailed":
                return t('errors.removalFailed', { error: sync.details })

            default:
                return t('errors.unknownError', { error: JSON.stringify(sync) })
        }
    }

    if (error.UnknownError) {
        return t('errors.unknownError', { error: error.UnknownError })
    }

    return t('errors.unknownError', { error: JSON.stringify(error) })
}
async function handleInstallFromZip() {
    const file = await open({
        multiple: false,
        filters: [{ name: 'ZIP Files', extensions: ['zip'] }]
    });

    loggingStore.logDebug("Selected file for mod installation from zip:", file);

    if (file && typeof file === 'string') {
        try {
            let modName = await modsStore.installModFromZip(file);

            loggingStore.logDebug(`Mod "${modName}" installed successfully from zip: ${file}`);

            toast.add({
                closable: true,
                summary: t('mods.modInstalledSuccess'),
                detail: modName,
                life: 3000,
            });
        } catch (error) {
            loggingStore.logError("Error installing mod from zip:", error);

            let errorMsg = getErrorMessage(error instanceof Error ? error.message : String(error));

            toast.add({
                closable: true,
                summary: t('errors.modInstallFailed'),
                detail: errorMsg,
                life: 5000,
                severity: 'error'
            });
        }
    }
}

const loggingStore = useLoggingStore()
const toast = useToast()

async function handleInstallFromFolder() {
    const folder = await open({
        directory: true,
        multiple: false
    });

    loggingStore.logDebug("Selected folder for mod installation:", folder);

    if (folder && typeof folder === 'string') {
        try {
            let modName = await modsStore.installModFromFolder(folder);

            loggingStore.logDebug(`Mod "${modName}" installed successfully from folder: ${folder}`);

            toast.add({
                closable: true,
                summary: t('mods.modInstalledSuccess'),
                detail: modName,
                life: 3000,
            });
        } catch (error) {
            loggingStore.logError("Error installing mod from folder:", error);


            toast.add({
                closable: true,
                summary: t('errors.modInstallFailed'),
                detail: error,
                life: 5000,
                severity: 'error'
            });
        }
    }
}


const addModMenuItems = computed(() => [
    {
        label: t('mods.actions.installFromZip'), clicked: handleInstallFromZip
    },
    {
        label: t('mods.actions.installFromFolder'), clicked: handleInstallFromFolder
    }
]);
useHeader({
    title: t('characters.title'), subtitle: computed(() =>
        t('mods.subtitle', {
            enabledModsCount: enabledModsCount.value,
            totalModsCount: totalModsCount.value
        })
    ), buttons: [
        { icon: RefreshCcw, label: 'Refresh', action: refreshData },
        {
            render: () =>
                h('div', { class: 'flex gap-2' }, [
                    h(Menu, {}, {
                        trigger: ({ toggle }: any) =>
                            h(Button, {
                                label: t('mods.actions.addMod'),
                                icon: FolderPlus,
                                variant: 'text',
                                onClick: toggle
                            }),
                        content: () =>
                            h(
                                'ul',
                                {},
                                addModMenuItems.value.map(item =>
                                    h(
                                        'li',
                                        { key: item.label },
                                        h(
                                            'button',
                                            {
                                                class: 'w-full cursor-pointer text-left px-4 py-2 hover:bg-interactive-bg-hover',
                                                onClick: item.clicked
                                            },
                                            item.label
                                        )
                                    )
                                )
                            )
                    }),
                ])
        },
    ]
})

const scrollTop = ref(0);
const scrollContainer = ref<HTMLElement | null>(null);

onDeactivated(() => {
    if (scrollContainer.value) scrollTop.value = scrollContainer.value.scrollTop;
});

onActivated(() => {
    if (scrollContainer.value) scrollContainer.value.scrollTop = scrollTop.value;
});
onMounted(() => {
    console.log(charactersStore.groupedCharacters)
    if (route.query.characterId) {
        const costume = charactersStore.characters.find(char => char.id === route.query.characterId);
        if (costume) {
            selectedCostume.value = costume;
            showCostumeModal.value = true;
        }
    }
})

const sortOptions = [
    { label: t('characters.sort.nameAsc'), value: 'a-z' },
    { label: t('characters.sort.nameDesc'), value: 'z-a' },
    { label: t('characters.sort.releaseDesc'), value: 'oldest' },
    { label: t('characters.sort.releaseAsc'), value: 'newest' },
    { label: t('characters.sort.modsDesc'), value: 'mods-desc' },
    { label: t('characters.sort.modsAsc'), value: 'mods-asc' },
];

const releasePeriodOptions = [
    { label: t('characters.filters.all'), value: 'all' },
    { label: t('characters.filters.lastWeek'), value: 'lastWeek' },
    { label: t('characters.filters.lastTwoWeeks'), value: 'lastTwoWeeks' },
    { label: t('characters.filters.lastMonth'), value: 'lastMonth' },
    { label: t('characters.filters.lastThreeMonths'), value: 'lastThreeMonths' },
    { label: t('characters.filters.lastSixMonths'), value: 'lastSixMonths' },
    { label: t('characters.filters.lastYear'), value: 'lastYear' }
]

// [TODO] use {{ route.query.characterId }}
watch(() => route.query.characterId, (newCharacterId) => {
    nextTick(() => {
        if (newCharacterId) {
            const costume = charactersStore.characters.find(char => char.id === newCharacterId);
            if (costume) {
                selectedCostume.value = costume;
                showCostumeModal.value = true;
            }
        } else {
            showCostumeModal.value = false;
            selectedCostume.value = null;
        }
    })
})

</script>

<template>
    <div class="flex flex-row w-full p-4 py-2 gap-4 bg-bg-deep overflow-hidden">

        <!-- {{ charactersGrid }} -->
        <div class="flex-1 overflow-y-auto" ref="scrollContainer">
            <CharacterList v-if="viewMode == 'list'" :items="charactersList" @openModDetails="openCostumeDetails" />
            <CharacterGrid v-else :items="charactersGrid" @openModDetails="openCostumeDetails" />
        </div>

        <div
            class="flex overflow-y-auto flex-col gap-2.5 items-start justify-start w-full max-w-[20%] min-w-50 p-2 bg-bg-surface rounded border border-border">
            <h1 class="text-lg font-bold text-primary flex flex-row justify-between items-center w-full">
                {{ t('characters.filters.title', 'Filters') }}
                <div class="flex gap-1">
                    <Button @click="viewMode = 'grid'" :icon="Grid3X3" :icon-class="{
                        'text-accent-primary': viewMode === 'grid',
                        'text-primary': viewMode !== 'grid',
                        'text-xs': true
                    }" />
                    <Button @click="viewMode = 'list'" :icon="List" :icon-class="{
                        'text-accent-primary': viewMode === 'list',
                        'text-primary': viewMode !== 'list',
                        'text-xs': true
                    }" />
                </div>
            </h1>

            <div class="flex flex-col gap-4 w-full">
                <div class="min-h-10 w-full">
                    <Input v-model="userFilters.searchQuery"
                        :placeholder="t('characters.filters.search', 'Search characters...')" class="w-64" />
                </div>

                <div class="flex flex-col gap-1.5">
                    <label class="text-sm font-semibold text-primary">{{ t('characters.filters.sortBy', 'Sort By')
                    }}</label>
                    <Select v-model="userFilters.sortBy" :options="sortOptions" />
                </div>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-semibold text-primary">{{ t('characters.filters.modStatus', 'Mod Status')
                    }}</label>
                    <div class="flex flex-col gap-2">
                        <div class="flex flex-col gap-1">
                            <label for="cutscene" class="text-xs font-medium text-secondary">{{
                                t('characters.filters.hasCutscene', 'Has Cutscene') }}</label>
                            <Select id="cutscene" v-model="userFilters.cutscene" :options="modStatusOptions" />
                        </div>
                        <div class="flex flex-col gap-1">
                            <label for="standing" class="text-xs font-medium text-secondary">{{
                                t('characters.filters.hasStanding', 'Has Standing') }}</label>
                            <Select id="standing" v-model="userFilters.standing" :options="modStatusOptions" />
                        </div>
                        <div class="flex flex-col gap-1">
                            <label for="dating" class="text-xs font-medium text-secondary">{{
                                t('characters.filters.hasDating', 'Has Dating') }}</label>
                            <Select id="dating" v-model="userFilters.dating" :options="modStatusOptions" />
                        </div>
                    </div>
                </div>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-semibold text-primary">{{ t('characters.filters.extraFilters', 'Extra Filters') }}</label>
                    <div class="flex flex-col gap-1.5">
                        <Checkbox v-model="userFilters.hideMenCharacters"
                            :label="t('characters.filters.hideMenCharacters', 'Hide Male Characters')" />
                        <Checkbox v-model="userFilters.hideWomenCharacters"
                            :label="t('characters.filters.hideWomenCharacters', 'Hide Female Characters')" />
                        <Checkbox v-model="userFilters.onlyWithMods"
                            :label="t('characters.filters.onlyWithMods', 'Only Characters With Mods')" />
                        <Checkbox v-model="userFilters.onlyWithoutMods"
                            :label="t('characters.filters.onlyWithoutMods', 'Only Characters Without Mods')" />
                    </div>
                </div>

                <div class="flex flex-col gap-2">
                    <label class="text-sm font-semibold text-primary">{{ t('characters.filters.releaseDate', 'Release Date') }}</label>
                    <Select v-model="userFilters.releasePeriod" :options="releasePeriodOptions" />
                </div>
            </div>
        </div>

        <CharacterModal v-model:show="showCostumeModal" :selectedCostume="selectedCostume"
            :getInstalledMods="getInstalledMods" :toggleMod="toggleMod" />
    </div>
</template>

<style scoped></style>