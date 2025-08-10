/// **CANONICAL ZFS UNIFIED CONFIGURATION**
/// The single source of truth for all ZFS configuration using StandardDomainConfig pattern
/// **CONSOLIDATES**:
/// - DatasetConfig (dataset.rs) - ELIMINATED
/// - PerformanceConfig (performance/types.rs) - ELIMINATED  
/// - CacheConfiguration (optimizer.rs) - ELIMINATED
/// - MigrationConfig (migration/types.rs) - INTEGRATED
/// - PoolSetupConfig (pool_setup/validation.rs) - ELIMINATED
/// - AdvancedConfig (types.rs) - INTEGRATED
/// - UnifiedZfsExtensions (unified_zfs_config.rs) - CONSOLIDATED
///
/// **PROVIDES**:
/// - Single canonical ZFS configuration using StandardDomainConfig<ZfsExtensions>
/// - Consistent patterns with all other NestGate services
/// - Full compatibility with unified configuration system
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// Import our smart abstractions
use nestgate_core::smart_abstractions::prelude::*;

// Import the standardized config pattern
use nestgate_core::unified_config_consolidation::StandardDomainConfig;

// Import supporting configuration modules
use super::{
    automation::AiAutomationSettings,
    health::HealthMonitoringConfig,
    metrics::{MetricsConfig, MetricsFormat},
    migration::{CapacityLimits, MigrationConfig, MigrationRules},
    pool::PoolDiscoveryConfig,
    security::{AccessControlConfig, KeyManagementConfig, SecurityConfig},
    tiers::{PerformanceProfile, TierConfig, TierConfigurations},
    CompressionType, PoolType,
};

// ==================== CANONICAL ZFS CONFIGURATION ====================

/// **THE** canonical ZFS configuration type
/// This is the single, authoritative configuration type for all ZFS operations
pub type ZfsConfig = StandardDomainConfig<ZfsExtensions>;

/// ZFS-specific configuration extensions
/// Domain-specific fields that don't belong in unified base configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsExtensions {
    /// Storage tier configurations
    pub storage_tiers: ZfsStorageTierSettings,
    /// Pool management settings
    pub pool_management: ZfsPoolManagementSettings,
    /// Dataset management settings  
    pub datasets: ZfsDatasetManagementSettings,
    /// Performance optimization
    pub performance: ZfsPerformanceSettings,
    /// Health and monitoring
    pub health_monitoring: ZfsHealthMonitoringSettings,
    /// Security and encryption
    pub security: ZfsSecuritySettings,
    /// AI automation features
    pub ai_automation: ZfsAiAutomationSettings,
}

impl SmartDefault for ZfsExtensions {
    fn smart_default() -> Self {
        Self {
            storage_tiers: ZfsStorageTierSettings::smart_default(),
            pool_management: ZfsPoolManagementSettings::default(),
            datasets: ZfsDatasetManagementSettings::default(),
            performance: ZfsPerformanceSettings::default(),
            health_monitoring: ZfsHealthMonitoringSettings::default(),
            security: ZfsSecuritySettings::default(),
            ai_automation: ZfsAiAutomationSettings::default(),
        }
    }
}

impl Default for ZfsExtensions {
    fn default() -> Self {
        Self::smart_default()
    }
}

// ==================== ZFS-SPECIFIC CONFIGURATION SECTIONS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsStorageTierSettings {
    /// Tier configurations for hot/warm/cold storage
    pub tier_configurations: TierConfigurations,
    /// Automatic tiering rules
    pub auto_tiering_enabled: bool,
    /// Tiering decision interval
    pub tiering_interval: Duration,
    /// Data age thresholds for tier migration
    pub age_thresholds: HashMap<String, Duration>,
    /// Tier performance profiles
    pub tier_profiles: HashMap<String, PerformanceProfile>,
}

impl SmartDefault for ZfsStorageTierSettings {
    fn smart_default() -> Self {
        let mut age_thresholds = HashMap::new();
        age_thresholds.insert(
            "hot_to_warm".to_string(),
            Duration::from_secs(30 * 24 * 3600),
        ); // 30 days
        age_thresholds.insert(
            "warm_to_cold".to_string(),
            Duration::from_secs(90 * 24 * 3600),
        ); // 90 days

        let mut tier_profiles = HashMap::new();
        tier_profiles.insert("hot".to_string(), PerformanceProfile::HighPerformance);
        tier_profiles.insert("warm".to_string(), PerformanceProfile::Balanced);
        tier_profiles.insert("cold".to_string(), PerformanceProfile::HighCompression);

        Self {
            tier_configurations: TierConfigurations::default(),
            auto_tiering_enabled: true,
            tiering_interval: Duration::from_secs(3600), // 1 hour
            age_thresholds,
            tier_profiles,
        }
    }
}

impl Default for ZfsStorageTierSettings {
    fn default() -> Self {
        Self::smart_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolManagementSettings {
    /// Pool discovery configuration
    pub pool_discovery: PoolDiscoveryConfig,
    /// Auto-create pools
    pub auto_create_pools: bool,
    /// Default pool type
    pub default_pool_type: PoolType,
    /// Pool optimization settings
    pub enable_pool_optimization: bool,
    /// Pool health check interval
    pub health_check_interval: Duration,
    /// Maximum pools per system
    pub max_pools: u32,
    /// Pool feature flags
    pub enabled_features: Vec<String>,
    /// Auto-import on startup
    pub enable_auto_import: bool,
}

impl Default for ZfsPoolManagementSettings {
    fn default() -> Self {
        Self {
            pool_discovery: PoolDiscoveryConfig::default(),
            auto_create_pools: false,
            default_pool_type: PoolType::default(),
            enable_pool_optimization: true,
            health_check_interval: Duration::from_secs(300), // 5 minutes
            max_pools: 10,
            enabled_features: vec![
                "async_destroy".to_string(),
                "empty_bpobj".to_string(),
                "lz4_compress".to_string(),
            ],
            enable_auto_import: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetManagementSettings {
    /// Dataset automation settings
    pub automation: AiAutomationSettings,
    /// Default dataset properties
    pub default_properties: HashMap<String, String>,
    /// Snapshot settings
    pub snapshot_settings: ZfsSnapshotSettings,
    /// Clone settings
    pub clone_settings: ZfsCloneSettings,
    /// Default dataset prefix
    pub default_prefix: String,
    /// Default quota (bytes, 0 = unlimited)
    pub default_quota_bytes: u64,
    /// Default reservation (bytes)
    pub default_reservation_bytes: u64,
    /// Default compression
    pub default_compression: CompressionType,
    /// Maximum datasets per pool
    pub max_datasets_per_pool: u32,
}

impl Default for ZfsDatasetManagementSettings {
    fn default() -> Self {
        let mut default_properties = HashMap::new();
        default_properties.insert("atime".to_string(), "off".to_string());
        default_properties.insert("relatime".to_string(), "on".to_string());
        default_properties.insert("xattr".to_string(), "sa".to_string());

        Self {
            automation: AiAutomationSettings::default(),
            default_properties,
            snapshot_settings: ZfsSnapshotSettings::default(),
            clone_settings: ZfsCloneSettings::default(),
            default_prefix: "nestgate".to_string(),
            default_quota_bytes: 0, // unlimited
            default_reservation_bytes: 0,
            default_compression: CompressionType::Lz4,
            max_datasets_per_pool: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSnapshotSettings {
    /// Enable automatic snapshots
    pub auto_snapshot: bool,
    /// Snapshot frequency
    pub snapshot_frequency: Duration,
    /// Snapshot retention period
    pub retention_period: Duration,
    /// Maximum snapshots per dataset
    pub max_snapshots: u32,
    /// Snapshot naming pattern
    pub naming_pattern: String,
}

impl Default for ZfsSnapshotSettings {
    fn default() -> Self {
        Self {
            auto_snapshot: true,
            snapshot_frequency: Duration::from_secs(24 * 3600), // daily
            retention_period: Duration::from_secs(30 * 24 * 3600), // 30 days
            max_snapshots: 100,
            naming_pattern: "auto-%Y%m%d-%H%M%S".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsCloneSettings {
    /// Enable dataset cloning
    pub enable_cloning: bool,
    /// Clone naming pattern
    pub naming_pattern: String,
    /// Clone properties inheritance
    pub inherit_properties: bool,
    /// Auto-promote clones
    pub auto_promote: bool,
}

impl Default for ZfsCloneSettings {
    fn default() -> Self {
        Self {
            enable_cloning: true,
            naming_pattern: "clone-%Y%m%d-%H%M%S".to_string(),
            inherit_properties: true,
            auto_promote: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceSettings {
    /// ARC cache settings
    pub arc_cache: ZfsArcCacheSettings,
    /// L2ARC (SSD cache) settings
    pub l2arc: ZfsL2ArcSettings,
    /// ZIL (write cache) settings
    pub zil: ZfsZilSettings,
    /// I/O optimization settings
    pub io_optimization: ZfsIOSettings,
    /// Compression settings
    pub compression: ZfsCompressionSettings,
    /// Deduplication settings
    pub deduplication: ZfsDeduplicationSettings,
}

impl Default for ZfsPerformanceSettings {
    fn default() -> Self {
        Self {
            arc_cache: ZfsArcCacheSettings::default(),
            l2arc: ZfsL2ArcSettings::default(),
            zil: ZfsZilSettings::default(),
            io_optimization: ZfsIOSettings::default(),
            compression: ZfsCompressionSettings::default(),
            deduplication: ZfsDeduplicationSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsArcCacheSettings {
    /// Minimum ARC size (bytes)
    pub min_size_bytes: u64,
    /// Maximum ARC size (bytes, 0 = auto)
    pub max_size_bytes: u64,
    /// ARC meta limit (bytes)
    pub meta_limit_bytes: u64,
    /// Enable ARC compression
    pub enable_compression: bool,
    /// ARC eviction policy
    pub eviction_policy: String,
}

impl Default for ZfsArcCacheSettings {
    fn default() -> Self {
        Self {
            min_size_bytes: 64 * 1024 * 1024, // 64MB
            max_size_bytes: 0,                // auto
            meta_limit_bytes: 0,              // auto
            enable_compression: true,
            eviction_policy: "lru".to_string(),
        }
    }
}

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

impl Default for ZfsL2ArcSettings {
    fn default() -> Self {
        Self {
            enable_l2arc: false,
            device_paths: Vec::new(),
            write_boost: 16 * 1024 * 1024, // 16MB
            noprefetch: false,
            enable_rebuild: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsZilSettings {
    /// Enable separate ZIL devices
    pub enable_separate_zil: bool,
    /// ZIL device paths
    pub device_paths: Vec<PathBuf>,
    /// Disable ZIL (dangerous!)
    pub disable_zil: bool,
}

impl Default for ZfsZilSettings {
    fn default() -> Self {
        Self {
            enable_separate_zil: false,
            device_paths: Vec::new(),
            disable_zil: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsIOSettings {
    /// Record size optimization
    pub enable_recordsize_optimization: bool,
    /// Prefetch settings
    pub prefetch_disable: bool,
    /// Sync settings
    pub sync_mode: String, // "standard", "always", "disabled"
    /// IOPS limits
    pub max_iops_per_dataset: Option<u64>,
}

impl Default for ZfsIOSettings {
    fn default() -> Self {
        Self {
            enable_recordsize_optimization: true,
            prefetch_disable: false,
            sync_mode: "standard".to_string(),
            max_iops_per_dataset: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsCompressionSettings {
    /// Default compression algorithm
    pub default_algorithm: CompressionType,
    /// Enable compression analysis
    pub enable_analysis: bool,
    /// Compression level (where applicable)
    pub compression_level: Option<u8>,
}

impl Default for ZfsCompressionSettings {
    fn default() -> Self {
        Self {
            default_algorithm: CompressionType::Lz4,
            enable_analysis: true,
            compression_level: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDeduplicationSettings {
    /// Enable deduplication
    pub enable_dedup: bool,
    /// Deduplication algorithm
    pub dedup_algorithm: String, // "sha256", "sha512", "skein", "edonr"
    /// Dedup table threshold
    pub dedup_threshold: f64,
}

impl Default for ZfsDeduplicationSettings {
    fn default() -> Self {
        Self {
            enable_dedup: false, // Disabled by default due to memory requirements
            dedup_algorithm: "sha256".to_string(),
            dedup_threshold: 1.5, // Minimum 1.5x savings to enable
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsHealthMonitoringSettings {
    /// Health monitoring configuration
    pub monitoring: HealthMonitoringConfig,
    /// Enable proactive monitoring
    pub enable_proactive_monitoring: bool,
    /// Health check frequency
    pub health_check_frequency: Duration,
    /// Enable performance metrics collection
    pub enable_metrics_collection: bool,
    /// Metrics retention period (days)
    pub metrics_retention_days: u32,
    /// Alert thresholds
    pub alert_thresholds: ZfsAlertThresholds,
}

impl Default for ZfsHealthMonitoringSettings {
    fn default() -> Self {
        Self {
            monitoring: HealthMonitoringConfig::default(),
            enable_proactive_monitoring: true,
            health_check_frequency: Duration::from_secs(300), // 5 minutes
            enable_metrics_collection: true,
            metrics_retention_days: 30,
            alert_thresholds: ZfsAlertThresholds::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsAlertThresholds {
    /// Pool space usage threshold (percentage)
    pub pool_space_warning: f64,
    /// Pool space critical threshold (percentage)
    pub pool_space_critical: f64,
    /// Error count threshold
    pub error_count_warning: u64,
    /// Scrub age warning (days)
    pub scrub_age_warning_days: u64,
}

impl Default for ZfsAlertThresholds {
    fn default() -> Self {
        Self {
            pool_space_warning: 80.0,
            pool_space_critical: 95.0,
            error_count_warning: 10,
            scrub_age_warning_days: 35,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsSecuritySettings {
    /// Security configuration
    pub security: SecurityConfig,
    /// Enable encryption by default
    pub enable_encryption_by_default: bool,
    /// Encryption algorithm
    pub default_encryption_algorithm: String,
    /// Key management settings
    pub key_management: KeyManagementConfig,
    /// Access control settings
    pub access_control: AccessControlConfig,
}

impl Default for ZfsSecuritySettings {
    fn default() -> Self {
        Self {
            security: SecurityConfig::default(),
            enable_encryption_by_default: false,
            default_encryption_algorithm: "aes-256-gcm".to_string(),
            key_management: KeyManagementConfig::default(),
            access_control: AccessControlConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsAiAutomationSettings {
    /// AI automation settings
    pub automation: AiAutomationSettings,
    /// Enable AI-driven optimization
    pub enable_ai_optimization: bool,
    /// AI optimization interval
    pub optimization_interval: Duration,
    /// Enable predictive analytics
    pub enable_predictive_analytics: bool,
}

impl Default for ZfsAiAutomationSettings {
    fn default() -> Self {
        Self {
            automation: AiAutomationSettings::default(),
            enable_ai_optimization: true,
            optimization_interval: Duration::from_secs(24 * 3600), // daily
            enable_predictive_analytics: true,
        }
    }
}

// ==================== CONFIGURATION FACTORY METHODS ====================

/// Helper functions for creating ZFS configurations
/// Cannot use impl blocks due to orphan rules (StandardDomainConfig is external)
pub mod zfs_config_factory {
    use super::*;

    /// Create a minimal base configuration
    pub fn base_config() -> ZfsConfig {
        StandardDomainConfig {
            service: Default::default(),
            network: Default::default(),
            security: Default::default(),
            monitoring: Default::default(),
            storage: Default::default(),
            memory: Default::default(),
            extensions: ZfsExtensions::default(),
            service_endpoints: HashMap::new(),
            feature_flags: HashMap::new(),
        }
    }

    pub fn development() -> ZfsConfig {
        let mut config = base_config();

        // Development-specific overrides
        config.extensions.pool_management.auto_create_pools = true;
        config.extensions.pool_management.max_pools = 5;
        config.extensions.datasets.max_datasets_per_pool = 100;
        config.extensions.health_monitoring.health_check_frequency = Duration::from_secs(60); // 1 minute
        config.extensions.performance.arc_cache.max_size_bytes = 512 * 1024 * 1024; // 512MB

        config
    }

    /// Create a production configuration
    pub fn production() -> ZfsConfig {
        let mut config = base_config();

        // Production-specific overrides
        config.extensions.pool_management.auto_create_pools = false;
        config.extensions.pool_management.max_pools = 50;
        config.extensions.datasets.max_datasets_per_pool = 10000;
        config.extensions.health_monitoring.health_check_frequency = Duration::from_secs(300); // 5 minutes
        config.extensions.security.enable_encryption_by_default = true;

        config
    }

    /// Create a high-performance configuration
    pub fn high_performance() -> ZfsConfig {
        let mut config = production();

        // High-performance overrides
        config.extensions.performance.arc_cache.max_size_bytes = 8 * 1024 * 1024 * 1024; // 8GB
        config.extensions.performance.l2arc.enable_l2arc = true;
        config.extensions.performance.zil.enable_separate_zil = true;
        config
            .extensions
            .performance
            .io_optimization
            .enable_recordsize_optimization = true;
        config.extensions.storage_tiers.auto_tiering_enabled = true;

        config
    }

    /// Create a backup-optimized configuration
    pub fn backup_optimized() -> ZfsConfig {
        let mut config = production();

        // Backup-optimized overrides
        config.extensions.performance.compression.default_algorithm = CompressionType::Gzip;
        config.extensions.datasets.snapshot_settings.auto_snapshot = true;
        config
            .extensions
            .datasets
            .snapshot_settings
            .retention_period = Duration::from_secs(365 * 24 * 3600); // 1 year
        config.extensions.storage_tiers.auto_tiering_enabled = false; // Keep everything accessible

        config
    }

    /// Create a database-optimized configuration  
    pub fn database_optimized() -> ZfsConfig {
        let mut config = production();

        // Database-optimized overrides
        config.extensions.performance.zil.enable_separate_zil = true;
        config.extensions.performance.io_optimization.sync_mode = "always".to_string();
        config
            .extensions
            .datasets
            .snapshot_settings
            .snapshot_frequency = Duration::from_secs(15 * 60); // 15 minutes
        config.extensions.performance.deduplication.enable_dedup = false; // Better for databases

        config
    }
}

// Note: Cannot implement Default for ZfsConfig due to orphan rules
// Use zfs_config_factory::development() or other factory methods instead
