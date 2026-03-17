<script setup lang="ts">
import { ComponentPublicInstance, computed, ref } from 'vue';
import { useVirtualizer } from '@tanstack/vue-virtual';
import { Character } from '../../stores/characters';
import CharacterListRow from './CharacterListRow.vue';

export interface CharacterGroup {
    name: string;
    costumes: CostumeWithMods[];
}

export interface CostumeWithMods extends Character {
    hasCutscene?: boolean;
    hasStanding?: boolean;
    hasDating?: boolean;
    modsCount: number;
}

export type CharacterListItem =
    | { type: 'header'; data: string; characterName: string }
    | { type: 'costume'; data: CostumeWithMods; characterName: string };

const props = defineProps<{
    items: CharacterGroup[];
}>();

const emit = defineEmits<{
    openModDetails: [costume: Character];
}>();

const allCharacters = computed<CharacterListItem[]>(() => {
    return props.items.reduce((acc, group) => {
        acc.push({
            type: 'header' as const,
            data: group.name,
            characterName: group.name
        });

        group.costumes.forEach(costume => {
            acc.push({
                type: 'costume' as const,
                data: costume,
                characterName: group.name
            });
        });

        return acc;
    }, [] as CharacterListItem[]);
});

const parentRef = ref<HTMLElement | null>(null);

const rowVirtualizer = useVirtualizer({
    get count() {
        return allCharacters.value.length;
    },
    getScrollElement: () => parentRef.value,
    estimateSize: (index) => {
        const items = allCharacters.value;
        if (index >= items.length) return 168;
        const item = items[index];
        return item?.type === 'header' ? 28 : 168;
    },
    overscan: 20,
    measureElement: (element, _entry, instance) => {
        const indexKey = Number(element.getAttribute('data-index'));
        const cachedMeasurement = instance.measurementsCache[indexKey]?.size;

        if (cachedMeasurement !== undefined) {
            return cachedMeasurement;
        }

        return element.getBoundingClientRect().height;
    },
})

const virtualRows = computed(() => rowVirtualizer.value.getVirtualItems());
const totalSize = computed(() => rowVirtualizer.value.getTotalSize());

const measureElement = (el: Element | ComponentPublicInstance | null) => {
    if (!el || !(el instanceof Element)) return;
    rowVirtualizer.value.measureElement(el);
};
</script>

<template>
    <div v-if="items.length === 0" class="text-center py-12 text-secondary">
        <p class="text-lg">
            {{ $t('charactersTab.charactersNotFound') }}
        </p>
    </div>

    <div v-else ref="parentRef" class="flex flex-col text-primary bg-bg-deep overflow-y-auto"
        style="height: 100%; contain: strict">
        <div :style="{
            height: `${totalSize}px`,
            width: '100%',
            position: 'relative',
        }">
            <div :style="{
                position: 'absolute',
                top: 0,
                left: 0,
                width: '100%',
                transform: `translateY(${virtualRows[0]?.start ?? 0}px)`,
            }">
                <CharacterListRow
                    v-for="virtualRow in virtualRows"
                    :key="virtualRow.key.toString()"
                    :data-index="virtualRow.index"
                    :ref="measureElement"
                    :item="allCharacters[virtualRow.index]"
                    @open-mod-details="emit('openModDetails', $event)"
                />
            </div>
        </div>
    </div>
</template>