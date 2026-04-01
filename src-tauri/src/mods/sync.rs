use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use chrono::Utc;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tempfile::NamedTempFile;

use crate::{
    mods::BD2Mod,
    utils::{
        files::{ensure_dir_exists, sync_dirs},
        misc::is_elevated,
    },
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SyncMethod {
    Copy,
    Hardlink,
    Symlink,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SyncManifest {
    pub method: SyncMethod,
    // pub synced_mods: HashMap<String, SyncedMod>, // mod name, disabled for now
    pub synced_mods: Vec<String>, // mod name
    pub synced_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SyncedMod {
    pub mod_path: PathBuf,      // staging path
    pub game_mod_path: PathBuf, // game mod path
}

// events
#[derive(Serialize, Clone)]
struct SyncStartEvent {
    r#type: SyncType,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SyncProgressEvent {
    r#type: SyncType,
    status: SyncStatus,
    mod_name: String,
    current: usize,
    total: usize,
    error: Option<SyncError>,
}

#[derive(Serialize, Clone)]
struct SyncEndEvent {
    r#type: SyncType,
    success: bool,
    synced: usize,
    total: usize,
    error: Option<SyncError>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "type", content = "details")]
pub enum SyncError {
    SymlinkAdminRequired,
    ModPathNotFound(String),
    CopyFailed(String),
    SymlinkFailed(String),
    HardlinkFailed(String),
    DirectoryCreationFailed(String),
    RemovalFailed(String),
}

#[derive(Serialize, Deserialize, Clone)]
enum SyncStatus {
    Synced,
    Removed,
    UpToDate, // no need to do anything
    Failed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum SyncType {
    Sync,
    Unsync,
}

fn remove_mod_path(path: &PathBuf) -> io::Result<()> {
    // symlnik is not removed by remove_dir_all
    if path.is_symlink() {
        fs::remove_dir(path).or_else(|_| fs::remove_file(path))
    } else {
        fs::remove_dir_all(path).or_else(|_| fs::remove_file(path))
    }
}

pub fn load_manifest(path: &PathBuf) -> Option<SyncManifest> {
    let content = fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

pub fn save_manifest(path: &PathBuf, manifest: SyncManifest) -> Result<(), io::Error> {
    let temp_file = NamedTempFile::new_in(path.parent().unwrap())?;
    let content = serde_json::to_string_pretty(&manifest)?;
    temp_file.as_file().write_all(content.as_bytes())?;
    temp_file.persist(path)?;
    Ok(())
}

pub fn sync_mods(
    app_handle: &tauri::AppHandle,
    game_directory: &PathBuf,
    mods: Vec<&BD2Mod>,
    method: SyncMethod,
) -> Result<(), SyncError> {
    app_handle
        .emit(
            "sync-start",
            SyncStartEvent {
                r#type: SyncType::Sync,
            },
        )
        .ok();

    // TODO: calculate the size that will be required to transfer, check if has space available
    // if is disk full or permission denied => sync end

    let manifest_path = game_directory.join(".bd2mm.json");
    let game_mods_path = game_directory.join("BepInEx/plugins/BrownDustX/mods/BD2MM");

    if let Err(e) = ensure_dir_exists(&game_mods_path) {
        app_handle
            .emit(
                "sync-end",
                SyncEndEvent {
                    r#type: SyncType::Sync,
                    success: false,
                    error: Some(SyncError::DirectoryCreationFailed(
                        game_mods_path.to_string_lossy().to_string(),
                    )),
                    synced: 0,
                    total: 0,
                },
            )
            .ok();
        warn!("Cannot create game mods directory: {}", e);
        return Err(SyncError::DirectoryCreationFailed(
            game_mods_path.to_string_lossy().to_string(),
        ));
    }

    if method == SyncMethod::Symlink {
        if let Ok(is_admin) = is_elevated() {
            if !is_admin {
                debug!("needs to be running as admin");
                app_handle
                    .emit(
                        "sync-end",
                        SyncEndEvent {
                            r#type: SyncType::Sync,
                            success: false,
                            error: Some(SyncError::SymlinkAdminRequired),
                            total: 0,
                            synced: 0,
                        },
                    )
                    .ok();
                return Err(SyncError::SymlinkAdminRequired);
            }
        }
    }

    let mut mods_to_remove = Vec::new();

    if let Some(previous_manifest) = load_manifest(&manifest_path) {
        // if method changed, then clean all synced
        if previous_manifest.method != method {
            debug!("sync method changed, removing all synced mods.");
            for entry in game_mods_path
                .read_dir()
                .unwrap_or_else(|_| fs::read_dir(".").unwrap())
            {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.symlink_metadata().is_err() {
                        debug!("no metadata, skipping.");
                        continue;
                    }
                    mods_to_remove.push(path);
                }
            }
        } else {
            // Remove mods that are in game mod folder but are no longer in the staging dir
            // the path is the parent of .modfile
            let current_mod_names: Vec<String> = mods.iter().map(|m| m.name.clone()).collect();

            debug!("current mod names in staging: {:?}", current_mod_names);

            let mut installed_mods: Vec<PathBuf> = Vec::new();

            for entry in walkdir::WalkDir::new(&game_mods_path)
                .follow_links(true)
                .min_depth(2)
            {
                match entry {
                    Ok(e) => {
                        let path = e.path();
                        if path
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| ext == "modfile")
                            .unwrap_or(false)
                        {
                            if let Some(parent) = path.parent() {
                                if !installed_mods.contains(&parent.to_path_buf()) {
                                    installed_mods.push(parent.to_path_buf());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        // broken symlink
                        // no .modfile to find
                        if let Some(path) = e.path() {
                            warn!("broken symlink detected: {:?}", path);
                            if !installed_mods.contains(&path.to_path_buf()) {
                                installed_mods.push(path.to_path_buf());
                            }
                        }
                    }
                }
            }

            debug!("installed mods in game folder: {:?}", installed_mods);

            mods_to_remove.extend(installed_mods.into_iter().filter(|mod_path| {
                if let Ok(relative) = mod_path.strip_prefix(&game_mods_path) {
                    // normalize separator so  / and \ paths match
                    let name = relative.to_string_lossy().replace("\\", "/");
                    // debug!("Checking if mod '{}' is in current mod names: {:?}", name, current_mod_names);
                    !current_mod_names.contains(&name)
                } else {
                    false
                }
            }));
        }
    } else {
        // No manifest  remove anything in game mods folder not in current mod list
        let current_mod_names: Vec<String> = mods.iter().map(|m| m.name.clone()).collect();

        for entry in game_mods_path
            .read_dir()
            .unwrap_or_else(|_| fs::read_dir(".").unwrap())
        {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !current_mod_names.contains(&name.to_string()) {
                        mods_to_remove.push(path);
                    }
                }
            }
        }
    }

    // skip disabled mods that are not in game folder or mods with errors that are enabled, we don't need to remove because it is never synced
    let mut index = 0;
    let mods_to_sync = mods.clone().into_iter().filter(|_mod| {
        let dst_path = game_mods_path.join(&_mod.name);

        if !dst_path.exists() && !_mod.enabled || !_mod.errors.is_empty() && _mod.enabled {
            false
        } else {
            true
        }
    });
    let total_mods_count = mods_to_sync.count() + mods_to_remove.iter().count();

    debug!(
        "Total mods to sync: {}, total mods to remove: {}",
        total_mods_count - mods_to_remove.len(),
        mods_to_remove.len()
    );
    
    #[cfg(debug_assertions)]
    println!("Mods to remove:");
    for path in &mods_to_remove {
        println!(
            "- {}",
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
        );
    }

    for path in mods_to_remove {
        debug!("Removing mod at path: {:?}", path);

        let (status, error) = match remove_mod_path(&path) {
            Ok(_) => {
                // remove parent dir if  empty
                if let Some(parent) = path.parent() {
                    if parent != game_mods_path {
                        let _ = fs::remove_dir(parent);
                    }
                }
                (SyncStatus::Removed, None)
            }
            Err(e) => (
                SyncStatus::Failed,
                Some(SyncError::RemovalFailed(e.to_string())),
            ),
        };

        index += 1;
        app_handle
            .emit(
                "sync-progress",
                SyncProgressEvent {
                    r#type: SyncType::Sync,
                    mod_name: path
                        .file_name()
                        .and_then(|f| f.to_str().map(|s| s.to_string()))
                        .unwrap_or_default(),
                    current: index,
                    total: total_mods_count,
                    status,
                    error,
                },
            )
            .ok();
    }

    // let mut synced_mods: HashMap<String, SyncedMod> = HashMap::new();
    let mut synced_mods: Vec<String> = Vec::new();
    for _mod in mods {
        // if mod has errors and is enabled, skip syncing
        if !_mod.errors.is_empty() && _mod.enabled {
            warn!(
                "mod {:?} has errors, skipping sync. Errors: {:?}",
                _mod.name, _mod.errors
            );
            continue;
        }

        let dst_path = game_mods_path.join(&_mod.name);

        if !_mod.path.exists() {
            warn!("mod {:?} does not exist in staging.", _mod.name);

            // if it was previously synced, remove it from game folder
            let (status, error) = if dst_path.exists() || dst_path.is_symlink() {
                match remove_mod_path(&dst_path) {
                    Ok(_) => {
                        // remove parent dir if now empty
                        if let Some(parent) = dst_path.parent() {
                            if parent != game_mods_path {
                                let _ = fs::remove_dir(parent);
                            }
                        }
                        (
                            SyncStatus::Removed,
                            Some(SyncError::ModPathNotFound(
                                _mod.path.to_string_lossy().to_string(),
                            )),
                        )
                    }
                    Err(e) => (
                        SyncStatus::Failed,
                        Some(SyncError::RemovalFailed(e.to_string())),
                    ),
                }
            } else {
                // was not in game folder, just report missing
                (
                    SyncStatus::Failed,
                    Some(SyncError::ModPathNotFound(
                        _mod.path.to_string_lossy().to_string(),
                    )),
                )
            };

            index = index + 1;
            app_handle
                .emit(
                    "sync-progress",
                    SyncProgressEvent {
                        r#type: SyncType::Sync,
                        current: index,
                        mod_name: _mod.name.clone(),
                        total: total_mods_count,
                        status,
                        error,
                    },
                )
                .ok();
            continue;
        }

        if !_mod.enabled {
            if dst_path.exists() || dst_path.is_symlink() {
                if let Err(e) = remove_mod_path(&dst_path) {
                    index = index + 1;
                    app_handle
                        .emit(
                            "sync-progress",
                            SyncProgressEvent {
                                r#type: SyncType::Sync,
                                current: index,
                                mod_name: _mod.name.clone(),
                                total: total_mods_count,
                                status: SyncStatus::Failed,
                                error: Some(SyncError::RemovalFailed(e.to_string())),
                            },
                        )
                        .ok();
                    continue;
                }

                // remove parent dir if now empty
                if let Some(parent) = dst_path.parent() {
                    if parent != game_mods_path {
                        let _ = fs::remove_dir(parent);
                    }
                }

                index = index + 1;

                app_handle
                    .emit(
                        "sync-progress",
                        SyncProgressEvent {
                            r#type: SyncType::Sync,
                            current: index,
                            mod_name: _mod.name.clone(),
                            total: total_mods_count,
                            status: SyncStatus::Removed,
                            error: None,
                        },
                    )
                    .ok();
            }
            continue;
        }

        let mut was_updated = false;
        let mut sync_error: Option<SyncError> = None;

        match method {
            SyncMethod::Copy => match sync_dirs(&_mod.path, &dst_path) {
                Ok(updated) => {
                    if updated {
                        was_updated = true;
                    }
                }
                Err(e) => {
                    sync_error = Some(SyncError::CopyFailed(e.to_string()));
                }
            },
            SyncMethod::Symlink => {
                let needs_update = if dst_path.is_symlink() {
                    dst_path.read_link().ok() != Some(_mod.path.clone())
                } else if dst_path.exists() {
                    true
                } else {
                    true
                };

                if needs_update {
                    // Remove existing if it's not a symlink pointing to the right place
                    if dst_path.exists() || dst_path.is_symlink() {
                        if let Err(e) = remove_mod_path(&dst_path) {
                            sync_error = Some(SyncError::RemovalFailed(e.to_string()));
                        }
                    }

                    // Create parent dirs
                    if sync_error.is_none() {
                        if let Some(parent) = dst_path.parent() {
                            if let Err(e) = fs::create_dir_all(parent) {
                                sync_error =
                                    Some(SyncError::DirectoryCreationFailed(e.to_string()));
                            }
                        }
                    }

                    if sync_error.is_none() {
                        #[cfg(target_family = "unix")]
                        {
                            use std::os::unix::fs::symlink;
                            if let Err(e) = symlink(&_mod.path, &dst_path) {
                                sync_error = Some(SyncError::SymlinkFailed(e.to_string()));
                            } else {
                                was_updated = true;
                            }
                        }
                        #[cfg(target_family = "windows")]
                        {
                            use std::os::windows::fs::symlink_dir;
                            if let Err(e) = symlink_dir(&_mod.path, &dst_path) {
                                sync_error = Some(SyncError::SymlinkFailed(e.to_string()));
                            } else {
                                was_updated = true;
                            }
                        }
                    }
                }
            }
            SyncMethod::Hardlink => {
                if dst_path.exists() {
                    if let Err(e) = fs::remove_dir_all(&dst_path) {
                        sync_error = Some(SyncError::RemovalFailed(e.to_string()));
                    }
                }

                if sync_error.is_none() {
                    if let Err(e) = ensure_dir_exists(&dst_path) {
                        sync_error = Some(SyncError::DirectoryCreationFailed(e.to_string()));
                    } else {
                        for entry in walkdir::WalkDir::new(&_mod.path) {
                            let entry = match entry {
                                Ok(e) => e,
                                Err(e) => {
                                    sync_error = Some(SyncError::HardlinkFailed(e.to_string()));
                                    break;
                                }
                            };
                            let relative = entry.path().strip_prefix(&_mod.path).unwrap();
                            let target = dst_path.join(relative);

                            if entry.file_type().is_dir() {
                                if let Err(e) = fs::create_dir_all(&target) {
                                    sync_error =
                                        Some(SyncError::DirectoryCreationFailed(e.to_string()));
                                    break;
                                }
                            } else {
                                if let Err(e) = fs::hard_link(entry.path(), &target) {
                                    sync_error = Some(SyncError::HardlinkFailed(e.to_string()));
                                    break;
                                }
                            }
                        }

                        if sync_error.is_none() {
                            was_updated = true;
                        }
                    }
                }
            }
        }

        index = index + 1;

        let (status, error) = if let Some(ref err) = sync_error {
            (SyncStatus::Failed, Some(err.clone()))
        } else if was_updated {
            (SyncStatus::Synced, None)
        } else {
            (SyncStatus::UpToDate, None)
        };

        app_handle
            .emit(
                "sync-progress",
                SyncProgressEvent {
                    r#type: SyncType::Sync,
                    current: index,
                    mod_name: _mod.name.clone(),
                    total: total_mods_count,
                    status,
                    error,
                },
            )
            .ok();

        // Only add to synced_mods if there was no error
        // mod that was disabled and removed, should we add to sync error?
        if sync_error.is_none() {
            synced_mods.push(_mod.name.clone());
        }
    }

    // if cancel sync, remove synced mods?
    debug!(
        "{:?} mods was synced to game folder.",
        synced_mods.iter().count()
    );

    let manifest = SyncManifest {
        method,
        synced_at: Utc::now(),
        synced_mods,
    };

    save_manifest(&manifest_path, manifest).ok();

    app_handle
        .emit(
            "sync-end",
            SyncEndEvent {
                r#type: SyncType::Sync,
                success: true,
                synced: index,
                total: total_mods_count,
                error: None,
            },
        )
        .ok();

    Ok(())
}
pub fn unsync_mods(
    app_handle: &tauri::AppHandle,
    game_directory: &PathBuf,
) -> Result<(), SyncError> {
    let manifest_path = game_directory.join(".bd2mm.json");
    let game_mods_path = game_directory.join("BepInEx/plugins/BrownDustX/mods/BD2MM");

    app_handle
        .emit(
            "sync-start",
            SyncStartEvent {
                r#type: SyncType::Unsync,
            },
        )
        .ok();

    let mut index = 0;
    let total_mods: usize = game_mods_path
        .read_dir()
        .unwrap_or_else(|_| fs::read_dir(".").unwrap())
        .count();

    for entry in game_mods_path
        .read_dir()
        .unwrap_or_else(|_| fs::read_dir(".").unwrap())
    {
        if let Ok(entry) = entry {
            let path = entry.path();
            debug!("Removing mod at path: {:?}", path);

            let result = remove_mod_path(&path);

            let (status, error) = match result {
                Ok(_) => (SyncStatus::Removed, None),
                Err(e) => (
                    SyncStatus::Failed,
                    Some(SyncError::RemovalFailed(e.to_string())),
                ),
            };

            index += 1;
            app_handle
                .emit(
                    "sync-progress",
                    SyncProgressEvent {
                        r#type: SyncType::Unsync,
                        mod_name: path
                            .file_name()
                            .and_then(|f| f.to_str().map(|s| s.to_string()))
                            .unwrap_or_default(),
                        current: index,
                        total: total_mods,
                        status,
                        error,
                    },
                )
                .ok();
        }
    }

    let _ = fs::remove_file(&manifest_path);

    app_handle
        .emit(
            "sync-end",
            SyncEndEvent {
                r#type: SyncType::Unsync,
                success: true,
                synced: index,
                total: total_mods,
                error: None,
            },
        )
        .ok();

    Ok(())
}

pub fn is_sync_needed(game_directory: &PathBuf, staging_mods: &[&BD2Mod]) -> bool {
    // simple check, todo: improve this by checking content too like sync do
    let manifest_path = game_directory.join(".bd2mm.json");
    let game_mods_path = game_directory.join("BepInEx/plugins/BrownDustX/mods/BD2MM");

    if !game_mods_path.exists() {
        debug!("Game mods path does not exist, sync is needed.");
        return true;
    }

    let manifest = match load_manifest(&manifest_path) {
        Some(m) => m,
        None => {
            debug!("Manifest cannot be loaded, sync is needed.");
            return true;
        }
    };

    let manifest_mods_set: std::collections::HashSet<&str> =
        manifest.synced_mods.iter().map(|s| s.as_str()).collect();

    let expected_mods_set: std::collections::HashSet<&str> = staging_mods
        .iter()
        .filter(|m| m.enabled && m.errors.is_empty())
        .map(|m| m.name.as_str())
        .collect();

    if manifest_mods_set != expected_mods_set {
        debug!(
            "Manifest mods and expected mods differ, sync is needed. Manifest: {:?}, Expected: {:?}",
            manifest_mods_set, expected_mods_set
        );
        return true;
    }

    for mod_name in &expected_mods_set {
        let dst_path = game_mods_path.join(mod_name);
        if !dst_path.exists() {
            debug!(
                "Mod {:?} missing from game folder, sync is needed.",
                mod_name
            );
            return true;
        }
    }
    let installed_mods: std::collections::HashSet<String> = {
        let mut set = std::collections::HashSet::new();
        for entry in walkdir::WalkDir::new(&game_mods_path).follow_links(true) {
            match entry {
                Ok(e) => {
                    if e.path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext == "modfile")
                        .unwrap_or(false)
                    {
                        if let Some(parent) = e.path().parent() {
                            if let Ok(rel) = parent.strip_prefix(&game_mods_path) {
                                set.insert(rel.to_string_lossy().replace("\\", "/"));
                            }
                        }
                    }
                }
                Err(e) => {
                    // broken symlink is installed
                    if let Some(path) = e.path() {
                        if let Ok(rel) = path.strip_prefix(&game_mods_path) {
                            set.insert(rel.to_string_lossy().replace("\\", "/"));
                        }
                    }
                }
            }
        }
        set
    };

    for installed in &installed_mods {
        if !expected_mods_set.contains(installed.as_str()) {
            debug!(
                "Extra mod {:?} found in game folder, sync is needed.",
                installed
            );
            return true;
        }
    }

    false
}
