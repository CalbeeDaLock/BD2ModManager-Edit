<script setup lang="ts">
import Button from '../common/Button.vue'
import Modal from '../common/Modal.vue'
import { ref, computed, watch, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { SyncStatus, useSyncStateStore } from '../../stores/syncState'
import { SyncProgressStatus, SyncType } from '../../composables/useSyncEvents'
import { refThrottled,  useVirtualList } from '@vueuse/core'
import {
  ArrowLeftFromLine,
  ArrowRightFromLine,
  ArrowRightLeft,
  RefreshCcw,
  TriangleAlert,
  X
} from 'lucide-vue-next'
import { useModsStore } from '../../stores/mods'

const { t } = useI18n()
const syncStateStore = useSyncStateStore()

const emit = defineEmits([
  'close',
  'cancel',
])

const props = defineProps({
  visible: Boolean
})

function handleClose() {
  emit('close')
}

// function _handleCancel() {
//   // [TODO]
// }

const getModStatusIcon = (status: SyncProgressStatus) => {
  switch (status) {
    case SyncProgressStatus.Synced: return ArrowRightFromLine
    case SyncProgressStatus.UpToDate: return ArrowRightLeft
    case SyncProgressStatus.Removed: return ArrowLeftFromLine
    case SyncProgressStatus.Failed: return TriangleAlert
  }
}

const getModStatusColor = (status: SyncProgressStatus) => {
  switch (status) {
    case SyncProgressStatus.Synced: return 'text-success'
    case SyncProgressStatus.UpToDate: return 'text-info'
    case SyncProgressStatus.Removed: return 'text-warning'
    case SyncProgressStatus.Failed: return 'text-danger'
    default: return 'text-secondary'
  }
}

// const getModStatusLabel = (status: SyncProgressStatus) => {
//   switch (status) {
//     case SyncProgressStatus.Synced: return t('modals.sync.synced', 'Synced')
//     case SyncProgressStatus.UpToDate: return t('modals.sync.upToDate', 'Up to Date')
//     case SyncProgressStatus.Removed: return t('modals.sync.removed', 'Removed')
//     case SyncProgressStatus.Failed: return t('modals.sync.failed', 'Failed')
//     default: return ''
//   }
// }

const formatTimestamp = (timestamp: string) => {
  return new Date(timestamp).toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

const title = computed(() => {
  if (syncStateStore.type === SyncType.Sync) {
    switch (syncStateStore.status) {
      case SyncStatus.SYNCING:
        return t('modals.sync.syncing', 'Syncing mods...')
      case SyncStatus.COMPLETED:
        return t('modals.sync.completed', 'Mods synced successfully')
      case SyncStatus.FAILED:
        return t('modals.sync.failed', 'Failed to sync mods')
      case SyncStatus.IDLE:
        return t('modals.sync.idle', 'Ready to sync mods')
    }
  } else {
    switch (syncStateStore.status) {
      case SyncStatus.SYNCING:
        return t('modals.sync.removing', 'Removing mods...')
      case SyncStatus.COMPLETED:
        return t('modals.sync.removed', 'Mods removed successfully')
      case SyncStatus.FAILED:
        return t('modals.sync.failedToRemove', 'Failed to remove mods')
      case SyncStatus.IDLE:
        return t('modals.sync.idleToRemove', 'Ready to remove mods')
    }
  }
})

const errorMessage = computed(() => {
  if (syncStateStore?.status === SyncStatus.FAILED) {
    return syncStateStore.error?.type === "SymlinkAdminRequired"
      ? "Administrator needed for mod sync"
      : syncStateStore.error?.type
  }
})

const realProgress = computed(() => {
  if (!syncStateStore?.progress.total) return 100
  return Math.round(
    (syncStateStore.progress.current / syncStateStore.progress.total) * 100
  )
})

const progress = refThrottled(realProgress, 250)

const syncedMods = computed(() => syncStateStore.syncedMods || [])

const {
  list,
  containerProps,
  wrapperProps,
  scrollTo
} = useVirtualList(
  syncedMods,
  {
    itemHeight: 36,
    overscan: 10,
  }
)

const lastSyncedMod = refThrottled(ref(syncStateStore.lastSyncedMod), 250)

watch(
  () => syncStateStore.lastSyncedMod,
  (newVal) => {
    lastSyncedMod.value = newVal
  }
)

watch(
  () => syncedMods.value.length,
  (length) => {
    if (length > 0)
      scrollTo(length - 1)
  }
)

const modsStore = useModsStore()

function sM() {
  modsStore.syncMods()
}

const isDev = ref(false)
onMounted(() => {
  isDev.value = import.meta.env.DEV
})
</script>

<template>
  <Modal
    :show="visible"
    @close="handleClose"
    class="w-full min-w-xl max-w-2xl min-h-[80vh]"
  >
    <template #header>
      <div class="flex items-center justify-between gap-3 min-w-0 p-4">
        <div class="min-w-0 flex-1" data-tauri-drag-region>
          <span class="text-sm font-semibold font-mono text-primary">
            {{ title }}
          </span>

          <div
            v-if="lastSyncedMod"
            class="mt-0.5 text-xs text-secondary font-mono truncate"
          >
            <component
              :is="getModStatusIcon(lastSyncedMod.status)"
              :class="getModStatusColor(lastSyncedMod.status) + ' w-3.5 h-3.5 inline-block mr-1'"
            />
            {{ lastSyncedMod.modName }}
          </div>

          <div v-if="errorMessage" class="mt-1 text-xs text-danger font-mono">
            {{ errorMessage }}
          </div>
        </div>

        <button v-if="isDev"
          class="shrink-0 flex items-center justify-center p-1 rounded-full
                 text-secondary hover:text-primary hover:bg-interactive-bg transition-colors cursor-pointer"
          @click="sM()"
        >
          <RefreshCcw class="w-4 h-4" />
        </button>
        <button
          class="shrink-0 flex items-center justify-center p-1 rounded-full
                 text-secondary hover:text-primary hover:bg-interactive-bg transition-colors cursor-pointer"
          @click="handleClose"
        >
          <X class="w-4 h-4" />
        </button>
      </div>
    </template>

    
    <template #footer>
      <div class="flex justify-end shrink-0 p-2 px-4">
        <Button
          :label="t('modals.sync.close', 'Close')"
          variant="default"
          @click="handleClose"
        />
      </div>
    </template>

    <div class="flex flex-col gap-2 flex-1 min-h-0 px-4">

      <div class="space-y-2 shrink-0">
        <div class="flex justify-between items-center">
          <span class="text-sm text-secondary font-mono">
            {{ progress }}%
          </span>

          <span
            v-if="syncStateStore.progress.current"
            class="text-sm text-secondary font-mono opacity-50"
          >
            {{ syncStateStore.progress.current }} /
            {{ syncStateStore.progress.total }}
            {{ t('modals.sync.mods', 'mods') }}
          </span>
        </div>

        <div class="h-2 bg-interactive-bg rounded-full overflow-hidden">
          <div
            class="h-full bg-accent-primary rounded-full"
            :class="progress === 0 ? 'transition-none' : 'transition-all duration-150 ease-out'"
            :style="{ width: `${progress}%` }"
          />
        </div>
      </div>

      <div class="rounded-lg overflow-hidden font-mono flex flex-col flex-1 min-h-0">

        <div class="py-1.5 flex justify-between items-center bg-border/10 shrink-0">
          <span class="text-xs text-secondary">
            {{ t('modals.sync.log', 'Sync Log') }}
          </span>

          <span
            v-if="syncedMods.length"
            class="text-xs py-0.5 px-1 rounded-xl text-secondary"
          >
            {{ syncedMods.length }}
            {{ t('modals.sync.mods', 'mods') }}
          </span>
        </div>

        <div
          v-if="!syncedMods.length"
          class="flex-1 flex items-center justify-center text-xs text-secondary italic"
        >
          {{ t('modals.sync.waitingToStart', 'Waiting for sync to start...') }}
        </div>

        <div
          v-else
          v-bind="containerProps"
          class="flex-1 min-h-0"
          style="height: 100%"
        >
          <div v-bind="wrapperProps">
            <div
              v-for="item in list"
              :key="item.index"
              class="flex items-center gap-1 py-1 px-2 rounded text-xs hover:bg-interactive-bg/50 transition-colors"
              style="height: 36px"
            >
              <span class="text-secondary opacity-50 shrink-0 w-14 tabular-nums">
                {{ formatTimestamp(item.data.timestamp) }}
              </span>

              <span
                class="shrink-0 text-xs font-medium px-1.5 py-0.5 rounded-full"
                :class="getModStatusColor(item.data.status)"
              >
                <component
                  :is="getModStatusIcon(item.data.status)"
                  class="w-4 h-4"
                />
              </span>

              <div class="flex-1 min-w-0 gap-2 flex">
                <span
                  class="text-primary truncate block"
                  :title="item.data.modName"
                >
                  {{ item.data.modName }}
                </span>

                <span
                  v-if="item.data.error"
                  class="text-danger text-xs mr-2"
                  :title="'details' in item.data.error ? item.data.error.details : ''"
                >
                  {{ item.data.error.type }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <div class="flex flex-wrap items-center py-2 gap-3 border-t border-border shrink-0">
          <div class="flex items-center gap-1.5 text-xs text-secondary">
            <ArrowRightFromLine class="w-3.5 h-3.5 text-success" />
            <span>{{ t('modals.sync.synced', 'Synced') }}</span>
          </div>

          <div class="flex items-center gap-1.5 text-xs text-secondary">
            <ArrowLeftFromLine class="w-3.5 h-3.5 text-warning" />
            <span>{{ t('modals.sync.removed', 'Removed') }}</span>
          </div>
          <div class="flex items-center gap-1.5 text-xs text-secondary">
            <ArrowRightLeft class="w-3.5 h-3.5 text-info" />
            <span>{{ t('modals.sync.upToDate', 'Up to Date') }}</span>
          </div>

          <div class="flex items-center gap-1.5 text-xs text-secondary">
            <TriangleAlert class="w-3.5 h-3.5 text-danger" />
            <span>{{ t('modals.sync.failed', 'Failed') }}</span>
          </div>
        </div>

      </div>


    </div>
  </Modal>
</template>