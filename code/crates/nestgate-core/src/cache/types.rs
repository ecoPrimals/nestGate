// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Cache Types and Configuration
/// Core types, enums, and configuration structures for the caching system.
use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
// Import EvictionPolicy from services module

// Internal re-exports will be handled by the module system

/// Storage tier for caching - use canonical definition
pub use crate::canonical_types::StorageTier;
impl StorageTier {
    /// Get tier priority (lower number = higher priority)
    #[must_use]
    pub fn priority(&self) -> u8 {
        match self {
            Self::Cache => 0, // Ultra-fast cache has highest priority
            Self::Hot => 1,
            Self::Warm => 2,
            Self::Cold => 3,
            Self::Archive => 4, // Archive has lowest priority
        }
    }

    /// Get typical access time for this tier
    #[must_use]
    pub fn typical_access_time(&self) -> Duration {
        match self {
            StorageTier::Cache => Duration::from_micros(100), // Ultra-fast cache
            StorageTier::Hot => Duration::from_millis(1),
            StorageTier::Warm => Duration::from_millis(10),
            StorageTier::Cold => Duration::from_millis(100),
            StorageTier::Archive => Duration::from_secs(10), // Archive can be very slow
        }
    }
}

/// Cache policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Cachepolicy
pub enum CachePolicy {
    /// No caching
    None,
    /// Read-only caching
    ReadOnly,
    /// Write-through caching (writes go to both cache and backing store)
    #[default]
    /// Writethrough
    WriteThrough,
    /// Write-back caching (writes go to cache, then are flushed to backing store)
    WriteBack,
}
impl std::fmt::Display for CachePolicy {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CachePolicy::None => write!(f, "none"),
            CachePolicy::ReadOnly => write!(f, "read-only"),
            CachePolicy::WriteThrough => write!(f, "write-through"),
            CachePolicy::WriteBack => write!(f, "write-back"),
        }
    }
}

/// Use this instead of the deprecated `CacheConfig`
// All duplicate UnifiedCacheConfig implementations removed
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cachestats
pub struct CacheStats {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Number of items in hot tier
    pub hot_tier_items: usize,
    /// Number of items in warm tier
    pub warm_tier_items: usize,
    /// Number of items in cold tier
    pub cold_tier_items: usize,
    /// Total size of hot tier in bytes
    pub hot_tier_size_bytes: u64,
    /// Total size of warm tier in bytes
    pub warm_tier_size_bytes: u64,
    /// Total size of cold tier in bytes
    pub cold_tier_size_bytes: u64,
    /// Number of evictions from hot tier
    pub hot_tier_evictions: u64,
    /// Number of evictions from warm tier
    pub warm_tier_evictions: u64,
    /// Number of evictions from cold tier
    pub cold_tier_evictions: u64,
    /// Average access time per tier
    pub tier_access_times: HashMap<StorageTier, Duration>,
    /// Cache efficiency metrics
    pub efficiency_metrics: EfficiencyMetrics,
}
impl Default for CacheStats {
    /// Returns the default instance
    fn default() -> Self {
        let mut tier_access_times = HashMap::new();
        tier_access_times.insert(StorageTier::Hot, StorageTier::Hot.typical_access_time());
        tier_access_times.insert(StorageTier::Warm, StorageTier::Warm.typical_access_time());
        tier_access_times.insert(StorageTier::Cold, StorageTier::Cold.typical_access_time());

        Self {
            hits: 0,
            misses: 0,
            hot_tier_items: 0,
            warm_tier_items: 0,
            cold_tier_items: 0,
            hot_tier_size_bytes: 0,
            warm_tier_size_bytes: 0,
            cold_tier_size_bytes: 0,
            hot_tier_evictions: 0,
            warm_tier_evictions: 0,
            cold_tier_evictions: 0,
            tier_access_times,
            efficiency_metrics: EfficiencyMetrics::default(),
        }
    }
}

impl CacheStats {
    /// Calculate hit ratio (0.0 to 1.0)
    #[must_use]
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Get total number of items across all tiers
    #[must_use]
    pub fn total_items(&self) -> usize {
        self.hot_tier_items + self.warm_tier_items + self.cold_tier_items
    }

    /// Get total size across all tiers
    #[must_use]
    pub fn total_size_bytes(&self) -> u64 {
        self.hot_tier_size_bytes + self.warm_tier_size_bytes + self.cold_tier_size_bytes
    }

    /// Get total evictions across all tiers
    #[must_use]
    pub fn total_evictions(&self) -> u64 {
        self.hot_tier_evictions + self.warm_tier_evictions + self.cold_tier_evictions
    }

    /// Update hit count
    pub fn record_hit(&mut self) {
        self.hits += 1;
        self.efficiency_metrics.update_hit();
    }

    /// Update miss count
    pub fn record_miss(&mut self) {
        self.misses += 1;
        self.efficiency_metrics.update_miss();
    }

    /// Record access time for a tier
    pub fn record_access_time(&mut self, tier: StorageTier, access_time: Duration) {
        // Update running average
        if let Some(current_avg) = self.tier_access_times.get(&tier) {
            let new_avg = Duration::from_nanos(
                ((current_avg.as_nanos() + access_time.as_nanos()) / 2) as u64,
            );
            self.tier_access_times.insert(tier, new_avg);
        } else {
            self.tier_access_times.insert(tier, access_time);
        }
    }
}

/// Cache efficiency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Efficiencymetrics
pub struct EfficiencyMetrics {
    /// Moving average hit ratio over last N operations
    pub moving_hit_ratio: f64,
    /// Peak hit ratio achieved
    pub peak_hit_ratio: f64,
    /// Average response time
    pub avg_response_time: Duration,
    /// Cache effectiveness score (0.0 to 100.0)
    pub effectiveness_score: f64,
    /// Last N operations for moving average
    last_operations: Vec<bool>, // true = hit, false = miss
    max_operations_tracked: usize,
}
impl Default for EfficiencyMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            moving_hit_ratio: 0.0,
            peak_hit_ratio: 0.0,
            avg_response_time: Duration::from_millis(0),
            effectiveness_score: 0.0,
            last_operations: Vec::with_capacity(1000),
            max_operations_tracked: 1000,
        }
    }
}

impl EfficiencyMetrics {
    /// Update metrics with a cache hit
    pub fn update_hit(&mut self) {
        self.add_operation(true);
        self.recalculate_metrics();
    }

    /// Update metrics with a cache miss
    pub fn update_miss(&mut self) {
        self.add_operation(false);
        self.recalculate_metrics();
    }

    /// Add Operation
    fn add_operation(&mut self, is_hit: bool) {
        if self.last_operations.len() >= self.max_operations_tracked {
            self.last_operations.remove(0);
        }
        self.last_operations.push(is_hit);
    }

    /// Recalculate Metrics
    fn recalculate_metrics(&mut self) {
        if self.last_operations.is_empty() {
            return;
        }

        // Calculate moving hit ratio
        let hits = self.last_operations.iter().filter(|&&h| h).count();
        self.moving_hit_ratio = hits as f64 / self.last_operations.len() as f64;

        // Update peak hit ratio
        if self.moving_hit_ratio > self.peak_hit_ratio {
            self.peak_hit_ratio = self.moving_hit_ratio;
        }

        // Calculate effectiveness score (weighted by various factors)
        self.effectiveness_score = self.calculate_effectiveness_score();
    }

    /// Calculate Effectiveness Score
    fn calculate_effectiveness_score(&self) -> f64 {
        // Simple effectiveness calculation (can be enhanced)
        let base_score = self.moving_hit_ratio * 100.0;

        // Bonus for consistent performance
        let consistency_bonus = if self.last_operations.len() >= 100 {
            let recent_variance = self.calculate_variance();
            if recent_variance < 0.1 {
                5.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        (base_score + consistency_bonus).min(100.0)
    }

    /// Calculate Variance
    fn calculate_variance(&self) -> f64 {
        if self.last_operations.len() < 10 {
            return 1.0; // High variance for small samples
        }

        // Calculate variance of hit ratios in sliding windows
        let window_size = 10;
        let mut window_hit_ratios = Vec::new();

        for window_start in 0..=(self.last_operations.len().saturating_sub(window_size)) {
            let window_end = (window_start + window_size).min(self.last_operations.len());
            let window = &self.last_operations[window_start..window_end];
            let hits = window.iter().filter(|&&h| h).count();
            let hit_ratio = hits as f64 / window.len() as f64;
            window_hit_ratios.push(hit_ratio);
        }

        if window_hit_ratios.len() < 2 {
            return 0.0;
        }

        let mean = window_hit_ratios.iter().sum::<f64>() / (window_hit_ratios.len() as f64);
        let variance = window_hit_ratios
            .iter()
            .map(|ratio| (ratio - mean).powi(2))
            .sum::<f64>()
            / (window_hit_ratios.len() as f64);

        variance
    }
}

/// Cache entry with data and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cacheentry
pub struct CacheEntry {
    /// Entry key
    pub key: String,
    /// Actual cached data
    pub data: Vec<u8>,
    /// Entry size in bytes
    pub size: u64,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last access timestamp
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    /// Access count
    pub access_count: u64,
    /// Current storage tier
    pub tier: StorageTier,
    /// Time to live
    pub ttl: Option<Duration>,
}
impl CacheEntry {
    /// Create a new cache entry
    #[must_use]
    pub fn new(key: String, data: Vec<u8>, tier: StorageTier) -> Self {
        let now = chrono::Utc::now();
        let size = data.len() as u64;
        Self {
            key,
            data,
            size,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            tier,
            ttl: None,
        }
    }

    /// Check if entry has expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            let expiry_time = self.created_at + chrono::Duration::from_std(ttl).unwrap_or_default();
            chrono::Utc::now() > expiry_time
        } else {
            false
        }
    }

    /// Update access timestamp and count
    pub fn touch(&mut self) {
        self.accessed_at = chrono::Utc::now();
        self.access_count += 1;
    }

    /// Get age of entry
    #[must_use]
    pub fn age(&self) -> chrono::Duration {
        chrono::Utc::now() - self.created_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== StorageTier Tests ====================

    #[test]
    fn test_storage_tier_priority() {
        assert_eq!(StorageTier::Cache.priority(), 0);
        assert_eq!(StorageTier::Hot.priority(), 1);
        assert_eq!(StorageTier::Warm.priority(), 2);
        assert_eq!(StorageTier::Cold.priority(), 3);
        assert_eq!(StorageTier::Archive.priority(), 4);
    }

    #[test]
    fn test_storage_tier_priority_ordering() {
        assert!(StorageTier::Cache.priority() < StorageTier::Hot.priority());
        assert!(StorageTier::Hot.priority() < StorageTier::Warm.priority());
        assert!(StorageTier::Warm.priority() < StorageTier::Cold.priority());
        assert!(StorageTier::Cold.priority() < StorageTier::Archive.priority());
    }

    #[test]
    fn test_storage_tier_typical_access_time() {
        assert_eq!(
            StorageTier::Cache.typical_access_time(),
            Duration::from_micros(100)
        );
        assert_eq!(
            StorageTier::Hot.typical_access_time(),
            Duration::from_millis(1)
        );
        assert_eq!(
            StorageTier::Warm.typical_access_time(),
            Duration::from_millis(10)
        );
        assert_eq!(
            StorageTier::Cold.typical_access_time(),
            Duration::from_millis(100)
        );
        assert_eq!(
            StorageTier::Archive.typical_access_time(),
            Duration::from_secs(10)
        );
    }

    #[test]
    fn test_storage_tier_access_time_ordering() {
        assert!(StorageTier::Cache.typical_access_time() < StorageTier::Hot.typical_access_time());
        assert!(StorageTier::Hot.typical_access_time() < StorageTier::Warm.typical_access_time());
        assert!(StorageTier::Warm.typical_access_time() < StorageTier::Cold.typical_access_time());
        assert!(
            StorageTier::Cold.typical_access_time() < StorageTier::Archive.typical_access_time()
        );
    }

    // ==================== CachePolicy Tests ====================

    #[test]
    fn test_cache_policy_display() {
        assert_eq!(CachePolicy::None.to_string(), "none");
        assert_eq!(CachePolicy::ReadOnly.to_string(), "read-only");
        assert_eq!(CachePolicy::WriteThrough.to_string(), "write-through");
        assert_eq!(CachePolicy::WriteBack.to_string(), "write-back");
    }

    #[test]
    fn test_cache_policy_default() {
        assert_eq!(CachePolicy::default(), CachePolicy::WriteThrough);
    }

    #[test]
    fn test_cache_policy_equality() {
        assert_eq!(CachePolicy::None, CachePolicy::None);
        assert_ne!(CachePolicy::None, CachePolicy::ReadOnly);
    }

    // ==================== CacheStats Tests ====================

    #[test]
    fn test_cache_stats_default() {
        let stats = CacheStats::default();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.hot_tier_items, 0);
        assert_eq!(stats.total_items(), 0);
        assert_eq!(stats.total_size_bytes(), 0);
        assert_eq!(stats.total_evictions(), 0);
    }

    #[test]
    fn test_cache_stats_hit_ratio_zero_operations() {
        let stats = CacheStats::default();
        assert_eq!(stats.hit_ratio(), 0.0);
    }

    #[test]
    fn test_cache_stats_hit_ratio_all_hits() {
        let stats = CacheStats {
            hits: 10,
            misses: 0,
            ..Default::default()
        };
        assert_eq!(stats.hit_ratio(), 1.0);
    }

    #[test]
    fn test_cache_stats_hit_ratio_all_misses() {
        let stats = CacheStats {
            hits: 0,
            misses: 10,
            ..Default::default()
        };
        assert_eq!(stats.hit_ratio(), 0.0);
    }

    #[test]
    fn test_cache_stats_hit_ratio_mixed() {
        let stats = CacheStats {
            hits: 7,
            misses: 3,
            ..Default::default()
        };
        assert!((stats.hit_ratio() - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_cache_stats_total_items() {
        let stats = CacheStats {
            hot_tier_items: 10,
            warm_tier_items: 20,
            cold_tier_items: 30,
            ..Default::default()
        };
        assert_eq!(stats.total_items(), 60);
    }

    #[test]
    fn test_cache_stats_total_size_bytes() {
        let stats = CacheStats {
            hot_tier_size_bytes: 1000,
            warm_tier_size_bytes: 2000,
            cold_tier_size_bytes: 3000,
            ..Default::default()
        };
        assert_eq!(stats.total_size_bytes(), 6000);
    }

    #[test]
    fn test_cache_stats_total_evictions() {
        let stats = CacheStats {
            hot_tier_evictions: 5,
            warm_tier_evictions: 10,
            cold_tier_evictions: 15,
            ..Default::default()
        };
        assert_eq!(stats.total_evictions(), 30);
    }

    #[test]
    fn test_cache_stats_record_hit() {
        let mut stats = CacheStats::default();
        assert_eq!(stats.hits, 0);
        stats.record_hit();
        assert_eq!(stats.hits, 1);
        stats.record_hit();
        assert_eq!(stats.hits, 2);
    }

    #[test]
    fn test_cache_stats_record_miss() {
        let mut stats = CacheStats::default();
        assert_eq!(stats.misses, 0);
        stats.record_miss();
        assert_eq!(stats.misses, 1);
        stats.record_miss();
        assert_eq!(stats.misses, 2);
    }

    #[test]
    fn test_cache_stats_record_access_time() {
        let mut stats = CacheStats::default();
        let access_time = Duration::from_millis(5);
        stats.record_access_time(StorageTier::Hot, access_time);

        let recorded_time = stats.tier_access_times.get(&StorageTier::Hot);
        assert!(recorded_time.is_some());
    }

    #[test]
    fn test_cache_stats_record_access_time_average() {
        let mut stats = CacheStats::default();
        // Default has Hot tier at 1ms
        // First record: (1ms + 10ms) / 2 = 5.5ms
        stats.record_access_time(StorageTier::Hot, Duration::from_millis(10));
        // Second record: (5.5ms + 20ms) / 2 = 12.75ms
        stats.record_access_time(StorageTier::Hot, Duration::from_millis(20));

        let avg = stats
            .tier_access_times
            .get(&StorageTier::Hot)
            .expect("Should have access time");
        // Average should be 12.75ms due to running average calculation
        assert_eq!(*avg, Duration::from_micros(12750));
    }

    // ==================== EfficiencyMetrics Tests ====================

    #[test]
    fn test_efficiency_metrics_default() {
        let metrics = EfficiencyMetrics::default();
        assert_eq!(metrics.moving_hit_ratio, 0.0);
        assert_eq!(metrics.peak_hit_ratio, 0.0);
        assert_eq!(metrics.effectiveness_score, 0.0);
    }

    #[test]
    fn test_efficiency_metrics_update_hit() {
        let mut metrics = EfficiencyMetrics::default();
        metrics.update_hit();
        assert_eq!(metrics.moving_hit_ratio, 1.0);
        assert_eq!(metrics.peak_hit_ratio, 1.0);
    }

    #[test]
    fn test_efficiency_metrics_update_miss() {
        let mut metrics = EfficiencyMetrics::default();
        metrics.update_miss();
        assert_eq!(metrics.moving_hit_ratio, 0.0);
    }

    #[test]
    fn test_efficiency_metrics_mixed_operations() {
        let mut metrics = EfficiencyMetrics::default();
        metrics.update_hit();
        metrics.update_hit();
        metrics.update_miss();
        // 2 hits out of 3 operations = 0.666...
        assert!((metrics.moving_hit_ratio - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_efficiency_metrics_peak_hit_ratio() {
        let mut metrics = EfficiencyMetrics::default();
        metrics.update_hit();
        metrics.update_hit();
        assert_eq!(metrics.peak_hit_ratio, 1.0);

        metrics.update_miss();
        metrics.update_miss();
        // Peak should remain at 1.0 even though current ratio dropped
        assert_eq!(metrics.peak_hit_ratio, 1.0);
    }

    #[test]
    fn test_efficiency_metrics_max_operations_tracking() {
        let mut metrics = EfficiencyMetrics::default();
        // Add more than max_operations_tracked (1000)
        for _ in 0..1100 {
            metrics.update_hit();
        }
        assert_eq!(metrics.last_operations.len(), 1000);
    }

    // ==================== CacheEntry Tests ====================

    #[test]
    fn test_cache_entry_new() {
        let key = "test-key".to_string();
        let data = vec![1, 2, 3, 4, 5];
        let entry = CacheEntry::new(key.clone(), data.clone(), StorageTier::Hot);

        assert_eq!(entry.key, key);
        assert_eq!(entry.data, data);
        assert_eq!(entry.size, 5);
        assert_eq!(entry.access_count, 0);
        assert_eq!(entry.tier, StorageTier::Hot);
        assert!(entry.ttl.is_none());
    }

    #[test]
    fn test_cache_entry_is_expired_no_ttl() {
        let entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_cache_entry_is_expired_with_future_ttl() {
        let mut entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);
        entry.ttl = Some(Duration::from_secs(3600)); // 1 hour
        assert!(!entry.is_expired());
    }

    #[tokio::test]
    async fn test_cache_entry_touch() {
        let mut entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);
        let initial_access_count = entry.access_count;
        let initial_accessed_at = entry.accessed_at;

        // Sleep a tiny bit to ensure time difference (non-blocking, concurrent)
        tokio::time::sleep(Duration::from_millis(10)).await;

        entry.touch();

        assert_eq!(entry.access_count, initial_access_count + 1);
        assert!(entry.accessed_at > initial_accessed_at);
    }

    #[test]
    fn test_cache_entry_touch_multiple_times() {
        let mut entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);

        entry.touch();
        entry.touch();
        entry.touch();

        assert_eq!(entry.access_count, 3);
    }

    #[test]
    fn test_cache_entry_age() {
        let entry = CacheEntry::new("key".to_string(), vec![1, 2, 3], StorageTier::Hot);
        let age = entry.age();

        // Age should be very small (just created)
        assert!(age.num_milliseconds() < 1000);
    }

    #[test]
    fn test_cache_entry_size_calculation() {
        let data = vec![0u8; 1024]; // 1 KB
        let entry = CacheEntry::new("key".to_string(), data, StorageTier::Hot);
        assert_eq!(entry.size, 1024);
    }

    #[test]
    fn test_cache_entry_different_tiers() {
        let entry_hot = CacheEntry::new("key".to_string(), vec![1], StorageTier::Hot);
        let entry_warm = CacheEntry::new("key".to_string(), vec![1], StorageTier::Warm);
        let entry_cold = CacheEntry::new("key".to_string(), vec![1], StorageTier::Cold);

        assert_eq!(entry_hot.tier, StorageTier::Hot);
        assert_eq!(entry_warm.tier, StorageTier::Warm);
        assert_eq!(entry_cold.tier, StorageTier::Cold);
    }
}
