//! Extension manifest structure

use serde::{Deserialize, Serialize};

/// Extension manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Extension ID
    pub id: String,
    /// Extension name
    pub name: String,
    /// Extension version
    pub version: String,
    /// Description
    pub description: String,
    /// Author
    pub author: String,
    /// Permissions requested by the extension
    #[serde(default)]
    pub permissions: Vec<Permission>,
    /// Entry point file
    #[serde(default)]
    pub entry: String,
}

/// Extension permission
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
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
    /// Network access
    Network,
}

impl Manifest {
    /// Create a new manifest
    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            version: version.into(),
            description: String::new(),
            author: String::new(),
            permissions: Vec::new(),
            entry: "main.js".to_string(),
        }
    }

    /// Add a permission
    pub fn with_permission(mut self, permission: Permission) -> Self {
        self.permissions.push(permission);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_creation() {
        let manifest =
            Manifest::new("test-ext", "Test Extension", "1.0.0").with_permission(Permission::Tabs);

        assert_eq!(manifest.id, "test-ext");
        assert_eq!(manifest.name, "Test Extension");
        assert!(manifest.permissions.contains(&Permission::Tabs));
    }

    #[test]
    fn test_manifest_serialization() {
        let manifest = Manifest::new("test-ext", "Test Extension", "1.0.0");
        let json = serde_json::to_string(&manifest).unwrap();
        let deserialized: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(manifest.id, deserialized.id);
    }
}
