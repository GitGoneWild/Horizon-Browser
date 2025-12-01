//! Sidebar navigation module for Horizon Browser
//!
//! Provides a collapsible left sidebar with navigation options.

use serde::{Deserialize, Serialize};

/// Sidebar navigation item
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SidebarItem {
    Search,
    Bookmarks,
    History,
    Settings,
    Stocks,
    Weather,
}

impl SidebarItem {
    /// Get the icon for this sidebar item
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Search => "🔍",
            Self::Bookmarks => "⭐",
            Self::History => "🕐",
            Self::Settings => "⚙",
            Self::Stocks => "📈",
            Self::Weather => "🌤",
        }
    }

    /// Get the label for this sidebar item
    pub fn label(&self) -> &'static str {
        match self {
            Self::Search => "Search",
            Self::Bookmarks => "Bookmarks",
            Self::History => "History",
            Self::Settings => "Settings",
            Self::Stocks => "Stocks",
            Self::Weather => "Weather",
        }
    }

    /// Get all sidebar items
    pub fn all() -> &'static [Self] {
        &[
            Self::Search,
            Self::Bookmarks,
            Self::History,
            Self::Settings,
            Self::Stocks,
            Self::Weather,
        ]
    }
}

/// Sidebar state and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sidebar {
    /// Whether the sidebar is collapsed
    pub collapsed: bool,
    /// Width of the sidebar when expanded
    pub width: f32,
    /// Currently selected item
    pub selected_item: Option<SidebarItem>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            collapsed: false,
            width: 250.0,
            selected_item: None,
        }
    }
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle sidebar collapsed state
    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    /// Select a sidebar item
    pub fn select_item(&mut self, item: SidebarItem) {
        self.selected_item = Some(item);
    }

    /// Get the effective width (collapsed or expanded)
    pub fn effective_width(&self) -> f32 {
        if self.collapsed {
            60.0 // Show only icons when collapsed
        } else {
            self.width
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sidebar_creation() {
        let sidebar = Sidebar::new();
        assert!(!sidebar.collapsed);
        assert_eq!(sidebar.width, 250.0);
    }

    #[test]
    fn test_sidebar_toggle() {
        let mut sidebar = Sidebar::new();
        sidebar.toggle_collapsed();
        assert!(sidebar.collapsed);
        sidebar.toggle_collapsed();
        assert!(!sidebar.collapsed);
    }

    #[test]
    fn test_sidebar_effective_width() {
        let mut sidebar = Sidebar::new();
        assert_eq!(sidebar.effective_width(), 250.0);
        sidebar.toggle_collapsed();
        assert_eq!(sidebar.effective_width(), 60.0);
    }

    #[test]
    fn test_sidebar_item_select() {
        let mut sidebar = Sidebar::new();
        sidebar.select_item(SidebarItem::Bookmarks);
        assert_eq!(sidebar.selected_item, Some(SidebarItem::Bookmarks));
    }
}
