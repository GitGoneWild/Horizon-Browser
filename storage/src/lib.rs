//! # Horizon Storage
//!
//! Storage layer for the Horizon Browser.
//! Provides user data storage, settings, profiles, and secure storage.

pub mod profile;
pub mod secure;
pub mod settings;
pub mod userdata;

use anyhow::Result;
use std::path::PathBuf;

/// Storage manager coordinates all storage operations
pub struct StorageManager {
    base_path: PathBuf,
    settings: settings::Settings,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new(base_path: PathBuf) -> Result<Self> {
        // Ensure base directory exists
        if !base_path.exists() {
            std::fs::create_dir_all(&base_path)?;
        }

        let settings = settings::Settings::load(&base_path.join("settings.toml"))?;

        Ok(Self {
            base_path,
            settings,
        })
    }

    /// Initialize the storage system
    pub fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing Storage Manager at {:?}", self.base_path);
        Ok(())
    }

    /// Get the base storage path
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    /// Get the settings
    pub fn settings(&self) -> &settings::Settings {
        &self.settings
    }

    /// Get mutable settings
    pub fn settings_mut(&mut self) -> &mut settings::Settings {
        &mut self.settings
    }

    /// Save settings to disk
    pub fn save_settings(&self) -> Result<()> {
        self.settings.save(&self.base_path.join("settings.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = StorageManager::new(temp_dir.path().to_path_buf());
        assert!(manager.is_ok());
    }

    #[test]
    fn test_storage_manager_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = StorageManager::new(temp_dir.path().to_path_buf()).unwrap();
        assert!(manager.initialize().is_ok());
    }
}
