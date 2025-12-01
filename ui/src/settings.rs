//! Settings UI module for Horizon Browser

use serde::{Deserialize, Serialize};

/// Settings UI state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsUI {
    /// General settings
    pub general: GeneralSettings,
    /// Privacy settings
    pub privacy: PrivacySettings,
    /// Appearance settings
    pub appearance: AppearanceSettings,
    /// Downloads settings
    pub downloads: DownloadsSettings,
    /// Advanced settings
    pub advanced: AdvancedSettings,
    /// Currently selected panel
    #[serde(skip)]
    pub selected_panel: SettingsPanel,
}

/// Settings panel enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsPanel {
    General,
    Privacy,
    Appearance,
    Downloads,
    Advanced,
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::General
    }
}

/// General settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    /// Homepage URL
    pub homepage: String,
    /// Search engine
    pub search_engine: SearchEngine,
    /// Restore tabs on startup
    pub restore_tabs_on_startup: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            homepage: "about:home".to_string(),
            search_engine: SearchEngine::DuckDuckGo,
            restore_tabs_on_startup: false,
        }
    }
}

/// Search engine options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchEngine {
    DuckDuckGo,
    Google,
    Bing,
    Brave,
}

impl SearchEngine {
    pub fn name(&self) -> &str {
        match self {
            Self::DuckDuckGo => "DuckDuckGo",
            Self::Google => "Google",
            Self::Bing => "Bing",
            Self::Brave => "Brave",
        }
    }

    pub fn search_url(&self, query: &str) -> String {
        let encoded_query = urlencoding::encode(query);
        match self {
            Self::DuckDuckGo => format!("https://duckduckgo.com/?q={}", encoded_query),
            Self::Google => format!("https://www.google.com/search?q={}", encoded_query),
            Self::Bing => format!("https://www.bing.com/search?q={}", encoded_query),
            Self::Brave => format!("https://search.brave.com/search?q={}", encoded_query),
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::DuckDuckGo, Self::Google, Self::Bing, Self::Brave]
    }
}

/// Privacy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Enable tracking protection
    pub tracking_protection: bool,
    /// Enable Do Not Track
    pub do_not_track: bool,
    /// Block third-party cookies
    pub block_third_party_cookies: bool,
    /// Clear data on exit
    pub clear_data_on_exit: bool,
    /// Enable HTTPS-only mode
    pub https_only: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            tracking_protection: true,
            do_not_track: true,
            block_third_party_cookies: true,
            clear_data_on_exit: false,
            https_only: false,
        }
    }
}

/// Appearance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceSettings {
    /// Theme selection
    pub theme: Theme,
    /// Font size
    pub font_size: u16,
    /// Show bookmarks bar
    pub show_bookmarks_bar: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            font_size: 14,
            show_bookmarks_bar: false,
        }
    }
}

/// Theme options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn name(&self) -> &str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Dark, Self::Light]
    }
}

/// Downloads settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadsSettings {
    /// Default download directory
    pub download_directory: String,
    /// Always ask where to save files
    pub ask_where_to_save: bool,
}

impl Default for DownloadsSettings {
    fn default() -> Self {
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
            download_directory: download_dir,
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

impl SettingsUI {
    /// Create new settings with defaults
    pub fn new() -> Self {
        Self {
            general: GeneralSettings::default(),
            privacy: PrivacySettings::default(),
            appearance: AppearanceSettings::default(),
            downloads: DownloadsSettings::default(),
            advanced: AdvancedSettings::default(),
            selected_panel: SettingsPanel::default(),
        }
    }

    /// Load settings from storage
    pub fn load() -> Self {
        // TODO: Load from actual storage
        Self::new()
    }

    /// Save settings to storage
    pub fn save(&self) {
        // TODO: Save to actual storage
        tracing::info!("Settings saved (not yet persisted to disk)");
    }
}

impl Default for SettingsUI {
    fn default() -> Self {
        Self::new()
    }
}
