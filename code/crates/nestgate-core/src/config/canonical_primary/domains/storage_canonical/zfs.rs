// **ZFS STORAGE CONFIGURATION**
//!
//! **CANONICAL ZFS CONFIGURATION** - Single source of truth for all ZFS settings
//!
//! This module consolidates all ZFS configuration from:
//! - `nestgate-zfs/src/canonical_zfs_config.rs` (old StandardDomainConfig pattern)
//! - `nestgate-zfs/src/config/unified_zfs_config.rs` (wrapper/re-export)
//! - Various specialized configs throughout the codebase
//!
//! Migration Date: November 7, 2025
//! Pattern: Following proven NetworkConfig migration success

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// ==================== PRIMARY ZFS CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsStorageConfig {
    pub enabled: bool,
    pub pools: Vec<ZfsPoolConfig>,
    pub datasets: ZfsDatasetConfig,
    pub snapshots: ZfsSnapshotConfig,
    pub maintenance: ZfsMaintenanceConfig,
    pub performance: ZfsPerformanceConfig,
    pub security: ZfsSecurityConfig,
    pub monitoring: ZfsMonitoringConfig,
    pub migration: ZfsMigrationConfig,
}

// ==================== POOL CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolConfig {
    pub name: String,
    pub devices: Vec<String>,
    pub redundancy: ZfsRedundancy,
    // Extended fields from canonical_zfs_config
    pub properties: HashMap<String, String>,
    pub auto_discovery: bool,
    pub health_check_interval: Duration,
    pub auto_pool_creation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPoolSettings {
    pub default_pool_name: String,
    pub max_pools: u32,
    pub auto_discovery: bool,
    pub health_check_interval: Duration,
    pub default_properties: HashMap<String, String>,
    pub auto_pool_creation: bool,
}

// ==================== DATASET CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetConfig {
    pub auto_create: bool,
    pub compression: ZfsCompression,
    pub deduplication: bool,
    // Extended fields from canonical_zfs_config
    pub max_datasets_per_pool: u32,
    pub quota_enforcement: bool,
    pub auto_snapshot: bool,
}

// ==================== SNAPSHOT CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSnapshotConfig {
    pub enabled: bool,
    pub interval: Duration,
    pub retention: u32,
    // Extended fields from canonical_zfs_config
    pub auto_snapshot: bool,
    pub retention_policy: RetentionPolicy,
    pub naming_convention: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub keep_hourly: u32,
    pub keep_daily: u32,
    pub keep_weekly: u32,
    pub keep_monthly: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMaintenanceConfig {
    pub scrub_interval: Duration,
    pub auto_scrub: bool,
}

// ==================== PERFORMANCE CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceConfig {
    pub arc_size: Option<u64>,
    pub prefetch: bool,
    // Extended fields from canonical_zfs_config
    pub arc_cache: ArcCacheConfig,
    pub l2arc: L2ArcConfig,
    pub zil: ZilConfig,
    pub prefetch_config: PrefetchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcCacheConfig {
    pub min_size_mb: u64,
    pub max_size_mb: u64,
    pub target_size_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct L2ArcConfig {
    pub enabled: bool,
    pub device_path: Option<PathBuf>,
    pub write_boost: u64,
    pub headroom: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZilConfig {
    pub enabled: bool,
    pub device_path: Option<PathBuf>,
    pub sync_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrefetchConfig {
    pub enabled: bool,
    pub streams: u32,
    pub max_distance: u64,
}

// ==================== SECURITY CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsSecurityConfig {
    pub encryption: bool,
    pub key_location: Option<String>,
}

// ==================== MONITORING CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsMonitoringConfig {
    pub health_check_enabled: bool,
    pub health_check_interval: Duration,
    pub metrics_collection: bool,
    pub alert_thresholds: AlertThresholds,
}

/// Alert thresholds for ZFS monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// Pool usage percentage that triggers a warning alert
    pub pool_usage_warning: f64,
    /// Pool usage percentage that triggers a critical alert
    pub pool_usage_critical: f64,
    /// Number of days since last scrub before warning
    pub scrub_age_warning_days: u32,
}

// ==================== MIGRATION CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsMigrationConfig {
    pub migration_enabled: bool,
    pub bandwidth_limit_mbps: Option<u64>,
    pub compression_during_migration: bool,
    pub verification_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsCompression {
    Off,
    Lzjb,
    Gzip,
    Zle,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsRedundancy {
    None,
    Mirror,
    RaidZ1,
    RaidZ2,
    RaidZ3,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for ZfsPoolConfig {
    fn default() -> Self {
        Self {
            name: "default-pool".to_string(),
            devices: Vec::new(),
            redundancy: ZfsRedundancy::None,
            properties: HashMap::new(),
            auto_discovery: true,
            health_check_interval: Duration::from_secs(300),
            auto_pool_creation: false,
        }
    }
}

impl Default for ZfsDatasetConfig {
    fn default() -> Self {
        Self {
            auto_create: true,
            compression: ZfsCompression::Lz4,
            deduplication: false,
            max_datasets_per_pool: 1000,
            quota_enforcement: true,
            auto_snapshot: true,
        }
    }
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

impl Default for ZfsSnapshotConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(3600), // 1 hour
            retention: 24,                       // 24 snapshots
            auto_snapshot: true,
            retention_policy: RetentionPolicy::default(),
            naming_convention: "auto-%Y%m%d-%H%M%S".to_string(),
        }
    }
}

impl Default for ZfsMaintenanceConfig {
    fn default() -> Self {
        Self {
            scrub_interval: Duration::from_secs(7 * 24 * 3600), // 1 week
            auto_scrub: true,
        }
    }
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

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            pool_usage_warning: 80.0,
            pool_usage_critical: 95.0,
            scrub_age_warning_days: 35,
        }
    }
}

impl Default for ZfsPerformanceConfig {
    fn default() -> Self {
        Self {
            arc_size: None,
            prefetch: true,
            arc_cache: ArcCacheConfig::default(),
            l2arc: L2ArcConfig::default(),
            zil: ZilConfig::default(),
            prefetch_config: PrefetchConfig::default(),
        }
    }
}

impl ZfsStorageConfig {
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
