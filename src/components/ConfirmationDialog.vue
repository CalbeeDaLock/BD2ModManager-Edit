<script setup lang="ts">
import { X } from 'lucide-vue-next';
import { confirmState } from '../plugins/ConfirmService';
import Checkbox from 'primevue/checkbox';
import { ref, watch } from 'vue';
import { TransitionChild, TransitionRoot } from '@headlessui/vue';

const state = confirmState;
const rememberChoice = ref(false);

watch(() => state.visible, (visible) => {
  if (visible) {
    rememberChoice.value = false;
  }
});

const accept = () => {
  state.resolve?.(true, rememberChoice.value);
  state.visible = false;
};

const reject = () => {
  state.resolve?.(false, rememberChoice.value);
  state.visible = false;
};
</script>

<template>
  <teleport to="body">
    <TransitionRoot appear :show="state.visible" as="template">
      <div class="relative text-primary  z-50">
        <TransitionChild as="template" enter="duration-150 ease-out" enter-from="opacity-0" enter-to="opacity-100"
          leave="duration-150 ease-in" leave-from="opacity-100" leave-to="opacity-0">
          <div class="fixed inset-0 bg-black/25 backdrop-blur-xs" />
        </TransitionChild>
        
        <div class="inset-0 fixed flex justify-center items-center ">
          <TransitionChild enter="ease-out duration-150" enter-from="opacity-0 scale-95" enter-to="opacity-100 scale-100"
            leave="ease-in duration-150" leave-from="opacity-100 scale-100" leave-to="opacity-0 scale-95" as="template">
  
            <div class="bg-bg-surface p-2.5 rounded border border-border shadow-lg min-w-80 max-w-120">
              <div class="flex justify-between items-center" data-tauri-drag-region>
                <div v-if="state.title" class="flex items-center gap-2">
                  <component v-if="state.icon" :is="state.icon" class="inline-block w-[1.5em] h-[1.5em]" />
                  <h1 class="font-medium text-lg">
                    {{ state.title }}
                  </h1>
                </div>
                <span v-else></span>
                <span>
                  <X @click="reject"
                    class="text-primary hover:text-accent-primary! w-[1.5em] h-[1.5em] transition-colors cursor-pointer" />
                </span>
              </div>
              <p class="py-4 text-md text-primary">{{ state.message }}</p>
  
              <div class="flex justify-between items-center">
                <div v-if="state.showRememberChoice">
                  <div class="flex gap-1.5 items-center">
                    <Checkbox v-model="rememberChoice" input-id="remember-choice" binary />
                    <label for="remember-choice" class="cursor-pointer select-none">Don't show again</label>
                  </div>
                </div>
                <div v-else></div>
                <div class="flex justify-end gap-2">
                  <button @click="reject"
                    class="px-4 py-2 rounded bg-interactive-bg hover:bg-interactive-bg-hover! cursor-pointer">{{
                      state.rejectButton?.label }}</button>
                  <button @click="accept"
                    class="px-4 py-2 rounded bg-accent-primary hover:bg-accent-primary-hover! cursor-pointer">{{
                      state.acceptButton?.label }}</button>
                </div>
              </div>
            </div>
  
          </TransitionChild>

        </div>
      </div>
    </TransitionRoot>
  </teleport>
</template>