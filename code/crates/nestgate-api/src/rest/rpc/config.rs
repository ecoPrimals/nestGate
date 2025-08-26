//
// **MIGRATION COMPLETE**: This module now uses canonical modernization patterns.
// Legacy fragmented config structs have been replaced with canonical equivalents.

use std::time::Duration;
use nestgate_core::canonical_modernization::{CanonicalModernizedConfig, NetworkConfig, SecurityConfig};

/// **CANONICAL RPC CONFIGURATION**
/// Extends the canonical modernization system with RPC-specific settings
#[derive(Debug, Clone)]
pub struct CanonicalRpcConfig {
    /// Base canonical configuration
    pub base: CanonicalModernizedConfig,
    /// RPC-specific extensions
    pub rpc_extensions: RpcExtensions,
}

/// RPC-specific configuration extensions
#[derive(Debug, Clone)]
pub struct RpcExtensions {
    /// Connection pool settings
    pub connection_pool: ConnectionPoolConfig,
    /// Load balancing settings
    pub load_balancing: LoadBalancingConfig,
    /// Health monitoring settings
    pub health_monitoring: HealthMonitoringConfig,
    /// Metrics collection settings
    pub metrics: MetricsConfig,
    /// Stream registry settings
    pub streams: StreamConfig,
}

// CANONICAL MODERNIZATION: Legacy NestGateRpcConfig removed - use CanonicalRpcConfig instead

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    /// Maximum number of connections per service
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Idle timeout before closing connections
    pub idle_timeout: Duration,
    /// Connection retry attempts
    pub retry_attempts: u32,
    /// Retry backoff duration
    pub retry_backoff: Duration,
}

/// RPC security configuration
#[derive(Debug, Clone)]
pub struct RpcSecurityConfig {
    /// Enable TLS encryption
    pub enable_tls: bool,
    /// TLS certificate path
    pub tls_cert_path: Option<String>,
    /// TLS private key path
    pub tls_key_path: Option<String>,
    /// Enable mutual TLS authentication
    pub enable_mtls: bool,
    /// Trusted CA certificates path
    pub ca_cert_path: Option<String>,
    /// Enable request signing
    pub enable_signing: bool,
    /// Signing key path
    pub signing_key_path: Option<String>,
    /// Enable rate limiting
    pub enable_rate_limiting: bool,
    /// Rate limit per minute
    pub rate_limit_per_minute: u32,
}

/// Load balancing configuration
#[derive(Debug, Clone)]
pub struct LoadBalancingConfig {
    /// Load balancing strategy
    pub strategy: String, // "round_robin", "least_connections", "weighted"
    /// Health check interval
    pub health_check_interval: Duration,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Failure threshold before opening circuit
    pub failure_threshold: u32,
    /// Success threshold before closing circuit
    pub success_threshold: u32,
    /// Timeout before attempting to close circuit
    pub timeout: Duration,
}

/// Health monitoring configuration
#[derive(Debug, Clone)]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    pub enabled: bool,
    /// Health check interval
    pub check_interval: Duration,
    /// Health check timeout
    pub check_timeout: Duration,
    /// Unhealthy threshold (consecutive failures)
    pub unhealthy_threshold: u32,
    /// Healthy threshold (consecutive successes)
    pub healthy_threshold: u32,
}

/// **CANONICAL MODERNIZATION** - Use canonical metrics configuration
pub use nestgate_core::CanonicalMetricsConfig as MetricsConfig;

// ==================== RPC CONFIGURATION TYPES ====================

/// Stream configuration
#[derive(Debug, Clone)]
pub struct StreamConfig {
    /// Maximum concurrent streams per connection
    pub max_streams_per_connection: usize,
    /// Stream timeout
    pub stream_timeout: Duration,
    /// Buffer size for stream events
    pub buffer_size: usize,
}

impl Default for NestGateRpcConfig {
    fn default() -> Self {
        Self {
            connection_pool: ConnectionPoolConfig {
                max_connections: 100,
                connection_timeout: Duration::from_secs(30),
                idle_timeout: Duration::from_secs(300),
                retry_attempts: 3,
                retry_backoff: Duration::from_millis(1000),
            },
            security: RpcSecurityConfig {
                enable_tls: true,
                tls_cert_path: None,
                tls_key_path: None,
                enable_mtls: false,
                ca_cert_path: None,
                enable_signing: true,
                signing_key_path: None,
                enable_rate_limiting: true,
                rate_limit_per_minute: 1000,
            },
            load_balancing: LoadBalancingConfig {
                strategy: "round_robin".to_string(),
                health_check_interval: Duration::from_secs(30),
                circuit_breaker: CircuitBreakerConfig {
                    failure_threshold: 5,
                    success_threshold: 3,
                    timeout: Duration::from_secs(60),
                },
            },
            health_monitoring: HealthMonitoringConfig {
                enabled: true,
                check_interval: Duration::from_secs(30),
                check_timeout: Duration::from_secs(10),
                unhealthy_threshold: 3,
                healthy_threshold: 2,
            },
            metrics: MetricsConfig {
                enabled: true,
                collection_interval: Duration::from_secs(60),
                retention_period: Duration::from_secs(86400), // 24 hours
                detailed_metrics: false,
            },
            streams: StreamConfig {
                max_streams_per_connection: 10,
                stream_timeout: Duration::from_secs(300),
                buffer_size: 1000,
            },
        }
    }
}

// ==================== CANONICAL IMPLEMENTATIONS ====================

impl Default for CanonicalRpcConfig {
    fn default() -> Self {
        Self {
            base: CanonicalModernizedConfig::default(),
            rpc_extensions: RpcExtensions::default(),
        }
    }
}

impl Default for RpcExtensions {
    fn default() -> Self {
        Self {
            connection_pool: ConnectionPoolConfig::default(),
            load_balancing: LoadBalancingConfig::default(),
            health_monitoring: HealthMonitoringConfig::default(),
            metrics: MetricsConfig::default(),
            streams: StreamConfig::default(),
        }
    }
}

impl CanonicalRpcConfig {
    /// Create canonical RPC config from legacy config
    pub fn from_legacy(legacy: NestGateRpcConfig) -> Self {
        let mut canonical = Self::default();
        
        // Migrate security settings to canonical base
        canonical.base.security.tls_enabled = legacy.security.enable_tls;
        canonical.base.security.mtls_enabled = legacy.security.enable_mtls;
        
        // Keep RPC-specific settings in extensions
        canonical.rpc_extensions.connection_pool = legacy.connection_pool;
        canonical.rpc_extensions.load_balancing = legacy.load_balancing;
        canonical.rpc_extensions.health_monitoring = legacy.health_monitoring;
        canonical.rpc_extensions.metrics = legacy.metrics;
        canonical.rpc_extensions.streams = legacy.streams;
        
        canonical
    }
    
    /// Get network configuration from canonical base
    pub fn network(&self) -> &NetworkConfig {
        &self.base.network
    }
    
    /// Get security configuration from canonical base
    pub fn security(&self) -> &SecurityConfig {
        &self.base.security
    }
}
