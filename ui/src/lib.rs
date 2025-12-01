//! # Horizon UI
//!
//! User interface layer for the Horizon Browser.
//! Provides window management and UI components.

pub mod tabs;
pub mod theme;
pub mod window;

use anyhow::Result;

/// UI manager coordinates all UI components
pub struct UIManager {
    theme: theme::Theme,
}

impl UIManager {
    /// Create a new UI manager
    pub fn new() -> Self {
        Self {
            theme: theme::Theme::default(),
        }
    }

    /// Initialize the UI system
    pub fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing UI Manager");
        Ok(())
    }

    /// Get the current theme
    pub fn theme(&self) -> &theme::Theme {
        &self.theme
    }

    /// Set the theme
    pub fn set_theme(&mut self, theme: theme::Theme) {
        self.theme = theme;
    }
}

impl Default for UIManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_manager_creation() {
        let manager = UIManager::new();
        assert_eq!(manager.theme().name(), "Dark");
    }

    #[test]
    fn test_ui_manager_initialization() {
        let mut manager = UIManager::new();
        assert!(manager.initialize().is_ok());
    }
}
