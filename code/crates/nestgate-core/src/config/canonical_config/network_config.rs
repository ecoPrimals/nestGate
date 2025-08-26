//
// Network-related configuration including HTTP server, RPC, load balancing,
// circuit breakers, rate limiting, and connection management.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// Import unified constants
use crate::canonical_modernization::canonical_constants::{
    network::{DEFAULT_API_PORT, REQUEST_TIMEOUT_SECS, DEFAULT_BIND_ADDRESS},
};

/// Network configuration (consolidates 15+ network configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct NetworkConfig {
    /// HTTP server configuration
    pub http_server: HttpServerConfig,
    /// RPC configuration
    pub rpc: RpcConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// Timeout configuration
    pub timeouts: TimeoutConfig,
    /// Connection pooling
    pub connection_pool: ConnectionPoolConfig,
    /// TLS/SSL configuration
    pub tls: TlsConfig,
}

/// HTTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpServerConfig {
    /// Server bind address
    pub bind_address: String,
    /// Server port
    pub port: u16,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Maximum request body size
    pub max_request_body_size: usize,
    /// Enable compression
    pub enable_compression: bool,
}

/// RPC configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcConfig {
    /// RPC server port
    pub port: u16,
    /// Maximum message size
    pub max_message_size: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Enable TLS
    pub enable_tls: bool,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing strategy
    pub strategy: LoadBalancingStrategy,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery threshold
    pub recovery_threshold: u32,
}

/// Load balancing strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum LoadBalancingStrategy {
    /// Round robin
    #[default]
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round robin
    WeightedRoundRobin,
    /// Random
    Random,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery timeout
    pub recovery_timeout: Duration,
    /// Half-open max calls
    pub half_open_max_calls: u32,
    /// Minimum request threshold
    pub min_request_threshold: u32,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per second
    pub requests_per_second: u32,
    /// Burst size
    pub burst_size: u32,
    /// Window size
    pub window_size: Duration,
    /// Enable rate limiting
    pub enabled: bool,
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Keep-alive timeout
    pub keep_alive_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// Maximum connections
    pub max_connections: usize,
    /// Minimum connections
    pub min_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout
    pub idle_timeout: Duration,
    /// Maximum lifetime
    pub max_lifetime: Duration,
}

/// TLS/SSL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct TlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Certificate file path
    pub cert_file: Option<String>,
    /// Private key file path
    pub key_file: Option<String>,
    /// CA certificate file path
    pub ca_file: Option<String>,
    /// Verify client certificates
    pub verify_client: bool,
}


impl Default for HttpServerConfig {
    fn default() -> Self {
        Self {
            bind_address: DEFAULT_BIND_ADDRESS.to_string(),
            port: DEFAULT_API_PORT,
            max_connections: 10_000,
            keep_alive_timeout: Duration::from_secs(60),
            request_timeout: Duration::from_secs(REQUEST_TIMEOUT_SECS),
            max_request_body_size: 16 * 1024 * 1024, // 16MB
            enable_compression: true,
        }
    }
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            port: 9090,
            max_message_size: 4 * 1024 * 1024, // 4MB
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(REQUEST_TIMEOUT_SECS),
            enable_tls: false,
        }
    }
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            strategy: LoadBalancingStrategy::RoundRobin,
            health_check_interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}


impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
            min_request_threshold: 10,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 1000,
            burst_size: 100,
            window_size: Duration::from_secs(1),
            enabled: true,
        }
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(REQUEST_TIMEOUT_SECS),
            keep_alive_timeout: Duration::from_secs(60),
            idle_timeout: Duration::from_secs(300),
        }
    }
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            min_connections: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(1800),
        }
    }
}

 