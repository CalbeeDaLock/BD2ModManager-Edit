<script setup lang="ts">
import { computed, ref } from 'vue';
import { X } from 'lucide-vue-next';
import Modal from '../../../components/common/Modal.vue';
import Button from '../../../components/common/Button.vue';

const visible = ref(false)

const modName = ref('')
const newModName = ref('')
const on_save = ref<((newName: string) => void) | undefined>(undefined)

defineExpose({
    open(payload: {
        modName: string;
        onSave?: (newName: string) => void;
    }) {
        visible.value = true;
        modName.value = payload.modName;
        newModName.value = payload.modName;
        on_save.value = payload.onSave;
    }
});



function saveChanges() {
    if (on_save.value) {
        on_save.value(newModName.value);
    }
    visible.value = false;
}

function cancel() {
    visible.value = false;
    newModName.value = '';
    modName.value = '';
}

const isNameValid = computed(() => {
    // check for invalid characters in newModName (as folders in windows cannot have these characters in their name)
    const invalidChars = /[<>:"\/\\|?*]/;
    return newModName.value.trim().length > 0 && !invalidChars.test(newModName.value);
})
</script>
<template>
    <Modal v-model:show="visible" @close="visible = false">
        <template #header>
            <div class="flex flex-col gap-0.5">
                <div class="flex justify-between items-center">
                    <h2 class="text-lg font-semibold">{{ $t('modals.changeModName.title') }}</h2>
                    <X class="w-5 h-5 cursor-pointer" @click="cancel" />
                </div>
                <p class="text-sm text-secondary">{{ $t('modals.changeModName.description') }}</p>
            </div>
        </template>
        <template #footer>
            <div class="flex justify-end space-x-2">
                <Button variant="default" @click="cancel">Cancel</Button>
                <Button variant="default" @click="saveChanges" :disabled="!isNameValid">Save Changes</Button>
            </div>
        </template>

        <div class="p-4 flex flex-col gap-4">
            <div>
                <p class="text-md text-primary mb-1 font-bold">{{ $t('modals.changeModName.labels.modName') }}</p>
                <!-- <p class="font-medium text-sm truncate text-primary">{{ modName }}</p> -->
                <input type="text" v-model="newModName" class="w-full mt-1 p-2 border rounded" />
            </div>
        </div>
    </Modal>
</template>