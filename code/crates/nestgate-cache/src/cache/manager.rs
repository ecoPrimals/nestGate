// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Cache management system
// Provides multi-tier caching with TTL and eviction policies

//! Manager module

use std::collections::HashMap;
// CLEANED: Removed unused imports as part of canonical modernization
// use std::sync::Arc;
use std::time::{Duration, SystemTime};
// use tokio::sync::RwLock;
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::UnifiedCacheConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::UnifiedCacheConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `UnifiedCache`
pub struct UnifiedCacheConfig {
    /// Size of max
    pub max_size: usize,
    /// Ttl Seconds
    pub ttl_seconds: Option<u64>,
    /// Cache Dir
    pub cache_dir: Option<std::path::PathBuf>,
    /// Eviction Policy
    pub eviction_policy: String,
}
#[allow(deprecated)]
impl Default for UnifiedCacheConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_size: 1000,
            ttl_seconds: Some(3600), // 1 hour
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        }
    }
}

/// Cache entry with metadata
#[derive(Debug, Clone)]
/// Cacheentry
pub struct CacheEntry {
    /// Data
    pub data: Vec<u8>,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Last Accessed
    pub last_accessed: SystemTime,
    /// Count of access
    pub access_count: u64,
}
impl CacheEntry {
    /// Create a new cache entry with the given data
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        let now = SystemTime::now();
        Self {
            data,
            created_at: now,
            last_accessed: now,
            access_count: 1,
        }
    }

    /// Check if this cache entry has expired based on the given TTL
    #[must_use]
    pub fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed().unwrap_or(Duration::ZERO) > ttl
    }

    /// Record an access to this cache entry
    pub fn access(&mut self) {
        self.last_accessed = SystemTime::now();
        self.access_count += 1;
    }
}

/// Cache statistics
#[derive(Debug, Default, Clone)]
/// Cachestats
pub struct CacheStats {
    /// Hits
    pub hits: u64,
    /// Misses
    pub misses: u64,
    /// Evictions
    pub evictions: u64,
    /// Size
    pub size: usize,
}
impl CacheStats {
    /// Calculate the cache hit rate as a percentage (0.0 to 1.0)
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            self.hits as f64 / (self.hits + self.misses) as f64
        }
    }
}

/// Multi-tier cache manager
pub struct CacheManager {
    hot_tier: HashMap<String, CacheEntry>,
    warm_tier: HashMap<String, CacheEntry>,
    cold_tier: HashMap<String, CacheEntry>,
    #[allow(deprecated)]
    config: UnifiedCacheConfig,
    stats: CacheStats,
}
impl CacheManager {
    /// Create new cache manager with configuration
    #[must_use]
    #[allow(deprecated)]
    pub fn new(config: UnifiedCacheConfig) -> Self {
        Self {
            hot_tier: HashMap::new(),
            warm_tier: HashMap::new(),
            cold_tier: HashMap::new(),
            config,
            stats: CacheStats::default(),
        }
    }

    /// Get value from cache
    #[must_use]
    pub fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        // Check hot tier first
        if let Some(entry) = self.hot_tier.get_mut(key) {
            entry.access();
            self.stats.hits += 1;
            debug!("Cache hit in hot tier for key: {}", key);
            return Some(entry.data.clone());
        }

        // Check warm tier
        if let Some(mut entry) = self.warm_tier.remove(key) {
            entry.access();
            let data = entry.data.clone();
            // Promote to hot tier
            self.hot_tier.insert(key.to_string(), entry);
            self.stats.hits += 1;
            debug!("Cache hit in warm tier for key: {}", key);
            return Some(data);
        }

        // Check cold tier
        if let Some(mut entry) = self.cold_tier.remove(key) {
            entry.access();
            let data = entry.data.clone();
            // Promote to warm tier
            self.warm_tier.insert(key.to_string(), entry);
            self.stats.hits += 1;
            debug!("Cache hit in cold tier for key: {}", key);
            return Some(data);
        }

        // Not found
        self.stats.misses += 1;
        debug!("Cache miss for key: {}", key);
        None
    }

    /// Put value in cache
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn put(&mut self, key: String, data: Vec<u8>) -> Result<()> {
        let entry = CacheEntry::new(data);

        // Always insert into hot tier
        self.hot_tier.insert(key.clone(), entry);
        self.stats.size += 1;

        // Check if we need to evict
        self.evict_if_needed().await?;

        debug!("Cached entry for key: {}", key);
        Ok(())
    }

    /// Remove entry from cache
    pub fn remove(&mut self, key: &str) -> bool {
        let removed = self.hot_tier.remove(key).is_some()
            || self.warm_tier.remove(key).is_some()
            || self.cold_tier.remove(key).is_some();

        if removed {
            self.stats.size = self.stats.size.saturating_sub(1);
            debug!("Removed cache entry for key: {}", key);
        }

        removed
    }

    /// Clear all cache entries
    pub fn clear(&mut self) {
        self.hot_tier.clear();
        self.warm_tier.clear();
        self.cold_tier.clear();
        self.stats = CacheStats::default();
        debug!("Cache cleared");
    }

    /// Get cache statistics
    #[must_use]
    pub const fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Perform cache maintenance
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn maintenance(&mut self) -> Result<()> {
        self.expire_entries()?;
        self.evict_if_needed().await?;
        debug!("Cache maintenance completed");
        Ok(())
    }

    /// Expire old entries based on TTL
    fn expire_entries(&mut self) -> Result<()> {
        if let Some(ttl_seconds) = self.config.ttl_seconds {
            let ttl = Duration::from_secs(ttl_seconds);

            // Expire from all tiers - process each tier separately to avoid borrowing conflicts
            self.expire_hot_tier(ttl);
            self.expire_warm_tier(ttl);
            self.expire_cold_tier(ttl);
        }
        Ok(())
    }

    /// Expire entries from hot tier
    fn expire_hot_tier(&mut self, ttl: Duration) {
        let keys_to_remove: Vec<String> = self
            .hot_tier
            .iter()
            .filter(|(_, entry)| entry.is_expired(ttl))
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            self.hot_tier.remove(&key);
            self.stats.evictions += 1;
            self.stats.size = self.stats.size.saturating_sub(1);
            debug!("Expired hot tier cache entry: {}", key);
        }
    }

    /// Expire entries from warm tier
    fn expire_warm_tier(&mut self, ttl: Duration) {
        let keys_to_remove: Vec<String> = self
            .warm_tier
            .iter()
            .filter(|(_, entry)| entry.is_expired(ttl))
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            self.warm_tier.remove(&key);
            self.stats.evictions += 1;
            self.stats.size = self.stats.size.saturating_sub(1);
            debug!("Expired warm tier cache entry: {}", key);
        }
    }

    /// Expire entries from cold tier
    fn expire_cold_tier(&mut self, ttl: Duration) {
        let keys_to_remove: Vec<String> = self
            .cold_tier
            .iter()
            .filter(|(_, entry)| entry.is_expired(ttl))
            .map(|(key, _)| key.clone())
            .collect();

        for key in keys_to_remove {
            self.cold_tier.remove(&key);
            self.stats.evictions += 1;
            self.stats.size = self.stats.size.saturating_sub(1);
            debug!("Expired cold tier cache entry: {}", key);
        }
    }

    /// Evict entries if cache is over capacity
    async fn evict_if_needed(&mut self) -> Result<()> {
        while self.total_size() > self.config.max_size {
            self.evict_one_entry()?;
        }
        Ok(())
    }

    /// Get total cache size across all tiers
    fn total_size(&self) -> usize {
        self.hot_tier.len() + self.warm_tier.len() + self.cold_tier.len()
    }

    /// Evict one entry using LRU policy
    fn evict_one_entry(&mut self) -> Result<()> {
        // Try to evict from cold tier first
        if !self.cold_tier.is_empty()
            && let Some(key) = self.find_lru_key(&self.cold_tier)
        {
            self.cold_tier.remove(&key);
            self.stats.evictions += 1;
            self.stats.size = self.stats.size.saturating_sub(1);
            debug!("Evicted from cold tier: {}", key);
            return Ok(());
        }

        // Then warm tier
        if !self.warm_tier.is_empty()
            && let Some(key) = self.find_lru_key(&self.warm_tier)
        {
            self.warm_tier.remove(&key);
            self.stats.evictions += 1;
            self.stats.size = self.stats.size.saturating_sub(1);
            debug!("Evicted from warm tier: {}", key);
            return Ok(());
        }

        // Finally hot tier
        if !self.hot_tier.is_empty()
            && let Some(key) = self.find_lru_key(&self.hot_tier)
        {
            self.hot_tier.remove(&key);
            self.stats.evictions += 1;
            self.stats.size = self.stats.size.saturating_sub(1);
            debug!("Evicted from hot tier: {}", key);
            return Ok(());
        }

        Ok(())
    }

    /// Find least recently used key in a tier
    fn find_lru_key(&self, tier: &HashMap<String, CacheEntry>) -> Option<String> {
        tier.iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(key, _)| key.clone())
    }

    /// Get cache path for a key
    #[allow(dead_code)] // Framework method - intentionally unused
    fn get_cache_path(&self, key: &str) -> std::path::PathBuf {
        let filename = format!("{key}.cache");
        if let Some(cache_dir) = &self.config.cache_dir {
            cache_dir.join(filename)
        } else {
            std::env::temp_dir().join(filename)
        }
    }

    /// Load cache data from disk
    #[allow(dead_code)] // Framework method - intentionally unused
    async fn load_from_disk(&self, path: &std::path::Path) -> Result<Vec<u8>> {
        tokio::fs::read(path)
            .await
            .map_err(|e| NestGateError::io_error(e.to_string()))
    }

    /// Save cache data to disk
    #[allow(dead_code)] // Framework method - intentionally unused
    async fn save_to_disk(&self, key: &str, data: &[u8]) -> Result<()> {
        let cache_path = self.get_cache_path(key);

        if let Some(parent) = cache_path.parent() {
            let _ = tokio::fs::create_dir_all(parent).await;
        }

        tokio::fs::write(&cache_path, data)
            .await
            .map_err(|e| NestGateError::io_error(e.to_string()))
    }

    /// Reset cache statistics
    pub fn reset_stats(&mut self) {
        self.stats = CacheStats::default();
        debug!("Cache statistics reset");
    }

    /// Flush all cache data
    pub fn flush(&mut self) -> nestgate_types::Result<()> {
        self.hot_tier.clear();
        self.warm_tier.clear();
        self.cold_tier.clear();
        self.stats = CacheStats::default();
        debug!("Cache flushed successfully");
        Ok(())
    }
}

impl Default for CacheManager {
    /// Returns the default instance
    fn default() -> Self {
        let config = UnifiedCacheConfig::default();
        Self {
            hot_tier: HashMap::new(),
            warm_tier: HashMap::new(),
            cold_tier: HashMap::new(),
            config,
            stats: CacheStats::default(),
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
/// Type alias for Unifiedcacheconfigcanonical
pub type UnifiedCacheConfigCanonical =
    nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using UnifiedCacheConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let mut cache = CacheManager::default();

        // Test put and get
        cache
            .put("test".to_string(), b"data".to_vec())
            .await
            .expect("Cache operation failed");
        let result = cache.get("test");
        assert_eq!(result, Some(b"data".to_vec()));

        // Test stats
        assert_eq!(cache.stats().hits, 1);
        assert_eq!(cache.stats().misses, 0);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let mut cache = CacheManager::default();
        let result = cache.get("nonexistent");
        assert_eq!(result, None);
        assert_eq!(cache.stats().misses, 1);
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let config = UnifiedCacheConfig {
            max_size: 2,
            ttl_seconds: Some(3600),
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };

        let mut cache = CacheManager::new(config);

        // Fill cache beyond capacity
        cache
            .put("key1".to_string(), b"data1".to_vec())
            .await
            .expect("Cache operation failed");
        cache
            .put("key2".to_string(), b"data2".to_vec())
            .await
            .expect("Cache operation failed");
        cache
            .put("key3".to_string(), b"data3".to_vec())
            .await
            .expect("Cache operation failed");

        // Should have evicted oldest entry
        assert!(cache.total_size() <= 2);
        assert!(cache.stats().evictions > 0);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let config = UnifiedCacheConfig {
            max_size: 1000,
            ttl_seconds: Some(0), // Expire immediately
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };

        let mut cache = CacheManager::new(config);
        cache
            .put("test".to_string(), b"data".to_vec())
            .await
            .expect("Cache operation failed");

        cache
            .maintenance()
            .await
            .expect("test: maintenance with zero TTL should succeed");

        assert_eq!(cache.get("test"), None);
    }

    #[tokio::test]
    async fn test_cache_remove_and_clear() {
        let mut cache = CacheManager::default();
        cache
            .put("a".to_string(), vec![1])
            .await
            .expect("test: put a");
        cache
            .put("b".to_string(), vec![2])
            .await
            .expect("test: put b");
        assert!(cache.remove("a"));
        assert!(!cache.remove("a"));
        cache.clear();
        assert_eq!(cache.stats().size, 0);
        assert_eq!(cache.get("b"), None);
    }

    #[tokio::test]
    async fn test_cache_flush_and_reset_stats() {
        let mut cache = CacheManager::default();
        cache
            .put("k".to_string(), vec![9])
            .await
            .expect("test: put");
        let _ = cache.get("k");
        cache.flush().expect("test: flush");
        assert_eq!(cache.stats().hits, 0);
        cache
            .put("k2".to_string(), vec![1])
            .await
            .expect("test: put after flush");
        let _ = cache.get("k2");
        assert_eq!(cache.stats().hits, 1);
        cache.reset_stats();
        assert_eq!(cache.stats().hits, 0);
        assert_eq!(cache.stats().misses, 0);
    }

    #[test]
    fn test_cache_stats_hit_rate() {
        let mut s = CacheStats::default();
        assert_eq!(s.hit_rate(), 0.0);
        s.hits = 3;
        s.misses = 1;
        assert!((s.hit_rate() - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cache_entry_access_and_expiry() {
        let mut e = CacheEntry::new(vec![0u8; 4]);
        assert_eq!(e.access_count, 1);
        e.access();
        assert_eq!(e.access_count, 2);
        assert!(!e.is_expired(Duration::from_secs(3600)));
    }

    #[tokio::test]
    async fn test_cache_warm_tier_promotion() {
        let config = UnifiedCacheConfig {
            max_size: 10,
            ttl_seconds: Some(3600),
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };
        let mut cache = CacheManager::new(config);
        cache
            .put("warm_key".to_string(), b"payload".to_vec())
            .await
            .expect("test: put");
        let warm_entry = cache.hot_tier.remove("warm_key").expect("test: take hot");
        cache.warm_tier.insert("warm_key".to_string(), warm_entry);
        let data = cache.get("warm_key").expect("test: promote warm to hot");
        assert_eq!(data, b"payload");
    }

    #[tokio::test]
    async fn test_cache_cold_tier_promotion() {
        let config = UnifiedCacheConfig {
            max_size: 10,
            ttl_seconds: Some(3600),
            cache_dir: None,
            eviction_policy: "lru".to_string(),
        };
        let mut cache = CacheManager::new(config);
        cache
            .put("cold_key".to_string(), b"c".to_vec())
            .await
            .expect("test: put");
        let entry = cache.hot_tier.remove("cold_key").expect("test: take hot");
        cache.cold_tier.insert("cold_key".to_string(), entry);
        let data = cache.get("cold_key").expect("test: cold to warm");
        assert_eq!(data, b"c");
        let back = cache.get("cold_key").expect("test: warm to hot");
        assert_eq!(back, b"c");
    }
}
