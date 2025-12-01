//! Extension registry - manages loaded extensions

use anyhow::Result;
use std::collections::HashMap;

/// Extension registry
pub struct ExtensionRegistry {
    extensions: HashMap<String, ExtensionInfo>,
}

/// Information about a loaded extension
#[derive(Debug, Clone)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

impl ExtensionRegistry {
    /// Create a new extension registry
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    /// Register an extension
    pub fn register(
        &mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<()> {
        let id = id.into();
        let info = ExtensionInfo {
            id: id.clone(),
            name: name.into(),
            version: version.into(),
            enabled: true,
        };

        self.extensions.insert(id, info);
        Ok(())
    }

    /// Unregister an extension
    pub fn unregister(&mut self, id: &str) -> Result<()> {
        if self.extensions.remove(id).is_some() {
            Ok(())
        } else {
            anyhow::bail!("Extension not found")
        }
    }

    /// Get extension info
    pub fn get(&self, id: &str) -> Option<&ExtensionInfo> {
        self.extensions.get(id)
    }

    /// List all extensions
    pub fn list(&self) -> Vec<&ExtensionInfo> {
        self.extensions.values().collect()
    }

    /// Enable an extension
    pub fn enable(&mut self, id: &str) -> Result<()> {
        if let Some(info) = self.extensions.get_mut(id) {
            info.enabled = true;
            Ok(())
        } else {
            anyhow::bail!("Extension not found")
        }
    }

    /// Disable an extension
    pub fn disable(&mut self, id: &str) -> Result<()> {
        if let Some(info) = self.extensions.get_mut(id) {
            info.enabled = false;
            Ok(())
        } else {
            anyhow::bail!("Extension not found")
        }
    }
}

impl Default for ExtensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry() {
        let mut registry = ExtensionRegistry::new();

        registry
            .register("test-1", "Test Extension 1", "1.0.0")
            .unwrap();
        assert_eq!(registry.list().len(), 1);

        let info = registry.get("test-1").unwrap();
        assert_eq!(info.name, "Test Extension 1");
        assert!(info.enabled);

        registry.disable("test-1").unwrap();
        assert!(!registry.get("test-1").unwrap().enabled);

        registry.unregister("test-1").unwrap();
        assert_eq!(registry.list().len(), 0);
    }
}
