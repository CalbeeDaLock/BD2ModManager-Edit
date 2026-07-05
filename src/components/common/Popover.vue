<script setup lang="ts">
import { autoUpdate, flip, offset, shift, useFloating, type Placement } from '@floating-ui/vue'
import { onClickOutside } from '@vueuse/core'
import { TransitionRoot } from '@headlessui/vue'

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
  placement: props.placement ?? 'bottom',
  strategy: 'fixed',
  middleware: [offset(4), flip(), shift({ padding: 8 })],
  whileElementsMounted: autoUpdate,
})

onClickOutside(contentRef, close, { ignore: [triggerRef] })

defineSlots<{
  trigger(props: {
    isOpen: boolean
    open: () => void
    close: () => void
    toggle: () => void
  }): any
  default(props: { close: () => void }): any
}>()
</script>

<template>
  <div ref="triggerRef">
    <slot
      name="trigger"
      :isOpen="isOpen"
      :open="open"
      :close="close"
      :toggle="toggle"
    />
  </div>

  <Teleport to="body">
    <div
      ref="contentRef"
      :style="floatingStyles"
      class="z-50"
      :class="{ 'pointer-events-none': !isOpen }"
    >
      <TransitionRoot
        :show="isOpen"
        enter="transition ease-out duration-100"
        enter-from="transform opacity-0 scale-95"
        enter-to="transform opacity-100 scale-100"
        leave="transition ease-in duration-75"
        leave-from="transform opacity-100 scale-100"
        leave-to="transform opacity-0 scale-95"
      >
        <slot :close="close" />
      </TransitionRoot>
    </div>
  </Teleport>
</template>