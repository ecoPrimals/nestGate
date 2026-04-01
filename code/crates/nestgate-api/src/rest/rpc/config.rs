// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Config module

use nestgate_core::config::SecurityConfig;
use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
use nestgate_core::config::canonical_primary::domains::performance::MetricsConfig;
use nestgate_core::config::canonical_primary::domains::security_canonical::TlsSecurityConfig;

use std::time::Duration;
// Note: canonical_modernization module structure changed
use nestgate_core::canonical_modernization::CanonicalModernizedConfig;

/// **CANONICAL RPC CONFIGURATION**
/// Extends the canonical modernization system with RPC-specific settings
#[derive(Debug, Clone, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CanonicalRpcConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::CanonicalRpcConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `CanonicalRpc`
pub struct CanonicalRpcConfig {
    /// Base canonical configuration
    pub base: CanonicalModernizedConfig,
    /// RPC-specific extensions
    pub rpc_extensions: RpcExtensions,
}
/// RPC-specific configuration extensions
#[derive(Debug, Clone, Default)]
/// Rpcextensions
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
/// **NESTGATE RPC CONFIGURATION**
///
/// Comprehensive RPC system configuration including connection pooling,
/// security, load balancing, health monitoring, metrics, and streaming.
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NestGateRpcConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NestGateRpcConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `NestGateRpc`
pub struct NestGateRpcConfig {
    /// Connection pool configuration for managing RPC connections
    pub connection_pool: ConnectionPoolConfig,
    /// Security configuration for RPC authentication and encryption
    pub security: RpcSecurityConfig,
    /// Load balancing configuration for distributing RPC requests
    pub load_balancing: LoadBalancingConfig,
    /// Health monitoring configuration for connection health checks
    pub health_monitoring: HealthMonitoringConfig,
    /// Metrics collection configuration for RPC performance monitoring
    pub metrics: MetricsConfig,
    /// Streaming configuration for bidirectional RPC communication
    pub streams: StreamConfig,
}
/// Connection pool configuration
#[derive(Debug, Clone, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::ConnectionPoolConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ConnectionPoolConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `ConnectionPool`
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
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::RpcSecurityConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::RpcSecurityConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::RpcSecurityConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::RpcSecurityConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `RpcSecurity`
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
#[derive(Debug, Clone, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::LoadBalancingConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::LoadBalancingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::LoadBalancingConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::LoadBalancingConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `LoadBalancing`
pub struct LoadBalancingConfig {
    /// Load balancing strategy
    pub strategy: String, // "round_robin", "least_connections", "weighted"
    /// Health check interval
    pub health_check_interval: Duration,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
}
/// Circuit breaker configuration
#[derive(Debug, Clone, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CircuitBreakerConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::CircuitBreakerConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `CircuitBreaker`
pub struct CircuitBreakerConfig {
    /// Failure threshold before opening circuit
    pub failure_threshold: u32,
    /// Success threshold before closing circuit
    pub success_threshold: u32,
    /// Timeout before attempting to close circuit
    pub timeout: Duration,
}
/// Health monitoring configuration
#[derive(Debug, Clone, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::HealthMonitoringConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::HealthMonitoringConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `HealthMonitoring`
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
// Note: CanonicalMetricsConfig moved or renamed; wire `MetricsConfig` from canonical types when needed.
// ==================== SECTION ====================
/// Stream configuration
#[derive(Debug, Clone, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::StreamConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::StreamConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Stream
pub struct StreamConfig {
    /// Maximum concurrent streams per connection
    pub max_streams_per_connection: usize,
    /// Stream timeout
    pub stream_timeout: Duration,
    /// Buffer size for stream events
    pub buffer_size: usize,
}
impl Default for NestGateRpcConfig {
    /// Returns the default instance
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
                collection_interval: Duration::from_secs(60),
                metrics: vec![
                    nestgate_core::config::canonical_primary::domains::performance::monitoring::PerformanceMetric::CpuUsage,
                    nestgate_core::config::canonical_primary::domains::performance::monitoring::PerformanceMetric::MemoryUsage,
                ],
                retention: Duration::from_secs(86400), // 24 hours
            },
            streams: StreamConfig {
                max_streams_per_connection: 10,
                stream_timeout: Duration::from_secs(300),
                buffer_size: 1000,
            },
        }
    }
}

// ==================== SECTION ====================

impl CanonicalRpcConfig {
    /// Create canonical RPC config from legacy config
    #[must_use]
    pub fn from_legacy(legacy: NestGateRpcConfig) -> Self {
        let mut canonical = Self::default();

        // Migrate security settings to canonical base
        // Note: SecurityConfig uses TlsSecurityConfig from canonical_primary
        if legacy.security.enable_tls {
            canonical.base.security.security_settings.insert(
                "tls_enabled".to_string(),
                serde_json::to_value(TlsSecurityConfig::default())
                    .unwrap_or(serde_json::Value::Bool(true)),
            );
        }

        // Keep RPC-specific settings in extensions
        canonical.rpc_extensions.connection_pool = legacy.connection_pool;
        canonical.rpc_extensions.load_balancing = legacy.load_balancing;
        canonical.rpc_extensions.health_monitoring = legacy.health_monitoring;
        canonical.rpc_extensions.metrics = legacy.metrics;
        canonical.rpc_extensions.streams = legacy.streams;

        canonical
    }

    /// Get network configuration from canonical base
    #[must_use]
    pub const fn network(&self) -> &CanonicalNetworkConfig {
        &self.base.network
    }

    /// Get security configuration from canonical base
    #[must_use]
    pub const fn security(&self) -> &SecurityConfig {
        &self.base.security
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Connectionpoolconfigcanonical
pub type ConnectionPoolConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ConnectionPoolConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Rpcsecurityconfigcanonical
pub type RpcSecurityConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using RpcSecurityConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Loadbalancingconfigcanonical
pub type LoadBalancingConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using LoadBalancingConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Circuitbreakerconfigcanonical
pub type CircuitBreakerConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CircuitBreakerConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Healthmonitoringconfigcanonical
pub type HealthMonitoringConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using HealthMonitoringConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Streamconfigcanonical
pub type StreamConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StreamConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Canonicalrpcconfigcanonical
pub type CanonicalRpcConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CanonicalRpcConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Nestgaterpcconfigcanonical
pub type NestGateRpcConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NestGateRpcConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::*;

    #[test]
    fn nestgate_rpc_config_default_smoke() {
        let c = NestGateRpcConfig::default();
        assert!(c.security.enable_tls);
        assert_eq!(c.load_balancing.strategy, "round_robin");
        assert!(c.health_monitoring.enabled);
    }

    #[test]
    fn canonical_rpc_config_default_and_from_legacy() {
        let legacy = NestGateRpcConfig::default();
        let migrated = CanonicalRpcConfig::from_legacy(legacy);
        assert!(
            migrated
                .base
                .security
                .security_settings
                .contains_key("tls_enabled")
        );
        assert_eq!(
            migrated.rpc_extensions.load_balancing.strategy,
            "round_robin"
        );
    }

    #[test]
    fn canonical_rpc_config_accessors() {
        let c = CanonicalRpcConfig::default();
        let _net: &CanonicalNetworkConfig = c.network();
        let _sec: &SecurityConfig = c.security();
    }

    #[test]
    fn rpc_extensions_default() {
        let e = RpcExtensions::default();
        assert_eq!(e.connection_pool.max_connections, 0);
    }

    #[test]
    fn type_alias_smoke_compile() {
        let _: ConnectionPoolConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig::default();
    }
}
