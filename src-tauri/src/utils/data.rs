use std::fs;
use tauri::Manager;

const CHARACTERS: &str = include_str!("../../data/characters.json");
const NPC: &str = include_str!("../../data/npc.json");

pub fn move_data_to_appdata(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let app_data = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    fs::create_dir_all(&app_data).map_err(|e| e.to_string())?;

    seed_versioned_file(&app_data.join("characters.json"), CHARACTERS, "characters.json")?;
    seed_versioned_file(&app_data.join("npc.json"), NPC, "npc.json")?;


    Ok(())
}

/// Seeds `bundled` into `path`, or overwrites it when the bundled version is
/// newer. Both the bundled string and the existing file must carry a top-level
/// semver `"version"` string.
fn seed_versioned_file(path: &std::path::Path, bundled: &str, label: &str) -> Result<(), String> {
    if !path.exists() {
        println!("Seeding {label} to appdata");
        fs::write(path, bundled).map_err(|e| e.to_string())?;
        return Ok(());
    }

    let bundled_version = semver::Version::parse(
        serde_json::from_str::<serde_json::Value>(bundled)
            .map_err(|e| e.to_string())?["version"]
            .as_str()
            .ok_or_else(|| format!("Missing 'version' in bundled {label}"))?,
    )
    .map_err(|e| e.to_string())?;

    let existing_version = semver::Version::parse(
        serde_json::from_str::<serde_json::Value>(
            &fs::read_to_string(path).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?["version"]
            .as_str()
            .ok_or_else(|| format!("Missing 'version' in existing {label}"))?,
    )
    .map_err(|e| e.to_string())?;

    println!("{label} bundled v{bundled_version} vs appdata v{existing_version}");

    if bundled_version > existing_version {
        println!("Updating {label} from bundled (newer version)");
        fs::write(path, bundled).map_err(|e| e.to_string())?;
    }

    Ok(())
}

