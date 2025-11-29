//! Unified Types module

use std::collections::HashMap;
//
// **CANONICAL MODERNIZATION**: Essential types that were previously in the
// deprecated unified_types module, now consolidated here.
//
// **CONSOLIDATES**:
// - Storage types from unified_types/consolidated_storage_types.rs
// - Replication types from universal_storage/enterprise/replication.rs
// - Analytics types from universal_storage/enterprise/analytics.rs
// - Access patterns from unified_types/access_patterns.rs

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

// ==================== SECTION ====================

/// Canonical storage change record - consolidates all storage change tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalstoragechange
pub struct CanonicalStorageChange {
    /// Unique identifier
    pub id: String,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Data
    pub data: serde_json::Value,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Source
    pub source: String,
    /// Destination
    pub destination: Option<String>,
}
/// Canonical storage directory entry - unified file system representation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalstoragedirectoryentry
pub struct CanonicalStorageDirectoryEntry {
    /// Name
    pub name: String,
    /// Whether directory
    pub is_directory: bool,
    /// Size
    pub size: u64,
    /// Modified
    pub modified: SystemTime,
    /// Permissions
    pub permissions: Option<String>,
    /// Owner
    pub owner: Option<String>,
    /// Group
    pub group: Option<String>,
    /// Checksum
    pub checksum: Option<String>,
}
/// Canonical storage range specification - unified range operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalstoragerange
pub struct CanonicalStorageRange {
    /// Start
    pub start: u64,
    /// End
    pub end: u64,
    /// Inclusive
    pub inclusive: bool,
    /// Size of chunk
    pub chunk_size: Option<u64>,
}
/// Canonical replication status - unified across all replication systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for CanonicalReplication
pub enum CanonicalReplicationStatus {
    /// Active
    Active,
    /// Paused
    Paused,
    /// Failed
    Failed,
    /// Disabled
    Disabled,
    /// Initializing
    Initializing,
    /// Syncing
    Syncing,
    /// Completed
    Completed,
    /// Queued
    Queued,
    /// Inprogress
    InProgress,
    /// Cancelled
    Cancelled,
}
impl Default for CanonicalReplicationStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Disabled
    }
}

impl std::fmt::Display for CanonicalReplicationStatus {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Paused => write!(f, "paused"),
            Self::Failed => write!(f, "failed"),
            Self::Disabled => write!(f, "disabled"),
            Self::Initializing => write!(f, "initializing"),
            Self::Syncing => write!(f, "syncing"),
            Self::Completed => write!(f, "completed"),
            Self::Queued => write!(f, "queued"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Canonical storage replication result - unified replication tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalstoragereplicationresult
pub struct CanonicalStorageReplicationResult {
    /// Success
    pub success: bool,
    /// Replicated Bytes
    pub replicated_bytes: u64,
    /// Duration Ms
    pub duration_ms: u64,
    /// Error Message
    pub error_message: Option<String>,
    /// Source
    pub source: String,
    /// Destination
    pub destination: String,
    /// Checksum
    pub checksum: Option<String>,
    /// Status
    pub status: CanonicalReplicationStatus,
    /// Started At
    pub started_at: SystemTime,
    /// Completed At
    pub completed_at: Option<SystemTime>,
}
/// Canonical storage target - unified replication targets
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalstoragetarget
pub struct CanonicalStorageTarget {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Target Type
    pub target_type: CanonicalTargetType,
    /// Endpoint
    pub endpoint: String,
    /// Credentials
    pub credentials: Option<HashMap<String, String>>,
    /// Compression
    pub compression: bool,
    /// Encryption
    pub encryption: bool,
    /// Priority
    pub priority: u8,
}
/// Canonical target types - unified storage target classification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of CanonicalTarget
pub enum CanonicalTargetType {
    /// Filesystem
    Filesystem,
    /// S3Compatible
    S3Compatible,
    /// Remotenestgate
    RemoteNestGate,
    /// Networkshare
    NetworkShare,
    /// Zfspool
    ZfsPool,
    /// Cloudstorage
    CloudStorage,
}
/// Canonical backup types - unified backup classification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of CanonicalBackup
pub enum CanonicalBackupType {
    /// Full
    Full,
    /// Incremental
    Incremental,
    /// Differential
    Differential,
    /// Snapshot
    Snapshot,
}
/// Canonical optimization categories - unified optimization tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Canonicaloptimizationcategory
pub enum CanonicalOptimizationCategory {
    /// Compression
    Compression,
    /// Caching
    Caching,
    /// Tiering
    Tiering,
    /// Deduplication
    Deduplication,
    /// Networkoptimization
    NetworkOptimization,
    /// Indexoptimization
    IndexOptimization,
    /// Memorymanagement
    MemoryManagement,
    /// Iooptimization
    IOOptimization,
    /// Replication
    Replication,
    /// Backup
    Backup,
}
/// Canonical priority levels - unified priority system
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalpriority
pub enum CanonicalPriority {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium
    Medium,
    /// Low
    Low,
    /// Minimal
    Minimal,
}
/// Canonical access patterns - unified access pattern tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalaccesspatterns
pub struct CanonicalAccessPatterns {
    // Core frequency metrics
    pub read_frequency: u64,
    /// Write Frequency
    pub write_frequency: u64,
    /// Count of daily access
    pub daily_access_count: u64,
    // Temporal tracking
    pub last_access: Option<SystemTime>,
    /// Temporal Patterns
    pub temporal_patterns: Vec<CanonicalAccessTimePattern>,
    /// Peak Access Hours
    pub peak_access_hours: Vec<u8>,

    // Access method tracking
    pub access_methods: Vec<String>,
    /// Access Types
    pub access_types: Vec<String>,

    // User and performance metrics
    pub user_access_count: HashMap<String, u64>,
    /// Read Write Ratio
    pub read_write_ratio: f64,
    /// Sequential Access Ratio
    pub sequential_access_ratio: f64,
    /// Size of average file
    pub average_file_size: u64,
}

/// Canonical access time pattern - unified temporal access tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Canonicalaccesstimepattern
pub struct CanonicalAccessTimePattern {
    /// Hour
    pub hour: u8,
    /// Day Of Week
    pub day_of_week: u8,
    /// Count of access
    pub access_count: u64,
    /// Average Duration
    pub average_duration: Duration,
}
impl Default for CanonicalAccessPatterns {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            read_frequency: 0,
            write_frequency: 0,
            daily_access_count: 0,
            last_access: None,
            temporal_patterns: Vec::new(),
            peak_access_hours: Vec::new(),
            access_methods: Vec::new(),
            access_types: Vec::new(),
            user_access_count: HashMap::new(),
            read_write_ratio: 1.0,
            sequential_access_ratio: 0.5,
            average_file_size: 0,
        }
    }
}

// ==================== SECTION ====================

// **DEPRECATED CODE REMOVED** - UnifiedConfig has been successfully migrated
// to the canonical NestGateCanonicalConfig system. All usage has been updated
// to use crate::config::canonical_primary::NestGateCanonicalConfig instead.
//
// **MIGRATION COMPLETE**: This deprecated struct has been eliminated as part
// of the canonical modernization cleanup.

/// Canonical storage configuration - unified storage settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for CanonicalStorage
pub struct CanonicalStorageConfig {
    /// Default Backend
    pub default_backend: String,
    /// Replication Targets
    pub replication_targets: Vec<CanonicalStorageTarget>,
    /// Configuration for backup
    pub backup_config: CanonicalBackupConfig,
    /// Configuration for optimization
    pub optimization_config: CanonicalOptimizationConfig,
    /// Access Patterns
    pub access_patterns: CanonicalAccessPatterns,
}
impl Default for CanonicalStorageConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            default_backend: "filesystem".to_string(),
            replication_targets: Vec::new(),
            backup_config: CanonicalBackupConfig::default(),
            optimization_config: CanonicalOptimizationConfig::default(),
            access_patterns: CanonicalAccessPatterns::default(),
        }
    }
}

/// Canonical backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for CanonicalBackup
pub struct CanonicalBackupConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Backup Type
    pub backup_type: CanonicalBackupType,
    /// Schedule
    pub schedule: String,
    /// Retention Days
    pub retention_days: u32,
    /// Compression
    pub compression: bool,
    /// Encryption
    pub encryption: bool,
}
impl Default for CanonicalBackupConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            backup_type: CanonicalBackupType::Incremental,
            schedule: "0 2 * * *".to_string(), // Daily at 2 AM
            retention_days: 30,
            compression: true,
            encryption: true,
        }
    }
}

/// Canonical optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for CanonicalOptimization
pub struct CanonicalOptimizationConfig {
    /// Enabled Categories
    pub enabled_categories: Vec<CanonicalOptimizationCategory>,
    /// Auto Optimization
    pub auto_optimization: bool,
    /// Optimization Schedule
    pub optimization_schedule: String,
    /// Performance Threshold
    pub performance_threshold: f64,
}
impl Default for CanonicalOptimizationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled_categories: vec![
                CanonicalOptimizationCategory::Compression,
                CanonicalOptimizationCategory::Caching,
                CanonicalOptimizationCategory::Deduplication,
            ],
            auto_optimization: true,
            optimization_schedule: "0 3 * * 0".to_string(), // Weekly on Sunday at 3 AM
            performance_threshold: 0.8,
        }
    }
}

/// **UNIFIED SERVICE CONFIG**
///
/// Service-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::UnifiedServiceConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::UnifiedServiceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for UnifiedService
pub struct UnifiedServiceConfig {
    /// Service name
    pub name: String,
    /// Service version
    pub version: String,
    /// Service port
    pub port: u16,
    /// Service bind address
    pub bind_endpoint: String,
    /// Service metadata
    pub metadata: HashMap<String, String>,
}
impl Default for UnifiedServiceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            name: "nestgate".to_string(),
            version: "1.0.0".to_string(),
            port: 8080,
            bind_endpoint: "0.0.0.0".to_string(),
            metadata: HashMap::new(),
        }
    }
}

/// **UNIFIED NETWORK CONFIG**
///
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_primary::domains::network`
/// Network-specific configuration
#[deprecated(
    since = "0.9.0",
    note = "Use canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::UnifiedNetworkConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::UnifiedNetworkConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for UnifiedNetwork
pub struct UnifiedNetworkConfig {
    /// Maximum connections
    pub max_connections: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Buffer size
    pub buffer_size: usize,
}
#[allow(deprecated)]
impl Default for UnifiedNetworkConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_connections: 1000,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(30),
            buffer_size: 65536,
        }
    }
}

/// **UNIFIED PERFORMANCE CONFIG**
///
/// Performance-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for UnifiedPerformance
pub struct UnifiedPerformanceConfig {
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
}
impl Default for UnifiedPerformanceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            metrics_interval: Duration::from_secs(60),
            thresholds: PerformanceThresholds::default(),
        }
    }
}

/// **UNIFIED PERFORMANCE TEST CONFIG**
///
/// Configuration for performance testing
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for UnifiedPerformanceTest
pub struct UnifiedPerformanceTestConfig {
    /// Test duration
    pub duration: Duration,
    /// Number of concurrent requests
    pub concurrency: usize,
    /// Request rate per second
    pub rate_per_second: usize,
    /// Enable detailed metrics
    pub detailed_metrics: bool,
}
impl Default for UnifiedPerformanceTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(60),
            concurrency: 10,
            rate_per_second: 100,
            detailed_metrics: true,
        }
    }
}

/// **PERFORMANCE THRESHOLDS**
///
/// Performance monitoring thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancethresholds
pub struct PerformanceThresholds {
    /// Maximum latency in milliseconds
    pub max_latency_ms: u64,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
}
impl Default for PerformanceThresholds {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_latency_ms: 1000,
            max_cpu_percent: 80.0,
            max_memory_bytes: 1024 * 1024 * 1024, // 1GB
        }
    }
}

/// **CONFIG METADATA**
///
/// Metadata about configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configmetadata
pub struct ConfigMetadata {
    /// Configuration version
    pub version: String,
    /// Creation timestamp
    pub created_at: SystemTime,
    /// Last modified timestamp
    pub modified_at: SystemTime,
    /// Configuration source
    pub source: String,
}
impl Default for ConfigMetadata {
    /// Returns the default instance
    fn default() -> Self {
        let now = SystemTime::now();
        Self {
            version: "1.0.0".to_string(),
            created_at: now,
            modified_at: now,
            source: "default".to_string(),
        }
    }
}

/// **UNIFIED CACHE CONFIG**
///
/// Cache configuration for various caching strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::config::UnifiedCacheConfig;
/// 
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::UnifiedCacheConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for UnifiedCache
pub struct UnifiedCacheConfig {
    /// Cache size in bytes
    pub cache_size_bytes: u64,
    /// Maximum number of cache entries
    pub max_entries: usize,
    /// Time-to-live in seconds
    pub ttl_secs: u64,
    /// Cache eviction strategy
    pub eviction_strategy: String,
    /// Enable cache compression
    pub enable_compression: bool,
    /// Cache persistence enabled
    pub enable_persistence: bool,
    /// Cache directory path
    pub cache_directory: String,
}
impl Default for UnifiedCacheConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cache_size_bytes: 1024 * 1024 * 100, // 100MB
            max_entries: 10000,
            ttl_secs: 3600, // 1 hour
            eviction_strategy: "lru".to_string(),
            enable_compression: false,
            enable_persistence: false,
            cache_directory: "/tmp/nestgate_cache".to_string(),
        }
    }
}

impl UnifiedCacheConfig {
    /// Create a new cache config with specified size
    #[must_use]
    pub fn with_size(cache_size_bytes: u64) -> Self {
        Self {
            cache_size_bytes,
            ..Default::default()
        }
    }

    /// Enable compression
    #[must_use]
    pub fn with_compression(mut self) -> Self {
        self.enable_compression = true;
        self
    }

    /// Enable persistence
    #[must_use]
    pub fn with_persistence(mut self, directory: String) -> Self {
        self.enable_persistence = true;
        self.cache_directory = directory;
        self
    }
}

/// **CUSTOM METRICS MAP**
///
/// Type alias for custom metrics storage
pub type CustomMetricsMap = HashMap<String, MetricValue>;
/// **METRIC VALUE**
///
/// Represents different types of metric values
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Metricvalue
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    /// Summary
    Summary { sum: f64, count: u64 },
    String(String),
}
// ==================== SECTION ====================

/// Migration utility to convert from legacy storage types
pub mod migration {
    use super::{CanonicalOptimizationCategory, CanonicalReplicationStatus};
    /// Convert legacy `ReplicationStatus` to canonical
    #[must_use]
    pub fn migrate_replication_status(legacy_status: &str) -> CanonicalReplicationStatus {
        match legacy_status.to_lowercase().as_str() {
            "active" => CanonicalReplicationStatus::Active,
            "paused" => CanonicalReplicationStatus::Paused,
            "failed" => CanonicalReplicationStatus::Failed,
            "disabled" => CanonicalReplicationStatus::Disabled,
            "initializing" => CanonicalReplicationStatus::Initializing,
            "syncing" => CanonicalReplicationStatus::Syncing,
            "completed" => CanonicalReplicationStatus::Completed,
            "queued" => CanonicalReplicationStatus::Queued,
            "in_progress" | "inprogress" => CanonicalReplicationStatus::InProgress,
            "cancelled" => CanonicalReplicationStatus::Cancelled,
            _ => CanonicalReplicationStatus::Disabled,
        }
    }

    /// Convert legacy optimization category to canonical
    #[must_use]
    pub fn migrate_optimization_category(legacy_category: &str) -> CanonicalOptimizationCategory {
        match legacy_category.to_lowercase().as_str() {
            "compression" => CanonicalOptimizationCategory::Compression,
            "caching" => CanonicalOptimizationCategory::Caching,
            "tiering" => CanonicalOptimizationCategory::Tiering,
            "deduplication" => CanonicalOptimizationCategory::Deduplication,
            "network" | "networkoptimization" => CanonicalOptimizationCategory::NetworkOptimization,
            "index" | "indexoptimization" => CanonicalOptimizationCategory::IndexOptimization,
            "memory" | "memorymanagement" => CanonicalOptimizationCategory::MemoryManagement,
            "io" | "iooptimization" => CanonicalOptimizationCategory::IOOptimization,
            "replication" => CanonicalOptimizationCategory::Replication,
            "backup" => CanonicalOptimizationCategory::Backup,
            _ => CanonicalOptimizationCategory::Compression,
        }
    }
}


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Unifiednetworkconfigcanonical
pub type UnifiedNetworkConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UnifiedNetworkConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Unifiedserviceconfigcanonical
pub type UnifiedServiceConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UnifiedServiceConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.


// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Unifiedcacheconfigcanonical
pub type UnifiedCacheConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UnifiedCacheConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_replication_status_display() {
        assert_eq!(CanonicalReplicationStatus::Active.to_string(), "active");
        assert_eq!(CanonicalReplicationStatus::Failed.to_string(), "failed");
        assert_eq!(
            CanonicalReplicationStatus::InProgress.to_string(),
            "in_progress"
        );
    }

    #[test]
    fn test_canonical_storage_range_creation() {
        let range = CanonicalStorageRange {
            start: 0,
            end: 100,
            inclusive: true,
            chunk_size: Some(10),
        };
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 100);
        assert!(range.inclusive);
        assert_eq!(range.chunk_size, Some(10));
    }

    #[test]
    fn test_migration_utilities() {
        use super::migration::*;

        assert_eq!(
            migrate_replication_status("active"),
            CanonicalReplicationStatus::Active
        );
        assert_eq!(
            migrate_replication_status("in_progress"),
            CanonicalReplicationStatus::InProgress
        );

        assert_eq!(
            migrate_optimization_category("compression"),
            CanonicalOptimizationCategory::Compression
        );
        assert_eq!(
            migrate_optimization_category("networkoptimization"),
            CanonicalOptimizationCategory::NetworkOptimization
        );
    }

    #[test]
    fn test_canonical_access_patterns_default() {
        let patterns = CanonicalAccessPatterns::default();
        assert_eq!(patterns.read_frequency, 0);
        assert_eq!(patterns.write_frequency, 0);
        assert_eq!(patterns.read_write_ratio, 1.0);
        assert_eq!(patterns.sequential_access_ratio, 0.5);
    }
}
