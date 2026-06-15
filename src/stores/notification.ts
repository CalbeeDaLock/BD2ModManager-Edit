import { defineStore } from "pinia";
import { readonly, ref } from "vue";

export interface Notification {
    id: number,
    severity: "info" | "success" | "error" | "warn" ,
    title?: string,
    message?: string,
    duration?: number,
    closable?: boolean,
    showProgress?: boolean
}

export const useNotificationStore = defineStore("notification", () => {
    const notifications = ref<Notification[]>([]);

    function add(notification: Omit<Notification, "id">) {
        const id = Date.now()
        notifications.value.push({ ...notification, id })
    }

    function remove(id: number) {
        notifications.value = notifications.value.filter(n => n.id !== id);
    }

    return {
        notifications: readonly(notifications),
        add,
        remove
    }
})