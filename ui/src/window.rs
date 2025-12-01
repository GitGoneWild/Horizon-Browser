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

// Constants for tab title display
const MAX_TAB_TITLE_LENGTH: usize = 25;
const TRUNCATE_AT: usize = 22;

/// The main browser application state
struct BrowserApp {
    /// Tab manager
    tab_manager: TabManager,
    /// URL input buffer
    url_input: String,
    /// Tab to close (deferred)
    tab_to_close: Option<usize>,
    /// Settings state
    settings: crate::settings::SettingsUI,
    /// Sidebar state
    sidebar: crate::sidebar::Sidebar,
}

impl BrowserApp {
    /// Create a new browser application
    fn new() -> Self {
        let tab_manager = TabManager::new();
        let url_input = tab_manager.active_tab().url.clone();
        let settings = crate::settings::SettingsUI::load();
        let sidebar = crate::sidebar::Sidebar::new();

        Self {
            tab_manager,
            url_input,
            tab_to_close: None,
            settings,
            sidebar,
        }
    }

    /// Process URL input and return a properly formatted URL
    fn process_url_input(&self, input: &str) -> String {
        let trimmed = input.trim();
        
        // Check for special URLs
        if trimmed.starts_with("about:") {
            return trimmed.to_string();
        }
        
        // Check for explicit protocol
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return trimmed.to_string();
        }
        
        // Check if it looks like a domain/URL:
        // - Contains at least one dot
        // - Doesn't contain spaces
        // - Has a valid TLD-like pattern (at least 2 chars after last dot)
        if trimmed.contains('.') && !trimmed.contains(' ') {
            let parts: Vec<&str> = trimmed.split('.').collect();
            if parts.len() >= 2 {
                let last_part = parts.last().unwrap();
                // Check if the last part looks like a TLD (2+ characters, alphanumeric)
                if last_part.len() >= 2 && last_part.chars().all(|c| c.is_alphanumeric()) {
                    return format!("https://{}", trimmed);
                }
            }
        }
        
        // Treat as search query
        self.settings.general.search_engine.search_url(trimmed)
    }

    /// Render the home page content with enhanced dashboard
    fn render_home_page(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);

            // Branded Horizon header with gradient-style colors
            ui.heading(
                egui::RichText::new("🌅 Horizon")
                    .size(64.0)
                    .strong()
                    .color(egui::Color32::from_rgb(0, 191, 255)), // Neon blue
            );

            ui.add_space(12.0);

            ui.label(
                egui::RichText::new("Modern, Privacy-Focused Web Browser")
                    .size(20.0)
                    .color(egui::Color32::from_rgb(160, 32, 240)), // Neon purple accent
            );

            ui.add_space(40.0);

            // Central search bar
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 600.0) / 2.0);
                
                egui::Frame::none()
                    .fill(egui::Color32::from_rgba_premultiplied(22, 27, 34, 200))
                    .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 191, 255)))
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                    .rounding(egui::Rounding::same(8.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🔍").size(20.0));
                            ui.add_space(8.0);
                            let mut search_text = String::new();
                            ui.add(
                                egui::TextEdit::singleline(&mut search_text)
                                    .desired_width(520.0)
                                    .hint_text("Search the web...")
                                    .frame(false)
                            );
                        });
                    });
            });

            ui.add_space(50.0);

            // App shortcut cards grid
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 740.0) / 2.0);

                // Designer card
                self.render_app_card(ui, "🎨", "Designer", "Creative tools");
                ui.add_space(20.0);

                // Complex Shader card
                self.render_app_card(ui, "✨", "Complex Shader", "GPU rendering");
                ui.add_space(20.0);

                // News card
                self.render_app_card(ui, "📰", "News", "Latest updates");
            });

            ui.add_space(40.0);

            // Two column layout for widgets
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 740.0) / 2.0);

                // Left column: Weather widget
                ui.vertical(|ui| {
                    self.render_weather_widget(ui);
                });

                ui.add_space(20.0);

                // Right column: News feed
                ui.vertical(|ui| {
                    self.render_news_feed(ui);
                });
            });

            ui.add_space(50.0);

            // Social media icons
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 200.0) / 2.0);
                
                ui.label(egui::RichText::new("Connect:").size(14.0).color(egui::Color32::from_rgb(125, 140, 160)));
                ui.add_space(10.0);
                
                if ui.button(egui::RichText::new("📘").size(20.0)).clicked() {
                    tracing::info!("Facebook link clicked");
                }
                ui.add_space(8.0);
                
                if ui.button(egui::RichText::new("📷").size(20.0)).clicked() {
                    tracing::info!("Instagram link clicked");
                }
                ui.add_space(8.0);
                
                if ui.button(egui::RichText::new("🐦").size(20.0)).clicked() {
                    tracing::info!("Twitter link clicked");
                }
            });

            ui.add_space(40.0);
        });
    }

    /// Render an app shortcut card
    fn render_app_card(&self, ui: &mut egui::Ui, icon: &str, title: &str, subtitle: &str) {
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(51, 51, 51)))
            .inner_margin(egui::Margin::same(20.0))
            .rounding(egui::Rounding::same(8.0))
            .show(ui, |ui| {
                ui.set_width(220.0);
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new(icon).size(48.0));
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new(title).size(18.0).strong().color(egui::Color32::from_rgb(230, 237, 243)));
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(subtitle)
                            .size(13.0)
                            .color(egui::Color32::from_rgb(125, 140, 160)),
                    );
                });
            });
    }

    /// Render weather widget
    fn render_weather_widget(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(51, 51, 51)))
            .inner_margin(egui::Margin::same(16.0))
            .rounding(egui::Rounding::same(8.0))
            .show(ui, |ui| {
                ui.set_width(360.0);
                ui.heading(egui::RichText::new("🌤 Weather").size(18.0).color(egui::Color32::from_rgb(230, 237, 243)));
                ui.add_space(12.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("☀️").size(40.0));
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("72°F / 22°C").size(24.0).strong().color(egui::Color32::from_rgb(230, 237, 243)));
                        ui.label(egui::RichText::new("Sunny").size(14.0).color(egui::Color32::from_rgb(125, 140, 160)));
                    });
                });
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(8.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("💧 Humidity:").size(13.0).color(egui::Color32::from_rgb(125, 140, 160)));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("45%").size(13.0).color(egui::Color32::from_rgb(230, 237, 243)));
                });
            });
    }

    /// Render news feed widget
    fn render_news_feed(&self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(30, 30, 30))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(51, 51, 51)))
            .inner_margin(egui::Margin::same(16.0))
            .rounding(egui::Rounding::same(8.0))
            .show(ui, |ui| {
                ui.set_width(360.0);
                ui.heading(egui::RichText::new("📰 Latest News").size(18.0).color(egui::Color32::from_rgb(230, 237, 243)));
                ui.add_space(12.0);
                
                // News item 1
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📷").size(24.0));
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Horizon Browser v0.0.1 Released").size(14.0).strong().color(egui::Color32::from_rgb(230, 237, 243)));
                        ui.label(egui::RichText::new("New UI design with modern features").size(12.0).color(egui::Color32::from_rgb(125, 140, 160)));
                    });
                });
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                // News item 2
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("🔒").size(24.0));
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Enhanced Privacy Features").size(14.0).strong().color(egui::Color32::from_rgb(230, 237, 243)));
                        ui.label(egui::RichText::new("Better tracking protection added").size(12.0).color(egui::Color32::from_rgb(125, 140, 160)));
                    });
                });
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
    fn render_content(&mut self, ui: &mut egui::Ui) {
        let url = &self.tab_manager.active_tab().url.clone();

        if url == "about:settings" {
            self.render_settings_page(ui);
        } else if url == "about:home" {
            self.render_home_page(ui);
        } else if url == "about:blank" {
            self.render_blank_page(ui);
        } else {
            self.render_web_page(ui, url);
        }
    }

    /// Render the settings page
    fn render_settings_page(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Left sidebar for settings navigation
            egui::SidePanel::left("settings_sidebar")
                .default_width(200.0)
                .resizable(false)
                .frame(
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(22, 27, 34))
                        .inner_margin(egui::Margin::same(16.0)),
                )
                .show_inside(ui, |ui| {
                    ui.heading(
                        egui::RichText::new("⚙ Settings")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(230, 237, 243)),
                    );

                    ui.add_space(20.0);

                    // Settings navigation buttons
                    let selected = &mut self.settings.selected_panel;

                    if ui
                        .selectable_label(
                            *selected == crate::settings::SettingsPanel::General,
                            "🏠 General",
                        )
                        .clicked()
                    {
                        *selected = crate::settings::SettingsPanel::General;
                    }

                    if ui
                        .selectable_label(
                            *selected == crate::settings::SettingsPanel::Privacy,
                            "🔒 Privacy",
                        )
                        .clicked()
                    {
                        *selected = crate::settings::SettingsPanel::Privacy;
                    }

                    if ui
                        .selectable_label(
                            *selected == crate::settings::SettingsPanel::Appearance,
                            "🎨 Appearance",
                        )
                        .clicked()
                    {
                        *selected = crate::settings::SettingsPanel::Appearance;
                    }

                    if ui
                        .selectable_label(
                            *selected == crate::settings::SettingsPanel::Network,
                            "🌐 Network & VPN",
                        )
                        .clicked()
                    {
                        *selected = crate::settings::SettingsPanel::Network;
                    }

                    if ui
                        .selectable_label(
                            *selected == crate::settings::SettingsPanel::Passwords,
                            "🔑 Passwords",
                        )
                        .clicked()
                    {
                        *selected = crate::settings::SettingsPanel::Passwords;
                    }

                    if ui
                        .selectable_label(
                            *selected == crate::settings::SettingsPanel::Extensions,
                            "🧩 Extensions",
                        )
                        .clicked()
                    {
                        *selected = crate::settings::SettingsPanel::Extensions;
                    }

                    if ui
                        .selectable_label(
                            *selected == crate::settings::SettingsPanel::Downloads,
                            "📥 Downloads",
                        )
                        .clicked()
                    {
                        *selected = crate::settings::SettingsPanel::Downloads;
                    }

                    if ui
                        .selectable_label(
                            *selected == crate::settings::SettingsPanel::Advanced,
                            "🔧 Advanced",
                        )
                        .clicked()
                    {
                        *selected = crate::settings::SettingsPanel::Advanced;
                    }
                });

            // Right panel for settings content
            ui.vertical(|ui| {
                ui.add_space(20.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add_space(10.0);

                        match self.settings.selected_panel {
                            crate::settings::SettingsPanel::General => {
                                self.render_general_settings(ui);
                            }
                            crate::settings::SettingsPanel::Privacy => {
                                self.render_privacy_settings(ui);
                            }
                            crate::settings::SettingsPanel::Appearance => {
                                self.render_appearance_settings(ui);
                            }
                            crate::settings::SettingsPanel::Network => {
                                self.render_network_settings(ui);
                            }
                            crate::settings::SettingsPanel::Passwords => {
                                self.render_passwords_settings(ui);
                            }
                            crate::settings::SettingsPanel::Extensions => {
                                self.render_extensions_settings(ui);
                            }
                            crate::settings::SettingsPanel::Downloads => {
                                self.render_downloads_settings(ui);
                            }
                            crate::settings::SettingsPanel::Advanced => {
                                self.render_advanced_settings(ui);
                            }
                        }

                        ui.add_space(20.0);

                        // Save button
                        ui.horizontal(|ui| {
                            if ui.button("💾 Save Settings").clicked() {
                                self.settings.save();
                            }
                        });
                    });
            });
        });
    }

    /// Render general settings panel
    fn render_general_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("General Settings")
                .size(18.0)
                .color(egui::Color32::from_rgb(230, 237, 243)),
        );
        ui.add_space(10.0);

        ui.label("Homepage URL:");
        ui.text_edit_singleline(&mut self.settings.general.homepage);
        ui.add_space(10.0);

        ui.label("Default Search Engine:");
        egui::ComboBox::from_label("")
            .selected_text(self.settings.general.search_engine.name())
            .show_ui(ui, |ui| {
                for engine in crate::settings::SearchEngine::all() {
                    ui.selectable_value(
                        &mut self.settings.general.search_engine,
                        *engine,
                        engine.name(),
                    );
                }
            });
        ui.add_space(10.0);

        ui.checkbox(
            &mut self.settings.general.restore_tabs_on_startup,
            "Restore tabs on startup",
        );
    }

    /// Render privacy settings panel
    fn render_privacy_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("Privacy & Security")
                .size(18.0)
                .color(egui::Color32::from_rgb(230, 237, 243)),
        );
        ui.add_space(10.0);

        ui.checkbox(
            &mut self.settings.privacy.tracking_protection,
            "Enable tracking protection",
        );
        ui.label(
            egui::RichText::new("Blocks known trackers from loading")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        ui.checkbox(&mut self.settings.privacy.do_not_track, "Send Do Not Track");
        ui.label(
            egui::RichText::new("Tells websites you don't want to be tracked")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        ui.checkbox(
            &mut self.settings.privacy.block_third_party_cookies,
            "Block third-party cookies",
        );
        ui.label(
            egui::RichText::new("Prevents cross-site tracking")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        ui.checkbox(
            &mut self.settings.privacy.https_only,
            "Enable HTTPS-only mode",
        );
        ui.label(
            egui::RichText::new("Only connect to secure HTTPS websites")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        ui.checkbox(
            &mut self.settings.privacy.clear_data_on_exit,
            "Clear browsing data on exit",
        );
        ui.label(
            egui::RichText::new("Clears cookies, cache, and history when closing the browser")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
    }

    /// Render appearance settings panel
    fn render_appearance_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("Appearance")
                .size(18.0)
                .color(egui::Color32::from_rgb(230, 237, 243)),
        );
        ui.add_space(10.0);

        ui.label("Theme:");
        egui::ComboBox::from_label("")
            .selected_text(self.settings.appearance.theme.name())
            .show_ui(ui, |ui| {
                for theme in crate::settings::Theme::all() {
                    ui.selectable_value(
                        &mut self.settings.appearance.theme,
                        *theme,
                        theme.name(),
                    );
                }
            });
        ui.add_space(10.0);

        ui.label("Font Size:");
        ui.add(egui::Slider::new(&mut self.settings.appearance.font_size, 10..=20).suffix(" px"));
        ui.add_space(10.0);

        ui.checkbox(
            &mut self.settings.appearance.show_bookmarks_bar,
            "Show bookmarks bar",
        );
    }

    /// Render downloads settings panel
    fn render_downloads_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("Downloads")
                .size(18.0)
                .color(egui::Color32::from_rgb(230, 237, 243)),
        );
        ui.add_space(10.0);

        ui.label("Download Directory:");
        ui.text_edit_singleline(&mut self.settings.downloads.download_directory);
        ui.add_space(10.0);

        ui.checkbox(
            &mut self.settings.downloads.ask_where_to_save,
            "Always ask where to save files",
        );
    }

    /// Render advanced settings panel
    fn render_advanced_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("Advanced")
                .size(18.0)
                .color(egui::Color32::from_rgb(230, 237, 243)),
        );
        ui.add_space(10.0);

        ui.checkbox(
            &mut self.settings.advanced.enable_developer_tools,
            "Enable developer tools",
        );
        ui.label(
            egui::RichText::new("Enables debugging and inspection features")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        ui.checkbox(
            &mut self.settings.advanced.hardware_acceleration,
            "Use hardware acceleration",
        );
        ui.label(
            egui::RichText::new("Improves rendering performance")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        ui.checkbox(
            &mut self.settings.advanced.experimental_features,
            "Enable experimental features",
        );
        ui.label(
            egui::RichText::new("Try new features before they're officially released")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
    }

    /// Render network settings panel
    fn render_network_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("Network & VPN")
                .size(18.0)
                .color(egui::Color32::from_rgb(230, 237, 243)),
        );
        ui.add_space(10.0);

        // DNS Settings
        ui.label(egui::RichText::new("DNS Configuration").size(16.0).strong());
        ui.add_space(5.0);

        ui.label("DNS Provider:");
        egui::ComboBox::from_label("")
            .selected_text(self.settings.network.dns_provider.name())
            .show_ui(ui, |ui| {
                for provider in crate::settings::DnsProvider::all() {
                    ui.selectable_value(
                        &mut self.settings.network.dns_provider,
                        *provider,
                        provider.name(),
                    );
                }
            });
        ui.label(
            egui::RichText::new("Changes take effect immediately")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        if self.settings.network.dns_provider == crate::settings::DnsProvider::Custom {
            ui.label("Custom DNS Servers (comma-separated):");
            ui.text_edit_singleline(&mut self.settings.network.custom_dns_servers);
            ui.label(
                egui::RichText::new("Example: 1.1.1.1, 8.8.8.8")
                    .size(12.0)
                    .color(egui::Color32::from_rgb(125, 140, 160)),
            );
            ui.add_space(8.0);
        }

        ui.separator();
        ui.add_space(10.0);

        // VPN Settings
        ui.label(egui::RichText::new("VPN Configuration").size(16.0).strong());
        ui.add_space(5.0);

        ui.checkbox(&mut self.settings.network.vpn_enabled, "Enable VPN");
        ui.label(
            egui::RichText::new("Route all browser traffic through VPN")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        if self.settings.network.vpn_enabled {
            ui.label("VPN Type:");
            egui::ComboBox::from_label("")
                .selected_text(self.settings.network.vpn_type.name())
                .show_ui(ui, |ui| {
                    for vpn_type in crate::settings::VpnType::all() {
                        ui.selectable_value(
                            &mut self.settings.network.vpn_type,
                            *vpn_type,
                            vpn_type.name(),
                        );
                    }
                });
            ui.add_space(8.0);

            match self.settings.network.vpn_type {
                crate::settings::VpnType::Proxy | crate::settings::VpnType::Socks5 => {
                    ui.label("Proxy Host:");
                    ui.text_edit_singleline(&mut self.settings.network.proxy_host);
                    ui.add_space(5.0);

                    ui.label("Proxy Port:");
                    ui.add(egui::Slider::new(&mut self.settings.network.proxy_port, 1..=65535));
                    ui.add_space(5.0);
                }
                crate::settings::VpnType::OpenVpn => {
                    ui.label(
                        egui::RichText::new("Upload .ovpn file to configure OpenVPN")
                            .color(egui::Color32::from_rgb(88, 166, 255)),
                    );
                    if ui.button("📁 Select .ovpn File").clicked() {
                        // File picker would be implemented here
                        tracing::info!("OpenVPN file selection requested");
                    }
                }
            }
        }

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // Speed Test
        ui.label(egui::RichText::new("Network Speed Test").size(16.0).strong());
        ui.add_space(5.0);

        if ui.button("🚀 Run Speed Test").clicked() {
            tracing::info!("Speed test requested");
            // Speed test would be triggered here
        }
        ui.label(
            egui::RichText::new("Test your connection's download/upload speed and latency")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
    }

    /// Render passwords settings panel
    fn render_passwords_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("Saved Passwords")
                .size(18.0)
                .color(egui::Color32::from_rgb(230, 237, 243)),
        );
        ui.add_space(10.0);

        ui.label(
            egui::RichText::new("Password Manager")
                .size(16.0)
                .strong(),
        );
        ui.add_space(5.0);

        ui.label("Horizon Browser can securely save and autofill your passwords.");
        ui.add_space(10.0);

        // Search box (placeholder for MVP - will be connected to password manager in full implementation)
        let mut search_query = String::new();
        ui.label("Search passwords (Coming soon):");
        ui.text_edit_singleline(&mut search_query);
        ui.add_space(10.0);

        // Password list (placeholder)
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(22, 27, 34))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 54, 61)))
            .inner_margin(egui::Margin::same(15.0))
            .rounding(egui::Rounding::same(6.0))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("🔑").size(40.0));
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("No saved passwords yet")
                            .size(16.0)
                            .color(egui::Color32::from_rgb(125, 140, 160)),
                    );
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new("Passwords will appear here when you save them")
                            .size(13.0)
                            .color(egui::Color32::from_rgb(125, 140, 160)),
                    );
                });
            });

        ui.add_space(15.0);

        // Password options
        ui.label(egui::RichText::new("Options").size(16.0).strong());
        ui.add_space(5.0);

        // Note: These are placeholder checkboxes for MVP. In full implementation,
        // these would be connected to settings storage and password manager state.
        let mut save_passwords = true;
        ui.checkbox(&mut save_passwords, "Offer to save passwords");
        ui.label(
            egui::RichText::new("Ask before saving passwords for websites (Coming soon)")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        let mut autofill_passwords = true;
        ui.checkbox(&mut autofill_passwords, "Auto-fill passwords");
        ui.label(
            egui::RichText::new("Automatically fill in saved passwords (Coming soon)")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(10.0);

        if ui.button("🗑️ Clear All Passwords").clicked() {
            tracing::warn!("Clear all passwords requested (not yet implemented)");
        }
    }

    /// Render extensions settings panel
    fn render_extensions_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            egui::RichText::new("Extensions")
                .size(18.0)
                .color(egui::Color32::from_rgb(230, 237, 243)),
        );
        ui.add_space(10.0);

        ui.label(
            egui::RichText::new("Manage Browser Extensions")
                .size(16.0)
                .strong(),
        );
        ui.add_space(5.0);

        ui.label("Horizon Browser supports Firefox-compatible extensions.");
        ui.add_space(10.0);

        // Extension management buttons
        ui.horizontal(|ui| {
            if ui.button("➕ Install Extension").clicked() {
                tracing::info!("Install extension requested");
            }
            
            if ui.button("📦 Install from File").clicked() {
                tracing::info!("Install from file requested");
            }
        });
        ui.add_space(15.0);

        // Installed extensions (placeholder)
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(22, 27, 34))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(48, 54, 61)))
            .inner_margin(egui::Margin::same(15.0))
            .rounding(egui::Rounding::same(6.0))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("🧩").size(40.0));
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("No extensions installed")
                            .size(16.0)
                            .color(egui::Color32::from_rgb(125, 140, 160)),
                    );
                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new("Install extensions to enhance your browser")
                            .size(13.0)
                            .color(egui::Color32::from_rgb(125, 140, 160)),
                    );
                });
            });

        ui.add_space(15.0);

        // Extension settings
        ui.label(egui::RichText::new("Extension Settings").size(16.0).strong());
        ui.add_space(5.0);

        // Note: These are placeholder checkboxes for MVP. In full implementation,
        // these would be connected to extension manager state and settings storage.
        let mut allow_extensions = true;
        ui.checkbox(&mut allow_extensions, "Allow extensions");
        ui.label(
            egui::RichText::new("Enable or disable all extensions (Coming soon)")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
        ui.add_space(8.0);

        let mut extension_updates = true;
        ui.checkbox(&mut extension_updates, "Automatic extension updates");
        ui.label(
            egui::RichText::new("Keep extensions up to date automatically (Coming soon)")
                .size(12.0)
                .color(egui::Color32::from_rgb(125, 140, 160)),
        );
    }
}

impl Default for BrowserApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply custom modern dark theme with gradient-inspired colors
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::dark();

        // Customize colors to match modern dark theme with gradients (#1E1E1E to #2A2A2A)
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 30);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(34, 34, 34);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(42, 42, 42);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 191, 255); // Neon blue for active
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(30, 30, 30);
        style.visuals.window_fill = egui::Color32::from_rgb(30, 30, 30);
        style.visuals.panel_fill = egui::Color32::from_rgb(30, 30, 30);
        
        // Set rounding to 8px for modern rounded corners
        style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(8.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(8.0);
        
        // Enhance selection colors with neon accents
        style.visuals.selection.bg_fill = egui::Color32::from_rgba_premultiplied(0, 191, 255, 80);
        style.visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 191, 255));

        ctx.set_style(style);

        // Handle keyboard shortcuts
        ctx.input(|i| {
            // Ctrl+T: New tab
            if i.modifiers.command && i.key_pressed(egui::Key::T) {
                self.tab_manager.new_tab("about:home");
                self.url_input = "about:home".to_string();
            }

            // Ctrl+W: Close tab
            if i.modifiers.command && i.key_pressed(egui::Key::W) {
                let current_index = self.tab_manager.active_tab_index();
                if self.tab_manager.tab_count() > 1 {
                    self.tab_to_close = Some(current_index);
                }
            }

            // Ctrl+R or F5: Reload
            if (i.modifiers.command && i.key_pressed(egui::Key::R))
                || i.key_pressed(egui::Key::F5)
            {
                self.tab_manager.active_tab_mut().reload();
            }

            // Alt+Left: Back
            if i.modifiers.alt && i.key_pressed(egui::Key::ArrowLeft)
                && self.tab_manager.active_tab().can_go_back() {
                    self.tab_manager.active_tab_mut().go_back();
                    self.url_input = self.tab_manager.active_tab().url.clone();
                }

            // Alt+Right: Forward
            if i.modifiers.alt && i.key_pressed(egui::Key::ArrowRight)
                && self.tab_manager.active_tab().can_go_forward() {
                    self.tab_manager.active_tab_mut().go_forward();
                    self.url_input = self.tab_manager.active_tab().url.clone();
                }

            // Alt+Home: Go to home
            if i.modifiers.alt && i.key_pressed(egui::Key::Home) {
                self.tab_manager
                    .active_tab_mut()
                    .navigate_to(&self.settings.general.homepage);
                self.url_input = self.settings.general.homepage.clone();
            }

            // Ctrl+L: Focus address bar (simulated by clearing it)
            if i.modifiers.command && i.key_pressed(egui::Key::L) {
                // Request focus on address bar in next frame
                tracing::debug!("Focus address bar");
            }
        });

        // Handle deferred tab close
        if let Some(index) = self.tab_to_close.take() {
            self.tab_manager.close_tab(index);
            self.url_input = self.tab_manager.active_tab().url.clone();
        }

        // Tab bar with modern styling
        let mut switch_to_tab: Option<usize> = None;
        let mut new_tab_clicked = false;

        egui::TopBottomPanel::top("tab_bar")
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(34, 34, 34))
                    .inner_margin(egui::Margin::symmetric(8.0, 6.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Render each tab
                    let active_index = self.tab_manager.active_tab_index();

                    for (index, tab) in self.tab_manager.tabs().iter().enumerate() {
                        let is_active = index == active_index;

                        // Pill-shaped tab styling with gradient-inspired colors
                        let (bg_color, stroke_color) = if is_active {
                            (
                                egui::Color32::from_rgb(42, 42, 42),
                                egui::Color32::from_rgb(0, 191, 255) // Neon blue border for active
                            )
                        } else {
                            (
                                egui::Color32::from_rgb(30, 30, 30),
                                egui::Color32::from_rgb(48, 54, 61)
                            )
                        };

                        egui::Frame::none()
                            .fill(bg_color)
                            .stroke(egui::Stroke::new(if is_active { 2.0 } else { 1.0 }, stroke_color))
                            .inner_margin(egui::Margin::symmetric(14.0, 8.0))
                            .rounding(egui::Rounding::same(8.0)) // Fully rounded pill shape
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    // Loading indicator
                                    if tab.is_loading {
                                        ui.label(
                                            egui::RichText::new("⟳")
                                                .size(10.0)
                                                .color(egui::Color32::from_rgb(88, 166, 255)),
                                        );
                                    }

                                    // Tab title
                                    let title = tab.display_title();
                                    let truncated_title = if title.len() > MAX_TAB_TITLE_LENGTH {
                                        format!("{}...", &title[..TRUNCATE_AT])
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
                                                    .color(text_color)
                                                    .size(13.0),
                                            )
                                            .sense(egui::Sense::click()),
                                        )
                                        .clicked()
                                    {
                                        switch_to_tab = Some(index);
                                    }

                                    // Close button
                                    let close_color = if is_active {
                                        egui::Color32::from_rgb(230, 237, 243)
                                    } else {
                                        egui::Color32::from_rgb(125, 140, 160)
                                    };

                                    if ui
                                        .add(
                                            egui::Button::new(
                                                egui::RichText::new("✕")
                                                    .size(12.0)
                                                    .color(close_color),
                                            )
                                            .frame(false)
                                            .small(),
                                        )
                                        .clicked()
                                    {
                                        self.tab_to_close = Some(index);
                                    }
                                });
                            });

                        ui.add_space(2.0);
                    }

                    // New tab button
                    ui.add_space(4.0);
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("➕").size(14.0))
                                .frame(true)
                                .small(),
                        )
                        .on_hover_text("New tab (Ctrl+T)")
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

        // Left sidebar navigation
        let mut sidebar_action: Option<crate::sidebar::SidebarItem> = None;
        let mut toggle_sidebar = false;
        
        egui::SidePanel::left("sidebar")
            .default_width(self.sidebar.effective_width())
            .resizable(false)
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(30, 30, 30))
                    .inner_margin(egui::Margin::same(8.0)),
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(8.0);
                    
                    // Toggle sidebar button
                    if ui.button(
                        egui::RichText::new(if self.sidebar.collapsed { "☰" } else { "◀" })
                            .size(18.0)
                    ).on_hover_text("Toggle sidebar").clicked() {
                        toggle_sidebar = true;
                    }
                    
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);
                    
                    // Sidebar items
                    for item in crate::sidebar::SidebarItem::all() {
                        let is_selected = self.sidebar.selected_item == Some(*item);
                        
                        let button_text = if self.sidebar.collapsed {
                            egui::RichText::new(item.icon()).size(24.0)
                        } else {
                            egui::RichText::new(format!("{} {}", item.icon(), item.label())).size(16.0)
                        };
                        
                        let button = egui::Button::new(button_text)
                            .fill(if is_selected {
                                egui::Color32::from_rgb(42, 42, 42)
                            } else {
                                egui::Color32::from_rgb(30, 30, 30)
                            })
                            .stroke(if is_selected {
                                egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 191, 255))
                            } else {
                                egui::Stroke::NONE
                            })
                            .rounding(egui::Rounding::same(8.0));
                        
                        if ui.add_sized([ui.available_width(), 40.0], button)
                            .on_hover_text(item.label())
                            .clicked() {
                            sidebar_action = Some(*item);
                        }
                        
                        ui.add_space(4.0);
                    }
                });
            });

        // Handle sidebar actions
        if toggle_sidebar {
            self.sidebar.toggle_collapsed();
        }
        
        if let Some(item) = sidebar_action {
            self.sidebar.select_item(item);
            match item {
                crate::sidebar::SidebarItem::Settings => {
                    self.tab_manager.active_tab_mut().navigate_to("about:settings");
                    self.url_input = "about:settings".to_string();
                }
                crate::sidebar::SidebarItem::Search => {
                    self.tab_manager.active_tab_mut().navigate_to("about:home");
                    self.url_input = "about:home".to_string();
                }
                _ => {
                    tracing::info!("Sidebar item {:?} clicked (not yet implemented)", item);
                }
            }
        }

        // Navigation bar with modern styling
        egui::TopBottomPanel::top("nav_bar")
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(34, 34, 34))
                    .inner_margin(egui::Margin::symmetric(12.0, 10.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Navigation arrows
                    let can_go_back = self.tab_manager.active_tab().can_go_back();
                    if ui
                        .add_enabled(
                            can_go_back, 
                            egui::Button::new(egui::RichText::new("◀").size(16.0))
                                .rounding(egui::Rounding::same(6.0))
                        )
                        .on_hover_text("Go back (Alt+Left)")
                        .clicked()
                    {
                        self.tab_manager.active_tab_mut().go_back();
                        self.url_input = self.tab_manager.active_tab().url.clone();
                    }

                    let can_go_forward = self.tab_manager.active_tab().can_go_forward();
                    if ui
                        .add_enabled(
                            can_go_forward, 
                            egui::Button::new(egui::RichText::new("▶").size(16.0))
                                .rounding(egui::Rounding::same(6.0))
                        )
                        .on_hover_text("Go forward (Alt+Right)")
                        .clicked()
                    {
                        self.tab_manager.active_tab_mut().go_forward();
                        self.url_input = self.tab_manager.active_tab().url.clone();
                    }

                    // Reload/Stop button
                    let is_loading = self.tab_manager.active_tab().is_loading;
                    let reload_text = if is_loading { "⏹" } else { "⟳" };
                    let reload_tooltip = if is_loading {
                        "Stop loading"
                    } else {
                        "Reload page (Ctrl+R)"
                    };

                    if ui.add(
                        egui::Button::new(egui::RichText::new(reload_text).size(16.0))
                            .rounding(egui::Rounding::same(6.0))
                    )
                        .on_hover_text(reload_tooltip)
                        .clicked()
                    {
                        if is_loading {
                            self.tab_manager.active_tab_mut().finish_loading();
                            tracing::info!("Stopped loading");
                        } else {
                            self.tab_manager.active_tab_mut().reload();
                            tracing::info!("Reloading page");
                        }
                    }

                    // Home button
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("🏠").size(16.0))
                                .rounding(egui::Rounding::same(6.0))
                        )
                        .on_hover_text("Go to home page (Alt+Home)")
                        .clicked()
                    {
                        let homepage = self.settings.general.homepage.clone();
                        self.tab_manager.active_tab_mut().navigate_to(&homepage);
                        self.url_input = homepage;
                        tracing::info!("Navigating to home");
                    }

                    ui.add_space(12.0);

                    // SSL/Security lock icon
                    let current_url = &self.tab_manager.active_tab().url;
                    let (security_icon, security_color, security_tooltip) = if current_url.starts_with("https://") {
                        ("🔒", egui::Color32::from_rgb(63, 185, 80), "Secure connection (HTTPS)")
                    } else if current_url.starts_with("http://") {
                        ("⚠", egui::Color32::from_rgb(187, 128, 9), "Not secure (HTTP)")
                    } else if current_url.starts_with("about:") {
                        ("ℹ", egui::Color32::from_rgb(0, 191, 255), "Internal page")
                    } else {
                        ("🌐", egui::Color32::from_rgb(125, 140, 160), "Local or unknown")
                    };

                    ui.label(
                        egui::RichText::new(security_icon)
                            .size(16.0)
                            .color(security_color)
                    ).on_hover_text(security_tooltip);

                    ui.add_space(6.0);

                    // Address bar with glassmorphism-inspired styling
                    let address_bar_response = ui.add(
                        egui::TextEdit::singleline(&mut self.url_input)
                            .desired_width(ui.available_width() - 120.0)
                            .hint_text("Search or enter address...")
                            .frame(true)
                    );

                    // Navigate on Enter key
                    if address_bar_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        let url = self.process_url_input(&self.url_input);
                        self.tab_manager.active_tab_mut().navigate_to(&url);
                        self.url_input = url;
                        tracing::info!("Navigating to: {}", self.url_input);
                    }

                    ui.add_space(6.0);

                    // Bookmark/Star icon
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("⭐").size(16.0))
                                .rounding(egui::Rounding::same(6.0))
                        )
                        .on_hover_text("Bookmark this page")
                        .clicked()
                    {
                        tracing::info!("Bookmark clicked (not yet implemented)");
                    }

                    ui.add_space(4.0);

                    // Profile/Avatar button
                    if ui
                        .add(
                            egui::Button::new(egui::RichText::new("👤").size(16.0))
                                .rounding(egui::Rounding::same(6.0))
                        )
                        .on_hover_text("Profile")
                        .clicked()
                    {
                        tracing::info!("Profile clicked (not yet implemented)");
                    }
                });
            });

        // Central panel for content with gradient-inspired background
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 30)))
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
