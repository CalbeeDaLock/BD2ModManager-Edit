<script setup>
import { ref, computed, watch, useTemplateRef } from "vue"
import { Listbox, ListboxButton, ListboxOptions, ListboxOption } from "@headlessui/vue"
import { ChevronsUpDownIcon, Check } from "lucide-vue-next"
import { flip, offset, shift, useFloating } from "@floating-ui/vue"

const props = defineProps({
    modelValue: {
        type: String,
        default: null
    },
    options: {
        type: Array,
        required: true // format: [{ label: 'Light', value: 'light', disabled: true }]
    },
    placeholder: {
        type: String,
        default: "Select an option"
    },
    disabled: {
        type: Boolean,
        default: false
    },
    multiple: {
        type: Boolean,
        default: false
    }
})

const emit = defineEmits(["update:modelValue"])

const internalValue = ref(props.modelValue)

watch(
    () => props.modelValue,
    (val) => {
        internalValue.value = val
    }
)

const updateValue = (val) => {
    emit("update:modelValue", val)
}

const selectedLabel = computed(() => {
    const found = props.options?.find((o) => o.value === internalValue.value)
    return found ? found.label : props.placeholder
})

const reference = useTemplateRef("reference")
const floating = useTemplateRef("floating")
const { floatingStyles } = useFloating(reference, floating, {
    placement: "bottom-start",
    // strategy: "fixed",
    middleware: [
        offset(4),
        flip(),
        shift({ padding: 4 })
    ]
})
</script>
<template>
    <Listbox v-model="internalValue" @update:model-value="updateValue" :multiple="multiple" :disabled="disabled">
        <div class="relative">
            <ListboxButton ref="reference" class="relative py-1 h-full pl-3 pr-8 w-full cursor-pointer rounded-sm border border-interactive-border bg-interactive-bg text-left text-primary text-sm transition-colors duration-200 focus:outline-none focus:ring-0">
                <span v-if="multiple">
                    <span v-if="Array.isArray(internalValue) && internalValue.length > 0" class="block truncate">
                        {{ internalValue.map(v => {
                            const found = options.find(o => o.value === v)
                            return found ? found.label : v
                        }).join(', ') }}
                    </span>
                    <span v-else class="block truncate text-text-secondary">
                        {{ placeholder }}
                    </span>
                </span>
                <span v-else class="block truncate">
                    {{ selectedLabel }}
                </span>
                <span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
                    <ChevronsUpDownIcon class="h-4 w-4 text-text-primary opacity-60" />
                </span>
            </ListboxButton>

            <transition enter-active-class="transition duration-200 ease-out" enter-from-class="opacity-0 scale-95"
                enter-to-class="opacity-100 scale-100" leave-active-class="transition duration-150 ease-in"
                leave-from-class="opacity-100 scale-100" leave-to-class="opacity-0 scale-95">
                <ListboxOptions ref="floating" :style="floatingStyles"
                    class="w-full z-9999 overflow-hidden rounded-sm border border-interactive-border bg-interactive-bg shadow-lg focus:outline-none">
                    <ListboxOption v-for="option in options" :key="option.value" :value="option.value"
                        v-slot="{ selected }" :disabled="option.disabled">
                        <li
                            class="relative cursor-pointer select-none py-2 pl-10 pr-4 text-primary text-sm hover:bg-interactive-bg-hover">
                            <span :class="[selected ? 'font-semibold' : 'font-medium', 'block truncate']">
                                {{ option.label }}
                            </span>
                            <span v-if="selected" class="absolute inset-y-0 left-0 flex items-center pl-3 text-primary">
                                <Check class="h-4 w-4" />
                            </span>
                        </li>
                    </ListboxOption>
                </ListboxOptions>
            </transition>
        </div>
    </Listbox>
</template>

<style scoped></style>