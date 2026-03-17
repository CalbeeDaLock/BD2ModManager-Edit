use std::path::Path;

#[tauri::command]
pub fn path_exists(path: String) -> bool {
    Path::new(&path).exists()
}