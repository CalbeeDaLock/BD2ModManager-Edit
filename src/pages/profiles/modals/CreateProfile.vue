<script setup lang="ts">
import { computed, ref } from 'vue'
import { Profile } from '../../../stores/profiles'
import { ChevronsUpDownIcon, X } from 'lucide-vue-next'
import Input from '../../../components/common/Input.vue'
import Button from '../../../components/common/Button.vue'
import Modal from '../../../components/common/Modal.vue'

interface CreateProfileOptions {
    name: string
    description: string | null
    templateId: string | null
}

const visible = ref(false)

const profileSelected = ref<CreateProfileOptions>({
    name: '',
    description: null,
    templateId: null
})

const templateProfiles = ref<Profile[]>([])

defineExpose({
    show: (profiles: Profile[]) => {
        templateProfiles.value = [...profiles]
        profileSelected.value = { name: '', description: null, templateId: null }
        visible.value = true
    }
})

const emit = defineEmits<{
    (e: 'on-profile-create', name: string, description: string | null, profileTemplateId: string | null): void
}>()

function create() {
    const { name, description, templateId } = profileSelected.value
    if (!name.trim()) return

    emit(
        'on-profile-create',
        name.trim(),
        description?.trim() || null,
        templateId
    )

    visible.value = false
}

const canCreateProfile = computed(() => {
    return profileSelected.value.name.trim().length > 0 && !templateProfiles.value.some(p => p.name === profileSelected.value.name.trim())
})

function onClose() {
    profileSelected.value = { name: '', description: null, templateId: null }
    visible.value = false
}
</script>

<template>
    <Modal v-model:show="visible" class="min-w-120 max-h-[80%] max-w-200"
        :title="$t('profiles.modals.createProfile.title')" :subtitle="$t('profiles.modals.createProfile.description')" @close="onClose">
        <div class="text-primary flex flex-col gap-4 p-4">

            <div class="flex flex-col gap-1">
                <label for="name" class="font-semibold">
                    {{ $t('profiles.modals.createProfile.placeholders.profileName') }}
                </label>
                <div class="h-10">
                    <Input id="name" class="flex-auto"
                        :placeholder="$t('profiles.modals.createProfile.placeholders.profileName')"
                        :model-value="profileSelected.name ?? ''"
                        @update:model-value="val => { if (profileSelected) profileSelected.name = val }" />
                </div>
            </div>

            <div class="flex flex-col gap-1">
                <label for="description" class="font-semibold">
                    {{ $t('profiles.modals.createProfile.placeholders.profileDescription') }}
                </label>
                <div class="h-10">
                    <Input id="description" class="flex-auto"
                        :placeholder="$t('profiles.modals.createProfile.placeholders.profileDescription')"
                        :model-value="profileSelected.description ?? ''" />
                </div>
            </div>

            <div class="flex flex-col gap-1">
                <label for="template" class="font-semibold">
                    {{ $t('profiles.modals.createProfile.placeholders.templateProfile') }}
                </label>

                <div class="relative flex">
                    <select id="template" v-model="profileSelected.templateId" class="w-full cursor-pointer rounded-sm border border-interactive-border bg-interactive-bg
                   text-left text-primary text-sm py-2 pl-3 pr-8 transition-colors
                   duration-200 focus:outline-none focus:ring-0 appearance-none">
                        <option :value="null" disabled>
                            {{ $t('profiles.modals.createProfile.placeholders.templateProfile') }}
                        </option>

                        <option v-for="profile in templateProfiles" :key="profile.id" :value="profile.id">
                            {{ profile.name }}
                        </option>
                    </select>

                    <span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                        <ChevronsUpDownIcon class="h-4 w-4 text-text-primary opacity-60" />
                    </span>

                    <button v-if="profileSelected.templateId" @click="profileSelected.templateId = null" type="button"
                        class="absolute cursor-pointer inset-y-0 right-6 flex items-center pr-2
                   text-primary hover:text-red-200">
                        <X class="h-4 w-4" />
                    </button>
                </div>
            </div>

        </div>

        <template #footer>
            <div class="flex justify-end gap-2">
                <Button type="button" :label="$t('profiles.modals.createProfile.actions.cancel')"
                    @click="onClose" />
                <Button type="button" :label="$t('profiles.modals.createProfile.actions.create')" @click="create" :disabled="!canCreateProfile" />
            </div>
        </template>
    </Modal>
</template>