export interface Settings {
    theme?: string;
    language?: string;
    gameDirectory?: string | null;
    stagingDirectory?: string | null;
    searchModsRecursively?: boolean;
    syncMethod?: string;
    checkForAppUpdates?: boolean;
    autoUpdateModPreview?: boolean;
    autoUpdateGameData?: boolean;
    skipUpdateVersion?: string | null;
    autoSyncMods?: boolean;
    isFirstLaunch?: boolean;
}

export interface SettingsStoreType {
    settings: Readonly<{ value: Settings }>;
    loadSettings: () => Promise<void>;
    getSettings: () => Promise<Settings>;
    saveSettings: (value: Partial<Settings>) => Promise<any>;
    getAppVersion: () => Promise<string | null>;
    getModPreviewVersion: () => Promise<string | null>;
    getBrowndustxVersion: () => Promise<{ status: "INSTALLED" | "NOT_INSTALLED" | "GAME_NOT_FOUND", version: string } | null>;
    getGameVersion: () => Promise<string | null>;
    checkForAppUpdate: () => Promise<any>;
    checkForModPreviewUpdate: () => Promise<any>;
    updateModPreview: () => Promise<any>;
    updateGameData: () => Promise<any>;
    locateGamePath: () => Promise<string[] | null>;
    validateGamePath: (path: string) => Promise<boolean>;
    availableThemes: { label: string; value: string }[];
}
