<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { useHeader } from '../composables/useHeader';
import Button from './common/Button.vue';

const {
    title,
    subtitle,
    buttons,
} = useHeader()

const route = useRoute()
const displayTitle = computed(() => title.value || (route.meta?.title as string) || '')

</script>

<template>
    <div class="px-2 flex select-none overflow-hidden min-h-10 h-12 shrink-0 sticky justify-between transition-[max-height] duration-300 ease-out">
        <!-- :style="{ maxHeight: showSyncBar ? '52px' : '40px' }"  -->
        <div class="flex min-w-0 items-center gap-2 px-2 py-1 justify-center">
            <span class="font-mono truncate font-bold text-lg select-none flex">
                {{ displayTitle }}
            </span>

            <span v-if="subtitle"
                class="hidden md:block truncate flex-1 font-medium text-xs text-text-secondary select-none">
                {{ subtitle }}
            </span>
        </div>
        <div class="flex items-center shrink-0 gap-1 md:gap-2" v-if="buttons.length > 0">
            <template v-for="(button, index) in buttons">
                <component v-if="button.render" :is="button.render" :key="index" />
                <Button v-else @click="button.action" :icon="button.icon" :label="button.label"
                    :variant="button.variant || 'text'" class="truncate"
                    :label-class="button.labelClass"></Button>
            </template>
        </div>
    </div>
</template>

<style scoped></style>