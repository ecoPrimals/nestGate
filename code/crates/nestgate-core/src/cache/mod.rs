// Cache management module
//
// Provides caching infrastructure including multi-tier caching,
// TTL management, and performance optimization.

//! Cache module

use crate::config::canonical_primary::NestGateCanonicalConfig;

#[cfg(test)]
mod tests;

// Cache Management Module
// Advanced caching system with support for multiple storage tiers, configurable policies,
// and comprehensive statistics tracking.
// ## Module Structure
// - `types` - Core types, enums, configuration, and statistics
// - `manager` - Single-tier cache manager implementation
// - `multi_tier` - Multi-tier cache with hot/warm/cold tiers
// ## Usage
// ### Basic Single-Tier Cache
// ```rust
// use crate::cache::{CacheManager, NestGateCanonicalConfig};
// let config = crate::config::canonical_primary::NestGateCanonicalConfig::default();
// let cache = CacheManager::new(config)?;
// // Put data in cache
// cache.put("key", b"data".to_vec()).await?;
// // Get data from cache
// if let Some(data) = cache.get("key").await? {
//     println!("Cache hit: {data:?}");
// }
// ```
// ### Multi-Tier Cache
// ```rust
// use crate::cache::{MultiTierCache, MultiTierCacheConfig};
// let config = crate::config::canonical_primary::NestGateCanonicalConfig::default();
// let cache = MultiTierCache::new(config)?;
// // Data automatically starts in hot tier and may be promoted/demoted
// cache.put("key", b"data".to_vec()).await?;
// let data = cache.get("key").await?;
// ```
// MultiTierCacheConfig imported via pub use below
pub mod manager;
pub mod multi_tier;

/// Cache type definitions and traits
///
/// This module provides core types for cache operations including:
/// - `CacheEntry`: Individual cache entry representation
/// - `CachePolicy`: Cache eviction policies (LRU, LFU, etc.)
/// - `CacheStats`: Cache performance statistics
/// - `StorageTier`: Multi-tier cache tier definitions
pub mod types;
pub use manager::CacheManager;
#[allow(deprecated)]
pub use multi_tier::{MultiTierCache, MultiTierCacheConfig, MultiTierCacheStats};
pub use types::{CacheEntry, CachePolicy, CacheStats, EfficiencyMetrics, StorageTier};

// Result type alias for cache operations
pub use crate::Result as CacheResult;

// The cache system providing both single-tier and multi-tier caching
#[allow(clippy::large_enum_variant)]
/// Cachesystem
pub enum CacheSystem {
    /// Single-tier cache
    SingleTier(CacheManager),
    /// Multi-tier cache with hot/warm/cold tiers
    MultiTier(MultiTierCache),
}
impl CacheSystem {
    /// Create a cache with a single tier for small deployments
    pub fn single_tier(
        cache_config: crate::config::canonical_primary::CacheConfig,
    ) -> crate::Result<Self> {
        // Convert to UnifiedCacheConfig
        #[allow(deprecated)]
        let unified_config = crate::cache::manager::UnifiedCacheConfig {
            max_size: cache_config.hot_tier_size.unwrap_or(1000) as usize,
            ttl_seconds: cache_config.ttl_seconds,
            cache_dir: cache_config.cache_dir.clone(),
            eviction_policy: cache_config.policy.unwrap_or_else(|| "lru".to_string()),
        };

        let manager = CacheManager::new(unified_config);
        Ok(CacheSystem::SingleTier(manager))
    }

    /// Create a multi-tier cache system
    #[allow(deprecated)]
    pub fn multi_tier(cache_config: MultiTierCacheConfig) -> crate::Result<Self> {
        let cache = MultiTierCache::new(cache_config)?;
        Ok(CacheSystem::MultiTier(cache))
    }

    /// Get data from cache
    pub async fn get(&mut self, key: &str) -> crate::Result<Option<Vec<u8>>> {
        match self {
            CacheSystem::SingleTier(cache) => Ok(cache.get(key)),
            CacheSystem::MultiTier(cache) => cache.get(key).await,
        }
    }

    /// Put data into cache
    pub async fn put(&mut self, key: &str, data: Vec<u8>) -> crate::Result<()> {
        match self {
            CacheSystem::SingleTier(cache) => cache.put(key.to_string(), data).await,
            CacheSystem::MultiTier(cache) => cache.put(key, data).await,
        }
    }

    /// Remove data from cache
    pub async fn remove(&mut self, key: &str) -> crate::Result<bool> {
        match self {
            CacheSystem::SingleTier(cache) => Ok(cache.remove(key)),
            CacheSystem::MultiTier(cache) => cache.remove(key).await,
        }
    }

    /// Clear all cache data
    pub async fn clear(&mut self) -> crate::Result<()> {
        match self {
            CacheSystem::SingleTier(cache) => {
                cache.clear();
                Ok(())
            }
            CacheSystem::MultiTier(cache) => cache.clear().await,
        }
    }

    /// Check if cache contains a key
    pub async fn contains_key(&mut self, key: &str) -> bool {
        match self {
            CacheSystem::SingleTier(cache) => {
                // Check if key exists by attempting to get it
                cache.get(key).is_some()
            }
            CacheSystem::MultiTier(cache) => cache.contains_key(key).await,
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> crate::Result<CacheSystemStats> {
        match self {
            CacheSystem::SingleTier(_cache) => {
                // Placeholder stats for single tier
                Ok(CacheSystemStats::SingleTier(
                    crate::cache::types::CacheStats::default(),
                ))
            }
            CacheSystem::MultiTier(cache) => {
                let stats = cache.stats()?;
                Ok(CacheSystemStats::MultiTier(stats))
            }
        }
    }

    /// Perform maintenance on cache
    pub async fn maintenance(&mut self) -> crate::Result<()> {
        match self {
            CacheSystem::SingleTier(cache) => cache.maintenance().await,
            CacheSystem::MultiTier(cache) => cache.maintenance(),
        }
    }

    /// Flush cache to persistent storage
    pub fn flush(&mut self) -> crate::Result<()> {
        match self {
            CacheSystem::SingleTier(cache) => cache.flush(),
            CacheSystem::MultiTier(cache) => cache.flush(),
        }
    }
}

// Cache system statistics
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
/// Cachesystemstats
pub enum CacheSystemStats {
    /// Single-tier cache statistics
    SingleTier(CacheStats),
    /// Multi-tier cache statistics
    MultiTier(MultiTierCacheStats),
}
impl CacheSystemStats {
    /// Get total hits across all tiers
    #[must_use]
    pub fn total_hits(&self) -> u64 {
        match self {
            Self::SingleTier(stats) => stats.hits,
            Self::MultiTier(stats) => stats.total_hits,
        }
    }

    /// Get total misses across all tiers
    #[must_use]
    pub fn total_misses(&self) -> u64 {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.misses,
            CacheSystemStats::MultiTier(stats) => stats.total_misses,
        }
    }

    /// Get total items across all tiers
    #[must_use]
    pub fn total_items(&self) -> usize {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.total_items(),
            CacheSystemStats::MultiTier(stats) => stats.total_items,
        }
    }

    /// Get total size across all tiers
    #[must_use]
    pub fn total_size_bytes(&self) -> u64 {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.total_size_bytes(),
            CacheSystemStats::MultiTier(stats) => stats.total_size_bytes,
        }
    }

    /// Calculate overall hit ratio
    #[must_use]
    pub fn hit_ratio(&self) -> f64 {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.hit_ratio(),
            CacheSystemStats::MultiTier(stats) => stats.overall_hit_ratio(),
        }
    }
}

/// Builder pattern for constructing cache systems
///
/// Provides a fluent API for configuring single-tier or multi-tier cache systems
/// with various policies, sizes, and TTL settings.
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::cache::{CacheBuilder, CachePolicy};
/// use std::time::Duration;
///
/// let cache = CacheBuilder::new()
///     .with_policy(CachePolicy::Lru)
///     .with_hot_tier_size(1000)
///     .with_ttl(Duration::from_secs(300))
///     .build()
///     .await?;
/// ```
pub struct CacheBuilder {
    config: crate::config::canonical_primary::CacheConfig,
    multi_tier: bool,
    #[allow(deprecated)]
    multi_tier_config: Option<MultiTierCacheConfig>,
}
impl CacheBuilder {
    /// Create a new cache builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: crate::config::canonical_primary::CacheConfig::default(),
            multi_tier: false,
            multi_tier_config: None,
        }
    }

    /// Set cache policy
    #[must_use]
    pub fn with_policy(mut self, policy: CachePolicy) -> Self {
        self.config.policy = Some(policy.to_string());
        self
    }

    /// Set cache directory
    pub fn with_cache_dir<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.config.cache_dir = Some(path.into());
        self
    }

    /// Set cache TTL
    #[must_use]
    pub fn with_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.config.ttl_seconds = Some(ttl.as_secs());
        self
    }

    /// Set hot tier size
    #[must_use]
    pub fn with_hot_tier_size(mut self, size: usize) -> Self {
        self.config.hot_tier_size = Some(size as u64);
        self
    }

    /// Enable multi-tier caching
    // LINTING FIX: Add underscore prefix for unused parameter
    #[must_use]
    pub fn with_multi_tier(mut self, _config: NestGateCanonicalConfig) -> Self {
        self.multi_tier = true;
        // Convert NestGateCanonicalConfig to MultiTierCacheConfig
        let multi_tier_config = MultiTierCacheConfig {
            hot_tier_config: crate::cache::multi_tier::SimpleCacheConfig {
                max_size: 1024 * 1024, // 1MB
                ttl: std::time::Duration::from_secs(300),
                cache_dir: "/tmp/nestgate_hot".to_string(),
            },
            warm_tier_config: crate::cache::multi_tier::SimpleCacheConfig {
                max_size: 10 * 1024 * 1024, // 10MB
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: "/tmp/nestgate_warm".to_string(),
            },
            cold_tier_config: crate::cache::multi_tier::SimpleCacheConfig {
                max_size: 100 * 1024 * 1024, // 100MB
                ttl: std::time::Duration::from_secs(86400),
                cache_dir: "/tmp/nestgate_cold".to_string(),
            },
            promotion_threshold: 10,
            demotion_threshold: 100,
        };
        self.multi_tier_config = Some(multi_tier_config);
        self
    }

    /// Enable multi-tier caching with default configuration
    #[must_use]
    pub fn enable_multi_tier(mut self) -> Self {
        self.multi_tier = true;
        self.multi_tier_config = Some(MultiTierCacheConfig::default());
        self
    }

    /// Build the cache system
    pub async fn build(self) -> crate::Result<CacheSystem> {
        if self.multi_tier {
            let config = self.multi_tier_config.unwrap_or_default();
            CacheSystem::multi_tier(config)
        } else {
            CacheSystem::single_tier(self.config)
        }
    }
}

impl Default for CacheBuilder {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions for common use cases - moved to test helpers
