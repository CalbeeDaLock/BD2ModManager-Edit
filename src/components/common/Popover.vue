<script setup lang="ts">
import { flip, offset, shift, useFloating, type Placement } from '@floating-ui/vue'
import { onClickOutside } from '@vueuse/core';
import { ref } from 'vue'


const props = defineProps<{
  placement?: Placement
}>()

const triggerRef = ref<HTMLElement | null>(null)
const contentRef = ref<HTMLElement | null>(null)
const isOpen = ref(false)

const open = () => (isOpen.value = true)
const close = () => (isOpen.value = false)
const toggle = () => (isOpen.value = !isOpen.value)

const { floatingStyles } = useFloating(triggerRef, contentRef, {
  placement: props.placement || 'bottom',
  middleware: [offset(4), shift(), flip()],
})

onClickOutside(contentRef, close, { ignore: [triggerRef] })

const slots = defineSlots<{
  trigger(props: {
    isOpen: boolean
    open: () => void
    close: () => void
    toggle: () => void
  }): any
  default(): any
}>()
</script>

<template>
  <div class="h-full">
    <div
      ref="triggerRef"
      class="h-full"
    >
      <slot
        name="trigger"
        :isOpen="isOpen"
        :open="open"
        :close="close"
        :toggle="toggle"
      />
    </div>

    <div
      v-if="isOpen"
      ref="contentRef"
      :style="floatingStyles"
      class="z-50"
    >
      <slot />
    </div>
  </div>
</template>