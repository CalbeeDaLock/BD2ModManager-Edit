<script setup lang="ts">
import { computed } from 'vue';
import type { NpcEntry } from './types';
import { getNpcIcon } from './npcIcons';

const props = defineProps<{
    npc: NpcEntry
}>();

defineEmits<{
    'open-mod-details': [npc: NpcEntry]
}>();

const iconSrc = computed(() => getNpcIcon(props.npc.id));
</script>

<template>
    <div @click="$emit('open-mod-details', npc)"
        class="rounded-lg cursor-pointer transition-all bg-interactive-bg text-primary hover:bg-interactive-bg-hover/40 flex flex-col h-full overflow-hidden">
        <!-- NPC mods don't have a standing-image asset like costumes do, so we use
             a simple colored header band with the NPC's id, matching the visual
             weight of the character grid cards. -->
        <div class="relative h-32 flex items-center justify-center bg-accent-primary/10 border-b border-border overflow-hidden">
            <img v-if="iconSrc" :src="iconSrc" :alt="npc.name"
                class="absolute inset-0 w-full h-full object-cover object-top select-none pointer-events-none" />
            <span v-else class="font-mono text-2xl font-bold text-accent-primary/70 select-none">
                {{ npc.id }}
            </span>
            <div class="absolute top-2.5 left-2.5 flex gap-1">
                <div v-if="npc.modsCount > 0"
                    class="bg-accent-primary/75 backdrop-blur-sm text-white text-xs px-2 py-1 rounded-full font-medium">
                    {{ $t('npcsTab.tags.modsCount', { count: npc.modsCount }) }}
                </div>
            </div>
        </div>

        <div class="px-2 py-2 flex flex-col shrink-0">
            <div class="mb-2 min-w-0">
                <div class="font-semibold text-md truncate" :title="npc.name">{{ npc.name }}</div>
                <div class="text-secondary text-xs truncate">
                    {{ $t('npcsTab.id', { id: npc.id }) }}
                </div>
            </div>

            <div class="flex justify-between items-center text-xs text-primary font-mono">
                <span class="text-secondary">{{ $t('npcsTab.tags.enabledCount', { count: npc.enabledCount }) }}</span>
                <span class="text-secondary">{{ npc.modsCount }}</span>
            </div>
        </div>
    </div>
</template>
