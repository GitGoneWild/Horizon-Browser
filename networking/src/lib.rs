//! # Horizon Networking
//!
//! Networking layer for the Horizon Browser.
//! Provides HTTP client, DNS resolution, and request/response handling.

pub mod client;
pub mod dns;
pub mod request;
pub mod response;

use anyhow::Result;

/// Network manager coordinates all networking operations
pub struct NetworkManager {
    client: client::HttpClient,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: client::HttpClient::new()?,
        })
    }

    /// Initialize the networking system
    pub async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing Network Manager");
        Ok(())
    }

    /// Get a reference to the HTTP client
    pub fn client(&self) -> &client::HttpClient {
        &self.client
    }
}

impl Default for NetworkManager {
    fn default() -> Self {
        Self::new().expect("Failed to create NetworkManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_manager_creation() {
        let manager = NetworkManager::new();
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_network_manager_initialization() {
        let mut manager = NetworkManager::new().unwrap();
        assert!(manager.initialize().await.is_ok());
    }
}
