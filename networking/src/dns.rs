//! DNS resolution module with configurable DNS providers

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

/// DNS provider options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DnsProvider {
    /// Use system DNS resolver (default)
    #[default]
    System,
    /// Google Public DNS (8.8.8.8, 8.8.4.4)
    Google,
    /// Cloudflare DNS (1.1.1.1, 1.0.0.1)
    Cloudflare,
    /// Quad9 DNS (9.9.9.9, 149.112.112.112)
    Quad9,
    /// Custom DNS servers
    Custom,
}

impl DnsProvider {
    /// Get the name of the DNS provider
    pub fn name(&self) -> &str {
        match self {
            Self::System => "System Default",
            Self::Google => "Google DNS",
            Self::Cloudflare => "Cloudflare DNS",
            Self::Quad9 => "Quad9 DNS",
            Self::Custom => "Custom DNS",
        }
    }

    /// Get the DNS server addresses for this provider
    pub fn servers(&self) -> Vec<IpAddr> {
        match self {
            Self::System => vec![],
            Self::Google => vec![
                "8.8.8.8".parse().unwrap(),
                "8.8.4.4".parse().unwrap(),
            ],
            Self::Cloudflare => vec![
                "1.1.1.1".parse().unwrap(),
                "1.0.0.1".parse().unwrap(),
            ],
            Self::Quad9 => vec![
                "9.9.9.9".parse().unwrap(),
                "149.112.112.112".parse().unwrap(),
            ],
            Self::Custom => vec![],
        }
    }

    /// Get all available DNS providers
    pub fn all() -> &'static [Self] {
        &[
            Self::System,
            Self::Google,
            Self::Cloudflare,
            Self::Quad9,
            Self::Custom,
        ]
    }
}

/// DNS resolver configuration
#[derive(Debug, Clone, Default)]
pub struct DnsConfig {
    /// DNS provider to use
    pub provider: DnsProvider,
    /// Custom DNS servers (used when provider is Custom)
    pub custom_servers: Vec<IpAddr>,
}

/// DNS resolver with configurable providers
pub struct DnsResolver {
    config: DnsConfig,
}

impl DnsResolver {
    /// Create a new DNS resolver with default configuration
    pub fn new() -> Self {
        Self {
            config: DnsConfig::default(),
        }
    }

    /// Create a DNS resolver with custom configuration
    pub fn with_config(config: DnsConfig) -> Self {
        Self { config }
    }

    /// Get the current DNS configuration
    pub fn config(&self) -> &DnsConfig {
        &self.config
    }

    /// Update the DNS configuration
    pub fn set_config(&mut self, config: DnsConfig) {
        tracing::info!(
            "DNS configuration updated to: {}",
            config.provider.name()
        );
        self.config = config;
    }

    /// Set DNS provider
    pub fn set_provider(&mut self, provider: DnsProvider) {
        self.config.provider = provider;
        tracing::info!("DNS provider set to: {}", provider.name());
    }

    /// Set custom DNS servers
    pub fn set_custom_servers(&mut self, servers: Vec<IpAddr>) {
        tracing::info!("Custom DNS servers set: {:?}", servers);
        self.config.custom_servers = servers;
    }

    /// Get the active DNS servers
    pub fn active_servers(&self) -> Vec<IpAddr> {
        match self.config.provider {
            DnsProvider::Custom => self.config.custom_servers.clone(),
            _ => self.config.provider.servers(),
        }
    }

    /// Resolve a hostname to IP addresses
    pub async fn resolve(&self, hostname: &str) -> Result<Vec<IpAddr>> {
        tracing::debug!(
            "Resolving DNS for {} using {}",
            hostname,
            self.config.provider.name()
        );
        
        // Note: In a full implementation, this would use the configured DNS servers
        // For now, we use the system resolver regardless of configuration
        // A complete implementation would use libraries like trust-dns-resolver
        let addrs: Vec<IpAddr> = tokio::net::lookup_host(format!("{}:80", hostname))
            .await?
            .map(|addr| addr.ip())
            .collect();
        
        tracing::debug!("Resolved {} to {:?}", hostname, addrs);
        Ok(addrs)
    }
}

impl Default for DnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_resolver_creation() {
        let _resolver = DnsResolver::new();
    }

    #[tokio::test]
    async fn test_dns_resolution() {
        let resolver = DnsResolver::new();
        // Test with localhost which should always resolve
        let result = resolver.resolve("localhost").await;
        assert!(result.is_ok());
    }
}
