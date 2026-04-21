<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  src: string;
  errorSrc?: string;
  onError?: (event: Event) => void;
  fallbackSources?: string[];
}>()

const currentSrc = ref(props.src)
const fallbackIndex = ref(0)

watch(() => props.src, (newSrc) => {
  currentSrc.value = newSrc
  fallbackIndex.value = 0
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
</script>

<template>
  <img
    v-bind="$attrs"
    :src="currentSrc"
    @error="onError"
  />
</template>