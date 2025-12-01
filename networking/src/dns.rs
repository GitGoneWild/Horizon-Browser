//! DNS resolution module

use anyhow::Result;
use std::net::IpAddr;

/// DNS resolver
pub struct DnsResolver;

impl DnsResolver {
    /// Create a new DNS resolver
    pub fn new() -> Self {
        Self
    }

    /// Resolve a hostname to IP addresses
    pub async fn resolve(&self, hostname: &str) -> Result<Vec<IpAddr>> {
        tracing::debug!("Resolving DNS for {}", hostname);
        // Placeholder: use system DNS resolver
        let addrs: Vec<IpAddr> = tokio::net::lookup_host(format!("{}:80", hostname))
            .await?
            .map(|addr| addr.ip())
            .collect();
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
