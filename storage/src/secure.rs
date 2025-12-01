//! Secure storage for sensitive data

use anyhow::Result;
use std::collections::HashMap;

/// Secure storage for credentials and sensitive data
pub struct SecureStorage {
    store: HashMap<String, Vec<u8>>,
}

impl SecureStorage {
    /// Create a new secure storage
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// Store a value securely
    pub fn store(&mut self, key: impl Into<String>, value: Vec<u8>) -> Result<()> {
        // Placeholder: In production, this would use OS-specific secure storage
        // (e.g., Windows Credential Manager, macOS Keychain, Linux Secret Service)
        self.store.insert(key.into(), value);
        Ok(())
    }

    /// Retrieve a value
    pub fn retrieve(&self, key: &str) -> Option<&[u8]> {
        self.store.get(key).map(|v| v.as_slice())
    }

    /// Remove a value
    pub fn remove(&mut self, key: &str) -> Result<()> {
        self.store.remove(key);
        Ok(())
    }

    /// Check if a key exists
    pub fn contains(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }
}

impl Default for SecureStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_storage() {
        let mut storage = SecureStorage::new();

        let key = "test_credential";
        let value = b"secret_password".to_vec();

        storage.store(key, value.clone()).unwrap();
        assert!(storage.contains(key));

        let retrieved = storage.retrieve(key).unwrap();
        assert_eq!(retrieved, value.as_slice());

        storage.remove(key).unwrap();
        assert!(!storage.contains(key));
    }
}
