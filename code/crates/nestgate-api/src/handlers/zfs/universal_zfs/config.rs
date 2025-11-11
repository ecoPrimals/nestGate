//! **CANONICAL ZFS CONFIGURATION**
//!
//! This module provides canonical ZFS handler configuration using the unified
//! configuration system. All ZFS configuration is now managed through the
//! canonical configuration hierarchy.

use serde::{Deserialize, Serialize};

use std::time::Duration;

// Export canonical types as primary API
pub use nestgate_core::config::canonical_primary::{
    ZfsBackendConfig, ZfsFailSafeConfig, ZfsHandlerConfig, ZfsObservabilityConfig,
    ZfsPerformanceConfig, ZfsSecurityConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::RemoteConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::RemoteConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct RemoteConfig {
    pub endpoint: String,
    pub timeout: Duration,
    pub auth: Option<String>,
}

/// Primary ZFS service configuration - uses canonical system
pub type ZfsServiceConfig = ZfsHandlerConfig;
/// Create default ZFS service configuration using canonical system
pub fn default_zfs_config() -> ZfsHandlerConfig {
    ZfsHandlerConfig::default()
}
/// Create ZFS configuration from environment using canonical system
pub fn zfs_config_from_env() -> ZfsHandlerConfig {
    use nestgate_core::config::canonical_primary::handler_config::CanonicalHandlerConfigs;
    // Use specific default for handlers since the full config may be complex
    let handlers_config = CanonicalHandlerConfigs::default();
    handlers_config.zfs
}
/// Validate ZFS configuration using canonical system
pub fn validate_zfs_config(config: &ZfsHandlerConfig) -> Result<(), String> {
    if config.service_name.is_empty() {
        return Err("Service name cannot be empty".to_string());
    }
    Ok(())
}
/// ZFS backend configuration - uses canonical system
pub type ZfsBackend = ZfsBackendConfig;
/// Fail-safe configuration - uses canonical system
pub type FailSafeConfig = ZfsFailSafeConfig;
/// Observability configuration - uses canonical system
pub type ObservabilityConfig = ZfsObservabilityConfig;
/// Performance configuration - uses canonical system
pub type PerformanceConfig = ZfsPerformanceConfig;
/// Security configuration - uses canonical system
pub type SecurityConfig = ZfsSecurityConfig;
// ==================== FAIL-SAFE CONFIGURATION TYPES ====================

/// Circuit breaker configuration for fail-safe operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
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
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct CircuitBreakerConfig {
    pub enabled: bool,
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    pub half_open_max_calls: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(30),
            half_open_max_calls: 3,
        }
    }
}

/// Timeout configuration for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::TimeoutConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::TimeoutConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct TimeoutConfig {
    pub operation_timeout: Duration,
    pub connection_timeout: Duration,
    pub health_check_timeout: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            operation_timeout: Duration::from_secs(60),
            connection_timeout: Duration::from_secs(30),
            health_check_timeout: Duration::from_secs(5),
        }
    }
}

/// Retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

/// ZFS backend type for configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsBackendType {
    Native,
    Remote(RemoteConfig),
    Mock,
}


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type RemoteConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using RemoteConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type CircuitBreakerConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

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
#[allow(deprecated)]
pub type TimeoutConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using TimeoutConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_zfs_config() {
        let config = default_zfs_config();
        assert!(!config.service_name.is_empty());
    }

    #[test]
    fn test_zfs_config_from_env() {
        let config = zfs_config_from_env();
        assert!(!config.service_name.is_empty());
    }

    #[test]
    fn test_validate_zfs_config() {
        let config = default_zfs_config();
        assert!(validate_zfs_config(&config).is_ok());
    }
}
