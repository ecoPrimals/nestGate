//
// This module provides the canonical ZFS configuration that integrates with
// NestGate's unified configuration system, replacing all fragmented ZFS configs.

use nestgate_core::config::canonical_unified::NestGateCanonicalUnifiedConfig;
use nestgate_core::error::{NestGateError, Result};
use nestgate_core::traits_root::config::Configuration;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// **CANONICAL ZFS CONFIGURATION**
/// Single source of truth for all ZFS-related configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalZfsConfig {
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

// Note: CanonicalDomainConfig trait implementation removed as trait is not available
// The following methods would be part of the trait implementation:

impl CanonicalZfsConfig {
    /// Get the domain name for this configuration
    pub fn domain() -> &'static str {
        "zfs"
    }

    /// Validate the configuration (placeholder implementation)
    fn _validate(&self) -> Result<()> {
        // Validate pool settings
        if self.pools.default_pool_name.is_empty() {
            return Err(NestGateError::validation_error("pool_name", "ZFS pool name cannot be empty"));
        }

        if self.pools.max_pools == 0 {
            return Err(NestGateError::validation_error(
                "max_pools",
                "Maximum pools must be greater than 0"
            ));
        }

        // Validate dataset settings
        if self.datasets.max_datasets_per_pool == 0 {
            return Err(NestGateError::validation_error(
                "max_datasets_per_pool",
                "Maximum datasets per pool must be greater than 0"
            ));
        }

        // Validate performance settings
        if self.performance.arc_cache.min_size_mb > self.performance.arc_cache.max_size_mb
            && self.performance.arc_cache.max_size_mb > 0
        {
            return Err(NestGateError::validation_error(
                "arc_cache",
                "ARC minimum size cannot be greater than maximum size"
            ));
        }

        Ok(())
    }

    /// Merge configurations (placeholder implementation)
    fn _merge(self, other: CanonicalZfsConfig) -> CanonicalZfsConfig {
        CanonicalZfsConfig {
            pools: self.pools.merge(other.pools),
            datasets: self.datasets.merge(other.datasets),
            performance: self.performance.merge(other.performance),
            monitoring: self.monitoring.merge(other.monitoring),
            snapshots: self.snapshots.merge(other.snapshots),
            migration: self.migration.merge(other.migration),
        }
    }

    /// Create configuration from environment variables (placeholder implementation)
    fn _from_environment() -> Result<CanonicalZfsConfig> {
        use std::env;

        let mut config = CanonicalZfsConfig::default();

        // Load pool settings from environment
        if let Ok(pool_name) = env::var("NESTGATE_ZFS_POOL_NAME") {
            config.pools.default_pool_name = pool_name;
        }

        if let Ok(max_pools) = env::var("NESTGATE_ZFS_MAX_POOLS") {
            config.pools.max_pools = max_pools.parse().unwrap_or(config.pools.max_pools);
        }

        // Load dataset settings
        if let Ok(max_datasets) = env::var("NESTGATE_ZFS_MAX_DATASETS_PER_POOL") {
            config.datasets.max_datasets_per_pool = max_datasets
                .parse()
                .unwrap_or(config.datasets.max_datasets_per_pool);
        }

        // Load performance settings
        if let Ok(arc_min) = env::var("NESTGATE_ZFS_ARC_MIN_SIZE_MB") {
            config.performance.arc_cache.min_size_mb = arc_min
                .parse()
                .unwrap_or(config.performance.arc_cache.min_size_mb);
        }

        if let Ok(arc_max) = env::var("NESTGATE_ZFS_ARC_MAX_SIZE_MB") {
            config.performance.arc_cache.max_size_mb = arc_max
                .parse()
                .unwrap_or(config.performance.arc_cache.max_size_mb);
        }

        config.validate()?;
        Ok(config)
    }

    /// Get JSON schema for configuration (placeholder implementation)
    fn _schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "pools": {
                    "type": "object",
                    "properties": {
                        "default_pool_name": {"type": "string"},
                        "max_pools": {"type": "number", "minimum": 1}
                    }
                },
                "datasets": {
                    "type": "object",
                    "properties": {
                        "max_datasets_per_pool": {"type": "number", "minimum": 1}
                    }
                }
            },
            "required": ["pools", "datasets"]
        })
    }
}

// ==================== CONFIGURATION TRAIT IMPLEMENTATION ====================

impl Configuration for CanonicalZfsConfig {
    fn name(&self) -> &str {
        "canonical_zfs_config"
    }

    fn validate(&self) -> Result<()> {
        // Validate pool configuration
        if self.pools.default_pool_name.is_empty() {
            return Err(NestGateError::configuration_error(
                "validation",
                "Default pool name cannot be empty"
            ));
        }

        if self.pools.max_pools == 0 {
            return Err(NestGateError::configuration_error(
                "validation", 
                "Maximum pools must be greater than 0"
            ));
        }

        // Validate performance settings
        if self.performance.arc_cache.max_size_mb == 0 {
            return Err(NestGateError::configuration_error(
                "validation",
                "ARC cache max size must be greater than 0"
            ));
        }

        // Additional validations can be added here
        Ok(())
    }

    fn to_json(&self) -> Result<serde_json::Value> {
        serde_json::to_value(self).map_err(|e| NestGateError::configuration_error(
            "serialization",
            &format!("Failed to serialize ZFS config to JSON: {}", e)
        ))
    }
}

// ==================== ZFS CONFIGURATION COMPONENTS ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPoolConfig {
    pub default_pool_name: String,
    pub max_pools: u32,
    pub scrub_schedule: ZfsScrubSchedule,
    pub vdev_config: ZfsVdevConfig,
    pub enabled_features: Vec<String>,
}

impl ZfsPoolConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            default_pool_name: if other.default_pool_name.is_empty() {
                self.default_pool_name
            } else {
                other.default_pool_name
            },
            max_pools: if other.max_pools == 0 {
                self.max_pools
            } else {
                other.max_pools
            },
            scrub_schedule: self.scrub_schedule.merge(other.scrub_schedule),
            vdev_config: self.vdev_config.merge(other.vdev_config),
            enabled_features: if other.enabled_features.is_empty() {
                self.enabled_features
            } else {
                other.enabled_features
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsDatasetConfig {
    pub default_prefix: String,
    pub default_quota_mb: u64,
    pub default_reservation_mb: u64,
    pub default_compression: String,
    pub default_checksum: String,
    pub default_recordsize: String,
    pub max_datasets_per_pool: u32,
    pub encryption_algorithm: String,
    pub default_mount_options: Vec<String>,
}

impl ZfsDatasetConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            default_prefix: if other.default_prefix.is_empty() {
                self.default_prefix
            } else {
                other.default_prefix
            },
            default_quota_mb: if other.default_quota_mb == 0 {
                self.default_quota_mb
            } else {
                other.default_quota_mb
            },
            default_reservation_mb: if other.default_reservation_mb == 0 {
                self.default_reservation_mb
            } else {
                other.default_reservation_mb
            },
            default_compression: if other.default_compression.is_empty() {
                self.default_compression
            } else {
                other.default_compression
            },
            default_checksum: if other.default_checksum.is_empty() {
                self.default_checksum
            } else {
                other.default_checksum
            },
            default_recordsize: if other.default_recordsize.is_empty() {
                self.default_recordsize
            } else {
                other.default_recordsize
            },
            max_datasets_per_pool: if other.max_datasets_per_pool == 0 {
                self.max_datasets_per_pool
            } else {
                other.max_datasets_per_pool
            },
            encryption_algorithm: if other.encryption_algorithm.is_empty() {
                self.encryption_algorithm
            } else {
                other.encryption_algorithm
            },
            default_mount_options: if other.default_mount_options.is_empty() {
                self.default_mount_options
            } else {
                other.default_mount_options
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPerformanceConfig {
    pub arc_cache: ZfsArcCacheConfig,
    pub l2arc: ZfsL2ArcConfig,
    pub zil: ZfsZilConfig,
    pub prefetch: ZfsPrefetchConfig,
    pub io_scheduler: ZfsIoSchedulerConfig,
    pub optimization_level: String, // "conservative", "balanced", "aggressive"
    pub enable_performance_monitoring: bool,
}

impl ZfsPerformanceConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            arc_cache: self.arc_cache.merge(other.arc_cache),
            l2arc: self.l2arc.merge(other.l2arc),
            zil: self.zil.merge(other.zil),
            prefetch: self.prefetch.merge(other.prefetch),
            io_scheduler: self.io_scheduler.merge(other.io_scheduler),
            optimization_level: if other.optimization_level.is_empty() {
                self.optimization_level
            } else {
                other.optimization_level
            },
            enable_performance_monitoring: other.enable_performance_monitoring,
        }
    }
}

// ==================== SUPPORTING CONFIGURATION TYPES ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsScrubSchedule {
    pub enable_auto_scrub: bool,
    pub frequency_days: u32,
    pub bandwidth_limit_mbps: u32,
    pub maintenance_window_only: bool,
    pub maintenance_start_hour: u8,
    pub maintenance_duration_hours: u8,
}

impl ZfsScrubSchedule {
    fn merge(self, other: Self) -> Self {
        Self {
            enable_auto_scrub: other.enable_auto_scrub,
            frequency_days: if other.frequency_days == 0 {
                self.frequency_days
            } else {
                other.frequency_days
            },
            bandwidth_limit_mbps: if other.bandwidth_limit_mbps == 0 {
                self.bandwidth_limit_mbps
            } else {
                other.bandwidth_limit_mbps
            },
            maintenance_window_only: other.maintenance_window_only,
            maintenance_start_hour: other.maintenance_start_hour,
            maintenance_duration_hours: if other.maintenance_duration_hours == 0 {
                self.maintenance_duration_hours
            } else {
                other.maintenance_duration_hours
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsVdevConfig {
    pub enable_cache: bool,
    pub enable_log: bool,
    pub cache_device_paths: Vec<PathBuf>,
    pub log_device_paths: Vec<PathBuf>,
}

impl ZfsVdevConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            enable_cache: other.enable_cache,
            enable_log: other.enable_log,
            cache_device_paths: if other.cache_device_paths.is_empty() {
                self.cache_device_paths
            } else {
                other.cache_device_paths
            },
            log_device_paths: if other.log_device_paths.is_empty() {
                self.log_device_paths
            } else {
                other.log_device_paths
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsArcCacheConfig {
    pub min_size_mb: u64,
    pub max_size_mb: u64,
    pub meta_limit_mb: u64,
    pub enable_compression: bool,
    pub eviction_policy: String,
}

impl ZfsArcCacheConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            min_size_mb: if other.min_size_mb == 0 {
                self.min_size_mb
            } else {
                other.min_size_mb
            },
            max_size_mb: if other.max_size_mb == 0 {
                self.max_size_mb
            } else {
                other.max_size_mb
            },
            meta_limit_mb: if other.meta_limit_mb == 0 {
                self.meta_limit_mb
            } else {
                other.meta_limit_mb
            },
            enable_compression: other.enable_compression,
            eviction_policy: if other.eviction_policy.is_empty() {
                self.eviction_policy
            } else {
                other.eviction_policy
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsL2ArcConfig {
    pub enable_l2arc: bool,
    pub device_paths: Vec<PathBuf>,
    pub noprefetch: bool,
    pub enable_rebuild: bool,
}

impl ZfsL2ArcConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            enable_l2arc: other.enable_l2arc,
            device_paths: if other.device_paths.is_empty() {
                self.device_paths
            } else {
                other.device_paths
            },
            noprefetch: other.noprefetch,
            enable_rebuild: other.enable_rebuild,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsZilConfig {
    pub enable_separate_zil: bool,
    pub device_paths: Vec<PathBuf>,
    pub disable_zil: bool,
    pub sync_frequency: Duration,
}

impl ZfsZilConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            enable_separate_zil: other.enable_separate_zil,
            device_paths: if other.device_paths.is_empty() {
                self.device_paths
            } else {
                other.device_paths
            },
            disable_zil: other.disable_zil,
            sync_frequency: other.sync_frequency,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPrefetchConfig {
    pub enable_data_prefetch: bool,
    pub enable_metadata_prefetch: bool,
    pub max_streams: u32,
}

impl ZfsPrefetchConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            enable_data_prefetch: other.enable_data_prefetch,
            enable_metadata_prefetch: other.enable_metadata_prefetch,
            max_streams: if other.max_streams == 0 {
                self.max_streams
            } else {
                other.max_streams
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsIoSchedulerConfig {
    pub scheduler_type: String,
    pub delay_scale: u32,
    pub max_active_ios: u32,
}

impl ZfsIoSchedulerConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            scheduler_type: if other.scheduler_type.is_empty() {
                self.scheduler_type
            } else {
                other.scheduler_type
            },
            delay_scale: if other.delay_scale == 0 {
                self.delay_scale
            } else {
                other.delay_scale
            },
            max_active_ios: if other.max_active_ios == 0 {
                self.max_active_ios
            } else {
                other.max_active_ios
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsMonitoringConfig {
    pub enable_metrics_collection: bool,
    pub enable_event_monitoring: bool,
    pub enable_smart_monitoring: bool,
    pub monitoring_interval: Duration,
    pub health_check_interval: Duration,
    pub metrics_retention_days: u32,
    pub alert_thresholds: ZfsAlertThresholds,
}

impl ZfsMonitoringConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            enable_metrics_collection: other.enable_metrics_collection,
            enable_event_monitoring: other.enable_event_monitoring,
            enable_smart_monitoring: other.enable_smart_monitoring,
            monitoring_interval: other.monitoring_interval,
            health_check_interval: other.health_check_interval,
            metrics_retention_days: if other.metrics_retention_days == 0 {
                self.metrics_retention_days
            } else {
                other.metrics_retention_days
            },
            alert_thresholds: self.alert_thresholds.merge(other.alert_thresholds),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsAlertThresholds {
    pub pool_usage_warning: f64,
    pub pool_usage_critical: f64,
    pub fragmentation_warning: f64,
    pub fragmentation_critical: f64,
    pub temperature_warning: f64,
    pub temperature_critical: f64,
}

impl Default for ZfsAlertThresholds {
    fn default() -> Self {
        Self {
            pool_usage_warning: 80.0,
            pool_usage_critical: 90.0,
            fragmentation_warning: 70.0,
            fragmentation_critical: 85.0,
            temperature_warning: 55.0,
            temperature_critical: 65.0,
        }
    }
}

impl ZfsAlertThresholds {
    fn merge(self, other: Self) -> Self {
        Self {
            pool_usage_warning: if other.pool_usage_warning == 0.0 {
                self.pool_usage_warning
            } else {
                other.pool_usage_warning
            },
            pool_usage_critical: if other.pool_usage_critical == 0.0 {
                self.pool_usage_critical
            } else {
                other.pool_usage_critical
            },
            fragmentation_warning: if other.fragmentation_warning == 0.0 {
                self.fragmentation_warning
            } else {
                other.fragmentation_warning
            },
            fragmentation_critical: if other.fragmentation_critical == 0.0 {
                self.fragmentation_critical
            } else {
                other.fragmentation_critical
            },
            temperature_warning: if other.temperature_warning == 0.0 {
                self.temperature_warning
            } else {
                other.temperature_warning
            },
            temperature_critical: if other.temperature_critical == 0.0 {
                self.temperature_critical
            } else {
                other.temperature_critical
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsSnapshotConfig {
    pub enable_auto_snapshots: bool,
    pub snapshot_frequency: Duration,
    pub retention: ZfsSnapshotRetention,
    pub enable_incremental: bool,
    pub max_snapshots_per_dataset: u32,
}

impl ZfsSnapshotConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            enable_auto_snapshots: other.enable_auto_snapshots,
            snapshot_frequency: other.snapshot_frequency,
            retention: self.retention.merge(other.retention),
            enable_incremental: other.enable_incremental,
            max_snapshots_per_dataset: if other.max_snapshots_per_dataset == 0 {
                self.max_snapshots_per_dataset
            } else {
                other.max_snapshots_per_dataset
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsSnapshotRetention {
    pub hourly_retention_hours: u32,
    pub daily_retention_days: u32,
    pub weekly_retention_weeks: u32,
    pub monthly_retention_months: u32,
    pub yearly_retention_years: u32,
}

impl ZfsSnapshotRetention {
    fn merge(self, other: Self) -> Self {
        Self {
            hourly_retention_hours: if other.hourly_retention_hours == 0 {
                self.hourly_retention_hours
            } else {
                other.hourly_retention_hours
            },
            daily_retention_days: if other.daily_retention_days == 0 {
                self.daily_retention_days
            } else {
                other.daily_retention_days
            },
            weekly_retention_weeks: if other.weekly_retention_weeks == 0 {
                self.weekly_retention_weeks
            } else {
                other.weekly_retention_weeks
            },
            monthly_retention_months: if other.monthly_retention_months == 0 {
                self.monthly_retention_months
            } else {
                other.monthly_retention_months
            },
            yearly_retention_years: if other.yearly_retention_years == 0 {
                self.yearly_retention_years
            } else {
                other.yearly_retention_years
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsMigrationConfig {
    pub enable_migration: bool,
    pub retry_attempts: u32,
    pub retry_delay: Duration,
    pub bandwidth_limit_mbps: u32,
    pub timeout_minutes: u32,
    pub temp_space_mb: u64,
}

impl ZfsMigrationConfig {
    fn merge(self, other: Self) -> Self {
        Self {
            enable_migration: other.enable_migration,
            retry_attempts: if other.retry_attempts == 0 {
                self.retry_attempts
            } else {
                other.retry_attempts
            },
            retry_delay: other.retry_delay,
            bandwidth_limit_mbps: if other.bandwidth_limit_mbps == 0 {
                self.bandwidth_limit_mbps
            } else {
                other.bandwidth_limit_mbps
            },
            timeout_minutes: if other.timeout_minutes == 0 {
                self.timeout_minutes
            } else {
                other.timeout_minutes
            },
            temp_space_mb: if other.temp_space_mb == 0 {
                self.temp_space_mb
            } else {
                other.temp_space_mb
            },
        }
    }
}

// ==================== CANONICAL ZFS CONFIGURATION BUILDERS ====================

/// Canonical ZFS configuration builder with fluent API
#[derive(Debug, Clone, Default)]
pub struct CanonicalZfsConfigBuilder {
    config: CanonicalZfsConfig,
}

impl CanonicalZfsConfigBuilder {
    /// Create a new ZFS configuration builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set pool configuration
    pub fn with_pools(mut self, pools: ZfsPoolConfig) -> Self {
        self.config.pools = pools;
        self
    }

    /// Set dataset configuration
    pub fn with_datasets(mut self, datasets: ZfsDatasetConfig) -> Self {
        self.config.datasets = datasets;
        self
    }

    /// Set performance configuration
    pub fn with_performance(mut self, performance: ZfsPerformanceConfig) -> Self {
        self.config.performance = performance;
        self
    }

    /// Set monitoring configuration
    pub fn with_monitoring(mut self, monitoring: ZfsMonitoringConfig) -> Self {
        self.config.monitoring = monitoring;
        self
    }

    /// Build the configuration with validation
    pub fn build(self) -> Result<CanonicalZfsConfig> {
        self.config.validate()?;
        Ok(self.config)
    }

    /// Build with environment variable overrides
    pub fn build_with_env(self) -> Result<CanonicalZfsConfig> {
        // Placeholder implementation until trait methods are available
        // let env_config = CanonicalZfsConfig::from_environment()?;
        // let merged = self.config.merge(env_config);
        // merged.validate()?;
        Ok(self.config)
    }

    /// Create development configuration
    pub fn development() -> Result<CanonicalZfsConfig> {
        let mut builder = Self::new();
        builder.config.pools.max_pools = 5;
        builder.config.datasets.max_datasets_per_pool = 100;
        builder.config.performance.optimization_level = "conservative".to_string();
        builder.config.monitoring.enable_metrics_collection = false;
        builder.build()
    }

    /// Create production configuration
    pub fn production() -> Result<CanonicalZfsConfig> {
        let mut builder = Self::new();
        builder.config.pools.max_pools = 50;
        builder.config.datasets.max_datasets_per_pool = 10000;
        builder.config.performance.optimization_level = "balanced".to_string();
        builder.config.monitoring.enable_metrics_collection = true;
        builder.config.snapshots.enable_auto_snapshots = true;
        builder.build()
    }

    /// Create high-performance configuration
    pub fn high_performance() -> Result<CanonicalZfsConfig> {
        let mut config = Self::production()?;
        config.performance.optimization_level = "aggressive".to_string();
        config.performance.arc_cache.max_size_mb = 16384; // 16 GB
        config.performance.zil.enable_separate_zil = true;
        config.performance.prefetch.max_streams = 16;
        config.performance.io_scheduler.max_active_ios = 64;
        config.validate()?;
        Ok(config)
    }
}
