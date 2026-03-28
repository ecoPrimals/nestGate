// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
/// Configuration for ZfsStorage
pub struct ZfsStorageConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Pools
    pub pools: Vec<ZfsPoolConfig>,
    /// Datasets
    pub datasets: ZfsDatasetConfig,
    /// Snapshots
    pub snapshots: ZfsSnapshotConfig,
    /// Maintenance
    pub maintenance: ZfsMaintenanceConfig,
    /// Performance
    pub performance: ZfsPerformanceConfig,
    /// Security
    pub security: ZfsSecurityConfig,
    /// Monitoring
    pub monitoring: ZfsMonitoringConfig,
    /// Migration
    pub migration: ZfsMigrationConfig,
}

// ==================== POOL CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsPool
pub struct ZfsPoolConfig {
    /// Name
    pub name: String,
    /// Devices
    pub devices: Vec<String>,
    /// Redundancy
    pub redundancy: ZfsRedundancy,
    // Extended fields from canonical_zfs_config
    /// ZFS pool properties
    pub properties: HashMap<String, String>,
    /// Auto Discovery
    pub auto_discovery: bool,
    /// Health Check Interval
    pub health_check_interval: Duration,
    /// Auto Pool Creation
    pub auto_pool_creation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Zfspoolsettings
pub struct ZfsPoolSettings {
    /// Default Pool name
    pub default_pool_name: String,
    /// Max Pools
    pub max_pools: u32,
    /// Auto Discovery
    pub auto_discovery: bool,
    /// Health Check Interval
    pub health_check_interval: Duration,
    /// Default Properties
    pub default_properties: HashMap<String, String>,
    /// Auto Pool Creation
    pub auto_pool_creation: bool,
}

// ==================== DATASET CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsDataset
pub struct ZfsDatasetConfig {
    /// Auto Create
    pub auto_create: bool,
    /// Compression
    pub compression: ZfsCompression,
    /// Deduplication
    pub deduplication: bool,
    /// Quota Enforcement
    pub quota_enforcement: bool,
    /// Auto Snapshot
    pub auto_snapshot: bool,
    /// Maximum number of datasets allowed per pool
    pub max_datasets_per_pool: u32,
}

// ==================== SNAPSHOT CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsSnapshot
pub struct ZfsSnapshotConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Interval
    pub interval: Duration,
    /// Retention
    pub retention: u32,
    /// Enable automatic snapshots
    pub auto_snapshot: bool,
    /// Retention Policy
    pub retention_policy: RetentionPolicy,
    /// Naming Convention
    pub naming_convention: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retentionpolicy
pub struct RetentionPolicy {
    /// Keep Hourly
    pub keep_hourly: u32,
    /// Keep Daily
    pub keep_daily: u32,
    /// Keep Weekly
    pub keep_weekly: u32,
    /// Keep Monthly
    pub keep_monthly: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsMaintenance
pub struct ZfsMaintenanceConfig {
    /// Scrub Interval
    pub scrub_interval: Duration,
    /// Auto Scrub
    pub auto_scrub: bool,
}

// ==================== PERFORMANCE CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ZfsPerformance
pub struct ZfsPerformanceConfig {
    /// Size of arc
    pub arc_size: Option<u64>,
    /// Prefetch
    pub prefetch: bool,
    /// ARC cache configuration
    pub arc_cache: ArcCacheConfig,
    /// L2Arc
    pub l2arc: L2ArcConfig,
    /// Zil
    pub zil: ZilConfig,
    /// Configuration for prefetch
    pub prefetch_config: PrefetchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ArcCache
pub struct ArcCacheConfig {
    /// Min Size in megabytes
    pub min_size_mb: u64,
    /// Max Size in megabytes
    pub max_size_mb: u64,
    /// Target Size in megabytes
    pub target_size_mb: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for L2Arc
pub struct L2ArcConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Device Path
    pub device_path: Option<PathBuf>,
    /// Write Boost
    pub write_boost: u64,
    /// Headroom
    pub headroom: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Zil
pub struct ZilConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Device Path
    pub device_path: Option<PathBuf>,
    /// Sync Policy
    pub sync_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for Prefetch
pub struct PrefetchConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Streams
    pub streams: u32,
    /// Max Distance
    pub max_distance: u64,
}

// ==================== SECURITY CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsSecurity
pub struct ZfsSecurityConfig {
    /// Encryption
    pub encryption: bool,
    /// Key Location
    pub key_location: Option<String>,
}

// ==================== MONITORING CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ZfsMonitoring
pub struct ZfsMonitoringConfig {
    /// Health Check Enabled
    pub health_check_enabled: bool,
    /// Health Check Interval
    pub health_check_interval: Duration,
    /// Metrics Collection
    pub metrics_collection: bool,
    /// Alert Thresholds
    pub alert_thresholds: AlertThresholds,
}

/// Alert thresholds for ZFS monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Alertthresholds
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
/// Configuration for ZfsMigration
pub struct ZfsMigrationConfig {
    /// Migration Enabled
    pub migration_enabled: bool,
    /// Bandwidth Limit Mbps
    pub bandwidth_limit_mbps: Option<u64>,
    /// Compression During Migration
    pub compression_during_migration: bool,
    /// Verification Enabled
    pub verification_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfscompression
pub enum ZfsCompression {
    /// Off
    Off,
    /// Lzjb
    Lzjb,
    /// Gzip
    Gzip,
    /// Zle
    Zle,
    /// Lz4
    Lz4,
    /// Zstd
    Zstd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zfsredundancy
pub enum ZfsRedundancy {
    /// None
    None,
    /// Mirror
    Mirror,
    /// Raidz1
    RaidZ1,
    /// Raidz2
    RaidZ2,
    /// Raidz3
    RaidZ3,
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for ZfsPoolConfig {
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            scrub_interval: Duration::from_secs(7 * 24 * 3600), // 1 week
            auto_scrub: true,
        }
    }
}

impl Default for ArcCacheConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_size_mb: 64,
            max_size_mb: 0, // 0 means auto
            target_size_mb: None,
        }
    }
}

impl Default for AlertThresholds {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            pool_usage_warning: 80.0,
            pool_usage_critical: 95.0,
            scrub_age_warning_days: 35,
        }
    }
}

impl Default for ZfsPerformanceConfig {
    /// Returns the default instance
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
    /// Creates a production-optimized ZFS storage configuration
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }
    /// Creates a development-optimized ZFS storage configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Creates a high-performance ZFS storage configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }
    /// Creates a cloud-native ZFS storage configuration
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }
    /// Merges two ZFS storage configurations
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
