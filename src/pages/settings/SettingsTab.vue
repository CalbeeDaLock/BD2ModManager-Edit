<script setup lang="ts">
import { Tab, TabGroup, TabList, TabPanels } from '@headlessui/vue';
import { CircleArrowUp, Monitor, Settings } from 'lucide-vue-next';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import GeneralTab from './tabs/GeneralTab.vue';
import InterfaceTab from './tabs/InterfaceTab.vue';
import UpdateTab from './tabs/UpdateTab.vue';
import { useHeader } from '../../composables/useHeader';


const { t } = useI18n()

const tabs = computed(() => [{
    label: t(`settingsTab.general.label`),
    value: 'general',
    icon: Settings,
}, {
    label: t('settingsTab.interface.label'),
    value: 'interface',
    icon: Monitor
}, {
    label: t('settingsTab.updates.label'),
    value: 'updates',
    icon: CircleArrowUp
}
])
</script>

<template>
    <div class="text-primary h-full overflow-y-auto flex flex-col w-full">
        <TabGroup vertical>
            <div class="relative flex flex-col p-2 border-r border-border">
                <TabList class="flex gap-1 transition-all">
                    <Tab v-for="tab in tabs" as="template" :key="tab.value" v-slot="{ selected }">
                        <button class="
                        px-4 py-2
                        text-center 
                        rounded 
                        transition-colors
                        text-md
                        cursor-pointer
                        ring-0 ring-transparent
                        hover:bg-interactive-bg-hover
                        flex gap-1.5 items-center"
                            :class="{ 'bg-interactive-bg text-primary font-medium': selected, 'text-secondary': !selected }">
                            <component v-if="tab.icon" :is="tab.icon" :class="['w-[1.25em] h-[1.25em] text-primary']" />
                            {{ tab.label }}
                        </button>
                    </Tab>
                </TabList>
            </div>
            <TabPanels class="flex-1 p-4 py-2 w-full">
                <GeneralTab />
                <InterfaceTab />
                <UpdateTab />
                <!-- <ExperimentalTab /> -->
            </TabPanels>
        </TabGroup>
    </div>
</template>

<style scoped></style>