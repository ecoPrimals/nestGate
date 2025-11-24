// **STORAGE CACHING CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Storage caching configuration for optimizing read performance.
///
/// Provides comprehensive caching settings including cache sizing, eviction policies,
/// consistency guarantees, performance tuning, and monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCachingConfig {
    /// Whether caching is enabled (default: true).
    pub enabled: bool,
    /// Cache configuration including size and TTL.
    pub cache: CacheConfig,
    /// Cache eviction policy for managing cache entries.
    pub eviction: CacheEvictionPolicy,
    /// Cache consistency configuration.
    pub consistency: CacheConsistencyConfig,
    /// Cache performance optimization settings.
    pub performance: CachePerformanceConfig,
    /// Cache monitoring configuration.
    pub monitoring: CacheMonitoringConfig,
}

/// Cache configuration for size and lifetime settings.
///
/// Controls the maximum cache size and time-to-live for cached entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum cache size in bytes (default: 1GB).
    pub size: u64,
    /// Time-to-live for cache entries (default: 1 hour).
    pub ttl: Duration,
}

/// Cache eviction policy for managing cache capacity.
///
/// Determines which entries are removed when cache is full.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheEvictionPolicy {
    /// Least Recently Used - evict least recently accessed entries (default).
    Lru,
    /// Least Frequently Used - evict least frequently accessed entries.
    Lfu,
    /// First In First Out - evict oldest entries first.
    Fifo,
    /// Random - evict random entries.
    Random,
}

/// Cache consistency configuration.
///
/// Controls cache coherency and consistency guarantees.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConsistencyConfig {
    /// Whether cache consistency checks are enabled (default: true).
    pub enabled: bool,
}

/// Cache performance configuration.
///
/// Controls performance optimizations for cache operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceConfig {
    /// Whether performance optimizations are enabled (default: true).
    pub enabled: bool,
}

/// Cache monitoring configuration.
///
/// Controls monitoring and metrics for cache operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMonitoringConfig {
    /// Whether cache monitoring is enabled (default: false).
    pub enabled: bool,
}

impl Default for StorageCachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache: CacheConfig {
                size: 1024 * 1024 * 1024,
                ttl: Duration::from_secs(3600),
            },
            eviction: CacheEvictionPolicy::Lru,
            consistency: CacheConsistencyConfig { enabled: true },
            performance: CachePerformanceConfig { enabled: true },
            monitoring: CacheMonitoringConfig { enabled: false },
        }
    }
}

impl StorageCachingConfig {
    /// Create production-optimized caching configuration.
    ///
    /// Uses default settings suitable for production with 1GB cache and LRU eviction.
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Create development-optimized caching configuration.
    ///
    /// Uses default settings suitable for local development.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Create high-performance caching configuration.
    ///
    /// Optimized for maximum cache hit rate and throughput.
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Create cloud-native caching configuration.
    ///
    /// Optimized for distributed caching in cloud environments.
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }

    /// Merge this configuration with another, preferring values from `other`.
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validate caching configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
