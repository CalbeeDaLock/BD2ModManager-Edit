<script setup lang="ts">
import Image from '../../components/common/Image.vue';
import { Character, isCostumeNew } from '../../stores/characters';
import { formatCharName, useLang } from '../../utils/formatCharName.ts';
import { CharacterListItem, CostumeWithMods } from './CharacterList.vue';
import { convertFileSrc } from '@tauri-apps/api/core';
import { computed } from 'vue';

const props = defineProps<{
    item: CharacterListItem
}>()

const emit = defineEmits<{
    openModDetails: [costume: Character]
}>()

const imageUrl = computed(() => {
    if (props.item.type !== 'costume') return '#'
    const costume = props.item.data
    const ids = Array.isArray(costume.id)
        ? costume.id.join(',')
        : costume.id
    return convertFileSrc(`standing/${ids}`, "bd2assets")
})

const lang = useLang()

const charName = computed(() => {   
    if (props.item.type == "header") return
    return formatCharName(props.item.data as CostumeWithMods, lang.value)
})
</script>

<template>
    <div v-if="item.type === 'header'" class="text-lg font-bold px-2 pb-1">
        {{ item.data }}
    </div>

    <div v-else-if="item.type === 'costume'"
        @click="emit('openModDetails', item.data)"
        class="flex bg-surface-card rounded-lg overflow-hidden cursor-pointer hover:bg-state-hover transition-colors mx-2 mb-2">

        <div class="shrink-0">
            <Image
                :src="imageUrl"
                :alt="charName"
                class="w-42 h-42 object-cover object-top rounded-t-md aspect-square"
                error-src="characters/standing/placeholder_character.png"
                error-class="bg-text-primary"
                skeleton
            />
        </div>

        <div class="flex flex-col flex-1 p-2 min-w-0">
            <div class="text-lg font-medium flex gap-2 items-center flex-wrap">
                <div v-if="isCostumeNew(item.data)"
                    class="bg-error-bg text-error text-xs px-2 py-0.5 rounded-sm font-medium">
                    {{ $t('charactersTab.tags.new') }}
                </div>

                <span class="truncate">
                    {{ charName }}
                </span>

                <div v-if="item.data.modsCount > 0"
                    class="flex bg-accent/75 text-text-on-accent text-xs px-2 py-1 rounded-full font-medium whitespace-nowrap">
                    {{ $t('charactersTab.tags.modsCount', { count: item.data.modsCount }) }}
                </div>

                <div v-if="item.data.is_collab"
                    class="bg-warning-bg text-warning text-xs px-2 py-1 rounded-full font-medium">
                    {{ $t('charactersTab.tags.collab') }}
                </div>
            </div>

            <div class="flex flex-1 items-end gap-8 md:gap-12 mr-4 md:mr-8 mt-2">
                <div class="flex flex-col items-center">
                    <span class="font-semibold text-sm md:text-base">
                        {{ $t('charactersTab.modTypes.cutscene') }}
                    </span>
                    <span class="font-mono text-xs md:text-sm" :class="{
                        'text-success': item.data.hasCutscene,
                        'text-error': !item.data.hasCutscene
                    }">
                        {{ item.data.hasCutscene ? $t('charactersTab.modTypes.states.enabled', 'Enabled') : $t('charactersTab.modTypes.states.disabled', 'Disabled') }}
                    </span>
                </div>

                <div class="flex flex-col items-center">
                    <span class="font-semibold text-sm md:text-base">
                        {{ $t('charactersTab.modTypes.standing') }}
                    </span>
                    <span class="font-mono text-xs md:text-sm" :class="{
                        'text-success': item.data.hasStanding,
                        'text-error': !item.data.hasStanding
                    }">
                        {{ item.data.hasStanding ? $t('charactersTab.modTypes.states.enabled', 'Enabled') : $t('charactersTab.modTypes.states.disabled', 'Disabled') }}
                    </span>
                </div>

                <div v-if="item.data.dating_id" class="flex flex-col items-center">
                    <span class="font-semibold text-sm md:text-base">
                        {{ $t('charactersTab.modTypes.dating') }}
                    </span>
                    <span class="font-mono text-xs md:text-sm" :class="{
                        'text-success': item.data.hasDating,
                        'text-error': !item.data.hasDating
                    }">
                        {{ item.data.hasDating ? $t('charactersTab.modTypes.states.enabled', 'Enabled') : $t('charactersTab.modTypes.states.disabled', 'Disabled') }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>