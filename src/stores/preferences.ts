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


  return {
    visibleModListColumns,
    enableModTypeColors,
    modNameDisplay,
    characterDisplay,
    modTypeDisplay
      };
});
