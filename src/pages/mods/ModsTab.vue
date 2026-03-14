<script setup lang="ts">
import { Folder, FolderMinus, FolderPlus, FolderSync, RefreshCcw } from "lucide-vue-next";

import { computed, h, onActivated, onDeactivated, onMounted, reactive, ref, useTemplateRef, watch } from "vue";
import { useLocalStorage, watchDebounced } from "@vueuse/core";
import { useToast } from "primevue/usetoast";
import { useI18n } from "vue-i18n";

import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import { openPath } from "@tauri-apps/plugin-opener";

import { useConfirm } from "../../plugins/ConfirmService";

import { BD2Mod, useModsStore } from "../../stores/mods";
import { useSettingsStore } from "../../stores/settings";
import { useHeader } from "../../composables/useHeader";

import { useLoggingStore } from "../../stores/logging";
import UpdateAuthorModal from "./modals/UpdateAuthorModal.vue";
import RenameModModal from "./modals/RenameModModal.vue";
import ModsHeader from "./ModsHeader.vue";
import Modlist from "./Modlist.vue";
import Button from "../../components/common/Button.vue";
import Menu from "../../components/common/Menu.vue";


let unlistenFns: Array<() => void> = []

const updateAuthorModal = useTemplateRef("updateAuthorModal")
const renameModModal = useTemplateRef("renameModModal")

const loggingStore = useLoggingStore()

const toast = useToast()
const confirm = useConfirm()

const { t } = useI18n();
const modsStore = useModsStore();
const settingsStore = useSettingsStore();

const isRefreshing = ref(false);
const isSyncing = ref(false);
const isUnsyncing = ref(false);

const bdxVersion = ref<{
  status: "INSTALLED" | "NOT_INSTALLED" | "GAME_NOT_FOUND",
  version: string
} | null>(null);

const debouncedSearchQuery = ref('');
const skipSyncConfirmation = useLocalStorage('skipSyncModsConfirmation', false)
const skipUnsyncConfirmation = useLocalStorage('skipUnsyncModsConfirmation', false)

let filters = reactive({
  searchQuery: '',
  // searchType: 'mod', // 'mod' | 'character' | 'author'
  modTypes: [] as ("Standing" | "Cutscene" | "Scene" | "Npc" | "Dating" | "Minigame")[],
  onlyEnabled: false,
  onlyDisabled: false,
  onlyConflicts: false,
  onlyErrors: false,
});

function getErrorMessage(error: any) {
  if (!error) return t('errors.unknownError')

  if (error === "GameDirectoryNotSet") {
    return t('errors.gameDirectoryNotSet')
  }

  if (error.SyncMethodInvalid) {
    return t('errors.syncMethodInvalid', { method: error.SyncMethodInvalid })
  }

  if (error.SyncFailed) {
    const sync = error.SyncFailed

    switch (sync.type) {
      case "SymlinkAdminRequired":
        return t('errors.symlinkAdminRequired')

      case "ModPathNotFound":
        return t('errors.modPathNotFound', { path: sync.details })

      case "CopyFailed":
        return t('errors.copyFailed', { error: sync.details })

      case "SymlinkFailed":
        return t('errors.symlinkFailed', { error: sync.details })

      case "HardlinkFailed":
        return t('errors.hardlinkFailed', { error: sync.details })

      case "DirectoryCreationFailed":
        return t('errors.directoryCreationFailed', { error: sync.details })

      case "RemovalFailed":
        return t('errors.removalFailed', { error: sync.details })

      default:
        return t('errors.unknownError', { error: JSON.stringify(sync) })
    }
  }

  if (error.UnknownError) {
    return t('errors.unknownError', { error: error.UnknownError })
  }

  return t('errors.unknownError', { error: JSON.stringify(error) })
}

const totalModsCount = computed(() => modsStore.mods.length)
const enabledModsCount = computed(() => modsStore.mods.filter(mod => mod.enabled).length)

const filteredMods = computed(() => {
  return modsStore.mods.filter((mod) => {
    const conflictMatch = debouncedSearchQuery.value.match(/conflictsWith:"([^"]+)"/i)
    const conflictFilter = conflictMatch ? conflictMatch[1].toLowerCase() : null

    const cleanedQuery = debouncedSearchQuery.value.replace(/conflictsWith:"[^"]*"/i, '').trim()

    if (cleanedQuery) {
      const queries = [...cleanedQuery.toLowerCase().matchAll(/"([^"]+)"|([^,]+)/g)]
        .map(match => {
          const raw = match[0].trim()
          const isExact = raw.startsWith('"') && raw.endsWith('"')
          return {
            value: isExact ? raw.slice(1, -1) : raw,
            exact: isExact
          }
        })
        .filter(q => q.value.length > 0)

      const matchesAnyQuery = queries.some(({ value, exact }) => {
        if (exact) {
          return mod.name.toLowerCase() === value
        }
        return mod.name.toLowerCase().includes(value) ||
          (mod.author && mod.author.toLowerCase().includes(value)) ||
          (mod.character &&
            `${mod.character.character.toLowerCase()} - ${mod.character.costume.toLowerCase()}`.includes(value))
      })

      if (!matchesAnyQuery) return false
    }

    if (conflictFilter) {
      if (
        mod.name.toLowerCase() !== conflictFilter &&
        !mod.conflictsWith.map(c => c.toLowerCase()).includes(conflictFilter)
      ) {
        return false
      }
    }

    if (filters.modTypes.length > 0) {
      if (!mod.modType) return false
      if (!filters.modTypes.includes(mod.modType.type)) return false
    }

    if (filters.onlyEnabled && !mod.enabled) return false
    if (filters.onlyDisabled && mod.enabled) return false
    if (filters.onlyErrors && mod.errors.length === 0) return false
    // mods with conflicts, a conflict is when the mod has at least one mod in its conflictsWith array that is also enabled
    // if (filters.onlyConflicts && mod.conflictsWith.length === 0) return false
    if (filters.onlyConflicts && mod.conflictingMods.length === 0) return false

    return true
  })
})

function handleEnableMods(mods: BD2Mod[]) {
  loggingStore.logDebug("Enabling mods:", JSON.stringify(mods.map(m => m.name)));
  modsStore.enableMods(mods.map(m => m.name))
    .then(() => {
      if (settingsStore.settings.autoSyncMods) {
        loggingStore.logDebug("Auto-sync is enabled, syncing mods after enabling.");
        modsStore.syncMods()
      }
    }).catch((error) => {
      loggingStore.logError("Error enabling mods:", error);
    })
}

function handleDisableMods(mods: BD2Mod[]) {
  loggingStore.logDebug("Disabling mods:", JSON.stringify(mods.map(m => m.name)));
  modsStore.disableMods(mods.map(m => m.name)).then(() => {
    if (settingsStore.settings.autoSyncMods) {
      loggingStore.logDebug("Auto-sync is enabled, syncing mods after disabling.");
      modsStore.syncMods()
    }
  }).catch((error) => {
    loggingStore.logError("Error disabling mods:", error);
  })
}

function handlePreviewMod(mod: BD2Mod) {
  loggingStore.logDebug("Previewing mod:", mod.name);

  if (mod.errors.length > 0) {
    toast.add({
      severity: "error",
      closable: true,
      detail: t("errors.modPreviewContainsErrors"),
      life: 3000
    })
    return
  }

  modsStore.previewMod(mod.name).then(() => {
    // loggingStore.logDebug("Mod previewed successfully:", mod.name);
    // toast.add({
    //   severity: "success",
    //   closable: true,
    //   detail: t("mods.messages.previewSuccess.description", { modName: mod.name }),
    //   life: 3000
    // })
  }).catch((error) => {
    let errorMsg = getErrorMessage(error);
    toast.add({
      severity: "error",
      closable: true,
      detail: errorMsg,
      life: 5000
    })

    loggingStore.logError("Error previewing mod:", error);
  })
}

async function handleOpenModFolder(mod: BD2Mod) {
  loggingStore.logDebug("Opening mod folder:", mod.name);
  // [TODO] adds checks if folder exists
  await openPath(mod.path) 
}

async function handleOpenModsFolder() {
  let stagingDir = settingsStore.settings.stagingDirectory

  loggingStore.logDebug("Opening mods folder: ", stagingDir);

  if (!stagingDir) {
    loggingStore.logError("Staging directory is not set.");

    return toast.add({
      severity: "error",
      closable: true,
      detail: t('errors.stagingDirectoryNotSet'),
      life: 5000
    });
  }
  // [TODO] check if directory exists
  await openPath(stagingDir);
}

async function handleSyncMods() {
  try {
    loggingStore.logDebug(`Syncing mods [confirmation skipped: ${skipSyncConfirmation.value}]`);

    const hasConflicts = modsStore.mods.some(mod => mod.conflictingMods.length > 0)

    if (hasConflicts) {
      const { confirmed } = await confirm.confirm({
        title: t('mods.confirmations.syncModsWithConflicts.title'),
        message: t('mods.confirmations.syncModsWithConflicts.message'),
        acceptButton: {
          label: t('common.actions.continue'),
        },
        rejectButton: {
          label: t('common.actions.cancel'),
        },
      })

      if (!confirmed) {
        loggingStore.logDebug("User cancelled sync due to conflicts.");
        return
      }
    }

    if (!skipSyncConfirmation.value && false) {
      const { confirmed, rememberChoice } = await confirm.confirm({
        title: t('mods.confirmations.syncModsConfirmation.title'),
        message: t('mods.confirmations.syncModsConfirmation.message'),
        acceptButton: {
          label: t('common.actions.continue'),
        },
        rejectButton: {
          label: t('common.actions.cancel'),
        },
        showRememberChoice: true
      })

      if (!confirmed) {
        return
      }

      if (rememberChoice) {
        skipSyncConfirmation.value = true
      }
    }

    isSyncing.value = true

    // [TODO] syncmods return error
    let result = await modsStore.syncMods().then((res) => {
      loggingStore.logDebug("Mods sync result:", res);

      return res;
    }).catch((error) => {
      loggingStore.logError("Error during mods sync:", error);
      throw error;
    });

    loggingStore.logDebug(`Command mods sync called succesfully: ${result}.`);

    toast.add({
      closable: true,
      summary: t('mods.messages.syncSuccess.title'),
      detail: t('mods.messages.syncSuccess.description'),
      life: 3000,
    })
    // if (result) {
    // }
  } catch (error: any) {
    loggingStore.logError("Error syncing mods:", JSON.stringify(error));

    console.log(typeof error, error instanceof Error, error.message);

    let errorMessage = getErrorMessage(error)

    toast.add({
      closable: true,
      summary: t('errors.syncFailed'),
      detail: errorMessage,
      life: 5000,
      severity: 'error'
    });
  } finally {
    isSyncing.value = false
  }
}

async function handleUnsyncMods() {
  try {
    if (!skipUnsyncConfirmation.value) {
      const { confirmed, rememberChoice } = await confirm.confirm({
        title: t('mods.confirmations.unsyncModsConfirmation.title'),
        message: t('mods.confirmations.unsyncModsConfirmation.message'),
        acceptButton: {
          label: t('common.actions.continue'),
        },
        rejectButton: {
          label: t('common.actions.cancel'),
        },
        showRememberChoice: true
      })

      if (!confirmed) {
        return
      }

      if (rememberChoice) {
        skipUnsyncConfirmation.value = true
      }
    }

    isUnsyncing.value = true

    const result = await modsStore.unsyncMods();

    isUnsyncing.value = false

    loggingStore.logDebug(`Command mods unsync called succesfully: ${result}.`);

    toast.add({
      closable: true,
      summary: t('mods.messages.unsyncSuccess.title'),
      detail: t('mods.messages.unsyncSuccess.description'),
      life: 3000,
    });
  } catch (error: any) {
    loggingStore.logError("Error unsyncing mods:", error);

    let errorMessage = getErrorMessage(error)

    toast.add({
      closable: true,
      summary: t('errors.unsyncFailed'),
      detail: errorMessage,
      life: 5000,
      severity: 'error'
    });

    isUnsyncing.value = false
  }
}

async function handleInstallFromZip() {
  const file = await open({
    multiple: false,
    filters: [{ name: 'ZIP Files', extensions: ['zip'] }]
  });

  loggingStore.logDebug("Selected file for mod installation from zip:", file);

  if (file && typeof file === 'string') {
    try {
      let modName = await modsStore.installModFromZip(file);

      loggingStore.logDebug(`Mod "${modName}" installed successfully from zip: ${file}`);

      toast.add({
        closable: true,
        summary: t('mods.modInstalledSuccess'),
        detail: modName,
        life: 3000,
      });
    } catch (error) {
      loggingStore.logError("Error installing mod from zip:", error);

      let errorMsg = getErrorMessage(error instanceof Error ? error.message : String(error));

      toast.add({
        closable: true,
        summary: t('errors.modInstallFailed'),
        detail: errorMsg,
        life: 5000,
        severity: 'error'
      });
    }
  }
}

async function handleInstallFromFolder() {
  const folder = await open({
    directory: true,
    multiple: false
  });

  loggingStore.logDebug("Selected folder for mod installation:", folder);

  if (folder && typeof folder === 'string') {
    try {
      let modName = await modsStore.installModFromFolder(folder);

      loggingStore.logDebug(`Mod "${modName}" installed successfully from folder: ${folder}`);

      toast.add({
        closable: true,
        summary: t('mods.modInstalledSuccess'),
        detail: modName,
        life: 3000,
      });
    } catch (error) {
      loggingStore.logError("Error installing mod from folder:", error);

      let errorMsg = getErrorMessage(error instanceof Error ? error.message : String(error))
      toast.add({
        closable: true,
        summary: t('errors.modInstallFailed'),
        detail: errorMsg,
        life: 5000,
        severity: 'error'
      });
    }
  }
}

function handleChangeModAuthor(mod: BD2Mod) {
  loggingStore.logDebug("Changing author for mod:", mod.name, "Current author:", mod.author);
  updateAuthorModal.value?.open({
    modName: mod.name,
    modAuthor: mod.author || '',
    onSave: (newAuthor: string) => {
      loggingStore.logDebug(`Change author of mod "${mod.name}" to "${newAuthor}"`);
      modsStore.setModAuthor(mod.name, newAuthor);
    }
  });
}

// function handleEditModfile(mod: BD2Mod) {
//   // [TODO]
// }

async function handleDeleteMods(mods: BD2Mod[]) {
  loggingStore.logDebug("Deleting mods:", JSON.stringify(mods.map(m => m.name)));

  if (mods.length === 0) {
    loggingStore.logDebug("No mods selected for deletion, skipping.");
    return;
  }

  const result = await confirm.confirm({
    title: mods.length === 1 ? t('mods.confirmations.deleteMod.title') : t('mods.confirmations.deleteSelectedMods.title'),
    message: mods.length === 1 ? t('mods.confirmations.deleteMod.message', { modName: mods[0].name }) : t('mods.confirmations.deleteSelectedMods.message', { count: mods.length }),
    acceptButton: {
      label: t('common.actions.delete'),
    },
    rejectButton: {
      label: t('common.actions.cancel'),
    },
  })

  if (result.confirmed) {
    modsStore.deleteMods(mods.map(m => m.name));
  }
}

async function handleRefreshMods() {
  if (isRefreshing.value) {
    loggingStore.logDebug("Refresh already in progress, skipping.");
    return;
  }

  isRefreshing.value = true
  await modsStore.discoverMods()
  isRefreshing.value = false
}

async function updateBDXVersion() {
  try {
    const result = await settingsStore.getBrowndustxVersion();

    loggingStore.logDebug("BDX version:", JSON.stringify(result));

    if (result) {
      bdxVersion.value = {
        status: result.status,
        version: result.version
      };
    } else {
      bdxVersion.value = null;
    }

  } catch (error) {
    loggingStore.logError("Error getting BDX version:", error);
  }
}

// "7z" | "tar" | "gz" | "bz2" | "" | "tgz"
const SUPPORTED_FORMATS = [
  "rar",
  "zip",
  "7z",
  "gz",
  "bz2",
  "xz",
  "tgz"
]

async function setupEventListeners() {
  // remove existing listeners if any
  unlistenFns.forEach((unlisten) => unlisten())
  unlistenFns = []

  console.log("Setting up event listeners for ModsTab");

  const unlistenDragDrop = await listen("tauri://drag-drop", async (event: any) => {
    const paths = event.payload?.paths as string[]

    for (const path of paths) {
      try {
        let modName = null
        // [TODO] more support for others file types, .rar, 7z
        if (SUPPORTED_FORMATS.includes(`.${path.split('.').pop()?.toLowerCase() || ''}`)) {
          modName = await modsStore.installModFromZip(path)
        } else {
          modName = await modsStore.installModFromFolder(path)
        }

        toast.add({
          closable: true,
          summary: t('mods.messages.modInstallSuccess.title'),
          detail: t('mods.messages.modInstallSuccess.detail', { modName }),
          life: 3000,
        });

      } catch (error: any) {
        let errorMsg = getErrorMessage(error)

        toast.add({
          closable: true,
          summary: t('errors.modInstallFailed'),
          detail: `Failed to install mod from ${path.split('/').pop()}: ${errorMsg}`,
          life: 5000,
          severity: 'error'
        });
      }
    }
  })

  unlistenFns = [unlistenDragDrop]
}

onMounted(async () => {
  Promise.all([
    updateBDXVersion()
  ])
})

onActivated(async () => {
  loggingStore.logDebug("Mounting ModsTab, setting up event listeners");
  Promise.all([
    setupEventListeners()
  ])
})

onDeactivated(() => {
  loggingStore.logDebug("Unmounting ModsTab, removing event listeners");
  unlistenFns.forEach((unlisten) => unlisten())
  unlistenFns = []
})


watchDebounced(
  () => filters.searchQuery,
  (newValue) => {
    debouncedSearchQuery.value = newValue;
  },
  { debounce: 50 }
);

watch(() => settingsStore.settings.gameDirectory, (newDir, oldDir) => {
  // if (oldDir === null || oldDir === undefined) {
  //   loggingStore.logDebug("Skipping initial game directory load");
  //   return;
  // }

  // [info] It triggers when the game directory is set from config.json to settings store

  loggingStore.logDebug(`Game directory changed from "${oldDir}" to "${newDir}"`);

  if (newDir && newDir !== oldDir) {
    updateBDXVersion();
  }
});

watch(() => settingsStore.settings.stagingDirectory, (newDir, oldDir) => {
  loggingStore.logDebug("Staging directory changed from", oldDir, "to", newDir);

  if (oldDir === null || oldDir === undefined) {
    loggingStore.logDebug("Skipping initial settings load");
    return;
  }

  if (newDir && newDir !== oldDir) {
    loggingStore.logDebug("Staging directory changed, discovering mods...");
    modsStore.discoverMods();
  }
});

watch(() => settingsStore.settings.searchModsRecursively, (newValue, oldValue) => {
  if (oldValue === null || oldValue === undefined) {
    loggingStore.logDebug("Skipping initial searchModsRecursively load");
    return;
  }

  if (newValue !== oldValue) {
    loggingStore.logDebug("Search recursively changed:", oldValue, "→", newValue);
    modsStore.discoverMods();
  }
});

const addModMenuItems = computed(() => [
  {
    label: t('mods.actions.installFromZip'), clicked: handleInstallFromZip
  },
  {
    label: t('mods.actions.installFromFolder'), clicked: handleInstallFromFolder
  }
]);

useHeader({
  title: t("mods.title"),
  subtitle: computed(() =>
    t('mods.subtitle', {
      enabledModsCount: enabledModsCount.value,
      totalModsCount: totalModsCount.value
    })
  ),
  buttons: [
    {
      label: computed(() => t('mods.actions.refreshMods')),
      icon: RefreshCcw,
      action: async () => {
        await handleRefreshMods();
      }
    },
    {
      render: () =>
        h('div', { class: 'flex gap-2' }, [
          h(Menu, {}, {
            trigger: ({ toggle }: any) =>
              h(Button, {
                label: t('mods.actions.addMod'),
                icon: FolderPlus,
                variant: 'text',
                onClick: toggle
              }),
            content: () =>
              h(
                'ul',
                {},
                addModMenuItems.value.map(item =>
                  h(
                    'li',
                    { key: item.label },
                    h(
                      'button',
                      {
                        class: 'w-full cursor-pointer text-left px-4 py-2 hover:bg-interactive-bg-hover',
                        onClick: item.clicked
                      },
                      item.label
                    )
                  )
                )
              )
          }),
        ])
    },

  ]
})


function handleRenameMod(mod: BD2Mod) {
  loggingStore.logDebug("Renaming mod:", mod.name);
  renameModModal.value?.open({
    modName: mod.name,
    onSave: (newName: string) => {
      if (newName === mod.name) {
        return;
      }
      loggingStore.logDebug(`Change name  of mod "${mod.name}" to "${newName}" ${typeof newName}`);
      modsStore.renameMod(mod.name, newName);
    }
  });
}

function handleShowModConflicts(mod: BD2Mod) {
  // filters.searchQuery = `"${mod.name}", "${mod.conflictsWith.join(', ')}"`;
  // const names = [mod.name, ...mod.conflictsWith]
  // filters.searchQuery = names.map(n => `"${n}"`).join(', ')
  filters.searchQuery = `conflictsWith:"${mod.name}"`;
  // searhc by id?
  // show modal?
}

</script>
<template>
  <div class="flex flex-col h-full gap-0 select-none p-4 py-2">
    <UpdateAuthorModal ref="updateAuthorModal" />
    <RenameModModal ref="renameModModal" />

    <div class="shrink-0">
      <ModsHeader v-model:filters="filters" />
    </div>

    <div class="flex-1 overflow-hidden min-h-0 my-2">
      <Modlist :mods="filteredMods" @refresh-mods="handleRefreshMods" @enable-mods="handleEnableMods"
        @disable-mods="handleDisableMods" @change-mod-author="handleChangeModAuthor" @delete-mods="handleDeleteMods"
        @open-mod-folder="handleOpenModFolder" @preview-mod="handlePreviewMod" @rename-mod="handleRenameMod"
        @show-mod-conflicts="handleShowModConflicts" />
    </div>

    <div class="flex justify-between items-center shrink-0">
      <div class="flex flex-col">
        <span class="text-primary font-semibold">
          <template v-if="bdxVersion?.status == 'INSTALLED'">
            BrownDustX v{{ bdxVersion.version }}
          </template>
          <template v-else-if="bdxVersion?.status == 'GAME_NOT_FOUND'">
            {{ $t("mods.browndustx.gamePathNotSet") }}
          </template>
          <template v-else>
            {{ $t("mods.browndustx.notInstalled") }}
          </template>
        </span>
        <RouterLink to="bdx" class="text-secondary text-xs hover:underline">
          {{ $t("mods.browndustx.navigation") }}
        </RouterLink>
      </div>

      <div class="flex gap-2 text-primary">
        <Button variant="alt" :label="$t('mods.actions.openModsFolder')" :icon="Folder" @click="handleOpenModsFolder" />
        <Button :disabled="isSyncing || isUnsyncing" variant="alt" :label="$t('mods.actions.unsyncMods')"
          :icon="FolderMinus" @click="handleUnsyncMods" />
        <Button :disabled="isSyncing || isUnsyncing" variant="alt" :label="$t('mods.actions.syncMods')"
          :icon="FolderSync" @click="handleSyncMods" :class="{
            'bg-accent-primary! animate-pulse hover:animate-none hoverbg-accent-primary-hover!': false
          }" />
      </div>
    </div>
  </div>
</template>