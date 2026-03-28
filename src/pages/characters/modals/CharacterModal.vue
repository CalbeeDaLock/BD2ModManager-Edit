<script setup lang="ts">
import { X, Calendar, Tag, Gem, Eye } from 'lucide-vue-next';
import { Character } from '../../../stores/characters';
import { BD2Mod, useModsStore } from '../../../stores/mods';
import { computed } from 'vue';
import Button from '../../../components/common/Button.vue';
import Image from '../../../components/common/Image.vue';
import Modal from '../../../components/common/Modal.vue';
import Checkbox from '../../../components/common/Checkbox.vue';
import { useLoggingStore } from '../../../stores/logging';
import { getErrorMessage } from '../../../utils/errors';
import { useToast } from 'primevue/usetoast';
import { useI18n } from 'vue-i18n';
import { convertFileSrc } from '@tauri-apps/api/core';
import { useAppDir } from '../../../composables/useAppDir';

const loggingStore = useLoggingStore();
const toast = useToast()
const { t } = useI18n()

const show = defineModel('show', {
    type: Boolean,
    required: true,
});

const props = defineProps<{
    selectedCostume: Character | null;
    toggleMod: (mod: BD2Mod) => void;
}>();

const modsStore = useModsStore();

const installedMods = computed(() => {
    if (!props.selectedCostume) return [];
    return modsStore.mods.filter(mod => {
        if (!mod.modType) return false;
        const { type } = mod.modType;
        if (['Cutscene', 'Standing'].includes(type)) {
            return 'id' in mod.modType && mod.modType.id === props.selectedCostume?.id;
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
        if (mod.modType?.type && mod.modType.type in grouped) {
            grouped[mod.modType.type as keyof typeof grouped].push(mod);
        }
    });

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
        toast.add({
            severity: "error",
            closable: true,
            summary: t("modsTab.errors.modPreview.title"),
            detail: errorMsg,
            life: 5000
        })

        loggingStore.logError("Error previewing mod:", error);
    })
}

const baseDir = useAppDir()
</script>

<template>
    <Modal v-model:show="show" class="min-w-150 max-h-[85vh]">
        <template #header></template>
        <template #footer>
            <div class="flex p-3 justify-end items-center w-full border-t border-border">
                <Button variant="default" :label="$t('common.actions.close')" @click="show = false" />
            </div>
        </template>

        <div v-if="selectedCostume" class="overflow-y-auto min-h-0 text-primary">

            <div class="flex items-stretch border-b border-border">
                <Image :src="`characters/standing/${selectedCostume.id}.png`"
                    :alt="`${selectedCostume?.character} - ${selectedCostume.costume}`"
                    class="w-40 h-40 object-cover shrink-0 border-r border-border"
                    :fallback-sources="[
                        convertFileSrc(`${baseDir}/assets/standing/${selectedCostume?.id}.png`),
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
                                    <Tag class="w-4 h-4" />
                                    {{ selectedCostume.id }}
                                </span>
                                <span v-if="selectedCostume.release_date" class="flex items-center gap-1">
                                    <Calendar class="w-4 h-4" />
                                    {{ new Date(selectedCostume.release_date).toLocaleDateString() }}
                                </span>
                                <span v-if="selectedCostume.is_collab"
                                    class="flex items-center gap-1 px-1.5 py-0.5 bg-accent-primary/20 text-accent-primary rounded text-xs font-medium">
                                    <Gem class="w-4 h-4" /> {{ $t('characters.tags.collab') }}
                                </span>
                            </div>
                        </div>

                        <button @click="show = false"
                            class="text-secondary cursor-pointer hover:text-primary transition-colors p-1 rounded hover:bg-interactive-bg-hover"
                            :aria-label="$t('common.actions.close')">
                            <X class="w-4 h-4" />
                        </button>
                    </div>

                    <div class="flex gap-5 mt-3">
                        <div>
                            <p class="text-base font-semibold leading-none">{{ enabledModsCount }}</p>
                            <p class="text-xs text-secondary mt-0.5">
                                {{ $t('charactersTab.characterModal.enabledMods') }}
                            </p>
                        </div>
                        <div>
                            <p class="text-base font-semibold leading-none">{{ installedMods.length }}</p>
                            <p class="text-xs text-secondary mt-0.5">
                                {{ $t('charactersTab.characterModal.totalMods') }}
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            <div>
                <div v-if="installedMods.length === 0" class="text-center py-12 px-4 text-secondary">
                    <p class="text-sm font-medium mb-1">{{ $t('charactersTab.characterModal.noModsFound.title') }}</p>
                    <p class="text-xs text-secondary">{{ $t('charactersTab.characterModal.noModsFound.description') }}
                    </p>
                </div>

                <template v-else>
                    <div v-for="(mods, type) in modsByType" :key="type" v-show="mods.length > 0">
                        <div
                            class="flex items-center justify-between px-4 py-2 bg-bg-surface border-b border-border sticky top-0 z-10">
                            <span class="text-xs font-medium text-secondary uppercase tracking-wide">{{
                                $t(`charactersTab.modTypes.${type.toLowerCase()}`) }}</span>
                            <span class="text-xs text-secondary">
                                {{mods.filter(m => m.enabled).length}}/{{ mods.length }}
                            </span>
                        </div>

                        <label v-for="mod in mods" :key="mod.name"
                            class="flex items-center gap-3 px-4 py-2.5 border-b border-border cursor-pointer hover:bg-interactive-bg-hover transition-colors"
                            :class="{ 'bg-bg-surface': !mod.enabled }">
                            <Checkbox :model-value="mod.enabled" @update:model-value="toggleMod(mod)"
                                class="shrink-0" />
                            <button @click.stop="openPreviewMod(mod)"
                                :aria-label="$t('charactersTab.characterModal.previewMod')">
                                <Eye
                                    class="w-6 h-6 cursor-pointer hover:text-primary! transition-colors active:scale-95 text-secondary" />
                            </button>
                            <div class="flex-1 min-w-0">
                                <p class="text-sm truncate" :class="mod.enabled ? 'text-primary' : 'text-secondary'">
                                    {{ mod.name }}
                                </p>
                                <p v-if="mod.author" class="text-xs text-secondary mt-0.5">{{ mod.author }}</p>
                            </div>
                        </label>

                    </div>
                </template>
            </div>

        </div>
    </Modal>
</template>