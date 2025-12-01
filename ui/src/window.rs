//! Window management module

use anyhow::Result;
use eframe::egui;

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub resizable: bool,
    pub decorated: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Horizon Browser".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: true,
            decorated: true,
        }
    }
}

/// Main browser window application
pub struct BrowserWindow {
    config: WindowConfig,
}

impl BrowserWindow {
    /// Create a new browser window
    pub fn new(config: WindowConfig) -> Self {
        Self { config }
    }

    /// Run the browser window (blocking)
    pub fn run(self) -> Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([self.config.width, self.config.height])
                .with_resizable(self.config.resizable)
                .with_decorations(self.config.decorated),
            ..Default::default()
        };

        eframe::run_native(
            &self.config.title,
            options,
            Box::new(|_cc| Ok(Box::new(BrowserApp::default()))),
        )
        .map_err(|e| anyhow::anyhow!("Failed to run window: {}", e))
    }
}

/// The main browser application state
struct BrowserApp {
    url: String,
}

impl Default for BrowserApp {
    fn default() -> Self {
        Self {
            url: "https://example.com".to_string(),
        }
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply dark theme
        ctx.set_visuals(egui::Visuals::dark());

        // Top panel for address bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("URL:");
                ui.text_edit_singleline(&mut self.url);
                if ui.button("Go").clicked() {
                    tracing::info!("Navigating to: {}", self.url);
                }
            });
        });

        // Central panel for content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Horizon Browser");
            ui.separator();
            ui.label("Welcome to Horizon Browser - A lightweight, privacy-focused browser");
            ui.label(format!("Current URL: {}", self.url));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.title, "Horizon Browser");
        assert_eq!(config.width, 1280.0);
        assert_eq!(config.height, 720.0);
    }

    #[test]
    fn test_browser_window_creation() {
        let config = WindowConfig::default();
        let _window = BrowserWindow::new(config);
    }
}
