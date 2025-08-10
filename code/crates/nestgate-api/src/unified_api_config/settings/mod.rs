//! API Configuration Settings
//!
//! This module contains all API configuration settings organized into logical sub-modules.
//! Originally extracted from a large 902-line file for better maintainability.

pub mod http;
pub mod performance;
pub mod streaming;

// Re-export all settings types for backward compatibility
pub use http::ApiHttpServerSettings;
pub use performance::{
    ApiCircuitBreakerSettings, ApiConnectionPoolSettings, ApiPerformanceSettings,
    ApiRetryPolicySettings,
};
pub use streaming::{ApiSseSettings, ApiStreamingSettings};

// Additional settings types that were in the original file
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// Service mesh configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServiceMeshSettings {
    /// Enable service mesh
    pub enable_service_mesh: bool,
    /// Service mesh provider
    pub service_mesh_provider: String,
    /// Service mesh namespace
    pub service_mesh_namespace: String,
    /// Enable service mesh TLS
    pub enable_service_mesh_tls: bool,
    /// Service mesh configuration file path
    pub service_mesh_config_path: Option<PathBuf>,
}

/// Primal configuration settings for universal architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPrimalSettings {
    /// Enable primal discovery
    pub enable_primal_discovery: bool,
    /// Primal discovery interval
    pub primal_discovery_interval: Duration,
    /// Maximum primal connections
    pub max_primal_connections: usize,
    /// Primal connection timeout
    pub primal_connection_timeout: Duration,
    /// Enable primal load balancing
    pub enable_primal_load_balancing: bool,
    /// Primal health check interval
    pub primal_health_check_interval: Duration,
}

/// Authentication configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthSettings {
    /// Enable authentication
    pub enable_auth: bool,
    /// Authentication provider
    pub auth_provider: String,
    /// JWT secret key
    pub jwt_secret: Option<String>,
    /// JWT expiration time
    pub jwt_expiration: Duration,
    /// Enable refresh tokens
    pub enable_refresh_tokens: bool,
    /// Refresh token expiration
    pub refresh_token_expiration: Duration,
    /// Enable API key authentication
    pub enable_api_key_auth: bool,
}

/// Health monitoring configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiHealthSettings {
    /// Enable health checks
    pub enable_health_checks: bool,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Health check timeout
    pub health_check_timeout: Duration,
    /// Enable health check caching
    pub enable_health_check_caching: bool,
    /// Health check cache TTL
    pub health_check_cache_ttl: Duration,
    /// Health check endpoints
    pub health_check_endpoints: Vec<String>,
}

/// Storage configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStorageSettings {
    /// Enable storage operations
    pub enable_storage_operations: bool,
    /// Storage provider
    pub storage_provider: String,
    /// Storage configuration
    pub storage_config: HashMap<String, String>,
    /// Enable storage caching
    pub enable_storage_caching: bool,
    /// Storage cache TTL
    pub storage_cache_ttl: Duration,
    /// Maximum storage operation timeout
    pub max_storage_operation_timeout: Duration,
}

/// Stream retry configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStreamRetrySettings {
    /// Enable stream retries
    pub enable_stream_retries: bool,
    /// Maximum stream retry attempts
    pub max_stream_retry_attempts: u32,
    /// Stream retry delay
    pub stream_retry_delay: Duration,
    /// Stream retry backoff multiplier
    pub stream_retry_backoff_multiplier: f64,
}

/// Service discovery configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServiceDiscoverySettings {
    /// Enable service discovery
    pub enable_service_discovery: bool,
    /// Service discovery provider
    pub service_discovery_provider: String,
    /// Service discovery refresh interval
    pub service_discovery_refresh_interval: Duration,
    /// Service discovery cache TTL
    pub service_discovery_cache_ttl: Duration,
}

/// RPC timeout configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRpcTimeoutSettings {
    /// Default RPC timeout
    pub default_rpc_timeout: Duration,
    /// RPC timeouts per service
    pub rpc_timeouts_per_service: HashMap<String, Duration>,
    /// RPC timeout multiplier for retries
    pub rpc_timeout_multiplier_for_retries: f64,
}

// Default implementations for additional settings
impl Default for ApiServiceMeshSettings {
    fn default() -> Self {
        Self {
            enable_service_mesh: false,
            service_mesh_provider: "istio".to_string(),
            service_mesh_namespace: "nestgate".to_string(),
            enable_service_mesh_tls: true,
            service_mesh_config_path: None,
        }
    }
}

impl Default for ApiPrimalSettings {
    fn default() -> Self {
        Self {
            enable_primal_discovery: true,
            primal_discovery_interval: Duration::from_secs(30),
            max_primal_connections: 100,
            primal_connection_timeout: Duration::from_secs(10),
            enable_primal_load_balancing: true,
            primal_health_check_interval: Duration::from_secs(60),
        }
    }
}

impl Default for ApiAuthSettings {
    fn default() -> Self {
        Self {
            enable_auth: true,
            auth_provider: "jwt".to_string(),
            jwt_secret: None,
            jwt_expiration: Duration::from_secs(3600), // 1 hour
            enable_refresh_tokens: true,
            refresh_token_expiration: Duration::from_secs(604800), // 1 week
            enable_api_key_auth: true,
        }
    }
}

impl Default for ApiHealthSettings {
    fn default() -> Self {
        Self {
            enable_health_checks: true,
            health_check_interval: Duration::from_secs(30),
            health_check_timeout: Duration::from_secs(5),
            enable_health_check_caching: true,
            health_check_cache_ttl: Duration::from_secs(60),
            health_check_endpoints: vec!["/health".to_string(), "/ready".to_string()],
        }
    }
}

impl Default for ApiStorageSettings {
    fn default() -> Self {
        Self {
            enable_storage_operations: true,
            storage_provider: "zfs".to_string(),
            storage_config: HashMap::new(),
            enable_storage_caching: true,
            storage_cache_ttl: Duration::from_secs(300), // 5 minutes
            max_storage_operation_timeout: Duration::from_secs(300), // 5 minutes
        }
    }
}

impl Default for ApiStreamRetrySettings {
    fn default() -> Self {
        Self {
            enable_stream_retries: true,
            max_stream_retry_attempts: 3,
            stream_retry_delay: Duration::from_millis(100),
            stream_retry_backoff_multiplier: 2.0,
        }
    }
}

impl Default for ApiServiceDiscoverySettings {
    fn default() -> Self {
        Self {
            enable_service_discovery: true,
            service_discovery_provider: "consul".to_string(),
            service_discovery_refresh_interval: Duration::from_secs(30),
            service_discovery_cache_ttl: Duration::from_secs(300), // 5 minutes
        }
    }
}

impl Default for ApiRpcTimeoutSettings {
    fn default() -> Self {
        Self {
            default_rpc_timeout: Duration::from_secs(30),
            rpc_timeouts_per_service: HashMap::new(),
            rpc_timeout_multiplier_for_retries: 1.5,
        }
    }
} 