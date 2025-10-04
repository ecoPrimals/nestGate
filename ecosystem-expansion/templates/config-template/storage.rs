//! Storage and ZFS configuration structures
//!
//! **TEMPLATE NOTE**: This template demonstrates the canonical pattern.
//! Use CanonicalStorageConfig from canonical_master/domains/storage_canonical
//! for all new code.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== CANONICAL PATTERN ====================

// **RECOMMENDED**: Use canonical storage configuration
pub use nestgate_core::config::canonical_master::domains::storage_canonical::CanonicalStorageConfig;

// **TYPE ALIAS PATTERN**: For module-specific naming
pub type StorageConfig = CanonicalStorageConfig;

// ==================== HELPER TYPES ====================
// If you need simplified types for specific use cases, define them separately
// and provide conversion functions to/from CanonicalStorageConfig

/// ZFS-specific configuration (helper struct for compatibility)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfig {
    /// Default ZFS pool name
    pub default_pool: String,
    /// ZFS command timeout
    pub command_timeout: Duration,
    /// Enable ZFS snapshots
    pub snapshots_enabled: bool,
    /// Snapshot retention policy
    pub snapshot_retention_days: u32,
    /// Enable ZFS compression
    pub compression_enabled: bool,
    /// Compression algorithm
    pub compression_algorithm: String,
    /// Enable ZFS deduplication
    pub deduplication_enabled: bool,
    /// Enable ZFS encryption
    pub encryption_enabled: bool,
    // COMPATIBILITY: Add missing fields for legacy code
    pub pool_name: String,
    pub dataset_name: String,
    pub compression: String,
    pub use_sudo: bool,
}

/// NAS protocol configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasConfig {
    pub enabled: bool,
}

/// Storage tier configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    pub enabled: bool,
}

/// Storage performance configuration placeholder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceConfig {
    pub enabled: bool,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub size_mb: u32,
    pub ttl_seconds: u32,
    pub hot_tier_size: Option<u64>,
    pub warm_tier_size: Option<u64>,
    pub cold_tier_size: Option<u64>,
    pub eviction_policy: String,
    pub policy: String, // Alias for eviction_policy for compatibility
}

// Default implementations
impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            zfs: ZfsConfig::default(),
            nas: NasConfig::default(),
            tiers: TierConfig::default(),
            performance: StoragePerformanceConfig::default(),
            cache: CacheConfig::default(),
            backend_type: "zfs".to_string(),
        }
    }
}

impl Default for ZfsConfig {
    fn default() -> Self {
        Self {
            default_pool: "tank".to_string(),
            command_timeout: Duration::from_secs(30),
            snapshots_enabled: true,
            snapshot_retention_days: 30,
            compression_enabled: true,
            compression_algorithm: "lz4".to_string(),
            deduplication_enabled: false,
            encryption_enabled: false,
            pool_name: "tank".to_string(),
            dataset_name: "nestgate".to_string(),
            compression: "lz4".to_string(),
            use_sudo: true,
        }
    }
}

impl Default for NasConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl Default for TierConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl Default for StoragePerformanceConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size_mb: 512,
            ttl_seconds: 3600,
            hot_tier_size: std::env::var("NESTGATE_CACHE_HOT_TIER_SIZE")
                .ok()
                .and_then(|v| v.parse().ok()),
            warm_tier_size: std::env::var("NESTGATE_CACHE_WARM_TIER_SIZE")
                .ok()
                .and_then(|v| v.parse().ok()),
            cold_tier_size: std::env::var("NESTGATE_CACHE_COLD_TIER_SIZE")
                .ok()
                .and_then(|v| v.parse().ok()),
            eviction_policy: std::env::var("NESTGATE_CACHE_EVICTION_POLICY")
                .unwrap_or_else(|_| "lru".to_string()), // Least Recently Used by default
            policy: std::env::var("NESTGATE_CACHE_POLICY").unwrap_or_else(|_| "lru".to_string()), // Compatibility alias
        }
    }
}

impl CacheConfig {
    /// Create a development-optimized cache configuration
    #[must_use]
    pub fn development() -> Self {
        Self {
            enabled: true,
            size_mb: 128,                           // Smaller cache for development
            ttl_seconds: 1800,                      // 30 minutes
            hot_tier_size: Some(64 * 1024 * 1024),  // 64MB hot tier
            warm_tier_size: Some(32 * 1024 * 1024), // 32MB warm tier
            cold_tier_size: Some(32 * 1024 * 1024), // 32MB cold tier
            eviction_policy: "lru".to_string(),
            policy: "lru".to_string(),
        }
    }

    /// Create a high-performance cache configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self {
            enabled: true,
            size_mb: 2048,                           // 2GB cache
            ttl_seconds: 7200,                       // 2 hours
            hot_tier_size: Some(1024 * 1024 * 1024), // 1GB hot tier
            warm_tier_size: Some(512 * 1024 * 1024), // 512MB warm tier
            cold_tier_size: Some(512 * 1024 * 1024), // 512MB cold tier
            eviction_policy: "lfu".to_string(),      // Least Frequently Used for performance
            policy: "lfu".to_string(),
        }
    }
}
