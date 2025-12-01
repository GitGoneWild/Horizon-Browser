//! # Horizon Sandbox
//!
//! Sandboxing and process isolation for the Horizon Browser.
//! Provides security policies and runtime guards.

pub mod isolation;
pub mod policy;

use anyhow::Result;

/// Sandbox manager
pub struct SandboxManager {
    policy: policy::SecurityPolicy,
}

impl SandboxManager {
    /// Create a new sandbox manager
    pub fn new() -> Self {
        Self {
            policy: policy::SecurityPolicy::default(),
        }
    }

    /// Initialize the sandbox system
    pub fn initialize(&mut self) -> Result<()> {
        tracing::info!("Initializing Sandbox Manager");
        Ok(())
    }

    /// Get the security policy
    pub fn policy(&self) -> &policy::SecurityPolicy {
        &self.policy
    }

    /// Update the security policy
    pub fn set_policy(&mut self, policy: policy::SecurityPolicy) {
        self.policy = policy;
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_manager() {
        let mut manager = SandboxManager::new();
        assert!(manager.initialize().is_ok());
    }
}
