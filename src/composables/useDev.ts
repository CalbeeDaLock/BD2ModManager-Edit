import { ref } from "vue"

export function useDev() {
    const isDev = ref<boolean>(import.meta.env.DEV)
    return { isDev }
}