//! Settings management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Browser settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Privacy settings
    pub privacy: PrivacySettings,
    /// Appearance settings
    pub appearance: AppearanceSettings,
    /// General settings
    pub general: GeneralSettings,
}

/// Privacy-related settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Enable tracking protection
    pub tracking_protection: bool,
    /// Clear data on exit
    pub clear_on_exit: bool,
    /// Enable Do Not Track
    pub do_not_track: bool,
    /// Block third-party cookies
    pub block_third_party_cookies: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            tracking_protection: true,
            clear_on_exit: false,
            do_not_track: true,
            block_third_party_cookies: true,
        }
    }
}

/// Appearance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    /// Current theme name
    pub theme: String,
    /// Font size
    pub font_size: u16,
    /// Show bookmarks bar
    pub show_bookmarks_bar: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: "Dark".to_string(),
            font_size: 14,
            show_bookmarks_bar: true,
        }
    }
}

/// General settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    /// Default homepage URL
    pub homepage: String,
    /// Search engine
    pub search_engine: String,
    /// Download directory
    pub download_directory: String,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        // Use platform-appropriate download directory
        let download_dir = dirs::download_dir()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| {
                if cfg!(target_os = "windows") {
                    "C:\\Users\\Downloads".to_string()
                } else {
                    "/tmp/downloads".to_string()
                }
            });

        Self {
            homepage: "about:blank".to_string(),
            search_engine: "DuckDuckGo".to_string(),
            download_directory: download_dir,
        }
    }
}

impl Settings {
    /// Create new settings with defaults
    pub fn new() -> Self {
        Self {
            privacy: PrivacySettings::default(),
            appearance: AppearanceSettings::default(),
            general: GeneralSettings::default(),
        }
    }

    /// Load settings from a file
    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            let settings: Settings = toml::from_str(&contents)?;
            Ok(settings)
        } else {
            Ok(Self::new())
        }
    }

    /// Save settings to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_settings_default() {
        let settings = Settings::default();
        assert!(settings.privacy.tracking_protection);
        assert_eq!(settings.appearance.theme, "Dark");
    }

    #[test]
    fn test_settings_save_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let settings = Settings::default();

        settings.save(temp_file.path()).unwrap();
        let loaded = Settings::load(temp_file.path()).unwrap();

        assert_eq!(
            settings.privacy.tracking_protection,
            loaded.privacy.tracking_protection
        );
        assert_eq!(settings.appearance.theme, loaded.appearance.theme);
    }
}
