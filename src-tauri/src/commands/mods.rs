use std::{fs, path::PathBuf};

use bd2modmanager_lib::{
    mods::{discover::discover_staging_mods, sync::{SyncError, SyncMethod}, BD2Mod},
    utils::{files::{copy_dir_all, is_archive, mod_descriptor_key, mod_texture_signature, texture_signature_overlap}, path::{get_mod_preview_path, get_staging_dir}},
};
use log::warn;
use serde::Serialize;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

use crate::AppState;

#[tauri::command]
pub fn discover_mods(app_handle: tauri::AppHandle, state: tauri::State<AppState>) {
    let config = state.config.lock().unwrap();
    let mut mod_manager = state.mod_manager.lock().unwrap();
    mod_manager.discover_mods(
        &app_handle,
        &get_staging_dir(&config),
        config.search_mods_recursively,
    );
}

#[tauri::command]
pub fn get_mods(state: tauri::State<AppState>) -> Vec<BD2Mod> {
    let mod_manager = state.mod_manager.lock().unwrap();
    let mut mods: Vec<BD2Mod> = mod_manager.cached_mods.values().cloned().collect();

    mods.sort_by(|a, b| a.name.cmp(&b.name));
    mods
}

#[tauri::command]
pub fn enable_mods(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    mod_names: Vec<String>,
) {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    mod_manager.enable_mods(&app_handle, mod_names);
}

#[tauri::command]
pub fn disable_mods(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    mod_names: Vec<String>,
) {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    mod_manager.disable_mods(&app_handle, mod_names);
}

#[tauri::command]
pub fn enable_mods_in_profile(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    profile_id: String,
    mod_names: Vec<String>,
) -> Result<(), bd2modmanager_lib::profiles::types::ProfileError> {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    mod_manager.enable_mods_in_profile(&app_handle, &profile_id, mod_names)
}

/// Replace a profile's entire enabled-mods list. Passing an empty list clears
/// it. Used by the Profiles tab for manual editing and the "clear" action.
#[tauri::command]
pub fn set_profile_enabled_mods(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    profile_id: String,
    mod_names: Vec<String>,
) -> Result<(), bd2modmanager_lib::profiles::types::ProfileError> {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    mod_manager.set_profile_enabled_mods(&app_handle, &profile_id, mod_names)
}

// Check if the folder is a texture mod
fn is_texture_mod(path: &PathBuf) -> bool {
    if !path.join("textures").is_dir() {
        return false;
    }

    let has_skel = fs::read_dir(path)
        .map(|entries| {
            entries.filter_map(Result::ok).any(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|e| e == "skel")
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);

    let has_atlas = fs::read_dir(path)
        .map(|entries| {
            entries.filter_map(Result::ok).any(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|e| e == "atlas")
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);

    !has_skel && !has_atlas
}

fn preview_image(app_handle: AppHandle, path: &PathBuf) -> tauri::Result<()> {
    let dir = path.join("textures");
    if !dir.exists() {
        return Ok(());
    }

    let mut img_path: Option<PathBuf> = None;
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map(|e| e == "png").unwrap_or(false) {
            img_path = Some(path);
            break;
        }
    }

    if let Some(img) = img_path {
        let img_str = img.to_string_lossy().to_string();
        app_handle.opener().open_path(img_str, None::<&str>).ok();
    }

    Ok(())
}

// Tauri command
#[tauri::command]
pub fn preview_mod(app_handle: AppHandle, path: String) -> tauri::Result<()> {
    let path_buf = PathBuf::from(&path);

    if is_texture_mod(&path_buf) {
        return preview_image(app_handle, &path_buf);
    }

    if let Some(mod_preview_exe) = get_mod_preview_path(&app_handle) {
        std::process::Command::new(mod_preview_exe)
            .arg(path)
            .spawn()
            .ok();
    }

    Ok(())
}

#[tauri::command]
pub fn install_mod_from_zip(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    path: String,
) -> Result<String, bd2modmanager_lib::mods::install::ModInstallError> {
    let config = state.config.lock().unwrap();
    let staging_dir = get_staging_dir(&config);
    let mut mod_manager = state.mod_manager.lock().unwrap();
    mod_manager.install_mod(&app_handle, PathBuf::from(path), &staging_dir)
}

#[tauri::command]
pub fn install_mod_from_folder(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    path: String,
) -> Result<String, bd2modmanager_lib::mods::install::ModInstallError> {
    let config = state.config.lock().unwrap();
    let staging_dir = get_staging_dir(&config);
    let mut mod_manager = state.mod_manager.lock().unwrap();
    mod_manager.install_mod(&app_handle, PathBuf::from(path), &staging_dir)
}

#[derive(Serialize, Clone, Debug)]
pub enum SyncCommandError {
    GameDirectoryNotSet,
    SyncMethodInvalid(String),
    SyncFailed(SyncError), 
    UnknownError(String),
}

impl From<SyncError> for SyncCommandError {
    fn from(e: SyncError) -> Self {
        SyncCommandError::SyncFailed(e)
    }
}

#[tauri::command]
pub async fn sync_mods(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), SyncCommandError> {
    let app_handle = app_handle.clone();
    let config_handle = state.config.clone();
    let mod_manager_handle = state.mod_manager.clone();

    let result = tauri::async_runtime::spawn_blocking(move || {
        let config = config_handle.lock().unwrap();
        let mut mod_manager = mod_manager_handle.lock().unwrap();

        let sync_method = match config.sync_method.as_str() {
            "copy" => SyncMethod::Copy,
            "hardlink" => SyncMethod::Hardlink,
            "symlink" => SyncMethod::Symlink,
            other => return Err(SyncCommandError::SyncMethodInvalid(other.to_string())),
        };

        let game_dir = match &config.game_directory {
            Some(dir) => dir.clone(),
            None => return Err(SyncCommandError::GameDirectoryNotSet),
        };

        mod_manager
            .sync_mods(&app_handle, &PathBuf::from(game_dir), sync_method)
            .map_err(SyncCommandError::from)
    })
    .await;

    match result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Sync task panicked: {:?}", e);
            Err(SyncCommandError::UnknownError(format!("{:?}", e)))
        }
    }
}

#[tauri::command]
pub async fn unsync_mods(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<(), SyncCommandError> {
    let app_handle_clone = app_handle.clone();
    let mod_manager_handle = state.mod_manager.clone();
    let config_handle = state.config.clone();

    let game_dir = {
        let config = config_handle.lock().unwrap();
        match &config.game_directory {
            Some(dir) => dir.clone(),
            None => return Err(SyncCommandError::GameDirectoryNotSet),
        }
    };

    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut mod_manager = mod_manager_handle.lock().unwrap();
        mod_manager.unsync_mods(&app_handle_clone, &PathBuf::from(game_dir)).map_err(SyncCommandError::from)
    })
    .await;

    match result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Unsync task panicked: {:?}", e);
            Err(SyncCommandError::UnknownError(format!("{:?}", e)))
        }
    }
}

#[tauri::command]
pub fn is_sync_needed(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<bool, SyncCommandError> {
    let mod_manager = state.mod_manager.lock().unwrap();
    let config = state.config.lock().unwrap();

    let game_dir = match &config.game_directory {
        Some(dir) => dir.clone(),
        None => return Err(SyncCommandError::GameDirectoryNotSet),
    };

    Ok(mod_manager.is_sync_needed(&app_handle, &PathBuf::from(game_dir)))
}

#[tauri::command]
pub async fn delete_mods(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    mod_names: Vec<String>,
) -> Result<(), String> {
    let mod_manager_handle = state.mod_manager.clone();
    let app_handle_clone = app_handle.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut mod_manager = mod_manager_handle.lock().unwrap();
        let _ = mod_manager.delete_mods(&app_handle_clone, mod_names);
        Ok(())
    })
    .await;

    match result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Delete mods task panicked: {:?}", e);
            Err(format!("Delete mods task panicked: {:?}", e))
        }
    }
}

#[tauri::command]
pub async fn rename_mod(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    let mod_manager_handle = state.mod_manager.clone();
    let app_handle_clone = app_handle.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        let mut mod_manager = mod_manager_handle.lock().unwrap();
        let _ = mod_manager.rename_mod(&app_handle_clone, old_name, new_name);
        Ok(())
    })
    .await;

    match result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Rename mod task panicked: {:?}", e);
            Err(format!("Rename mod task panicked: {:?}", e))
        }
    }
}

#[tauri::command]
pub fn set_mod_author(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    mod_names: Vec<String>,
    author: Option<String>,
) {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    mod_manager.set_mod_author(&app_handle, mod_names, author);
}

// --- Experimental Mod Sync & Recognition -------------------------------------

/// An active game mod that was matched to a staging source by content
/// fingerprint (name-agnostic). `staging_name` is the mod name used by profiles.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveMatch {
    pub active_path: PathBuf,
    pub active_name: String,
    pub staging_name: String,
    pub staging_path: PathBuf,
}

/// An active game mod with no matching source in the staging directory.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveOrphan {
    pub active_path: PathBuf,
    pub active_name: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveScanResult {
    pub matched: Vec<ActiveMatch>,
    pub orphans: Vec<ActiveOrphan>,
}

/// Progress event emitted during a scan so the UI can show how many active mods
/// have been fingerprinted so far.
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecognizeProgress {
    pub scanned: usize,
    pub total: usize,
}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type", content = "details")]
pub enum RecognizeError {
    ActiveDirectoryNotSet,
    ActiveDirectoryNotFound(String),
    ScanFailed(String),
    CopyFailed(String),
    DeleteFailed(String),
}

/// Resolve the folder that holds the game's *active* (manually-installed) mods.
/// Prefers the user-configured `game_mods_directory`; otherwise derives it from
/// the game directory. Returns `None` when neither is available.
fn resolve_active_mods_dir(config: &bd2modmanager_lib::config::BD2Config) -> Option<PathBuf> {
    if let Some(dir) = &config.game_mods_directory {
        if !dir.trim().is_empty() {
            return Some(PathBuf::from(dir));
        }
    }
    config
        .game_directory
        .as_ref()
        .filter(|d| !d.trim().is_empty())
        .map(|d| PathBuf::from(d).join("BepInEx/plugins/BrownDustX/mods"))
}

/// A path is one of the manager's own synced copies when it lives under the
/// `BD2MM` folder the app syncs into. Those must never be treated as active mods.
fn is_managed_sync_path(path: &std::path::Path) -> bool {
    path.components().any(|c| c.as_os_str() == "BD2MM")
}

/// The lowercased leaf folder name of a mod path, used as a secondary identity
/// for matching. Returns `None` when the path has no file-name component.
fn leaf_folder_name(path: &std::path::Path) -> Option<String> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.to_ascii_lowercase())
}

/// Scan the active mods folder and match each active mod against the staging
/// directory by content fingerprint (so matching is independent of folder
/// names). Returns matched pairs plus orphans that have no staging source.
///
/// This runs on a blocking thread pool (not the UI thread) and fingerprints
/// mods in parallel with rayon, so a large staging library no longer freezes
/// the app. Fingerprints are metadata-only (relative paths + file sizes), which
/// avoids reading every file's contents while still reliably matching a copied
/// mod folder.
#[tauri::command]
pub async fn scan_active_mods(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<ActiveScanResult, RecognizeError> {
    use tauri::Emitter;

    // Extract everything we need from config, then release the lock before the
    // heavy work so we never hold it across the blocking task.
    let (active_dir, staging_dir, recursive) = {
        let config = state.config.lock().unwrap();
        let active_dir =
            resolve_active_mods_dir(&config).ok_or(RecognizeError::ActiveDirectoryNotSet)?;
        (
            active_dir,
            get_staging_dir(&config),
            config.search_mods_recursively,
        )
    };

    if !active_dir.is_dir() {
        return Err(RecognizeError::ActiveDirectoryNotFound(
            active_dir.to_string_lossy().to_string(),
        ));
    }

    // Offload discovery + matching to a blocking thread so the UI stays
    // responsive. rayon parallelizes the per-mod work inside.
    tauri::async_runtime::spawn_blocking(move || {
        use rayon::prelude::*;

        // A staging candidate: the mod plus its texture signature, precomputed
        // once so the tie-break below is cheap.
        #[derive(Clone)]
        struct StagingCandidate {
            bd2mod: BD2Mod,
            textures: Vec<(String, u64)>,
        }

        // Index staging mods by their descriptor key (content identity). Several
        // staging mods can share one key — e.g. an original and a "redit" edition
        // with identical descriptors but different textures — so each key maps to
        // a *list* of candidates that the active mod picks the closest of. The
        // per-key list is sorted by staging name so the index is deterministic
        // (fixes unmatched flip-flopping between rescans).
        let staging_mods = discover_staging_mods(&staging_dir, recursive)
            .into_iter()
            .filter(|m| !is_archive(&m.path))
            .collect::<Vec<_>>();

        // The leaf folder name, lowercased, is a strong secondary identity: a
        // mod copied from library to the game keeps its folder name even when
        // its contents diverge (a re-exported `.skel`, an added `.rar`/`.skelbk`,
        // renamed `.atlas`, etc.). We capture it alongside the descriptor key.
        let staging_indexed: Vec<(String, Option<String>, StagingCandidate)> = staging_mods
            .into_par_iter()
            .filter_map(|m| match mod_descriptor_key(&m.path) {
                Ok(key) => {
                    let textures = mod_texture_signature(&m.path).unwrap_or_default();
                    let leaf = leaf_folder_name(&m.path);
                    Some((key, leaf, StagingCandidate { bd2mod: m, textures }))
                }
                Err(e) => {
                    warn!("Failed to key staging mod {:?}: {}", m.path, e);
                    None
                }
            })
            .collect();

        let mut staging_by_key: std::collections::HashMap<String, Vec<StagingCandidate>> =
            std::collections::HashMap::new();
        // Secondary index by leaf folder name for the fallback tier.
        let mut staging_by_leaf: std::collections::HashMap<String, Vec<StagingCandidate>> =
            std::collections::HashMap::new();
        for (key, leaf, cand) in staging_indexed {
            if let Some(leaf) = leaf {
                staging_by_leaf.entry(leaf).or_default().push(cand.clone());
            }
            staging_by_key.entry(key).or_default().push(cand);
        }
        for candidates in staging_by_key.values_mut() {
            candidates.sort_by(|a, b| a.bd2mod.name.cmp(&b.bd2mod.name));
        }
        for candidates in staging_by_leaf.values_mut() {
            candidates.sort_by(|a, b| a.bd2mod.name.cmp(&b.bd2mod.name));
        }

        // Discover active mods, skipping the manager's own BD2MM sync folder and
        // any loose archive files (`.zip`, `.rar`, …) — an unextracted archive is
        // not an installed mod, so it must never be matched or counted.
        let active_mods: Vec<BD2Mod> = discover_staging_mods(&active_dir, recursive)
            .into_iter()
            .filter(|m| !is_managed_sync_path(&m.path))
            .filter(|m| !is_archive(&m.path))
            .collect();

        // Emit the total up front, then a running count so the UI can show how
        // many active mods have been scanned.
        let total = active_mods.len();
        let scanned = std::sync::atomic::AtomicUsize::new(0);
        app_handle
            .emit(
                "recognize-progress",
                RecognizeProgress { scanned: 0, total },
            )
            .ok();

        // Match each active mod by descriptor key, then break ties by texture
        // signature overlap so the closest staging edition wins.
        let results: Vec<Result<ActiveMatch, ActiveOrphan>> = active_mods
            .into_par_iter()
            .map(|active| {
                let done = scanned.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                app_handle
                    .emit(
                        "recognize-progress",
                        RecognizeProgress { scanned: done, total },
                    )
                    .ok();
                let active_name = active
                    .path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| active.display_name.clone());

                let key = match mod_descriptor_key(&active.path) {
                    Ok(k) => k,
                    Err(e) => {
                        warn!("Failed to key active mod {:?}: {}", active.path, e);
                        return Err(ActiveOrphan {
                            active_path: active.path.clone(),
                            active_name,
                        });
                    }
                };

                // Helper: from a candidate list, pick the one whose textures
                // overlap the active mod's the most. Candidate lists are
                // name-sorted and `max_by_key` is deterministic, so the choice is
                // stable across rescans (fixes unmatched flip-flopping).
                let pick_best = |candidates: &[StagingCandidate]| -> Option<(String, PathBuf)> {
                    if candidates.is_empty() {
                        return None;
                    }
                    let active_textures =
                        mod_texture_signature(&active.path).unwrap_or_default();
                    let best = candidates
                        .iter()
                        .max_by_key(|c| {
                            texture_signature_overlap(&active_textures, &c.textures)
                        })
                        .unwrap();
                    Some((best.bd2mod.name.clone(), best.bd2mod.path.clone()))
                };

                // Tier 1: descriptor-key (content identity). Tier 2 fallback:
                // identical leaf folder name — a mod copied from library keeps
                // its folder name even when its contents later diverge (missing
                // or re-exported `.skel`, added `.rar`/`.skelbk`, renamed
                // `.atlas`, etc.), which tier 1 alone would miss.
                let hit = staging_by_key
                    .get(&key)
                    .and_then(|c| pick_best(c))
                    .or_else(|| {
                        leaf_folder_name(&active.path)
                            .and_then(|leaf| staging_by_leaf.get(&leaf))
                            .and_then(|c| pick_best(c))
                    });

                match hit {
                    Some((staging_name, staging_path)) => Ok(ActiveMatch {
                        active_path: active.path.clone(),
                        active_name,
                        staging_name,
                        staging_path,
                    }),
                    None => Err(ActiveOrphan {
                        active_path: active.path.clone(),
                        active_name,
                    }),
                }
            })
            .collect();

        let mut matched = Vec::new();
        let mut orphans = Vec::new();
        for r in results {
            match r {
                Ok(m) => matched.push(m),
                Err(o) => orphans.push(o),
            }
        }

        Ok(ActiveScanResult { matched, orphans })
    })
    .await
    .map_err(|e| RecognizeError::ScanFailed(e.to_string()))?
}

/// Copy an orphan active mod folder into the staging directory (recursively,
/// preserving the folder as a single multi-file unit), then re-discover mods so
/// the new source appears. Returns the new staging mod name (its relative path).
#[tauri::command]
pub fn import_orphan_mod(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    source_path: String,
    destination_path: String,
) -> Result<String, RecognizeError> {
    let source = PathBuf::from(&source_path);
    let destination = PathBuf::from(&destination_path);

    let folder_name = source
        .file_name()
        .ok_or_else(|| RecognizeError::CopyFailed(format!("Invalid source path: {}", source_path)))?
        .to_owned();

    let dest_dir = destination.join(&folder_name);

    copy_dir_all(&source, &dest_dir)
        .map_err(|e| RecognizeError::CopyFailed(e.to_string()))?;

    // Re-discover so the imported mod is cached, then compute its staging name
    // (relative path from the staging dir) for profile integration.
    let staging_dir = {
        let config = state.config.lock().unwrap();
        get_staging_dir(&config)
    };
    {
        let config = state.config.lock().unwrap();
        let mut mod_manager = state.mod_manager.lock().unwrap();
        mod_manager.discover_mods(
            &app_handle,
            &staging_dir,
            config.search_mods_recursively,
        );
    }

    let staging_name = dest_dir
        .strip_prefix(&staging_dir)
        .unwrap_or(&dest_dir)
        .to_string_lossy()
        .replace('\\', "/");

    Ok(staging_name)
}

/// Delete the original active-mod folders left loose in the game mods folder.
/// The frontend gates this behind an explicit second confirmation. Refuses to
/// touch anything under the manager's BD2MM sync folder.
#[tauri::command]
pub fn delete_active_mods(paths: Vec<String>) -> Result<(), RecognizeError> {
    for p in paths {
        let path = PathBuf::from(&p);
        if is_managed_sync_path(&path) {
            warn!("Refusing to delete managed sync path: {:?}", path);
            continue;
        }
        if !path.exists() {
            continue;
        }
        let result = if path.is_dir() {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        };
        result.map_err(|e| RecognizeError::DeleteFailed(format!("{:?}: {}", path, e)))?;
    }
    Ok(())
}
