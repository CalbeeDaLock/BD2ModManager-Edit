// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bd2modmanager_lib::{
    BD2ModManager, config::{BD2Config, config::PartialAppConfig}, mods::metadata::ModMetadataStore, profiles::ProfileManager, utils::{
        data, files::ensure_dir_exists, path::{
            get_default_profiles_dir, get_default_staging_dir
        }
    }
};
use tauri::Manager;

mod commands;
use commands::*;

mod update;

pub struct AppState {
    pub mod_manager: Arc<Mutex<BD2ModManager>>,
    pub config: Arc<Mutex<BD2Config>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .clear_targets()
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(2))
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    }),
                ])
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            log::info!("Starting app...");

            let app_handle = app.app_handle();

            let mut config = BD2Config::new(app_handle.clone());
            config.load_config();

            let profiles_dir: PathBuf = get_default_profiles_dir(app_handle, false);
            // let temp_dir = get_temp_dir();

            let staging_dir = match &config.staging_directory {
                Some(path) => PathBuf::from(path),
                None => {
                    let staging_dir = get_default_staging_dir();

                    config
                        .update_config(PartialAppConfig {
                            staging_directory: Some(staging_dir.to_string_lossy().to_string()),
                            ..Default::default()
                        })
                        .expect("Failed to update config with default staging directory");

                    staging_dir
                }
            };

            // ensure_dir_exists(&temp_dir).expect("Failed to create temp directory");
            ensure_dir_exists(&profiles_dir).expect("Failed to get profiles dir");
            ensure_dir_exists(&staging_dir).expect("Failed to create mods directory");

            let profile_manager: ProfileManager = ProfileManager::new(profiles_dir);

            let metadata_path = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to resolve AppData dir")
                .join("mod_metadata.json");
            let metadata_store = ModMetadataStore::new(metadata_path);

            let mut mod_manager: BD2ModManager =
                BD2ModManager::new(profile_manager, metadata_store);

            mod_manager
                .load_profiles()
                .expect("failed to load profiles");

            let app_state: AppState = AppState {
                mod_manager: Arc::new(Mutex::new(mod_manager)),
                config: Arc::new(Mutex::new(config)),
            };

            app.manage(app_state);

            // move data to appdata
            data::move_data_to_appdata(&app_handle).expect("Failed to move data to appdata");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // mods
            mods::discover_mods,
            mods::get_mods,
            mods::enable_mods,
            mods::disable_mods,
            mods::delete_mods,
            mods::rename_mod,
            mods::set_mod_author,
            mods::install_mod_from_zip,
            mods::install_mod_from_folder,
            mods::sync_mods,
            mods::unsync_mods,
            mods::is_sync_needed,
            mods::preview_mod,

            // profiles
            profiles::get_profiles,
            profiles::switch_profile,
            profiles::edit_profile,
            profiles::create_profile,
            profiles::delete_profile,

            // config
            config::get_settings,
            config::set_settings,
            
            // game
            game::locate_game,
            game::validate_game_path,
            game::launch_game,
            game::get_game_version,
            game::get_browndustx_version,
            game::get_bepinex_version,
            game::get_configmanager_version,
            game::install_bepinex_from_archive,
            game::install_bepinex_from_url,
            game::install_browndustx_from_archive,
            game::install_configmanager_from_archive,
            game::uninstall_bepinex,
            game::uninstall_browndustx,
            game::uninstall_configmanager,
            game::determine_archive_type,
            game::get_characters,

            // updater
            updater::get_mod_preview_version,
            updater::check_for_app_update,
            updater::check_for_mod_preview_update,
            updater::update_mod_preview,
            updater::update_game_data,

            // utils
            utils::is_folder,
            utils::path_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
