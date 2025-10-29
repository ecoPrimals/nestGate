//! **CANONICAL ZFS CONFIGURATION - UNIFIED**
//!
//! This module provides the canonical ZFS configuration that integrates with
//! `NestGate`'s unified configuration system, replacing all fragmented ZFS configs.
//! **MIGRATED**: Now uses `StandardDomainConfig` pattern for consistency

use nestgate_core::unified_config_consolidation::StandardDomainConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// ==================== UNIFIED ZFS CONFIGURATION ====================

/// **MIGRATED**: ZFS configuration now uses `StandardDomainConfig` pattern
/// This replaces the old fragmented ZFS configs with unified configuration
pub type ZfsConfig = StandardDomainConfig<ZfsExtensions>;
/// ZFS-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Default)]
pub struct ZfsExtensions {
    /// ZFS pool configuration
    pub pools: ZfsPoolConfig,
    /// ZFS dataset configuration  
    pub datasets: ZfsDatasetConfig,
    /// ZFS performance settings
    pub performance: ZfsPerformanceConfig,
    /// ZFS monitoring and health
    pub monitoring: ZfsMonitoringConfig,
    /// ZFS snapshot configuration
    pub snapshots: ZfsSnapshotConfig,
    /// ZFS migration settings
    pub migration: ZfsMigrationConfig,
}

// ==================== ZFS DOMAIN CONFIGURATIONS ====================

/// ZFS pool configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPoolConfig {
    pub default_pool_name: String,
    pub max_pools: u32,
    pub auto_discovery: bool,
    pub health_check_interval: Duration,
    pub default_properties: HashMap<String, String>,
    pub auto_pool_creation: bool,
}
/// ZFS dataset configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsDatasetConfig {
    pub max_datasets_per_pool: u32,
    pub default_compression: String,
    pub default_deduplication: bool,
    pub auto_snapshot: bool,
    pub quota_enforcement: bool,
}
/// ZFS performance configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPerformanceConfig {
    pub arc_cache: ArcCacheConfig,
    pub l2arc: L2ArcConfig,
    pub zil: ZilConfig,
    pub prefetch: PrefetchConfig,
}
/// ARC cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcCacheConfig {
    pub min_size_mb: u64,
    pub max_size_mb: u64,
    pub target_size_mb: Option<u64>,
}
impl Default for ArcCacheConfig {
    fn default() -> Self {
        Self {
            min_size_mb: 64,
            max_size_mb: 0, // 0 means auto
            target_size_mb: None,
        }
    }
}

/// L2ARC configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct L2ArcConfig {
    pub enabled: bool,
    pub device_path: Option<PathBuf>,
    pub write_boost: u64,
    pub headroom: u64,
}
/// ZIL configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZilConfig {
    pub enabled: bool,
    pub device_path: Option<PathBuf>,
    pub sync_policy: String,
}
/// Prefetch configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrefetchConfig {
    pub enabled: bool,
    pub streams: u32,
    pub max_distance: u64,
}
/// ZFS monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsMonitoringConfig {
    pub health_check_enabled: bool,
    pub health_check_interval: Duration,
    pub metrics_collection: bool,
    pub alert_thresholds: AlertThresholds,
}
/// Alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub pool_usage_warning: f64,
    pub pool_usage_critical: f64,
    pub scrub_age_warning_days: u32,
}
impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            pool_usage_warning: 80.0,
            pool_usage_critical: 95.0,
            scrub_age_warning_days: 35,
        }
    }
}

/// ZFS snapshot configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsSnapshotConfig {
    pub auto_snapshot: bool,
    pub snapshot_frequency: Duration,
    pub retention_policy: RetentionPolicy,
    pub naming_convention: String,
}
/// Snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub keep_hourly: u32,
    pub keep_daily: u32,
    pub keep_weekly: u32,
    pub keep_monthly: u32,
}
impl Default for RetentionPolicy {
    fn default() -> Self {
        Self {
            keep_hourly: 24,
            keep_daily: 7,
            keep_weekly: 4,
            keep_monthly: 12,
        }
    }
}

/// ZFS migration configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsMigrationConfig {
    pub migration_enabled: bool,
    pub bandwidth_limit_mbps: Option<u64>,
    pub compression_during_migration: bool,
    pub verification_enabled: bool,
}
// ==================== CONFIGURATION FACTORY METHODS ====================

/* // Cannot implement methods for external type - entire impl block commented out
impl ZfsConfig {
    /// Create production ZFS configuration
    pub fn production() -> Self {
        let mut config = Self::default();

        // Configure for production workloads
        config.extensions.pools.max_pools = 10;
        config.extensions.pools.auto_discovery = true;
        config.extensions.pools.health_check_interval = Duration::from_secs(300);

        config.extensions.datasets.max_datasets_per_pool = 1000;
        config.extensions.datasets.default_compression = "lz4".to_string();
        config.extensions.datasets.auto_snapshot = true;

        config.extensions.performance.arc_cache.max_size_mb = 0; // Auto
        config.extensions.performance.l2arc.enabled = true;

        config.extensions.monitoring.health_check_enabled = true;
        config.extensions.monitoring.metrics_collection = true;

        config
    }

    /// Create development ZFS configuration
    pub fn development() -> Self {
        let mut config = Self::default();

        // Configure for development
        config.extensions.pools.max_pools = 3;
        config.extensions.pools.auto_discovery = false;
        config.extensions.pools.health_check_interval = Duration::from_secs(600);

        config.extensions.datasets.max_datasets_per_pool = 100;
        config.extensions.datasets.default_compression = "lz4".to_string();
        config.extensions.datasets.auto_snapshot = false;

        config.extensions.performance.arc_cache.max_size_mb = 512;
        config.extensions.performance.l2arc.enabled = false;

        config.extensions.monitoring.health_check_enabled = false;
        config.extensions.monitoring.metrics_collection = false;

        config
    }

    /// Create testing ZFS configuration
    pub fn testing() -> Self {
        let mut config = Self::default();

        // Configure for testing
        config.extensions.pools.max_pools = 1;
        config.extensions.pools.auto_discovery = false;
        config.extensions.pools.health_check_interval = Duration::from_secs(60);

        config.extensions.datasets.max_datasets_per_pool = 10;
        config.extensions.datasets.default_compression = "off".to_string();
        config.extensions.datasets.auto_snapshot = false;

        config.extensions.performance.arc_cache.max_size_mb = 64;
        config.extensions.performance.l2arc.enabled = false;

        config.extensions.monitoring.health_check_enabled = false;
        config.extensions.monitoring.metrics_collection = false;

        config
    }
}

// ==================== VALIDATION METHODS ====================

impl ZfsExtensions {
    /// Validate ZFS configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate(&self) -> Result<()>  {
        // Validate pool settings
        if self.pools.default_pool_name.is_empty() {
            return Err(NestGateError::validation("pool_name"));
        }

        if self.pools.max_pools == 0 {
            return Err(NestGateError::validation("max_pools"));
        }

        // Validate dataset settings
        if self.datasets.max_datasets_per_pool == 0 {
            return Err(NestGateError::validation("max_datasets_per_pool"));
        }

        // Validate performance settings
        if self.performance.arc_cache.min_size_mb > self.performance.arc_cache.max_size_mb
            && self.performance.arc_cache.max_size_mb > 0
        {
            return Err(NestGateError::validation("arc_cache"));
        }

        Ok(())
    }
} // End of commented impl block */
