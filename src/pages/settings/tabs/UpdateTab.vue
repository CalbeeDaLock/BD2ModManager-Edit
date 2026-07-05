<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { TabPanel } from '@headlessui/vue';
import Button from '../../../components/common/Button.vue';
import Checkbox from '../../../components/common/Checkbox.vue';
import Section from '../Section.vue';
import { invoke } from '@tauri-apps/api/core';
import { useSettingsStore } from '../../../stores/settings';
import { getVersion } from '@tauri-apps/api/app';

const settingsStore = useSettingsStore();

const appVersion = ref('0.0.0')
const modPreviewVersion = ref<string | null>('0.0.0')

onMounted(async () => {
    appVersion.value = await getVersion()
    modPreviewVersion.value = await settingsStore.getModPreviewVersion()
})

const settings = computed(() => settingsStore.settings);

function onCheckForAppUpdates(value: boolean) {
  settingsStore.saveSettings({ checkForAppUpdates: value });
  if (value) {
    settingsStore.checkForAppUpdate();
  }
}

function onAutoUpdateModPreview(value: boolean) {
  settingsStore.saveSettings({ autoUpdateModPreview: value });
}

interface UpdateInfo {
  latestVersion: string;
  downloadUrl: string;
}

const isCheckingModPreviewUpdates = ref(false)
const modPreviewUpdatesAvailable = ref<UpdateInfo | null>(null)

function checkModPreviewUpdates() {
  console.log('Checking for mod preview updates...');
  isCheckingModPreviewUpdates.value = true;
  settingsStore.checkForModPreviewUpdate()
  .then((result) => {
    const updateAvailable = result as UpdateInfo | null;
    console.log('Mod preview update available:', updateAvailable);
    modPreviewUpdatesAvailable.value = updateAvailable;
  })
  .finally(() => {
    isCheckingModPreviewUpdates.value = false;
  })
}

async function updateModPreview() {
  modPreviewUpdatesAvailable.value = null;
  await invoke('update_mod_preview');
}
</script>

<template>
  <TabPanel>
    <div class="flex flex-col">
      <Section :title="$t('settingsTab.updates.sections.updates.title')">
        <div class="flex flex-col gap-3 mb-2">
          <Checkbox
            inputId="checkForAppUpdates"
            :model-value="settings.checkForAppUpdates"
            @update:model-value="onCheckForAppUpdates"
            :label="$t('settingsTab.updates.sections.updates.autoCheckForUpdates.label')"
            :description="$t('settingsTab.updates.sections.updates.autoCheckForUpdates.description')"
          />

          <Checkbox
            inputId="autoUpdateModPreview"
            :model-value="settings.autoUpdateModPreview"
            @update:model-value="onAutoUpdateModPreview"
            :label="$t('settingsTab.updates.sections.updates.autoUpdateModPreview.label')"
            :description="$t('settingsTab.updates.sections.updates.autoUpdateModPreview.description')"
          />
        </div>

        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-text-primary">BD2ModPreview</div>
            <div class="text-sm text-text-secondary">
              Version {{ modPreviewVersion || 'Unknown' }}
            </div>
          </div>
          <div>
            <Button
              v-if="modPreviewUpdatesAvailable && modPreviewUpdatesAvailable.downloadUrl"
              :label="$t('settingsTab.updates.sections.updates.actions.updateModPreview')"
              @click="updateModPreview" />
            <Button
              v-else
              :label="isCheckingModPreviewUpdates ? $t('settingsTab.updates.sections.updates.actions.checkingForUpdates') : $t('settingsTab.updates.sections.updates.actions.checkForUpdates')"
              @click="checkModPreviewUpdates"
              :disabled="isCheckingModPreviewUpdates" />
          </div>
        </div>
      </Section>
    </div>
  </TabPanel>
</template>