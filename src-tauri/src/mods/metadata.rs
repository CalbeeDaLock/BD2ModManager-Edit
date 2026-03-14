use std::{collections::HashMap, fs, path::PathBuf};

use log::{info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
}

pub struct ModMetadataStore {
    path: PathBuf,
    data: HashMap<String, ModMetadata>,
}

impl ModMetadataStore {
    pub fn new(path: PathBuf) -> Self {
        let mut store = Self {
            path,
            data: HashMap::new(),
        };
        store.load();
        store
    }

    fn load(&mut self) {
        if self.path.exists() {
            match fs::read_to_string(&self.path) {
                Ok(contents) => match serde_json::from_str(&contents) {
                    Ok(data) => {
                        self.data = data;
                        info!("Loaded mod metadata from {:?}", self.path);
                    }
                    Err(e) => {
                        warn!("Failed to parse mod metadata: {:?}", e);
                    }
                },
                Err(e) => {
                    warn!("Failed to read mod metadata file: {:?}", e);
                }
            }
        }
    }

    fn save(&self) {
        match serde_json::to_string_pretty(&self.data) {
            Ok(json) => {
                if let Err(e) = fs::write(&self.path, json) {
                    warn!("Failed to save mod metadata: {:?}", e);
                }
            }
            Err(e) => {
                warn!("Failed to serialize mod metadata: {:?}", e);
            }
        }
    }

    pub fn get_author(&self, mod_name: &str) -> Option<String> {
        self.data.get(mod_name).and_then(|m| m.author.clone())
    }

    pub fn set_author(&mut self, mod_name: String, author: Option<String>) {
        let entry = self.data.entry(mod_name).or_default();
        entry.author = author;
        self.save();
    }

    pub fn apply_to_mods(&self, mods: &mut HashMap<String, super::BD2Mod>) {
        for (name, metadata) in &self.data {
            if let Some(bd2mod) = mods.get_mut(name) {
                bd2mod.author = metadata.author.clone();
            }
        }
    }

    pub fn rename(&mut self, old_name: &str, new_name: &str) {
        if let Some(metadata) = self.data.remove(old_name) {
            self.data.insert(new_name.to_string(), metadata);
            self.save();
        }
    }
}
