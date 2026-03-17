
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { ref, Ref } from 'vue'

export enum SyncType {
  Sync = "Sync",
  Unsync = "Unsync"
}

export enum SyncProgressStatus {
  Synced = "Synced",
  UpToDate = "UpToDate",
  Removed = "Removed",
  Failed = "Failed"
}


export interface SyncStartEvent {
  type: SyncType
}

export interface SyncProgressEvent {
  type: SyncType
  status: SyncProgressStatus
  modName: string
  current: number
  total: number
  error?: SyncError
}

export interface SyncEndEvent {
  type: SyncType
  success: boolean
  synced: number
  total: number
  error?: SyncError
}

export type SyncError =
  | { type: "SymlinkAdminRequired" }
  | { type: "ModPathNotFound"; details: string }
  | { type: "CopyFailed"; details: string }
  | { type: "SymlinkFailed"; details: string }
  | { type: "HardlinkFailed"; details: string }
  | { type: "DirectoryCreationFailed"; details: string }
  | { type: "RemovalFailed"; details: string }
  | { type: "PermissionDenied" }
  | { type: "DiskFull" };

export interface SyncEventHandlers {
  onStart: (callback: (event: SyncStartEvent) => void) => void
  onProgress: (callback: (event: SyncProgressEvent) => void) => void
  onEnd: (callback: (event: SyncEndEvent) => void) => void
  clearEvents: () => void
}

export function useSyncEvents(): SyncEventHandlers {
  const unlistenFns: Ref<UnlistenFn[]> = ref([])

  const createListener = <T>(eventName: string) => {
    return async (callback: (payload: T) => void) => {
      try {
        const unlisten = await listen(eventName, (event) => {
          callback(event.payload as T)
        })
        unlistenFns.value.push(unlisten)
      } catch (error) {
        console.error(`Failed to listen to ${eventName}:`, error)
      }
    }
  }

  function clearEvents() {
    unlistenFns.value.forEach(unlisten => {
      try {
        unlisten()
      } catch (error) {
        console.error('Error during unlisten:', error)
      }
    })
    unlistenFns.value = []
  }

  return {
    onStart: createListener<SyncStartEvent>('sync-start'),
    onProgress: createListener<SyncProgressEvent>('sync-progress'),
    onEnd: createListener<SyncEndEvent>('sync-end'),
    clearEvents
  }
}