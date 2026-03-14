use std::fs;
use tauri::{path::BaseDirectory, Manager};

pub fn move_data_to_appdata(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let app_data = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let chars_path = app_data.join("characters.json");

    let bundled = app_handle
        .path()
        .resolve("data/characters.json", BaseDirectory::Resource)
        .map_err(|e| e.to_string())?;

    if !chars_path.exists() {
        println!("Seeding characters.json from: {}", bundled.display());
        fs::copy(&bundled, &chars_path).map_err(|e| e.to_string())?;
    } else {
        let bundled_version = semver::Version::parse(
            serde_json::from_str::<serde_json::Value>(
                &fs::read_to_string(&bundled).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?["version"]
                .as_str()
                .ok_or("Missing 'version' in bundled characters.json")?,
        )
        .map_err(|e| e.to_string())?;

        let existing_version = semver::Version::parse(
            serde_json::from_str::<serde_json::Value>(
                &fs::read_to_string(&chars_path).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?["version"]
                .as_str()
                .ok_or("Missing 'version' in existing characters.json")?,
        )
        .map_err(|e| e.to_string())?;

        println!(
            "characters.json bundled v{} vs appdata v{}",
            bundled_version, existing_version
        );

        if bundled_version > existing_version {
            println!("Updating characters.json from bundled (newer version)");
            fs::copy(&bundled, &chars_path).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
