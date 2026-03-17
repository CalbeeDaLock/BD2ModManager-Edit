<script setup lang="ts">
import { Character, isCostumeNew } from '../../stores/characters';
import { Check, X } from 'lucide-vue-next';
import Image from '../../components/common/Image.vue';

export interface CostumeWithMods extends Character {
    hasCutscene?: boolean;
    hasStanding?: boolean;
    hasDating?: boolean;
    modsCount: number;
}

const props = defineProps<{
    items: CostumeWithMods[];
}>();

defineEmits<{
    'open-mod-details': [costume: Character];
}>();
</script>

<template>
        <div v-if="items.length === 0" class="text-center py-12 text-secondary">
        <p class="text-lg">
            {{ $t('charactersTab.charactersNotFound') }}
        </p>
    </div>
    <div v-else
        class="bg-bg-deep overflow-y-auto grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
        <template v-for="costume in items" :key="costume.id">
            <div @click="$emit('open-mod-details', costume)" class="
                                rounded-lg
                                cursor-pointer 
                                transition-all 
                                bg-interactive-bg
                                text-primary
                                hover:bg-interactive-bg-hover/40
                                flex
                                flex-col
                                ">
                <div class="relative">
                    <Image loading="lazy" :src="`characters/standing/${costume.id}.png`" :alt="`${costume.character} - ${costume.costume}`"
                        class="w-full h-full rounded-md" error-src="characters/standing/placeholder_character.png" />
                    <div class="absolute top-2.5 left-2.5 flex gap-1">
                        <div v-if="costume.modsCount > 0"
                            class="  bg-accent-primary/75 backdrop-blur-sm text-white text-xs px-2 py-1 rounded-full font-medium">
                            {{ $t('charactersTab.tags.modsCount', {count: costume.modsCount}) }}
                        </div>
                        <div v-if="costume.is_collab"
                            class=" bg-yellow-500/75 backdrop-blur-sm text-yellow-100 text-xs px-2 py-1 rounded-full font-medium">
                            {{ $t('charactersTab.tags.collab') }}
                        </div>

                    </div>
                </div>

                <div class="px-2 py-2 flex flex-col flex-1">
                    <div class="mb-3">
                        <div class="font-semibold truncate text-md flex gap-2 items-center"
                            :title="costume.character + ' - ' + costume.costume">
                            <div v-if="isCostumeNew(costume)"
                                class=" bg-red-500/75 backdrop-blur-sm text-red-100 text-xs px-2 py-0.5 rounded-sm font-medium">
                                {{ $t('charactersTab.tags.new') }}
                            </div>
                            {{ costume.character }} - {{ costume.costume }}
                        </div>
                        <div class="text-secondary text-xs">
                            {{ $t('charactersTab.id', { id: costume.id }) }}
                        </div>
                    </div>

                    <div class="flex justify-around items-center gap-1 text-xs text-primary font-mono overflow-x-auto">
                        <div class="text-center p-1">
                            <div class="mb-1">
                                {{ $t('charactersTab.modTypes.cutscene') }}
                            </div>
                            <div class="flex items-center justify-center"
                                :class="costume.hasCutscene ? 'text-success font-bold' : 'text-danger'">
                                <Check class="w-[1.25em] h-[1.25em]" v-if="costume.hasCutscene" />
                                <X v-else class="w-[1.25em] h-[1.25em]" />
                            </div>
                        </div>

                        <div class="text-center p-1">
                            <div class=" mb-1">{{ $t('charactersTab.modTypes.standing') }}</div>
                            <div class="flex justify-center"
                                :class="costume.hasStanding ? 'text-success font-bold' : 'text-danger'">
                                <Check class="w-[1.25em] h-[1.25em]" v-if="costume.hasStanding" />
                                <X v-else class="w-[1.25em] h-[1.25em]" />
                            </div>
                        </div>

                        <div class="text-center p-1" v-if="costume.dating_id">
                            <div class=" mb-1">{{ $t('charactersTab.modTypes.dating') }}</div>
                            <div class="flex justify-center"
                                :class="costume.hasDating ? 'text-success font-bold' : 'text-danger'">
                                <Check class="w-[1.25em] h-[1.25em]" v-if="costume.hasDating" />
                                <X v-else class="w-[1.25em] h-[1.25em]" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </template>
    </div>
</template>