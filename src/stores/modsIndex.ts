import { defineStore } from "pinia";
import { computed, readonly, ref } from "vue";
import { useLoggingStore } from "./logging";

interface ModIndex {
    author: string
    type: string
    is_paid: boolean
}

interface Author {
    name: string;
    links: {
        patreon: string | null
        "ko-fi": string | null
        discord: string | null
        afdian: string | null
    };
}

interface ModsIndexData {
    version: string,
    last_updated: string,
    authors: Record<string, Author>
    characters: Record<string, { mods: ModIndex[] }>
}

export const useModsIndexStore = defineStore('modsIndex', () => {
    const modsIndex = ref<ModsIndexData>({} as ModsIndexData);
    const isUpdating = ref(false);

    const loggingStore = useLoggingStore()

    async function fetchModsIndex() {
        isUpdating.value = true;
        try {
            const res = await fetch('https://raw.githubusercontent.com/bruhnn/BD2ModManager/main/src/data/mods_index.json', { cache: 'no-cache' });
            if (!res.ok) throw new Error('Failed to fetch mods index');
            modsIndex.value = await res.json();
            loggingStore.logInfo('Mods index updated from GitHub');
        } catch (e) {
            loggingStore.logError('Could not fetch latest mods index.', e);
        } finally {
            isUpdating.value = false;
        }
    }

    function getAuthor(authorId: string): Author | null {
        return modsIndex.value.authors[authorId] ?? null;
    }

    function _getMods(characterId: string, modType: string): ModIndex[] {
        if (!modsIndex.value.characters || !(characterId in modsIndex.value.characters)) return [];
        const character = modsIndex.value.characters[characterId];
        if (!character) return [];
        return character.mods.filter(mod => mod.type === modType);
    }

    function getMods(characterId: string, modType: string) {
        return _getMods(characterId, modType).map(mod => ({
            ...mod,
            authorData: getAuthor(mod.author),
        }));
    }

    return { 
        modsIndex, 
        latestUpdate: readonly(computed(() => modsIndex.value.last_updated)),
        isUpdating, 
        fetchModsIndex, 
        getAuthor, 
        getMods };
});