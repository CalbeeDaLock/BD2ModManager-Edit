/**
 * A single Wallpaper entry shown on the Wallpapers management page, derived
 * from the Wallpaper-typed mods in the mods store (grouped by modType.id).
 */
export interface WallpaperEntry {
    /** The wallpaper id, taken from `mod.modType.id` (the .modfile filename). */
    id: string;
    /** Custom nickname for this wallpaper (empty string when unset). */
    nickname: string;
    /** Total number of wallpaper mods for this id. */
    modsCount: number;
    /** Number of currently-enabled, error-free wallpaper mods. */
    enabledCount: number;
}
