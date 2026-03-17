<script lang="ts" setup>
import { Filter } from 'lucide-vue-next'
import Input from '../../components/common/Input.vue'
import Button from '../../components/common/Button.vue'
import Checkbox from '../../components/common/Checkbox.vue'
import Popover from '../../components/common/Popover.vue'

interface Filters {
    searchQuery: string
    // searchType: string
    modTypes: string[]
    onlyEnabled: boolean
    onlyDisabled: boolean
    onlyConflicts: boolean
    onlyErrors: boolean
}

const filters = defineModel<Filters>("filters", {
    default: {
        searchQuery: '',
        // searchType: 'mod',
        modTypes: [],
        onlyEnabled: false,
        onlyDisabled: false,
        onlyConflicts: false,
        onlyErrors: false,
    }
})

const modTypes = ["Cutscene", "Standing", "Scene", "Dating", "NPC", "Minigame"];

function toggleModTypes(modType: string) {
    let index = filters.value.modTypes.indexOf(modType)

    if (index == -1) {
        filters.value.modTypes.push(modType)
    } else {
        filters.value.modTypes.splice(index, 1)
    }
}

// [TODO]: Add filters to search; character:value, author:value, modname:value
</script>

<template>
    <div class="flex flex-col justify-between items-stretch gap-2">
        <div class="flex items-stretch">
            <!-- Search -->
            <div class="flex flex-1 gap-2 justify-start align-center h-9 items-stretch overflow-hidden">
                <Popover placement="bottom-start" trigger="hover">
                    <template #trigger="{isOpen, open, close}">
                        <Button :icon="Filter" :label="$t('modsTab.header.actions.filters')" @click="isOpen ? close() : open()" />
                    </template>

                    <template #default>
                        <div
                            class="flex flex-col min-w-[20rem] bg-bg-surface border-border border-2 text-primary p-4 py-0 pb-2 rounded-md shadow-lg">
                            <div class="font-semibold text-lg text-left border-interactive-border py-2">
                                {{ $t('modsTab.header.advancedFilters.title') }}
                            </div>

                            <div class="flex flex-col gap-1 text-md">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <Checkbox v-model="filters.onlyEnabled" />
                                    {{ $t('modsTab.header.advancedFilters.actions.onlyEnabledMods') }}
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <Checkbox v-model="filters.onlyDisabled" />
                                    {{ $t('modsTab.header.advancedFilters.actions.onlyDisabledMods') }}
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <Checkbox v-model="filters.onlyConflicts" />
                                    {{ $t('modsTab.header.advancedFilters.actions.onlyConflictsMods') }}
                                </label>
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <Checkbox v-model="filters.onlyErrors" />
                                    {{ $t('modsTab.header.advancedFilters.actions.onlyErrorsMods') }}
                                </label>
                            </div>
                        </div>
                    </template>
                </Popover>
                <div class="relative flex-1 flex gap-1.5">
                    <Input v-model="filters.searchQuery" :placeholder="$t('modsTab.header.searchPlaceholder')" />
                </div>

            </div>
        </div>

        <!-- TODO: one click add show filter, another click to add to remove filter and last click to disable filter -->
        <div class="flex justify-between items-center gap-2 ">
            <div class="flex gap-2">
                <button v-for="modType in modTypes" :key="modType" @click="toggleModTypes(modType)"
                    class="text-xs transition-all bg-bg-surface text-secondary border-interactive-border cursor-pointer capitalize items-center gap-1.5 flex border font-semibold rounded-sm py-1 px-2"
                    :class="{
                        'text-cutscene! bg-cutscene-bg! border-cutscene-bg! hover:bg-cutscene-bg!': modType == 'Cutscene' && filters.modTypes.includes(modType),
                        'text-standing! bg-standing-bg! border-standing-bg! hover:bg-standing-bg!': modType == 'Standing' && filters.modTypes.includes(modType),
                        'text-scene!    bg-scene-bg!    border-scene-bg!    hover:bg-scene-bg!': modType == 'Scene' && filters.modTypes.includes(modType),
                        'text-dating!   bg-dating-bg!   border-dating-bg!   hover:bg-dating-bg!   ': modType == 'Dating' && filters.modTypes.includes(modType),
                        'text-npc!      bg-npc-bg!      border-npc-bg!      hover:bg-npc-bg!      ': modType == 'NPC' && filters.modTypes.includes(modType),
                        'text-minigame! bg-minigame-bg! border-minigame-bg! hover:bg-minigame-bg! ': modType == 'Minigame' && filters.modTypes.includes(modType),
                        'hover:bg-interactive-bg-hover!': !filters.modTypes.includes(modType)
                    }">
                    {{ $t(`common.modTypes.${modType.toLowerCase()}`) }}
                </button>
            </div>
        </div>
    </div>
</template>