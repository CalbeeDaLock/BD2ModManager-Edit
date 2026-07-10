import type { Language } from '../../stores/settings';

/**
 * A single NPC entry shown on the NPC Characters page, derived by merging the
 * static NPC catalog (npc.json, loaded via charactersStore.npcDefinitions) with
 * live mod data from the mods store.
 */
export interface NpcEntry {
    /** The NPC id, taken from `mod.modType.id` for NPC-typed mods. */
    id: string;
    /** Display name resolved through the current language / forceEnglishNames preference. */
    name: string;
    /** Full image path from npc.json (e.g. "/npc/illust_npc000001.png"). Undefined
     *  for NPCs discovered only from mods (no static definition). */
    image?: string;
    /** Multilang name map from npc.json. Undefined for mod-only NPCs. */
    character_name?: { [Lang in Language]: string };
    /** Total number of NPC mods for this NPC. */
    modsCount: number;
    /** Number of currently-enabled, error-free NPC mods. */
    enabledCount: number;
    /** True when at least one NPC (Standing) mod is enabled. */
    hasStanding: boolean;
}
