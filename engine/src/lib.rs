//! # Horizon Engine
//!
//! Core rendering engine for the Horizon Browser.
//! Provides the foundational rendering pipeline and view management.

pub mod renderer;
pub mod view;

use anyhow::Result;
use async_trait::async_trait;

/// Trait defining the core engine interface
#[async_trait]
pub trait Engine: Send + Sync {
    /// Initialize the rendering engine
    async fn initialize(&mut self) -> Result<()>;

    /// Render a single frame
    async fn render_frame(&mut self) -> Result<()>;

    /// Shutdown the engine gracefully
    async fn shutdown(&mut self) -> Result<()>;
}

/// Main engine implementation
pub struct HorizonEngine {
    initialized: bool,
}

impl HorizonEngine {
    /// Create a new engine instance
    pub fn new() -> Self {
        tracing::info!("Creating new Horizon Engine");
        Self { initialized: false }
    }
}

impl Default for HorizonEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Engine for HorizonEngine {
    async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing Horizon Engine");
        self.initialized = true;
        Ok(())
    }

    async fn render_frame(&mut self) -> Result<()> {
        if !self.initialized {
            anyhow::bail!("Engine not initialized");
        }
        // Placeholder: actual rendering logic will be implemented later
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        tracing::info!("Shutting down Horizon Engine");
        self.initialized = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_initialization() {
        let mut engine = HorizonEngine::new();
        assert!(engine.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_engine_render_before_init() {
        let mut engine = HorizonEngine::new();
        assert!(engine.render_frame().await.is_err());
    }

    #[tokio::test]
    async fn test_engine_lifecycle() {
        let mut engine = HorizonEngine::new();
        engine.initialize().await.unwrap();
        assert!(engine.render_frame().await.is_ok());
        assert!(engine.shutdown().await.is_ok());
    }
}
