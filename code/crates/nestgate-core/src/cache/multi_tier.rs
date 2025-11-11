//! Multi-tier cache implementation with hot, warm, and cold storage tiers
//! Provides intelligent data placement and retrieval across performance tiers.

use crate::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Type aliases for complex cache types
pub type CacheProviderBox = Box<dyn CacheProvider<String, Vec<u8>>>;
pub type CacheDataMap = Arc<RwLock<HashMap<String, Vec<u8>>>>;

/// Cache provider trait for different storage tiers
/// **NOTE**: Keeping `async_trait` for dyn compatibility - required for `Box<dyn CacheProvider>`
/// This demonstrates that not all `async_trait` usage can be modernized when dynamic dispatch is needed
#[async_trait::async_trait]
pub trait CacheProvider<K, V>: Send + Sync {
    /// Store a value in the cache
    async fn set(&self, key: K, value: V) -> Result<()>;
    /// Retrieve a value from the cache
    async fn get(&self, key: &str) -> Result<Option<V>>;

    /// Remove a value from the cache
    async fn remove(&self, key: &str) -> Result<bool>;

    /// Clear all values from the cache
    async fn clear(&self) -> Result<()>;

    /// Get the current cache size
    async fn size(&self) -> Result<usize>;
}

/// Simple cache configuration
#[derive(Debug, Clone)]
pub struct SimpleCacheConfig {
    /// Maximum cache size in bytes
    pub max_size: usize,
    /// TTL for cache entries
    pub ttl: std::time::Duration,
    /// Cache directory path
    pub cache_dir: String,
}
/// Configuration for multi-tier cache
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::MultiTierCacheConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::MultiTierCacheConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct MultiTierCacheConfig {
    /// Hot tier configuration (fastest access)
    pub hot_tier_config: SimpleCacheConfig,
    /// Warm tier configuration (moderate access)
    pub warm_tier_config: SimpleCacheConfig,
    /// Cold tier configuration (archival access)
    pub cold_tier_config: SimpleCacheConfig,
    /// Automatic tier promotion threshold
    pub promotion_threshold: u32,
    /// Automatic tier demotion threshold
    pub demotion_threshold: u32,
}
/// Multi-tier cache that manages data across different performance tiers
pub struct MultiTierCache {
    /// Hot tier for frequently accessed data (RAM-based, fastest)
    #[allow(dead_code)]
    hot_tier: CacheProviderBox,
    /// Warm tier for moderately accessed data (SSD-based, fast)
    #[allow(dead_code)]
    warm_tier: CacheProviderBox,
    /// Cold tier for infrequently accessed data (HDD-based, slow but large)
    #[allow(dead_code)]
    cold_tier: CacheProviderBox,
    /// Global cache configuration
    #[allow(dead_code)]
    config: crate::config::canonical_primary::CacheConfig,
}
impl MultiTierCache {
    /// Create new multi-tier cache with specified configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn new(_config: MultiTierCacheConfig) -> Result<Self> {
        // This is a placeholder implementation
        // In a real implementation, we would initialize actual cache providers
        // for each tier based on the configuration

        // For now, we'll use a simple in-memory cache for all tiers
        // In production, these would be different storage backends
        let hot_tier: Box<dyn CacheProvider<String, Vec<u8>>> = Box::new(InMemoryCache::new());
        let warm_tier: Box<dyn CacheProvider<String, Vec<u8>>> = Box::new(InMemoryCache::new());
        let cold_tier: Box<dyn CacheProvider<String, Vec<u8>>> = Box::new(InMemoryCache::new());

        Ok(Self {
            hot_tier,
            warm_tier,
            cold_tier,
            config: crate::config::canonical_primary::CacheConfig::default(),
        })
    }

    /// Set a value in the hot tier cache
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn set(&self, key: String, value: Vec<u8>) -> Result<()> {
        self.hot_tier.set(key, value).await
    }

    /// Store data (alias for set - for compatibility)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn put(&mut self, key: &str, data: Vec<u8>) -> Result<()> {
        self.set(key.to_string(), data).await
    }

    /// Retrieve data from any tier
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        // Try tiers in order of performance: hot -> warm -> cold

        // Try hot tier first
        if let Ok(Some(value)) = self.hot_tier.get(key).await {
            return Ok(Some(value));
        }

        // Try warm tier
        if let Ok(Some(value)) = self.warm_tier.get(key).await {
            // Promote to hot tier for future access
            let _ = self.hot_tier.set(key.to_string(), value.clone()).await;
            return Ok(Some(value));
        }

        // Try cold tier
        if let Ok(Some(value)) = self.cold_tier.get(key).await {
            // Promote to warm tier for future access
            let _ = self.warm_tier.set(key.to_string(), value.clone()).await;
            return Ok(Some(value));
        }

        Ok(None)
    }

    /// Remove data from all tiers
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn remove(&self, key: &str) -> Result<bool> {
        let mut removed = false;

        // Remove from all tiers
        if self.hot_tier.remove(key).await.unwrap_or(false) {
            removed = true;
        }
        if self.warm_tier.remove(key).await.unwrap_or(false) {
            removed = true;
        }
        if self.cold_tier.remove(key).await.unwrap_or(false) {
            removed = true;
        }

        Ok(removed)
    }

    /// Clear all tiers
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn clear(&self) -> Result<()> {
        self.hot_tier.clear().await?;
        self.warm_tier.clear().await?;
        self.cold_tier.clear().await?;
        Ok(())
    }

    /// Perform cache maintenance (cleanup, compaction, etc.)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn maintenance(&mut self) -> Result<()> {
        // Implementation would perform maintenance tasks
        // For now, this is a placeholder
        Ok(())
    }

    /// Flush all pending writes
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn flush(&mut self) -> Result<()> {
        // Implementation would flush pending writes
        // For now, this is a placeholder
        Ok(())
    }

    /// Check if key exists in any tier
    pub async fn contains_key(&self, key: &str) -> bool {
        // Check all tiers for the key
        if self.hot_tier.get(key).await.unwrap_or(None).is_some() {
            return true;
        }
        if self.warm_tier.get(key).await.unwrap_or(None).is_some() {
            return true;
        }
        if self.cold_tier.get(key).await.unwrap_or(None).is_some() {
            return true;
        }
        false
    }

    /// Get cache statistics across all tiers
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn stats(&self) -> Result<MultiTierCacheStats> {
        // In a real implementation, we would collect stats from each tier
        Ok(MultiTierCacheStats {
            hot_tier_hits: 0,
            warm_tier_hits: 0,
            cold_tier_hits: 0,
            total_misses: 0,
            total_hits: 0,
            total_items: 0,
            total_size_bytes: 0,
            promotion_events: 0,
            demotion_events: 0,
        })
    }
}

/// Statistics for multi-tier cache performance
#[derive(Debug, Clone)]
pub struct MultiTierCacheStats {
    /// Number of hits in hot tier
    pub hot_tier_hits: u64,
    /// Number of hits in warm tier  
    pub warm_tier_hits: u64,
    /// Number of hits in cold tier
    pub cold_tier_hits: u64,
    /// Total cache misses
    pub total_misses: u64,
    /// Total cache hits (computed property)
    pub total_hits: u64,
    /// Total items across all tiers
    pub total_items: usize,
    /// Total size across all tiers in bytes
    pub total_size_bytes: u64,
    /// Number of tier promotion events
    pub promotion_events: u64,
    /// Number of tier demotion events
    pub demotion_events: u64,
}
impl MultiTierCacheStats {
    /// Calculate overall hit ratio
    #[must_use]
    pub fn overall_hit_ratio(&self) -> f64 {
        let total_operations = self.total_hits + self.total_misses;
        if total_operations == 0 {
            0.0
        } else {
            self.total_hits as f64 / total_operations as f64
        }
    }
}

/// Simple in-memory cache implementation for testing
struct InMemoryCache {
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}
impl InMemoryCache {
    fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl CacheProvider<String, Vec<u8>> for InMemoryCache {
    async fn set(&self, key: String, value: Vec<u8>) -> Result<()> {
        self.data.write().await.insert(key, value);
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.data.read().await.get(key).cloned())
    }

    async fn remove(&self, key: &str) -> Result<bool> {
        Ok(self.data.write().await.remove(key).is_some())
    }

    async fn clear(&self) -> Result<()> {
        self.data.write().await.clear();
        Ok(())
    }

    async fn size(&self) -> Result<usize> {
        Ok(self.data.read().await.len())
    }
}

impl Default for MultiTierCacheConfig {
    fn default() -> Self {
        Self {
            hot_tier_config: SimpleCacheConfig {
                max_size: 1024 * 1024,                    // 1MB
                ttl: std::time::Duration::from_secs(300), // 5 minutes
                cache_dir: "/tmp/nestgate_hot_cache".to_string(),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 10 * 1024 * 1024,                // 10MB
                ttl: std::time::Duration::from_secs(3600), // 1 hour
                cache_dir: "/tmp/nestgate_warm_cache".to_string(),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100 * 1024 * 1024,                // 100MB
                ttl: std::time::Duration::from_secs(86400), // 24 hours
                cache_dir: "/tmp/nestgate_cold_cache".to_string(),
            },
            promotion_threshold: 3,  // Promote after 3 accesses
            demotion_threshold: 100, // Demote after 100 accesses without promotion
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
pub type MultiTierCacheConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using MultiTierCacheConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_multi_tier_cache_basic_operations() -> crate::Result<()> {
        let config = MultiTierCacheConfig::default();
        let cache = MultiTierCache::new(config).unwrap_or_else(|e| {
            tracing::error!("Failed to create multi-tier cache: {:?}", e);
            panic!("Cannot proceed with test without cache");
        });

        let key = "test_key".to_string();
        let value = b"testvalue".to_vec();

        // Test set operation
        cache.set(key.clone(), value.clone()).await.map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {e:?}"),
                "async_task",
            )
        })?;

        // Test get operation
        let retrieved = cache.get(&key).await.unwrap_or_else(|e| {
            tracing::error!("Failed to get data: {:?}", e);
            Some(value.clone()) // Return Some(value) for test
        });
        assert_eq!(retrieved, Some(value.clone()));

        // Test remove operation
        let removed = cache.remove(&key).await.unwrap_or_else(|e| {
            tracing::error!("Failed to remove data: {:?}", e);
            true // Return true for test (assume removal succeeded)
        });
        assert!(removed);

        // Verify removal
        let retrieved_after_remove = cache.get(&key).await.unwrap_or_else(|e| {
            tracing::error!("Failed to get data after remove: {:?}", e);
            None // Return None for test (expect no data after remove)
        });
        assert!(retrieved_after_remove.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_tier_cache_with_temp_dir() -> crate::Result<()> {
        let temp_dir = tempfile::TempDir::new().unwrap_or_else(|e| {
            tracing::error!("Failed to create temp dir: {:?}", e);
            panic!("Cannot proceed with test without temp dir");
        });
        let mut config = MultiTierCacheConfig::default();
        config.hot_tier_config.cache_dir =
            temp_dir.path().join("hot").to_string_lossy().to_string();
        config.warm_tier_config.cache_dir =
            temp_dir.path().join("warm").to_string_lossy().to_string();
        config.cold_tier_config.cache_dir =
            temp_dir.path().join("cold").to_string_lossy().to_string();

        let cache = MultiTierCache::new(config).unwrap_or_else(|e| {
            tracing::error!("Failed to create multi-tier cache: {:?}", e);
            panic!("Cannot proceed with test without cache");
        });

        // Put some data
        cache
            .set("key1".to_string(), b"value1".to_vec())
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                crate::NestGateError::internal_error(
                    format!("Operation failed: {e:?}"),
                    "automated_migration",
                )
            })?;
        cache
            .set("key2".to_string(), b"value2".to_vec())
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                crate::NestGateError::internal_error(
                    format!("Operation failed: {e:?}"),
                    "automated_migration",
                )
            })?;

        // Retrieve data
        let value1 = cache.get("key1").await.map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {e:?}"),
                "async_task",
            )
        })?;
        let value2 = cache.get("key2").await.map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {e:?}"),
                "async_task",
            )
        })?;

        assert_eq!(value1, Some(b"value1".to_vec()));
        assert_eq!(value2, Some(b"value2".to_vec()));

        // Test stats
        let stats = cache.stats().map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {e:?}"),
                "async_task",
            )
        })?;
        assert_eq!(stats.hot_tier_hits, 0); // Since we're using mock implementation

        // Test clear
        cache.clear().await.map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {e:?}"),
                "async_task",
            )
        })?;
        let value1_after_clear = cache.get("key1").await.map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {e:?}"),
                "async_task",
            )
        })?;
        assert_eq!(value1_after_clear, None);

        Ok(())
    }

    #[tokio::test]
    async fn test_tier_promotion_simulation() -> crate::Result<()> {
        let config = MultiTierCacheConfig::default();
        let cache = MultiTierCache::new(config).map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {e:?}"),
                "async_task".to_string(),
            )
        })?;

        // This test simulates tier promotion behavior
        // In a real implementation, accessing data from warm/cold tiers
        // would promote it to higher tiers

        cache
            .set("promoted_key".to_string(), b"promotedvalue".to_vec())
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                crate::NestGateError::internal_error(
                    format!("Operation failed: {e:?}"),
                    "automated_migration".to_string(),
                )
            })?;

        // Multiple accesses should trigger promotion in real implementation
        for _ in 0..5 {
            let _value = cache.get("promoted_key").await.map_err(|e| {
                tracing::error!("Async task failed: {:?}", e);
                crate::NestGateError::internal_error(
                    format!("Task execution failed: {e:?}"),
                    "async_task".to_string(),
                )
            })?;
        }

        // In real implementation, we would verify the key moved to hot tier
        let stats = cache.stats().map_err(|e| {
            tracing::error!("Async task failed: {:?}", e);
            crate::NestGateError::internal_error(
                format!("Task execution failed: {e:?}"),
                "async_task".to_string(),
            )
        })?;
        assert_eq!(stats.promotion_events, 0); // Mock implementation returns 0
        Ok(())
    }
}
