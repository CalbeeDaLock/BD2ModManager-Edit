<script setup lang="ts">
import Image from '../../components/common/Image.vue';
import { Character } from '../../stores/characters';
import { CharacterListItem } from './CharacterList.vue';

const props = defineProps<{
    item: CharacterListItem
}>()

const emit = defineEmits<{
    openModDetails: [costume: Character]
}>()

function isCostumeNew(costume: Character): boolean {
    if (!costume.release_date) return false;
    const daysCount = 31;
    const releaseDate = new Date(costume.release_date).getTime();
    const daysAgo = Date.now() - (daysCount * 24 * 60 * 60 * 1000);
    return releaseDate > daysAgo;
}
</script>

<template>
    <div v-if="item.type === 'header'" class="text-lg font-bold px-2 pb-1">
        {{ item.data }}
    </div>

    <div v-else-if="item.type === 'costume'"
        @click="emit('openModDetails', item.data)"
        class="flex bg-interactive-bg rounded-lg overflow-hidden cursor-pointer hover:bg-interactive-bg-hover/30 transition-colors mx-2 mb-2">
        
        <div class="shrink-0">
            <Image
                loading="lazy"
                :src="`characters/standing/${item.data.id}.png`"
                :alt="`${item.data.character} - ${item.data.costume}`"
                class="w-42 h-42 object-cover"
                error-src="/characters/standing/placeholder_character.png"
            />
        </div>

        <div class="flex flex-col flex-1 p-2 min-w-0">
            <div class="text-lg font-medium flex gap-2 items-center flex-wrap">
                <div v-if="isCostumeNew(item.data)"
                    class="bg-red-500/75 backdrop-blur-sm text-red-100 text-xs px-2 py-0.5 rounded-sm font-medium">
                    {{ $t('characters.tags.new') }}
                </div>

                <span class="truncate">
                    {{ item.data.character }} - {{ item.data.costume }}
                </span>

                <div v-if="item.data.modsCount > 0"
                    class="flex bg-accent-primary/75 backdrop-blur-sm text-white text-xs px-2 py-1 rounded-full font-medium whitespace-nowrap">
                    {{ item.data.modsCount }} mod{{ item.data.modsCount !== 1 ? 's' : '' }}
                </div>

                <div v-if="item.data.is_collab"
                    class="bg-yellow-500/75 backdrop-blur-sm text-yellow-100 text-xs px-2 py-1 rounded-full font-medium">
                    {{ $t('characters.tags.collab') }}
                </div>
            </div>

            <span class="text-secondary font-mono text-sm">
                {{ $t('characters.id') }}: {{ item.data.id }}
            </span>

            <div class="flex flex-1 items-end gap-8 md:gap-12 mr-4 md:mr-8 mt-2">
                <div class="flex flex-col items-center">
                    <span class="font-semibold text-sm md:text-base">
                        {{ $t('characters.filters.Cutscene') }}
                    </span>
                    <span class="font-mono text-xs md:text-sm" :class="{
                        'text-success': item.data.hasCutscene,
                        'text-danger': !item.data.hasCutscene
                    }">
                        {{ item.data.hasCutscene ? $t('common.states.enabled', 'Enabled') : $t('common.states.disabled', 'Disabled') }}
                    </span>
                </div>

                <div class="flex flex-col items-center">
                    <span class="font-semibold text-sm md:text-base">
                        {{ $t('characters.filters.Standing') }}
                    </span>
                    <span class="font-mono text-xs md:text-sm" :class="{
                        'text-success': item.data.hasStanding,
                        'text-danger': !item.data.hasStanding
                    }">
                        {{ item.data.hasStanding ? $t('common.states.enabled', 'Enabled') : $t('common.states.disabled', 'Disabled') }}
                    </span>
                </div>

                <div v-if="item.data.dating_id" class="flex flex-col items-center">
                    <span class="font-semibold text-sm md:text-base">
                        {{ $t('characters.filters.Dating') }}
                    </span>
                    <span class="font-mono text-xs md:text-sm" :class="{
                        'text-success': item.data.hasDating,
                        'text-danger': !item.data.hasDating
                    }">
                        {{ item.data.hasDating ? $t('common.states.enabled', 'Enabled') : $t('common.states.disabled', 'Disabled') }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>