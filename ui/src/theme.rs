//! Theme system for Horizon Browser
//!
//! Provides theming capabilities with support for dark and light modes.

use serde::{Deserialize, Serialize};

/// Color in RGB format
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Create a new color
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create color from hex string
    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Self { r, g, b })
    }
}

/// Firefox-inspired color palette for light and dark modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    // Window and panel backgrounds
    pub bg_window: Color,
    pub bg_toolbar: Color,
    pub bg_tab_active: Color,
    pub bg_tab_inactive: Color,

    // Text colors
    pub text_primary: Color,
    pub text_secondary: Color,

    // Accent colors
    pub accent: Color,
    pub accent_hover: Color,

    // Status colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,

    // Border colors
    pub border_subtle: Color,
}

impl ColorPalette {
    /// Create Firefox-inspired dark theme palette
    pub fn dark() -> Self {
        Self {
            // Dark mode backgrounds (Firefox-inspired)
            bg_window: Color::new(17, 24, 39),        // #111827 - Main window
            bg_toolbar: Color::new(31, 41, 51),       // #1F2933 - Toolbar/tab strip
            bg_tab_active: Color::new(17, 24, 39),    // #111827 - Active tab matches window
            bg_tab_inactive: Color::new(31, 41, 51),  // #1F2933 - Inactive tabs

            // Text colors
            text_primary: Color::new(249, 250, 251),  // #F9FAFB - Primary text
            text_secondary: Color::new(156, 163, 175), // #9CA3AF - Secondary text

            // Accent colors (Firefox-inspired blue)
            accent: Color::new(59, 130, 246),         // #3B82F6 - Primary accent
            accent_hover: Color::new(96, 165, 250),   // #60A5FA - Hover state

            // Status colors
            success: Color::new(34, 197, 94),         // #22C55E - Success/green
            warning: Color::new(251, 191, 36),        // #FBBF24 - Warning/yellow
            error: Color::new(248, 113, 113),         // #F87171 - Error/red

            // Borders
            border_subtle: Color::new(55, 65, 81),    // #374151 - Subtle borders
        }
    }

    /// Create Firefox-inspired light theme palette
    pub fn light() -> Self {
        Self {
            // Light mode backgrounds (Firefox-inspired)
            bg_window: Color::new(249, 250, 251),     // #F9FAFB - Main window
            bg_toolbar: Color::new(229, 231, 235),    // #E5E7EB - Toolbar/tab strip
            bg_tab_active: Color::new(255, 255, 255), // #FFFFFF - Active tab
            bg_tab_inactive: Color::new(229, 231, 235), // #E5E7EB - Inactive tabs

            // Text colors
            text_primary: Color::new(17, 24, 39),     // #111827 - Primary text
            text_secondary: Color::new(75, 85, 99),   // #4B5563 - Secondary text

            // Accent colors (Firefox-inspired blue)
            accent: Color::new(37, 99, 235),          // #2563EB - Primary accent
            accent_hover: Color::new(29, 78, 216),    // #1D4ED8 - Hover state

            // Status colors
            success: Color::new(22, 163, 74),         // #16A34A - Success/green
            warning: Color::new(234, 179, 8),         // #EAB308 - Warning/yellow
            error: Color::new(220, 38, 38),           // #DC2626 - Error/red

            // Borders
            border_subtle: Color::new(209, 213, 219), // #D1D5DB - Subtle borders
        }
    }
}

/// Spacing system based on 4px unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub unit: i32,       // Base unit: 4px
    pub small: i32,      // 4px
    pub standard: i32,   // 8px
    pub large: i32,      // 12-16px
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            unit: 4,
            small: 4,
            standard: 8,
            large: 12,
        }
    }
}

/// Border radii for different component types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Radii {
    pub button: f32,     // 4px
    pub tab: f32,        // 4px
    pub field: f32,      // 6-8px for inputs
    pub panel: f32,      // 6px for panels/cards
}

impl Default for Radii {
    fn default() -> Self {
        Self {
            button: 4.0,
            tab: 4.0,
            field: 6.0,
            panel: 6.0,
        }
    }
}

/// Typography settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    pub base_size: u16,    // 14px
    pub tab_size: u16,     // 13px
    pub menu_size: u16,    // 13px
    pub font_family: String,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            base_size: 14,
            tab_size: 13,
            menu_size: 13,
            font_family: "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif".to_string(),
        }
    }
}

/// Main theme structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    name: String,
    palette: ColorPalette,
    spacing: Spacing,
    radii: Radii,
    typography: Typography,
}

impl Theme {
    /// Create a new theme
    pub fn new(name: impl Into<String>, palette: ColorPalette) -> Self {
        Self {
            name: name.into(),
            palette,
            spacing: Spacing::default(),
            radii: Radii::default(),
            typography: Typography::default(),
        }
    }

    /// Get theme name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the color palette
    pub fn palette(&self) -> &ColorPalette {
        &self.palette
    }

    /// Get spacing
    pub fn spacing(&self) -> &Spacing {
        &self.spacing
    }

    /// Get radii
    pub fn radii(&self) -> &Radii {
        &self.radii
    }

    /// Get typography
    pub fn typography(&self) -> &Typography {
        &self.typography
    }

    /// Get font family (convenience method)
    pub fn font_family(&self) -> &str {
        &self.typography.font_family
    }

    /// Get font size (convenience method)
    pub fn font_size(&self) -> u16 {
        self.typography.base_size
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new("Dark", ColorPalette::dark())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 128, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_color_from_hex() {
        let color = Color::from_hex("#ff8000").unwrap();
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_dark_theme() {
        let theme = Theme::default();
        assert_eq!(theme.name(), "Dark");
        assert_eq!(theme.palette().bg_window.r, 17);
    }

    #[test]
    fn test_light_theme() {
        let theme = Theme::new("Light", ColorPalette::light());
        assert_eq!(theme.name(), "Light");
        assert_eq!(theme.palette().bg_window.r, 249);
    }
}
