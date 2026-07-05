<script setup lang="ts">
defineProps({
    title: String
})
</script>

<template>
    <div>
        <div class="p-2 px-4 flex gap-3 flex-col pt-2">
            <span class="font-bold text-xs text-primary">
                {{ title }}
            </span>
            <div class="flex flex-col gap-1">
                <slot></slot>
            </div>
        </div>
    </div>
</template>
<style></style>