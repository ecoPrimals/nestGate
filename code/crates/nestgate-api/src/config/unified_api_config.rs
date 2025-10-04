//! **MIGRATED API CONFIGURATION MODULE**
//!
//! This module now uses canonical constants instead of hardcoded values,
//! consolidating scattered network constants into the unified system.

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// Use canonical constants instead of hardcoded values
use nestgate_core::constants::canonical_defaults::{
    concurrency::{DEFAULT_MAX_CONNECTIONS, DEFAULT_THREAD_POOL_SIZE},
    network::{DEFAULT_API_PORT, DEFAULT_BIND_ADDRESS, LOCALHOST},
    sizes::{DEFAULT_BUFFER_SIZE, DEFAULT_CACHE_SIZE},
    timeouts::{DEFAULT_CONNECTION_TIMEOUT, DEFAULT_REQUEST_TIMEOUT},
};

/// **UNIFIED API CONFIGURATION**
/// Replaces scattered API configurations with canonical constants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedApiConfig {
    /// API server configuration using canonical constants
    pub server: ApiServerConfig,
    /// Security configuration
    pub security: ApiSecurityConfig,
    /// Performance configuration
    pub performance: ApiPerformanceConfig,
    /// Monitoring configuration
    pub monitoring: ApiMonitoringConfig,
}
impl Default for UnifiedApiConfig {
    fn default() -> Self {
        Self {
            server: ApiServerConfig::default(),
            security: ApiSecurityConfig::default(),
            performance: ApiPerformanceConfig::default(),
            monitoring: ApiMonitoringConfig::default(),
        }
    }
}

/// API server configuration using canonical constants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServerConfig {
    /// Host address using canonical constant instead of hardcoded "127.0.0.1"
    pub host: String,
    /// Port using canonical constant instead of hardcoded 8080
    pub port: u16,
    /// Maximum connections using canonical constant
    pub max_connections: usize,
    /// Request timeout using canonical constant
    pub request_timeout: std::time::Duration,
    /// Connection timeout using canonical constant
    pub connection_timeout: std::time::Duration,
}
impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            // **CONSOLIDATED**: Use canonical constant instead of hardcoded "127.0.0.1"
            host: std::env::var("NESTGATE_API_HOST")
                .unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_string()),
            // **CONSOLIDATED**: Use canonical constant instead of hardcoded 8080
            port: std::env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(DEFAULT_API_PORT),
            // **CONSOLIDATED**: Use canonical constant instead of hardcoded values
            max_connections: DEFAULT_MAX_CONNECTIONS,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            connection_timeout: DEFAULT_CONNECTION_TIMEOUT,
        }
    }
}

impl ApiServerConfig {
    /// Build socket address using canonical constants
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn socket_addr(&self) -> Result<SocketAddr, std::net::AddrParseError>  {
        format!("{}:{}", self.host, self.port).parse()
    }

    /// Build API URL using canonical constants
    pub fn api_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    /// Build API endpoint using canonical constants  
    pub fn api_endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// API security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSecurityConfig {
    /// Enable TLS
    pub tls_enabled: bool,
    /// Certificate path
    pub cert_path: Option<String>,
    /// Private key path
    pub key_path: Option<String>,
    /// Enable authentication
    pub auth_enabled: bool,
    /// JWT secret
    pub jwt_secret: Option<String>,
}
impl Default for ApiSecurityConfig {
    fn default() -> Self {
        Self {
            tls_enabled: std::env::var("NESTGATE_TLS_ENABLED")
                .map(|v| v.parse().unwrap_or(false))
                .unwrap_or(false),
            cert_path: std::env::var("NESTGATE_CERT_PATH").ok(),
            key_path: std::env::var("NESTGATE_KEY_PATH").ok(),
            auth_enabled: std::env::var("NESTGATE_AUTH_ENABLED")
                .map(|v| v.parse().unwrap_or(true))
                .unwrap_or(true),
            jwt_secret: std::env::var("NESTGATE_JWT_SECRET").ok(),
        }
    }
}

/// API performance configuration using canonical constants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPerformanceConfig {
    /// Buffer size using canonical constant
    pub buffer_size: usize,
    /// Thread pool size using canonical constant
    pub thread_pool_size: usize,
    /// Cache size using canonical constant
    pub cache_size: u64,
    /// Enable compression
    pub compression_enabled: bool,
}
impl Default for ApiPerformanceConfig {
    fn default() -> Self {
        Self {
            // **CONSOLIDATED**: Use canonical constants instead of hardcoded values
            buffer_size: DEFAULT_BUFFER_SIZE,
            thread_pool_size: DEFAULT_THREAD_POOL_SIZE,
            cache_size: DEFAULT_CACHE_SIZE,
            compression_enabled: true,
        }
    }
}

/// API monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMonitoringConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Enable health checks
    pub health_checks_enabled: bool,
    /// Enable tracing
    pub tracing_enabled: bool,
    /// Metrics endpoint path
    pub metrics_path: String,
    /// Health check endpoint path
    pub health_path: String,
}
impl Default for ApiMonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            health_checks_enabled: true,
            tracing_enabled: true,
            metrics_path: "/api/v1/monitoring/metrics".to_string(),
            health_path: "/health".to_string(),
        }
    }
}

// ==================== CONVENIENCE FUNCTIONS ====================

/// Create a new unified API configuration
pub fn new_api_config() -> UnifiedApiConfig {
    UnifiedApiConfig::default()
}
/// Create a development-optimized API configuration
pub fn dev_api_config() -> UnifiedApiConfig {
    let mut config = UnifiedApiConfig::default();
    // Development-specific optimizations using canonical constants
    config.server.host = LOCALHOST.to_string();
    config.security.tls_enabled = false;
    config.monitoring.tracing_enabled = true;
    config
}
/// Create a production-optimized API configuration
pub fn prod_api_config() -> UnifiedApiConfig {
    let mut config = UnifiedApiConfig::default();
    // Production-specific optimizations using canonical constants
    config.security.tls_enabled = true;
    config.security.auth_enabled = true;
    config.performance.compression_enabled = true;
    config
}
