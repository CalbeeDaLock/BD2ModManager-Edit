<script lang="ts" setup>
import { Listbox, ListboxButton, ListboxOptions, ListboxOption } from '@headlessui/vue'
import { ChevronsUpDownIcon, Check } from 'lucide-vue-next';
import { useProfilesStore } from '../../stores/profiles';

const profilesStore = useProfilesStore()

function onProfileSelected(profile_id: string) {
    profilesStore.switchProfile(profile_id)
}

</script>

<template>
    <Listbox v-model:model-value="profilesStore.activeProfileId" @update:model-value="onProfileSelected">
        <div class="relative">
            <ListboxButton class="
                relative w-full py-3 pl-4 pr-10
                cursor-pointer 
                rounded-lg
                bg-interactive-bg 
                text-left text-primary 
                transition-colors duration-200">
                <span class="block truncate font-medium">{{ profilesStore.activeProfile?.name }}</span>
                <span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
                    <ChevronsUpDownIcon class="h-5 w-5 text-text-primary opacity-60" aria-hidden="true" />
                </span>
            </ListboxButton>

            <transition enter-active-class="transition duration-200 ease-out" enter-from-class="opacity-0 scale-95"
                enter-to-class="opacity-100 scale-100" leave-active-class="transition duration-150 ease-in"
                leave-from-class="opacity-100 scale-100" leave-to-class="opacity-0 scale-95">
                <ListboxOptions class="
                    absolute z-10 mt-2 w-full 
                    overflow-hidden 
                    rounded-lg border border-interactive-border
                    bg-interactive-bg
                    shadow-xl 
                    focus:outline-none">
                    <ListboxOption v-slot="{ selected }" v-for="profile in profilesStore.sortedProfiles"
                        :key="profile.id" :value="profile.id" as="template">
                        <li class="
                        hover:bg-interactive-bg-hover
                        relative cursor-pointer select-none py-3 pl-10 pr-4 text-primary transition-colors duration-150"
                            :class="[selected ? '' : '']">
                            <span :class="[
                                selected ? 'font-semibold text-primary' : 'font-medium',
                                'block truncate',
                            ]">{{ profile.name }}</span>
                            <span v-if="selected" class="absolute inset-y-0 left-0 flex items-center pl-3 text-primary">
                                <Check class="h-5 w-5" aria-hidden="true" />
                            </span>
                        </li>
                    </ListboxOption>
                </ListboxOptions>
            </transition>
        </div>
    </Listbox>
</template>