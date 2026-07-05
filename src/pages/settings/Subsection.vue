<script setup lang=ts>
defineProps<{
    title: string;
}>();
</script>
<template>
    <div class="flex flex-col gap-2">
        <span class="text-md font-bold uppercase">
            {{ title }}
        </span>
        <div class="flex flex-col gap-2 ml-1">
            <slot></slot>
        </div>
    </div>
</template>
<style scoped></style>