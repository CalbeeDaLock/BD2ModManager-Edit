<script setup lang="ts">
import { ref, watch } from 'vue';
import { ChevronDown, Download, ExternalLink } from 'lucide-vue-next';
import Modal from '../../../components/common/Modal.vue';
import Button from '../../../components/common/Button.vue';
import { Disclosure, DisclosureButton, DisclosurePanel } from '@headlessui/vue';
import { openUrl } from '@tauri-apps/plugin-opener';

const BEPINEX_RELEASES_URL = "https://api.github.com/repos/BepInEx/BepInEx/releases";

const bepinexVersions = ref([] as any[]);
const loading = ref(false);

async function fetchBepInExVersions() {
    loading.value = true;
    try {
        const response = await fetch(BEPINEX_RELEASES_URL);
        const data = await response.json();

        // const currentPlatform = platform()
        bepinexVersions.value = data
    } catch (error) {
        console.error('Error fetching BepInEx versions:', error);
    } finally {
        loading.value = false;
    }
}

const showBepInEx = defineModel('show', {
    type: Boolean,
    required: true
});

const emit = defineEmits(['close', 'version-selected']);

watch(
    () => showBepInEx.value,
    async (newVal) => {
        if (newVal && bepinexVersions.value.length === 0) {
            await fetchBepInExVersions();
        }
    }
);

function formatDate(dateString: string) {
    return new Date(dateString).toLocaleDateString();
}

function handleVersionSelected(downloadUrl: string) {
    showBepInEx.value = false;
    emit('version-selected', downloadUrl);
}

</script>

<template>
    <Modal v-model:show="showBepInEx" @close="showBepInEx = false" class="w-150 max-h-[80%]"
        :title="$t('browndustxTab.modals.bepinexGithub.title')">
        <template #footer>
            <div class="flex justify-center gap-2 p-2">
                <Button label="Cancel" @click="$emit('close')" />
            </div>
        </template>

        <div class="p-4">
            <div v-if="loading" class="text-center py-8 text-secondary">
                {{ $t('browndustxTab.modals.bepinexGithub.loading') }}
            </div>

            <div v-else-if="bepinexVersions.length > 0" class="space-y-2">
                <Disclosure v-for="version in bepinexVersions.slice(0, 20)" :key="version.id" v-slot="{ open }">
                    <DisclosureButton
                        class="w-full flex items-center gap-3 p-3 border border-border rounded hover:bg-bg-deep">
                        <ChevronDown :class="[
                            'w-4 h-4 text-secondary transition-transform duration-200',
                            open ? 'rotate-180' : ''
                        ]" />
                        <div class="flex flex-col flex-1 items-start justify-center">
                            <div class="flex items-center gap-2">
                                <span class="text-primary font-semibold">BepInEx {{ version.tag_name }}</span>
                            </div>
                            <div class="text-sm text-secondary">
                                {{ formatDate(version.published_at) }}
                            </div>
                        </div>

                        <div class="flex gap-2 items-center">
                            <span :href="version.html_url" target="_blank"
                                class="px-3 py-1 cursor-pointer items-center justify-center flex gap-1 border border-border rounded text-sm text-secondary hover:bg-bg-deep"
                                @click.stop="async () => {
                                    await openUrl(version.html_url);
                                }">
                                <ExternalLink class="w-4 h-4 inline-block mr-1" />
                                {{ $t('browndustxTab.modals.bepinexGithub.actions.viewOnGithub') }}
                        </span>
                        </div>
                    </DisclosureButton>
                    <DisclosurePanel class="rounded-lg border border-border bg-bg-deep p-3 space-y-2">
                        <div v-for="asset in version.assets" :key="asset.id"
                            class="flex items-center justify-between rounded-md px-3 py-2 hover:bg-bg-surface transition-colors">
                            <div class="flex flex-col">
                                <span class="text-primary font-medium">
                                    {{ asset.name }}
                                </span>
                                <span class="text-xs text-secondary">
                                    {{ (asset.size / 1024 / 1024).toFixed(2) }} MB
                                </span>
                            </div>

                            <Button size="sm" class="px-3" @click="handleVersionSelected(asset.browser_download_url)">
                                <Download class="w-4 h-4 inline-block mr-1" />
                                {{ $t('browndustxTab.modals.bepinexGithub.actions.install') }}
                            </Button>
                        </div>
                    </DisclosurePanel>
                </Disclosure>
                <!-- </div> -->
            </div>

            <div v-else class="text-center py-8">
                <p class="text-secondary mb-2">{{ $t('browndustxTab.modals.bepinexGithub.failedToLoadVersions') }}</p>
                <button @click="fetchBepInExVersions"
                    class="px-3 py-1 bg-interactive-bg border border-interactive-border rounded text-primary">
                    {{ $t('browndustxTab.modals.bepinexGithub.actions.retry') }}
                </button>
            </div>
        </div>
    </Modal>
</template>