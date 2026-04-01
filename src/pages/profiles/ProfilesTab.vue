<script setup lang="ts">
import { computed, onMounted, ref, useTemplateRef } from 'vue'
import { useProfilesStore } from '../../stores/profiles'
import { Edit, PlusCircle, RefreshCcw, Trash2, TriangleAlert } from 'lucide-vue-next'
import { useHeader } from '../../composables/useHeader'
import { useI18n } from 'vue-i18n'
import EditProfile from './modals/EditProfile.vue'
import CreateProfile from './modals/CreateProfile.vue'
import Button from '../../components/common/Button.vue'
import { useToast } from 'primevue/usetoast'
import { useConfirm } from '../../plugins/ConfirmService'

const profilesStore = useProfilesStore()
const toast = useToast()
const confirm = useConfirm()
const { t } = useI18n()

const editProfileModal = useTemplateRef('editProfileModal')
const createProfileModal = useTemplateRef('createProfileModal')

const profileSelectedId = ref<string | null>('default')

const selectedProfile = computed(() => {
  if (!profileSelectedId.value) return null
  return profilesStore.getProfileById(profileSelectedId.value)
})

function editSelected() {
  if (!profileSelectedId.value || profileSelectedId.value === 'default') return

  const profile = profilesStore.getProfileById(profileSelectedId.value)

  editProfileModal.value?.show({
    id: profile?.id || '',
    name: profile?.name || '',
    description: profile?.description || null
  })
}

function createNewProfile() {
  createProfileModal.value?.show(profilesStore.profiles)
}

async function deleteSelected() {
  if (!profileSelectedId.value || profileSelectedId.value === 'default') return

  // const _profile = profilesStore.getProfileById(profileSelectedId.value)

  const result = await confirm.confirm({
    title: t('profilesTab.confirmations.deleteProfile.title'),
    message: t('profilesTab.confirmations.deleteProfile.message', { profileName: selectedProfile.value?.name }),
    icon: TriangleAlert,
    acceptButton: {
      label: t('profilesTab.confirmations.deleteProfile.actions.deleteProfile'),
      position: 'right'
    },
    rejectButton: {
      label: t('common.actions.cancel'),
      position: 'right'
    }
  })

  if (!result.confirmed) return

  await profilesStore.deleteProfile(profileSelectedId.value)

  profileSelectedId.value = null

  toast.add({
    severity: 'success',
    summary: t('profilesTab.notifications.profileDeleted.title'),
    detail: t('profilesTab.notifications.profileDeleted.description', { profileName: selectedProfile.value?.name }),
    life: 3000
  })
}
function onProfileEdit(id: string, name: string, description: string | null) {
  profilesStore.editProfile(id, name, description)

  toast.add({
    severity: 'success',
    summary: t('profilesTab.notifications.profileUpdated.title'),
    detail: t('profilesTab.notifications.profileUpdated.description', { profileName: name }),
    life: 3000
  })
}

function onProfileCreate(
  name: string,
  description: string | null,
  profileTemplateId: string | null
) {
  profilesStore.createProfile(name, description, profileTemplateId)

  toast.add({
    severity: 'success',
    summary: t('profilesTab.notifications.profileCreated.title'),
    detail: t('profilesTab.notifications.profileCreated.description', { profileName: name }),
    life: 3000
  })
}

function onProfileSwitch(id: string) {
  profilesStore.switchProfile(id)
}

function refreshProfiles() {
  profilesStore.loadProfiles()
}

useHeader({
  title: t('profilesTab.title'),
  subtitle: t('profilesTab.subtitle'),
  buttons: [
    {
      label: t("profilesTab.actions.refreshProfiles"),
      icon: RefreshCcw,
      action: refreshProfiles
    },
    {
      label: t('profilesTab.actions.createProfile'),
      icon: PlusCircle,
      action: createNewProfile
    }
  ]
})
</script>

<template>
  <div class="w-full h-full flex flex-col">
    <CreateProfile ref="createProfileModal" @on-profile-create="onProfileCreate" />
    <EditProfile ref="editProfileModal" @on-profile-edit="onProfileEdit" />

    <div class="text-primary flex flex-col p-4 py-2 w-full h-full">
      <div class="w-full flex-1 border rounded border-border bg-bg-surface flex overflow-hidden">
        <div class="flex-1 overflow-y-auto border-r border-border">
          <div class="flex flex-col">
            <div v-for="profile in profilesStore.sortedProfiles" :key="profile.id"
              class="p-3 cursor-pointer flex items-center justify-between gap-2 hover:bg-interactive-bg transition-colors duration-150 border-b border-border"
              :class="profileSelectedId && profileSelectedId == profile.id
                ? 'bg-bg-deep'
                : 'bg-bg-surface'
                " @click="profileSelectedId = profile.id">
              <div class="flex flex-col min-w-0 overflow-hidden">
                <span class="font-medium">{{ profile.name }}</span>
                <span class="text-sm text-secondary mt-1 truncate">
                  {{ profile.description === "d3f4ult" ? $t('profilesTab.defaultProfile') : profile.description ||
                    $t('profilesTab.emptyDescription') }}
                </span>
              </div>

              <div>
                <span class="font-medium text-success" v-if="profilesStore.activeProfileId == profile.id">
                  {{ $t('profilesTab.activeProfile') }}
                </span>
                <span v-else @click.stop="onProfileSwitch(profile.id)"
                  class="font-medium text-primary whitespace-nowrap hover:text-accent-primary! transition-colors duration-150 cursor-pointer">
                  {{ $t('profilesTab.actions.setAsCurrent') }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <div class="flex-1 overflow-y-auto p-4 box-border">
          <div v-if="selectedProfile" class="flex flex-col gap-4">
            <div class="flex justify-between items-center gap-2">
              <div class="overflow-hidden min-w-0">
                <h3 class="font-bold text-lg">
                  {{ selectedProfile.name }}
                </h3>

                <p class="text-secondary text-sm truncate">
                  {{ selectedProfile.description === "d3f4ult" ? $t('profilesTab.defaultProfile') :
                    selectedProfile.description || $t('profilesTab.emptyDescription') }}
                </p>
              </div>

              <div class="flex gap-2">
                <Button variant="text" :label="$t('profilesTab.actions.editProfile')" :icon="Edit"
                  :disabled="selectedProfile.id === 'default'" @click="editSelected" />

                <Button variant="text" :label="$t('profilesTab.actions.deleteProfile')" :icon="Trash2"
                  :disabled="selectedProfile.id === 'default'" @click="deleteSelected" />
              </div>
            </div>

            <div class="flex flex-col gap-1 h-full flex-1">
              <h4 class="font-semibold text-md text-primary">
                {{ $t('profilesTab.enabledMods', { count: selectedProfile.enabledMods?.length || 0 }) }}
              </h4>

              <div class="flex flex-col gap-1 bg-bg-deep rounded p-2 flex-1 h-full overflow-y-auto">
                <div v-for="mod in selectedProfile.enabledMods" :key="mod" class="text-sm text-secondary">
                  {{ mod }}
                </div>

                <div v-if="
                  !selectedProfile.enabledMods ||
                  selectedProfile.enabledMods.length === 0
                " class="text-sm text-secondary italic">
                  {{ $t('profilesTab.noModsEnabled') }}
                </div>
              </div>
            </div>
          </div>

          <div v-else class="text-secondary text-center mt-8">
            {{ $t('profilesTab.noProfileSelected') }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>