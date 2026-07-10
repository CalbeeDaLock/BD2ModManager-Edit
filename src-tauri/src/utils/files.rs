use std::{
    fs::{self, copy, create_dir_all, read, read_dir, remove_dir_all, remove_file},
    io,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use std::time::SystemTime;
use serde_json;

pub fn has_extension(path: &Path, extensions: &[&str]) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return extensions.iter().any(|e| e.eq_ignore_ascii_case(ext_str));
        }
    }
    false
}

pub fn is_archive(path: &Path) -> bool {
    let archive_exts = ["zip", "rar", "7z", "tar", "gz", "bz2", "xz"];
    has_extension(path, &archive_exts)
}

pub fn is_image(path: &Path) -> bool {
    let image_exts = ["png", "jpg", "jpeg"];
    has_extension(path, &image_exts)
}

pub fn has_folder(folder: &Path, name: &str) -> bool {
    if let Ok(entries) = read_dir(folder) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                if let Some(filename) = path.file_name() {
                    if filename == name {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        create_dir_all(dst)?;
    }

    for entry in read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_all(&entry_path, &dest_path)?;
        } else {
            copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

pub fn sync_dirs(src: &Path, dst: &Path) -> io::Result<bool> {
    let mut updated = false;

    if !dst.exists() {
        create_dir_all(dst)?;
        updated = true;
    }

    // Copy/update files from src too dst
    for entry in read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            if sync_dirs(&src_path, &dst_path)? {
                updated = true;
            }
        } else {
            let needs_copy = if dst_path.exists() {
                let src_meta = src_path.metadata()?;
                let dst_meta = dst_path.metadata()?;

                src_meta.len() != dst_meta.len()
                    || src_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH)
                        != dst_meta.modified().unwrap_or(SystemTime::UNIX_EPOCH)
            } else {
                true
            };

            if needs_copy {
                copy(&src_path, &dst_path)?;
                updated = true;
            }
        }
    }

    //  Remove files from dst that don’t exist in src
    for entry in read_dir(dst)? {
        let entry = entry?;
        let dst_path = entry.path();
        let src_path = src.join(entry.file_name());

        if !src_path.exists() {
            if dst_path.is_dir() {
                remove_dir_all(&dst_path)?;
            } else {
                remove_file(&dst_path)?;
            }
            updated = true;
        }
    }

    Ok(updated)
}

pub fn hash_directory<P: AsRef<Path>>(dir: P) -> io::Result<String> {
    let mut hasher = Sha256::new();

    let entries: Vec<_> = WalkDir::new(&dir)
        .sort_by_file_name()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    for entry in entries {
        if entry.file_type().is_file() {
            if let Ok(relative_path) = entry.path().strip_prefix(&dir) {
                hasher.update(relative_path.to_string_lossy().as_bytes());
            }

            let contents = read(entry.path())?;
            hasher.update(&contents);
        }
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// OS/editor junk files that get sprinkled into folders and must not influence
/// a mod's identity (they differ between an installed copy and its source).
fn is_junk_file(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "desktop.ini" | "thumbs.db" | ".ds_store" | "ehthumbs.db"
    )
}

/// Descriptor files that define what a mod *is* (its modfile plus the
/// spine/atlas/json descriptors). Their **content** (not name) identifies a mod.
fn is_descriptor_file(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".modfile")
        || lower.ends_with(".atlas")
        || lower.ends_with(".skel")
        || lower.ends_with(".json")
}

/// How much of a descriptor file to read. Descriptors are small, except spine
/// `.json` files that can reach tens of MB — but their identity is established
/// well within the first slice, and the watermark that varies between copies
/// lives at the very end. Reading a capped prefix is both fast and stable.
const DESCRIPTOR_READ_CAP: usize = 1 << 20; // 1 MiB

/// Read up to `DESCRIPTOR_READ_CAP` bytes of a descriptor file.
fn read_descriptor_prefix(path: &Path) -> io::Result<Vec<u8>> {
    use std::io::Read;
    let mut file = fs::File::open(path)?;
    let mut buf = Vec::new();
    file.by_ref()
        .take(DESCRIPTOR_READ_CAP as u64)
        .read_to_end(&mut buf)?;
    Ok(buf)
}

/// Normalize descriptor bytes so cosmetic or game-version-dependent differences
/// don't change a mod's identity:
/// - `.modfile`: parse as JSON and hash only the stable `assetName` values from
///   `modify[]`, ignoring the `original` path — the game periodically renames
///   internal hierarchy paths (e.g. game updates add an `AnchorCollider/` prefix),
///   so an installed copy and its staging source can have different `original`
///   values while being the same mod. The `assetName` (the texture the mod
///   injects) is always stable. Falls back to raw bytes if parsing fails.
/// - `.atlas`: drop `#`-prefixed comment lines (some tools stamp a
///   `# watermark:` line), and
/// - `.json`: truncate at the trailing `"_wmrk"` watermark field.
/// Everything else is returned unchanged.
fn normalize_descriptor(name: &str, data: &[u8]) -> Vec<u8> {
    let lower = name.to_ascii_lowercase();
    if lower.ends_with(".modfile") {
        // Extract `assetName` values from the `modify[]` array and hash those
        // only. The `original` path is ignored: it changes across game updates
        // but the mod is still the same mod.
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(data) {
            if let Some(modify) = json.get("modify").and_then(|m| m.as_array()) {
                let mut asset_names: Vec<String> = modify
                    .iter()
                    .filter_map(|entry| {
                        entry.get("assetName")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_ascii_lowercase())
                    })
                    .collect();
                if !asset_names.is_empty() {
                    asset_names.sort();
                    return asset_names.join("\n").into_bytes();
                }
            }
        }
        // Fallback: return raw bytes when the JSON can't be parsed or has no
        // modify array (e.g. empty .modfile placeholders).
        data.to_vec()
    } else if lower.ends_with(".atlas") {
        let mut out: Vec<u8> = Vec::with_capacity(data.len());
        for line in data.split(|&b| b == b'\n') {
            let trimmed = line
                .iter()
                .position(|&b| b != b' ' && b != b'\t')
                .map(|i| &line[i..])
                .unwrap_or(line);
            if trimmed.first() == Some(&b'#') {
                continue;
            }
            out.extend_from_slice(line);
            out.push(b'\n');
        }
        out
    } else if lower.ends_with(".json") {
        if let Some(idx) = find_subslice(data, b"\"_wmrk\"") {
            data[..idx].to_vec()
        } else {
            data.to_vec()
        }
    } else {
        data.to_vec()
    }
}

/// Find the first index of `needle` within `haystack`, if present.
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// A mod's **identity key** for recognition matching.
///
/// A mod's identity lives entirely in its **descriptor files** (`.modfile`,
/// `.atlas`, `.skel`, `.json`) — *not* in file names, texture bytes, or which
/// extra files happen to sit alongside it. The same mod is routinely shipped:
/// - with a renamed `.modfile` (e.g. `pack1003_title1` vs `LocalPackTitle…`),
/// - with textures re-exported to slightly different byte sizes,
/// - with per-copy watermarks stamped into the atlas/json,
/// - with extra tool/doc/backup files in one copy only (`fix_edges.py`,
///   `notes.txt`, `char.png.old`, a `_redit.png` preview, …).
///
/// So the key is a SHA over the **sorted set of descriptor-content hashes**, and
/// nothing else. Descriptors are read as a capped prefix with per-copy
/// watermarks stripped (`# …` atlas comments, the trailing `"_wmrk"` json
/// field). Two folders that are the same mod produce the same key regardless of
/// naming or surrounding files; the result is also deterministic (sorted set).
pub fn mod_descriptor_key<P: AsRef<Path>>(dir: P) -> io::Result<String> {
    let dir = dir.as_ref();

    let entries: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // One content hash per descriptor file; sorted so order never matters.
    let mut descriptor_hashes: Vec<String> = Vec::new();

    for entry in entries {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };

        if is_junk_file(file_name) || is_archive(path) {
            continue;
        }

        if is_descriptor_file(file_name) {
            let raw = read_descriptor_prefix(path)?;
            let normalized = normalize_descriptor(file_name, &raw);
            let mut fh = Sha256::new();
            fh.update(&normalized);
            descriptor_hashes.push(format!("{:x}", fh.finalize()));
        }
    }

    descriptor_hashes.sort();

    let mut hasher = Sha256::new();
    for h in descriptor_hashes {
        hasher.update(h.as_bytes());
        hasher.update(b"\n");
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// A texture/binary **signature** for tie-breaking between candidates that share
/// a descriptor key. Some mods ship multiple variants with *identical*
/// descriptors that differ only in their textures (e.g. an original vs. a
/// "redit" edition). When several staging mods share one descriptor key, the
/// caller ranks candidates by how much of this signature overlaps the active
/// mod's, so the closest texture set wins.
///
/// Returns a sorted list of `(lowercased file name, byte size)` for every
/// non-descriptor, non-junk, non-archive file. File **names** are used (not
/// relative paths) so a differing folder layout doesn't matter, and sizes give a
/// cheap, read-free discriminator.
pub fn mod_texture_signature<P: AsRef<Path>>(dir: P) -> io::Result<Vec<(String, u64)>> {
    let dir = dir.as_ref();

    let entries: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let mut sig: Vec<(String, u64)> = Vec::new();

    for entry in entries {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();

        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n,
            None => continue,
        };

        if is_junk_file(file_name) || is_archive(path) || is_descriptor_file(file_name) {
            continue;
        }

        let len = entry.metadata().map(|m| m.len()).unwrap_or(0);
        sig.push((file_name.to_ascii_lowercase(), len));
    }

    sig.sort();
    Ok(sig)
}

/// Score how well two texture signatures overlap: the size of their multiset
/// intersection on `(name, size)`. Higher means a closer texture set. Used only
/// to rank staging candidates that already share a descriptor key.
pub fn texture_signature_overlap(a: &[(String, u64)], b: &[(String, u64)]) -> usize {
    use std::collections::HashMap;
    let mut counts: HashMap<&(String, u64), i32> = HashMap::new();
    for item in a {
        *counts.entry(item).or_insert(0) += 1;
    }
    let mut common = 0usize;
    for item in b {
        if let Some(c) = counts.get_mut(item) {
            if *c > 0 {
                *c -= 1;
                common += 1;
            }
        }
    }
    common
}

pub fn ensure_dir_exists(path: &PathBuf) -> Result<(), std::io::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn cleanup_empty_dirs(game_path: &PathBuf, file_list: &[String]) {
    let mut dirs: Vec<PathBuf> = Vec::new();
    for f in file_list {
        let full = game_path.join(f);
        let mut current = full.parent().map(|p| p.to_path_buf());
        while let Some(dir) = current {
            if !dir.starts_with(game_path) || dir == *game_path {
                break;
            }
            dirs.push(dir.clone());
            current = dir.parent().map(|p| p.to_path_buf());
        }
    }
    dirs.sort();
    dirs.dedup();
    dirs.sort_by(|a, b| b.components().count().cmp(&a.components().count()));

    for dir in &dirs {
        if dir.exists() && dir.is_dir() {
            let _ = fs::remove_dir(dir);
        }
    }
}