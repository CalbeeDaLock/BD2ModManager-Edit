<script lang="ts" setup>
import { TabPanel } from '@headlessui/vue';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import Section from '../Section.vue';
import Select from '../../../components/common/Select.vue';
import Checkbox from '../../../components/common/Checkbox.vue';
import { usePreferencesStore } from '../../../stores/preferences';

const preferencesStore = usePreferencesStore()

const { t } = useI18n()

const columnsOptions = computed(() => [
    { label: t('settings.interface.columns.visibleColumns.columns.modName'), value: 'name' },
    { label: t('settings.interface.columns.visibleColumns.columns.character'), value: 'character' },
    { label: t('settings.interface.columns.visibleColumns.columns.modType'), value: 'modType' },
    { label: t('settings.interface.columns.visibleColumns.columns.author'), value: 'author' },
])

const modNameDisplayOptions = computed(() => [
    { label: t('settings.interface.modName.options.short'), value: 'short' },
    { label: t('settings.interface.modName.options.full'), value: 'full' },
])

const characterDisplayOptions = computed(() => [
    { label: t('settings.interface.display.character.full'), value: 'full' },
    { label: t('settings.interface.display.character.iconOnly'), value: 'iconOnly' },
    { label: t('settings.interface.display.character.nameOnly'), value: 'nameOnly' },
])

const modTypeDisplayOptions = computed(() => [
    { label: t('settings.interface.display.modType.full'), value: 'full' },
    { label: t('settings.interface.display.modType.iconOnly'), value: 'iconOnly' },
    { label: t('settings.interface.display.modType.labelOnly'), value: 'labelOnly' },
])
</script>

<template>
    <TabPanel>
        <div class="flex flex-col gap-4 text-primary">
            <Section :title="$t('settings.interface.columns.title')">
                <div class="flex flex-col gap-1">
                    <label class="text-sm font-medium">
                        {{ $t('settings.interface.columns.visibleColumns.label') }}
                    </label>
                    <Select
                        v-model="preferencesStore.visibleModListColumns"
                        :options="columnsOptions"
                        option-label="label"
                        option-value="value"
                        class="w-full"
                        multiple
                    />
                    <p class="text-xs text-secondary">
                        {{ $t('settings.interface.columns.visibleColumns.description') }}
                    </p>
                </div>
            </Section>

            <Section :title="$t('settings.interface.columns.modNameColumn.label')">
                <div class="flex flex-col gap-4">
                    <div class="grid grid-cols-3">
                        <div class="flex flex-col">
                            <span class="text-sm font-medium">{{ $t('settings.interface.modName.label') }}</span>
                            <p class="text-xs text-secondary">{{ $t('settings.interface.modName.description') }}</p>
                        </div>
                        <Select :model-value="preferencesStore.modNameDisplay" @update:model-value="preferencesStore.modNameDisplay = $event" :options="modNameDisplayOptions" class="col-span-2" />
                    </div>

                    <!-- <div class="flex items-center gap-3">
                        <Checkbox inputId="hide-conflicts" binary v-model="preferencesStore.hideConflicts" class="mt-0.5" />
                        <label for="hide-conflicts" class="flex flex-col gap-0.5 cursor-pointer">
                            <span class="text-sm font-medium">{{ $t('settings.interface.modName.hideConflicts.label') }}</span>
                            <p class="text-xs text-secondary">{{ $t('settings.interface.modName.hideConflicts.description') }}</p>
                        </label>
                    </div> -->
                </div>
            </Section>

            <Section :title="$t('settings.interface.columns.characterColumn.label')">
                <div class="grid grid-cols-3">
                    <div class="flex flex-col">
                        <span class="text-sm font-medium">{{ $t('settings.interface.character.label') }}</span>
                        <p class="text-xs text-secondary">{{ $t('settings.interface.display.character.description') }}</p>
                    </div>
                    <Select :model-value="preferencesStore.characterDisplay" @update:model-value="preferencesStore.characterDisplay = $event" :options="characterDisplayOptions" class="col-span-2" />
                </div>
            </Section>

            <Section :title="$t('settings.interface.columns.modTypeColumn.label')">
                <div class="flex flex-col gap-4">
                    <div class="grid grid-cols-3">
                        <div class="flex flex-col">
                            <span class="text-sm font-medium">{{ $t('settings.interface.display.modType.label') }}</span>
                            <p class="text-xs text-secondary">{{ $t('settings.interface.display.modType.description') }}</p>
                        </div>
                        <Select :model-value="preferencesStore.modTypeDisplay" @update:model-value="preferencesStore.modTypeDisplay = $event" :options="modTypeDisplayOptions" class="col-span-2" />
                    </div>

                    <div class="flex items-start gap-3">
                        <Checkbox inputId="enable-colors" binary v-model="preferencesStore.enableModTypeColors" class="mt-0.5" :label="$t('settings.interface.display.modType.enableColors.label')" :description="$t('settings.interface.display.modType.enableColors.description')" />
                    </div>
                </div>
            </Section>

        </div>
    </TabPanel>
</template>

<style scoped>
</style>