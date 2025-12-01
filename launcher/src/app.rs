//! Horizon Browser application

use anyhow::Result;
use horizon_engine::Engine;
use std::path::PathBuf;

/// Main application state
pub struct HorizonApp {
    engine: horizon_engine::HorizonEngine,
    ui_manager: horizon_ui::UIManager,
    network_manager: horizon_networking::NetworkManager,
    storage_manager: horizon_storage::StorageManager,
    extension_manager: horizon_extensions::ExtensionManager,
    sandbox_manager: horizon_sandbox::SandboxManager,
}

impl HorizonApp {
    /// Create a new Horizon application
    pub fn new() -> Result<Self> {
        tracing::info!("Initializing Horizon Browser");

        // Determine the data directory
        let data_dir = Self::get_data_directory()?;
        tracing::info!("Using data directory: {:?}", data_dir);

        Ok(Self {
            engine: horizon_engine::HorizonEngine::new(),
            ui_manager: horizon_ui::UIManager::new(),
            network_manager: horizon_networking::NetworkManager::new()?,
            storage_manager: horizon_storage::StorageManager::new(data_dir)?,
            extension_manager: horizon_extensions::ExtensionManager::new(),
            sandbox_manager: horizon_sandbox::SandboxManager::new(),
        })
    }

    /// Initialize all subsystems
    async fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing subsystems");

        self.sandbox_manager.initialize()?;
        self.storage_manager.initialize()?;
        self.ui_manager.initialize()?;
        self.network_manager.initialize().await?;
        self.extension_manager.initialize().await?;
        self.engine.initialize().await?;

        tracing::info!("All subsystems initialized successfully");
        Ok(())
    }

    /// Run the application
    pub async fn run(mut self) -> Result<()> {
        // Initialize all subsystems
        self.initialize().await?;

        tracing::info!("Launching browser window");

        // Create and run the main window
        let window_config = horizon_ui::window::WindowConfig::default();
        let window = horizon_ui::window::BrowserWindow::new(window_config);

        // Run the window (this blocks until the window is closed)
        window.run()?;

        // Shutdown
        self.shutdown().await?;

        Ok(())
    }

    /// Shutdown the application
    async fn shutdown(mut self) -> Result<()> {
        tracing::info!("Shutting down Horizon Browser");

        self.engine.shutdown().await?;

        // Save settings before exiting
        if let Err(e) = self.storage_manager.save_settings() {
            tracing::error!("Failed to save settings: {}", e);
        }

        tracing::info!("Shutdown complete");
        Ok(())
    }

    /// Get the platform-specific data directory
    fn get_data_directory() -> Result<PathBuf> {
        let data_dir = if cfg!(target_os = "windows") {
            // Windows: %APPDATA%/Horizon
            dirs::data_dir()
                .ok_or_else(|| anyhow::anyhow!("Failed to get data directory"))?
                .join("Horizon")
        } else if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/Horizon
            dirs::data_dir()
                .ok_or_else(|| anyhow::anyhow!("Failed to get data directory"))?
                .join("Horizon")
        } else {
            // Linux: ~/.local/share/horizon
            dirs::data_dir()
                .ok_or_else(|| anyhow::anyhow!("Failed to get data directory"))?
                .join("horizon")
        };

        Ok(data_dir)
    }
}
