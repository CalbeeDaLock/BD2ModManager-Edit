<script setup lang="ts">
import { ref } from 'vue';
import { X } from 'lucide-vue-next';
import Modal from '../../../components/common/Modal.vue';
import Button from '../../../components/common/Button.vue';

const visible = ref(false)

const modName = ref('')
const modAuthor = ref('')
const on_save = ref<((newAuthor: string) => void) | undefined>(undefined)

const newModAuthor = ref('')

defineExpose({
    open(payload: {
        modName: string;
        modAuthor: string;
        onSave?: (newAuthor: string) => void;
    }) {
        visible.value = true;
        modName.value = payload.modName;
        modAuthor.value = payload.modAuthor;
        newModAuthor.value = payload.modAuthor;
        on_save.value = payload.onSave;
    }
});



function saveChanges() {
    if (on_save.value) {
        on_save.value(newModAuthor.value);
    }
    visible.value = false;
}

function cancel() {
    visible.value = false;
    modAuthor.value = '';
    newModAuthor.value = '';
    modName.value = '';
}
</script>
<template>
    <Modal v-model:show="visible" @close="visible = false">
        <template #header>
            <div class="flex flex-col gap-0.5">
                <div class="flex justify-between items-center">
                    <h2 class="text-lg font-semibold">{{ $t('modals.changeModAuthor.title') }}</h2>
                    <X class="w-5 h-5 cursor-pointer" @click="cancel" />
                </div>
                <p class="text-sm text-secondary">{{ $t('modals.changeModAuthor.description') }}</p>
            </div>
        </template>
        <template #footer>
            <div class="flex justify-end space-x-2">
                <Button variant="default" @click="cancel">Cancel</Button>
                <Button variant="default" @click="saveChanges">Save Changes</Button>
            </div>
        </template>

        <div class="p-4 flex flex-col gap-4">
            <div>
                <p class="text-md text-primary mb-1 font-bold">{{ $t('modals.changeModAuthor.labels.modName') }}</p>
                <p class="font-medium text-sm truncate text-primary">{{ modName }}</p>
            </div>

            <div>
                <label for="newModAuthor" class="block text-sm font-medium text-primary">{{
                    $t('modals.changeModAuthor.labels.newAuthor') }}</label>
                <input v-model="newModAuthor" id="newModAuthor"
                    :placeholder="$t('modals.changeModAuthor.placeholders.modAuthor')"
                    class="mt-2 p-2 border bg-interactive-bg-hover border-interactive-border rounded w-full" />
            </div>
        </div>
    </Modal>
</template>