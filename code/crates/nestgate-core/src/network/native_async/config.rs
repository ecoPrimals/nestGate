/// Network Configuration Module
/// Configuration structures for native async network implementation
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Network configuration for native async implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Enable TLS
    pub enable_tls: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            enable_tls: false,
        }
    }
}
