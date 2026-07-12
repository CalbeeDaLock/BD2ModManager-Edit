import { invoke } from "@tauri-apps/api/core";

import { useSettingsStore } from "../stores/settings";
import { useLoggingStore } from "../stores/logging";

// When the sync method is "symlink", Windows requires the app to run elevated.
// Elevation can't be granted in place, so we relaunch the app through a UAC
// prompt.
export function useSymlinkElevation() {
  const settingsStore = useSettingsStore();
  const loggingStore = useLoggingStore();

  // Returns true when elevation is not required for the current sync method,
  // i.e. the method is not "symlink". Otherwise checks the current process.
  async function needsElevation(): Promise<boolean> {
    if (settingsStore.settings.syncMethod !== "symlink") return false;

    const elevated = await invoke<boolean>("is_elevated").catch(() => false);
    return !elevated;
  }

  // Ensures the app is elevated when the sync method is "symlink". Returns true
  // when we're clear to sync, and false when a relaunch was requested (the
  // current process is about to exit). Throws "SymlinkAdminRequired" when the
  // user declines the UAC prompt.
  async function ensureSymlinkElevation(): Promise<boolean> {
    if (!(await needsElevation())) return true;

    try {
      await invoke("relaunch_as_admin");
    } catch (error) {
      loggingStore.logError("User declined elevation for symlink sync:", error);
      throw "SymlinkAdminRequired";
    }
    return false;
  }

  return { needsElevation, ensureSymlinkElevation };
}
