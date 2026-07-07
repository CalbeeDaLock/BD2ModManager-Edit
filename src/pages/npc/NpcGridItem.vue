<script setup lang="ts">
import { computed } from 'vue';
import { Check, X } from 'lucide-vue-next';
import Image from '../../components/common/Image.vue';
import type { NpcEntry } from './types';
import { getNpcIcon } from './npcIcons';

const props = defineProps<{
    npc: NpcEntry
}>();

defineEmits<{
    'open-mod-details': [npc: NpcEntry]
}>();

const PLACEHOLDER = 'characters/standing/placeholder_character.png';

const imageUrl = computed(() => getNpcIcon(props.npc.id) ?? PLACEHOLDER);
</script>

<template>
    <div @click="$emit('open-mod-details', npc)"
        class="rounded-lg cursor-pointer transition-all bg-surface-card text-text-primary hover:bg-state-hover flex flex-col h-full overflow-hidden">
        <div class="relative">
            <Image :src="imageUrl" :alt="npc.name" class="w-full aspect-square rounded-t-md"
                error-src="characters/standing/placeholder_character.png" skeleton error-class="bg-text-primary" />
            <div class="absolute top-2.5 left-2.5 flex gap-1">
                <div v-if="npc.modsCount > 0"
                    class="bg-accent/75 text-text-on-accent text-xs px-2 py-1 rounded-full font-medium">
                    {{ $t('npcTab.tags.modsCount', { count: npc.modsCount }) }}
                </div>
            </div>
        </div>

        <div class="px-2 py-2 flex flex-col shrink-0">
            <div class="mb-2 min-w-0">
                <div class="font-semibold text-sm truncate" :title="npc.name">{{ npc.name }}</div>
                <div class="text-text-secondary text-xs truncate">
                    {{ $t('npcTab.id', { id: npc.id }) }}
                </div>
            </div>

            <div
                class="flex justify-around items-center gap-2 text-xs text-text-primary font-mono overflow-x-auto scrollbar-hide">
                <div class="text-center p-1">
                    <div class="mb-1 flex items-center font-bold gap-1">
                        {{ $t('charactersTab.modTypes.standing') }}
                    </div>
                    <div class="flex justify-center"
                        :class="npc.hasStanding ? 'text-success font-bold' : 'text-error'">
                        <Check class="w-[1.25em] h-[1.25em]" v-if="npc.hasStanding" />
                        <X class="w-[1.25em] h-[1.25em]" v-else />
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style></style>
