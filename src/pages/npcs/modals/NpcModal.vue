<script setup lang="ts">
import { X, Eye } from 'lucide-vue-next';
import { computed } from 'vue';
import { BD2Mod, useModsStore } from '../../../stores/mods';
import { useLoggingStore } from '../../../stores/logging';
import { getErrorMessage } from '../../../utils/errors';
import { useToast } from 'primevue/usetoast';
import { useI18n } from 'vue-i18n';
import Button from '../../../components/common/Button.vue';
import Modal from '../../../components/common/Modal.vue';
import Checkbox from '../../../components/common/Checkbox.vue';
import { sortModsByPath } from '../../../utils/sortMods';
import { getNpcIcon } from '../npcIcons';
import type { NpcEntry } from '../types';

const loggingStore = useLoggingStore();
const toast = useToast();
const { t } = useI18n();
const modsStore = useModsStore();

const show = defineModel('show', {
    type: Boolean,
    required: true,
});

const props = defineProps<{
    selectedNpc: NpcEntry | null;
    toggleMod: (mod: BD2Mod) => void;
}>();

const installedMods = computed<BD2Mod[]>(() => {
    const selected = props.selectedNpc;
    if (!selected) return [];
    return modsStore.mods.filter(mod =>
        mod.modType?.type === 'NPC' &&
        'id' in mod.modType &&
        mod.modType.id === selected.id
    );
});

// Single type group (NPC) — sorted A-Z by folder path via the shared helper so
// the modal behaves identically to the per-character CharacterModal.
const sortedMods = computed(() => sortModsByPath(installedMods.value));

const iconSrc = computed(() => props.selectedNpc ? getNpcIcon(props.selectedNpc.id) : undefined);

const enabledModsCount = computed(() =>
    installedMods.value.filter(mod => mod.enabled).length
);

async function openPreviewMod(mod: BD2Mod) {
    modsStore.previewMod(mod.name).then(() => {
        loggingStore.logDebug("Mod previewed successfully:", mod.name);
    }).catch((error) => {
        const errorMsg = getErrorMessage(t, error);
        toast.add({
            severity: "error",
            closable: true,
            summary: t("modsTab.errors.modPreview.title"),
            detail: errorMsg,
            life: 5000
        });
        loggingStore.logError("Error previewing mod:", error);
    });
}
</script>

<template>
    <Modal v-model:show="show" class="w-[50vw] max-h-[85vh]" @close="() => show = false">
        <template #footer>
            <div class="flex p-3 justify-end items-center w-full border-t border-border">
                <Button variant="default" :label="$t('common.actions.close')" @click="show = false" />
            </div>
        </template>
        <template #header></template>

        <div v-if="selectedNpc" class="flex flex-col min-h-0 text-primary overflow-hidden">
            <div class="flex items-stretch border-b border-border shrink-0">
                <div class="w-40 h-40 flex items-center justify-center bg-accent-primary/10 border-r border-border shrink-0 overflow-hidden">
                    <img v-if="iconSrc" :src="iconSrc" class="w-full h-full object-contain" :alt="selectedNpc.name" />
                    <span v-else class="font-mono text-2xl font-bold text-accent-primary/70 select-none">
                        {{ selectedNpc.id }}
                    </span>
                </div>

                <div class="flex-1 px-4 py-3">
                    <div class="flex items-start justify-between gap-2">
                        <div>
                            <div class="flex items-baseline gap-2">
                                <h3 class="font-semibold text-base text-primary">{{ selectedNpc.name }}</h3>
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
                            <p class="text-xs text-secondary mt-0.5">{{ $t('npcsTab.npcModal.enabledMods') }}</p>
                        </div>
                        <div>
                            <p class="text-base font-semibold leading-none">{{ installedMods.length }}</p>
                            <p class="text-xs text-secondary mt-0.5">{{ $t('npcsTab.npcModal.totalMods') }}</p>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex flex-col min-h-0 flex-1 overflow-y-auto">
                <div v-if="installedMods.length === 0" class="text-center py-12 px-4 text-secondary">
                    <p class="text-sm font-medium mb-1">{{ $t('npcsTab.npcModal.noModsFound.title') }}</p>
                    <p class="text-xs text-secondary">{{ $t('npcsTab.npcModal.noModsFound.description') }}</p>
                </div>
                <template v-else>
                    <label v-for="mod in sortedMods" :key="mod.name"
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
                        </div>
                    </label>
                </template>
            </div>
        </div>
    </Modal>
</template>
