<script setup lang="ts">
import { TriangleAlert, Upload, Github, Trash2 } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'
import Button from '../../components/common/Button.vue'
import { computed } from 'vue';
import { PluginState, Status } from './types';

const props = defineProps<{
    label: string
    description: string
    state: PluginState,
    disabled?: boolean
    showGithub?: boolean
    requiresBepInEx?: boolean
    bepInExInstalled?: boolean
}>()

const emit = defineEmits<{
    install: []
    installFromGithub: []
    remove: []
}>()

const { t } = useI18n()

function getStatusMessage(status: Status) {
    switch (status) {
        case Status.Installed: return t("browndustxTab.status.installed")
        case Status.InstalledButOutdated: return t("browndustxTab.status.installedButOutdated")
        case Status.NotInstalled: return t("browndustxTab.status.notInstalled")
        case Status.BepInExMissing: return t("browndustxTab.status.bepinexMissing")
        default: return status
    }
}

function isInstalled(status: Status) {
    return status === Status.Installed || status === Status.InstalledButOutdated
}

const canInstall = computed(() =>
    props.state.status === Status.NotInstalled &&
    (!props.requiresBepInEx || props.bepInExInstalled)
)
</script>

<template>
    <div class="grid grid-cols-[1fr_auto] lg:grid-cols-[2fr_100px_120px_280px] gap-2 lg:gap-4 items-center py-2.5 border-t border-border">
        <div class="min-w-0">
            <div class="flex items-center gap-1.5">
                <TriangleAlert v-if="state.status === Status.BepInExMissing" class="w-4 h-4 text-warning" />
                <span class="text-sm text-primary">{{ label }}</span>
                <span v-if="state.version" class="inline lg:hidden text-sm text-secondary">
                    v{{ state.version }}
                </span>
            </div>
            <p class="text-xs text-secondary truncate" :title="description">{{ description }}</p>
        </div>

        <span class="hidden lg:inline text-sm" :class="state.version ? 'text-primary' : 'text-secondary'">
            {{ state.version ?? '-' }}
        </span>

        <div class="hidden lg:flex items-center gap-1">
            <span class="text-xs" :class="{
                'text-success': isInstalled(state.status),
                'text-warning': state.status === Status.BepInExMissing || state.status === Status.InstalledButOutdated,
                'text-secondary': state.status === Status.NotInstalled,
            }">{{ getStatusMessage(state.status) }}</span>
        </div>

        <div class="flex gap-1 justify-end items-center">
            <div class="flex gap-1 justify-end items-center">
                <template v-if="canInstall">
                    <Button v-if="showGithub" variant="text" :label="$t('browndustxTab.actions.installFromGithub')"
                        :icon="Github" label-class="hidden lg:inline" :disabled="disabled"
                        @click="emit('installFromGithub')" />
                    <Button variant="text" :label="$t('browndustxTab.actions.selectFile')" :icon="Upload"
                        label-class="hidden lg:inline" :disabled="disabled" @click="emit('install')" />
                </template>

                <template v-else-if="isInstalled(state.status)">
                    <Button v-if="state.canRemove?.type === 'Yes'" variant="text"
                        :label="$t('browndustxTab.actions.remove')" :icon="Trash2" label-class="hidden lg:inline"
                        :disabled="disabled" @click="emit('remove')" />
                    <p v-else-if="state.canRemove?.type === 'No'"
                        class="hidden lg:inline text-xs text-secondary italic">
                        {{ $t(`browndustxTab.canRemoveReasons.${state.canRemove.reason}`, { name: label }) }}
                    </p>
                </template>
            </div>
        </div>
    </div>
</template>