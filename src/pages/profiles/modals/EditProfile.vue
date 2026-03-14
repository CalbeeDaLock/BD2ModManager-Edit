<script setup lang="ts">
import { ref } from 'vue'
import Input from '../../../components/common/Input.vue'
import Button from '../../../components/common/Button.vue'
import Modal from '../../../components/common/Modal.vue'

interface EditProfileOptions {
  id: string
  name: string
  description: string | null
}

const visible = ref(false)
const profileSelected = ref<EditProfileOptions | null>(null)

defineExpose({
  show: (opts: EditProfileOptions) => {
    profileSelected.value = { ...opts }
    visible.value = true
  }
})

const emit = defineEmits<{
  (e: 'on-profile-edit', id: string, name: string, description: string | null): void
}>()

function save() {
  if (!profileSelected.value) return

  const { id, name, description } = profileSelected.value
  if (!name.trim()) return

  emit(
    'on-profile-edit',
    id,
    name.trim(),
    description?.trim() || null
  )

  visible.value = false
}
</script>

<template>
  <Modal
    v-model:show="visible"
    class="min-w-120 max-w-200 max-h-[80%]"
    :title="$t('profiles.modals.editProfile.title')"
    :subtitle="$t('profiles.modals.editProfile.description')"
    @close="visible = false"
  >
    <div
      v-if="profileSelected"
      class="text-primary flex flex-col gap-4 p-4"
    >

      <div class="flex flex-col gap-1">
        <label for="edit-name" class="font-semibold">
          {{ $t('profiles.modals.editProfile.placeholders.profileName') }}
        </label>
        <div class="h-10">
          <Input
            id="edit-name"
            class="flex-auto"
            :placeholder="$t('profiles.modals.editProfile.placeholders.profileName')"
            :model-value="profileSelected.name ?? ''"
            @update:model-value="val => { if (profileSelected) profileSelected.name = val }"
          />
        </div>
      </div>

      <div class="flex flex-col gap-1">
        <label for="edit-description" class="font-semibold">
          {{ $t('profiles.modals.editProfile.placeholders.profileDescription') }}
        </label>
        <div class="h-10">
          <Input
            id="edit-description"
            class="flex-auto"
            autocomplete="off"
            :placeholder="$t('profiles.modals.editProfile.placeholders.profileDescription')"
            :model-value="profileSelected.description ?? ''"
            @update:model-value="val => { if (profileSelected) profileSelected.description = val }"
          />
        </div>
      </div>

    </div>

    <template #footer>
      <div class="flex justify-end gap-2">
        <Button
          type="button"
          :label="$t('profiles.modals.editProfile.actions.cancel')"
          @click="visible = false"
        />
        <Button
          type="button"
          :label="$t('profiles.modals.editProfile.actions.save')"
          @click="save"
        />
      </div>
    </template>
  </Modal>
</template>