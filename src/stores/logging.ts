import { defineStore } from "pinia";
import { readonly, ref } from "vue";

const LOG_TO_CONSOLE = true;

enum LogLevel {
    Success = 'Success',
    Info = 'Info',
    Warning = 'Warning',
    Error = 'Error',
    Debug = 'Debug'
}

interface LogMessage {
    level: LogLevel;
    message: string;
    timestamp: Date;
}

export const useLoggingStore = defineStore("logging", () => {
    const logs = ref<LogMessage[]>([]);

    function addLog(level: LogLevel, message: string) {
        const timestamp = new Date();
        logs.value.push({ level, message, timestamp });

        if (LOG_TO_CONSOLE) {
            const levelStr = LogLevel[level].toUpperCase();
            console.log(`[${timestamp}] [${levelStr}] ${message}`);
        }
    }

    function logSuccess(...args: any[]) {
        addLog(LogLevel.Success, args.join(" "));
    }

    function logInfo(...args: any[]) {
        addLog(LogLevel.Info, args.join(" "));
    }

    function logWarning(...args: any[]) {
        addLog(LogLevel.Warning, args.join(" "));
    }

    function logError(...args: any[]) {
        addLog(LogLevel.Error, args.join(" "));
    }

    function logDebug(...args: any[]) {
        addLog(LogLevel.Debug, args.join(" "));
    }

    function clearLogs() {
        logs.value = [];
    }

    return {
        logs: readonly(logs),
        logSuccess,
        logInfo,
        logWarning,
        logError,
        logDebug,
        clearLogs,
    };
});
