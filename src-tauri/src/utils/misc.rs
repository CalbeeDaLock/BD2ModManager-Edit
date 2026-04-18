use std::path::PathBuf;
use pelite::pe32::Pe;
use pelite::FileMap;
#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::CloseHandle;
#[cfg(target_os = "windows")]
use windows_sys::Win32::Security::{
    GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
};
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

#[cfg(not(target_os = "windows"))]
use libc;

pub fn is_elevated() -> Result<bool, String> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let process = GetCurrentProcess();
            let mut token = std::ptr::null_mut();

            if OpenProcessToken(process, TOKEN_QUERY, &mut token) != 0 {
                let mut elevation: TOKEN_ELEVATION = TOKEN_ELEVATION { TokenIsElevated: 0 };
                let mut return_length: u32 = 0;

                let result = GetTokenInformation(
                    token,
                    TokenElevation,
                    &mut elevation as *mut TOKEN_ELEVATION as *mut _,
                    std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                    &mut return_length,
                ) != 0;

                CloseHandle(token);

                if result {
                    return Ok(elevation.TokenIsElevated != 0);
                }
            }
        }

        return Ok(false);
    }

    #[cfg(not(target_os = "windows"))]
    {
        return Ok(unsafe { libc::geteuid() == 0 });
    }
}

pub fn get_dll_version(dll_path: &PathBuf) -> Option<String> {
    if !dll_path.exists() {
        return None;
    }

    let file_map = FileMap::open(dll_path).ok()?;
    let pe = pelite::pe32::PeFile::from_bytes(&file_map).ok()?;
    let resources = pe.resources().ok()?;
    let version_info = resources.version_info().ok()?;
    let file_info = version_info.file_info();

    for (_lang, strings) in file_info.strings {
        for (key, value) in strings {
            if key == "FileVersion" {
                return Some(value.to_string());
            }
        }
    }

    None
}

pub fn compare_versions(first: &str, second: &str) -> std::cmp::Ordering {
    let first_parts: Vec<u64> = first
        .split('.')
        .filter_map(|part| part.parse().ok())
        .collect();
    let second_parts: Vec<u64> = second
        .split('.')
        .filter_map(|part| part.parse().ok())
        .collect();

    for index in 0..first_parts.len().max(second_parts.len()) {
        let first_compnent = first_parts.get(index).copied().unwrap_or(0);
        let second_component = second_parts.get(index).copied().unwrap_or(0);

        match first_compnent.cmp(&second_component) {
            std::cmp::Ordering::Equal => continue,
            ordering => return ordering,
        }
    }

    std::cmp::Ordering::Equal
}
