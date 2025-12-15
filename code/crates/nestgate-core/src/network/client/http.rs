//! HTTP Client Implementation
//!
//! High-level HTTP client with connection pooling, retry logic,
//! and comprehensive error handling.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::config::ClientConfig;
use super::pool::ConnectionPool;
use super::request::{Request, Response};
use super::types::Endpoint;
use crate::error::{NestGateError, Result};

// ==================== HTTP CLIENT ====================

/// Modern HTTP client with connection pooling and retry logic
///
/// Provides high-level HTTP operations with automatic retry,
/// connection pooling, and comprehensive error handling.
///
/// # Examples
/// ```ignore
/// let client = HttpClient::default();
/// let endpoint = Endpoint::from_url("http://api.example.com")?;
/// let response = client.get(&endpoint, "/users").await?;
/// ```
#[derive(Debug)]
pub struct HttpClient {
    pool: ConnectionPool,
    config: ClientConfig,
}

impl HttpClient {
    /// Create a new HTTP client with custom configuration
    pub fn new(config: ClientConfig) -> Self {
        let pool = ConnectionPool::new(config.clone());
        Self { pool, config }
    }

    /// Send a GET request
    pub async fn get(&self, endpoint: &Endpoint, path: &str) -> Result<Response> {
        let request = Request::get(path);
        self.send_request(endpoint, &request).await
    }

    /// Send a POST request with JSON body
    pub async fn post_json(&self, endpoint: &Endpoint, path: &str, body: &str) -> Result<Response> {
        let request = Request::post_json(path, body);
        self.send_request(endpoint, &request).await
    }

    /// Send a request with automatic retry logic
    ///
    /// Implements exponential backoff for retries:
    /// - Attempt 1: immediate
    /// - Attempt 2: 100ms delay
    /// - Attempt 3: 200ms delay
    pub async fn send_request(
        &self,
        endpoint: &Endpoint,
        request: &Request<'_>,
    ) -> Result<Response> {
        const MAX_ATTEMPTS: usize = 3;
        const BASE_DELAY_MS: u64 = 100;

        for attempt in 0..MAX_ATTEMPTS {
            match self.send_request_once(endpoint, request).await {
                Ok(response) => return Ok(response),
                Err(e) if attempt == MAX_ATTEMPTS - 1 => return Err(e),
                Err(_) => {
                    // Exponential backoff
                    let delay = Duration::from_millis(BASE_DELAY_MS * (1 << attempt));
                    tokio::time::sleep(delay).await;
                }
            }
        }

        unreachable!("Loop returns or errors before reaching here")
    }

    /// Send a single request attempt (no retry)
    async fn send_request_once(
        &self,
        endpoint: &Endpoint,
        request: &Request<'_>,
    ) -> Result<Response> {
        // Get connection from pool
        let mut connection = self.pool.get_connection(endpoint).await?;

        // Send request with timeout
        let response = tokio::time::timeout(self.config.timeout, connection.send(request))
            .await
            .map_err(|_| NestGateError::timeout_error("HTTP request", self.config.timeout))?;

        // Return connection to pool
        self.pool.return_connection(connection).await;

        response
    }

    /// Get client statistics
    pub async fn stats(&self) -> ClientStats {
        let pool_stats = self.pool.stats().await;

        ClientStats {
            total_connections: pool_stats.total_connections,
            active_requests: pool_stats.active_connections,
            idle_connections: pool_stats.idle_connections,
            endpoints: pool_stats.endpoints,
        }
    }

    /// Clean up idle connections
    pub async fn cleanup(&self) {
        self.pool.cleanup_idle().await;
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new(ClientConfig::default())
    }
}

// ==================== STATISTICS ====================

/// Client-level statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientStats {
    /// Total connections in pool
    pub total_connections: usize,
    /// Connections currently sending requests
    pub active_requests: usize,
    /// Connections idle and available
    pub idle_connections: usize,
    /// Number of unique endpoints
    pub endpoints: usize,
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = HttpClient::default();
        // Client should be created successfully
        drop(client);
    }

    #[test]
    fn test_client_with_config() {
        let config = ClientConfig::default()
            .with_timeout(Duration::from_secs(60))
            .with_max_retries(5);

        let client = HttpClient::new(config);
        drop(client);
    }

    #[tokio::test]
    async fn test_client_stats() {
        let client = HttpClient::default();
        let stats = client.stats().await;

        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.endpoints, 0);
    }
}
