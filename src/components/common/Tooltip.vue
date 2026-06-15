<script setup lang="ts">
import { autoUpdate, offset, shift, useFloating } from '@floating-ui/vue';
import { computed, ref } from 'vue'

defineProps<{
  text: string
}>()

const triggerRef = ref<HTMLElement | null>(null)
const tooltipRef = ref<HTMLElement | null>(null)
const { floatingStyles } = useFloating(triggerRef, tooltipRef, {
  placement: 'top',
  whileElementsMounted: autoUpdate,
  middleware: [offset(4), shift()],
})

const isVisible = ref(false)
const isRendered = ref(false)
let hideTimeout: ReturnType<typeof setTimeout> | null = null
let renderTimeout: ReturnType<typeof setTimeout> | null = null

const mergedStyles = computed(() => ({
  ...floatingStyles.value,
  visibility: isRendered.value ? 'visible' as const : 'hidden' as const,
  pointerEvents: 'none' as const,
}))

function show() {
  if (hideTimeout) { clearTimeout(hideTimeout); hideTimeout = null }
  if (renderTimeout) { clearTimeout(renderTimeout); renderTimeout = null }
  isRendered.value = true
  requestAnimationFrame(() => { isVisible.value = true })
}

function hide() {
  hideTimeout = setTimeout(() => {
    isVisible.value = false
    renderTimeout = setTimeout(() => { isRendered.value = false }, 150)
  }, 100)
}
</script>

<template>
  <div ref="triggerRef" class="relative inline-flex" @mouseenter="show" @mouseleave="hide">
    <slot />
    <Teleport to="body">
      <div
        ref="tooltipRef"
        :style="mergedStyles"
        class="z-9999 w-max max-w-xs rounded-md border border-border-default bg-bg-elevated text-text-primary px-3 py-2 text-sm shadow-lg transition-all duration-150"
        :class="isVisible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-1'"
      >
        {{ text }}
      </div>
    </Teleport>
  </div>
</template>