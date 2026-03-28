import { appDataDir } from '@tauri-apps/api/path'
import { ref, onMounted } from 'vue'

export function useAppDir() {
  const baseDir = ref('')
  onMounted(async () => { baseDir.value = await appDataDir() })
  return baseDir
}