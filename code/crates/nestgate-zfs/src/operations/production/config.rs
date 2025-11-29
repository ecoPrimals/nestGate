// **ZFS OPERATIONS CONFIGURATION**
///
// Configuration management for production ZFS operations

//! Config module

use serde::{Deserialize, Serialize};
use std::time::Duration;
use nestgate_core::error::Result;
use nestgate_core::constants::zfs;

// ==================== ZFS OPERATIONS CONFIGURATION ====================

/// **ZFS OPERATIONS CONFIGURATION**
///
/// Configuration for production ZFS operations with safe defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsOperations
pub struct ZfsOperationsConfig {
    /// Whether to use sudo for ZFS commands
    pub use_sudo: bool,
    /// Command timeout
    pub command_timeout: Duration,
    /// Maximum concurrent operations
    pub max_concurrent_operations: usize,
    /// Enable command caching
    pub enable_caching: bool,
    /// Cache TTL
    pub cache_ttl: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Metrics collection interval
    pub metrics_interval: Duration,
}

impl Default for ZfsOperationsConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            use_sudo: true,
            command_timeout: zfs::DEFAULT_COMMAND_TIMEOUT,
            max_concurrent_operations: zfs::MAX_CONCURRENT_OPERATIONS,
            enable_caching: true,
            cache_ttl: zfs::COMMAND_CACHE_TTL,
            health_check_interval: zfs::HEALTH_CHECK_INTERVAL,
            metrics_interval: zfs::METRICS_COLLECTION_INTERVAL,
        }
    }
}

/// **ZFS PRODUCTION CONFIGURATION**
///
/// Production-optimized ZFS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsProduction
pub struct ZfsProductionConfig {
    /// Base operations config
    pub operations: ZfsOperationsConfig,
    /// Production-specific settings
    pub production_mode: bool,
    /// Enhanced logging
    pub verbose_logging: bool,
    /// Automatic recovery
    pub auto_recovery: bool,
}

impl Default for ZfsProductionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            operations: ZfsOperationsConfig {
                command_timeout: zfs::LONG_COMMAND_TIMEOUT,
                max_concurrent_operations: zfs::MAX_CONCURRENT_OPERATIONS,
                ..Default::default()
            },
            production_mode: true,
            verbose_logging: true,
            auto_recovery: true,
        }
    }
}

impl ZfsOperationsConfig {
    /// Create development configuration
    pub fn development() -> Self {
        Self {
            use_sudo: false,
            command_timeout: Duration::from_secs(10),
            max_concurrent_operations: 5,
            enable_caching: false,
            cache_ttl: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            metrics_interval: Duration::from_secs(10),
        }
    }

    /// Create production configuration
    pub fn production() -> Self {
        Self {
            use_sudo: true,
            command_timeout: zfs::LONG_COMMAND_TIMEOUT,
            max_concurrent_operations: zfs::MAX_CONCURRENT_OPERATIONS * 2,
            enable_caching: true,
            cache_ttl: zfs::COMMAND_CACHE_TTL,
            health_check_interval: zfs::HEALTH_CHECK_INTERVAL,
            metrics_interval: zfs::METRICS_COLLECTION_INTERVAL,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), NestGateUnifiedError> {
        if self.max_concurrent_operations == 0 {
            return Err(nestgate_core::error::NestGateUnifiedError::Configuration(
                Box::new(nestgate_core::error::ConfigurationErrorDetails {
                    field: "max_concurrent_operations".to_string(),
                    message: "Must be greater than 0".to_string(),
                    currentvalue: Some("0".to_string()),
                    expected: Some("> 0".to_string()),
                    user_error: true,
                })
            ));
        }

        if self.command_timeout.as_secs() == 0 {
            return Err(nestgate_core::error::NestGateUnifiedError::Configuration(
                Box::new(nestgate_core::error::ConfigurationErrorDetails {
                    field: "command_timeout".to_string(),
                    message: "Must be greater than 0".to_string(),
                    currentvalue: Some("0".to_string()),
                    expected: Some("> 0 seconds".to_string()),
                    user_error: true,
                })
            ));
        }

        Ok(())
    }
} 