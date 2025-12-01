//! HTTP response module

use anyhow::Result;
use std::collections::HashMap;

/// HTTP response
#[derive(Debug)]
pub struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    /// Create a new response
    pub fn new(status: u16, body: Vec<u8>) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body,
        }
    }

    /// Create from reqwest response
    pub async fn from_reqwest(response: reqwest::Response) -> Result<Self> {
        let status = response.status().as_u16();
        let mut headers = HashMap::new();

        for (name, value) in response.headers() {
            headers.insert(name.to_string(), value.to_str().unwrap_or("").to_string());
        }

        let body = response.bytes().await?.to_vec();

        Ok(Self {
            status,
            headers,
            body,
        })
    }

    /// Get the status code
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Check if the response was successful
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Get a header value
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).map(|s| s.as_str())
    }

    /// Get the response body
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Get the response body as a string
    pub fn body_string(&self) -> Result<String> {
        String::from_utf8(self.body.clone()).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_creation() {
        let response = Response::new(200, b"OK".to_vec());
        assert_eq!(response.status(), 200);
        assert!(response.is_success());
    }

    #[test]
    fn test_response_body_string() {
        let response = Response::new(200, b"Hello, World!".to_vec());
        assert_eq!(response.body_string().unwrap(), "Hello, World!");
    }

    #[test]
    fn test_response_success() {
        let success = Response::new(200, vec![]);
        let redirect = Response::new(301, vec![]);
        let error = Response::new(404, vec![]);

        assert!(success.is_success());
        assert!(!redirect.is_success());
        assert!(!error.is_success());
    }
}
