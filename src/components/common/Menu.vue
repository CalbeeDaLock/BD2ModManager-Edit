<script lang="ts" setup>
import { TransitionRoot } from '@headlessui/vue';
import { ref } from 'vue';
import { onClickOutside } from '@vueuse/core';
import { useFloating, autoUpdate, offset, flip, shift } from '@floating-ui/vue';

const containerRef = ref<HTMLElement | null>(null);
const triggerArea = ref<HTMLElement | null>(null);
const menuRef = ref<HTMLElement | null>(null);
const isVisible = ref(false);

const { floatingStyles } = useFloating(triggerArea, menuRef, {
  placement: 'bottom',
  strategy: 'fixed',
  middleware: [offset(4), flip(), shift({ padding: 8 })],
  whileElementsMounted: autoUpdate,
});

const toggleMenu = () => {
  isVisible.value = !isVisible.value;
};

onClickOutside(
  containerRef,
  () => { isVisible.value = false; },
  { ignore: [menuRef, triggerArea] }
);
</script>

<template>
  <div ref="containerRef" class="flex relative">
    <div ref="triggerArea">
      <slot name="trigger" :toggle="toggleMenu"></slot>
    </div>

    <Teleport to="body">
      <div
        ref="menuRef"
        :style="floatingStyles"
        class="z-50"
        :class="{ 'pointer-events-none': !isVisible }"
      >
        <TransitionRoot
          :show="isVisible"
          enter="transition ease-out duration-100"
          enter-from="transform opacity-0 scale-95"
          enter-to="transform opacity-100 scale-100"
          leave="transition ease-in duration-75"
          leave-from="transform opacity-100 scale-100"
          leave-to="transform opacity-0 scale-95"
        >
          <div class="w-48 bg-bg-surface border-2 border-border rounded shadow-lg">
            <slot name="content"></slot>
          </div>
        </TransitionRoot>
      </div>
    </Teleport>
  </div>
</template>