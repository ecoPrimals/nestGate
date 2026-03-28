// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **ZFS DOMAIN CONFIGURATION**
//!
//! Consolidates all ZFS-related configurations:
//! - Pool management (`PoolConfig`, `DatasetConfig`, `SnapshotConfig`)
//! - Service configuration (`ZfsServiceConfig`, `ZfsHandlerConfig`)
//! - Performance and monitoring configurations
//!
//! ## Architecture
//!
//! ```text
//! ZfsDomainConfig
//! ├── pools: Pool management
//! ├── datasets: Dataset management
//! ├── snapshots: Snapshot policies
//! ├── service: Service configuration
//! ├── performance: Performance tuning
//! ├── monitoring: Metrics collection
//! └── failover: High availability
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_core::config::canonical_primary::domains::consolidated_domains::zfs::*;
//!
//! let zfs_config = ZfsDomainConfig::default();
//! assert_eq!(zfs_config.pools.default_pool, "tank");
//! ```

use super::validation::DomainConfigValidation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== ZFS DOMAIN CONFIGURATION ====================

/// **ZFS DOMAIN CONFIGURATION**
///
/// Consolidates all ZFS-related configurations including pools, datasets,
/// snapshots, and operational settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsDomainConfig {
    /// Pool management configuration
    pub pools: ZfsPoolsConfig,

    /// Dataset management configuration
    pub datasets: ZfsDatasetsConfig,

    /// Snapshot management configuration
    pub snapshots: ZfsSnapshotsConfig,

    /// ZFS service configuration
    pub service: ZfsServiceConfig,

    /// Performance optimization configuration
    pub performance: ZfsPerformanceConfig,

    /// Monitoring and metrics configuration
    pub monitoring: ZfsMonitoringConfig,

    /// Failover and redundancy configuration
    pub failover: ZfsFailoverConfig,
}

// ==================== POOL CONFIGURATION ====================

/// ZFS pools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolsConfig {
    /// Default pool settings
    pub default_pool: String,

    /// Pool-specific configurations
    pub pool_configs: HashMap<String, ZfsPoolConfig>,

    /// Auto-creation settings
    pub auto_create: bool,

    /// Scrub scheduling (cron format)
    pub scrub_schedule: Option<String>,
}

impl Default for ZfsPoolsConfig {
    fn default() -> Self {
        Self {
            default_pool: "tank".to_string(),
            pool_configs: HashMap::new(),
            auto_create: false,
            scrub_schedule: Some("0 2 * * 0".to_string()), // Weekly at 2 AM Sunday
        }
    }
}

/// Individual ZFS pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolConfig {
    /// Pool name
    pub name: String,

    /// Pool size limits
    pub size_limits: ZfsSizeLimits,

    /// Compression settings
    pub compression: ZfsCompressionConfig,

    /// Deduplication settings
    pub deduplication: bool,

    /// Custom properties
    pub properties: HashMap<String, String>,
}

/// Size limits for ZFS operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsSizeLimits {
    /// Minimum size in bytes
    pub min_size: u64,
    /// Maximum size in bytes
    pub max_size: u64,
}

/// Configuration for ZFS compression settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsCompressionConfig {
    /// Compression algorithm (e.g., "lz4", "zstd")
    pub algorithm: String,
    /// Compression level (0-9)
    pub level: u8,
}

// ==================== DATASET CONFIGURATION ====================

/// ZFS datasets configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsDatasetsConfig {
    /// Default dataset settings
    pub defaults: ZfsDatasetDefaults,

    /// Dataset-specific configurations
    pub dataset_configs: HashMap<String, ZfsDatasetConfig>,

    /// Auto-snapshot settings
    pub auto_snapshot: ZfsAutoSnapshotConfig,
}

/// Default settings for ZFS datasets
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsDatasetDefaults {
    /// Default quota in bytes
    pub quota: Option<u64>,
    /// Default reservation in bytes
    pub reservation: Option<u64>,
}

/// Configuration for a specific ZFS dataset
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsDatasetConfig {
    /// Dataset name
    pub name: String,
    /// Quota in bytes
    pub quota: Option<u64>,
}

/// Configuration for ZFS automatic snapshot scheduling
///
/// Controls whether automatic snapshots are taken and at what interval.
/// Snapshots provide point-in-time recovery capabilities for ZFS datasets.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsAutoSnapshotConfig {
    /// Enable or disable automatic snapshots
    pub enabled: bool,
    /// Interval between automatic snapshots
    pub interval: Duration,
}

// ==================== SNAPSHOT CONFIGURATION ====================

/// ZFS snapshots configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSnapshotsConfig {
    /// Retention policies
    pub retention: ZfsRetentionConfig,

    /// Snapshot naming patterns
    pub naming_pattern: String,

    /// Automatic snapshot intervals
    pub intervals: ZfsSnapshotIntervals,

    /// Cleanup policies
    pub cleanup: ZfsCleanupConfig,
}

impl Default for ZfsSnapshotsConfig {
    fn default() -> Self {
        Self {
            retention: ZfsRetentionConfig::default(),
            naming_pattern: "%Y%m%d_%H%M%S".to_string(),
            intervals: ZfsSnapshotIntervals::default(),
            cleanup: ZfsCleanupConfig::default(),
        }
    }
}

/// Configuration for ZFS snapshot retention policies
///
/// Defines how many snapshots to retain at each time interval.
/// Older snapshots beyond these limits are automatically cleaned up.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsRetentionConfig {
    /// Number of daily snapshots to retain
    pub daily: u32,
    /// Number of weekly snapshots to retain
    pub weekly: u32,
    /// Number of monthly snapshots to retain
    pub monthly: u32,
}

/// Configuration for ZFS snapshot interval scheduling
///
/// Controls which snapshot intervals are enabled. Multiple intervals
/// can be active simultaneously for comprehensive backup coverage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsSnapshotIntervals {
    /// Enable hourly snapshots
    pub hourly: bool,
    /// Enable daily snapshots
    pub daily: bool,
    /// Enable weekly snapshots
    pub weekly: bool,
}

/// Configuration for ZFS automatic cleanup behavior
///
/// Controls when and how ZFS performs automatic cleanup of old snapshots
/// and other temporary data to prevent disk space exhaustion.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsCleanupConfig {
    /// Enable automatic cleanup of old snapshots
    pub auto_cleanup: bool,
    /// Disk usage threshold (0.0-1.0) that triggers cleanup
    pub cleanup_threshold: f64,
}

// ==================== SERVICE CONFIGURATION ====================

/// Configuration for ZFS service-level settings
///
/// Placeholder for future ZFS service configuration options such as
/// service discovery, health checks, and service-specific parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsServiceConfig {}

/// Configuration for ZFS performance tuning
///
/// Placeholder for future ZFS performance configuration options such as
/// cache sizes, I/O scheduling, and prefetch behavior.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPerformanceConfig {}

/// Configuration for ZFS monitoring and metrics
///
/// Placeholder for future ZFS monitoring configuration options such as
/// metrics collection intervals, alert thresholds, and logging levels.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsMonitoringConfig {}

/// Configuration for ZFS high-availability and failover
///
/// Placeholder for future ZFS failover configuration options such as
/// replication targets, failover policies, and health check intervals.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsFailoverConfig {}

// ==================== VALIDATION ====================

impl DomainConfigValidation for ZfsDomainConfig {
    fn validate(&self) -> nestgate_types::error::Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate pool configuration
        if self.pools.default_pool.is_empty() {
            warnings.push("Default pool name is empty".to_string());
        }

        // Validate snapshot retention
        if self.snapshots.retention.daily == 0
            && self.snapshots.retention.weekly == 0
            && self.snapshots.retention.monthly == 0
        {
            warnings.push("No snapshot retention configured".to_string());
        }

        Ok(warnings)
    }

    fn validate_for_environment(&self, _env: &str) -> nestgate_types::error::Result<()> {
        // Environment-specific validation can be added here
        Ok(())
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["pools.default_pool"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["pools.scrub_schedule", "datasets.auto_snapshot"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zfs_domain_config_default() {
        let config = ZfsDomainConfig::default();
        assert_eq!(config.pools.default_pool, "tank");
    }

    #[test]
    fn test_zfs_pools_config() {
        let pools = ZfsPoolsConfig::default();
        assert_eq!(pools.default_pool, "tank");
        assert!(!pools.auto_create);
        assert!(pools.scrub_schedule.is_some());
    }

    #[test]
    fn test_zfs_snapshots_config() {
        let snapshots = ZfsSnapshotsConfig::default();
        assert_eq!(snapshots.naming_pattern, "%Y%m%d_%H%M%S");
    }

    #[test]
    fn test_validation() {
        let config = ZfsDomainConfig::default();
        let warnings = config.validate().expect("Should validate");
        // Default config should have minimal warnings
        assert!(warnings.len() <= 1);
    }

    #[test]
    fn test_environment_validation() {
        let config = ZfsDomainConfig::default();
        assert!(config.validate_for_environment("production").is_ok());
        assert!(config.validate_for_environment("dev").is_ok());
    }
}
