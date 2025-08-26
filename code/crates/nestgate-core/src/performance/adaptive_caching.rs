use std::collections::HashMap;
//
// Intelligent caching with adaptive algorithms and workload-aware optimization.

use crate::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

    pub async fn insert(&self, key: K, value: V, ttl: Option<Duration>) -> Result<()> {
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

    pub async fn get_metrics(&self) -> CacheMetrics {
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
