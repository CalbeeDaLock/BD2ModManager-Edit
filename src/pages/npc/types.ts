/**
 * A single NPC entry shown on the NPC Characters page, derived from the
 * NPC-typed mods in the mods store (grouped by modType.id).
 */
export interface NpcEntry {
    /** The NPC id, taken from `mod.modType.id` for NPC-typed mods. */
    id: string;
    /** Display name: the resolved character name, or "NPC <id>" fallback. */
    name: string;
    /** Total number of NPC mods for this NPC. */
    modsCount: number;
    /** Number of currently-enabled, error-free NPC mods. */
    enabledCount: number;
    /** True when at least one NPC (Standing) mod is enabled. */
    hasStanding: boolean;
}
