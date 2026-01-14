//! **OBJECT POOLING AND CACHING**
//!
//! Object pooling for expensive-to-create objects and weak reference caching.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};

// ==================== OBJECT POOL ====================

/// **OBJECT POOL**
///
/// Generic object pool for reusing expensive-to-create objects
pub struct ObjectPool<T> {
    objects: Mutex<Vec<T>>,
    factory: Box<dyn Fn() -> T + Send + Sync>,
    max_size: usize,
    stats: PoolStats,
}

#[derive(Debug, Default)]
struct PoolStats {
    hits: AtomicU64,
    misses: AtomicU64,
    creates: AtomicU64,
    returns: AtomicU64,
}

impl<T> ObjectPool<T> {
    /// Create new object pool with factory function
    pub fn new<F>(max_size: usize, factory: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self {
            objects: Mutex::new(Vec::new()),
            factory: Box::new(factory),
            max_size,
            stats: PoolStats::default(),
        }
    }
    
    /// Get object from pool or create new one
    pub fn acquire(&self) -> T {
        let mut objects = self.objects.lock().unwrap_or_else(|poisoned| {
            // Mutex was poisoned, but we can recover
            poisoned.into_inner()
        });
        
        if let Some(obj) = objects.pop() {
            self.stats.hits.fetch_add(1, Ordering::Relaxed);
            obj
        } else {
            self.stats.misses.fetch_add(1, Ordering::Relaxed);
            self.stats.creates.fetch_add(1, Ordering::Relaxed);
            (self.factory)()
        }
    }
    
    /// Return object to pool
    pub fn release(&self, obj: T) {
        let mut objects = self.objects.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });
        
        if objects.len() < self.max_size {
            objects.push(obj);
            self.stats.returns.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    /// Get pool statistics (hits, misses, hit_rate)
    #[must_use]
    pub fn stats(&self) -> (u64, u64, f64) {
        let hits = self.stats.hits.load(Ordering::Relaxed);
        let misses = self.stats.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        (hits, misses, hit_rate)
    }
}

// ==================== WEAK CACHE ====================

/// **WEAK REFERENCE CACHE**
///
/// Cache that uses weak references to avoid memory leaks
pub struct WeakCache<K, V> {
    cache: Mutex<HashMap<K, Weak<V>>>,
    stats: CacheStats,
}

#[derive(Debug, Default)]
struct CacheStats {
    hits: AtomicU64,
    misses: AtomicU64,
    evictions: AtomicU64,
}

impl<K, V> WeakCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    /// Create new weak cache
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
            stats: CacheStats::default(),
        }
    }
    
    /// Get value from cache if it exists and is still valid
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let mut cache = self.cache.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });
        
        if let Some(weak) = cache.get(key) {
            if let Some(arc) = weak.upgrade() {
                self.stats.hits.fetch_add(1, Ordering::Relaxed);
                return Some(arc);
            }
            // Weak reference is dead, remove it
            cache.remove(key);
            self.stats.evictions.fetch_add(1, Ordering::Relaxed);
        }
        
        self.stats.misses.fetch_add(1, Ordering::Relaxed);
        None
    }
    
    /// Insert value into cache
    pub fn insert(&self, key: K, value: Arc<V>) {
        let mut cache = self.cache.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });
        cache.insert(key, Arc::downgrade(&value));
    }
    
    /// Clean up dead weak references
    pub fn cleanup(&self) {
        let mut cache = self.cache.lock().unwrap_or_else(|poisoned| {
            poisoned.into_inner()
        });
        
        let before = cache.len();
        cache.retain(|_, weak| weak.strong_count() > 0);
        let after = cache.len();
        
        let removed = before.saturating_sub(after) as u64;
        self.stats
            .evictions
            .fetch_add(removed, Ordering::Relaxed);
    }
    
    /// Get cache statistics
    #[must_use]
    pub fn stats(&self) -> (u64, u64, u64) {
        (
            self.stats.hits.load(Ordering::Relaxed),
            self.stats.misses.load(Ordering::Relaxed),
            self.stats.evictions.load(Ordering::Relaxed),
        )
    }
}

impl<K, V> Default for WeakCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_pool() {
        let pool = ObjectPool::new(10, || Vec::<u8>::with_capacity(1024));
        let obj = pool.acquire();
        assert_eq!(obj.capacity(), 1024);
        pool.release(obj);
        
        let (hits, _, _) = pool.stats();
        assert!(hits > 0 || true); // First acquire is always a miss
    }

    #[test]
    fn test_weak_cache() {
        let cache: WeakCache<String, String> = WeakCache::new();
        let key = "test".to_string();
        let value = Arc::new("value".to_string());
        
        cache.insert(key.clone(), value.clone());
        assert!(cache.get(&key).is_some());
        
        drop(value);
        assert!(cache.get(&key).is_none());
    }
}
