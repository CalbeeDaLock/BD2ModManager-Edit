<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/core'
import { CostumeWithMods } from './CharacterGrid.vue'
import { useAppDir } from '../../composables/useAppDir'
import { isCostumeNew } from '../../stores/characters'
import Image from '../../components/common/Image.vue'
import { Check, X } from 'lucide-vue-next'

const baseDir = useAppDir()

defineProps<{
    costume: CostumeWithMods
}>()
</script>

<template>
    <div @click="$emit('open-mod-details', costume)"
        class="rounded-lg cursor-pointer transition-all bg-interactive-bg text-primary hover:bg-interactive-bg-hover/40 flex flex-col h-full overflow-hidden">
        <div class="relative">
            <Image loading="lazy" :src="`characters/standing/${costume.id}.png`"
                :alt="`${costume.character} - ${costume.costume}`"
                class="w-full h-full object-cover object-top rounded-t-md" :fallback-sources="[
                    convertFileSrc(`${baseDir}/assets/standing/${costume.id}.png`),
                    '/characters/standing/placeholder_character.png'
                ]" />
            <div class="absolute top-2.5 left-2.5 flex gap-1">
                <div v-if="costume.modsCount > 0"
                    class="bg-accent-primary/75 backdrop-blur-sm text-white text-xs px-2 py-1 rounded-full font-medium">
                    {{ $t('charactersTab.tags.modsCount', { count: costume.modsCount }) }}
                </div>
                <div v-if="costume.is_collab"
                    class="bg-yellow-500/75 backdrop-blur-sm text-yellow-100 text-xs px-2 py-1 rounded-full font-medium">
                    {{ $t('charactersTab.tags.collab') }}
                </div>
            </div>
        </div>

        <div class="px-2 py-2 flex flex-col shrink-0">
            <div class="mb-2">
                <div class="font-semibold text-md min-w-0 flex gap-2 items-center"
                    :title="costume.character + ' - ' + costume.costume">
                    <div v-if="isCostumeNew(costume)"
                        class="bg-red-500/75 backdrop-blur-sm text-red-100 text-xs px-2 py-0.5 rounded-sm font-medium shrink-0">
                        {{ $t('charactersTab.tags.new') }}
                    </div>
                    <span class="truncate">
                        {{ costume.character }} - {{ costume.costume }}
                    </span>
                </div>
                <div class="text-secondary text-xs">
                    {{ $t('charactersTab.id', { id: costume.id }) }}
                </div>
            </div>

            <div ref="modStatusRef"
                class="flex justify-around items-center gap-2 text-xs text-primary font-mono overflow-x-auto scrollbar-hide">
                <div class="text-center p-1">
                    <div class="mb-1 flex items-center font-semibold gap-1">
                        {{ $t('charactersTab.modTypes.cutscene') }}
                    </div>
                    <div class="flex items-center justify-center"
                        :class="costume.hasCutscene ? 'text-success font-bold' : 'text-danger'">
                        <Check class="w-[1.25em] h-[1.25em]" v-if="costume.hasCutscene" />
                        <X class="w-[1.25em] h-[1.25em]" v-else />
                    </div>
                </div>

                <div class="text-center p-1">
                    <div class="mb-1 flex items-center font-bold gap-1">
                        {{ $t('charactersTab.modTypes.standing') }}
                    </div>
                    <div class="flex justify-center"
                        :class="costume.hasStanding ? 'text-success font-bold' : 'text-danger'">
                        <Check class="w-[1.25em] h-[1.25em]" v-if="costume.hasStanding" />
                        <X class="w-[1.25em] h-[1.25em]" v-else />
                    </div>
                </div>

                <div class="text-center p-1" v-if="costume.dating_id">
                    <div class="mb-1 flex items-center font-bold gap-1">
                        {{ $t('charactersTab.modTypes.dating') }}
                    </div>
                    <div class="flex justify-center"
                        :class="costume.hasDating ? 'text-success font-bold' : 'text-danger'">
                        <Check class="w-[1.25em] h-[1.25em]" v-if="costume.hasDating" />
                        <X class="w-[1.25em] h-[1.25em]" v-else />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style>

</style>