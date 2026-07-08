// Manual icon overrides for specific NPCs. Most NPC mods have no standing
// asset of their own, so the grid/modal fall back to a plain id band. These
// ids have a known portrait we can show instead. Keyed by the numeric NPC id
// (no "npc" prefix), matching mod.modType.id.
const NPC_ICON_OVERRIDES: Record<string, string> = {
    '000001': '/npc/standing/illust_npc000001.png',
    '000005': '/npc/standing/illust_0000050101_1131.png',
    '0033': '/characters/standing/003301.png',
    '300302': '/characters/standing/003101.png',
    '400034': '/npc/standing/illust-greta-npc.png',
};

/** Returns the icon URL for an NPC id, or undefined when none is defined. */
export function getNpcIcon(id: string): string | undefined {
    return NPC_ICON_OVERRIDES[id];
}
