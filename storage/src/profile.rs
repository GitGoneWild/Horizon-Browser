//! User profile management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// User profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Profile ID
    pub id: String,
    /// Profile name
    pub name: String,
    /// Profile path
    #[serde(skip)]
    pub path: PathBuf,
}

impl Profile {
    /// Create a new profile
    pub fn new(id: impl Into<String>, name: impl Into<String>, path: PathBuf) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            path,
        }
    }

    /// Get the profile ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the profile name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the profile path
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Profile manager
pub struct ProfileManager {
    profiles_dir: PathBuf,
    profiles: Vec<Profile>,
    active_profile: Option<String>,
}

impl ProfileManager {
    /// Create a new profile manager
    pub fn new(profiles_dir: PathBuf) -> Result<Self> {
        if !profiles_dir.exists() {
            std::fs::create_dir_all(&profiles_dir)?;
        }

        Ok(Self {
            profiles_dir,
            profiles: Vec::new(),
            active_profile: None,
        })
    }

    /// Create a new profile
    pub fn create_profile(&mut self, name: impl Into<String>) -> Result<Profile> {
        let id = uuid::Uuid::new_v4().to_string();
        let name = name.into();
        let path = self.profiles_dir.join(&id);

        std::fs::create_dir_all(&path)?;

        let profile = Profile::new(id.clone(), name, path);
        self.profiles.push(profile.clone());

        if self.active_profile.is_none() {
            self.active_profile = Some(id);
        }

        Ok(profile)
    }

    /// Get all profiles
    pub fn profiles(&self) -> &[Profile] {
        &self.profiles
    }

    /// Get the active profile
    pub fn active_profile(&self) -> Option<&Profile> {
        self.active_profile
            .as_ref()
            .and_then(|id| self.profiles.iter().find(|p| p.id == *id))
    }

    /// Set the active profile
    pub fn set_active_profile(&mut self, id: &str) -> Result<()> {
        if self.profiles.iter().any(|p| p.id == id) {
            self.active_profile = Some(id.to_string());
            Ok(())
        } else {
            anyhow::bail!("Profile not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_profile_creation() {
        let temp_dir = TempDir::new().unwrap();
        let profile = Profile::new("test-id", "Test Profile", temp_dir.path().to_path_buf());
        assert_eq!(profile.id(), "test-id");
        assert_eq!(profile.name(), "Test Profile");
    }

    #[test]
    fn test_profile_manager() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = ProfileManager::new(temp_dir.path().to_path_buf()).unwrap();

        let _profile = manager.create_profile("Default").unwrap();
        assert_eq!(manager.profiles().len(), 1);
        assert!(manager.active_profile().is_some());
        assert_eq!(manager.active_profile().unwrap().name(), "Default");
    }
}
