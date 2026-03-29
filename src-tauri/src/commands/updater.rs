use bd2modmanager_lib::utils::path::get_mod_preview_path;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncWriteExt;
use std::sync::Mutex;
use tauri::{State};
use tauri_plugin_updater::{Update, UpdaterExt};

use crate::update;

#[derive(Debug, Deserialize, Clone, Serialize)]
struct UpdateInfo {
    version: String,
    download_url: String,
}

#[tauri::command]
pub fn get_mod_preview_version(app_handle: tauri::AppHandle) -> Option<String> {
    update::mod_preview::get_mod_preview_version(app_handle)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAvailablePayload {
    latest_version: String,
    download_url: String,
}

#[tauri::command]
pub async fn check_for_mod_preview_update(
    app_handle: AppHandle,
) -> Result<UpdateAvailablePayload, String> {
    let (latest_version, _download_url) =
        update::mod_preview::check_for_update(app_handle).await?;
    if let Some(download_url) = _download_url {
        Ok(UpdateAvailablePayload {
            latest_version: latest_version.unwrap_or_default(),
            download_url,
        })
    } else {
        Err("No update available".to_string())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ProgressPayload {
    version: String,
    progress: u8,
}

#[tauri::command]
pub async fn update_mod_preview(app_handle: AppHandle) {
    app_handle.emit("update:modPreview:checking", ()).ok();

    let result = update::mod_preview::check_for_update(app_handle.clone()).await;

    match result {
        Ok((Some(latest_version), Some(download_url))) => {
            app_handle
                .emit(
                    "update:modPreview:available",
                    (latest_version.clone(), download_url.clone()),
                )
                .ok();

            let dest_path = match get_mod_preview_path(&app_handle) {
                Some(p) => p,
                None => {
                    app_handle
                        .emit(
                            "update:modPreview:error",
                            "Failed to resolve mod preview path",
                        )
                        .ok();
                    return;
                }
            };

            if let Some(parent) = dest_path.parent() {
                if let Err(e) = tokio::fs::create_dir_all(parent).await {
                    app_handle
                        .emit(
                            "update:modPreview:error",
                            format!("Failed to create directory: {e}"),
                        )
                        .ok();
                    return;
                }
            }

            let client = reqwest::Client::new();
            let response = match client
                .get(&download_url)
                .header("User-Agent", "BD2ModManager")
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    app_handle
                        .emit(
                            "update:modPreview:error",
                            format!("Download request failed: {e}"),
                        )
                        .ok();
                    return;
                }
            };

            if !response.status().is_success() {
                app_handle
                    .emit(
                        "update:modPreview:error",
                        format!("Download failed: HTTP {}", response.status()),
                    )
                    .ok();
                return;
            }

            let total_size = response.content_length().unwrap_or(0);
            let mut downloaded: u64 = 0;

            let mut file = match tokio::fs::File::create(&dest_path).await {
                Ok(f) => f,
                Err(e) => {
                    app_handle
                        .emit(
                            "update:modPreview:error",
                            format!("Failed to create file: {e}"),
                        )
                        .ok();
                    return;
                }
            };

            let mut stream = response.bytes_stream();
            use futures_util::StreamExt;

            while let Some(chunk) = stream.next().await {
                let chunk = match chunk {
                    Ok(c) => c,
                    Err(e) => {
                        app_handle
                            .emit("update:modPreview:error", format!("Download error: {e}"))
                            .ok();
                        return;
                    }
                };

                if let Err(e) = file.write_all(&chunk).await {
                    app_handle
                        .emit("update:modPreview:error", format!("Write error: {e}"))
                        .ok();
                    return;
                }

                downloaded += chunk.len() as u64;

                let progress = if total_size > 0 {
                    (downloaded as f64 / total_size as f64 * 100.0) as u8
                } else {
                    0
                };

                app_handle
                    .emit(
                        "update:modPreview:downloading",
                        ProgressPayload {
                            version: latest_version.clone(),
                            progress,
                        },
                    )
                    .ok();
            }

            app_handle
                .emit("update:modPreview:downloaded", latest_version.clone())
                .ok();
        }

        Ok((None, None)) => {
            app_handle.emit("update:modPreview:uptodate", ()).ok();
        }

        Ok(_) => {
            app_handle
                .emit("update:modPreview:error", "Invalid update state")
                .ok();
        }

        Err(err) => {
            app_handle.emit("update:modPreview:error", err).ok();
        }
    }
}

#[tauri::command]
pub async fn update_game_data(app_handle: AppHandle) -> Result<(), String> {
    update::game_data::update_characters(app_handle).await
}



#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Updater(#[from] tauri_plugin_updater::Error),
    #[error("there is no pending update")]
    NoPendingUpdate,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

type ResultUpdate<T> = std::result::Result<T, Error>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetadata {
    version: String,
    current_version: String,
}

pub struct PendingUpdate(pub Mutex<Option<(Update, Vec<u8>)>>);

#[tauri::command]
pub async fn check_for_app_update(
    app_handle: AppHandle,
    pending_update: State<'_, PendingUpdate>,
) -> ResultUpdate<Option<UpdateMetadata>> {
    if cfg!(debug_assertions) {
        return Ok(Some(UpdateMetadata {
            version: "4.0.6".to_string(),
            current_version: "4.0.5".to_string(),
        }));
    }

    let update = app_handle.updater()?.check().await?;

    match update {
        None => Ok(None),
        Some(update) => {
            let metadata = UpdateMetadata {
                version: update.version.clone(),
                current_version: update.current_version.clone(),
            };

            let bytes = update.download(|_, _| {}, || {}).await?;

            *pending_update.0.lock().unwrap() = Some((update, bytes));

            Ok(Some(metadata))
        }
    }
}

#[tauri::command]
pub async fn install_app_update(
    app_handle: AppHandle,
    pending_update: State<'_, PendingUpdate>,
) -> ResultUpdate<()> {
    let Some((update, bytes)) = pending_update.0.lock().unwrap().take() else {
        return Err(Error::NoPendingUpdate);
    };

    update.install(bytes)?;
    app_handle.restart();

    Ok(())
}