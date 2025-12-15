//! HTTP Client Utility Functions
//!
//! Convenience functions for common HTTP operations.

use super::http::HttpClient;
use super::types::{Endpoint, Port};
use crate::Result;

// ==================== UTILITY FUNCTIONS ====================

/// Create a default HTTP client
///
/// Convenience function for quick client creation with default settings.
///
/// # Examples
/// ```ignore
/// let client = create_client();
/// ```
pub fn create_client() -> HttpClient {
    HttpClient::default()
}

/// Create an HTTP endpoint from host and port
///
/// # Examples
/// ```ignore
/// let endpoint = http_endpoint("localhost", 8080).await?;
/// ```
pub async fn http_endpoint(host: &str, port: u16) -> Result<Endpoint> {
    let port = Port::new(port)?;
    Ok(Endpoint::http(host.to_string(), port))
}

/// Create an HTTPS endpoint from host and port
///
/// # Examples
/// ```ignore
/// let endpoint = https_endpoint("api.example.com", 443).await?;
/// ```
pub async fn https_endpoint(host: &str, port: u16) -> Result<Endpoint> {
    let port = Port::new(port)?;
    Ok(Endpoint::https(host.to_string(), port))
}

/// Parse a URL string into an endpoint
///
/// # Examples
/// ```ignore
/// let endpoint = parse_endpoint("https://api.example.com:443").await?;
/// ```
pub async fn parse_endpoint(url: &str) -> Result<Endpoint> {
    Endpoint::from_url(url)
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client() {
        let client = create_client();
        drop(client);
    }

    #[tokio::test]
    async fn test_http_endpoint() {
        let endpoint = http_endpoint("localhost", 8080).await.unwrap();
        assert_eq!(endpoint.host, "localhost");
        assert_eq!(endpoint.port.get(), 8080);
    }

    #[tokio::test]
    async fn test_https_endpoint() {
        let endpoint = https_endpoint("api.example.com", 443).await.unwrap();
        assert_eq!(endpoint.host, "api.example.com");
        assert_eq!(endpoint.port.get(), 443);
    }

    #[tokio::test]
    async fn test_parse_endpoint() {
        let endpoint = parse_endpoint("https://example.com:8443").await.unwrap();
        assert_eq!(endpoint.host, "example.com");
        assert_eq!(endpoint.port.get(), 8443);
    }
}
