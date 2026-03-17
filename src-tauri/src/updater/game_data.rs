use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tauri::{Emitter, Manager};

const CHARACTERS_URL: &str =
    "https://raw.githubusercontent.com/bruhnn/BD2ModManager-tauri/refs/heads/main/src-tauri/data/characters.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ProgressPayload {
    progress: u8,
    label: String,
}

async fn fetch_latest_characters_data() -> Result<(String, serde_json::Value), String> {
    let json = reqwest::get(CHARACTERS_URL)
        .await
        .map_err(|e| format!("Failed to fetch characters.json: {e}"))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("Failed to parse characters.json: {e}"))?;

    let version = json
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.1")
        .to_string();

    Ok((version, json))
}

fn get_local_characters_version(app_handle: &tauri::AppHandle) -> Option<String> {
    let path = app_handle
        .path()
        .app_data_dir()
        .ok()?
        .join("characters.json");
    let data = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&data).ok()?;
    json.get("version")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

async fn download_missing_assets(
    app_handle: &tauri::AppHandle,
    characters: &serde_json::Value,
    old_characters: &serde_json::Value,
) -> Result<(), String> {
    let client = reqwest::Client::new();

    let characters_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|_| "Failed to get app data dir")?
        .join("characters")
        .join("standing");

    tokio::fs::create_dir_all(&characters_dir)
        .await
        .map_err(|e| format!("Failed creating dir: {e}"))?;

    let old_ids: HashSet<&str> = old_characters["characters"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|c| c["id"].as_str()).collect())
        .unwrap_or_default();

    let new_characters: Vec<&serde_json::Value> = characters["characters"]
        .as_array()
        .ok_or("Invalid characters format")?
        .iter()
        .filter(|c| c["id"].as_str().map_or(false, |id| !old_ids.contains(id)))
        .collect();

    let total = new_characters.len();
    println!("Found {total} new characters to download");

    for (index, character) in new_characters.iter().enumerate() {
        let id = character["id"].as_str().ok_or("Character missing id")?;
        let filename = format!("{id}.png");
        let file_path = characters_dir.join(&filename);

        let url = format!(
            "https://raw.githubusercontent.com/bruhnn/BD2ModManager-tauri/main/public/characters/standing/{filename}"
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Download failed for {filename}: {e}"))?;

        if !response.status().is_success() {
            return Err(format!(
                "Failed to download {filename}: HTTP {}",
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed reading bytes for {filename}: {e}"))?;

        tokio::fs::write(&file_path, &bytes)
            .await
            .map_err(|e| format!("Failed saving {filename}: {e}"))?;

        let progress = ((index + 1) as f32 / total as f32 * 100.0) as u8;
        app_handle
            .emit(
                "update:gameData:downloadingAssets",
                ProgressPayload {
                    label: filename,
                    progress,
                },
            )
            .ok();
    }

    Ok(())
}

pub async fn update_characters(app_handle: tauri::AppHandle) -> Result<(), String> {
    app_handle.emit("update:gameData:checking", ()).ok();

    let (latest_version_str, latest_data) = fetch_latest_characters_data()
        .await
        .map_err(|e| format!("Failed to fetch latest characters data: {e}"))?;

    let latest_version =
        Version::parse(&latest_version_str).map_err(|e| format!("Invalid latest version: {e}"))?;

    println!("Latest characters version: {latest_version}");

    if let Some(local_str) = get_local_characters_version(&app_handle) {
        let local_version =
            Version::parse(&local_str).map_err(|e| format!("Invalid local version: {e}"))?;

        println!("Local characters version: {local_version}");

        if local_version >= latest_version {
            println!("Characters data is up to date (version {local_version})");
            return Ok(());
        }
    }

    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|_| "Failed to get app data directory")?;
    let characters_path = app_dir.join("characters.json");

    let old_characters: serde_json::Value = std::fs::read_to_string(&characters_path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| serde_json::json!({"characters": []}));

    std::fs::write(
        &characters_path,
        serde_json::to_string_pretty(&latest_data).unwrap(),
    )
    .map_err(|e| format!("Failed to save characters.json: {e}"))?;

    download_missing_assets(&app_handle, &latest_data, &old_characters).await?;

    app_handle.emit("update:gameData:updated", ()).ok();
    Ok(())
}
