use std::path::Path;
use sys_locale::get_locale;

#[tauri::command]
pub fn path_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command]
pub fn is_folder(path: String) -> bool {
    Path::new(&path).is_dir()
}

/// Returns true when the current process is running with elevated
/// (administrator) privileges.
#[tauri::command]
pub fn is_elevated() -> bool {
    bd2modmanager_lib::utils::misc::is_admin()
}

/// Relaunches the current executable elevated via a Windows UAC prompt.
///
/// On success (user accepts the prompt) the elevated instance is spawned and
/// the current, non-elevated process exits so the elevated one takes over.
/// If the user declines the prompt (or elevation otherwise fails) this returns
/// an error string and the current process keeps running.
#[tauri::command]
pub fn relaunch_as_admin() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::UI::Shell::ShellExecuteW;
        use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

        // Already elevated, nothing to do.
        if bd2modmanager_lib::utils::misc::is_admin() {
            return Ok(());
        }

        let exe = std::env::current_exe().map_err(|e| e.to_string())?;

        // Encode "runas" verb and the exe path as null-terminated UTF-16.
        let verb: Vec<u16> = std::ffi::OsStr::new("runas")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let file: Vec<u16> = exe
            .as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // ShellExecuteW returns an HINSTANCE that is > 32 on success. Any value
        // <= 32 is an error code; ERROR_CANCELLED (1223) maps to a return of 5
        // (SE_ERR_ACCESSDENIED) when the user declines the UAC prompt.
        let result = unsafe {
            ShellExecuteW(
                std::ptr::null_mut(),
                verb.as_ptr(),
                file.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            )
        };

        if (result as isize) <= 32 {
            return Err(format!("Elevation cancelled or failed (code {})", result as isize));
        }

        // The elevated instance is starting; exit this one.
        std::process::exit(0);
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Elevation is only supported on Windows".to_string())
    }
}

/// Writes the given text content to an absolute file path. Used by the Logs
/// modal to export the in-memory logs to a file.
#[tauri::command]
pub fn write_log_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn is_portable() -> bool {
    #[cfg(feature = "portable")]
    {
        true
    }
    #[cfg(not(feature = "portable"))]
    {
        false
    }
}

#[tauri::command]
pub fn get_user_locale() -> String {
    get_locale().unwrap_or_else(|| String::from("en-US"))
}