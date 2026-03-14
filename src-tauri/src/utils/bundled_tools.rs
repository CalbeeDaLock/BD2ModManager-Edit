use tauri::{path::BaseDirectory, AppHandle};

use tauri::Manager;

pub fn move_bundled_tools(app_handle: &AppHandle) -> std::io::Result<()> {
    let bundled_tools_dir = app_handle
        .path()
        .resolve("bundled_tools", BaseDirectory::Resource)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string()))?;

    let target_dir = app_handle
        .path()
        .resolve("tools", BaseDirectory::AppData)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e.to_string()))?;

    if !target_dir.exists() {
        std::fs::create_dir_all(&target_dir)?;
    }

    for entry in std::fs::read_dir(bundled_tools_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_file() {
            let target_file = target_dir.join(entry.file_name());
            if !target_file.exists() {
                std::fs::copy(entry.path(), target_file)?;
            }
        }
    }

    Ok(())
}
