use std::{
    collections::HashMap,
    fs::{create_dir, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use chrono::Utc;
use log::{info, warn};
use tempfile::NamedTempFile;

use crate::profiles::types::{Profile, ProfileError};

use std::fs::read_dir;

pub struct ProfileManager {
    directory: PathBuf,
    profiles: HashMap<String, Profile>,
    active_profile_id: Option<String>,
}

impl ProfileManager {
    pub fn new(directory: PathBuf) -> ProfileManager {
        Self {
            directory,
            profiles: HashMap::new(),
            active_profile_id: None,
        }
    }

    fn _create_default_profile(&mut self) {
        let default_profile = Profile {
            id: String::from("default"),
            name: String::from("default"),
            description: String::from("d3f4ult"),
            created_at: Utc::now().to_rfc3339(),
            active: false,
            enabled_mods: Vec::new(),
        };

        if let Ok(_created) = self._create_profile_json(&default_profile) {
            self.profiles
                .insert(default_profile.id.clone(), default_profile);
        };
    }

    fn _create_profile_json(
        &mut self,
        profile: &Profile,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut path = self.directory.clone();
        path.push(format!("{}.json", profile.id));

        info!("Creating Profile Json -> {:?}", path);

        let mut file = File::create(&path)?;
        serde_json::to_writer_pretty(&mut file, profile)?;
        file.flush()?;
        file.sync_all()?;

        info!("({:?}) Profile Json created successfully.", profile);

        Ok(())
    }

    pub fn save_profile(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        let mut path = self.directory.clone();
        path.push(format!("{}.json", profile.id));

        info!("Saving Profile Json -> {:?}", path);

        let mut file = NamedTempFile::new_in(path.parent().unwrap())?;
        serde_json::to_writer_pretty(&mut file, profile)?;
        file.flush()?;
        file.as_file().sync_all()?;
        file.persist(path)?;

        Ok(())
    }

    pub fn load_profiles(&mut self) -> Result<(), ProfileError> {
        if !self.directory.exists() {
            if let Err(error) = create_dir(&self.directory) {
                warn!(
                    "An error ocurred when creating the profiles directory: ({})",
                    error
                );
            };
        }

        if let Ok(entries) = read_dir(&self.directory) {
            entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    if let Some(ext) = entry.path().extension() {
                        ext == "json"
                    } else {
                        false
                    }
                })
                .for_each(|profile_path| {
                    let path = profile_path.path();

                    info!("Profile found {:?}", path);

                    if let Ok(data) = read_to_string(&path) {
                        match serde_json::from_str::<Profile>(&data) {
                            Ok(profile) => {
                                info!("Profile {} ({}) was loaded.", profile.name, profile.id);

                                if profile.active {
                                    self.active_profile_id = Some(profile.id.clone());
                                }

                                self.profiles.insert(profile.id.clone(), profile);
                            }
                            Err(error) => warn!("Failed to parse profile {:?}: {}", path, error),
                        }
                    }
                });
        }

        if self.profiles.get("default").is_none() {
            warn!("Default profile not found. Creating a new.");
            self._create_default_profile();
        }

        if self.active_profile_id.is_none() {
            if let Some(default_profile) = self.profiles.get_mut("default") {
                warn!("No active profile found. Setting 'Default' as active.");
                self.active_profile_id = Some(default_profile.id.clone());
                default_profile.active = true;
            }
        }

        Ok(())
    }

    pub fn create_profile(
        &mut self,
        name: String,
        description: Option<String>,
        _template_id: Option<String>,
    ) -> Result<String, ProfileError> {
        let desc = description.unwrap_or_default();

        let profile = Profile::new(name, desc);

        match self.save_profile(&profile) {
            Ok(()) => {
                let profile_id = profile.id.clone();
                self.profiles.insert(profile_id.clone(), profile);
                Ok(profile_id)
            }
            Err(error) => Err(ProfileError::SaveFailed(error)),
        }
    }

    pub fn delete_profile(&mut self, profile_id: String) -> Result<String, ProfileError> {
        if profile_id == "default" {
            return Err(ProfileError::CannotDeleteDefault);
        }

        let profile_deleted: bool;

        if (profile_id == self.active_profile_id.clone().unwrap_or_default())
            && (self.profiles.len() > 1)
        {
            // If the profile to be deleted is the active one, switch to default first
            self.set_active_profile("default".to_string())?;
        }

        if let Some(profile) = self.profiles.get(&profile_id) {
            let mut path = self.directory.clone();
            path.push(format!("{}.json", profile.id));

            if path.exists() {
                match std::fs::remove_file(&path) {
                    Ok(()) => {
                        profile_deleted = true;
                    }
                    Err(error) => return Err(ProfileError::DeleteFailed(error)),
                }
            } else {
                return Err(ProfileError::NotFound(profile_id));
            }
        } else {
            return Err(ProfileError::NotFound(profile_id));
        }

        if profile_deleted {
            self.profiles.remove(&profile_id);
            Ok(profile_id)
        } else {
            Err(ProfileError::NotFound(profile_id))
        }
    }

    pub fn edit_profile(
        &mut self,
        profile_id: String,
        name: String,
        description: Option<String>,
    ) -> Result<(), ProfileError> {
        if let Some(profile) = self.profiles.get_mut(&profile_id) {
            profile.name = name;
            profile.description = description.unwrap_or_default();
        } else {
            return Err(ProfileError::NotFound(profile_id));
        }

        match self.save_profile(self.profiles.get(&profile_id).unwrap()) {
            Ok(()) => Ok(()),
            Err(error) => Err(ProfileError::SaveFailed(error)),
        }
    }

    pub fn get_profiles(&self) -> Vec<Profile> {
        self.profiles.values().cloned().collect()
    }

    pub fn get_active_profile(&mut self) -> Option<&mut Profile> {
        self.active_profile_id
            .as_ref()
            .and_then(|id| self.profiles.get_mut(id))
    }

    pub fn set_active_profile(&mut self, profile_id: String) -> Result<String, ProfileError> {
        let mut changed_ids = Vec::new();
        for profile in self.profiles.values_mut() {
            if profile.active {
                profile.active = false;
                changed_ids.push(profile.id.clone());
            }
        }

        for profile_id in changed_ids {
            if let Some(profile) = self.profiles.get(&profile_id) {
                self.save_profile(&profile.clone()).ok();
            }
        }

        if self.profiles.contains_key(&profile_id) {
            {
                let profile = self.profiles.get_mut(&profile_id).unwrap();
                profile.active = true;
            }

            let profile = self.profiles.get(&profile_id).unwrap().clone();
            self.save_profile(&profile).ok();
            self.active_profile_id = Some(profile.id.clone());
            Ok(profile.id)
        } else {
            Err(ProfileError::NotFound(profile_id))
        }
    }

    pub fn save_active_profile(&mut self) -> Result<(), ProfileError> {
        if let Some(active_profile) = self.get_active_profile() {
            let profile_to_save = active_profile.clone();

            //drop(active_profile);

            self.save_profile(&profile_to_save)
                .map_err(|e| ProfileError::SaveFailed(e))?;
            Ok(())
        } else {
            Err(ProfileError::NotFound(
                "No active profile to save.".to_string(),
            ))
        }
    }
}
