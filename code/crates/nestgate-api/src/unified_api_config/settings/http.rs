//! HTTP Server Configuration Settings
//!
//! HTTP server configuration types for the NestGate API.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HTTP server configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiHttpServerSettings {
    /// Enable CORS
    pub enable_cors: bool,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
    /// CORS allowed methods
    pub cors_methods: Vec<String>,
    /// CORS allowed headers
    pub cors_headers: Vec<String>,
    /// Maximum request body size (bytes)
    pub max_request_body_size: usize,
    /// Request timeout
    pub request_timeout: Duration,
    /// Connection keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// Maximum concurrent connections
    pub max_concurrent_connections: usize,
    /// Enable request compression
    pub enable_compression: bool,
    /// Compression level (1-9)
    pub compression_level: u32,
    /// Enable HTTP/2
    pub enable_http2: bool,
    /// Enable graceful shutdown
    pub enable_graceful_shutdown: bool,
    /// Graceful shutdown timeout
    pub graceful_shutdown_timeout: Duration,
}

impl Default for ApiHttpServerSettings {
    fn default() -> Self {
        Self {
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
            cors_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
            cors_headers: vec![
                "accept".to_string(),
                "authorization".to_string(),
                "content-type".to_string(),
                "user-agent".to_string(),
                "x-csrftoken".to_string(),
                "x-requested-with".to_string(),
            ],
            max_request_body_size: 10 * 1024 * 1024, // 10MB
            request_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(75),
            max_concurrent_connections: 1000,
            enable_compression: true,
            compression_level: 6,
            enable_http2: true,
            enable_graceful_shutdown: true,
            graceful_shutdown_timeout: Duration::from_secs(30),
        }
    }
} 