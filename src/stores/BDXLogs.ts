import { defineStore } from "pinia";
import { ref } from "vue";

interface LogMessage {
    level: 'success' | 'info' | 'warn' | 'error' | 'debug';
    scope: "All" | "BepInEx" | "BrownDustX" | "Configuration Manager";
    message: string;
    timestamp: string;
}

export const useBdxLogsStore = defineStore('bdxlogs', () => {
    const logs = ref([] as LogMessage[])

    return {
        logs
    }
})