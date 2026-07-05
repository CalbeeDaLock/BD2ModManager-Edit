<script setup lang="ts">
import { computed, MaybeRef } from 'vue';


const props = defineProps<{
    label?: MaybeRef<string>
    icon?: any
    labelClass?: string
    iconClass?: string | object | Array<string | object>
    disabled?: boolean
    variant?: "default" | "alt" | "text" | 'unstyled'
    size?: "sm" | "md" | "lg"
    confirmation?: boolean
    confirmationMessage?: string | null
}>(
)

const emit = defineEmits<{
    (event: 'click', e: MouseEvent): void
}>()

const variants = {
    "default": "flex justify-center items-center gap-2 cursor-pointer rounded-md h-full px-2.5 py-2 bg-interactive-bg hover:bg-interactive-bg-hover text-primary font-medium transition-colors active:scale-98 border border-interactive-border",
    "alt": "flex items-center justify-center gap-2 cursor-pointer py-2 px-4 border border-interactive-border rounded-md bg-bg-surface text-primary min-w-[160px] truncate transition-colors duration-150 hover:bg-interactive-bg-hover active:scale-98",
    "text": "flex items-center justify-center rounded-full gap-2 cursor-pointer text-primary font-medium transition-colors duration-150 group group-hover:!text-accent-primary hover:bg-interactive-bg-hover/50 px-2 py-1 rounded-sm active:scale-98",    
    "unstyled": ""
}

async function handleClick(e: MouseEvent) {
    if (!props.disabled) {
        if (props.confirmation) {
            if (await confirm(props.confirmationMessage || "Are you sure?")) {
                emit('click', e)
            }
        } else {
            emit('click', e)
        }
    }
}

const classList = computed(() => {
    let classList = variants[props.variant || 'default']
    if (props.disabled) classList += ' opacity-50 cursor-not-allowed'
    return classList
})

</script>

<template>
    <button type="button" :class="[classList, $attrs.class]" v-bind="$attrs" @click="handleClick">
        <component v-if="icon" :is="icon" :class="[iconClass, 'w-[1.15em] h-[1.15em] scale-125 text-primary']" />
        <span v-if="label" :class="[labelClass, 'truncate text-sm']">{{ label }}</span>
        <slot></slot>
    </button>
</template>
