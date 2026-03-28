<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  src: string;
  errorSrc?: string;
  onError?: (event: Event) => void;
  fallbackSources?: string[];
}>()

const currentSrc = ref(props.src)
const hasError = ref(false)

function onError(event: Event) {
  hasError.value = true;
  props.onError?.(event);
  if (props.fallbackSources && props.fallbackSources.length > 0) {
    const nextSrc = props.fallbackSources.shift()!;
    currentSrc.value = nextSrc;
    hasError.value = false;
  }
}
</script>

<template>
  <img
    v-bind="$attrs"
  
    :src="hasError && props.errorSrc ? props.errorSrc : currentSrc"
    @error="onError"
  />
</template>
