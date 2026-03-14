<script setup lang="ts">
import { TransitionRoot, TransitionChild, Dialog, DialogPanel } from '@headlessui/vue'

defineOptions({ inheritAttrs: false });

const show = defineModel('show', {
  type: Boolean,
  required: true,
});

withDefaults(
  defineProps<{
    overlay?: boolean
  }>(),
  {
    overlay: true
  }
);

defineEmits(["close"])
</script>

<template>
  <TransitionRoot
    :show="show"
    as="template"
    enter="duration-300 ease-out"
    enter-from="opacity-0"
    enter-to="opacity-100"
    leave="duration-200 ease-in"
    leave-from="opacity-100"
    leave-to="opacity-0"
  >
    <Dialog as="div" class="relative z-9999" @close="$emit('close')" :close-on-escape="false">
      <TransitionChild v-if="overlay"
        as="template"
        enter="ease-out duration-300"
        enter-from="opacity-0"
        enter-to="opacity-100"
        leave="ease-in duration-200"
        leave-from="opacity-100"
        leave-to="opacity-0"
      >
        <div class="fixed inset-0 bg-black/30" />
      </TransitionChild>

      <div class="fixed inset-0 flex w-screen items-center justify-center">
        <TransitionChild
          as="template"
          enter="ease-out duration-300"
          enter-from="opacity-0 scale-95"
          enter-to="opacity-100 scale-100"
          leave="ease-in duration-200"
          leave-from="opacity-100 scale-100"
          leave-to="opacity-0 scale-95"
        >
          <DialogPanel 
            :class="[
              'flex flex-col mx-4 rounded bg-bg-surface border border-border',
              $attrs.class
            ]"
          >
            <div v-if="$slots.header" class="bg-bg-surface border-b border-border p-4">
              <slot name="header" />
            </div>

            <div class="bg-bg-surface p-0 overflow-y-auto">
              <slot />
            </div>

            <div v-if="$slots.footer" class="bg-bg-surface border-t border-border p-4">
              <slot name="footer" />
            </div>
          </DialogPanel>
        </TransitionChild>
      </div>
    </Dialog>
  </TransitionRoot>
</template>
