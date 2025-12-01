//! HTTP client abstraction

use anyhow::Result;
use async_trait::async_trait;

/// HTTP method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Patch,
}

/// HTTP client trait
#[async_trait]
pub trait Client: Send + Sync {
    /// Send an HTTP request
    async fn send(&self, request: super::request::Request) -> Result<super::response::Response>;
}

/// HTTP client implementation
pub struct HttpClient {
    inner: reqwest::Client,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Result<Self> {
        let inner = reqwest::Client::builder()
            .user_agent("Horizon/0.1.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self { inner })
    }

    /// Perform a GET request
    pub async fn get(&self, url: &str) -> Result<super::response::Response> {
        tracing::debug!("GET request to {}", url);
        let response = self.inner.get(url).send().await?;
        super::response::Response::from_reqwest(response).await
    }

    /// Perform a POST request
    pub async fn post(&self, url: &str, body: Vec<u8>) -> Result<super::response::Response> {
        tracing::debug!("POST request to {}", url);
        let response = self.inner.post(url).body(body).send().await?;
        super::response::Response::from_reqwest(response).await
    }
}

#[async_trait]
impl Client for HttpClient {
    async fn send(&self, request: super::request::Request) -> Result<super::response::Response> {
        match request.method() {
            HttpMethod::Get => self.get(request.url()).await,
            _ => anyhow::bail!("Method not implemented"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_method() {
        assert_eq!(HttpMethod::Get, HttpMethod::Get);
        assert_ne!(HttpMethod::Get, HttpMethod::Post);
    }
}
