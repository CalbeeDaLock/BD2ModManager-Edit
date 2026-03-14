<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  src: string;
  errorSrc?: string;
}>()

const hasError = ref(false)
</script>

<template>
  <img
    v-bind="$attrs"
    
    :src="hasError && props.errorSrc ? props.errorSrc : props.src"
    @error="(e) => {hasError = true; console.log(e)}"
  />
</template>
