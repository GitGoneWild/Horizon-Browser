//! # Horizon Extensions
//!
//! Extension framework for the Horizon Browser.
//! Provides a plugin system for extending browser functionality.

pub mod loader;
pub mod manifest;
pub mod registry;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Extension metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
}

/// Base trait for all extensions
#[async_trait]
pub trait Extension: Send + Sync {
    /// Get extension metadata
    fn metadata(&self) -> &ExtensionMetadata;

    /// Initialize the extension
    async fn initialize(&mut self) -> Result<()>;

    /// Called when the extension is enabled
    async fn on_enable(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when the extension is disabled
    async fn on_disable(&mut self) -> Result<()> {
        Ok(())
    }

    /// Shutdown the extension
    async fn shutdown(&mut self) -> Result<()>;
}

/// Extension manager
pub struct ExtensionManager {
    registry: registry::ExtensionRegistry,
}

impl ExtensionManager {
    /// Create a new extension manager
    pub fn new() -> Self {
        Self {
            registry: registry::ExtensionRegistry::new(),
        }
    }

    /// Initialize the extension system
    pub async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing Extension Manager");
        Ok(())
    }

    /// Get the extension registry
    pub fn registry(&self) -> &registry::ExtensionRegistry {
        &self.registry
    }

    /// Get mutable extension registry
    pub fn registry_mut(&mut self) -> &mut registry::ExtensionRegistry {
        &mut self.registry
    }
}

impl Default for ExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extension_manager() {
        let mut manager = ExtensionManager::new();
        assert!(manager.initialize().await.is_ok());
    }
}
