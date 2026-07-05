<script setup lang="ts">
<<<<<<< HEAD
import { X, Calendar, Tag, Gem, Eye, BadgeDollarSign, ExternalLink, Info } from 'lucide-vue-next';
=======
import { X, Calendar, Eye, BadgeDollarSign, ExternalLink, Info, Tag } from 'lucide-vue-next';
>>>>>>> upstream/main
import { Character } from '../../../stores/characters';
import { BD2Mod, useModsStore } from '../../../stores/mods';
import { computed, ref } from 'vue';
import Button from '../../../components/common/Button.vue';
import Image from '../../../components/common/Image.vue';
import Modal from '../../../components/common/Modal.vue';
import Checkbox from '../../../components/common/Checkbox.vue';
import { useLoggingStore } from '../../../stores/logging';
import { getErrorMessage } from '../../../utils/errors';
<<<<<<< HEAD
import { useToast } from 'primevue/usetoast';
=======
>>>>>>> upstream/main
import { useI18n } from 'vue-i18n';
import { Tab, TabGroup, TabList, TabPanels, TabPanel } from '@headlessui/vue';
import { useModsIndexStore } from '../../../stores/modsIndex';
import KofiIcon from '../../../components/icons/KofiIcon.vue';
import DiscordIcon from '../../../components/icons/DiscordIcon.vue';
import PatreonIcon from '../../../components/icons/PatreonIcon.vue';
import { openUrl } from '@tauri-apps/plugin-opener';
import AfDianIcon from '../../../components/icons/AfDianIcon.vue';
import { convertFileSrc } from '@tauri-apps/api/core';
<<<<<<< HEAD
import { useAppDir } from '../../../composables/useAppDir';
import { sortModsByPath } from '../../../utils/sortMods';

const loggingStore = useLoggingStore();
const toast = useToast();
=======
import { useNotificationStore } from '../../../stores/notification.ts';
import { getCharName, useLang } from '../../../utils/formatCharName.ts';

const loggingStore = useLoggingStore();
const notificationStore = useNotificationStore();
>>>>>>> upstream/main
const { t } = useI18n();
const modsIndex = useModsIndexStore();

const show = defineModel('show', {
    type: Boolean,
    required: true,
});

const props = defineProps<{
    selectedCostume: Character | null;
    toggleMod: (mod: BD2Mod) => void;
}>();

const modsStore = useModsStore();

<<<<<<< HEAD
const costumeId = computed(() => {
    const id = props.selectedCostume?.id;
    return Array.isArray(id) ? id[0] : id;
});

=======
>>>>>>> upstream/main
const costumeIds = computed((): string[] => {
    const id = props.selectedCostume?.id;
    return Array.isArray(id) ? [...id] : id ? [id as string] : [];
});

const installedMods = computed(() => {
    if (!props.selectedCostume) return [];
    return modsStore.mods.filter(mod => {
        if (!mod.modType) return false;
        const { type } = mod.modType;
        if (['Cutscene', 'Standing'].includes(type)) {
            return 'id' in mod.modType && costumeIds.value.includes(mod.modType.id);
        }
        if (type === 'NPC') {
            if (!mod.character) return false;
            const modCharIds = Array.isArray(mod.character.id) ? [...mod.character.id] : [mod.character.id];
            return modCharIds.some(id => costumeIds.value.includes(id));
        }
        if (type === 'Dating') {
            return 'id' in mod.modType && mod.modType.id === props.selectedCostume?.dating_id;
        }
        return false;
    });
});

const modsByType = computed(() => {
    const grouped = {
        Cutscene: [] as BD2Mod[],
        Standing: [] as BD2Mod[],
        Dating: [] as BD2Mod[]
    };
    installedMods.value.forEach(mod => {
        const type = mod.modType?.type === 'NPC' ? 'Standing' : mod.modType?.type;
        if (type && type in grouped) {
            grouped[type as keyof typeof grouped].push(mod);
        }
    });
<<<<<<< HEAD
    // Sort each group A-Z by folder path so the mod list is alphabetical
    // regardless of the underlying store order.
    (Object.keys(grouped) as (keyof typeof grouped)[]).forEach(type => {
        grouped[type] = sortModsByPath(grouped[type]);
    });
=======
>>>>>>> upstream/main
    return grouped;
});

const enabledModsCount = computed(() =>
    installedMods.value.filter(mod => mod.enabled).length
);

async function openPreviewMod(mod: BD2Mod) {
    modsStore.previewMod(mod.name).then(() => {
        loggingStore.logDebug("Mod previewed successfully:", mod.name);
    }).catch((error) => {
        let errorMsg = getErrorMessage(t, error);
<<<<<<< HEAD
        toast.add({
            severity: "error",
            closable: true,
            summary: t("modsTab.errors.modPreview.title"),
            detail: errorMsg,
            life: 5000
=======
        notificationStore.add({
            severity: "error",
            closable: true,
            title: t("modsTab.errors.modPreview.title"),
            message: errorMsg,
            duration: 5000
>>>>>>> upstream/main
        });
        loggingStore.logError("Error previewing mod:", error);
    });
}

<<<<<<< HEAD
const baseDir = useAppDir();

=======
>>>>>>> upstream/main
function getIconForLink(key: string) {
    switch (key) {
        case "patreon": return PatreonIcon;
        case "ko-fi": return KofiIcon;
        case "discord": return DiscordIcon;
        case "afdian": return AfDianIcon;
        default: return ExternalLink;
    }
}

function handleOpenLink(link: string | null) {
    if (!link) return;
    openUrl(link);
}

const tooltipVisible = ref<string | null>(null);

const hasIndexMods = computed(() => {
    if (!props.selectedCostume) return false;
    return ['cutscene', 'standing', 'dating'].some(
        type => costumeIds.value.some(id => modsIndex.getMods(id, type).length > 0)
    );
});

function getIndexMods(type: string) {
    return costumeIds.value.flatMap(id => modsIndex.getMods(id, type));
}
<<<<<<< HEAD
=======

const imageUrl = computed(() => {
    if (!props.selectedCostume) return "#"
    const ids = Array.isArray(props.selectedCostume.id)
        ? props.selectedCostume.id.join(',')
        : props.selectedCostume.id
    return convertFileSrc(`standing/${ids}`, "bd2assets")
})

const lang = useLang()

const charName = computed(() => {   
    if (!props.selectedCostume) return
    return getCharName(props.selectedCostume, lang.value)
})
>>>>>>> upstream/main
</script>

<template>
    <Modal v-model:show="show" class="w-[50vw] max-h-[85vh]" @close="() => show = false">
        <template #footer>
<<<<<<< HEAD
            <div class="flex p-3 justify-end items-center w-full border-t border-border">
=======
            <div class="flex p-3 justify-end items-center w-full border-t border-border-default">
>>>>>>> upstream/main
                <Button variant="default" :label="$t('common.actions.close')" @click="show = false" />
            </div>
        </template>
        <template #header></template>

<<<<<<< HEAD
        <div v-if="selectedCostume" class="flex flex-col min-h-0 text-primary overflow-hidden">
            <div class="flex items-stretch border-b border-border shrink-0">
                <Image :src="`characters/standing/${costumeId}.png`"
                    :alt="`${selectedCostume.character} - ${selectedCostume.costume}`"
                    class="w-40 h-40 object-cover shrink-0 border-r border-border"
                    :fallback-sources="[
                        convertFileSrc(`${baseDir}/assets/standing/${costumeId}.png`),
                        '/characters/standing/placeholder_character.png'
                    ]" />

                <div class="flex-1 px-4 py-3">
                    <div class="flex items-start justify-between gap-2">
                        <div>
                            <div class="flex items-baseline gap-2">
                                <h3 class="font-semibold text-base text-primary">{{ selectedCostume.character }}</h3>
                                <span class="text-sm text-secondary">{{ selectedCostume.costume }}</span>
                            </div>
                            <div class="flex items-center gap-3 mt-1 text-xs text-secondary">
                                <span class="flex items-center gap-1">
                                    <Tag class="w-4 h-4" />{{ Array.isArray(selectedCostume.id) ? selectedCostume.id.join(', ') : selectedCostume.id }}
=======
        <div v-if="selectedCostume" class="flex flex-col min-h-0 text-text-primary overflow-hidden">
            <div class="flex items-stretch border-b border-border-default shrink-0">
                <!-- [TODO] when clicked it shows full image -->
                <Image :src="imageUrl"
                    class="w-40 h-40 object-cover shrink-0 border-r border-border-default aspect-square"
                    skeleton
                    error-src="characters/standing/placeholder_character.png"
                    error-class="bg-text-primary" />

                <div class="flex-1 px-4 py-3 flex flex-col">
                    <div class="flex items-start justify-between gap-2">
                        <div>
                            <div class="flex items-baseline gap-2">
                                <h3 class="font-semibold text-base text-text-primary">{{ charName?.character }}</h3>
                                <span class="text-sm text-text-secondary">{{ charName?.costume }}</span>
                            </div>
                            <div class="flex flex-col tems-center gap-3 mt-1 text-xs text-text-secondary">
                                <span class="flex items-center gap-1">
                                    <Tag class="w-4 h-4"/>
                                    {{ Array.isArray(selectedCostume.id) ? selectedCostume.id.join(', ') : selectedCostume.id }}
>>>>>>> upstream/main
                                </span>
                                <span v-if="selectedCostume.release_date" class="flex items-center gap-1">
                                    <Calendar class="w-4 h-4" />
                                    {{ new Date(selectedCostume.release_date).toLocaleDateString() }}
                                </span>
<<<<<<< HEAD
                                <span v-if="selectedCostume.is_collab"
                                    class="flex items-center gap-1 px-1.5 py-0.5 bg-accent-primary/20 text-accent-primary rounded text-xs font-medium">
                                    <Gem class="w-4 h-4" /> {{ $t('characters.tags.collab') }}
                                </span>
=======
                                <!-- <span v-if="selectedCostume.is_collab"
                                    class="flex items-center gap-1 px-1.5 py-0.5 bg-accent-muted text-accent rounded text-xs font-medium">
                                    <Gem class="w-4 h-4" /> {{ $t('charactersTab.tags.collab') }}
                                </span> -->
>>>>>>> upstream/main
                            </div>
                        </div>

                        <button @click="show = false"
<<<<<<< HEAD
                            class="text-secondary cursor-pointer hover:text-primary transition-colors p-1 rounded hover:bg-interactive-bg-hover"
=======
                            class="text-text-secondary cursor-pointer hover:text-text-primary transition-colors p-1 rounded-sm hover:bg-state-hover"
>>>>>>> upstream/main
                            :aria-label="$t('common.actions.close')">
                            <X class="w-4 h-4" />
                        </button>
                    </div>

<<<<<<< HEAD
                    <div class="flex gap-5 mt-3">
                        <div>
                            <p class="text-base font-semibold leading-none">{{ enabledModsCount }}</p>
                            <p class="text-xs text-secondary mt-0.5">{{ $t('charactersTab.characterModal.enabledMods') }}</p>
                        </div>
                        <div>
                            <p class="text-base font-semibold leading-none">{{ installedMods.length }}</p>
                            <p class="text-xs text-secondary mt-0.5">{{ $t('charactersTab.characterModal.totalMods') }}</p>
=======
                    <div class="flex gap-5 mt-3 flex-1 items-center">
                        <div>
                            <p class="text-base font-semibold leading-none">{{ enabledModsCount }}</p>
                            <p class="text-xs text-text-secondary mt-0.5">{{ $t('charactersTab.characterModal.enabledMods') }}</p>
                        </div>
                        <div>
                            <p class="text-base font-semibold leading-none">{{ installedMods.length }}</p>
                            <p class="text-xs text-text-secondary mt-0.5">{{ $t('charactersTab.characterModal.totalMods') }}</p>
>>>>>>> upstream/main
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex flex-col min-h-0 flex-1">
                <TabGroup>
<<<<<<< HEAD
                    <TabList class="flex shrink-0 items-center gap-1 px-2 py-2 border-b border-border" as="div">
                        <Tab v-slot="{ selected }" key="modsTab" as="template">
                            <button class="px-4 py-1.5 text-sm rounded transition-colors outline-none cursor-pointer"
                                :class="selected ? 'bg-accent-primary text-primary font-medium' : 'text-secondary hover:text-primary'">
=======
                    <TabList class="flex shrink-0 items-center gap-1 px-2 py-2 border-b border-border-default" as="div">
                        <Tab v-slot="{ selected }" key="modsTab" as="template">
                            <button class="px-4 py-1.5 text-sm rounded-sm transition-colors outline-none cursor-pointer hover:bg-state-hover"
                                :class="selected ? 'bg-accent! text-text-on-accent font-medium' : 'text-text-secondary hover:text-text-primary'">
>>>>>>> upstream/main
                                {{ $t('charactersTab.characterModal.modsTab') }}
                            </button>
                        </Tab>
                        <Tab v-slot="{ selected }" key="discoverModsTab" as="template">
<<<<<<< HEAD
                            <button class="px-4 py-1.5 text-sm rounded transition-colors outline-none cursor-pointer"
                                :class="selected ? 'bg-accent-primary text-primary font-medium' : 'text-secondary hover:text-primary'">
=======
                            <button class="px-4 py-1.5 text-sm rounded-sm transition-colors outline-none cursor-pointer  hover:bg-state-hover"
                                :class="selected ? 'bg-accent! text-text-on-accent font-medium' : 'text-text-secondary hover:text-text-primary'">
>>>>>>> upstream/main
                                {{ $t('charactersTab.characterModal.discoverModsTab') }}
                            </button>
                        </Tab>
                    </TabList>

                    <TabPanels as="div" class="overflow-y-auto h-100">
                        <TabPanel key="modsPanel">
                            <div>
<<<<<<< HEAD
                                <div v-if="installedMods.length === 0" class="text-center py-12 px-4 text-secondary">
                                    <p class="text-sm font-medium mb-1">{{ $t('charactersTab.characterModal.noModsFound.title') }}</p>
                                    <p class="text-xs text-secondary">{{ $t('charactersTab.characterModal.noModsFound.description') }}</p>
                                </div>
                                <template v-else>
                                    <div v-for="(mods, type) in modsByType" :key="type" v-show="mods.length > 0">
                                        <div class="flex items-center justify-between px-4 py-2 bg-bg-surface border-b border-border sticky top-0 z-10">
                                            <span class="text-xs font-medium text-secondary uppercase tracking-wide">
                                                {{ $t(`charactersTab.modTypes.${type.toLowerCase()}`) }}
                                            </span>
                                            <span class="text-xs text-secondary">{{ mods.filter(m => m.enabled).length }}/{{ mods.length }}</span>
                                        </div>

                                        <label v-for="mod in mods" :key="mod.name"
                                            class="flex items-center gap-3 px-4 py-2.5 border-b border-border cursor-pointer hover:bg-interactive-bg-hover transition-colors"
                                            :class="{ 'bg-bg-surface': !mod.enabled }">
                                            <Checkbox :model-value="mod.enabled" @update:model-value="toggleMod(mod)" class="shrink-0" />
                                            <button @click.stop="openPreviewMod(mod)" :aria-label="$t('charactersTab.characterModal.previewMod')">
                                                <Eye class="w-6 h-6 cursor-pointer hover:text-primary! transition-colors active:scale-95 text-secondary" />
                                            </button>
                                            <div class="flex-1 min-w-0">
                                                <p class="text-sm truncate" :class="mod.enabled ? 'text-primary' : 'text-secondary'">{{ mod.name }}</p>
                                                <p v-if="mod.author" class="text-xs text-secondary mt-0.5">{{ mod.author }}</p>
                                                <p class="text-xs text-secondary mt-0.5 font-mono truncate" :title="mod.path">{{ mod.path }}</p>
=======
                                <div v-if="installedMods.length === 0" class="text-center py-12 px-4 text-text-secondary">
                                    <p class="text-sm font-medium mb-1">{{ $t('charactersTab.characterModal.noModsFound.title') }}</p>
                                    <p class="text-xs text-text-secondary">{{ $t('charactersTab.characterModal.noModsFound.description') }}</p>
                                </div>
                                <template v-else>
                                    <div v-for="(mods, type) in modsByType" :key="type" v-show="mods.length > 0">
                                        <div class="flex items-center justify-between px-4 py-2 bg-surface-dialog border-b border-border-default top-0 z-10">
                                            <span class="text-xs font-medium text-text-secondary uppercase tracking-wide">
                                                {{ $t(`charactersTab.modTypes.${type.toLowerCase()}`) }}
                                            </span>
                                            <span class="text-xs text-text-secondary">{{ mods.filter(m => m.enabled).length }}/{{ mods.length }}</span>
                                        </div>

                                        <label v-for="mod in mods" :key="mod.name"
                                            class="flex items-center gap-3 px-4 py-2.5 border-b border-border-default cursor-pointer hover:bg-state-hover transition-colors"
                                            :class="{ 'bg-surface-dialog': !mod.enabled }">
                                            <Checkbox :model-value="mod.enabled" @update:model-value="toggleMod(mod)" class="shrink-0" />
                                            <button @click.stop="openPreviewMod(mod)" :aria-label="$t('charactersTab.characterModal.previewMod')">
                                                <Eye class="w-6 h-6 cursor-pointer hover:text-text-primary! transition-colors active:scale-95 text-text-secondary" />
                                            </button>
                                            <div class="flex-1 min-w-0">
                                                <p class="text-sm truncate" :class="mod.enabled ? 'text-text-primary' : 'text-text-secondary'">{{ mod.name }}</p>
                                                <p v-if="mod.author" class="text-xs text-text-secondary mt-0.5">{{ mod.author }}</p>
>>>>>>> upstream/main
                                            </div>
                                        </label>
                                    </div>
                                </template>
                            </div>
                        </TabPanel>

                        <TabPanel key="discoverModsPanel">
                            <div>
<<<<<<< HEAD
                                <div class="bg-bg-surface/80 border border-border rounded-lg p-3 m-2 flex items-start gap-2">
                                    <Info class="w-5 h-5 text-secondary" />
                                    <p class="text-sm text-secondary font-medium text-wrap">
=======
                                <div class="bg-surface-card border border-border-default rounded-lg p-3 m-2 flex items-start gap-2">
                                    <Info class="w-5 h-5 text-text-secondary" />
                                    <p class="text-sm text-text-secondary font-medium text-wrap">
>>>>>>> upstream/main
                                        {{ $t('charactersTab.characterModal.discoverModsDescription', { latestUpdate: modsIndex.latestUpdate }) }}
                                    </p>
                                </div>
                                <template v-for="type in ['cutscene', 'standing', 'dating']">
                                    <div v-if="getIndexMods(type).length > 0"
<<<<<<< HEAD
                                        class="flex items-center justify-between px-4 py-2 bg-bg-surface border-b border-border sticky top-0 z-10">
                                        <span class="text-xs font-medium text-secondary uppercase tracking-wide">
=======
                                        class="flex items-center justify-between px-4 py-2 bg-surface-dialog border-b border-border-default sticky top-0 z-10">
                                        <span class="text-xs font-medium text-text-secondary uppercase tracking-wide">
>>>>>>> upstream/main
                                            {{ $t(`charactersTab.modTypes.${type}`) }}
                                        </span>
                                    </div>

                                    <label v-for="(mod, index) in getIndexMods(type)" :key="`${type}-${index}`"
<<<<<<< HEAD
                                        class="flex items-center gap-3 px-4 py-2.5 border-b border-border transition-colors">
=======
                                        class="flex items-center gap-3 px-4 py-2.5 border-b border-border-default transition-colors">
>>>>>>> upstream/main
                                        <div class="flex-1 min-w-0 flex items-center gap-2">
                                            <div class="relative flex items-center">
                                                <BadgeDollarSign class="w-5 h-5"
                                                    :class="{ 'text-success/70': !mod.is_paid, 'text-warning': mod.is_paid }"
                                                    @mouseenter="tooltipVisible = `${type}-${index}`"
                                                    @mouseleave="tooltipVisible = null" />
<<<<<<< HEAD
                                                <span v-show="tooltipVisible === `${type}-${index}`"
                                                    class="absolute top-0 left-full ml-2 text-xs text-primary font-medium bg-interactive-bg border border-border px-2 py-1 rounded whitespace-nowrap">
                                                    {{ mod.is_paid ? $t('charactersTab.characterModal.paidMod') : $t('charactersTab.characterModal.freeMod') }}
                                                </span>
                                            </div>

                                            <p class="text-sm truncate text-primary flex-1">{{ mod.authorData?.name }}</p>
=======
                                                
                                                    <span v-show="tooltipVisible === `${type}-${index}`"
                                                        class="absolute top-[50%] translate-y-[-50%] z-15 left-full ml-2 text-xs text-text-primary font-medium bg-surface-popover border border-border-default px-2 py-1 rounded-sm whitespace-nowrap">
                                                        {{ mod.is_paid ? $t('charactersTab.characterModal.paidMod') : $t('charactersTab.characterModal.freeMod') }}
                                                    </span>

                                            </div>

                                            <p class="text-sm truncate text-text-primary flex-1">{{ mod.authorData?.name }}</p>
>>>>>>> upstream/main

                                            <div class="flex flex-row gap-4">
                                                <div v-for="(link, key) in Object.fromEntries(Object.entries(mod.authorData?.links || {}).filter(([_, link]) => link))"
                                                    :key="key" class="flex items-center gap-1.5 cursor-pointer group"
                                                    @click="handleOpenLink(link)">
                                                    <component :is="getIconForLink(key)" class="w-4 h-4"
                                                        :color="{ 'patreon': '#FF424D', 'ko-fi': '#29ABE0', 'discord': '#5865F2', 'afdian': '#946CE6' }[key]" />
                                                    <span :title="link || ''"
<<<<<<< HEAD
                                                        class="text-sm text-primary font-medium group-hover:text-accent-primary! transition-colors">{{ key[0].toUpperCase() + key.slice(1) }}</span>
=======
                                                        class="text-sm text-text-primary font-medium transition-colors" :class="[
                                                            `group-hover:text-[${{ 'patreon': '#FF424D', 'ko-fi': '#29ABE0', 'discord': '#5865F2', 'afdian': '#946CE6' }[key]}]`
                                                        ]">{{ key[0].toUpperCase() + key.slice(1) }}</span>
>>>>>>> upstream/main
                                                </div>
                                            </div>
                                        </div>
                                    </label>
                                </template>
                                <div v-if="!hasIndexMods" class="text-center py-12 px-4">
<<<<<<< HEAD
                                    <p class="text-sm font-medium text-secondary mb-1">{{ $t('charactersTab.characterModal.noIndexModsFound.title') }}</p>
                                    <p class="text-xs text-secondary">{{ $t('charactersTab.characterModal.noIndexModsFound.description') }}</p>
=======
                                    <p class="text-sm font-medium text-text-secondary mb-1">{{ $t('charactersTab.characterModal.noIndexModsFound.title') }}</p>
                                    <p class="text-xs text-text-secondary">{{ $t('charactersTab.characterModal.noIndexModsFound.description') }}</p>
>>>>>>> upstream/main
                                </div>
                            </div>
                        </TabPanel>
                    </TabPanels>
                </TabGroup>
            </div>
        </div>
    </Modal>
</template>