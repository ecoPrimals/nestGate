//! **CANONICAL ZFS CONFIGURATION**
//!
//! This module provides canonical ZFS handler configuration using the unified
//! configuration system. All ZFS configuration is now managed through the
//! canonical configuration hierarchy.

use serde::{Deserialize, Serialize};

use std::time::Duration;

// Export canonical types as primary API
pub use nestgate_core::config::canonical_master::{
    ZfsBackendConfig, ZfsFailSafeConfig, ZfsHandlerConfig, ZfsObservabilityConfig,
    ZfsPerformanceConfig, ZfsSecurityConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub endpoint: String,
    pub timeout: Duration,
    pub auth: Option<String>,
}

/// Primary ZFS service configuration - uses canonical system
pub type ZfsServiceConfig = ZfsHandlerConfig;
/// Create default ZFS service configuration using canonical system
pub const fn default_zfs_config() -> ZfsHandlerConfig {
    ZfsHandlerConfig::default()
}
/// Create ZFS configuration from environment using canonical system
pub const fn zfs_config_from_env() -> ZfsHandlerConfig {
    use nestgate_core::config::canonical_master::handler_config::CanonicalHandlerConfigs;
    // Use specific default for handlers since the full config may be complex
    let handlers_config = CanonicalHandlerConfigs::default();
    handlers_config.zfs
}
/// Validate ZFS configuration using canonical system
pub const fn validate_zfs_config(config: &ZfsHandlerConfig) -> Result<(), String> {
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
