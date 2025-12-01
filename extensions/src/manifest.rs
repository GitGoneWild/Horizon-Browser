//! Extension manifest structure for Firefox-compatible extensions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Extension manifest (Firefox WebExtensions compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Manifest version (typically 2 or 3 for Firefox)
    pub manifest_version: u8,
    /// Extension ID (optional in Firefox, auto-generated if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Extension name
    pub name: String,
    /// Extension version
    pub version: String,
    /// Short description
    pub description: String,
    /// Author/developer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Homepage URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage_url: Option<String>,
    /// Icons for different sizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<HashMap<String, String>>,
    /// Permissions requested by the extension
    #[serde(default)]
    pub permissions: Vec<String>,
    /// Optional permissions that can be granted at runtime
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub optional_permissions: Vec<String>,
    /// Background scripts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<BackgroundScripts>,
    /// Content scripts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content_scripts: Vec<ContentScript>,
    /// Browser action (toolbar button)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_action: Option<BrowserAction>,
    /// Page action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_action: Option<PageAction>,
    /// Options page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_ui: Option<OptionsUI>,
    /// Web accessible resources
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub web_accessible_resources: Vec<String>,
}

/// Background scripts configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundScripts {
    /// Background script files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Vec<String>>,
    /// Background page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// Whether the background should be persistent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
}

/// Content script configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentScript {
    /// URL patterns to match
    pub matches: Vec<String>,
    /// JavaScript files to inject
    #[serde(default)]
    pub js: Vec<String>,
    /// CSS files to inject
    #[serde(default)]
    pub css: Vec<String>,
    /// When to inject the script
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_at: Option<String>,
}

/// Browser action configuration (toolbar button)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserAction {
    /// Default icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_icon: Option<String>,
    /// Default title (tooltip)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_title: Option<String>,
    /// Default popup HTML
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_popup: Option<String>,
}

/// Page action configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageAction {
    /// Default icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_icon: Option<String>,
    /// Default title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_title: Option<String>,
    /// Default popup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_popup: Option<String>,
}

/// Options UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionsUI {
    /// Options page HTML file
    pub page: String,
    /// Whether to open in a browser tab
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_in_tab: Option<bool>,
}

/// Standard Firefox extension permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Permission {
    /// Access to tabs
    Tabs,
    /// Access to bookmarks
    Bookmarks,
    /// Access to history
    History,
    /// Access to storage
    Storage,
    /// Access to cookies
    Cookies,
    /// Web request interception
    WebRequest,
    /// Web request blocking
    WebRequestBlocking,
    /// Network access
    Network,
    /// Downloads
    Downloads,
    /// Notifications
    Notifications,
    /// Context menus
    ContextMenus,
    /// All URLs access
    AllUrls,
}

impl Permission {
    /// Get the permission string for manifest
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tabs => "tabs",
            Self::Bookmarks => "bookmarks",
            Self::History => "history",
            Self::Storage => "storage",
            Self::Cookies => "cookies",
            Self::WebRequest => "webRequest",
            Self::WebRequestBlocking => "webRequestBlocking",
            Self::Network => "network",
            Self::Downloads => "downloads",
            Self::Notifications => "notifications",
            Self::ContextMenus => "contextMenus",
            Self::AllUrls => "<all_urls>",
        }
    }
}

impl Manifest {
    /// Create a new manifest with required fields
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            manifest_version: 2, // Firefox WebExtensions use manifest v2
            id: None,
            name: name.into(),
            version: version.into(),
            description: description.into(),
            author: None,
            homepage_url: None,
            icons: None,
            permissions: Vec::new(),
            optional_permissions: Vec::new(),
            background: None,
            content_scripts: Vec::new(),
            browser_action: None,
            page_action: None,
            options_ui: None,
            web_accessible_resources: Vec::new(),
        }
    }

    /// Set the extension ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the author
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Add a permission
    pub fn with_permission(mut self, permission: impl Into<String>) -> Self {
        self.permissions.push(permission.into());
        self
    }

    /// Add multiple permissions
    pub fn with_permissions(mut self, permissions: Vec<String>) -> Self {
        self.permissions.extend(permissions);
        self
    }

    /// Set background scripts
    pub fn with_background_scripts(mut self, scripts: Vec<String>) -> Self {
        self.background = Some(BackgroundScripts {
            scripts: Some(scripts),
            page: None,
            persistent: Some(true),
        });
        self
    }

    /// Add a content script
    pub fn with_content_script(mut self, matches: Vec<String>, js: Vec<String>) -> Self {
        self.content_scripts.push(ContentScript {
            matches,
            js,
            css: Vec::new(),
            run_at: None,
        });
        self
    }

    /// Set browser action
    pub fn with_browser_action(
        mut self,
        title: impl Into<String>,
        popup: impl Into<String>,
    ) -> Self {
        self.browser_action = Some(BrowserAction {
            default_icon: None,
            default_title: Some(title.into()),
            default_popup: Some(popup.into()),
        });
        self
    }

    /// Parse manifest from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Validate the manifest
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Extension name is required".to_string());
        }
        if self.version.is_empty() {
            return Err("Extension version is required".to_string());
        }
        if self.manifest_version < 2 || self.manifest_version > 3 {
            return Err("Manifest version must be 2 or 3".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_creation() {
        let manifest = Manifest::new("Test Extension", "1.0.0", "A test extension")
            .with_id("test-ext")
            .with_permission("tabs".to_string());

        assert_eq!(manifest.id, Some("test-ext".to_string()));
        assert_eq!(manifest.name, "Test Extension");
        assert!(manifest.permissions.contains(&"tabs".to_string()));
    }

    #[test]
    fn test_manifest_serialization() {
        let manifest =
            Manifest::new("Test Extension", "1.0.0", "Test description").with_id("test-ext");
        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.id, deserialized.id);
        assert_eq!(manifest.manifest_version, 2);
    }

    #[test]
    fn test_manifest_validation() {
        let manifest = Manifest::new("Test", "1.0.0", "Description");
        assert!(manifest.validate().is_ok());

        let invalid = Manifest {
            manifest_version: 1,
            ..Manifest::new("Test", "1.0.0", "Description")
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_firefox_compatibility() {
        let manifest = Manifest::new("Test Extension", "1.0.0", "Firefox compatible")
            .with_permissions(vec!["tabs".to_string(), "storage".to_string()])
            .with_background_scripts(vec!["background.js".to_string()])
            .with_browser_action("Test", "popup.html");

        assert_eq!(manifest.manifest_version, 2);
        assert!(manifest.background.is_some());
        assert!(manifest.browser_action.is_some());
    }
}
