//! HTTP request module

use super::client::HttpMethod;

/// HTTP request
#[derive(Debug, Clone)]
pub struct Request {
    method: HttpMethod,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

impl Request {
    /// Create a new GET request
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: Vec::new(),
            body: None,
        }
    }

    /// Create a new POST request
    pub fn post(url: impl Into<String>, body: Vec<u8>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: Vec::new(),
            body: Some(body),
        }
    }

    /// Get the HTTP method
    pub fn method(&self) -> HttpMethod {
        self.method
    }

    /// Get the URL
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Add a header
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    /// Get headers
    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    /// Get the body
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_request() {
        let req = Request::get("https://example.com");
        assert_eq!(req.method(), HttpMethod::Get);
        assert_eq!(req.url(), "https://example.com");
    }

    #[test]
    fn test_post_request() {
        let body = b"test data".to_vec();
        let req = Request::post("https://example.com", body.clone());
        assert_eq!(req.method(), HttpMethod::Post);
        assert_eq!(req.body(), Some(body.as_slice()));
    }

    #[test]
    fn test_request_with_headers() {
        let req = Request::get("https://example.com").header("Content-Type", "application/json");
        assert_eq!(req.headers().len(), 1);
    }
}
