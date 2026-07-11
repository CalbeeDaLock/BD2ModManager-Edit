// Manual icon overrides for specific NPCs. Most NPC mods have no standing
// asset of their own, so the grid/modal fall back to a plain id band. These
// ids have a known portrait we can show instead. Keyed by the full NPC id
// (including the "npc" prefix), matching mod.modType.id.
const NPC_ICON_OVERRIDES: Record<string, string> = {
    'npc000001': '/npc/standing/000001.png',
    'npc000005': '/npc/standing/000005.png',
    'npc0033': '/characters/standing/003301.png',
    'npc300302': '/characters/standing/003101.png',
    'npc400034': '/npc/standing/400034.png',
};

/** Returns the icon URL for an NPC id, or undefined when none is defined. */
export function getNpcIcon(id: string): string | undefined {
    return NPC_ICON_OVERRIDES[id];
}
