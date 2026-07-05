<script setup lang="ts">
import { TransitionRoot } from '@headlessui/vue'
import { SearchIcon, X } from 'lucide-vue-next'
import { useAttrs } from 'vue'

defineOptions({
  inheritAttrs: false
})

const searchQuery = defineModel<string>({
  default: ''
})

const props = defineProps<{
  placeholder?: string
  disabled?: boolean
  readonly?: boolean
}>()

const emit = defineEmits<{
  (event: 'update:modelValue', value: string): void
  (event: 'enter'): void
}>()

const attrs = useAttrs()
</script>

<template>
  <div class="relative w-full h-full">
    <input
      v-model="searchQuery"
      v-bind="attrs"
      :placeholder="placeholder"
      :disabled="props.disabled"
      :readonly="props.readonly"
      :tabindex="props.readonly ? -1 : undefined"
      class="w-full h-full pl-7 px-3 rounded-sm text-slate-200 text-sm bg-interactive-bg transition-all border border-interactive-border 
             focus:outline-none focus:ring-2 focus:ring-focus-ring focus:border-accent-primary
             disabled:opacity-50 disabled:cursor-not-allowed read-only:cursor-default"
      type="text"
      @keydown.enter.prevent="emit('enter')"
    />

    <SearchIcon class="absolute left-2 top-1/2 -translate-y-1/2 w-4 h-4 text-secondary" />

    <TransitionRoot
      :show="typeof searchQuery === 'string' && searchQuery.length > 0 && !props.disabled && !props.readonly"
      enter="transition-opacity duration-75"
      enter-from="opacity-0"
      enter-to="opacity-100"
      leave="transition-opacity duration-150"
      leave-from="opacity-100"
      leave-to="opacity-0"
    >
      <X
        class="absolute right-2 top-1/2 -translate-y-1/2 w-4 h-4 text-primary cursor-pointer hover:text-accent-primary-hover"
        @click="searchQuery = ''"
      />
    </TransitionRoot>
  </div>
</template>