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
pub struct CanonicalStorageChange {
    pub id: String,
    pub timestamp: SystemTime,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub source: String,
    pub destination: Option<String>,
}
/// Canonical storage directory entry - unified file system representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalStorageDirectoryEntry {
    pub name: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: SystemTime,
    pub permissions: Option<String>,
    pub owner: Option<String>,
    pub group: Option<String>,
    pub checksum: Option<String>,
}
/// Canonical storage range specification - unified range operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalStorageRange {
    pub start: u64,
    pub end: u64,
    pub inclusive: bool,
    pub chunk_size: Option<u64>,
}
/// Canonical replication status - unified across all replication systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CanonicalReplicationStatus {
    Active,
    Paused,
    Failed,
    Disabled,
    Initializing,
    Syncing,
    Completed,
    Queued,
    InProgress,
    Cancelled,
}
impl Default for CanonicalReplicationStatus {
    fn default() -> Self {
        Self::Disabled
    }
}

impl std::fmt::Display for CanonicalReplicationStatus {
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
pub struct CanonicalStorageReplicationResult {
    pub success: bool,
    pub replicated_bytes: u64,
    pub duration_ms: u64,
    pub error_message: Option<String>,
    pub source: String,
    pub destination: String,
    pub checksum: Option<String>,
    pub status: CanonicalReplicationStatus,
    pub started_at: SystemTime,
    pub completed_at: Option<SystemTime>,
}
/// Canonical storage target - unified replication targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalStorageTarget {
    pub id: String,
    pub name: String,
    pub target_type: CanonicalTargetType,
    pub endpoint: String,
    pub credentials: Option<HashMap<String, String>>,
    pub compression: bool,
    pub encryption: bool,
    pub priority: u8,
}
/// Canonical target types - unified storage target classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CanonicalTargetType {
    Filesystem,
    S3Compatible,
    RemoteNestGate,
    NetworkShare,
    ZfsPool,
    CloudStorage,
}
/// Canonical backup types - unified backup classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CanonicalBackupType {
    Full,
    Incremental,
    Differential,
    Snapshot,
}
/// Canonical optimization categories - unified optimization tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CanonicalOptimizationCategory {
    Compression,
    Caching,
    Tiering,
    Deduplication,
    NetworkOptimization,
    IndexOptimization,
    MemoryManagement,
    IOOptimization,
    Replication,
    Backup,
}
/// Canonical priority levels - unified priority system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CanonicalPriority {
    Critical,
    High,
    Medium,
    Low,
    Minimal,
}
/// Canonical access patterns - unified access pattern tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAccessPatterns {
    // Core frequency metrics
    pub read_frequency: u64,
    pub write_frequency: u64,
    pub daily_access_count: u64,
    // Temporal tracking
    pub last_access: Option<SystemTime>,
    pub temporal_patterns: Vec<CanonicalAccessTimePattern>,
    pub peak_access_hours: Vec<u8>,

    // Access method tracking
    pub access_methods: Vec<String>,
    pub access_types: Vec<String>,

    // User and performance metrics
    pub user_access_count: HashMap<String, u64>,
    pub read_write_ratio: f64,
    pub sequential_access_ratio: f64,
    pub average_file_size: u64,
}

/// Canonical access time pattern - unified temporal access tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAccessTimePattern {
    pub hour: u8,
    pub day_of_week: u8,
    pub access_count: u64,
    pub average_duration: Duration,
}
impl Default for CanonicalAccessPatterns {
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
pub struct CanonicalStorageConfig {
    pub default_backend: String,
    pub replication_targets: Vec<CanonicalStorageTarget>,
    pub backup_config: CanonicalBackupConfig,
    pub optimization_config: CanonicalOptimizationConfig,
    pub access_patterns: CanonicalAccessPatterns,
}
impl Default for CanonicalStorageConfig {
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
pub struct CanonicalBackupConfig {
    pub enabled: bool,
    pub backup_type: CanonicalBackupType,
    pub schedule: String,
    pub retention_days: u32,
    pub compression: bool,
    pub encryption: bool,
}
impl Default for CanonicalBackupConfig {
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
pub struct CanonicalOptimizationConfig {
    pub enabled_categories: Vec<CanonicalOptimizationCategory>,
    pub auto_optimization: bool,
    pub optimization_schedule: String,
    pub performance_threshold: f64,
}
impl Default for CanonicalOptimizationConfig {
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
pub struct UnifiedPerformanceConfig {
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
}
impl Default for UnifiedPerformanceConfig {
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
pub struct PerformanceThresholds {
    /// Maximum latency in milliseconds
    pub max_latency_ms: u64,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
}
impl Default for PerformanceThresholds {
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
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
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
