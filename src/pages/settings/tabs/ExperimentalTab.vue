<script setup lang="ts">
import { TabPanel } from '@headlessui/vue';
import Section from '../Section.vue';
import Button from '../../../components/common/Button.vue';
import Select from '../../../components/common/Select.vue';
import { ref } from 'vue';
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

interface MigrateError {
    type: 'IoError' | 'ProfileImportError' | 'ProfileDeleteError' | 'ModsImportError' | 'ModsParseError' | 'ModsDeleteError' | 'ModsRefreshError'
    message: string
}

const profilesIdChoose = ref<string[]>([])
const legacyProfiles = ref<LegacyProfile[]>([])
const loggingStore = useLoggingStore()

function handleMigrateError(err: unknown) {
    const e = err as Partial<MigrateError>
    const key = `settingsTab.experimental.sections.migration.errors.${e.type ?? 'Unknown'}`
    const detail = t(key, { message: e.message ?? String(err) })
    toast.add({ severity: 'error', summary: t('common.error'), detail, life: 5000 })
}

async function searchLegacyProfiles() {
    try {
        const profiles = await invoke('get_legacy_profiles') as LegacyProfile[]
        legacyProfiles.value = profiles
        if (!profiles.length) {
            toast.add({ severity: 'info', summary: t('settingsTab.experimental.sections.migration.notifications.noProfilesToImport.title'), detail: t('settingsTab.experimental.sections.migration.notifications.noProfilesToImport.description'), life: 3000 })
        }
    } catch (err) {
        loggingStore.logError("Error fetching legacy profiles:", err)
        handleMigrateError(err)
    }
}

function importProfiles() {
    if (profilesIdChoose.value.length === 0) {
        toast.add({ severity: 'warn', summary: t('settingsTab.experimental.sections.migration.notifications.noProfilesSelected.title'), detail: t('settingsTab.experimental.sections.migration.notifications.noProfilesSelected.description'), life: 3000 })
        return
    }

    invoke<boolean>('import_legacy_profiles', { profileIds: profilesIdChoose.value })
        .then(async (success) => {
            if (!success) {
                toast.add({ severity: 'info', summary: t('settingsTab.experimental.sections.migration.notifications.noProfilesToImport.title'), detail: t('settingsTab.experimental.sections.migration.notifications.noProfilesToImport.description'), life: 3000 })
                return
            }

            toast.add({ severity: 'success', summary: t('settingsTab.experimental.sections.migration.notifications.importProfilesSuccess.title'), detail: t('settingsTab.experimental.sections.migration.notifications.importProfilesSuccess.description'), life: 5000 })
            
            profilesIdChoose.value = []
            profilesStore.loadProfiles()
            
            await searchLegacyProfiles()
        })
        .catch((err) => {
            loggingStore.logError("Error importing profiles:", err)
            handleMigrateError(err)
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

    invoke<boolean>('import_legacy_mod_authors')
        .then((success) => {
            if (!success) {
                return toast.add({ severity: 'info', summary: t('settingsTab.experimental.sections.migration.notifications.noModAuthorsToImport.title'), detail: t('settingsTab.experimental.sections.migration.notifications.noModAuthorsToImport.description'), life: 3000 })
            }
            toast.add({ severity: 'success', summary: t('settingsTab.experimental.sections.migration.notifications.importModAuthorsSuccess.title'), detail: t('settingsTab.experimental.sections.migration.notifications.importModAuthorsSuccess.description'), life: 5000 })
        })
        .catch((err) => {
            loggingStore.logError("Error importing mod authors:", err)
            handleMigrateError(err)
        })
}
</script>
<template>
    <TabPanel>
        <div class="flex flex-col">
            <Section :title="t('settingsTab.experimental.sections.migration.title')">
                <div class="flex flex-col gap-2">
                    <div class="flex justify-between gap-2">                        
                        <div class="flex flex-col min-w-0">
                            <p class="text-primary font-medium shrink-0">{{ t('settingsTab.experimental.sections.migration.profiles.title') }}</p>
                            <p class="text-secondary truncate" :title="t('settingsTab.experimental.sections.migration.profiles.description')">
                                {{ t('settingsTab.experimental.sections.migration.profiles.description') }}
                            </p>
                        </div>
                        <div class="flex gap-2 items-center">
                            <Select class="w-64 h-full" :options="legacyProfiles.map(p => ({ label: p.name, value: p.id }))" :placeholder="t('settingsTab.experimental.sections.migration.profiles.selectPlaceholder')" :multiple="true" v-model="profilesIdChoose" />
                            <Button class="h-full" variant="alt" @click="importProfiles">{{ t('settingsTab.experimental.sections.migration.actions.importProfiles') }}</Button>
                            <Button class="h-full" variant="alt" @click="searchLegacyProfiles">{{ t('settingsTab.experimental.sections.migration.actions.searchProfiles', 'Search Profiles') }}</Button>
                        </div>
                    </div>
                    <div class="flex justify-between gap-2">
                        <div class="flex flex-col min-w-0">
                            <p class="text-primary font-medium">{{ t('settingsTab.experimental.sections.migration.modAuthors.title') }}</p>
                            <p class="text-secondary truncate" :title="t('settingsTab.experimental.sections.migration.modAuthors.description')">
                                {{ t('settingsTab.experimental.sections.migration.modAuthors.description') }}
                            </p>
                        </div>
                        <div>
                            <Button class="h-full" variant="alt" @click="importModAuthors">{{ t('settingsTab.experimental.sections.migration.actions.importModAuthors') }}</Button>
                        </div>
                    </div>
                </div>
            </Section>
        </div>
    </TabPanel>
</template>