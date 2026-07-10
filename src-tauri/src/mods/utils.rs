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

/// Returns true when a `.modfile` describes a wallpaper mod. Wallpaper mods
/// have inconsistent filename prefixes (`pack...`, `P..._EventPackGUI`,
/// `bg_home_wallpaper_...`), so prefix matching is unreliable. Instead we read
/// the modfile's JSON content and look for a modify entry with
/// `"original": "bg"`, which all analyzed wallpaper modfiles share.
fn is_wallpaper_modfile(path: &Path) -> bool {
    let Ok(content) = fs::read_to_string(path) else {
        return false;
    };

    match serde_json::from_str::<serde_json::Value>(&content) {
        Ok(json) => json
            .get("modify")
            .and_then(|m| m.as_array())
            .map(|entries| {
                entries.iter().any(|entry| {
                    entry.get("original").and_then(|v| v.as_str()) == Some("bg")
                })
            })
            .unwrap_or(false),
        Err(_) => false,
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
            ("Scene", "illust_talk"),
            ("Scene", "storypack2_2"),
            ("NPC", "npc"),
            ("Minigame", "RhythmHitAnim"),
        ];

        // Collect modfile paths so we can run the prefix checks first and fall
        // back to a content-based wallpaper check afterwards.
        let mut modfiles: Vec<(String, std::path::PathBuf)> = Vec::new();

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
                                    "NPC" => Some(BD2ModType::NPC { id: mod_id }),
                                    _ => None,
                                };
                            }
                        }

                        modfiles.push((filename.to_string(), path.clone()));
                    }
                }
            }
        }

        // No prefix matched: check whether any modfile is a wallpaper. Use the
        // full modfile filename (minus the `.modfile` extension) as a stable id.
        for (filename, path) in &modfiles {
            if is_wallpaper_modfile(path) {
                // Strip the `.modfile` extension case-insensitively for a stable id.
                let id = if filename.len() >= 8
                    && filename[filename.len() - 8..].eq_ignore_ascii_case(".modfile")
                {
                    filename[..filename.len() - 8].to_string()
                } else {
                    filename.to_string()
                };
                return Some(BD2ModType::Wallpaper { id });
            }
        }
    }

    None
}
