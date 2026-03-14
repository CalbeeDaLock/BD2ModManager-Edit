import { computed, readonly, ref } from "vue"
import { useSettingsStore } from "./settings"
import { defineStore } from "pinia"

export const useThemesStore = defineStore('themes', () => {
    const settingsStore = useSettingsStore()
    const availableThemes = ref([{
        label: 'Dark', value: 'dark'
    }
    ])
    const currentTheme = computed(() => settingsStore.settings.theme)

    return {
        availableThemes: readonly(availableThemes),
        currentTheme
    }
})