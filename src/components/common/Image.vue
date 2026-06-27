<script setup lang="ts">
import { computed, ref, watch } from 'vue';

defineOptions({ inheritAttrs: false })

const props = defineProps<{
  src: string;
  errorSrc?: string;
  onError?: (event: Event) => void;
  fallbackSources?: string[];
  skeleton?: boolean
  imageClass?: string
  errorClass?: string
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

const isErrorSrc = computed(() => {
  return currentSrc.value == props.errorSrc
})
</script>

<template>
  <div v-if="skeleton" v-bind="$attrs" class="relative overflow-hidden">
    <div v-if="!loaded" class="absolute inset-0 skeleton animate-sweep" />
    <img
      v-if="!isErrorSrc"
      :src="currentSrc"
      class="w-full h-full object-cover object-top"
      :class="[{ 'invisible': !loaded }, imageClass]"
      @load="onLoaded"
      @error="onError"
    />
    <template v-else>
      <div
        class="w-full h-full"
        :class="[{ 'invisible': !loaded }, imageClass, errorClass]"
        :style="{
          maskImage: `url(${currentSrc})`,
          maskRepeat: 'no-repeat',
          maskPosition: 'center',
          maskSize: 'contain',
        }"
      />
      <img :src="currentSrc" class="hidden" @load="onLoaded" @error="onError" />
    </template>
  </div>

  <img v-else v-bind="$attrs" :src="currentSrc" :class="imageClass" @load="onLoaded" @error="onError" />
</template>