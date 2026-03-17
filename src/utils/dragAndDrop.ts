import { listen } from "@tauri-apps/api/event";
import { onActivated, onDeactivated } from "vue";

export function dragAndDrop(callback: CallableFunction) {
    let unlistenFns: Array<() => void> = []

    onActivated(async () => {
        const unlistenDragDrop = await listen<{
            paths: string[]
        }>("tauri://drag-drop", async (event) => {
            const paths = event.payload?.paths as string[]
            callback(paths)
        })

        unlistenFns = [unlistenDragDrop]
    })

    onDeactivated(() => {
        unlistenFns.forEach((unlisten) => unlisten())
        unlistenFns = []

        console.log("Drag and drop listeners removed")
    })
}