//! String Pool module

use crate::error::NestGateError;
use std::collections::HashMap;
//
// This module provides optimized string pooling to eliminate performance bottlenecks
// in string allocation and cloning operations.
//
// ## Performance Impact
// - **Before**: Frequent string allocations with .to_string()
// - **Target**: <10ns string retrieval from pool (20x+ improvement)
// - **Strategy**: Arc<str> sharing for zero-copy string operations

use crate::idiomatic_evolution::SafeResultExt;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLockReadGuard, RwLockWriteGuard};

use crate::canonical_modernization::canonical_constants::{
    api::{STATUS_OK, STATUS_NOT_FOUND},
    network::{LOCALHOST, DEFAULT_API_PORT},
};

// Type aliases to reduce complexity
type StringPoolReadGuard<'a> = RwLockReadGuard<'a, HashMap<String, Arc<str>>>;
/// Type alias for StringPoolWriteGuard
type StringPoolWriteGuard<'a> = RwLockWriteGuard<'a, HashMap<String, Arc<str>>>;

/// High-performance string pool with Arc-based sharing
#[derive(Debug)]
/// Stringpool
pub struct StringPool {
    /// Thread-safe string cache storage
    pool: crate::canonical::types::StringPoolMap,
    /// Hit counter for performance metrics
    hit_counter: Arc<AtomicU64>,
    /// Miss counter for cache efficiency tracking
    miss_counter: Arc<AtomicU64>,
}
impl StringPool {
    /// Create a new string pool instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            pool: Arc::new(std::sync::RwLock::new(HashMap::new())),
            hit_counter: Arc::new(AtomicU64::new(0)),
            miss_counter: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get existing string or intern new one
    pub fn get_or_intern(&self, value: &str) -> crate::Result<Arc<str>> {
        // ✅ IDIOMATIC EVOLUTION: Safe lock acquisition instead of unwrap()
        let pool = self.safe_read_lock()?;

        if let Some(existing) = pool.get(value) {
            self.hit_counter.fetch_add(1, Ordering::Relaxed);
            return Ok(Arc::clone(existing));
        }

        // Need to upgrade to write lock
        drop(pool);
        let mut pool = self.safe_write_lock()?;

        // Double-check pattern
        if let Some(existing) = pool.get(value) {
            self.hit_counter.fetch_add(1, Ordering::Relaxed);
            return Ok(Arc::clone(existing));
        }

        // Insert new string
        let arc_string: Arc<str> = Arc::from(value);
        pool.insert(value.to_string(), Arc::clone(&arc_string));
        self.miss_counter.fetch_add(1, Ordering::Relaxed);

        Ok(arc_string)
    }

    /// Get existing string if pooled
    pub fn get(&self, value: &str) -> crate::Result<Option<Arc<str>>> {
        let pool = self.safe_read_lock()?;
        Ok(pool.get(value).map(Arc::clone))
    }

    /// Remove string from pool
    pub fn remove(&self, value: &str) -> crate::Result<Option<Arc<str>>> {
        let mut pool = self.safe_write_lock()?;
        Ok(pool.remove(value))
    }

    /// Preload strings into the pool
    pub fn preload(&self, strings: Vec<&str>) {
        if let Ok(mut pool) = self.pool.write() {
            for s in strings {
                if !pool.contains_key(s) {
                    let shared_str: Arc<str> = Arc::from(s);
                    pool.insert(s.to_string(), shared_str);
                }
            }
        }
    }

    /// Sync version of preload for initialization contexts
    pub fn preload_sync(&self, strings: Vec<&str>) {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.preload(strings))
        });
    }

    /// Get pool statistics
    pub fn statistics(&self) -> StringPoolStatistics {
        let pool_size = self.pool.read().map(|p| p.len()).unwrap_or(0);
        let hits = self.hit_counter.load(Ordering::Relaxed);
        let misses = self.miss_counter.load(Ordering::Relaxed);
        let total_requests = hits + misses;
        let hit_ratio = if total_requests > 0 {
            hits as f64 / total_requests as f64
        } else {
            0.0
        };

        StringPoolStatistics {
            pool_size,
            total_requests,
            cache_hits: hits,
            cache_misses: misses,
            hit_ratio,
        }
    }

    /// Clear the pool
    pub fn clear(&self) -> crate::Result<()> {
        let mut pool = self.safe_write_lock()?;
        pool.clear();
        Ok(())
    }

    /// Get current pool size
    pub fn len(&self) -> crate::Result<usize> {
        let pool = self.safe_read_lock()?;
        Ok(pool.len())
    }

    /// Check if pool is empty
    pub fn is_empty(&self) -> crate::Result<bool> {
        let pool = self.safe_read_lock()?;
        Ok(pool.is_empty())
    }

    /// Get pool capacity
    pub fn capacity(&self) -> crate::Result<usize> {
        let pool = self.safe_read_lock()?;
        Ok(pool.len()) // HashMap doesn't expose capacity directly
    }
}

impl Clone for StringPool {
    /// Clone
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
            hit_counter: Arc::clone(&self.hit_counter),
            miss_counter: Arc::clone(&self.miss_counter),
        }
    }
}

impl Default for StringPool {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// **IDIOMATIC EVOLUTION**: Safe lock acquisition utilities
impl StringPool {
    /// Safe read lock acquisition with context
    fn safe_read_lock(&self) -> crate::Result<StringPoolReadGuard> {
        self.pool
            .read()
            .map_err(|_| crate::error::NestGateError::internal_error(
                "String pool read lock poisoned".to_string(),
                Some("safe_read_lock".to_string())
            ))
    }
    /// Safe Write Lock
    fn safe_write_lock(&self) -> crate::Result<StringPoolWriteGuard> {
        self.pool
            .write()
            .map_err(|_| crate::error::NestGateError::internal_error(
                "String pool write lock poisoned".to_string(),
                Some("safe_write_lock".to_string())
            ))
    }
}

/// String pool performance statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Stringpoolstatistics
pub struct StringPoolStatistics {
    /// Number of pooled strings
    pub pool_size: usize,
    /// Total string requests
    pub total_requests: u64,
    /// Pool hit count
    pub cache_hits: u64,
    /// Pool miss count  
    pub cache_misses: u64,
    /// Hit ratio (0.0 to 1.0)
    pub hit_ratio: f64,
}
impl StringPoolStatistics {
    /// Check if pool is performing well (>80% hit ratio is excellent)
    pub fn is_efficient(&self) -> bool {
        self.hit_ratio > 0.8
    }

    /// Get performance assessment
    pub fn performance_assessment(&self) -> &'static str {
        match self.hit_ratio {
            r if r > 0.95 => "Excellent",
            r if r > 0.8 => "Very Good",
            r if r > 0.6 => "Good",
            r if r > 0.4 => "Fair",
            _ => "Poor - Consider preloading common strings",
        }
    }
}

// Uses std::sync::LazyLock for thread-safe lazy initialization
// Global string pool instance for application-wide usage

/// Global string pool accessible throughout the application
pub static GLOBAL_STRING_POOL: std::sync::LazyLock<StringPool> = std::sync::LazyLock::new(|| {
    let pool = StringPool::new();

    // Port strings from canonical constants (avoids hardcoded "8080" and "3000")
    let api_port_str = DEFAULT_API_PORT.to_string();
    let dev_port_str = crate::constants::port_defaults::DEFAULT_DEV_PORT.to_string();

    // Preload common strings
    pool.preload_sync(vec![
        // Status strings
        "active", "inactive", "pending", "completed", "failed", "success", "error",
        "healthy", "unhealthy", "online", "offline", "provisioned", "deprecated",

        // Storage tiers
        "hot", "warm", "cold", "archive",

        // Protocols
        "HTTP", "HTTPS", "ZFS", "NFS", "SMB", "TCP", "UDP",

        // Common endpoints
        "/health", "/metrics", "/api/v1", "/ws",

        // Service names
        "nestgate", "storage", "zfs-manager", "security", "ai", "orchestration", "compute",

        // Common error messages
        "Not found", "Internal error", "Invalid input", "Unauthorized", "Timeout",

        // Configuration values (from port_defaults / canonical_constants)
        LOCALHOST,
        api_port_str.as_str(),
        dev_port_str.as_str(),
        STATUS_OK,
        "false",

        // ZFS specific
        "ONLINE", "DEGRADED", "FAULTED", "lz4", "gzip", "compression",
    ]);

    pool
});

/// Global string interning function - high performance API
pub fn intern_string(value: &str) -> Arc<str> {
    // ✅ IDIOMATIC EVOLUTION: Safe error handling instead of blocking on non-future
    GLOBAL_STRING_POOL.get_or_intern(value).unwrap_or_else(|_| {
        // Fallback: create Arc<str> directly if pool fails
        Arc::from(value)
    })
}
/// Global string retrieval function
pub fn get_string(value: &str) -> Option<Arc<str>> {
    GLOBAL_STRING_POOL
        .get(value)
        .unwrap_or_default_with_log("global_string_get")
}
/// Preload additional strings into global pool
pub fn preload_strings(strings: Vec<&str>) {
    GLOBAL_STRING_POOL.preload_sync(strings);
}
/// Get global string pool statistics
pub fn global_string_pool_statistics() -> StringPoolStatistics {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(GLOBAL_STRING_POOL.statistics())
    })
}
/// Macro for compile-time string interning
#[macro_export]
macro_rules! intern {
    ($s:expr) => {
        $crate::string_pool::intern_string($s)
    };
}
/// Macro for creating Arc<str> from string literal at compile time
#[macro_export]
macro_rules! static_str {
    ($s:literal) => {{
        static CACHED: std::sync::OnceLock<std::sync::Arc<str>> = std::sync::OnceLock::new();
        CACHED.get_or_init(|| std::sync::Arc::from($s)).clone()
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_string_pool_operations() -> Result<()> {
        let pool = StringPool::new();

        let str1 = pool.get_or_intern("test")?;
        let str2 = pool.get_or_intern("test")?;

        // Test that identical strings share the same Arc
        assert!(Arc::ptr_eq(&str1, &str2));

        println!("✅ String pool operations working");
        Ok(())
    }

    #[tokio::test]
    async fn test_string_pool_preloading() -> Result<()> {
        let pool = StringPool::new();

        // Preload common strings
        pool.preload(vec!["preloaded1".to_string(), "preloaded2".to_string()])?;

        let str1 = pool.get_or_intern("preloaded1")?;
        let str2 = pool.get_or_intern("preloaded2")?;

        assert_eq!(str1.as_ref(), "preloaded1");
        assert_eq!(str2.as_ref(), "preloaded2");

        println!("✅ String pool preloading working");
        Ok(())
    }

    #[test]
    fn test_global_string_pool() {
        let str1 = intern_string("global_test");
        let str2 = intern_string("global_test");

        assert!(Arc::ptr_eq(&str1, &str2));
        assert_eq!(&*str1, "global_test");
    }

    #[test]
    fn test_static_str_macro() {
        let str1 = static_str!("static_test");
        let str2 = static_str!("static_test");

        assert!(Arc::ptr_eq(&str1, &str2));
        assert_eq!(&*str1, "static_test");
    }
}
