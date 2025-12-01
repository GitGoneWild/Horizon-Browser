//! Tab management for the Horizon Browser

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single browser tab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    /// Unique identifier for the tab
    pub id: String,
    /// Current URL
    pub url: String,
    /// Page title
    pub title: String,
    /// Navigation history
    pub history: Vec<String>,
    /// Current position in history
    pub history_index: usize,
    /// Loading state
    pub is_loading: bool,
}

impl Tab {
    /// Create a new tab with the given URL
    pub fn new(url: impl Into<String>) -> Self {
        let url = url.into();
        Self {
            id: Uuid::new_v4().to_string(),
            url: url.clone(),
            title: "New Tab".to_string(),
            history: vec![url],
            history_index: 0,
            is_loading: false,
        }
    }

    /// Navigate to a new URL
    pub fn navigate_to(&mut self, url: impl Into<String>) {
        let url = url.into();

        // Remove any forward history if we're not at the end
        if self.history_index < self.history.len() - 1 {
            self.history.truncate(self.history_index + 1);
        }

        // Add new URL to history
        self.history.push(url.clone());
        self.history_index = self.history.len() - 1;
        self.url = url;
        self.is_loading = true;
    }

    /// Navigate back in history
    pub fn go_back(&mut self) -> bool {
        if self.can_go_back() {
            self.history_index -= 1;
            self.url = self.history[self.history_index].clone();
            self.is_loading = true;
            true
        } else {
            false
        }
    }

    /// Navigate forward in history
    pub fn go_forward(&mut self) -> bool {
        if self.can_go_forward() {
            self.history_index += 1;
            self.url = self.history[self.history_index].clone();
            self.is_loading = true;
            true
        } else {
            false
        }
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    /// Check if can go forward
    pub fn can_go_forward(&self) -> bool {
        self.history_index < self.history.len() - 1
    }

    /// Reload the current page
    pub fn reload(&mut self) {
        self.is_loading = true;
    }

    /// Update the tab title
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    /// Mark loading as complete
    pub fn finish_loading(&mut self) {
        self.is_loading = false;
    }

    /// Get display title (truncated if too long)
    pub fn display_title(&self) -> String {
        if self.title == "New Tab" || self.title.is_empty() {
            self.url.clone()
        } else {
            self.title.clone()
        }
    }
}

/// Manages all browser tabs
#[derive(Debug, Clone)]
pub struct TabManager {
    /// All tabs
    tabs: Vec<Tab>,
    /// Index of the active tab
    active_tab_index: usize,
}

impl TabManager {
    /// Create a new tab manager with a default tab
    pub fn new() -> Self {
        Self {
            tabs: vec![Tab::new("about:home")],
            active_tab_index: 0,
        }
    }

    /// Get the active tab
    ///
    /// # Panics
    /// Panics if active_tab_index is out of bounds. This should not happen
    /// in normal operation as the index is always kept in sync with the tabs vector.
    pub fn active_tab(&self) -> &Tab {
        debug_assert!(
            self.active_tab_index < self.tabs.len(),
            "Active tab index out of bounds"
        );
        &self.tabs[self.active_tab_index]
    }

    /// Get the active tab mutably
    ///
    /// # Panics
    /// Panics if active_tab_index is out of bounds. This should not happen
    /// in normal operation as the index is always kept in sync with the tabs vector.
    pub fn active_tab_mut(&mut self) -> &mut Tab {
        debug_assert!(
            self.active_tab_index < self.tabs.len(),
            "Active tab index out of bounds"
        );
        &mut self.tabs[self.active_tab_index]
    }

    /// Get all tabs
    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    /// Get active tab index
    pub fn active_tab_index(&self) -> usize {
        self.active_tab_index
    }

    /// Add a new tab
    pub fn new_tab(&mut self, url: impl Into<String>) {
        let tab = Tab::new(url);
        self.tabs.push(tab);
        self.active_tab_index = self.tabs.len() - 1;
    }

    /// Close a tab by index
    pub fn close_tab(&mut self, index: usize) -> bool {
        if self.tabs.len() <= 1 {
            // Don't close the last tab
            return false;
        }

        if index < self.tabs.len() {
            self.tabs.remove(index);

            // Adjust active tab index if needed
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            } else if index <= self.active_tab_index && self.active_tab_index > 0 {
                self.active_tab_index -= 1;
            }

            true
        } else {
            false
        }
    }

    /// Switch to a tab by index
    pub fn switch_to_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_tab_index = index;
            true
        } else {
            false
        }
    }

    /// Get tab count
    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    /// Check internal invariants (for testing/debugging)
    #[cfg(debug_assertions)]
    #[allow(dead_code)]
    fn check_invariants(&self) {
        assert!(
            !self.tabs.is_empty(),
            "TabManager must have at least one tab"
        );
        assert!(
            self.active_tab_index < self.tabs.len(),
            "Active tab index must be valid"
        );
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation() {
        let tab = Tab::new("https://example.com");
        assert_eq!(tab.url, "https://example.com");
        assert_eq!(tab.history.len(), 1);
        assert_eq!(tab.history_index, 0);
    }

    #[test]
    fn test_tab_navigation() {
        let mut tab = Tab::new("https://example.com");
        tab.navigate_to("https://example.com/page2");

        assert_eq!(tab.url, "https://example.com/page2");
        assert_eq!(tab.history.len(), 2);
        assert!(tab.can_go_back());
        assert!(!tab.can_go_forward());
    }

    #[test]
    fn test_tab_back_forward() {
        let mut tab = Tab::new("https://example.com");
        tab.navigate_to("https://example.com/page2");
        tab.navigate_to("https://example.com/page3");

        assert!(tab.go_back());
        assert_eq!(tab.url, "https://example.com/page2");

        assert!(tab.go_forward());
        assert_eq!(tab.url, "https://example.com/page3");
    }

    #[test]
    fn test_tab_manager_creation() {
        let manager = TabManager::new();
        assert_eq!(manager.tab_count(), 1);
        assert_eq!(manager.active_tab().url, "about:home");
    }

    #[test]
    fn test_tab_manager_new_tab() {
        let mut manager = TabManager::new();
        manager.new_tab("https://example.com");

        assert_eq!(manager.tab_count(), 2);
        assert_eq!(manager.active_tab_index(), 1);
        assert_eq!(manager.active_tab().url, "https://example.com");
    }

    #[test]
    fn test_tab_manager_close_tab() {
        let mut manager = TabManager::new();
        manager.new_tab("https://example.com");

        assert!(manager.close_tab(0));
        assert_eq!(manager.tab_count(), 1);
        assert_eq!(manager.active_tab().url, "https://example.com");
    }

    #[test]
    fn test_tab_manager_cannot_close_last_tab() {
        let mut manager = TabManager::new();
        assert!(!manager.close_tab(0));
        assert_eq!(manager.tab_count(), 1);
    }

    #[test]
    fn test_tab_manager_switch_tab() {
        let mut manager = TabManager::new();
        manager.new_tab("https://example.com");

        assert!(manager.switch_to_tab(0));
        assert_eq!(manager.active_tab_index(), 0);
        assert_eq!(manager.active_tab().url, "about:home");
    }
}
