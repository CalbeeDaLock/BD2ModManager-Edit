use log::{info, warn};
use rayon::prelude::*;
use std::{
    ffi::OsStr,
    fs::{self, read_dir},
    path::Path,
    sync::Arc,
    time::Instant,
};

use crate::{
    mods::{types::BD2ModError, utils::get_mod_type, BD2Mod},
    utils::{files::has_extension, is_archive},
};

const MAX_DEPTH: usize = 5;

static MOD_EXTENSIONS: &[&str] = &["png", "modfile", "atlas", "skel", "json"];

#[derive(Debug, Default)]
struct ModAnalysis {
    has_modfile: bool,
    has_images: bool,
    has_skel: bool,
    has_atlas: bool,
    has_textures_folder: bool,
}

impl ModAnalysis {
    fn analyze_entry(&mut self, entry: &fs::DirEntry) {
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(OsStr::to_str) {
                match ext.to_ascii_lowercase().as_str() {
                    "modfile" => self.has_modfile = true,
                    "png" => self.has_images = true,
                    "skel" | "json" => self.has_skel = true,
                    "atlas" => self.has_atlas = true,
                    _ => {}
                }
            }
        } else if path.is_dir() {
            if path.file_name() == Some(OsStr::new("textures")) {
                self.has_textures_folder = true;
            }
        }
    }

    fn determine_mod_status(&self) -> (bool, Option<BD2ModError>) {
        match (
            self.has_modfile,
            self.has_skel,
            self.has_atlas,
            self.has_textures_folder,
            self.has_images,
        ) {
            (true, _, _, _, _) => (true, None),
            (false, true, true, _, _) => (true, Some(BD2ModError::MissingModfile)),
            (false, true, false, _, _) => (true, Some(BD2ModError::MissingAtlasFile)),
            (false, false, _, true, _) => (true, Some(BD2ModError::MissingModfile)),
            (false, false, _, false, true) => (true, Some(BD2ModError::IncompleteMod)),
            _ => (false, None),
        }
    }
}

pub fn analyze_mod_path(path: &Path) -> (bool, Option<BD2ModError>) {
    if path.is_file() {
        return if is_archive(path) {
            (true, Some(BD2ModError::ArchiveNotExtracted))
        } else if has_extension(path, MOD_EXTENSIONS) {
            (true, Some(BD2ModError::ShouldBeInFolder))
        } else {
            (false, None)
        };
    }

    let mut analysis = ModAnalysis::default();

    if let Ok(entries) = read_dir(path) {
        entries
            .filter_map(Result::ok)
            .for_each(|entry| analysis.analyze_entry(&entry));
    }

    analysis.determine_mod_status()
}

pub fn create_mod(staging_directory: &Path, path: &Path, error: Option<BD2ModError>) -> BD2Mod {
    let full_name = path
        .strip_prefix(staging_directory)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned();

    let display_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".to_string());

    BD2Mod {
        path: path.to_path_buf(),
        name: full_name,
        display_name,
        mod_type: get_mod_type(path),
        errors: error.into_iter().collect(),
        ..Default::default()
    }
}

// fn is_textures_folder(entry: &walkdir::DirEntry) -> bool {
//     if let Some(file_name) = entry.path().file_name() {
//         file_name == "textures"
//     } else {
//         false
//     }
// }

// Very fast but if a mod is inside another mod it will take it too.
// pub fn discover_staging_mods_1(staging_directory: &Path, recursive: bool) -> Vec<BD2Mod> {
//     let start = Instant::now();
//     let staging_dir = Arc::new(staging_directory.to_path_buf());

//     let mods: Vec<BD2Mod> = if recursive {
//         WalkDir::new(staging_directory)
//             .min_depth(1)
//             .max_depth(MAX_DEPTH)
//             .into_iter()
//             .par_bridge()
//             .filter_map(Result::ok)
//             .filter(|entry| !is_textures_folder(entry))
//             .filter(|e| e.file_type().is_dir())
//             .filter_map(|entry| {
//                 let (is_mod, error) = analyze_mod_path(entry.path());
//                 if is_mod {
//                     Some(create_mod(&staging_dir, entry.path(), error))
//                 } else {
//                     None
//                 }
//             })
//             .collect()
//     } else {
//         match read_dir(staging_directory) {
//             Ok(entries) => entries
//                 .par_bridge()
//                 .filter_map(Result::ok)
//                 .filter_map(|entry| {
//                     let (is_mod, error) = analyze_mod_path(&entry.path());
//                     if is_mod {
//                         Some(create_mod(&staging_dir, &entry.path(), error))
//                     } else {
//                         None
//                     }
//                 })
//                 .collect(),
//             Err(e) => {
//                 eprintln!("Failed to read staging directory: {}", e);
//                 Vec::new()
//             }
//         }
//     };

//     info!("discover_staging_mods() took {:?}", start.elapsed());

//     mods
// }

// // 50-70ms
// fn discover_mods_recursive_1(dir: &Path, staging_directory: &Path, mods: &mut Vec<BD2Mod>) {
//     if let Ok(entries) = read_dir(dir) {
//         for entry in entries.filter_map(Result::ok) {
//             let path = entry.path();

//             if path.is_dir() {
//                 // Skip textures folders
//                 if path.file_name() == Some(OsStr::new("textures")) {
//                     continue;
//                 }

//                 // Check if this directory is a mod
//                 let (is_mod, error) = analyze_mod_path(&path);

//                 if is_mod {
//                     // println!("{:?}", path);
//                     mods.push(create_mod(staging_directory, &path, error));
//                     // Don't recurse into mod directories - this prevents nested detection
//                 } else {
//                     // Only recurse if this isn't a mod directory
//                     discover_mods_recursive_1(&path, staging_directory, mods);
//                 }
//             }
//         }
//     }
// }

// pub fn discover_staging_mods_2(staging_directory: &Path, recursive: bool) -> Vec<BD2Mod> {
//     let start = Instant::now();

//     let mods: Vec<BD2Mod> = if recursive {
//         let mut mods = Vec::new();
//         discover_mods_recursive_1(staging_directory, staging_directory, &mut mods);
//         mods
//     } else {
//         match read_dir(staging_directory) {
//             Ok(entries) => entries
//                 .par_bridge()
//                 .filter_map(Result::ok)
//                 .filter_map(|entry| {
//                     let (is_mod, error) = analyze_mod_path(&entry.path());
//                     if is_mod {
//                         Some(create_mod(staging_directory, &entry.path(), error))
//                     } else {
//                         None
//                     }
//                 })
//                 .collect(),
//             Err(e) => {
//                 eprintln!("Failed to read staging directory: {}", e);
//                 Vec::new()
//             }
//         }
//     };

//     info!("discover_staging_mods() took {:?}", start.elapsed());

//     mods
// }

fn discover_mods_recursive(dir: &Path, staging_directory: &Path, depth: usize) -> Vec<BD2Mod> {
    if depth >= MAX_DEPTH {
        return Vec::new();
    }

    let staging_dir = Arc::new(staging_directory.to_path_buf());

    match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .par_bridge()
            // .filter(|entry| entry.path().is_dir())
            .filter(|entry| entry.file_name() != OsStr::new("textures"))
            .flat_map(|entry| {
                let path = entry.path();
                let (is_mod, error) = analyze_mod_path(&path);
                info!("({:?}) {:?} -> {:?}", is_mod, path, error);
                if is_mod {
                    vec![create_mod(&staging_dir, &path, error)]
                } else {
                    if path.is_dir() {
                        discover_mods_recursive(&path, &staging_dir, depth + 1)
                    } else {
                        warn!("Path {:?} not valid! {:?}", path, error);
                        Vec::new()
                    }
                }
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub fn discover_staging_mods(staging_directory: &Path, recursive: bool) -> Vec<BD2Mod> {
    let start: Instant = Instant::now();

    let mods: Vec<BD2Mod> = if recursive {
        discover_mods_recursive(staging_directory, staging_directory, 0)
    } else {
        match fs::read_dir(staging_directory) {
            Ok(entries) => entries
                .par_bridge()
                .filter_map(Result::ok)
                // .filter(|entry| entry.path().is_dir())
                .filter_map(|entry| {
                    let path = entry.path();
                    let (is_mod, error) = analyze_mod_path(&path);
                    if is_mod {
                        Some(create_mod(staging_directory, &path, error))
                    } else {
                        None
                    }
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    };

    info!("discover_staging_mods() took {:?}", start.elapsed());
    mods
}
