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
    /// Network settings
    pub network: NetworkSettings,
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
#[derive(Default)]
pub enum SettingsPanel {
    #[default]
    General,
    Privacy,
    Appearance,
    Network,
    Passwords,
    Extensions,
    Downloads,
    Advanced,
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

/// Network settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    /// DNS provider
    pub dns_provider: DnsProvider,
    /// Custom DNS servers (comma-separated)
    pub custom_dns_servers: String,
    /// Enable VPN
    pub vpn_enabled: bool,
    /// VPN configuration type
    pub vpn_type: VpnType,
    /// Proxy host (for proxy-based VPN)
    pub proxy_host: String,
    /// Proxy port
    pub proxy_port: u16,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            dns_provider: DnsProvider::System,
            custom_dns_servers: String::new(),
            vpn_enabled: false,
            vpn_type: VpnType::Proxy,
            proxy_host: String::new(),
            proxy_port: 8080,
        }
    }
}

/// DNS provider options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsProvider {
    System,
    Google,
    Cloudflare,
    Quad9,
    Custom,
}

impl DnsProvider {
    pub fn name(&self) -> &str {
        match self {
            Self::System => "System Default",
            Self::Google => "Google DNS (8.8.8.8)",
            Self::Cloudflare => "Cloudflare DNS (1.1.1.1)",
            Self::Quad9 => "Quad9 DNS (9.9.9.9)",
            Self::Custom => "Custom DNS",
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::System, Self::Google, Self::Cloudflare, Self::Quad9, Self::Custom]
    }
}

/// VPN type options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VpnType {
    Proxy,
    Socks5,
    OpenVpn,
}

impl VpnType {
    pub fn name(&self) -> &str {
        match self {
            Self::Proxy => "HTTP/HTTPS Proxy",
            Self::Socks5 => "SOCKS5 Proxy",
            Self::OpenVpn => "OpenVPN (.ovpn file)",
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Proxy, Self::Socks5, Self::OpenVpn]
    }
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
            network: NetworkSettings::default(),
            downloads: DownloadsSettings::default(),
            advanced: AdvancedSettings::default(),
            selected_panel: SettingsPanel::default(),
        }
    }

    /// Get the settings file path
    fn get_settings_path() -> std::path::PathBuf {
        let data_dir = if cfg!(target_os = "windows") {
            dirs::data_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("C:\\ProgramData"))
                .join("Horizon")
        } else if cfg!(target_os = "macos") {
            dirs::data_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
                .join("Horizon")
        } else {
            dirs::data_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
                .join("Horizon")
        };

        // Ensure directory exists
        if !data_dir.exists() {
            let _ = std::fs::create_dir_all(&data_dir);
        }

        data_dir.join("settings.toml")
    }

    /// Load settings from storage
    pub fn load() -> Self {
        let settings_path = Self::get_settings_path();
        
        if settings_path.exists() {
            match horizon_storage::settings::Settings::load(&settings_path) {
                Ok(storage_settings) => {
                    tracing::info!("Settings loaded from {:?}", settings_path);
                    Self::from_storage(&storage_settings)
                }
                Err(e) => {
                    tracing::warn!("Failed to load settings: {}. Using defaults.", e);
                    Self::new()
                }
            }
        } else {
            tracing::info!("No settings file found. Using defaults.");
            Self::new()
        }
    }

    /// Convert from storage settings
    pub fn from_storage(storage_settings: &horizon_storage::settings::Settings) -> Self {
        let search_engine = match storage_settings.general.search_engine.as_str() {
            "Google" => SearchEngine::Google,
            "Bing" => SearchEngine::Bing,
            "Brave" => SearchEngine::Brave,
            _ => SearchEngine::DuckDuckGo,
        };

        let theme = match storage_settings.appearance.theme.as_str() {
            "Light" => Theme::Light,
            _ => Theme::Dark,
        };

        Self {
            general: GeneralSettings {
                homepage: storage_settings.general.homepage.clone(),
                search_engine,
                restore_tabs_on_startup: storage_settings.general.restore_tabs_on_startup,
            },
            privacy: PrivacySettings {
                tracking_protection: storage_settings.privacy.tracking_protection,
                do_not_track: storage_settings.privacy.do_not_track,
                block_third_party_cookies: storage_settings.privacy.block_third_party_cookies,
                clear_data_on_exit: storage_settings.privacy.clear_on_exit,
                https_only: storage_settings.privacy.https_only,
            },
            appearance: AppearanceSettings {
                theme,
                font_size: storage_settings.appearance.font_size,
                show_bookmarks_bar: storage_settings.appearance.show_bookmarks_bar,
            },
            network: NetworkSettings::default(), // Use defaults for new settings
            downloads: DownloadsSettings {
                download_directory: storage_settings.general.download_directory.clone(),
                ask_where_to_save: storage_settings.general.ask_where_to_save,
            },
            advanced: AdvancedSettings {
                enable_developer_tools: storage_settings.advanced.enable_developer_tools,
                hardware_acceleration: storage_settings.advanced.hardware_acceleration,
                experimental_features: storage_settings.advanced.experimental_features,
            },
            selected_panel: SettingsPanel::default(),
        }
    }

    /// Convert to storage settings
    pub fn to_storage(&self) -> horizon_storage::settings::Settings {
        horizon_storage::settings::Settings {
            general: horizon_storage::settings::GeneralSettings {
                homepage: self.general.homepage.clone(),
                search_engine: self.general.search_engine.name().to_string(),
                download_directory: self.downloads.download_directory.clone(),
                restore_tabs_on_startup: self.general.restore_tabs_on_startup,
                ask_where_to_save: self.downloads.ask_where_to_save,
            },
            privacy: horizon_storage::settings::PrivacySettings {
                tracking_protection: self.privacy.tracking_protection,
                do_not_track: self.privacy.do_not_track,
                block_third_party_cookies: self.privacy.block_third_party_cookies,
                clear_on_exit: self.privacy.clear_data_on_exit,
                https_only: self.privacy.https_only,
            },
            appearance: horizon_storage::settings::AppearanceSettings {
                theme: self.appearance.theme.name().to_string(),
                font_size: self.appearance.font_size,
                show_bookmarks_bar: self.appearance.show_bookmarks_bar,
            },
            advanced: horizon_storage::settings::AdvancedSettings {
                enable_developer_tools: self.advanced.enable_developer_tools,
                hardware_acceleration: self.advanced.hardware_acceleration,
                experimental_features: self.advanced.experimental_features,
            },
        }
    }

    /// Save settings to storage
    pub fn save(&self) {
        let settings_path = Self::get_settings_path();
        let storage_settings = self.to_storage();
        
        match storage_settings.save(&settings_path) {
            Ok(()) => {
                tracing::info!("Settings saved successfully to {:?}", settings_path);
            }
            Err(e) => {
                tracing::error!("Failed to save settings: {}", e);
            }
        }
    }
}

impl Default for SettingsUI {
    fn default() -> Self {
        Self::new()
    }
}
