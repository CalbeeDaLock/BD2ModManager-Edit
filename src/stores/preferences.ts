import { useLocalStorage } from "@vueuse/core";
import { defineStore } from "pinia";

export const usePreferencesStore = defineStore("preferences", () => {
  const visibleModListColumns = useLocalStorage("columns", [
    "name",
    "character",
    "modType",
    "author",
  ]);  
  
  const modNameDisplay = useLocalStorage("modNameDisplay", "short");
  const characterDisplay = useLocalStorage("characterDisplay", "full");
  const modTypeDisplay = useLocalStorage("modTypeDisplay", "full");
  const enableModTypeColors = useLocalStorage("modTypeColors", true);
  const forceEnglishNames  = useLocalStorage("forceEnglishNames", false)
  const showDatingInCharacters = useLocalStorage("showDatingInCharacters", true);

  // Custom display names for wallpaper mods, keyed by the wallpaper mod id
  // (the `.modfile` filename without its extension). Edited from the
  // Wallpapers management tab.
  const wallpaperNicknames = useLocalStorage<Record<string, string>>(
    "wallpaperNicknames",
    {}
  )


  return {
    visibleModListColumns,
    enableModTypeColors,
    modNameDisplay,
    characterDisplay,
    modTypeDisplay,
    forceEnglishNames,
    showDatingInCharacters,
    wallpaperNicknames
  };
});
