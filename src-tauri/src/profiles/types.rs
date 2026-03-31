use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ProfileMod {
//     pub name: String,
//     pub enabled: bool
// }

#[derive(Debug)]
pub enum ProfileError {
    NotFound(String),
    InvalidName(String),
    DuplicateId(String),
    SaveFailed(Box<dyn std::error::Error>),
    CannotDeleteDefault,
    DeleteFailed(std::io::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub active: bool,
    pub enabled_mods: Vec<String>,
}

impl Profile {
    pub fn new(name: String, description: String, enabled_mods: Vec<String>, created_at: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name,
            description: description,
            created_at: created_at.unwrap_or_else(|| Utc::now().to_rfc3339()),
            enabled_mods,
            active: false,
        }
    }

    pub fn get_mod_state(&self, mod_name: &String) -> bool {
        // some mods was stored with '\\' instead of '/' in the name, so we need to check both, but now all mods are stored with '/'
        self.enabled_mods.contains(mod_name) || self.enabled_mods.contains(&mod_name.replace("/", "\\"))
    }

    pub fn set_mod_state(&mut self, mod_name: &String, enabled: bool) {
        if enabled {
            if !self.get_mod_state(mod_name) {
                self.enabled_mods.push(mod_name.to_string());
            }
        } else {
            self.enabled_mods.retain(|m| m != mod_name && m != &mod_name.replace("/", "\\"));
        }
    }
}
