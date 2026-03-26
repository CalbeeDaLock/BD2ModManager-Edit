<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from "vue"
import { storeToRefs } from "pinia"
import { useI18n } from "vue-i18n"

import { useSettingsStore } from "./stores/settings"
import { provideHeader } from "./composables/useHeader"

import Toast from "primevue/toast"
import Titlebar from "./components/Titlebar.vue"
import Header from "./components/Header.vue"
import ConfirmationDialog from "./components/ConfirmationDialog.vue"
import WelcomeModal from "./components/modals/WelcomeModal.vue"
import { useAppInitializer } from "./composables/useAppInit"
import { useConfirm } from "./plugins/ConfirmService"
import { useModsStore } from "./stores/mods"
import { getCurrentWindow } from "@tauri-apps/api/window"
import Sidebar from "./components/sidebar/Sidebar.vue"
import { useToast } from "primevue/usetoast"

const { t, locale } = useI18n()

const settingsStore = useSettingsStore()
const { settings } = storeToRefs(settingsStore)

const { initialize } = useAppInitializer()
const isWelcomeModalVisible = ref(false)
const confirm = useConfirm()
const modsStore = useModsStore()

const toast = useToast()

let unlistenClose: (() => void) | null = null

provideHeader()

onMounted(async () => {
  if (!import.meta.env.DEV) {
    document.addEventListener("contextmenu", (event) => event.preventDefault())
  }
  
  const {isFirstLaunch, isBrownDustXOutdated} = await initialize()

  if (isFirstLaunch) isWelcomeModalVisible.value = true
  if (isBrownDustXOutdated) {
    toast.add({
      severity: "warn",
      summary: t('app.notifications.brownDustXOutdated.title'),
      detail: t('app.notifications.brownDustXOutdated.message'),
      life: 10000,
    })
  }

  watch(
    () => settings.value.theme,
    (newTheme) => document.documentElement.setAttribute("data-theme", newTheme || "dark"),
    { immediate: true }
  )

  watch(
    () => settings.value.language,
    (newLanguage) => (locale.value = newLanguage || "en_US"),
    { immediate: true}
  )

  const currentWindow = getCurrentWindow()
  unlistenClose = await currentWindow.listen("tauri://close-requested", async () => {
    let isSyncNeeded = false
    try {
      isSyncNeeded = await modsStore.isSyncNeeded()
    } catch (error) {
      console.error("Error checking sync status:", error)
      currentWindow.destroy()
      return
    }

    if (!isSyncNeeded) {
      currentWindow.destroy()
      return
    }

    const result = await confirm.confirm({
      title: t('app.confirmations.exit.title'),
      message: t('app.confirmations.exit.message'),
      acceptButton: { label: t('app.confirmations.exit.actions.exit') },
      rejectButton: { label: t('common.actions.cancel') },
    })

    if (result.confirmed) currentWindow.destroy()
  })
})

onUnmounted(() => {
  unlistenClose?.()
})


</script>

<template>
  <div class="w-full h-full bg-bg-deep overflow-hidden transition-colors text-sm select-none flex flex-col">
    <WelcomeModal :visible="isWelcomeModalVisible" />
    <Toast position="bottom-center" />
    <ConfirmationDialog />
    <Titlebar />

    <main class="flex-1 flex overflow-hidden min-h-0">
      <Sidebar />

      <div class="flex-1 flex flex-col overflow-hidden min-h-0">
        <Header />
        <div class="flex-1 overflow-hidden min-h-0">
          <router-view v-slot="{ Component }">
            <transition name="fade" mode="out-in">
              <keep-alive :include="['ModsTab', 'CharactersTab']">
                <component :is="Component" class="h-full" />
              </keep-alive>
            </transition>
          </router-view>
        </div>
      </div>
    </main>
  </div>


</template>