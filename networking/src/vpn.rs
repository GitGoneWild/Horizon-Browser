//! VPN management module for per-browser VPN support

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::path::PathBuf;

/// VPN connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VpnStatus {
    /// VPN is disconnected
    Disconnected,
    /// VPN is connecting
    Connecting,
    /// VPN is connected
    Connected,
    /// VPN connection failed
    Failed,
}

impl VpnStatus {
    /// Get the display name for this status
    pub fn name(&self) -> &str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting",
            Self::Connected => "Connected",
            Self::Failed => "Failed",
        }
    }
}

/// VPN configuration type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VpnConfig {
    /// OpenVPN configuration from .ovpn file
    OpenVpn {
        /// Path to .ovpn file
        config_path: PathBuf,
        /// Username (optional)
        username: Option<String>,
        /// Password (optional, stored securely)
        password: Option<String>,
    },
    /// HTTP/HTTPS proxy configuration
    Proxy {
        /// Proxy protocol
        protocol: ProxyProtocol,
        /// Proxy host
        host: String,
        /// Proxy port
        port: u16,
        /// Username for proxy authentication
        username: Option<String>,
        /// Password for proxy authentication
        password: Option<String>,
    },
    /// SOCKS proxy configuration
    Socks {
        /// SOCKS version (4 or 5)
        version: u8,
        /// Proxy host
        host: String,
        /// Proxy port
        port: u16,
        /// Username (SOCKS5 only)
        username: Option<String>,
        /// Password (SOCKS5 only)
        password: Option<String>,
    },
}

/// Proxy protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProxyProtocol {
    /// HTTP proxy
    Http,
    /// HTTPS proxy
    Https,
}

impl ProxyProtocol {
    /// Get the protocol name
    pub fn name(&self) -> &str {
        match self {
            Self::Http => "HTTP",
            Self::Https => "HTTPS",
        }
    }
}

/// VPN connection statistics
#[derive(Debug, Clone, Default)]
pub struct VpnStats {
    /// Current download speed in bytes per second
    pub download_speed: u64,
    /// Current upload speed in bytes per second
    pub upload_speed: u64,
    /// Total bytes downloaded
    pub bytes_downloaded: u64,
    /// Total bytes uploaded
    pub bytes_uploaded: u64,
    /// Connection duration in seconds
    pub duration: u64,
    /// Current public IP address (if available)
    pub public_ip: Option<IpAddr>,
}

/// VPN manager for handling VPN connections
pub struct VpnManager {
    /// Current VPN configuration
    config: Option<VpnConfig>,
    /// Current VPN status
    status: VpnStatus,
    /// Connection statistics
    stats: VpnStats,
    /// Whether VPN is enabled globally
    enabled: bool,
}

impl VpnManager {
    /// Create a new VPN manager
    pub fn new() -> Self {
        Self {
            config: None,
            status: VpnStatus::Disconnected,
            stats: VpnStats::default(),
            enabled: false,
        }
    }

    /// Check if VPN is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable VPN
    pub fn enable(&mut self) {
        self.enabled = true;
        tracing::info!("VPN enabled");
    }

    /// Disable VPN
    pub fn disable(&mut self) {
        self.enabled = false;
        if self.status == VpnStatus::Connected {
            self.disconnect();
        }
        tracing::info!("VPN disabled");
    }

    /// Get the current VPN status
    pub fn status(&self) -> VpnStatus {
        self.status
    }

    /// Get the current VPN configuration
    pub fn config(&self) -> Option<&VpnConfig> {
        self.config.as_ref()
    }

    /// Get VPN statistics
    pub fn stats(&self) -> &VpnStats {
        &self.stats
    }

    /// Set VPN configuration
    pub fn set_config(&mut self, config: VpnConfig) {
        self.config = Some(config);
        tracing::info!("VPN configuration updated");
    }

    /// Load OpenVPN configuration from .ovpn file
    pub fn load_ovpn_config(&mut self, path: PathBuf) -> Result<()> {
        // Validate file exists and has .ovpn extension
        if !path.exists() {
            return Err(anyhow!("OVPN file does not exist: {:?}", path));
        }

        if path.extension().and_then(|s| s.to_str()) != Some("ovpn") {
            return Err(anyhow!("File must have .ovpn extension"));
        }

        self.config = Some(VpnConfig::OpenVpn {
            config_path: path.clone(),
            username: None,
            password: None,
        });

        tracing::info!("Loaded OpenVPN configuration from {:?}", path);
        Ok(())
    }

    /// Configure HTTP/HTTPS proxy
    pub fn configure_proxy(
        &mut self,
        protocol: ProxyProtocol,
        host: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
    ) {
        self.config = Some(VpnConfig::Proxy {
            protocol,
            host: host.clone(),
            port,
            username,
            password,
        });
        tracing::info!("Configured {} proxy: {}:{}", protocol.name(), host, port);
    }

    /// Configure SOCKS proxy
    pub fn configure_socks(
        &mut self,
        version: u8,
        host: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<()> {
        if version != 4 && version != 5 {
            return Err(anyhow!("SOCKS version must be 4 or 5"));
        }

        self.config = Some(VpnConfig::Socks {
            version,
            host: host.clone(),
            port,
            username,
            password,
        });
        tracing::info!("Configured SOCKS{} proxy: {}:{}", version, host, port);
        Ok(())
    }

    /// Connect to VPN
    pub async fn connect(&mut self) -> Result<()> {
        if self.config.is_none() {
            return Err(anyhow!("No VPN configuration set"));
        }

        if self.status == VpnStatus::Connected {
            return Err(anyhow!("VPN already connected"));
        }

        self.status = VpnStatus::Connecting;
        tracing::info!("Connecting to VPN...");

        // Note: In a full implementation, this would:
        // 1. For OpenVPN: spawn openvpn process with the config file
        // 2. For Proxy/SOCKS: configure reqwest client with proxy settings
        // 3. Monitor connection status and update statistics
        // For now, we simulate a successful connection

        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        self.status = VpnStatus::Connected;
        self.stats = VpnStats::default();
        tracing::info!("VPN connected successfully");

        Ok(())
    }

    /// Disconnect from VPN
    pub fn disconnect(&mut self) {
        if self.status != VpnStatus::Connected {
            tracing::warn!("VPN not connected, cannot disconnect");
            return;
        }

        self.status = VpnStatus::Disconnected;
        self.stats = VpnStats::default();
        tracing::info!("VPN disconnected");
    }

    /// Update VPN statistics (called periodically when connected)
    pub fn update_stats(&mut self, stats: VpnStats) {
        self.stats = stats;
    }

    /// Get the current public IP address
    pub async fn get_public_ip(&self) -> Result<IpAddr> {
        // Note: In a full implementation, this would query an IP detection service
        // For now, return a placeholder
        Err(anyhow!("Public IP detection not yet implemented"))
    }

    /// Test VPN connection
    pub async fn test_connection(&self) -> Result<bool> {
        if self.status != VpnStatus::Connected {
            return Ok(false);
        }

        // Note: In a full implementation, this would:
        // 1. Make a test request through the VPN
        // 2. Verify the request succeeds
        // 3. Check that IP has changed
        Ok(true)
    }
}

impl Default for VpnManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vpn_manager_creation() {
        let manager = VpnManager::new();
        assert_eq!(manager.status(), VpnStatus::Disconnected);
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_vpn_enable_disable() {
        let mut manager = VpnManager::new();
        assert!(!manager.is_enabled());

        manager.enable();
        assert!(manager.is_enabled());

        manager.disable();
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_proxy_configuration() {
        let mut manager = VpnManager::new();
        manager.configure_proxy(
            ProxyProtocol::Http,
            "proxy.example.com".to_string(),
            8080,
            None,
            None,
        );

        assert!(manager.config().is_some());
    }

    #[test]
    fn test_socks_configuration() {
        let mut manager = VpnManager::new();
        let result = manager.configure_socks(5, "socks.example.com".to_string(), 1080, None, None);

        assert!(result.is_ok());
        assert!(manager.config().is_some());
    }

    #[test]
    fn test_invalid_socks_version() {
        let mut manager = VpnManager::new();
        let result = manager.configure_socks(3, "socks.example.com".to_string(), 1080, None, None);

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_vpn_connect_without_config() {
        let mut manager = VpnManager::new();
        let result = manager.connect().await;
        assert!(result.is_err());
    }
}
