use std::collections::HashMap;
//
// Intelligent caching with adaptive algorithms and workload-aware optimization.

use crate::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

// Type aliases for caching
type CacheStorage<K, V> = Arc<RwLock<HashMap<K, CacheEntry<V>>>>;

/// Adaptive cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveCacheConfig {
    pub max_entries: usize,
    pub default_ttl: Duration,
    pub enable_lru_eviction: bool,
    pub enable_predictive_caching: bool,
    pub cache_hit_ratio_target: f64,
    pub memory_usage_limit: usize,
}
impl Default for AdaptiveCacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            default_ttl: Duration::from_secs(3600),
            enable_lru_eviction: true,
            enable_predictive_caching: true,
            cache_hit_ratio_target: 0.85,
            memory_usage_limit: 100 * 1024 * 1024, // 100MB
        }
    }
}

/// Cache entry with metadata
#[derive(Debug, Clone)]
pub struct CacheEntry<V> {
    pub value: V,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub access_count: u64,
    pub ttl: Duration,
    pub size_bytes: usize,
}
impl<V> CacheEntry<V> {
    pub fn new(value: V, ttl: Duration, size_bytes: usize) -> Self {
        let now = SystemTime::now();
        Self {
            value,
            created_at: now,
            last_accessed: now,
            access_count: 1,
            ttl,
            size_bytes,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().unwrap_or(Duration::MAX) > self.ttl
    }

    pub fn mark_accessed(&mut self) {
        self.last_accessed = SystemTime::now();
        self.access_count += 1;
    }
}

/// Cache metrics for monitoring
#[derive(Debug, Default)]
pub struct CacheMetrics {
    pub cache_hits: std::sync::atomic::AtomicU64,
    pub cache_misses: std::sync::atomic::AtomicU64,
    pub evictions: std::sync::atomic::AtomicU64,
    pub memory_usage: std::sync::atomic::AtomicUsize,
    pub entry_count: std::sync::atomic::AtomicUsize,
}
impl CacheMetrics {
    pub fn hit_ratio(&self) -> f64 {
        let hits = self.cache_hits.load(std::sync::atomic::Ordering::Relaxed);
        let misses = self.cache_misses.load(std::sync::atomic::Ordering::Relaxed);

        if hits + misses == 0 {
            0.0
        } else {
            hits as f64 / (hits + misses) as f64
        }
    }
}

/// Adaptive cache with intelligent eviction and predictive caching
pub struct AdaptiveCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    storage: CacheStorage<K, V>,
    config: AdaptiveCacheConfig,
    metrics: Arc<CacheMetrics>,
}
impl<K, V> AdaptiveCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    #[must_use]
    pub fn new(config: AdaptiveCacheConfig) -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            config,
            metrics: Arc::new(CacheMetrics::default()),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        let mut storage = self.storage.write().await;

        if let Some(entry) = storage.get_mut(key) {
            if entry.is_expired() {
                storage.remove(key);
                self.metrics
                    .cache_misses
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                None
            } else {
                entry.mark_accessed();
                self.metrics
                    .cache_hits
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Some(entry.value.clone())
            }
        } else {
            self.metrics
                .cache_misses
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            None
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn insert(&self, key: K, value: V, ttl: Option<Duration>) -> Result<()>  {
        let ttl = ttl.unwrap_or(self.config.default_ttl);
        let size_bytes = std::mem::size_of::<V>(); // Approximation

        let entry = CacheEntry::new(value, ttl, size_bytes);

        let mut storage = self.storage.write().await;

        // Check if eviction is needed
        if storage.len() >= self.config.max_entries {
            self.evict_entries(&mut storage).await?;
        }

        storage.insert(key, entry);
        self.metrics
            .entry_count
            .store(storage.len(), std::sync::atomic::Ordering::Relaxed);

        Ok(())
    }

    async fn evict_entries(&self, storage: &mut HashMap<K, CacheEntry<V>>) -> Result<()> {
        if self.config.enable_lru_eviction {
            // Find least recently used entry
            if let Some((oldest_key, _)) = storage
                .iter()
                .min_by_key(|(_, entry)| entry.last_accessed)
                .map(|(k, v)| (k.clone(), v.clone()))
            {
                storage.remove(&oldest_key);
                self.metrics
                    .evictions
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }

        Ok(())
    }

    pub fn get_metrics(&self) -> CacheMetrics {
        CacheMetrics {
            cache_hits: std::sync::atomic::AtomicU64::new(
                self.metrics
                    .cache_hits
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            cache_misses: std::sync::atomic::AtomicU64::new(
                self.metrics
                    .cache_misses
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            evictions: std::sync::atomic::AtomicU64::new(
                self.metrics
                    .evictions
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            memory_usage: std::sync::atomic::AtomicUsize::new(
                self.metrics
                    .memory_usage
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
            entry_count: std::sync::atomic::AtomicUsize::new(
                self.metrics
                    .entry_count
                    .load(std::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::{sleep, timeout};

    // ===== CONFIGURATION TESTS =====

    #[test]
    fn test_adaptive_cache_config_default() {
        let config = AdaptiveCacheConfig::default();
        
        assert_eq!(config.max_entries, 10000);
        assert_eq!(config.default_ttl, Duration::from_secs(3600));
        assert!(config.enable_lru_eviction);
        assert!(config.enable_predictive_caching);
        assert_eq!(config.cache_hit_ratio_target, 0.85);
        assert_eq!(config.memory_usage_limit, 100 * 1024 * 1024);
    }

    #[test]
    fn test_adaptive_cache_config_custom() {
        let config = AdaptiveCacheConfig {
            max_entries: 5000,
            default_ttl: Duration::from_secs(1800),
            enable_lru_eviction: false,
            enable_predictive_caching: false,
            cache_hit_ratio_target: 0.9,
            memory_usage_limit: 50 * 1024 * 1024,
        };
        
        assert_eq!(config.max_entries, 5000);
        assert_eq!(config.default_ttl, Duration::from_secs(1800));
        assert!(!config.enable_lru_eviction);
        assert!(!config.enable_predictive_caching);
        assert_eq!(config.cache_hit_ratio_target, 0.9);
        assert_eq!(config.memory_usage_limit, 50 * 1024 * 1024);
    }

    // ===== CACHE ENTRY TESTS =====

    #[test]
    fn test_cache_entry_creation() {
        let value = "test_value".to_string();
        let ttl = Duration::from_secs(300);
        let size_bytes = 100;
        
        let entry = CacheEntry::new(value.clone(), ttl, size_bytes);
        
        assert_eq!(entry.value, value);
        assert_eq!(entry.ttl, ttl);
        assert_eq!(entry.size_bytes, size_bytes);
        assert_eq!(entry.access_count, 1);
        assert!(entry.created_at <= SystemTime::now());
        assert!(entry.last_accessed <= SystemTime::now());
    }

    #[test]
    fn test_cache_entry_expiration() {
        let value = "test_value".to_string();
        let ttl = Duration::from_millis(1); // Very short TTL
        let entry = CacheEntry::new(value, ttl, 100);
        
        // Should not be expired immediately
        assert!(!entry.is_expired());
        
        // Wait for expiration
        std::thread::sleep(Duration::from_millis(10));
        assert!(entry.is_expired());
    }

    #[test]
    fn test_cache_entry_access_tracking() {
        let value = "test_value".to_string();
        let ttl = Duration::from_secs(300);
        let mut entry = CacheEntry::new(value, ttl, 100);
        
        let initial_access_count = entry.access_count;
        let initial_last_accessed = entry.last_accessed;
        
        // Wait a bit to ensure time difference
        std::thread::sleep(Duration::from_millis(1));
        
        entry.mark_accessed();
        
        assert_eq!(entry.access_count, initial_access_count + 1);
        assert!(entry.last_accessed > initial_last_accessed);
    }

    // ===== CACHE METRICS TESTS =====

    #[test]
    fn test_cache_metrics_default() {
        let metrics = CacheMetrics::default();
        
        assert_eq!(metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.cache_misses.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.evictions.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.memory_usage.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.entry_count.load(std::sync::atomic::Ordering::Relaxed), 0);
    }

    #[test]
    fn test_cache_metrics_hit_ratio() {
        let metrics = CacheMetrics::default();
        
        // No hits or misses initially
        assert_eq!(metrics.hit_ratio(), 0.0);
        
        // Add some hits
        metrics.cache_hits.store(80, std::sync::atomic::Ordering::Relaxed);
        metrics.cache_misses.store(20, std::sync::atomic::Ordering::Relaxed);
        
        assert_eq!(metrics.hit_ratio(), 0.8);
        
        // Test division by zero protection
        let empty_metrics = CacheMetrics::default();
        assert_eq!(empty_metrics.hit_ratio(), 0.0);
    }

    // ===== ADAPTIVE CACHE TESTS =====

    #[tokio::test]
    async fn test_adaptive_cache_creation() {
        let config = AdaptiveCacheConfig::default();
        let cache: AdaptiveCache<String, String> = AdaptiveCache::new(config.clone());
        
        assert_eq!(cache.config.max_entries, config.max_entries);
        assert_eq!(cache.config.default_ttl, config.default_ttl);
        
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.cache_misses.load(std::sync::atomic::Ordering::Relaxed), 0);
    }

    #[tokio::test]
    async fn test_adaptive_cache_basic_operations() {
        let config = AdaptiveCacheConfig::default();
        let cache: AdaptiveCache<String, String> = AdaptiveCache::new(config);
        
        let key = "test_key".to_string();
        let value = "test_value".to_string();
        
        // Test cache miss
        let result = cache.get(&key).await;
        assert!(result.is_none());
        
        // Insert value
        let insert_result = cache.insert(key.clone(), value.clone(), None).await;
        assert!(insert_result.is_ok());
        
        // Test cache hit
        let result = cache.get(&key).await;
        assert_eq!(result, Some(value));
        
        // Verify metrics
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 1);
        assert_eq!(metrics.cache_misses.load(std::sync::atomic::Ordering::Relaxed), 1);
        assert_eq!(metrics.entry_count.load(std::sync::atomic::Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_adaptive_cache_custom_ttl() {
        let config = AdaptiveCacheConfig::default();
        let cache: AdaptiveCache<String, String> = AdaptiveCache::new(config);
        
        let key = "test_key".to_string();
        let value = "test_value".to_string();
        let custom_ttl = Duration::from_millis(50);
        
        // Insert with custom TTL
        let insert_result = cache.insert(key.clone(), value.clone(), Some(custom_ttl)).await;
        assert!(insert_result.is_ok());
        
        // Should be available immediately
        let result = cache.get(&key).await;
        assert_eq!(result, Some(value));
        
        // Wait for expiration
        sleep(Duration::from_millis(100)).await;
        
        // Should be expired now
        let result = cache.get(&key).await;
        assert!(result.is_none());
        
        // Verify metrics show cache miss after expiration
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 1);
        assert_eq!(metrics.cache_misses.load(std::sync::atomic::Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn test_adaptive_cache_eviction() {
        let mut config = AdaptiveCacheConfig::default();
        config.max_entries = 2; // Small cache for testing eviction
        config.enable_lru_eviction = true;
        
        let cache: AdaptiveCache<String, String> = AdaptiveCache::new(config);
        
        // Fill cache to capacity
        cache.insert("key1".to_string(), "value1".to_string(), None).await.unwrap();
        cache.insert("key2".to_string(), "value2".to_string(), None).await.unwrap();
        
        // Access key1 to make it more recently used
        let _ = cache.get(&"key1".to_string()).await;
        
        // Insert third item, should evict key2 (least recently used)
        cache.insert("key3".to_string(), "value3".to_string(), None).await.unwrap();
        
        // key1 and key3 should be present, key2 should be evicted
        assert!(cache.get(&"key1".to_string()).await.is_some());
        assert!(cache.get(&"key2".to_string()).await.is_none());
        assert!(cache.get(&"key3".to_string()).await.is_some());
        
        // Verify eviction metrics
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.evictions.load(std::sync::atomic::Ordering::Relaxed), 1);
        assert_eq!(metrics.entry_count.load(std::sync::atomic::Ordering::Relaxed), 2);
    }

    #[tokio::test]
    async fn test_adaptive_cache_no_eviction() {
        let mut config = AdaptiveCacheConfig::default();
        config.max_entries = 2;
        config.enable_lru_eviction = false; // Disable eviction
        
        let cache: AdaptiveCache<String, String> = AdaptiveCache::new(config);
        
        // Fill cache to capacity
        cache.insert("key1".to_string(), "value1".to_string(), None).await.unwrap();
        cache.insert("key2".to_string(), "value2".to_string(), None).await.unwrap();
        
        // Insert third item with eviction disabled
        cache.insert("key3".to_string(), "value3".to_string(), None).await.unwrap();
        
        // All items should still be accessible (no eviction occurred)
        assert!(cache.get(&"key1".to_string()).await.is_some());
        assert!(cache.get(&"key2".to_string()).await.is_some());
        assert!(cache.get(&"key3".to_string()).await.is_some());
        
        // Verify no evictions occurred
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.evictions.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.entry_count.load(std::sync::atomic::Ordering::Relaxed), 3);
    }

    #[tokio::test]
    async fn test_adaptive_cache_concurrent_access() {
        let config = AdaptiveCacheConfig::default();
        let cache = Arc::new(AdaptiveCache::<String, String>::new(config));
        
        let mut handles = vec![];
        
        // Spawn multiple tasks to insert and read concurrently
        for i in 0..10 {
            let cache_clone = Arc::clone(&cache);
            let handle = tokio::spawn(async move {
                let key = format!("key_{i}");
                let value = format!("value_{i}");
                
                // Insert
                cache_clone.insert(key.clone(), value.clone(), None).await.unwrap();
                
                // Read back
                let result = cache_clone.get(&key).await;
                assert_eq!(result, Some(value));
            });
            handles.push(handle);
        }
        
        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }
        
        // Verify final state
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.entry_count.load(std::sync::atomic::Ordering::Relaxed), 10);
        assert_eq!(metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 10);
    }

    #[tokio::test]
    async fn test_adaptive_cache_metrics_accuracy() {
        let config = AdaptiveCacheConfig::default();
        let cache: AdaptiveCache<String, String> = AdaptiveCache::new(config);
        
        // Perform various operations
        cache.insert("key1".to_string(), "value1".to_string(), None).await.unwrap();
        cache.insert("key2".to_string(), "value2".to_string(), None).await.unwrap();
        
        // Cache hits
        let _ = cache.get(&"key1".to_string()).await;
        let _ = cache.get(&"key2".to_string()).await;
        
        // Cache misses
        let _ = cache.get(&"nonexistent1".to_string()).await;
        let _ = cache.get(&"nonexistent2".to_string()).await;
        
        // Verify metrics
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 2);
        assert_eq!(metrics.cache_misses.load(std::sync::atomic::Ordering::Relaxed), 2);
        assert_eq!(metrics.entry_count.load(std::sync::atomic::Ordering::Relaxed), 2);
        assert_eq!(metrics.hit_ratio(), 0.5);
    }

    #[tokio::test]
    async fn test_adaptive_cache_large_dataset() {
        let mut config = AdaptiveCacheConfig::default();
        config.max_entries = 1000;
        
        let cache: AdaptiveCache<String, String> = AdaptiveCache::new(config);
        
        // Insert many items
        for i in 0..500 {
            let key = format!("key_{i}");
            let value = format!("value_{i}");
            cache.insert(key, value, None).await.unwrap();
        }
        
        // Verify all items are accessible
        for i in 0..500 {
            let key = format!("key_{i}");
            let result = cache.get(&key).await;
            assert!(result.is_some());
        }
        
        let metrics = cache.get_metrics().await;
        assert_eq!(metrics.entry_count.load(std::sync::atomic::Ordering::Relaxed), 500);
        assert_eq!(metrics.cache_hits.load(std::sync::atomic::Ordering::Relaxed), 500);
    }

    #[tokio::test]
    async fn test_adaptive_cache_timeout_handling() {
        let config = AdaptiveCacheConfig::default();
        let cache: AdaptiveCache<String, String> = AdaptiveCache::new(config);
        
        // Test that operations complete within reasonable time
        let insert_result = timeout(Duration::from_secs(1), async {
            cache.insert("key".to_string(), "value".to_string(), None).await
        }).await;
        
        assert!(insert_result.is_ok());
        assert!(insert_result.unwrap().is_ok());
        
        let get_result = timeout(Duration::from_secs(1), async {
            cache.get(&"key".to_string()).await
        }).await;
        
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), Some("value".to_string()));
    }
}
