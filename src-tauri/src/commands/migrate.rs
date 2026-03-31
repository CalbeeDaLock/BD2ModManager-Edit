use tauri::{AppHandle};

use crate::{AppState, migrate::{self, migrate::LegacyProfile}};

#[tauri::command]
pub fn get_legacy_profiles(app_handle: AppHandle) -> Result<Vec<LegacyProfile>, String> {
    migrate::get_profiles(&app_handle)
}

#[tauri::command]
pub fn import_legacy_profiles(
    app_handle: AppHandle,
    profile_ids: Vec<String>,
) -> Result<(), String> {
    migrate::import_profiles(&app_handle, profile_ids)
}


#[tauri::command]
pub fn import_legacy_mod_authors(app_handle: AppHandle, state: tauri::State<AppState>) -> Result<(), String> {
    let mut mod_manager = state.mod_manager.lock().unwrap();
    migrate::import_mod_authors(&app_handle, &mut mod_manager)
}