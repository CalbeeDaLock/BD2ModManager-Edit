
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

const isPortable = ref<boolean | null>(null);
const alreadyCalled = ref(false);

export function usePortable() {
  async function fetchPortable() {
    if (!alreadyCalled.value) {
      try {
        isPortable.value = await invoke<boolean>("is_portable")
      } catch (e) {
        console.error(e)
        isPortable.value = false
      }
      alreadyCalled.value = true
    }
  }

  fetchPortable()

  return { isPortable };
}