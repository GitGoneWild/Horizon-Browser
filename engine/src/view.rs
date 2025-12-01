//! View module - manages view containers and layout

use anyhow::Result;

/// Represents a view container in the browser
#[derive(Debug)]
pub struct View {
    id: String,
    title: String,
    url: Option<String>,
}

impl View {
    /// Create a new view
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            url: None,
        }
    }

    /// Get the view ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the view title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Set the view URL
    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = Some(url.into());
    }

    /// Get the view URL
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

/// View manager handles multiple views
pub struct ViewManager {
    views: Vec<View>,
    active_view: Option<usize>,
}

impl ViewManager {
    /// Create a new view manager
    pub fn new() -> Self {
        Self {
            views: Vec::new(),
            active_view: None,
        }
    }

    /// Add a new view
    pub fn add_view(&mut self, view: View) -> Result<usize> {
        self.views.push(view);
        let index = self.views.len() - 1;
        if self.active_view.is_none() {
            self.active_view = Some(index);
        }
        Ok(index)
    }

    /// Get the active view
    pub fn active_view(&self) -> Option<&View> {
        self.active_view.and_then(|idx| self.views.get(idx))
    }

    /// Set the active view by index
    pub fn set_active_view(&mut self, index: usize) -> Result<()> {
        if index < self.views.len() {
            self.active_view = Some(index);
            Ok(())
        } else {
            anyhow::bail!("Invalid view index")
        }
    }
}

impl Default for ViewManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_creation() {
        let view = View::new("view-1", "Test View");
        assert_eq!(view.id(), "view-1");
        assert_eq!(view.title(), "Test View");
    }

    #[test]
    fn test_view_manager() {
        let mut manager = ViewManager::new();
        let view = View::new("view-1", "Test View");
        manager.add_view(view).unwrap();
        assert!(manager.active_view().is_some());
    }
}
