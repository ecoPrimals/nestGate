//! Pool Setup Configuration
//!
//! Configuration structures and defaults for ZFS pool setup operations

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use nestgate_core::StorageTier;

/// Configuration for pool setup operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolSetupConfiguration {
    /// ZFS pool properties
    pub pool_properties: PoolPropertyConfig,
    /// Device detection settings
    pub device_detection: DeviceDetectionConfig,
    /// Safety and validation settings
    pub safety: SafetyConfig,
    /// Performance optimization settings
    pub performance: PerformanceConfig,
    /// Tier configuration
    pub tier_config: TierSetupConfig,
}

/// ZFS pool property configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolPropertyConfig {
    /// Default ashift value (sector size power of 2)
    pub default_ashift: u8,
    /// Enable autoexpand by default
    pub autoexpand: bool,
    /// Enable autotrim by default
    pub autotrim: bool,
    /// Default compression algorithm
    pub default_compression: String,
    /// Default recordsize in KB
    pub default_recordsize_kb: u32,
    /// Enable deduplication by default
    pub enable_dedup: bool,
}

/// Device detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDetectionConfig {
    /// Minimum device size in bytes
    pub min_device_size: u64,
    /// Maximum device size in bytes (0 = no limit)
    pub max_device_size: u64,
    /// Skip devices with these mount points
    pub skip_mountpoints: Vec<String>,
    /// Skip devices with these filesystem types
    pub skip_fstypes: Vec<String>,
    /// Device scanning timeout in seconds
    pub scan_timeout_seconds: u64,
    /// Include loop devices in scanning
    pub include_loop_devices: bool,
}

/// Safety configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    /// Require confirmation for destructive operations
    pub require_confirmation: bool,
    /// Allow overwriting existing pools
    pub allow_pool_overwrite: bool,
    /// Maximum number of devices per pool
    pub max_devices_per_pool: usize,
    /// Minimum free space required (percentage)
    pub min_free_space_percent: f64,
    /// Enable dry-run mode by default
    pub default_dry_run: bool,
    /// Backup existing data before operations
    pub auto_backup: bool,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Cache hit ratio thresholds
    pub cache_hit_thresholds: CacheThresholds,
    /// I/O performance thresholds
    pub io_thresholds: IoThresholds,
    /// Memory usage limits
    pub memory_limits: MemoryLimits,
    /// Optimization intervals
    pub optimization_intervals: OptimizationIntervals,
}

/// Cache hit ratio thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheThresholds {
    pub excellent: f64,
    pub good: f64,
    pub warning: f64,
    pub critical: f64,
}

/// I/O performance thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoThresholds {
    /// IOPS thresholds by tier
    pub iops_thresholds: HashMap<String, u64>,
    /// Latency thresholds in milliseconds
    pub latency_thresholds: HashMap<String, f64>,
    /// Throughput thresholds in MB/s
    pub throughput_thresholds: HashMap<String, f64>,
}

/// Memory usage limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLimits {
    /// Maximum ARC size as percentage of system RAM
    pub max_arc_percent: f64,
    /// Minimum ARC size in bytes
    pub min_arc_bytes: u64,
    /// L2ARC size limits
    pub l2arc_limits: L2ArcLimits,
}

/// L2ARC configuration limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2ArcLimits {
    pub max_size_bytes: u64,
    pub feed_secs: u32,
    pub headroom: u32,
}

/// Optimization intervals configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationIntervals {
    pub tier_optimization: u64,
    pub performance_analysis: u64,
    pub health_check: u64,
    pub metrics_collection: u64,
}

/// Tier setup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierSetupConfig {
    /// Tier-specific properties
    pub tier_properties: HashMap<String, TierProperties>,
    /// Migration thresholds
    pub migration_thresholds: MigrationThresholds,
    /// Tier size limits
    pub tier_limits: TierLimits,
}

/// Properties for a specific tier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierProperties {
    pub compression: String,
    pub recordsize: String,
    pub primarycache: String,
    pub secondarycache: String,
    pub logbias: String,
    pub sync: String,
    pub atime: String,
}

/// Migration thresholds configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationThresholds {
    /// Access frequency thresholds (accesses per day)
    pub access_frequency: HashMap<String, f64>,
    /// File age thresholds (days)
    pub file_age: HashMap<String, u32>,
    /// File size thresholds (bytes)
    pub file_size: HashMap<String, u64>,
}

/// Tier size limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierLimits {
    /// Maximum utilization percentage per tier
    pub max_utilization: HashMap<String, f64>,
    /// Warning thresholds per tier
    pub warning_thresholds: HashMap<String, f64>,
    /// Reserved space per tier (bytes)
    pub reserved_space: HashMap<String, u64>,
}

/// Pool setup configuration for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolSetupConfig {
    /// Pool name
    pub pool_name: String,
    /// Devices to use for the pool
    pub devices: Vec<String>,
    /// Pool topology (mirror, raidz, etc.)
    pub topology: PoolTopology,
    /// ZFS properties to set
    pub properties: HashMap<String, String>,
    /// Whether to create tier structure
    pub create_tiers: bool,
    /// Tier mappings to device types
    pub tier_mappings: HashMap<StorageTier, Vec<super::DeviceType>>,
}

/// Pool topology options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolTopology {
    Single,
    Mirror,
    RaidZ1,
    RaidZ2,
    RaidZ3,
}

// Default implementations
impl Default for PoolSetupConfiguration {
    fn default() -> Self {
        Self {
            pool_properties: PoolPropertyConfig::default(),
            device_detection: DeviceDetectionConfig::default(),
            safety: SafetyConfig::default(),
            performance: PerformanceConfig::default(),
            tier_config: TierSetupConfig::default(),
        }
    }
}

impl Default for PoolPropertyConfig {
    fn default() -> Self {
        Self {
            default_ashift: 12, // 4K sectors
            autoexpand: true,
            autotrim: true,
            default_compression: "lz4".to_string(),
            default_recordsize_kb: 128,
            enable_dedup: false,
        }
    }
}

impl Default for DeviceDetectionConfig {
    fn default() -> Self {
        Self {
            min_device_size: 1024 * 1024 * 1024, // 1GB
            max_device_size: 0, // No limit
            skip_mountpoints: vec!["/".to_string(), "/boot".to_string()],
            skip_fstypes: vec!["tmpfs".to_string(), "devtmpfs".to_string()],
            scan_timeout_seconds: 30,
            include_loop_devices: false,
        }
    }
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            require_confirmation: true,
            allow_pool_overwrite: false,
            max_devices_per_pool: 24,
            min_free_space_percent: 10.0,
            default_dry_run: true,
            auto_backup: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            cache_hit_thresholds: CacheThresholds::default(),
            io_thresholds: IoThresholds::default(),
            memory_limits: MemoryLimits::default(),
            optimization_intervals: OptimizationIntervals::default(),
        }
    }
}

impl Default for CacheThresholds {
    fn default() -> Self {
        Self {
            excellent: 0.95,
            good: 0.85,
            warning: 0.70,
            critical: 0.50,
        }
    }
}

impl Default for IoThresholds {
    fn default() -> Self {
        let mut iops_thresholds = HashMap::new();
        iops_thresholds.insert("hot".to_string(), 10000);
        iops_thresholds.insert("warm".to_string(), 5000);
        iops_thresholds.insert("cold".to_string(), 1000);

        let mut latency_thresholds = HashMap::new();
        latency_thresholds.insert("hot".to_string(), 1.0);
        latency_thresholds.insert("warm".to_string(), 5.0);
        latency_thresholds.insert("cold".to_string(), 20.0);

        let mut throughput_thresholds = HashMap::new();
        throughput_thresholds.insert("hot".to_string(), 1000.0);
        throughput_thresholds.insert("warm".to_string(), 500.0);
        throughput_thresholds.insert("cold".to_string(), 100.0);

        Self {
            iops_thresholds,
            latency_thresholds,
            throughput_thresholds,
        }
    }
}

impl Default for MemoryLimits {
    fn default() -> Self {
        Self {
            max_arc_percent: 50.0,
            min_arc_bytes: 64 * 1024 * 1024, // 64MB
            l2arc_limits: L2ArcLimits::default(),
        }
    }
}

impl Default for L2ArcLimits {
    fn default() -> Self {
        Self {
            max_size_bytes: 10 * 1024 * 1024 * 1024, // 10GB
            feed_secs: 1,
            headroom: 2,
        }
    }
}

impl Default for OptimizationIntervals {
    fn default() -> Self {
        Self {
            tier_optimization: 3600, // 1 hour
            performance_analysis: 300, // 5 minutes
            health_check: 60, // 1 minute
            metrics_collection: 30, // 30 seconds
        }
    }
}

impl Default for TierSetupConfig {
    fn default() -> Self {
        let mut tier_properties = HashMap::new();
        
        // Hot tier properties
        tier_properties.insert("hot".to_string(), TierProperties {
            compression: "lz4".to_string(),
            recordsize: "128K".to_string(),
            primarycache: "all".to_string(),
            secondarycache: "all".to_string(),
            logbias: "latency".to_string(),
            sync: "standard".to_string(),
            atime: "on".to_string(),
        });
        
        // Warm tier properties
        tier_properties.insert("warm".to_string(), TierProperties {
            compression: "gzip-6".to_string(),
            recordsize: "1M".to_string(),
            primarycache: "metadata".to_string(),
            secondarycache: "all".to_string(),
            logbias: "throughput".to_string(),
            sync: "standard".to_string(),
            atime: "off".to_string(),
        });
        
        // Cold tier properties
        tier_properties.insert("cold".to_string(), TierProperties {
            compression: "gzip-9".to_string(),
            recordsize: "1M".to_string(),
            primarycache: "metadata".to_string(),
            secondarycache: "metadata".to_string(),
            logbias: "throughput".to_string(),
            sync: "disabled".to_string(),
            atime: "off".to_string(),
        });

        Self {
            tier_properties,
            migration_thresholds: MigrationThresholds::default(),
            tier_limits: TierLimits::default(),
        }
    }
}

impl Default for MigrationThresholds {
    fn default() -> Self {
        let mut access_frequency = HashMap::new();
        access_frequency.insert("hot_to_warm".to_string(), 10.0);
        access_frequency.insert("warm_to_cold".to_string(), 1.0);
        access_frequency.insert("cold_to_warm".to_string(), 5.0);
        access_frequency.insert("warm_to_hot".to_string(), 50.0);

        let mut file_age = HashMap::new();
        file_age.insert("hot_to_warm".to_string(), 30);
        file_age.insert("warm_to_cold".to_string(), 90);

        let mut file_size = HashMap::new();
        file_size.insert("hot_max".to_string(), 100 * 1024 * 1024); // 100MB
        file_size.insert("warm_max".to_string(), 10 * 1024 * 1024 * 1024); // 10GB

        Self {
            access_frequency,
            file_age,
            file_size,
        }
    }
}

impl Default for TierLimits {
    fn default() -> Self {
        let mut max_utilization = HashMap::new();
        max_utilization.insert("hot".to_string(), 80.0);
        max_utilization.insert("warm".to_string(), 85.0);
        max_utilization.insert("cold".to_string(), 90.0);

        let mut warning_thresholds = HashMap::new();
        warning_thresholds.insert("hot".to_string(), 70.0);
        warning_thresholds.insert("warm".to_string(), 75.0);
        warning_thresholds.insert("cold".to_string(), 80.0);

        let mut reserved_space = HashMap::new();
        reserved_space.insert("hot".to_string(), 10 * 1024 * 1024 * 1024); // 10GB
        reserved_space.insert("warm".to_string(), 50 * 1024 * 1024 * 1024); // 50GB
        reserved_space.insert("cold".to_string(), 100 * 1024 * 1024 * 1024); // 100GB

        Self {
            max_utilization,
            warning_thresholds,
            reserved_space,
        }
    }
} 