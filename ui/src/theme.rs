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

/// Theme color palette inspired by modern browsers with gradient backgrounds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    // Background colors with gradient support
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_tertiary: Color,
    pub bg_gradient_start: Color,
    pub bg_gradient_end: Color,

    // Foreground colors
    pub fg_primary: Color,
    pub fg_secondary: Color,
    pub fg_muted: Color,

    // Accent colors with neon accents for modern look
    pub accent_primary: Color,
    pub accent_secondary: Color,
    pub accent_neon_blue: Color,
    pub accent_neon_purple: Color,
    pub accent_success: Color,
    pub accent_warning: Color,
    pub accent_danger: Color,

    // Border colors
    pub border_default: Color,
    pub border_muted: Color,
    pub border_focus: Color,
}

impl ColorPalette {
    /// Create the default dark theme palette with modern gradient backgrounds
    pub fn dark() -> Self {
        Self {
            // Deep dark backgrounds with gradient support
            bg_primary: Color::new(13, 17, 23),   // #0d1117
            bg_secondary: Color::new(22, 27, 34), // #161b22
            bg_tertiary: Color::new(33, 38, 45),  // #21262d
            bg_gradient_start: Color::new(30, 30, 30), // #1E1E1E
            bg_gradient_end: Color::new(42, 42, 42),   // #2A2A2A

            // Foreground colors
            fg_primary: Color::new(230, 237, 243),   // #e6edf3
            fg_secondary: Color::new(125, 140, 160), // #7d8ca0
            fg_muted: Color::new(87, 96, 106),       // #57606a

            // Accent colors with neon accents
            accent_primary: Color::new(88, 166, 255),  // #58a6ff
            accent_secondary: Color::new(139, 148, 255), // Lighter blue
            accent_neon_blue: Color::new(0, 191, 255), // #00BFFF - Neon blue for active states
            accent_neon_purple: Color::new(160, 32, 240), // #A020F0 - Neon purple for highlights
            accent_success: Color::new(63, 185, 80),   // #3fb950
            accent_warning: Color::new(187, 128, 9),   // #bb8009
            accent_danger: Color::new(248, 81, 73),    // #f85149

            // Borders with focus states
            border_default: Color::new(48, 54, 61),  // #30363d
            border_muted: Color::new(33, 38, 45),    // #21262d
            border_focus: Color::new(0, 191, 255),   // #00BFFF - Neon blue for focus
        }
    }

    /// Create a light theme palette (placeholder for future implementation)
    pub fn light() -> Self {
        Self {
            bg_primary: Color::new(255, 255, 255),
            bg_secondary: Color::new(246, 248, 250),
            bg_tertiary: Color::new(234, 238, 242),
            bg_gradient_start: Color::new(240, 240, 240),
            bg_gradient_end: Color::new(250, 250, 250),

            fg_primary: Color::new(36, 41, 47),
            fg_secondary: Color::new(87, 96, 106),
            fg_muted: Color::new(139, 148, 158),

            accent_primary: Color::new(9, 105, 218),
            accent_secondary: Color::new(88, 166, 255),
            accent_neon_blue: Color::new(0, 120, 215),
            accent_neon_purple: Color::new(120, 20, 180),
            accent_success: Color::new(26, 127, 55),
            accent_warning: Color::new(154, 103, 0),
            accent_danger: Color::new(207, 34, 46),

            border_default: Color::new(208, 215, 222),
            border_muted: Color::new(234, 238, 242),
            border_focus: Color::new(0, 120, 215),
        }
    }
}

/// Main theme structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    name: String,
    palette: ColorPalette,
    font_family: String,
    font_size: u16,
}

impl Theme {
    /// Create a new theme
    pub fn new(name: impl Into<String>, palette: ColorPalette) -> Self {
        Self {
            name: name.into(),
            palette,
            font_family: "Inter, system-ui, sans-serif".to_string(),
            font_size: 14,
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

    /// Get font family
    pub fn font_family(&self) -> &str {
        &self.font_family
    }

    /// Get font size
    pub fn font_size(&self) -> u16 {
        self.font_size
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
        assert_eq!(theme.palette().bg_primary.r, 13);
    }

    #[test]
    fn test_light_theme() {
        let theme = Theme::new("Light", ColorPalette::light());
        assert_eq!(theme.name(), "Light");
        assert_eq!(theme.palette().bg_primary.r, 255);
    }
}
