import { Virtualizer } from "@tanstack/vue-virtual";
import { computed, onActivated, onDeactivated, onMounted, onUnmounted, ref, Ref } from "vue";

export function useSaveScroll(virtualizer: Ref<Virtualizer<HTMLElement, Element>>) {
    const scrollContainerRef = computed(() => {
        if (virtualizer) {
            return virtualizer.value?.scrollElement as HTMLElement
        }
    })
    const scrollTop = ref(0)

    function saveScroll() {
        if (scrollContainerRef.value) {
            scrollTop.value = scrollContainerRef.value.scrollTop
        }
    }

    function restoreScroll() {
        if (scrollContainerRef.value) {
            virtualizer.value?.scrollToOffset(scrollTop.value)
        }
    }

    onActivated(() => {
        restoreScroll()
    })

    onMounted(() => {
        restoreScroll()
    })

    onDeactivated(() => {
        saveScroll()
    })

    onUnmounted(() => {
        saveScroll()
    })

    return { scrollTop }
}
