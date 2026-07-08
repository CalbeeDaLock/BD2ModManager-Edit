<script setup lang="ts">
import { X, Calendar, Eye, Tag } from 'lucide-vue-next';
import { Character, useCharactersStore } from '../../../stores/characters';
import { BD2Mod, useModsStore } from '../../../stores/mods';
import { computed } from 'vue';
import Button from '../../../components/common/Button.vue';
import Image from '../../../components/common/Image.vue';
import Modal from '../../../components/common/Modal.vue';
import Checkbox from '../../../components/common/Checkbox.vue';
import { useLoggingStore } from '../../../stores/logging';
import { getErrorMessage } from '../../../utils/errors';
import { useI18n } from 'vue-i18n';
import { convertFileSrc } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notification.ts';
import { getCharName, useLang } from '../../../utils/formatCharName.ts';
import { sortModsByPath } from '../../../utils/sortMods';

const loggingStore = useLoggingStore();
const notificationStore = useNotificationStore();
const { t } = useI18n();

const show = defineModel('show', {
    type: Boolean,
    required: true,
});

const props = defineProps<{
    selectedCostume: Character | null;
    toggleMod: (mod: BD2Mod) => void;
}>();

const modsStore = useModsStore();
const charactersStore = useCharactersStore();

// Only dating mods for the selected character's dating_id.
const installedMods = computed<BD2Mod[]>(() => {
    if (!props.selectedCostume?.dating_id) return [];
    const datingId = props.selectedCostume.dating_id;
    return sortModsByPath(
        modsStore.mods.filter(mod =>
            mod.modType?.type === 'Dating' &&
            'id' in mod.modType &&
            mod.modType.id === datingId
        )
    );
});

const enabledModsCount = computed(() =>
    installedMods.value.filter(mod => mod.enabled).length
);

// Labelled affection entries (from dating.json) paired with their matching
// Scene-typed mod, when one is installed. Entries with no installed mod are
// still shown (greyed out) so the user can see what's missing.
interface AffectionRow {
    label: string;
    id: string;
    mod: BD2Mod | null;
}

const affectionRows = computed<AffectionRow[]>(() => {
    if (!props.selectedCostume?.dating_id) return [];
    const affection = charactersStore.getAffection(props.selectedCostume.dating_id);
    return affection.map(entry => {
        const mod = modsStore.mods.find(m =>
            m.modType?.type === 'Scene' &&
            'id' in m.modType &&
            m.modType.id === entry.id
        ) ?? null;
        return { label: entry.label, id: entry.id, mod };
    });
});

const enabledAffectionCount = computed(() =>
    affectionRows.value.filter(r => r.mod?.enabled).length
);

async function openPreviewMod(mod: BD2Mod) {
    modsStore.previewMod(mod.name).then(() => {
        loggingStore.logDebug("Mod previewed successfully:", mod.name);
    }).catch((error) => {
        const errorMsg = getErrorMessage(t, error);
        notificationStore.add({
            severity: "error",
            closable: true,
            title: t("modsTab.errors.modPreview.title"),
            message: errorMsg,
            duration: 5000
        });
        loggingStore.logError("Error previewing mod:", error);
    });
}

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
</script>

<template>
    <Modal v-model:show="show" class="w-[50vw] max-h-[85vh]" @close="() => show = false">
        <template #footer>
            <div class="flex p-3 justify-end items-center w-full border-t border-border-default">
                <Button variant="default" :label="$t('common.actions.close')" @click="show = false" />
            </div>
        </template>
        <template #header></template>

        <div v-if="selectedCostume" class="flex flex-col min-h-0 text-text-primary overflow-hidden">
            <div class="flex items-stretch border-b border-border-default shrink-0">
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
                            <div class="flex flex-col items-start gap-3 mt-1 text-xs text-text-secondary">
                                <span class="flex items-center gap-1">
                                    <Tag class="w-4 h-4"/>
                                    {{ Array.isArray(selectedCostume.id) ? selectedCostume.id.join(', ') : selectedCostume.id }}
                                </span>
                                <span v-if="selectedCostume.release_date" class="flex items-center gap-1">
                                    <Calendar class="w-4 h-4" />
                                    {{ new Date(selectedCostume.release_date).toLocaleDateString() }}
                                </span>
                            </div>
                        </div>
                        <button @click="show = false"
                            class="text-text-secondary cursor-pointer hover:text-text-primary transition-colors p-1 rounded-sm hover:bg-state-hover"
                            :aria-label="$t('common.actions.close')">
                            <X class="w-4 h-4" />
                        </button>
                    </div>

                    <div class="flex gap-5 mt-3 flex-1 items-end">
                        <div>
                            <p class="text-base font-semibold leading-none">{{ enabledModsCount }}</p>
                            <p class="text-xs text-text-secondary mt-0.5">{{ $t('charactersTab.characterModal.enabledMods') }}</p>
                        </div>
                        <div>
                            <p class="text-base font-semibold leading-none">{{ installedMods.length }}</p>
                            <p class="text-xs text-text-secondary mt-0.5">{{ $t('charactersTab.characterModal.totalMods') }}</p>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex flex-col min-h-0 flex-1 overflow-y-auto h-100">
                <div v-if="installedMods.length === 0 && affectionRows.length === 0" class="text-center py-12 px-4 text-text-secondary">
                    <p class="text-sm font-medium mb-1">{{ $t('charactersTab.characterModal.noModsFound.title') }}</p>
                    <p class="text-xs text-text-secondary">{{ $t('charactersTab.characterModal.noModsFound.description') }}</p>
                </div>
                <template v-else>
                    <!-- Dating mods section -->
                    <template v-if="installedMods.length > 0">
                        <div class="flex items-center justify-between px-4 py-2 bg-surface-dialog border-b border-border-default top-0 z-10">
                            <span class="text-xs font-medium text-text-secondary uppercase tracking-wide">
                                {{ $t('charactersTab.modTypes.dating') }}
                            </span>
                            <span class="text-xs text-text-secondary">{{ enabledModsCount }}/{{ installedMods.length }}</span>
                        </div>
                        <label v-for="mod in installedMods" :key="mod.name"
                            class="flex items-center gap-3 px-4 py-2.5 border-b border-border-default cursor-pointer hover:bg-state-hover transition-colors"
                            :class="{ 'bg-surface-dialog': !mod.enabled }">
                            <Checkbox :model-value="mod.enabled" @update:model-value="toggleMod(mod)" class="shrink-0" />
                            <button @click.stop="openPreviewMod(mod)" :aria-label="$t('charactersTab.characterModal.previewMod')">
                                <Eye class="w-6 h-6 cursor-pointer hover:text-text-primary! transition-colors active:scale-95 text-text-secondary" />
                            </button>
                            <div class="flex-1 min-w-0">
                                <p class="text-sm truncate" :class="mod.enabled ? 'text-text-primary' : 'text-text-secondary'">{{ mod.name }}</p>
                                <p v-if="mod.author" class="text-xs text-text-secondary mt-0.5">{{ mod.author }}</p>
                            </div>
                        </label>
                    </template>

                    <!-- Affection mods section (labelled from dating.json) -->
                    <template v-if="affectionRows.length > 0">
                        <div class="flex items-center justify-between px-4 py-2 bg-surface-dialog border-b border-border-default top-0 z-10">
                            <span class="text-xs font-medium text-text-secondary uppercase tracking-wide">
                                {{ $t('datingTab.affection.title') }}
                            </span>
                            <span class="text-xs text-text-secondary">{{ enabledAffectionCount }}/{{ affectionRows.length }}</span>
                        </div>
                        <label v-for="row in affectionRows" :key="row.id"
                            class="flex items-center gap-3 px-4 py-2.5 border-b border-border-default transition-colors"
                            :class="row.mod
                                ? ['cursor-pointer hover:bg-state-hover', { 'bg-surface-dialog': !row.mod.enabled }]
                                : 'opacity-60'">
                            <Checkbox :model-value="!!row.mod?.enabled" :disabled="!row.mod"
                                @update:model-value="row.mod && toggleMod(row.mod)" class="shrink-0" />
                            <button v-if="row.mod" @click.stop="openPreviewMod(row.mod)" :aria-label="$t('charactersTab.characterModal.previewMod')">
                                <Eye class="w-6 h-6 cursor-pointer hover:text-text-primary! transition-colors active:scale-95 text-text-secondary" />
                            </button>
                            <div v-else class="w-6 h-6 shrink-0" />
                            <div class="flex-1 min-w-0">
                                <p class="text-sm truncate" :class="row.mod?.enabled ? 'text-text-primary' : 'text-text-secondary'">
                                    {{ $t('datingTab.affection.label', { label: row.label }) }}
                                </p>
                                <p v-if="row.mod" class="text-xs text-text-secondary mt-0.5 truncate">{{ row.mod.name }}</p>
                                <p v-else class="text-xs text-text-secondary mt-0.5 italic">{{ $t('datingTab.affection.notInstalled') }}</p>
                            </div>
                        </label>
                    </template>
                </template>
            </div>
        </div>
    </Modal>
</template>
