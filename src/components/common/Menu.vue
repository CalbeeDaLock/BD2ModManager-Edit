<script lang="ts" setup>
import { TransitionRoot } from '@headlessui/vue';
import { onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

const menuRef = useTemplateRef('menuRef')
const triggerArea = useTemplateRef("triggerArea")
const isVisible = ref(false)

const toggleMenu = async () => {
    isVisible.value = !isVisible.value
}

const handleClickOutside = (event: MouseEvent) => {
  if (menuRef.value && !menuRef.value.contains(event.target as Node) && isVisible.value && !triggerArea.value?.contains(event.target as Node)) {
    isVisible.value = false
  }
}

// Track trigger position for absolute positioning
const triggerRect = ref<DOMRect | null>(null)

const updatePosition = () => {
  if (triggerArea.value) {
    triggerRect.value = triggerArea.value.getBoundingClientRect()
  }
}

onMounted(() => {
    document.addEventListener("click", handleClickOutside)
    window.addEventListener("scroll", updatePosition, true)
    window.addEventListener("resize", updatePosition)
})

onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside)
    window.removeEventListener("scroll", updatePosition, true)
    window.removeEventListener("resize", updatePosition)
})
</script>

<template>
    <div class="flex relative">
        <div ref="triggerArea" @click="updatePosition">
            <slot name="trigger" :toggle="toggleMenu"></slot>
        </div>
        
        <Teleport to="body">
            <TransitionRoot
                as="template"
                :show="isVisible"
                enter="transition ease-out duration-100"
                enter-from="transform opacity-0 scale-95"
                enter-to="transform opacity-100 scale-100"
                leave="transition ease-in duration-75"
                leave-from="transform opacity-100 scale-100"
                leave-to="transform opacity-0 scale-95"
            >
                <div 
                    ref="menuRef"
                    class="
                    fixed w-48 
                    bg-bg-surface
                    border-2 border-border rounded 
                    shadow-lg 
                    z-50
                    "
                    :style="{
                        top: `${(triggerRect?.bottom || 0) + 4}px`,
                        left: `${(triggerRect?.left || 0) + (triggerRect?.width || 0) / 2}px`,
                        transform: 'translateX(-50%)'
                    }"
                >
                    <slot name="content"></slot>
                </div>
            </TransitionRoot>
        </Teleport>
    </div>
</template>