use std::{fs, path::Path};

use crate::mods::BD2ModType;

pub fn extract_character_id(filename: &str, prefix: &str) -> Option<String> {
    if filename.starts_with(prefix) && filename.ends_with(".modfile") {
        let without_prefix = &filename[prefix.len()..];
        let without_suffix = &without_prefix[..without_prefix.len() - 8]; // Remove ".modfile"

        // Handle cases like "123" or "123_456"
        if let Some(underscore_pos) = without_suffix.find('_') {
            without_suffix[..underscore_pos].to_string().into()
        } else {
            without_suffix.to_string().into()
        }
    } else {
        None
    }
}

pub fn get_mod_type(folder_path: &Path) -> Option<BD2ModType> {
    if let Ok(entries) = fs::read_dir(folder_path) {
        let patterns = [
            ("Cutscene", "cutscene_char"),
            ("Standing", "char"),
            ("Dating", "illust_dating"),
            ("Scene", "illust_special"),
            ("Scene", "specialillust"),
            // illust_talk assets belong to NPCs; the full asset id (prefix
            // included, e.g. "illust_talk7", "illust_talk9016") is kept as the
            // NPC id below. This must precede the Scene "illust_talk" line so
            // NPC wins.
            ("NPC", "illust_talk"),
            ("Scene", "illust_talk"),
            ("Scene", "storypack2_2"),
            ("NPC", "npc"),
            ("Minigame", "RhythmHitAnim"),
        ];

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    let filename_lower = filename.to_lowercase();
                    if filename_lower.ends_with(".modfile") {
                        for (mod_type_str, prefix) in &patterns {
                            if *mod_type_str == "Minigame" {
                                if filename_lower.starts_with(&prefix.to_lowercase()) {
                                    return Some(BD2ModType::Minigame { id: None });
                                }
                            } else if let Some(mod_id) =
                                extract_character_id(&filename_lower, prefix)
                            {
                                return match *mod_type_str {
                                    "Standing" => Some(BD2ModType::Standing { id: mod_id }),
                                    "Cutscene" => Some(BD2ModType::Cutscene { id: mod_id }),
                                    "Dating" => Some(BD2ModType::Dating { id: mod_id }),
                                    "Scene" => Some(BD2ModType::Scene { id: mod_id }),
                                    // Keep the prefix on NPC ids so the whole
                                    // asset id shows in the UI (e.g. "npc000001",
                                    // "illust_talk7") instead of just the number.
                                    "NPC" => Some(BD2ModType::NPC {
                                        id: format!("{}{}", prefix, mod_id),
                                    }),
                                    _ => None,
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    None
}
