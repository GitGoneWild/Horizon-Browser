//! Security policy definitions

/// Security policy configuration
#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    /// Enable Content Security Policy
    pub csp_enabled: bool,
    /// Enable Same-Origin Policy
    pub same_origin_policy: bool,
    /// Allow file:// URLs
    pub allow_file_urls: bool,
    /// Enable WebAssembly
    pub enable_wasm: bool,
    /// Enable JavaScript
    pub enable_javascript: bool,
    /// Block mixed content
    pub block_mixed_content: bool,
}

impl SecurityPolicy {
    /// Create a new security policy with strict defaults
    pub fn new() -> Self {
        Self {
            csp_enabled: true,
            same_origin_policy: true,
            allow_file_urls: false,
            enable_wasm: true,
            enable_javascript: true,
            block_mixed_content: true,
        }
    }

    /// Create a permissive policy (for development)
    pub fn permissive() -> Self {
        Self {
            csp_enabled: false,
            same_origin_policy: false,
            allow_file_urls: true,
            enable_wasm: true,
            enable_javascript: true,
            block_mixed_content: false,
        }
    }

    /// Check if a resource should be allowed
    pub fn allow_resource(&self, url: &str) -> bool {
        if url.starts_with("file://") && !self.allow_file_urls {
            return false;
        }
        true
    }
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_policy_default() {
        let policy = SecurityPolicy::default();
        assert!(policy.csp_enabled);
        assert!(policy.same_origin_policy);
        assert!(!policy.allow_file_urls);
    }

    #[test]
    fn test_security_policy_permissive() {
        let policy = SecurityPolicy::permissive();
        assert!(!policy.csp_enabled);
        assert!(policy.allow_file_urls);
    }

    #[test]
    fn test_allow_resource() {
        let policy = SecurityPolicy::default();
        assert!(policy.allow_resource("https://example.com"));
        assert!(!policy.allow_resource("file:///etc/passwd"));

        let permissive = SecurityPolicy::permissive();
        assert!(permissive.allow_resource("file:///home/user/file.txt"));
    }
}
