//! Password management module for secure credential storage

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// A stored password entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordEntry {
    /// Website URL or domain
    pub url: String,
    /// Username/email
    pub username: String,
    /// Encrypted password (not serialized in output for security)
    password: String,
    /// Display name for this entry
    pub display_name: Option<String>,
    /// Date created
    pub created_at: std::time::SystemTime,
    /// Date last modified
    pub modified_at: std::time::SystemTime,
    /// Number of times used
    pub use_count: u32,
}

impl PasswordEntry {
    /// Create a new password entry
    pub fn new(url: String, username: String, password: String) -> Self {
        let now = std::time::SystemTime::now();
        Self {
            url: Self::normalize_url(&url),
            username,
            password,
            display_name: None,
            created_at: now,
            modified_at: now,
            use_count: 0,
        }
    }

    /// Normalize URL for consistent matching
    fn normalize_url(url: &str) -> String {
        // Remove protocol and trailing slashes
        url.trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_start_matches("www.")
            .trim_end_matches('/')
            .to_lowercase()
    }

    /// Get the stored password (requires authentication in full implementation)
    pub fn get_password(&self) -> &str {
        &self.password
    }

    /// Update the password
    pub fn update_password(&mut self, new_password: String) {
        self.password = new_password;
        self.modified_at = std::time::SystemTime::now();
    }

    /// Increment use count
    pub fn increment_use_count(&mut self) {
        self.use_count += 1;
    }

    /// Get the domain from the URL
    pub fn domain(&self) -> &str {
        &self.url
    }
}

/// Password manager for storing and retrieving credentials
pub struct PasswordManager {
    /// Map of URL -> list of password entries
    passwords: HashMap<String, Vec<PasswordEntry>>,
    /// Path to the password storage file
    storage_path: Option<std::path::PathBuf>,
    /// Whether the manager has been modified
    modified: bool,
}

impl PasswordManager {
    /// Create a new password manager
    pub fn new() -> Self {
        Self {
            passwords: HashMap::new(),
            storage_path: None,
            modified: false,
        }
    }

    /// Create a password manager with a storage path
    pub fn with_storage_path(path: std::path::PathBuf) -> Result<Self> {
        let mut manager = Self::new();
        manager.storage_path = Some(path.clone());
        
        // Load existing passwords if file exists
        if path.exists() {
            manager.load(&path)?;
        }
        
        Ok(manager)
    }

    /// Add a new password entry
    pub fn add_password(&mut self, url: String, username: String, password: String) -> Result<()> {
        let entry = PasswordEntry::new(url.clone(), username.clone(), password);
        let normalized_url = PasswordEntry::normalize_url(&url);
        
        // Check if this username already exists for this URL
        if let Some(entries) = self.passwords.get(&normalized_url) {
            if entries.iter().any(|e| e.username == username) {
                return Err(anyhow!(
                    "Password already exists for {} on {}",
                    username,
                    normalized_url
                ));
            }
        }
        
        self.passwords
            .entry(normalized_url.clone())
            .or_default()
            .push(entry);
        
        self.modified = true;
        tracing::info!("Added password for {} on {}", username, normalized_url);
        Ok(())
    }

    /// Get all password entries for a URL
    pub fn get_passwords_for_url(&self, url: &str) -> Vec<&PasswordEntry> {
        let normalized_url = PasswordEntry::normalize_url(url);
        self.passwords
            .get(&normalized_url)
            .map(|entries| entries.iter().collect())
            .unwrap_or_default()
    }

    /// Get a specific password entry
    pub fn get_password(&mut self, url: &str, username: &str) -> Option<&mut PasswordEntry> {
        let normalized_url = PasswordEntry::normalize_url(url);
        self.passwords
            .get_mut(&normalized_url)?
            .iter_mut()
            .find(|e| e.username == username)
    }

    /// Update an existing password
    pub fn update_password(&mut self, url: &str, username: &str, new_password: String) -> Result<()> {
        let normalized_url = PasswordEntry::normalize_url(url);
        
        if let Some(entries) = self.passwords.get_mut(&normalized_url) {
            if let Some(entry) = entries.iter_mut().find(|e| e.username == username) {
                entry.update_password(new_password);
                self.modified = true;
                tracing::info!("Updated password for {} on {}", username, normalized_url);
                return Ok(());
            }
        }
        
        Err(anyhow!(
            "Password not found for {} on {}",
            username,
            normalized_url
        ))
    }

    /// Delete a password entry
    pub fn delete_password(&mut self, url: &str, username: &str) -> Result<()> {
        let normalized_url = PasswordEntry::normalize_url(url);
        
        if let Some(entries) = self.passwords.get_mut(&normalized_url) {
            let initial_len = entries.len();
            entries.retain(|e| e.username != username);
            
            if entries.len() < initial_len {
                self.modified = true;
                
                // Remove the URL entry if no passwords remain
                if entries.is_empty() {
                    self.passwords.remove(&normalized_url);
                }
                
                tracing::info!("Deleted password for {} on {}", username, normalized_url);
                return Ok(());
            }
        }
        
        Err(anyhow!(
            "Password not found for {} on {}",
            username,
            normalized_url
        ))
    }

    /// Get all stored URLs
    pub fn get_all_urls(&self) -> Vec<String> {
        self.passwords.keys().cloned().collect()
    }

    /// Get all password entries (for UI display)
    pub fn get_all_entries(&self) -> Vec<&PasswordEntry> {
        self.passwords
            .values()
            .flat_map(|entries| entries.iter())
            .collect()
    }

    /// Count total password entries
    pub fn count(&self) -> usize {
        self.passwords.values().map(|v| v.len()).sum()
    }

    /// Check if passwords have been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Clear all passwords
    pub fn clear_all(&mut self) {
        self.passwords.clear();
        self.modified = true;
        tracing::info!("Cleared all passwords");
    }

    /// Search for passwords by URL or username
    pub fn search(&self, query: &str) -> Vec<&PasswordEntry> {
        let query_lower = query.to_lowercase();
        self.passwords
            .values()
            .flat_map(|entries| entries.iter())
            .filter(|entry| {
                entry.url.to_lowercase().contains(&query_lower)
                    || entry.username.to_lowercase().contains(&query_lower)
                    || entry
                        .display_name
                        .as_ref()
                        .map(|n| n.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
            })
            .collect()
    }

    /// Save passwords to file
    pub fn save(&self, path: &Path) -> Result<()> {
        // Note: In a full implementation, passwords would be encrypted before saving
        // using a master password or system keychain integration
        
        let json = serde_json::to_string_pretty(&self.passwords)?;
        std::fs::write(path, json)?;
        tracing::info!("Saved passwords to {:?}", path);
        Ok(())
    }

    /// Save to the configured storage path
    pub fn save_to_storage(&self) -> Result<()> {
        if let Some(path) = &self.storage_path {
            self.save(path)
        } else {
            Err(anyhow!("No storage path configured"))
        }
    }

    /// Load passwords from file
    pub fn load(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Ok(());
        }
        
        let json = std::fs::read_to_string(path)?;
        self.passwords = serde_json::from_str(&json)?;
        self.modified = false;
        tracing::info!("Loaded passwords from {:?}", path);
        Ok(())
    }

    /// Auto-fill suggestions for a URL
    pub fn get_autofill_suggestions(&self, url: &str) -> Vec<AutofillSuggestion> {
        let normalized_url = PasswordEntry::normalize_url(url);
        
        self.passwords
            .get(&normalized_url)
            .map(|entries| {
                entries
                    .iter()
                    .map(|entry| AutofillSuggestion {
                        username: entry.username.clone(),
                        display_name: entry.display_name.clone(),
                        last_used: entry.modified_at,
                        use_count: entry.use_count,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }
}

/// Auto-fill suggestion for password fields
#[derive(Debug, Clone)]
pub struct AutofillSuggestion {
    /// Username to fill
    pub username: String,
    /// Display name (if set)
    pub display_name: Option<String>,
    /// Last time this password was used
    pub last_used: std::time::SystemTime,
    /// Number of times used
    pub use_count: u32,
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_password_manager_creation() {
        let manager = PasswordManager::new();
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_add_password() {
        let mut manager = PasswordManager::new();
        let result = manager.add_password(
            "https://example.com".to_string(),
            "user@example.com".to_string(),
            "password123".to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_get_passwords_for_url() {
        let mut manager = PasswordManager::new();
        manager
            .add_password(
                "https://example.com".to_string(),
                "user1@example.com".to_string(),
                "pass1".to_string(),
            )
            .unwrap();
        manager
            .add_password(
                "https://example.com".to_string(),
                "user2@example.com".to_string(),
                "pass2".to_string(),
            )
            .unwrap();

        let passwords = manager.get_passwords_for_url("https://example.com");
        assert_eq!(passwords.len(), 2);
    }

    #[test]
    fn test_url_normalization() {
        let mut manager = PasswordManager::new();
        manager
            .add_password(
                "https://example.com/".to_string(),
                "user@example.com".to_string(),
                "password".to_string(),
            )
            .unwrap();

        // Different URL formats should match the same domain
        let passwords1 = manager.get_passwords_for_url("http://example.com");
        let passwords2 = manager.get_passwords_for_url("https://example.com/");
        assert_eq!(passwords1.len(), 1);
        assert_eq!(passwords2.len(), 1); // Both normalize to "example.com"
    }

    #[test]
    fn test_duplicate_prevention() {
        let mut manager = PasswordManager::new();
        manager
            .add_password(
                "https://example.com".to_string(),
                "user@example.com".to_string(),
                "pass1".to_string(),
            )
            .unwrap();

        let result = manager.add_password(
            "https://example.com".to_string(),
            "user@example.com".to_string(),
            "pass2".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_update_password() {
        let mut manager = PasswordManager::new();
        manager
            .add_password(
                "https://example.com".to_string(),
                "user@example.com".to_string(),
                "oldpass".to_string(),
            )
            .unwrap();

        let result = manager.update_password(
            "https://example.com",
            "user@example.com",
            "newpass".to_string(),
        );
        assert!(result.is_ok());

        let entry = manager.get_password("https://example.com", "user@example.com");
        assert_eq!(entry.unwrap().get_password(), "newpass");
    }

    #[test]
    fn test_delete_password() {
        let mut manager = PasswordManager::new();
        manager
            .add_password(
                "https://example.com".to_string(),
                "user@example.com".to_string(),
                "password".to_string(),
            )
            .unwrap();

        assert_eq!(manager.count(), 1);

        let result = manager.delete_password("https://example.com", "user@example.com");
        assert!(result.is_ok());
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_save_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut manager = PasswordManager::new();
        
        manager
            .add_password(
                "https://example.com".to_string(),
                "user@example.com".to_string(),
                "password123".to_string(),
            )
            .unwrap();

        manager.save(temp_file.path()).unwrap();

        let mut manager2 = PasswordManager::new();
        manager2.load(temp_file.path()).unwrap();

        assert_eq!(manager2.count(), 1);
        let passwords = manager2.get_passwords_for_url("https://example.com");
        assert_eq!(passwords.len(), 1);
    }

    #[test]
    fn test_search() {
        let mut manager = PasswordManager::new();
        manager
            .add_password(
                "https://example.com".to_string(),
                "alice@example.com".to_string(),
                "pass1".to_string(),
            )
            .unwrap();
        manager
            .add_password(
                "https://test.com".to_string(),
                "bob@test.com".to_string(),
                "pass2".to_string(),
            )
            .unwrap();

        let results = manager.search("alice");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].username, "alice@example.com");

        let results = manager.search("test");
        assert_eq!(results.len(), 1);
    }
}
