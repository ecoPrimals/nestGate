/// **UNIFIED ZFS CONFIGURATION MODULE**
/// Consolidates all fragmented ZFS configuration structs into a single,
/// comprehensive configuration system using the StandardDomainConfig pattern.
/// 
/// **ELIMINATES**:
/// - DatasetConfig (dataset.rs)
/// - OptimizerConfig (advanced_zfs_optimization.rs)
/// - PerformanceConfig (performance/types.rs)
/// - MigrationConfig (migration/types.rs)
/// - PoolSetupConfig (pool_setup/validation.rs)
/// - AdvancedConfig (types.rs)
/// - Multiple fragmented ZFS configs across modules
/// 
/// **PROVIDES**:
/// - Single source of truth for all ZFS configuration  
/// - Consistent configuration patterns with base unified configs
/// - Extensible architecture for ZFS-specific settings
/// - Performance optimization consolidation

use nestgate_core::unified_config_consolidation::StandardDomainConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use std::path::PathBuf;

/// **UNIFIED ZFS EXTENSIONS**
/// Consolidates all ZFS-specific configuration patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedZfsExtensions {
    /// Pool management settings
    pub pools: ZfsPoolSettings,
    /// Dataset management settings
    pub datasets: ZfsDatasetSettings,
    /// Performance optimization settings
    pub performance: ZfsPerformanceSettings,
    /// Storage migration settings
    pub migration: ZfsMigrationSettings,
    /// Advanced ZFS features
    pub advanced: ZfsAdvancedSettings,
    /// Snapshot and backup settings
    pub snapshots: ZfsSnapshotSettings,
    /// Monitoring and health settings
    pub monitoring: ZfsMonitoringSettings,
    }

/// ZFS pool configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolSettings {
    /// Default pool creation settings
    pub default_pool_name: String,
    /// Default vdev configuration
    pub default_vdev_config: ZfsVdevConfig,
    /// Pool auto-import settings
    pub enable_auto_import: bool,
    /// Pool health check interval
    pub health_check_interval: Duration,
    /// Pool scrub schedule
    pub scrub_schedule: ZfsScrubSchedule,
    /// Pool feature flags
    pub enabled_features: Vec<String>,
    /// Maximum pools per system
    pub max_pools: u32,
    }

/// ZFS dataset configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetSettings {
    /// Default dataset prefix
    pub default_prefix: String,
    /// Default quota (MB, 0 = unlimited)
    pub default_quota_mb: u64,
    /// Default reservation (MB)
    pub default_reservation_mb: u64,
    /// Default compression algorithm
    pub default_compression: String,
    /// Default checksum algorithm
    pub default_checksum: String,
    /// Enable deduplication by default
    pub default_dedup: bool,
    /// Dataset mount options
    pub default_mount_options: Vec<String>,
    /// Enable encryption by default
    pub default_encryption: bool,
    /// Encryption algorithm
    pub encryption_algorithm: String,
    /// Maximum datasets per pool
    pub max_datasets_per_pool: u32,
    }

/// ZFS performance optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceSettings {
    /// ARC cache optimization
    pub arc_cache: ZfsArcCacheSettings,
    /// L2ARC (SSD cache) settings
    pub l2arc: ZfsL2ArcSettings,
    /// ZIL (write cache) settings
    pub zil: ZfsZilSettings,
    /// Prefetch settings
    pub prefetch: ZfsPrefetchSettings,
    /// I/O scheduler settings
    pub io_scheduler: ZfsIoSchedulerSettings,
    /// Performance monitoring
    pub enable_performance_monitoring: bool,
    /// Performance optimization level
    pub optimization_level: String, // "conservative", "balanced", "aggressive"
    }

/// ZFS migration configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMigrationSettings {
    /// Enable online migration
    pub enable_online_migration: bool,
    /// Migration bandwidth limit (MB/s, 0 = unlimited)
    pub bandwidth_limit_mbps: u32,
    /// Migration verification level
    pub verification_level: String, // "checksum", "full", "none"
    /// Resume interrupted migrations
    pub enable_resume: bool,
    /// Migration retry attempts
    pub retry_attempts: u32,
    /// Migration timeout (minutes)
    pub timeout_minutes: u32,
    /// Temporary migration space (MB)
    pub temp_space_mb: u64,
    }

/// ZFS advanced features settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsAdvancedSettings {
    /// Enable LZ4 compression
    pub enable_lz4: bool,
    /// Enable ZSTD compression
    pub enable_zstd: bool,
    /// Enable encryption at rest
    pub enable_encryption: bool,
    /// Enable deduplication
    pub enable_dedup: bool,
    /// Enable async destruction
    pub enable_async_destroy: bool,
    /// Enable spacemap v2
    pub enable_spacemap_v2: bool,
    /// Enable large blocks
    pub enable_large_blocks: bool,
    /// Large block size threshold (KB)
    pub large_block_threshold_kb: u32,
    /// Enable native checksums
    pub enable_native_checksums: bool,
    }

/// ZFS snapshot configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSnapshotSettings {
    /// Enable automatic snapshots
    pub enable_auto_snapshots: bool,
    /// Snapshot frequency
    pub snapshot_frequency: Duration,
    /// Snapshot retention policy
    pub retention: ZfsSnapshotRetention,
    /// Snapshot naming pattern
    pub naming_pattern: String,
    /// Enable snapshot compression
    pub enable_compression: bool,
    /// Enable incremental snapshots
    pub enable_incremental: bool,
    /// Maximum snapshots per dataset
    pub max_snapshots_per_dataset: u32,
    }

/// ZFS monitoring configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsMonitoringSettings {
    /// Enable ZFS event monitoring
    pub enable_event_monitoring: bool,
    /// Event monitoring interval
    pub monitoring_interval: Duration,
    /// Enable SMART monitoring
    pub enable_smart_monitoring: bool,
    /// Health check frequency
    pub health_check_frequency: Duration,
    /// Performance metrics collection
    pub enable_metrics_collection: bool,
    /// Metrics retention period (days)
    pub metrics_retention_days: u32,
    /// Alert thresholds
    pub alert_thresholds: ZfsAlertThresholds,
    }

/// ZFS vdev configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsVdevConfig {
    /// Vdev type (mirror, raidz1, raidz2, raidz3, striped)
    pub vdev_type: String,
    /// Minimum vdev count for type
    pub min_vdev_count: u32,
    /// Enable spare devices
    pub enable_spares: bool,
    /// Number of spare devices
    pub spare_count: u32,
    /// Enable cache devices
    pub enable_cache: bool,
    /// Enable log devices
    pub enable_log: bool,
    }

/// ZFS scrub schedule configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsScrubSchedule {
    /// Enable automatic scrubbing
    pub enable_auto_scrub: bool,
    /// Scrub frequency (days)
    pub frequency_days: u32,
    /// Scrub bandwidth limit (MB/s)
    pub bandwidth_limit_mbps: u32,
    /// Scrub during maintenance windows only
    pub maintenance_window_only: bool,
    /// Maintenance window start hour (24h format)
    pub maintenance_start_hour: u8,
    /// Maintenance window duration (hours)
    pub maintenance_duration_hours: u8,
    }

/// ZFS ARC cache settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsArcCacheSettings {
    /// Minimum ARC size (MB)
    pub min_size_mb: u64,
    /// Maximum ARC size (MB, 0 = auto)
    pub max_size_mb: u64,
    /// ARC meta limit (MB)
    pub meta_limit_mb: u64,
    /// Enable ARC compression
    pub enable_compression: bool,
    /// ARC eviction policy
    pub eviction_policy: String, // "lru", "lfu", "adaptive"
    }

/// ZFS L2ARC (SSD cache) settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsL2ArcSettings {
    /// Enable L2ARC
    pub enable_l2arc: bool,
    /// L2ARC device paths
    pub device_paths: Vec<PathBuf>,
    /// L2ARC write boost
    pub write_boost: u64,
    /// L2ARC noprefetch
    pub noprefetch: bool,
    /// L2ARC rebuild
    pub enable_rebuild: bool,
    }

/// ZFS ZIL (write cache) settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsZilSettings {
    /// Enable separate ZIL device
    pub enable_separate_zil: bool,
    /// ZIL device paths
    pub device_paths: Vec<PathBuf>,
    /// Disable ZIL (for testing only)
    pub disable_zil: bool,
    /// ZIL sync frequency
    pub sync_frequency: Duration,
    }

/// ZFS prefetch settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPrefetchSettings {
    /// Enable prefetch
    pub enable_prefetch: bool,
    /// Prefetch disable for random reads
    pub disable_random_reads: bool,
    /// Prefetch metadata
    pub enable_metadata_prefetch: bool,
    /// Prefetch streams
    pub max_streams: u32,
    }

/// ZFS I/O scheduler settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsIoSchedulerSettings {
    /// I/O scheduler type
    pub scheduler_type: String, // "noop", "deadline", "cfq", "bfq"
    /// I/O delay minimum
    pub delay_min_ms: u32,
    /// I/O delay scale
    pub delay_scale: u32,
    /// Maximum I/O requests
    pub max_active_ios: u32,
    }

/// ZFS snapshot retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSnapshotRetention {
    /// Keep hourly snapshots for X hours
    pub hourly_retention_hours: u32,
    /// Keep daily snapshots for X days
    pub daily_retention_days: u32,
    /// Keep weekly snapshots for X weeks
    pub weekly_retention_weeks: u32,
    /// Keep monthly snapshots for X months
    pub monthly_retention_months: u32,
    /// Keep yearly snapshots for X years
    pub yearly_retention_years: u32,
    }

/// ZFS alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsAlertThresholds {
    /// Pool usage warning threshold (percentage)
    pub pool_usage_warning: f64,
    /// Pool usage critical threshold (percentage)
    pub pool_usage_critical: f64,
    /// Pool fragmentation warning threshold (percentage)
    pub fragmentation_warning: f64,
    /// Pool fragmentation critical threshold (percentage)
    pub fragmentation_critical: f64,
    /// Disk error threshold (count)
    pub disk_error_threshold: u32,
    /// Temperature warning threshold (celsius)
    pub temperature_warning: f64,
    /// Temperature critical threshold (celsius)
    pub temperature_critical: f64,
    }

impl Default for UnifiedZfsExtensions {
    fn default() -> Self {
        Self {
            pools: ZfsPoolSettings {
                default_pool_name: "nestgate-pool".to_string(),
                default_vdev_config: ZfsVdevConfig {
                    vdev_type: "mirror".to_string(),
                    min_vdev_count: 2,
                    enable_spares: true,
                    spare_count: 1,
                    enable_cache: true,
                    enable_log: true,
                },
                enable_auto_import: true,
                health_check_interval: Duration::from_secs(300), // 5 minutes
                scrub_schedule: ZfsScrubSchedule {
                    enable_auto_scrub: true,
                    frequency_days: 7, // Weekly
                    bandwidth_limit_mbps: 100, // 100 MB/s
                    maintenance_window_only: true,
                    maintenance_start_hour: 2, // 2 AM
                    maintenance_duration_hours: 4, // 4 hours
                },
                enabled_features: vec![
                    "async_destroy".to_string(),
                    "empty_bpobj".to_string(),
                    "lz4_compress".to_string(),
                    "spacemap_histogram".to_string(),
                    "extensible_dataset".to_string(),
                    "bookmarks".to_string(),
                    "filesystem_limits".to_string(),
                    "large_blocks".to_string(),
                    "sha512".to_string(),
                    "skein".to_string(),
                    "edonr".to_string(),
                ],
                max_pools: 16,
            },
            datasets: ZfsDatasetSettings {
                default_prefix: "nestgate".to_string(),
                default_quota_mb: 0, // Unlimited
                default_reservation_mb: 0, // No reservation
                default_compression: "lz4".to_string(),
                default_checksum: "sha256".to_string(),
                default_dedup: false, // Disabled by default (performance impact)
                default_mount_options: vec!["noatime".to_string(), "async".to_string()],
                default_encryption: true,
                encryption_algorithm: "aes-256-gcm".to_string(),
                max_datasets_per_pool: 1000,
            },
            performance: ZfsPerformanceSettings {
                arc_cache: ZfsArcCacheSettings {
                    min_size_mb: 1024, // 1 GB minimum
                    max_size_mb: 0, // Auto-size
                    meta_limit_mb: 0, // Auto-size
                    enable_compression: true,
                    eviction_policy: "adaptive".to_string(),
                },
                l2arc: ZfsL2ArcSettings {
                    enable_l2arc: false, // Disabled by default
                    device_paths: Vec::new(),
                    write_boost: 268435456, // 256 MB
                    noprefetch: false,
                    enable_rebuild: true,
                },
                zil: ZfsZilSettings {
                    enable_separate_zil: false, // Use pool devices by default  
                    device_paths: Vec::new(),
                    disable_zil: false,
                    sync_frequency: Duration::from_secs(5),
                },
                prefetch: ZfsPrefetchSettings {
                    enable_prefetch: true,
                    disable_random_reads: false,
                    enable_metadata_prefetch: true,
                    max_streams: 8,
                },
                io_scheduler: ZfsIoSchedulerSettings {
                    scheduler_type: "deadline".to_string(),
                    delay_min_ms: 500,
                    delay_scale: 2,
                    max_active_ios: 32,
                },
                enable_performance_monitoring: true,
                optimization_level: "balanced".to_string(),
            },
            migration: ZfsMigrationSettings {
                enable_online_migration: true,
                bandwidth_limit_mbps: 100, // 100 MB/s limit
                verification_level: "checksum".to_string(),
                enable_resume: true,
                retry_attempts: 3,
                timeout_minutes: 240, // 4 hours
                temp_space_mb: 10240, // 10 GB
            },
            advanced: ZfsAdvancedSettings {
                enable_lz4: true,
                enable_zstd: false, // Feature availability dependent
                enable_encryption: true,
                enable_dedup: false, // Performance impact
                enable_async_destroy: true,
                enable_spacemap_v2: true,
                enable_large_blocks: true,
                large_block_threshold_kb: 128, // 128 KB
                enable_native_checksums: true,
            },
            snapshots: ZfsSnapshotSettings {
                enable_auto_snapshots: true,
                snapshot_frequency: Duration::from_secs(3600), // Hourly
                retention: ZfsSnapshotRetention {
                    hourly_retention_hours: 24,
                    daily_retention_days: 7,
                    weekly_retention_weeks: 4,
                    monthly_retention_months: 12,
                    yearly_retention_years: 5,
                },
                naming_pattern: "auto-%Y%m%d-%H%M%S".to_string(),
                enable_compression: true,
                enable_incremental: true,
                max_snapshots_per_dataset: 100,
            },
            monitoring: ZfsMonitoringSettings {
                enable_event_monitoring: true,
                monitoring_interval: Duration::from_secs(60),
                enable_smart_monitoring: true,
                health_check_frequency: Duration::from_secs(300),
                enable_metrics_collection: true,
                metrics_retention_days: 30,
                alert_thresholds: ZfsAlertThresholds {
                    pool_usage_warning: 80.0,
                    pool_usage_critical: 90.0,
                    fragmentation_warning: 50.0,
                    fragmentation_critical: 75.0,
                    disk_error_threshold: 5,
                    temperature_warning: 55.0,
                    temperature_critical: 65.0,
                },
            },
    }
    }
    }

/// **UNIFIED ZFS CONFIGURATION**
/// Single configuration type that replaces all ZFS config structs
pub type UnifiedZfsConfig = StandardDomainConfig<UnifiedZfsExtensions>;

impl UnifiedZfsConfig {
    /// Create development-focused ZFS configuration
    pub fn development() -> Self {
        let mut config = StandardDomainConfig::new(UnifiedZfsExtensions::default());
        
        // Configure service settings for ZFS development
        config.service.name = "nestgate-zfs".to_string();
        config.service.version = env!("CARGO_PKG_VERSION").to_string();
        config.service.description = "NestGate ZFS Storage Service".to_string();
        config.service.service_type = "storage".to_string();
        config.service.environment = "development".to_string();

        // Development-friendly network settings
        config.network.port = 8086; // ZFS default port
        config.network.bind_address = "127.0.0.1".to_string();
        config.network.enable_tls = false; // Dev uses plain connections
        config.network.max_connections = 50;

        // Development security settings (less strict)
        config.security.require_auth = false; // Dev bypasses auth
        config.security.enable_tls = false;
        config.security.allowed_origins = vec!["*".to_string()];
        config.security.allowed_ip_ranges = vec!["127.0.0.0/8".to_string()];

        // Development ZFS extensions (performance optimized for dev)
        config.extensions.pools.default_pool_name = "nestgate-dev-pool".to_string();
        config.extensions.datasets.default_encryption = false; // Dev performance
        config.extensions.advanced.enable_dedup = false; // Dev performance
        config.extensions.snapshots.enable_auto_snapshots = false; // Dev simplicity
        config.extensions.performance.optimization_level = "conservative".to_string();
        config.extensions.monitoring.enable_metrics_collection = false; // Dev simplicity
        
        config
    }

    /// Create production-ready ZFS configuration
    pub fn production() -> Self {
        let mut config = StandardDomainConfig::new(UnifiedZfsExtensions::default());
        
        // Configure service settings for ZFS production
        config.service.name = "nestgate-zfs".to_string();
        config.service.version = env!("CARGO_PKG_VERSION").to_string();
        config.service.description = "NestGate ZFS Storage Service".to_string();
        config.service.service_type = "storage".to_string();
        config.service.environment = "production".to_string();

        // Production network settings
        config.network.port = 8086;
        config.network.bind_address = "0.0.0.0".to_string();
        config.network.enable_tls = true; // Production requires TLS
        config.network.max_connections = 1000;

        // Production security settings
        config.security.require_auth = true;
        config.security.enable_tls = true;
        config.security.allowed_origins = vec![]; // Restrict origins in production
        config.security.allowed_ip_ranges = vec!["10.0.0.0/8".to_string()]; // Private networks only

        // Production ZFS extensions (security and reliability focused)
        config.extensions.pools.default_pool_name = "nestgate-prod-pool".to_string();
        config.extensions.datasets.default_encryption = true; // Production security
        config.extensions.advanced.enable_encryption = true;
        config.extensions.snapshots.enable_auto_snapshots = true; // Production backups
        config.extensions.performance.optimization_level = "balanced".to_string();
        config.extensions.monitoring.enable_metrics_collection = true; // Production monitoring
        config.extensions.monitoring.enable_event_monitoring = true;
        config.extensions.monitoring.enable_smart_monitoring = true;
        
        config
    }

    /// Create high-performance ZFS configuration
    pub fn high_performance() -> Self {
        let mut config = Self::production();
        
        // High-performance optimizations
        config.extensions.performance.optimization_level = "aggressive".to_string();
        config.extensions.performance.arc_cache.max_size_mb = 16384; // 16 GB ARC
        config.extensions.performance.l2arc.enable_l2arc = true; // Enable SSD cache
        config.extensions.performance.zil.enable_separate_zil = true; // Separate ZIL device
        config.extensions.performance.prefetch.max_streams = 16; // More prefetch streams
        config.extensions.performance.io_scheduler.max_active_ios = 64; // More concurrent I/O
        
        // Advanced features for performance
        config.extensions.advanced.enable_lz4 = true;
        config.extensions.advanced.enable_large_blocks = true;
        config.extensions.advanced.large_block_threshold_kb = 256; // Larger blocks
        
        // Optimize pool settings
        config.extensions.pools.scrub_schedule.bandwidth_limit_mbps = 500; // Higher scrub bandwidth
        
        config
    }

    /// Create backup-focused ZFS configuration
    pub fn backup_optimized() -> Self {
        let mut config = Self::production();
        
        // Backup optimizations
        config.extensions.snapshots.enable_auto_snapshots = true;
        config.extensions.snapshots.snapshot_frequency = Duration::from_secs(1800); // Every 30 minutes
        config.extensions.snapshots.enable_incremental = true;
        config.extensions.snapshots.retention.hourly_retention_hours = 48; // 2 days of hourly
        config.extensions.snapshots.retention.daily_retention_days = 30; // 30 days of daily
        config.extensions.snapshots.retention.weekly_retention_weeks = 12; // 3 months of weekly
        config.extensions.snapshots.retention.monthly_retention_months = 24; // 2 years of monthly
        config.extensions.snapshots.retention.yearly_retention_years = 10; // 10 years of yearly
        
        // Enhanced monitoring for backup integrity
        config.extensions.monitoring.enable_event_monitoring = true;
        config.extensions.monitoring.monitoring_interval = Duration::from_secs(30);
        config.extensions.monitoring.metrics_retention_days = 90; // 3 months of metrics
        
        // More aggressive scrubbing for data integrity
        config.extensions.pools.scrub_schedule.frequency_days = 3; // Every 3 days
        
        config
    }
    }

/// **MIGRATION HELPERS**
/// Functions to migrate from legacy ZFS configs to unified system

/// Create a unified config from basic parameters
/// Replaces legacy migration functions with a simpler approach
pub fn create_unified_config(
    pool_name: String,
    compression_enabled: bool,
    performance_mode: bool,
) -> UnifiedZfsConfig {
    let mut unified = if performance_mode {
        UnifiedZfsConfig::production()
    } else {
        UnifiedZfsConfig::development()
    };
    
    // Set basic configuration
    unified.base.name = format!("zfs-{}", pool_name);
    unified.extensions.datasets.default_compression = if compression_enabled {
        "lz4".to_string()
    } else {
        "off".to_string()
    };
    
    unified
}

// Legacy migration functions removed - use create_unified_config() instead
// The unified configuration system replaces all legacy config types

/// **VALIDATION HELPERS**
/// Validate ZFS-specific configuration settings

impl UnifiedZfsConfig {
    /// Validate ZFS configuration
    pub fn validate_zfs_config(&self) -> Result<(), String> {
        // Validate pool settings
        if self.extensions.pools.default_pool_name.is_empty() {
            return Err("Pool name cannot be empty".to_string());
    }

        if self.extensions.pools.max_pools == 0 {
            return Err("Maximum pools must be greater than 0".to_string());
    }

        // Validate dataset settings
        if self.extensions.datasets.max_datasets_per_pool == 0 {
            return Err("Maximum datasets per pool must be greater than 0".to_string());
    }

        // Validate performance settings
        if self.extensions.performance.arc_cache.min_size_mb > self.extensions.performance.arc_cache.max_size_mb && 
           self.extensions.performance.arc_cache.max_size_mb > 0 {
            return Err("ARC minimum size cannot be greater than maximum size".to_string());
    }

        // Validate migration settings
        if self.extensions.migration.retry_attempts == 0 {
            return Err("Migration retry attempts must be greater than 0".to_string());
    }

        // Validate snapshot settings
        if self.extensions.snapshots.enable_auto_snapshots && 
           self.extensions.snapshots.max_snapshots_per_dataset == 0 {
            return Err("Maximum snapshots per dataset must be greater than 0 when auto-snapshots are enabled".to_string());
    }

        // Validate alert thresholds
        let thresholds = &self.extensions.monitoring.alert_thresholds;
        if thresholds.pool_usage_warning >= thresholds.pool_usage_critical {
            return Err("Pool usage warning threshold must be less than critical threshold".to_string());
    }

        if thresholds.fragmentation_warning >= thresholds.fragmentation_critical {
            return Err("Fragmentation warning threshold must be less than critical threshold".to_string());
    }

        if thresholds.temperature_warning >= thresholds.temperature_critical {
            return Err("Temperature warning threshold must be less than critical threshold".to_string());
    }

    }

    /// Get optimal ZFS configuration for given workload type
    pub fn for_workload(workload_type: &str) -> Self {
        match workload_type {
            "database" => Self::database_optimized(),
            "backup" => Self::backup_optimized(),
            "high_performance" => Self::high_performance(),
            "development" => Self::development(),
            "production" | _ => Self::production(),
    }
    }

    /// Create database-optimized ZFS configuration
    pub fn database_optimized() -> Self {
        let mut config = Self::production();
        
        // Database optimizations
        config.extensions.performance.optimization_level = "aggressive".to_string();
        config.extensions.performance.zil.enable_separate_zil = true; // Critical for DB performance
        config.extensions.performance.arc_cache.max_size_mb = 8192; // 8 GB ARC for DB caching
        config.extensions.datasets.default_compression = "lz4".to_string(); // Fast compression
        config.extensions.advanced.enable_large_blocks = false; // DB prefers smaller blocks
        config.extensions.snapshots.snapshot_frequency = Duration::from_secs(300); // 5-minute snapshots
        
        // Database-specific dataset settings
        config.extensions.datasets.default_mount_options = vec![
            "noatime".to_string(), 
            "sync=disabled".to_string(), // WARNING: Only for non-critical dev DBs
        ];
        
        config
    }
} 