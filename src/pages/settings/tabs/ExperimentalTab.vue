<script setup lang="ts">
import { TabPanel } from '@headlessui/vue';
import Section from '../Section.vue';
import Button from '../../../components/common/Button.vue';
import Select from '../../../components/common/Select.vue';
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from 'primevue/usetoast';
import { useProfilesStore } from '../../../stores/profiles';
import { useLoggingStore } from '../../../stores/logging';
import { useConfirm } from '../../../plugins/ConfirmService';
import { useI18n } from 'vue-i18n';

const profilesStore = useProfilesStore()

const toast = useToast()
const confirmation = useConfirm()
const { t } = useI18n()

interface LegacyProfile {
    id: string,
    name: string
}

const profilesIdChoose = ref<string[]>([])
const legacyProfiles = ref<LegacyProfile[]>([])
const loggingStore = useLoggingStore()

async function updateLegacyProfiles() {
    try {
        const profiles = await invoke('get_legacy_profiles') as LegacyProfile[]
        legacyProfiles.value = profiles
    } catch (err) {
        loggingStore.logError("Error fetching legacy profiles:", err)
        toast.add({ severity: 'error', summary: t('settingsTab.experimental.sections.migration.notifications.errorFetchingProfiles.title'), detail: t('settingsTab.experimental.sections.migration.notifications.errorFetchingProfiles.description'), life: 5000 })
    }
}

function importProfiles() {
    if (profilesIdChoose.value.length === 0) {
        toast.add({ severity: 'warn', summary: t('settingsTab.experimental.sections.migration.notifications.noProfilesToImport.title'), detail: t('settingsTab.experimental.sections.migration.notifications.noProfilesToImport.description'), life: 3000 })
        return
    }

    invoke('import_legacy_profiles', { profileIds: profilesIdChoose.value })
        .then(async () => {
            toast.add({ severity: 'success', summary: t('settingsTab.experimental.sections.migration.notifications.importProfilesSuccess.title'), detail: t('settingsTab.experimental.sections.migration.notifications.importProfilesSuccess.description'), life: 5000 })
            profilesIdChoose.value = []
            profilesStore.loadProfiles()
            await updateLegacyProfiles()
        })
        .catch((err) => {
            loggingStore.logError("Error importing profiles:", err)
            toast.add({ severity: 'error', summary: t('settingsTab.experimental.sections.migration.notifications.importFailed.title'), detail: t('settingsTab.experimental.sections.migration.notifications.importFailed.description', { error: err }), life: 5000 })
        })
}

async function importModAuthors() {
    const confirmationResult = await confirmation.confirm({
        title: t('settingsTab.experimental.sections.migration.confirmations.importModAuthors.title'),
        message: t('settingsTab.experimental.sections.migration.confirmations.importModAuthors.message'),
        acceptButton: {
            label: t('settingsTab.experimental.sections.migration.confirmations.importModAuthors.actions.importModAuthors'),
        },
        rejectButton: {
            label: t('settingsTab.experimental.sections.migration.confirmations.importModAuthors.actions.cancel'),
        },
    })
    if (!confirmationResult.confirmed) return

    try {
        await invoke('import_legacy_mod_authors')
        toast.add({ severity: 'success', summary: t('settingsTab.experimental.sections.migration.notifications.importModAuthorsSuccess.title'), detail: t('settingsTab.experimental.sections.migration.notifications.importModAuthorsSuccess.description'), life: 5000 })
    } catch (err) {
        loggingStore.logError("Error importing mod authors:", err)
        toast.add({ severity: 'error', summary: t('settingsTab.experimental.sections.migration.notifications.importFailed.title'), detail: t('settingsTab.experimental.sections.migration.notifications.importFailed.description', { error: err }), life: 5000 })
    }
}

onMounted(async () => {
    await updateLegacyProfiles()
})
</script>
<template>
    <TabPanel>
        <div class="flex flex-col">
            <Section :title="t('settingsTab.experimental.sections.migration.title')" :description="t('settingsTab.experimental.sections.migration.description')">
                <div class="flex flex-col gap-2">
                    <div class="flex justify-between gap-2">
                        <div class="flex flex-col">
                            <p class="text-primary font-medium">{{ t('settingsTab.experimental.sections.migration.profiles.title') }}</p>
                            <p class="text-secondary">{{ t('settingsTab.experimental.sections.migration.profiles.description') }}</p>
                        </div>
                        <div class="flex gap-2">
                            <Select :options="legacyProfiles.map(p => ({ label: p.name, value: p.id }))" 
                                :placeholder="t('settingsTab.experimental.sections.migration.profiles.selectPlaceholder')" class="w-64" :multiple="true" v-model="profilesIdChoose" />
                            <Button variant="alt" @click="importProfiles">{{ t('settingsTab.experimental.sections.migration.actions.importProfiles') }}</Button>
                        </div>
                    </div>
                    <div class="flex justify-between gap-2">
                        <div class="flex flex-col">
                            <p class="text-primary font-medium">{{ t('settingsTab.experimental.sections.migration.modAuthors.title') }}</p>
                            <p class="text-secondary">{{ t('settingsTab.experimental.sections.migration.modAuthors.description') }}</p>
                        </div>
                        <div>
                            <Button variant="alt" @click="importModAuthors">{{ t('settingsTab.experimental.sections.migration.actions.importModAuthors') }}</Button>
                        </div>
                    </div>
                </div>
            </Section>
        </div>
    </TabPanel>
</template>