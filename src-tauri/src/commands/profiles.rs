use bd2modmanager_lib::{mods::BD2Mod, profiles::types::Profile};
use log::info;
use serde::{Deserialize, Serialize};
use tauri::Emitter;

use crate::AppState;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilesResult {
    active_profile: Profile,
    profiles: Vec<Profile>,
}

#[tauri::command]
pub fn get_profiles(state: tauri::State<AppState>) -> ProfilesResult {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    let profiles: Vec<Profile> = mod_manager.get_profiles();
    let active_profile: Profile = mod_manager.get_active_profile().unwrap().clone();

    ProfilesResult {
        active_profile: active_profile,
        profiles: profiles,
    }
}

#[tauri::command]
pub fn switch_profile(
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
    id: String,
) -> Option<String> {
    let mut mod_manager = state.mod_manager.lock().unwrap();

    if let Ok(profile_id) = mod_manager.switch_profile(&app_handle, id) {
        info!("Profile switched to {:?}", profile_id);
        Some(profile_id)
    } else {
        info!("Failed to switch profile");
        None
    }
}

#[tauri::command]
pub fn edit_profile(
    state: tauri::State<AppState>,
    id: String,
    name: String,
    description: Option<String>,
) -> bool {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    match mod_manager.edit_profile(id, name, description) {
        Ok(()) => true,
        Err(_) => false,
    }
}

#[tauri::command]
pub fn create_profile(
    state: tauri::State<AppState>,
    name: String,
    description: Option<String>,
    template_id: Option<String>,
) -> Option<String> {
    let mut mod_manager = state.mod_manager.lock().ok()?; // safe lock
    mod_manager
        .create_profile(name, description, template_id)
        .ok()
        .map(|profile| profile) // avoid cloning entire struct
}

#[tauri::command]
pub fn delete_profile(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppState>,
    id: String,
) -> bool {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    match mod_manager.delete_profile(id) {
        Ok(_s) => {
            let all_mods: Vec<&BD2Mod> = mod_manager.cached_mods.values().collect();
            app_handle.emit("mods-updated", all_mods).unwrap();
            true
        }
        Err(_) => false,
    }
}
