<script setup lang="ts">

defineProps<{
    title: string;
    description?: string;
}>();

</script>

<template>
    <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-1">
            <span class="text-lg font-semibold text-primary">
                {{ title }}
            </span>
            <span v-if="description" class="text-secondary">
                {{ description }}
            </span>
        </div>
        <div class="flex flex-col gap-2">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped></style>