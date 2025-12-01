//! User data storage (cache, history, bookmarks, etc.)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// User data types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Cache,
    History,
    Bookmarks,
    Cookies,
    LocalStorage,
}

/// User data manager
pub struct UserDataManager {
    data_dir: PathBuf,
}

impl UserDataManager {
    /// Create a new user data manager
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        if !data_dir.exists() {
            std::fs::create_dir_all(&data_dir)?;
        }

        Ok(Self { data_dir })
    }

    /// Get the path for a specific data type
    pub fn path_for(&self, data_type: DataType) -> PathBuf {
        let dir_name = match data_type {
            DataType::Cache => "cache",
            DataType::History => "history",
            DataType::Bookmarks => "bookmarks",
            DataType::Cookies => "cookies",
            DataType::LocalStorage => "local_storage",
        };

        self.data_dir.join(dir_name)
    }

    /// Clear data of a specific type
    pub fn clear(&self, data_type: DataType) -> Result<()> {
        let path = self.path_for(data_type);
        if path.exists() {
            if path.is_dir() {
                std::fs::remove_dir_all(&path)?;
            } else {
                std::fs::remove_file(&path)?;
            }
        }
        Ok(())
    }

    /// Clear all user data
    pub fn clear_all(&self) -> Result<()> {
        if self.data_dir.exists() {
            std::fs::remove_dir_all(&self.data_dir)?;
            std::fs::create_dir_all(&self.data_dir)?;
        }
        Ok(())
    }

    /// Get the data directory
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_user_data_manager() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UserDataManager::new(temp_dir.path().to_path_buf()).unwrap();

        let cache_path = manager.path_for(DataType::Cache);
        assert!(cache_path.to_str().unwrap().contains("cache"));
    }

    #[test]
    fn test_clear_data() {
        let temp_dir = TempDir::new().unwrap();
        let manager = UserDataManager::new(temp_dir.path().to_path_buf()).unwrap();

        let cache_path = manager.path_for(DataType::Cache);
        std::fs::create_dir_all(&cache_path).unwrap();
        std::fs::write(cache_path.join("test.txt"), b"test").unwrap();

        assert!(manager.clear(DataType::Cache).is_ok());
    }
}
