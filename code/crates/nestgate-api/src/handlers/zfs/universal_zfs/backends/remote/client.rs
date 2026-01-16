//! NOTE: HTTP removed per Concentrated Gap Architecture
//! Remote ZFS operations now via Unix sockets through Songbird gateway
#![allow(dead_code)]

use serde_json::Value;
use std::time::Duration;
use tracing::{debug, error, warn};

use crate::handlers::zfs::universal_zfs::config::RemoteConfig;
use crate::handlers::zfs::universal_zfs_types::{UniversalZfsError, UniversalZfsResult};

/// HTTP client for remote ZFS operations (DEPRECATED - HTTP removed)
#[derive(Debug, Clone)]
/// Httpclient
pub struct HttpClient {
    // HTTP client removed - use Unix sockets via Songbird
    endpoint: String,
    timeout: Duration,
}
impl HttpClient {
    /// Create a new HTTP client (DEPRECATED - HTTP removed)
    #[must_use]
    pub fn new(config: &RemoteConfig) -> Self {
        Self {
            endpoint: config.endpoint.clone(),
            timeout: config.timeout,
        }
    }

    /// Perform health check (HTTP removed)
    pub async fn health_check(&self) -> UniversalZfsResult<()> {
        unimplemented!("HTTP removed - use Unix sockets via Songbird gateway")
        let health_url = format!("{}/health", self.endpoint);

        // Try with exponential backoff
        for attempt in 0..3 {
            let delay = Duration::from_millis(100 * (2_u64.pow(attempt)));
            if attempt > 0 {
                tokio::time::sleep(delay).await;
                debug!("Retrying health check (attempt {})", attempt + 1);
            }

            match tokio::time::timeout(Duration::from_secs(5), self.client.get(&health_url).send())
                .await
            {
                Ok(Ok(response)) if response.status().is_success() => {
                    debug!("Health check successful");
                    return Ok(());
                }
                Ok(Ok(response)) => {
                    warn!("Health check failed with status: {}", response.status());
                }
                Ok(Err(e)) => {
                    warn!("Health check request failed: {}", e);
                }
                Err(_) => {
                    warn!("Health check timed out");
                }
            }
        }

        Err(UniversalZfsError::ServiceUnavailable {
            message: "Remote service health check failed after retries".to_string(),
        })
    }

    /// Make HTTP request with enhanced error handling
    pub async fn make_request(
        &self,
        path: &str,
        method: &str,
        body: Option<Value>,
    ) -> UniversalZfsResult<Value> {
        let url = if path.starts_with('/') {
            format!("{}{}", self.endpoint, path)
        } else {
            format!("{}/{}", self.endpoint, path)
        };

        debug!("Making {} request to: {}", method, url);

        let mut request_builder = match method.to_uppercase().as_str() {
            "GET" => self.client.get(&url),
            "POST" => self.client.post(&url),
            "PUT" => self.client.put(&url),
            "DELETE" => self.client.delete(&url),
            _ => {
                return Err(UniversalZfsError::Internal {
                    message: format!("Unsupported HTTP method: {method}"),
                });
            }
        };

        // Add JSON body if provided
        if let Some(body) = body {
            request_builder = request_builder
                .header("Content-Type", "application/json")
                .json(&body);
        }

        // Execute request with timeout
        let start_time = std::time::Instant::now();

        match tokio::time::timeout(self.timeout, request_builder.send()).await {
            Ok(Ok(response)) => {
                let status = response.status();
                let response_time = start_time.elapsed();

                debug!(
                    "Request completed in {:?} with status: {}",
                    response_time, status
                );

                if status.is_success() {
                    match response.json::<Value>().await {
                        Ok(json) => Ok(json),
                        Err(e) => {
                            error!("Failed to parse JSON response: {}", e);
                            Err(UniversalZfsError::Internal {
                                message: format!("Failed to parse JSON response: {e}"),
                            })
                        }
                    }
                } else {
                    let error_text = response.text().await.unwrap_or_default();
                    error!("HTTP request failed with status {}: {}", status, error_text);

                    Err(UniversalZfsError::ServiceUnavailable {
                        message: format!("HTTP {status} error: {error_text}"),
                    })
                }
            }
            Ok(Err(e)) => {
                error!("HTTP request failed: {}", e);
                Err(UniversalZfsError::ServiceUnavailable {
                    message: format!("Request failed: {e}"),
                })
            }
            Err(_) => {
                error!("HTTP request timed out after {:?}", self.timeout);
                Err(UniversalZfsError::ServiceUnavailable {
                    message: format!("Request timed out after {:?}", self.timeout),
                })
            }
        }
    }

    /// Make GET request
    pub async fn get(&self, path: &str) -> UniversalZfsResult<Value> {
        self.make_request(path, "GET", None).await
    }

    /// Make POST request
    pub async fn post(&self, path: &str, body: Value) -> UniversalZfsResult<Value> {
        self.make_request(path, "POST", Some(body)).await
    }

    /// Make PUT request
    pub async fn put(&self, path: &str, body: Value) -> UniversalZfsResult<Value> {
        self.make_request(path, "PUT", Some(body)).await
    }

    /// Make DELETE request
    pub async fn delete(&self, path: &str) -> UniversalZfsResult<Value> {
        self.make_request(path, "DELETE", None).await
    }
}
