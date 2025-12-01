//! Window management module

use crate::tabs::TabManager;
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
            Box::new(|_cc| Ok(Box::new(BrowserApp::new()))),
        )
        .map_err(|e| anyhow::anyhow!("Failed to run window: {}", e))
    }
}

/// The main browser application state
struct BrowserApp {
    /// Tab manager
    tab_manager: TabManager,
    /// URL input buffer
    url_input: String,
    /// Tab to close (deferred)
    tab_to_close: Option<usize>,
}

impl BrowserApp {
    /// Create a new browser application
    fn new() -> Self {
        let tab_manager = TabManager::new();
        let url_input = tab_manager.active_tab().url.clone();

        Self {
            tab_manager,
            url_input,
            tab_to_close: None,
        }
    }

    /// Render the home page content
    fn render_home_page(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);

            // Horizon logo/title
            ui.heading(
                egui::RichText::new("🌅 Horizon Browser")
                    .size(48.0)
                    .color(egui::Color32::from_rgb(88, 166, 255)),
            );

            ui.add_space(20.0);

            ui.label(
                egui::RichText::new("A Lightweight, Privacy-Focused Browser")
                    .size(20.0)
                    .color(egui::Color32::from_rgb(230, 237, 243)),
            );

            ui.add_space(40.0);

            // Features grid
            ui.horizontal(|ui| {
                ui.add_space(150.0);

                // Feature 1
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("🔒").size(32.0));
                    ui.label(egui::RichText::new("Privacy First").size(16.0).strong());
                    ui.label("Built-in tracking\nprotection");
                });

                ui.add_space(60.0);

                // Feature 2
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("⚡").size(32.0));
                    ui.label(
                        egui::RichText::new("Fast & Lightweight")
                            .size(16.0)
                            .strong(),
                    );
                    ui.label("Minimal resource\nusage");
                });

                ui.add_space(60.0);

                // Feature 3
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("🎨").size(32.0));
                    ui.label(egui::RichText::new("Modern UI").size(16.0).strong());
                    ui.label("Clean dark theme\ninterface");
                });
            });

            ui.add_space(60.0);

            ui.label(
                egui::RichText::new("Enter a URL in the address bar to start browsing")
                    .size(14.0)
                    .color(egui::Color32::from_rgb(125, 140, 160)),
            );
        });
    }

    /// Render a blank page
    fn render_blank_page(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(300.0);
            ui.label(
                egui::RichText::new("about:blank")
                    .size(24.0)
                    .color(egui::Color32::from_rgb(125, 140, 160)),
            );
        });
    }

    /// Render a generic web page
    fn render_web_page(&self, ui: &mut egui::Ui, url: &str) {
        ui.vertical(|ui| {
            ui.add_space(50.0);

            ui.horizontal(|ui| {
                ui.add_space(50.0);
                ui.vertical(|ui| {
                    ui.heading(egui::RichText::new("Web Page Preview")
                        .size(28.0)
                        .color(egui::Color32::from_rgb(88, 166, 255)));

                    ui.add_space(20.0);

                    ui.label(egui::RichText::new(format!("📄 {}", url))
                        .size(16.0)
                        .color(egui::Color32::from_rgb(230, 237, 243)));

                    ui.add_space(30.0);

                    ui.label(egui::RichText::new("This is a placeholder for web content.")
                        .size(14.0)
                        .color(egui::Color32::from_rgb(125, 140, 160)));

                    ui.add_space(10.0);

                    ui.label("In a full implementation, this area would display");
                    ui.label("the rendered web page using a WebView component.");

                    ui.add_space(30.0);

                    // Simulated content
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(22, 27, 34))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 54, 61)))
                        .inner_margin(egui::Margin::same(20.0))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("Simulated Web Content")
                                .size(18.0)
                                .strong());
                            ui.add_space(10.0);
                            ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
                            ui.label("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.");
                            ui.add_space(10.0);
                            ui.label("• List item 1");
                            ui.label("• List item 2");
                            ui.label("• List item 3");
                        });
                });
            });
        });
    }

    /// Render the content area based on current URL
    fn render_content(&self, ui: &mut egui::Ui) {
        let url = &self.tab_manager.active_tab().url;

        if url == "about:home" {
            self.render_home_page(ui);
        } else if url == "about:blank" {
            self.render_blank_page(ui);
        } else {
            self.render_web_page(ui, url);
        }
    }
}

impl Default for BrowserApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply custom dark theme
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();

        // Customize colors to match GitHub dark theme
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(13, 17, 23);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(22, 27, 34);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(33, 38, 45);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(33, 38, 45);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(13, 17, 23);
        style.visuals.window_fill = egui::Color32::from_rgb(13, 17, 23);
        style.visuals.panel_fill = egui::Color32::from_rgb(13, 17, 23);

        ctx.set_style(style);

        // Handle deferred tab close
        if let Some(index) = self.tab_to_close.take() {
            self.tab_manager.close_tab(index);
            self.url_input = self.tab_manager.active_tab().url.clone();
        }

        // Tab bar
        let mut switch_to_tab: Option<usize> = None;
        let mut new_tab_clicked = false;

        egui::TopBottomPanel::top("tab_bar")
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(22, 27, 34))
                    .inner_margin(egui::Margin::symmetric(8.0, 4.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Render each tab
                    let active_index = self.tab_manager.active_tab_index();

                    for (index, tab) in self.tab_manager.tabs().iter().enumerate() {
                        let is_active = index == active_index;

                        let bg_color = if is_active {
                            egui::Color32::from_rgb(13, 17, 23)
                        } else {
                            egui::Color32::from_rgb(22, 27, 34)
                        };

                        egui::Frame::none()
                            .fill(bg_color)
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 54, 61)))
                            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                            .rounding(egui::Rounding::same(4.0))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Tab title
                                    let title = tab.display_title();
                                    let truncated_title = if title.len() > 25 {
                                        format!("{}...", &title[..22])
                                    } else {
                                        title
                                    };

                                    let text_color = if is_active {
                                        egui::Color32::from_rgb(230, 237, 243)
                                    } else {
                                        egui::Color32::from_rgb(125, 140, 160)
                                    };

                                    if ui
                                        .add(
                                            egui::Label::new(
                                                egui::RichText::new(truncated_title)
                                                    .color(text_color),
                                            )
                                            .sense(egui::Sense::click()),
                                        )
                                        .clicked()
                                    {
                                        switch_to_tab = Some(index);
                                    }

                                    // Close button
                                    if ui
                                        .add(egui::Button::new("✕").frame(false).small())
                                        .clicked()
                                    {
                                        self.tab_to_close = Some(index);
                                    }
                                });
                            });

                        ui.add_space(2.0);
                    }

                    // New tab button
                    if ui
                        .add(egui::Button::new("➕").frame(true).small())
                        .clicked()
                    {
                        new_tab_clicked = true;
                    }
                });
            });

        // Handle tab switching
        if let Some(index) = switch_to_tab {
            self.tab_manager.switch_to_tab(index);
            self.url_input = self.tab_manager.active_tab().url.clone();
        }

        // Handle new tab
        if new_tab_clicked {
            self.tab_manager.new_tab("about:home");
            self.url_input = "about:home".to_string();
        }

        // Navigation bar
        egui::TopBottomPanel::top("nav_bar")
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(22, 27, 34))
                    .inner_margin(egui::Margin::symmetric(8.0, 8.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Back button
                    let can_go_back = self.tab_manager.active_tab().can_go_back();
                    if ui
                        .add_enabled(can_go_back, egui::Button::new("◀"))
                        .on_hover_text("Go back")
                        .clicked()
                    {
                        self.tab_manager.active_tab_mut().go_back();
                        self.url_input = self.tab_manager.active_tab().url.clone();
                    }

                    // Forward button
                    let can_go_forward = self.tab_manager.active_tab().can_go_forward();
                    if ui
                        .add_enabled(can_go_forward, egui::Button::new("▶"))
                        .on_hover_text("Go forward")
                        .clicked()
                    {
                        self.tab_manager.active_tab_mut().go_forward();
                        self.url_input = self.tab_manager.active_tab().url.clone();
                    }

                    // Reload button
                    if ui
                        .add(egui::Button::new("⟳"))
                        .on_hover_text("Reload page")
                        .clicked()
                    {
                        self.tab_manager.active_tab_mut().reload();
                        tracing::info!("Reloading page");
                    }

                    ui.add_space(8.0);

                    // Address bar
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut self.url_input)
                            .desired_width(ui.available_width() - 60.0)
                            .hint_text("Enter URL or search...")
                            .frame(true),
                    );

                    // Navigate on Enter key
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let url = if self.url_input.starts_with("http://")
                            || self.url_input.starts_with("https://")
                            || self.url_input.starts_with("about:")
                        {
                            self.url_input.clone()
                        } else if self.url_input.contains('.') {
                            format!("https://{}", self.url_input)
                        } else {
                            // Treat as search query
                            format!("https://duckduckgo.com/?q={}", self.url_input)
                        };

                        self.tab_manager.active_tab_mut().navigate_to(&url);
                        self.url_input = url;
                        tracing::info!("Navigating to: {}", self.url_input);
                    }

                    // Go button
                    if ui.button("Go").clicked() {
                        let url = if self.url_input.starts_with("http://")
                            || self.url_input.starts_with("https://")
                            || self.url_input.starts_with("about:")
                        {
                            self.url_input.clone()
                        } else if self.url_input.contains('.') {
                            format!("https://{}", self.url_input)
                        } else {
                            // Treat as search query
                            format!("https://duckduckgo.com/?q={}", self.url_input)
                        };

                        self.tab_manager.active_tab_mut().navigate_to(&url);
                        self.url_input = url;
                        tracing::info!("Navigating to: {}", self.url_input);
                    }
                });
            });

        // Central panel for content
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(13, 17, 23)))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.render_content(ui);
                    });
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
