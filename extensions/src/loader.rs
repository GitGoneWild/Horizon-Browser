//! Extension loader - loads extensions from disk

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Extension loader
pub struct ExtensionLoader {
    extensions_dir: PathBuf,
}

impl ExtensionLoader {
    /// Create a new extension loader
    pub fn new(extensions_dir: PathBuf) -> Self {
        Self { extensions_dir }
    }

    /// Load extensions from the extensions directory
    pub async fn load_extensions(&self) -> Result<Vec<String>> {
        if !self.extensions_dir.exists() {
            std::fs::create_dir_all(&self.extensions_dir)?;
            return Ok(Vec::new());
        }

        let mut extension_ids = Vec::new();

        for entry in std::fs::read_dir(&self.extensions_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let manifest_path = path.join("manifest.json");
                if manifest_path.exists() {
                    if let Some(id) = self.load_extension(&path).await? {
                        extension_ids.push(id);
                    }
                }
            }
        }

        Ok(extension_ids)
    }

    /// Load a single extension
    async fn load_extension(&self, path: &Path) -> Result<Option<String>> {
        let manifest_path = path.join("manifest.json");
        let manifest_content = std::fs::read_to_string(manifest_path)?;
        let manifest: super::manifest::Manifest = serde_json::from_str(&manifest_content)?;

        // Generate an ID if not provided in manifest
        let ext_id = manifest.id.clone().unwrap_or_else(|| {
            // Use directory name as fallback ID
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string()
        });

        tracing::info!("Loaded extension: {} ({})", manifest.name, ext_id);
        Ok(Some(ext_id))
    }

    /// Get the extensions directory
    pub fn extensions_dir(&self) -> &Path {
        &self.extensions_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_extension_loader() {
        let temp_dir = TempDir::new().unwrap();
        let loader = ExtensionLoader::new(temp_dir.path().to_path_buf());
        let extensions = loader.load_extensions().await.unwrap();
        assert_eq!(extensions.len(), 0);
    }
}
