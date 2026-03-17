// composables/useModInstaller.ts
import { open } from '@tauri-apps/plugin-dialog'
import { getErrorMessage } from '../utils/errors'
import { useI18n } from 'vue-i18n'
import { useToast } from 'primevue/usetoast'
import { useLoggingStore } from '../stores/logging'
import { useModsStore } from '../stores/mods'


// composable because modstab and cahracters tab will both need to install mods, and we want to keep the logic in one place
export function useModInstall() {
  const modsStore = useModsStore()
  const loggingStore = useLoggingStore()
  const toast = useToast()
  const { t } = useI18n()

  async function installFromZip() {
    const file = await open({
      multiple: false,
      filters: [{ name: 'Archive Files', extensions: ['zip', 'rar', '7z', 'tar', 'gz'] }]
    })

    loggingStore.logDebug("Selected file for mod installation from zip:", file)

    if (file && typeof file === 'string') {
      try {
        const modName = await modsStore.installModFromZip(file)

        toast.add({
          closable: true,
          summary: t('modsTab.notifications.modInstallSuccess.title'),
          detail: t('modsTab.notifications.modInstallSuccess.description', { modName }),
          life: 3000
        })

        return modName
      } catch (error) {
        loggingStore.logError("Error installing mod from zip:", error)

        const errorMsg = getErrorMessage(
          t,
          error instanceof Error ? error.message : String(error)
        )

        toast.add({
          closable: true,
          summary: t('errors.modInstallFailed.title'),
          detail: errorMsg,
          life: 5000,
          severity: 'error'
        })
      }
    }
  }

  async function installFromFolder() {
    const folder = await open({
      directory: true,
      multiple: false
    })

    loggingStore.logDebug("Selected folder for mod installation:", folder)

    if (folder && typeof folder === 'string') {
      try {
        const modName = await modsStore.installModFromFolder(folder)

        toast.add({
          closable: true,
          summary: t('modsTab.notifications.modInstallSuccess.title'),
          detail: t('modsTab.notifications.modInstallSuccess.description', { modName }),
          life: 3000
        })

        return modName
      } catch (error) {
        loggingStore.logError("Error installing mod from folder:", error)

        const errorMsg = getErrorMessage(
          t,
          error instanceof Error ? error.message : String(error)
        )

        toast.add({
          closable: true,
          summary: t('errors.modInstallFailed.title'),
          detail: errorMsg,
          life: 5000,
          severity: 'error'
        })
      }
    }
  }

  return {
    installFromZip,
    installFromFolder
  }
}