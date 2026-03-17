<script setup lang="ts">

defineProps<{
    title: string;
}>();

</script>

<template>
    <div class="flex flex-col gap-4">
        <span class="text-xl font-bold text-primary">
            {{ title }}
        </span>
        <div class="flex flex-col gap-2">
            <slot></slot>
        </div>
    </div>
</template>

<style scoped></style>