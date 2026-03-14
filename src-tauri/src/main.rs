// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bd2modmanager_lib::{
    config::{config::PartialAppConfig, BD2Config},
    mods::metadata::ModMetadataStore,
    profiles::ProfileManager,
    utils::{
        bundled_tools, data,
        path::{
            ensure_dir_exists, get_default_profiles_dir, get_default_staging_dir, get_temp_dir,
        },
    },
    BD2ModManager,
};
use tauri::Manager;

mod commands;
mod updater;

use commands::*;

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
            let temp_dir = get_temp_dir();

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

            ensure_dir_exists(&temp_dir).expect("Failed to create temp directory");
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

            // move bundled tools to appdata
            bundled_tools::move_bundled_tools(&app_handle).expect("Failed to move bundled tools");

            // move data to appdata
            data::move_data_to_appdata(&app_handle).expect("Failed to move data to appdata");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // mods
            discover_mods,
            get_mods,
            enable_mods,
            disable_mods,
            install_mod_from_zip,
            install_mod_from_folder,
            sync_mods,
            unsync_mods,
            is_sync_needed,
            // profiles
            get_profiles,
            switch_profile,
            edit_profile,
            create_profile,
            delete_profile,
            // config
            get_settings,
            set_settings,
            // game
            locate_game,
            validate_game_path,
            get_browndustx_version,
            get_bepinex_version,
            get_configmanager_version,
            install_bepinex_from_archive,
            install_bepinex_from_url,
            install_browndustx_from_archive,
            uninstall_bepinex,
            uninstall_browndustx,
            install_configmanager_from_archive,
            uninstall_configmanager,
            determine_archive_type,
            get_game_version,
            // preview
            preview_mod,
            get_mod_preview_version,
            check_for_app_update,
            check_for_mod_preview_update,
            update_mod_preview,
            update_game_data,
            get_characters,
            delete_mods,
            rename_mod,
            set_mod_author,
            launch_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
