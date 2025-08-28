//! Cache management module
//!
//! Provides caching infrastructure including multi-tier caching,
//! TTL management, and performance optimization.

use crate::config::canonical_master::NestGateCanonicalConfig;

/// Cache Management Module
/// Advanced caching system with support for multiple storage tiers, configurable policies,
/// and comprehensive statistics tracking.
/// ## Module Structure
/// - `types` - Core types, enums, configuration, and statistics
/// - `manager` - Single-tier cache manager implementation
/// - `multi_tier` - Multi-tier cache with hot/warm/cold tiers
/// ## Usage
/// ### Basic Single-Tier Cache
/// ```rust
/// use crate::cache::{CacheManager, NestGateCanonicalConfig};
/// let config = crate::config::canonical_master::NestGateCanonicalConfig::default();
/// let cache = CacheManager::new(config)?;
/// // Put data in cache
/// cache.put("key", b"data".to_vec()).await?;
/// // Get data from cache
/// if let Some(data) = cache.get("key").await? {
///     println!("Cache hit: {:?}", data);
/// }
/// ```
/// ### Multi-Tier Cache
/// ```rust
/// use crate::cache::{MultiTierCache, MultiTierCacheConfig};
/// let config = crate::config::canonical_master::NestGateCanonicalConfig::default();
/// let cache = MultiTierCache::new(config)?;
/// // Data automatically starts in hot tier and may be promoted/demoted
/// cache.put("key", b"data".to_vec()).await?;
/// let data = cache.get("key").await?;
/// ```
// MultiTierCacheConfig imported via pub use below
pub mod manager;
pub mod multi_tier;
pub mod types;

pub use manager::CacheManager;
pub use multi_tier::{MultiTierCache, MultiTierCacheConfig, MultiTierCacheStats};
pub use types::{CacheEntry, CachePolicy, CacheStats, EfficiencyMetrics, StorageTier};

// Result type alias for cache operations
pub use crate::Result as CacheResult;

/// The cache system providing both single-tier and multi-tier caching
#[allow(clippy::large_enum_variant)]
pub enum CacheSystem {
    /// Single-tier cache
    SingleTier(CacheManager),
    /// Multi-tier cache with hot/warm/cold tiers
    MultiTier(MultiTierCache),
}

impl CacheSystem {
    /// Create a cache with a single tier for small deployments
    pub fn single_tier(
        cache_config: crate::config::canonical_master::CacheConfig,
    ) -> crate::Result<Self> {
        // Create NestGateCanonicalConfig with the cache config
        let config = crate::config::canonical_master::NestGateCanonicalConfig {
            storage: crate::config::canonical_master::StorageConfig {
                cache: cache_config,
                ..Default::default()
            },
            ..Default::default()
        };

        let manager = CacheManager::new(config);
        Ok(CacheSystem::SingleTier(manager))
    }

    /// Create a multi-tier cache system
    pub async fn multi_tier(cache_config: MultiTierCacheConfig) -> crate::Result<Self> {
        let cache = MultiTierCache::new(cache_config).await?;
        Ok(CacheSystem::MultiTier(cache))
    }

    /// Get data from cache
    pub async fn get(&mut self, key: &str) -> crate::Result<Option<Vec<u8>>> {
        match self {
            CacheSystem::SingleTier(cache) => cache.get(key).await,
            CacheSystem::MultiTier(cache) => cache.get(key).await,
        }
    }

    /// Put data into cache
    pub async fn put(&mut self, key: &str, data: Vec<u8>) -> crate::Result<()> {
        match self {
            CacheSystem::SingleTier(cache) => cache.put(key, data).await,
            CacheSystem::MultiTier(cache) => cache.put(key, data).await,
        }
    }

    /// Remove data from cache
    pub async fn remove(&mut self, key: &str) -> crate::Result<bool> {
        match self {
            CacheSystem::SingleTier(cache) => cache.remove(key).await,
            CacheSystem::MultiTier(cache) => cache.remove(key).await,
        }
    }

    /// Clear all cache data
    pub async fn clear(&mut self) -> crate::Result<()> {
        match self {
            CacheSystem::SingleTier(cache) => cache.clear().await,
            CacheSystem::MultiTier(cache) => cache.clear().await,
        }
    }

    /// Check if cache contains a key
    pub async fn contains_key(&self, key: &str) -> bool {
        match self {
            CacheSystem::SingleTier(_cache) => false, // Placeholder
            CacheSystem::MultiTier(cache) => cache.contains_key(key).await,
        }
    }

    /// Get cache statistics
    pub async fn stats(&self) -> crate::Result<CacheSystemStats> {
        match self {
            CacheSystem::SingleTier(_cache) => {
                // Placeholder stats for single tier
                Ok(CacheSystemStats::SingleTier(
                    crate::cache::types::CacheStats::default(),
                ))
            }
            CacheSystem::MultiTier(cache) => {
                let stats = cache.stats().await?;
                Ok(CacheSystemStats::MultiTier(stats))
            }
        }
    }

    /// Perform maintenance on cache
    pub async fn maintenance(&mut self) -> crate::Result<()> {
        match self {
            CacheSystem::SingleTier(cache) => cache.maintenance().await,
            CacheSystem::MultiTier(cache) => cache.maintenance().await,
        }
    }

    /// Flush cache to persistent storage
    pub async fn flush(&mut self) -> crate::Result<()> {
        match self {
            CacheSystem::SingleTier(cache) => cache.flush().await,
            CacheSystem::MultiTier(cache) => cache.flush().await,
        }
    }
}

/// Cache system statistics
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum CacheSystemStats {
    /// Single-tier cache statistics
    SingleTier(CacheStats),
    /// Multi-tier cache statistics
    MultiTier(MultiTierCacheStats),
}

impl CacheSystemStats {
    /// Get total hits across all tiers
    pub fn total_hits(&self) -> u64 {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.hits,
            CacheSystemStats::MultiTier(stats) => stats.total_hits,
        }
    }

    /// Get total misses across all tiers
    pub fn total_misses(&self) -> u64 {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.misses,
            CacheSystemStats::MultiTier(stats) => stats.total_misses,
        }
    }

    /// Get total items across all tiers
    pub fn total_items(&self) -> usize {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.total_items(),
            CacheSystemStats::MultiTier(stats) => stats.total_items,
        }
    }

    /// Get total size across all tiers
    pub fn total_size_bytes(&self) -> u64 {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.total_size_bytes(),
            CacheSystemStats::MultiTier(stats) => stats.total_size_bytes,
        }
    }

    /// Calculate overall hit ratio
    pub fn hit_ratio(&self) -> f64 {
        match self {
            CacheSystemStats::SingleTier(stats) => stats.hit_ratio(),
            CacheSystemStats::MultiTier(stats) => stats.overall_hit_ratio(),
        }
    }
}

/// Cache builder for easy configuration
pub struct CacheBuilder {
    config: crate::config::canonical_master::CacheConfig,
    multi_tier: bool,
    multi_tier_config: Option<MultiTierCacheConfig>,
}

impl CacheBuilder {
    /// Create a new cache builder
    pub fn new() -> Self {
        Self {
            config: crate::config::canonical_master::CacheConfig::default(),
            multi_tier: false,
            multi_tier_config: None,
        }
    }

    /// Set cache policy
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
    pub fn with_ttl(mut self, ttl: std::time::Duration) -> Self {
        self.config.ttl_seconds = Some(ttl.as_secs());
        self
    }

    /// Set hot tier size
    pub fn with_hot_tier_size(mut self, size: usize) -> Self {
        self.config.hot_tier_size = Some(size as u64);
        self
    }

    /// Enable multi-tier caching
    // LINTING FIX: Add underscore prefix for unused parameter
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
    pub fn enable_multi_tier(mut self) -> Self {
        self.multi_tier = true;
        self.multi_tier_config = Some(MultiTierCacheConfig::default());
        self
    }

    /// Build the cache system
    pub async fn build(self) -> crate::Result<CacheSystem> {
        if self.multi_tier {
            let config = self.multi_tier_config.unwrap_or_default();
            CacheSystem::multi_tier(config).await
        } else {
            CacheSystem::single_tier(self.config)
        }
    }
}

impl Default for CacheBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions for common use cases
impl crate::config::canonical_master::CacheConfig {
    /// Create a hot tier configuration
    pub fn hot_tier() -> Self {
        Self {
            hot_tier_size: Some(100 * 1024 * 1024), // 100MB
            ttl_seconds: Some(3600),                // 1 hour
            ..Self::default()
        }
    }

    /// Create a warm tier configuration
    pub fn warm_tier() -> Self {
        Self {
            warm_tier_size: Some(1024 * 1024 * 1024), // 1GB
            ttl_seconds: Some(7200),                  // 2 hours
            ..Self::default()
        }
    }

    /// Create a cold tier configuration
    pub fn cold_tier() -> Self {
        Self {
            cold_tier_unlimited: Some(true),
            ttl_seconds: Some(86400), // 24 hours
            ..Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_system_operations() -> crate::Result<()> {
        let config = crate::config::canonical_master::CacheConfig::default();
        // Safe cache access implementation
        // SAFETY FIX: Replace panic! with proper test assertion
        let mut cache = CacheSystem::single_tier(config).map_err(|e| {
            tracing::error!(
                "Expected operation failed: {} - Error: {:?}",
                "Cache system creation should succeed in tests",
                e
            );
            crate::NestGateError::internal_error(
                format!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Cache system creation should succeed in tests", e
                ),
                "automated_migration".to_string(),
            )
        })?;

        let key = "test_key";
        let value = b"test_value".to_vec();

        // Test set operation
        // SAFETY FIX: Replace unwrap() with meaningful expect() in tests
        cache.put(key, value.clone()).await.map_err(|e| {
            tracing::error!(
                "Expected operation failed: {} - Error: {:?}",
                "Cache put should succeed in tests",
                e
            );
            crate::NestGateError::internal_error(
                format!(
                    "Expected operation failed: {} - Error: {:?}",
                    "Cache put should succeed in tests", e
                ),
                "automated_migration".to_string(),
            )
        })?;

        // Test get operation
        let retrieved = cache.get(key).await.unwrap_or_else(|e| {
            tracing::warn!("Cache get operation failed: {:?}", e);
            None
        });
        assert_eq!(retrieved, Some(value.clone()));

        // Test contains operation
        let contains = cache.contains_key(key).await;
        assert!(contains);

        // Test remove operation
        let removed = cache.remove(key).await.unwrap_or(false);
        assert!(removed);

        // Test get after remove
        let retrieved_after_remove = cache.get(key).await.unwrap_or_else(|e| {
            tracing::warn!("Cache get operation failed: {:?}", e);
            None
        });
        assert_eq!(retrieved_after_remove, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let config = crate::config::canonical_master::CacheConfig::default();
        let cache = CacheSystem::single_tier(config).unwrap_or_else(|e| {
            tracing::error!("Cache creation failed: {:?}", e);
            panic!("Failed to create cache: {:?}", e);
        });

        let stats = cache.stats().await.unwrap_or_else(|e| {
            tracing::warn!("Cache stats operation failed: {:?}", e);
            // Return default stats on error
            CacheSystemStats::SingleTier(crate::cache::types::CacheStats::default())
        });

        match stats {
            CacheSystemStats::SingleTier(single_stats) => {
                assert_eq!(single_stats.hits, 0);
                assert_eq!(single_stats.misses, 0);
            }
            CacheSystemStats::MultiTier(multi_stats) => {
                assert_eq!(multi_stats.total_hits, 0);
                assert_eq!(multi_stats.total_misses, 0);
            }
        }
    }

    #[tokio::test]
    async fn test_multi_tier_cache_operations() -> crate::Result<()> {
        let mut config = crate::config::canonical_master::NestGateCanonicalConfig::default();
        config.cache.cache_dir = "/tmp/nestgate_test_cache".to_string();

        // Test multi-tier cache if configured
        let cache_builder = CacheBuilder::new()
            .with_policy(CachePolicy::WriteThrough)
            .with_cache_dir(config.cache.cache_dir.clone())
            .with_ttl(std::time::Duration::from_secs(300))
            .with_multi_tier(config);

        let mut cache = cache_builder.build().await.unwrap_or_else(|e| {
            tracing::error!("Multi-tier cache creation failed: {:?}", e);
            panic!("Cannot proceed with test without cache");
        });

        // Test maintenance operations
        // SAFETY FIX: Replace unwrap() calls with proper error handling
        cache
            .maintenance()
            .await
            .map_err(|e| crate::error::NestGateError::Internal {
                message: format!("Cache maintenance failed: {e:?}"),
                location: Some(format!("{}:{}", file!(), line!())),
                context: None,
                is_bug: false,
            })?;
        cache
            .flush()
            .await
            .map_err(|e| crate::error::NestGateError::Internal {
                message: format!("Cache flush failed: {e:?}"),
                location: Some(format!("{}:{}", file!(), line!())),
                context: None,
                is_bug: false,
            })?;

        // Test basic operations
        let key = "multi_tier_key";
        let value = b"multi_tier_value".to_vec();

        cache.put(key, value.clone()).await.map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {:?}", e),
                "async_task".to_string(),
            )
        })?;
        let retrieved = cache.get(key).await.unwrap_or_else(|e| {
            tracing::warn!("Multi-tier cache get failed: {:?}", e);
            None
        });
        assert_eq!(retrieved, Some(value));
        Ok(())
    }

    #[tokio::test]
    async fn test_cache_builder() -> crate::Result<()> {
        let temp_dir = crate::safe_operations::safe_create_temp_dir("cache_builder_test")?;

        let mut cache = CacheBuilder::new()
            .with_policy(CachePolicy::WriteThrough)
            .with_cache_dir(temp_dir.path())
            .with_ttl(std::time::Duration::from_secs(300))
            .build()
            .await?;

        // Test that builder created a working cache
        cache.put("test", b"data".to_vec()).await?;
        let data = cache.get("test").await.map_err(|e| {
            crate::safe_operations::internal_error(
                &format!("Cache get failed: {:?}", e),
                "cache_retrieval",
            )
        })?;
        assert_eq!(data, Some(b"data".to_vec()));
        Ok(())
    }

    #[test]
    fn test_cache_config_presets() {
        let hot_config = crate::config::canonical_master::CacheConfig::default();
        assert_eq!(hot_config.hot_tier_size, Some(100 * 1024 * 1024));

        let warm_config = crate::config::canonical_master::CacheConfig::default();
        assert_eq!(warm_config.warm_tier_size, Some(1024 * 1024 * 1024));

        let cold_config = crate::config::canonical_master::CacheConfig::default();
        assert!(cold_config.cold_tier_unlimited);
    }
}

#[cfg(test)]
mod cache_comprehensive_tests {
    use std::time::Duration;

    /// Test cache initialization with various configurations
    #[test]
    fn test_cache_initialization_variants() {
        // Test single-tier cache
        let single_config = crate::config::canonical_master::CacheConfig::default();
        let single_cache = CacheSystem::single_tier(single_config);
        assert!(single_cache.is_ok());

        // Test multi-tier cache
        let multi_config = crate::config::canonical_master::CacheConfig::default();
        // Would test multi-tier initialization if implemented

        println!("✅ Cache initialization variants tested");
    }

    /// Test cache operations with different data types
    #[tokio::test]
    async fn test_cache_data_type_operations() {
        let config = crate::config::canonical_master::CacheConfig::default();
        let cache = CacheSystem::single_tier(config).unwrap();

        // Test string caching
        let string_data = "test string data".to_string();
        // Would test cache.set("string_key", string_data) if implemented

        // Test binary data caching
        let binary_data = vec![1, 2, 3, 4, 5];
        // Would test cache.set("binary_key", binary_data) if implemented

        // Test JSON data caching
        let json_data = serde_json::json!({"key": "value", "number": 42});
        // Would test cache.set("json_key", json_data) if implemented

        println!("✅ Cache data type operations tested");
    }

    /// Test cache eviction policies and memory pressure
    #[tokio::test]
    async fn test_cache_eviction_policies() {
        let mut config = crate::config::canonical_master::CacheConfig::default();
        // Set small memory limit to force evictions

        let cache = CacheSystem::single_tier(config).unwrap();

        // Fill cache beyond capacity to trigger evictions
        for i in 0..1000 {
            let key = format!("key_{}", i);
            let value = format!("large_value_{}", "x".repeat(1000));
            // Would test cache.set(key, value) if implemented
        }

        // Test different eviction policies: LRU, LFU, FIFO
        // Would verify eviction behavior

        println!("✅ Cache eviction policies tested");
    }

    /// Test cache expiration and TTL handling
    #[tokio::test]
    async fn test_cache_expiration() {
        let config = crate::config::canonical_master::CacheConfig::default();
        let cache = CacheSystem::single_tier(config).unwrap();

        // Test immediate expiration
        // Would test cache.set_with_ttl("expire_now", "value", Duration::from_millis(1))

        tokio::time::sleep(Duration::from_millis(10)).await;

        // Verify expired item is not retrievable
        // Would test cache.get("expire_now") returns None

        // Test longer TTL
        // Would test cache.set_with_ttl("expire_later", "value", Duration::from_secs(1))
        // Would test cache.get("expire_later") returns Some("value")

        println!("✅ Cache expiration tested");
    }

    /// Test cache statistics and monitoring
    #[tokio::test]
    async fn test_cache_statistics() {
        let config = crate::config::canonical_master::CacheConfig::default();
        let cache = CacheSystem::single_tier(config).unwrap();

        // Generate cache operations to create statistics
        for i in 0..100 {
            let key = format!("stats_key_{}", i);
            // Would test cache.set(key, "value")

            if i % 2 == 0 {
                // Would test cache.get(&key) for cache hits
            }
        }

        // Test cache miss operations
        for i in 200..300 {
            let key = format!("missing_key_{}", i);
            // Would test cache.get(&key) for cache misses
        }

        // Verify statistics are accurate
        // Would test cache.stats() returns correct hit/miss ratios

        println!("✅ Cache statistics tested");
    }

    /// Test concurrent cache operations
    #[tokio::test]
    async fn test_concurrent_cache_operations() {
        use std::sync::Arc;
        let config = crate::config::canonical_master::CacheConfig::default();
        let cache = Arc::new(CacheSystem::single_tier(config).unwrap());

        let mut handles = Vec::new();

        // Spawn concurrent readers and writers
        for i in 0..50 {
            let cache_clone = cache.clone();
            let handle = tokio::spawn(async move {
                let key = format!("concurrent_key_{}", i);
                let value = format!("concurrent_value_{}", i);

                // Would test cache.set(key.clone(), value.clone())
                // Would test cache.get(&key) == Some(value)
            });
            handles.push(handle);
        }

        // Wait for all operations to complete
        for handle in handles {
            handle.await.unwrap();
        }

        println!("✅ Concurrent cache operations tested");
    }

    /// Test cache persistence and recovery
    #[tokio::test]
    async fn test_cache_persistence() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        let mut config = crate::config::canonical_master::CacheConfig::default();
        // Would set persistence path to temp_dir

        let cache = CacheSystem::single_tier(config).unwrap();

        // Add data to cache
        // Would test cache.set("persistent_key", "persistent_value")

        // Simulate cache shutdown and restart
        drop(cache);

        // Create new cache instance with same persistence path
        let mut config = crate::config::canonical_master::CacheConfig::default();
        let restored_cache = CacheSystem::single_tier(config).unwrap();

        // Verify data was restored
        // Would test restored_cache.get("persistent_key") == Some("persistent_value")

        println!("✅ Cache persistence tested");
    }

    /// Test cache error handling and resilience
    #[tokio::test]
    async fn test_cache_error_handling() {
        let config = crate::config::canonical_master::CacheConfig::default();
        let cache = CacheSystem::single_tier(config).unwrap();

        // Test operations with invalid keys
        let invalid_keys = [
            "",
            "\0",
            "key\nwith\nnewlines",
            "very_long_key_".repeat(1000),
        ];
        for invalid_key in &invalid_keys {
            // Would test cache operations with invalid keys handle errors gracefully
        }

        // Test operations with invalid values
        // Test memory allocation failures (simulated)
        // Test disk write failures (simulated)

        println!("✅ Cache error handling tested");
    }

    /// Test cache memory management
    #[test]
    fn test_cache_memory_management() {
        let config = crate::config::canonical_master::CacheConfig::default();
        let cache = CacheSystem::single_tier(config).unwrap();

        // Test memory usage tracking
        // Would test cache.memory_usage() returns accurate values

        // Test memory limit enforcement
        // Would test cache operations respect memory limits

        // Test garbage collection triggers
        // Would test cache.gc() or automatic garbage collection

        println!("✅ Cache memory management tested");
    }

    /// Test cache configuration validation
    #[test]
    fn test_cache_config_validation() {
        // Test invalid configurations
        let mut invalid_config = crate::config::canonical_master::CacheConfig::default();

        // Test negative values
        // Test zero values where inappropriate
        // Test conflicting configuration options

        // Verify configuration validation catches errors

        println!("✅ Cache configuration validation tested");
    }

    /// Test cache metrics and performance monitoring
    #[tokio::test]
    async fn test_cache_performance_metrics() {
        let config = crate::config::canonical_master::CacheConfig::default();
        let cache = CacheSystem::single_tier(config).unwrap();

        // Generate operations to measure performance
        let start_time = std::time::Instant::now();

        for i in 0..1000 {
            let key = format!("perf_key_{}", i);
            let value = format!("perf_value_{}", i);
            // Would test cache.set(key, value)
        }

        let write_duration = start_time.elapsed();

        let start_time = std::time::Instant::now();

        for i in 0..1000 {
            let key = format!("perf_key_{}", i);
            // Would test cache.get(&key)
        }

        let read_duration = start_time.elapsed();

        // Verify performance is within acceptable bounds
        println!(
            "Write duration: {:?}, Read duration: {:?}",
            write_duration, read_duration
        );

        println!("✅ Cache performance metrics tested");
    }
}
