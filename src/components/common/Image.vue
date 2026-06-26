<script setup lang="ts">
import { ref, watch } from 'vue';

defineOptions({ inheritAttrs: false })

const props = defineProps<{
  src: string;
  errorSrc?: string;
  onError?: (event: Event) => void;
  fallbackSources?: string[];
  skeleton?: boolean
  imageClass?: string
}>()

const currentSrc = ref(props.src)
const fallbackIndex = ref(0)
const loaded = ref(false)

watch(() => props.src, (newSrc) => {
  currentSrc.value = newSrc
  fallbackIndex.value = 0
  loaded.value = false
})

function onError(event: Event) {
  props.onError?.(event);

  const fallbacks = props.fallbackSources ?? []
  if (fallbackIndex.value < fallbacks.length) {
    currentSrc.value = fallbacks[fallbackIndex.value]
    fallbackIndex.value++
  } else if (props.errorSrc) {
    currentSrc.value = props.errorSrc
  }
}

function onLoaded() {
  loaded.value = true
}
</script>

<template>
  <template v-if="skeleton">
    <div v-bind="$attrs" class="relative overflow-hidden">
      <div
        v-if="!loaded"
        class="absolute inset-0 skeleton animate-sweep"
      />
      <img
        :src="currentSrc"
        class="w-full h-full object-cover object-top"
        :class="[{ 'invisible': !loaded }, imageClass]"
        @load="onLoaded"
        @error="onError"
      />
    </div>
  </template>
  <img
    v-else
    v-bind="$attrs"
    :src="currentSrc"
    :class="imageClass"
    @load="onLoaded"
    @error="onError"
  />
</template>