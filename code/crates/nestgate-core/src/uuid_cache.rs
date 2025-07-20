//! High-Performance UUID Caching System
//!
//! This module provides optimized UUID caching to eliminate performance bottlenecks
//! in service registration and identification operations.
//!
//! ## Performance Impact
//! - **Before**: 274,587 ns/iter (frequent UUID generation)
//! - **Target**: <50,000 ns/iter (5x performance improvement)
//! - **Strategy**: Cache UUIDs using Arc&lt;Uuid&gt; for zero-copy sharing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// High-performance UUID cache with Arc-based sharing
#[derive(Debug)]
pub struct UuidCache {
    /// Thread-safe cache storage
    cache: Arc<RwLock<HashMap<String, Arc<Uuid>>>>,
    /// Generation counter for cache statistics
    generation_counter: Arc<AtomicU64>,
    /// Hit counter for performance metrics
    hit_counter: Arc<AtomicU64>,
    /// Miss counter for cache efficiency tracking
    miss_counter: Arc<AtomicU64>,
}

impl UuidCache {
    /// Create a new UUID cache instance
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            generation_counter: Arc::new(AtomicU64::new(0)),
            hit_counter: Arc::new(AtomicU64::new(0)),
            miss_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get or create a UUID for the given key
    ///
    /// This is the main performance-critical method that eliminates
    /// frequent UUID generation through intelligent caching.
    pub fn get_or_create(&self, key: &str) -> Arc<Uuid> {
        // Fast path: Check cache first (read lock)
        if let Ok(cache) = self.cache.read() {
            if let Some(uuid) = cache.get(key) {
                self.hit_counter.fetch_add(1, Ordering::Relaxed);
                return Arc::clone(uuid);
            }
        }

        // Slow path: Generate and cache (write lock)
        if let Ok(mut cache) = self.cache.write() {
            // Double-check pattern: UUID might have been created by another thread
            if let Some(uuid) = cache.get(key) {
                self.hit_counter.fetch_add(1, Ordering::Relaxed);
                return Arc::clone(uuid);
            }

            // Generate new UUID and cache it
            let new_uuid = Arc::new(Uuid::new_v4());
            cache.insert(key.to_string(), Arc::clone(&new_uuid));

            // Update counters
            self.generation_counter.fetch_add(1, Ordering::Relaxed);
            self.miss_counter.fetch_add(1, Ordering::Relaxed);

            new_uuid
        } else {
            // Fallback: If cache is poisoned, generate UUID without caching
            self.miss_counter.fetch_add(1, Ordering::Relaxed);
            Arc::new(Uuid::new_v4())
        }
    }

    /// Get a UUID from cache without creating if missing
    pub fn get(&self, key: &str) -> Option<Arc<Uuid>> {
        self.cache.read().ok()?.get(key).map(Arc::clone)
    }

    /// Pre-populate cache with known UUIDs for hot paths
    pub fn preload(&self, entries: Vec<(String, Uuid)>) {
        if let Ok(mut cache) = self.cache.write() {
            for (key, uuid) in entries {
                cache.insert(key, Arc::new(uuid));
            }
        }
    }

    /// Clear all cached entries
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// Get cache performance statistics
    pub fn statistics(&self) -> CacheStatistics {
        let generations = self.generation_counter.load(Ordering::Relaxed);
        let hits = self.hit_counter.load(Ordering::Relaxed);
        let misses = self.miss_counter.load(Ordering::Relaxed);
        let total_requests = hits + misses;

        let hit_ratio = if total_requests > 0 {
            hits as f64 / total_requests as f64
        } else {
            0.0
        };

        let cache_size = self.cache.read().map(|c| c.len()).unwrap_or(0);

        CacheStatistics {
            cache_size,
            total_generations: generations,
            cache_hits: hits,
            cache_misses: misses,
            hit_ratio,
        }
    }

    /// Get cache size (number of entries)
    pub fn size(&self) -> usize {
        self.cache.read().map(|c| c.len()).unwrap_or(0)
    }

    /// Remove a specific entry from cache
    pub fn remove(&self, key: &str) -> Option<Arc<Uuid>> {
        self.cache.write().ok()?.remove(key)
    }
}

impl Default for UuidCache {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Clone for UuidCache to allow sharing across components using explicit Arc::clone for clarity
impl Clone for UuidCache {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
            generation_counter: Arc::clone(&self.generation_counter),
            hit_counter: Arc::clone(&self.hit_counter),
            miss_counter: Arc::clone(&self.miss_counter),
        }
    }
}

/// Cache performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    /// Number of cached entries
    pub cache_size: usize,
    /// Total UUIDs generated
    pub total_generations: u64,
    /// Cache hit count
    pub cache_hits: u64,
    /// Cache miss count  
    pub cache_misses: u64,
    /// Hit ratio (0.0 to 1.0)
    pub hit_ratio: f64,
}

impl CacheStatistics {
    /// Check if cache is performing well (>70% hit ratio is good)
    pub fn is_efficient(&self) -> bool {
        self.hit_ratio > 0.7
    }

    /// Get performance assessment
    pub fn performance_assessment(&self) -> &'static str {
        match self.hit_ratio {
            r if r > 0.9 => "Excellent",
            r if r > 0.7 => "Good",
            r if r > 0.5 => "Fair",
            _ => "Poor - Consider preloading or key optimization",
        }
    }
}

// Global UUID cache instance for application-wide usage
lazy_static::lazy_static! {
    // Global UUID cache accessible throughout the application
    pub static ref GLOBAL_UUID_CACHE: UuidCache = UuidCache::new();
}

/// Convenience function for global UUID caching
pub fn get_or_create_uuid(key: &str) -> Arc<Uuid> {
    GLOBAL_UUID_CACHE.get_or_create(key)
}

/// Convenience function for getting UUID from global cache
pub fn get_uuid(key: &str) -> Option<Arc<Uuid>> {
    GLOBAL_UUID_CACHE.get(key)
}

/// Preload commonly used UUIDs into global cache
pub fn preload_common_uuids(entries: Vec<(String, Uuid)>) {
    GLOBAL_UUID_CACHE.preload(entries);
}

/// Get global cache statistics
pub fn global_cache_statistics() -> CacheStatistics {
    GLOBAL_UUID_CACHE.statistics()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_uuid_cache_basic_functionality() {
        let cache = UuidCache::new();

        // First call should generate and cache
        let uuid1 = cache.get_or_create("test-service");
        let uuid2 = cache.get_or_create("test-service");

        // Should return the same UUID (same Arc)
        assert_eq!(uuid1.to_string(), uuid2.to_string());

        // Statistics should show 1 generation, 1 hit, 1 miss
        let stats = cache.statistics();
        assert_eq!(stats.total_generations, 1);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
    }

    #[test]
    fn test_cache_performance() {
        let cache = UuidCache::new();

        // Generate multiple UUIDs for same key
        for _ in 0..100 {
            cache.get_or_create("performance-test");
        }

        let stats = cache.statistics();
        assert_eq!(stats.total_generations, 1); // Only 1 UUID generated
        assert_eq!(stats.cache_hits, 99); // 99 cache hits
        assert!(stats.hit_ratio > 0.9); // >90% hit ratio
        assert!(stats.is_efficient()); // Should be efficient
    }

    #[test]
    fn test_concurrent_access() {
        let cache = Arc::new(UuidCache::new());
        let mut handles = vec![];

        // Spawn multiple threads accessing same cache
        for i in 0..10 {
            let cache_clone = Arc::clone(&cache);
            let handle =
                thread::spawn(move || cache_clone.get_or_create(&format!("service-{}", i % 3)));
            handles.push(handle);
        }

        // Collect results
        let mut results = vec![];
        for handle in handles {
            results.push(handle.join().unwrap());
        }

        // Should have at most 3 different UUIDs (since we used i % 3)
        let stats = cache.statistics();
        assert!(stats.total_generations <= 3);
        assert!(stats.cache_hits >= 7); // Many hits due to overlapping keys
    }

    #[test]
    fn test_global_cache() {
        // Test global convenience functions
        let uuid1 = get_or_create_uuid("global-test");
        let uuid2 = get_or_create_uuid("global-test");

        assert_eq!(uuid1.to_string(), uuid2.to_string());

        // Test get function
        let uuid3 = get_uuid("global-test");
        assert!(uuid3.is_some());
        assert_eq!(uuid1.to_string(), uuid3.unwrap().to_string());
    }

    #[test]
    fn test_preloading() {
        let cache = UuidCache::new();
        let test_uuid = Uuid::new_v4();

        // Preload specific UUID
        cache.preload(vec![("preloaded-service".to_string(), test_uuid)]);

        // Should return the preloaded UUID
        let cached_uuid = cache.get_or_create("preloaded-service");
        assert_eq!(cached_uuid.to_string(), test_uuid.to_string());

        // Statistics should show 0 generations (preloaded)
        let stats = cache.statistics();
        assert_eq!(stats.total_generations, 0);
    }
}
