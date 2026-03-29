import { readonly, ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core"
import { useLoggingStore } from "./logging";

interface Settings {
    theme?: string,
    language?: string,
    gameDirectory?: string | null,
    stagingDirectory?: string | null,
    searchModsRecursively: boolean,  
    syncMethod?: string,
    checkForAppUpdates: boolean,     
    autoUpdateModPreview: boolean,   
    autoUpdateGameData: boolean,     
    skipUpdateVersion?: string | null,
    autoSyncMods: boolean,           
    isFirstLaunch: boolean,          
}

// [TODO]: REMOVE ALL THESE SET FUNCTIONS AND JUST USE A GENERIC SETTER FUNCTION

export const useSettingsStore = defineStore("settings", () => {
    const settings = ref<Settings>({} as Settings)
    const updateStatus = ref<{
        checking: boolean,
        version: string,
        currentVersion: string,
    } | null>(null)
    const loggingStore = useLoggingStore()

    async function loadSettings() {
        settings.value = await invoke<Settings>("get_settings");
    }

    async function getSettings(): Promise<Settings> {
        // const result = await invoke<Settings>("get_settings");
        // Object.assign(settings, result);
        return invoke<Settings>("get_settings");
    }

    async function saveSettings(value: Partial<Settings>) {
        // ex. {theme: ..., language: ...}
        // ex 2. {gameDirectory: ...}
        return invoke("set_settings", { value }).then((_value) => {
            loadSettings() // or Object.assign(settings, value)
        })
    }

    async function locateGamePath(): Promise<string[] | null> {
        // [TODO] find on steam too
        try {
            const paths = await invoke<string[]>("locate_game");
            return paths;
        } catch (err) {
            loggingStore.logError("Failed to locate game path", err);
            return null;
        }
    }

    function validateGamePath(path: string): Promise<boolean> {
        return invoke<boolean>("validate_game_path", { path })
    }

    function getAppVersion(): Promise<string | null> {
        return invoke('get_app_version')
    }

    function getModPreviewVersion(): Promise<string | null> {
        return invoke('get_mod_preview_version')
    }

    async function checkForAppUpdate() {
        updateStatus.value = {
            checking: true,
            version: '',
            currentVersion: '',
        }

        try {
            // it will block untils downloades finisih
            await invoke<{ version: string, currentVersion: string }>('check_for_app_update').then((result) => {
                if (result) {
                    updateStatus.value = {
                        checking: false,
                        version: result.version,
                        currentVersion: result.currentVersion,
                    }
                }
            })
        } catch (err) {
            loggingStore.logError("Failed to check for app update", err)
            updateStatus.value = null
        } finally {
            updateStatus.value!.checking = false
        }

        return updateStatus.value
    }

    async function installAppUpdate() {
        await invoke('install_app_update')
    }

    function checkForModPreviewUpdate() {
        return invoke('check_for_mod_preview_update')
    }

    function updateModPreview() {
        return invoke('update_mod_preview')
    }

    function updateGameData() {
        return invoke('update_game_data')
    }

    

    // it will stay on settings for now
    function getBrowndustxVersion(): Promise<{
        status: "INSTALLED" | "NOT_INSTALLED" | "GAME_NOT_FOUND" | "INSTALLED_BUT_OUTDATED",
        version: string
    } | null> {
        return invoke('get_browndustx_version')
    }

    function getGameVersion(): Promise<string | null> {
        return invoke('get_game_version')
    }

    const availableThemes = ref([
        {
            label: 'Dark', value: 'dark',
        }
    ])

    return {
        settings: readonly(settings),
        loadSettings,
        
        getSettings,
        saveSettings,
        
        // version/update
        getAppVersion,
        getModPreviewVersion,
        getBrowndustxVersion,
        getGameVersion,
        
        checkForAppUpdate,
        checkForModPreviewUpdate,
        updateModPreview,
        updateGameData,
        // checkForDataUpdate,
        
        // BrownDustX
        locateGamePath,
        validateGamePath,
        updateStatus: readonly(updateStatus),
        installAppUpdate,

        availableThemes: availableThemes,
    }
})