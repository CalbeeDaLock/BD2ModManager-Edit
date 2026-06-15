<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import { openUrl } from '@tauri-apps/plugin-opener';
import { storeToRefs } from 'pinia';
import Modal from '../common/Modal.vue';
import Button from '../common/Button.vue';
import Checkbox from '../common/Checkbox.vue';
import { useSettingsStore } from '../../stores/settings';
import { useLocalStorage } from '@vueuse/core';

// this will only show on portable

const visible = defineModel('visible', {
    type: Boolean,
    required: true,
    default: false
})

const settingsStore = useSettingsStore()
const { appUpdateStatus } = storeToRefs(settingsStore)

const skipUpdateVersion = useLocalStorage('skipUpdateVersion', '')
const checkboxSkipVersion = ref(false)

const appVersion = ref('0.0.0')

onMounted(async () => {
    appVersion.value = await getVersion()
})

const emit = defineEmits<{
  (e: 'close'): void
}>()
</script>

<template>
    <Modal v-model:show="visible" class="w-120 max-h-[85vh]" @close="$emit('close')"
        :title="$t('modals.updateAvailable.title')"
        :subtitle="$t('modals.updateAvailable.subtitle', { version: appUpdateStatus?.version })">
        <div class="flex flex-col gap-1 p-4">
            <p class="text-normal font-medium flex items-center gap-2 text-text-secondary mb-2">
                {{ $t("modals.updateAvailable.changelogLabel") }}
            </p>
            <div class="flex flex-col gap-1">
                <div v-for="(item, index) in appUpdateStatus?.changelog" :key="index"
                    class="flex items-start gap-3 px-1.5 py-1.5 text-sm">
                    <span class="text-text-secondary text-xs mt-0.5 shrink-0">{{ String(index + 1).padStart(2, '0')
                        }}</span>
                    <span class="text-text-primary">{{ item }}</span>
                </div>
            </div>
        </div>


        <template #footer>
            <div class="flex items-center gap-2 p-3">
                <div class="flex-1">
                    <Checkbox v-model="checkboxSkipVersion" @update:model-value="(value) => {
                        if (value) {
                            skipUpdateVersion = appUpdateStatus?.version ?? ''
                        } else {
                            skipUpdateVersion = ''
                        }
                    }"
                        :label="$t('modals.updateAvailable.skipLabel')" />
                </div>
                <Button variant="default" @click="$emit('close')">
                    {{ $t('modals.updateAvailable.actions.later') }}
                </Button>
                <Button variant="primary" @click="openUrl(appUpdateStatus?.downloadUrl??'')">
                    {{ $t('modals.updateAvailable.actions.goToReleases') }}
                </Button>
            </div>
        </template>
    </Modal>
</template>