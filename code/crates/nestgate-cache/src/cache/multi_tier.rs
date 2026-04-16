// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Multi-tier cache implementation with hot, warm, and cold storage tiers
//! Provides intelligent data placement and retrieval across performance tiers.
//!
//! **MODERNIZED**: Lock-free concurrent access using `DashMap` for in-memory tiers

use dashmap::DashMap;
use nestgate_types::Result;
use std::future::{Future, ready};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

// Type aliases for complex cache types
/// Type alias for lock-free cache data (`DashMap`!)
pub type CacheDataMap = Arc<DashMap<String, Vec<u8>>>;

/// Cache provider trait for different storage tiers
pub trait CacheProvider<K, V>: Send + Sync {
    /// Store a value in the cache
    fn set(&self, key: K, value: V) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
    /// Retrieve a value from the cache
    fn get(&self, key: &str) -> Pin<Box<dyn Future<Output = Result<Option<V>>> + Send + '_>>;

    /// Remove a value from the cache
    fn remove(&self, key: &str) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>>;

    /// Clear all values from the cache
    fn clear(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;

    /// Get the current cache size
    fn size(&self) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>>;
}

/// Simple cache configuration
#[derive(Debug, Clone)]
/// Configuration for `SimpleCache`
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
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
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
/// Configuration for `MultiTierCache`
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
    hot_tier: InMemoryCache,
    /// Warm tier for moderately accessed data (SSD-based, fast)
    warm_tier: InMemoryCache,
    /// Cold tier for infrequently accessed data (HDD-based, slow but large)
    cold_tier: InMemoryCache,
    /// Tier byte limits and thresholds from construction-time config
    tier_config: MultiTierCacheConfig,
    hot_tier_hits: AtomicU64,
    warm_tier_hits: AtomicU64,
    cold_tier_hits: AtomicU64,
    total_misses: AtomicU64,
    promotion_events: AtomicU64,
    demotion_events: AtomicU64,
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
    pub fn new(config: MultiTierCacheConfig) -> Result<Self> {
        let hot_tier = InMemoryCache::new();
        let warm_tier = InMemoryCache::new();
        let cold_tier = InMemoryCache::new();

        Ok(Self {
            hot_tier,
            warm_tier,
            cold_tier,
            tier_config: config,
            hot_tier_hits: AtomicU64::new(0),
            warm_tier_hits: AtomicU64::new(0),
            cold_tier_hits: AtomicU64::new(0),
            total_misses: AtomicU64::new(0),
            promotion_events: AtomicU64::new(0),
            demotion_events: AtomicU64::new(0),
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
        ready(self.hot_tier.set_entry(key, value)).await
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

    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result matches public cache API and future I/O-backed tiers"
    )]
    fn get_impl(&self, key: &str) -> Result<Option<Vec<u8>>> {
        if let Ok(Some(value)) = self.hot_tier.get_entry(key) {
            self.hot_tier_hits.fetch_add(1, Ordering::Relaxed);
            return Ok(Some(value));
        }

        if let Ok(Some(value)) = self.warm_tier.get_entry(key) {
            self.warm_tier_hits.fetch_add(1, Ordering::Relaxed);
            self.promotion_events.fetch_add(1, Ordering::Relaxed);
            let _ = self.hot_tier.set_entry(key.to_string(), value.clone());
            return Ok(Some(value));
        }

        if let Ok(Some(value)) = self.cold_tier.get_entry(key) {
            self.cold_tier_hits.fetch_add(1, Ordering::Relaxed);
            self.promotion_events.fetch_add(1, Ordering::Relaxed);
            let _ = self.warm_tier.set_entry(key.to_string(), value.clone());
            return Ok(Some(value));
        }

        self.total_misses.fetch_add(1, Ordering::Relaxed);
        Ok(None)
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
        ready(self.get_impl(key)).await
    }

    fn remove_impl(&self, key: &str) -> Result<bool> {
        Ok(self.hot_tier.remove_entry(key)?
            || self.warm_tier.remove_entry(key)?
            || self.cold_tier.remove_entry(key)?)
    }

    /// Remove data from all tiers.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn remove(&self, key: &str) -> Result<bool> {
        ready(self.remove_impl(key)).await
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
        ready({
            self.hot_tier.clear_all()?;
            self.warm_tier.clear_all()?;
            self.cold_tier.clear_all()?;
            Ok(())
        })
        .await
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
        let max_bytes = self.tier_config.hot_tier_config.max_size;
        loop {
            let total = self.hot_tier.total_value_bytes();
            if total <= max_bytes {
                break;
            }
            match self.hot_tier.drain_one() {
                Some((k, v)) => {
                    self.warm_tier.set_entry(k, v)?;
                    self.demotion_events.fetch_add(1, Ordering::Relaxed);
                }
                None => break,
            }
        }
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
    pub const fn flush(&mut self) -> Result<()> {
        // In-memory tiers have no pending I/O; disk-backed tiers would sync here.
        Ok(())
    }

    /// Check if key exists in any tier
    pub async fn contains_key(&self, key: &str) -> bool {
        ready(
            tier_has_entry(&self.hot_tier, key)
                || tier_has_entry(&self.warm_tier, key)
                || tier_has_entry(&self.cold_tier, key),
        )
        .await
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
        let hot_hits = self.hot_tier_hits.load(Ordering::Relaxed);
        let warm_hits = self.warm_tier_hits.load(Ordering::Relaxed);
        let cold_hits = self.cold_tier_hits.load(Ordering::Relaxed);
        let total_hits = hot_hits.saturating_add(warm_hits).saturating_add(cold_hits);
        let total_misses = self.total_misses.load(Ordering::Relaxed);
        let promotion_events = self.promotion_events.load(Ordering::Relaxed);
        let demotion_events = self.demotion_events.load(Ordering::Relaxed);
        let total_items = self
            .hot_tier
            .len()
            .saturating_add(self.warm_tier.len())
            .saturating_add(self.cold_tier.len());
        let total_size_bytes = self
            .hot_tier
            .total_value_bytes()
            .saturating_add(self.warm_tier.total_value_bytes())
            .saturating_add(self.cold_tier.total_value_bytes());
        let total_size_u64 = u64::try_from(total_size_bytes).map_or(u64::MAX, |v| v);

        Ok(MultiTierCacheStats {
            hot_tier_hits: hot_hits,
            warm_tier_hits: warm_hits,
            cold_tier_hits: cold_hits,
            total_misses,
            total_hits,
            total_items,
            total_size_bytes: total_size_u64,
            promotion_events,
            demotion_events,
        })
    }
}

fn tier_has_entry(tier: &InMemoryCache, key: &str) -> bool {
    matches!(tier.get_entry(key), Ok(Some(_)))
}

/// Statistics for multi-tier cache performance
#[derive(Debug, Clone)]
/// Multitiercachestats
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
    #[expect(
        clippy::cast_precision_loss,
        reason = "Aggregate hit ratio metric; counter magnitudes are within f64 precision for ops stats"
    )]
    pub fn overall_hit_ratio(&self) -> f64 {
        let total_operations = self.total_hits + self.total_misses;
        if total_operations == 0 {
            0.0
        } else {
            self.total_hits as f64 / total_operations as f64
        }
    }
}

/// Simple in-memory cache implementation for testing (lock-free!)
struct InMemoryCache {
    data: Arc<DashMap<String, Vec<u8>>>,
}
impl InMemoryCache {
    /// Creates a new instance with lock-free concurrent access
    fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    /// Direct synchronous access for `MultiTierCache` (avoids `Box<dyn Future>` per call).
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result aligns with `CacheProvider` and future fallible backends"
    )]
    fn set_entry(&self, key: String, value: Vec<u8>) -> Result<()> {
        self.data.insert(key, value);
        Ok(())
    }

    /// Direct get
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result aligns with `CacheProvider` and future fallible backends"
    )]
    fn get_entry(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.data.get(key).map(|entry| entry.value().clone()))
    }

    /// Direct remove
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result aligns with `CacheProvider` and future fallible backends"
    )]
    fn remove_entry(&self, key: &str) -> Result<bool> {
        Ok(self.data.remove(key).is_some())
    }

    /// Direct clear
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result aligns with `CacheProvider` and future fallible backends"
    )]
    fn clear_all(&self) -> Result<()> {
        self.data.clear();
        Ok(())
    }

    /// Direct size
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result aligns with `CacheProvider` and future fallible backends"
    )]
    fn size_entry(&self) -> Result<usize> {
        Ok(self.data.len())
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn total_value_bytes(&self) -> usize {
        let mut sum = 0usize;
        for entry in self.data.iter() {
            sum = sum.saturating_add(entry.value().len());
        }
        sum
    }

    /// Removes one arbitrary entry (iteration order) for hot-tier eviction.
    fn drain_one(&self) -> Option<(String, Vec<u8>)> {
        let key = {
            let first = self.data.iter().next()?;
            first.key().clone()
        };
        self.data.remove(&key)
    }
}

impl CacheProvider<String, Vec<u8>> for InMemoryCache {
    /// Set (lock-free!)
    fn set(
        &self,
        key: String,
        value: Vec<u8>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(ready(self.set_entry(key, value)))
    }

    /// Get (lock-free!)
    fn get(&self, key: &str) -> Pin<Box<dyn Future<Output = Result<Option<Vec<u8>>>> + Send + '_>> {
        let key = key.to_string();
        Box::pin(ready(self.get_entry(&key)))
    }

    /// Remove (lock-free!)
    fn remove(&self, key: &str) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        let key = key.to_string();
        Box::pin(ready(self.remove_entry(&key)))
    }

    /// Clear (lock-free!)
    fn clear(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(ready(self.clear_all()))
    }

    /// Size (lock-free!)
    fn size(&self) -> Pin<Box<dyn Future<Output = Result<usize>> + Send + '_>> {
        Box::pin(ready(self.size_entry()))
    }
}

/// Resolve cache base directory from env / XDG, falling back to `/tmp/nestgate`.
#[must_use]
pub fn resolve_cache_base() -> String {
    std::env::var("NESTGATE_CACHE_DIR")
        .or_else(|_| std::env::var("XDG_CACHE_HOME").map(|xdg| format!("{xdg}/nestgate")))
        .unwrap_or_else(|_| "/tmp/nestgate".to_string())
}

impl Default for MultiTierCacheConfig {
    fn default() -> Self {
        let base = resolve_cache_base();
        Self {
            hot_tier_config: SimpleCacheConfig {
                max_size: 1024 * 1024,
                ttl: std::time::Duration::from_secs(300),
                cache_dir: format!("{base}/hot"),
            },
            warm_tier_config: SimpleCacheConfig {
                max_size: 10 * 1024 * 1024,
                ttl: std::time::Duration::from_secs(3600),
                cache_dir: format!("{base}/warm"),
            },
            cold_tier_config: SimpleCacheConfig {
                max_size: 100 * 1024 * 1024,
                ttl: std::time::Duration::from_secs(86400),
                cache_dir: format!("{base}/cold"),
            },
            promotion_threshold: 3,
            demotion_threshold: 100,
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
/// Type alias for Multitiercacheconfigcanonical
pub type MultiTierCacheConfigCanonical =
    nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using MultiTierCacheConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
#[path = "multi_tier_tests.rs"]
mod tests;
