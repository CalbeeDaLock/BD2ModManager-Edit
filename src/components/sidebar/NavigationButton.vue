<script setup lang="ts">
import { computed } from 'vue';
import router from '../../router';

const props = defineProps<{
    icon: any
    label: string
    value: string
}>()

defineEmits(["click"])

const active = computed(() => {
    return router.currentRoute.value.name === props.value
})
</script>
<template>
    <RouterLink :to="value" :class="{ 'bg-accent-primary': active, 'text-secondary': !active, 'text-white/90': active }" 
        class="
        w-full px-3 py-2 gap-3 rounded-full 
        cursor-pointer capitalize
        transition-colors duration-200 transform active:scale-98
        flex items-center 
        text-sm font-medium
        hover:bg-interactive-bg-hover hover:text-white/90">
        <component :is="icon" class="w-[1.75em] h-[1.75em] text-primary" />
        {{ label }}
    </RouterLink>
</template>
<style scoped></style>