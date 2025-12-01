//! Settings management

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Browser settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Privacy settings
    #[serde(default)]
    pub privacy: PrivacySettings,
    /// Appearance settings
    #[serde(default)]
    pub appearance: AppearanceSettings,
    /// General settings
    #[serde(default)]
    pub general: GeneralSettings,
    /// Advanced settings
    #[serde(default)]
    pub advanced: AdvancedSettings,
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
    /// Enable HTTPS-only mode
    #[serde(default)]
    pub https_only: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            tracking_protection: true,
            clear_on_exit: false,
            do_not_track: true,
            block_third_party_cookies: true,
            https_only: false,
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
    /// Restore tabs on startup
    pub restore_tabs_on_startup: bool,
    /// Ask where to save files
    pub ask_where_to_save: bool,
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
            homepage: "about:home".to_string(),
            search_engine: "DuckDuckGo".to_string(),
            download_directory: download_dir,
            restore_tabs_on_startup: false,
            ask_where_to_save: true,
        }
    }
}

/// Advanced settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSettings {
    /// Enable developer tools
    pub enable_developer_tools: bool,
    /// Hardware acceleration
    pub hardware_acceleration: bool,
    /// Enable experimental features
    pub experimental_features: bool,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            enable_developer_tools: false,
            hardware_acceleration: true,
            experimental_features: false,
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
            advanced: AdvancedSettings::default(),
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

    #[test]
    fn test_settings_backward_compatibility() {
        // Test loading old settings file without https_only field
        let old_settings_toml = r#"
[privacy]
tracking_protection = true
clear_on_exit = false
do_not_track = true
block_third_party_cookies = true

[appearance]
theme = "Dark"
font_size = 14
show_bookmarks_bar = true

[general]
homepage = "about:home"
search_engine = "DuckDuckGo"
download_directory = "/tmp/downloads"
restore_tabs_on_startup = false
ask_where_to_save = true

[advanced]
enable_developer_tools = false
hardware_acceleration = true
experimental_features = false
"#;

        let settings: Settings = toml::from_str(old_settings_toml).unwrap();
        
        // Verify that https_only defaults to false when not present
        assert!(!settings.privacy.https_only);
        assert!(settings.privacy.tracking_protection);
        assert!(settings.privacy.do_not_track);
    }
}
