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

/// Storage tier for caching - use unified definition from `nestgate-types`
pub use nestgate_types::unified_enums::StorageTier;

/// Extension methods for [`StorageTier`] used by the cache layer (cannot use inherent impl on a foreign type).
pub trait CacheStorageTierExt {
    /// Get tier priority (lower number = higher priority)
    #[must_use]
    fn priority(&self) -> u8;

    /// Get typical access time for this tier
    #[must_use]
    fn typical_access_time(&self) -> Duration;
}

impl CacheStorageTierExt for StorageTier {
    fn priority(&self) -> u8 {
        match self {
            Self::Hot => 0,
            Self::Warm => 1,
            Self::Cool => 2,
            Self::Cold => 3,
            Self::Frozen => 4,
            Self::Custom(_) => 5,
        }
    }

    fn typical_access_time(&self) -> Duration {
        match self {
            Self::Hot => Duration::from_micros(100),
            Self::Warm => Duration::from_millis(1),
            Self::Cool => Duration::from_millis(10),
            Self::Cold => Duration::from_millis(100),
            Self::Frozen => Duration::from_secs(10),
            Self::Custom(_) => Duration::from_millis(50),
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
            Self::None => write!(f, "none"),
            Self::ReadOnly => write!(f, "read-only"),
            Self::WriteThrough => write!(f, "write-through"),
            Self::WriteBack => write!(f, "write-back"),
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
        tier_access_times.insert(StorageTier::Cool, StorageTier::Cool.typical_access_time());

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
    #[expect(
        clippy::cast_precision_loss,
        reason = "Hit ratio metric; tier hit/miss counts are within practical f64 precision"
    )]
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
    pub const fn total_items(&self) -> usize {
        self.hot_tier_items + self.warm_tier_items + self.cold_tier_items
    }

    /// Get total size across all tiers
    #[must_use]
    pub const fn total_size_bytes(&self) -> u64 {
        self.hot_tier_size_bytes + self.warm_tier_size_bytes + self.cold_tier_size_bytes
    }

    /// Get total evictions across all tiers
    #[must_use]
    pub const fn total_evictions(&self) -> u64 {
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
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Midpoint of chrono/std nanosecond durations for running average"
    )]
    pub fn record_access_time(&mut self, tier: StorageTier, access_time: Duration) {
        // Update running average
        if let Some(current_avg) = self.tier_access_times.get(&tier) {
            let new_avg = Duration::from_nanos(u128::midpoint(
                current_avg.as_nanos(),
                access_time.as_nanos(),
            ) as u64);
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
    #[expect(
        clippy::cast_precision_loss,
        reason = "Efficiency metrics use floating ratios over bounded operation windows"
    )]
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
            if recent_variance < 0.1 { 5.0 } else { 0.0 }
        } else {
            0.0
        };

        (base_score + consistency_bonus).min(100.0)
    }

    /// Calculate Variance
    #[expect(
        clippy::cast_precision_loss,
        reason = "Variance of hit ratios over bounded sliding windows"
    )]
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

        window_hit_ratios
            .iter()
            .map(|ratio| (ratio - mean).powi(2))
            .sum::<f64>()
            / (window_hit_ratios.len() as f64)
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
        self.ttl.is_some_and(|ttl| {
            let expiry_time = self.created_at + chrono::Duration::from_std(ttl).unwrap_or_default();
            chrono::Utc::now() > expiry_time
        })
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
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp
    )]

    use super::*;

    // ==================== StorageTier Tests ====================

    #[test]
    fn test_storage_tier_priority() {
        assert_eq!(StorageTier::Hot.priority(), 0);
        assert_eq!(StorageTier::Warm.priority(), 1);
        assert_eq!(StorageTier::Cool.priority(), 2);
        assert_eq!(StorageTier::Cold.priority(), 3);
        assert_eq!(StorageTier::Frozen.priority(), 4);
    }

    #[test]
    fn test_storage_tier_priority_ordering() {
        assert!(StorageTier::Hot.priority() < StorageTier::Warm.priority());
        assert!(StorageTier::Warm.priority() < StorageTier::Cool.priority());
        assert!(StorageTier::Cool.priority() < StorageTier::Cold.priority());
        assert!(StorageTier::Cold.priority() < StorageTier::Frozen.priority());
    }

    #[test]
    fn test_storage_tier_typical_access_time() {
        assert_eq!(
            StorageTier::Hot.typical_access_time(),
            Duration::from_micros(100)
        );
        assert_eq!(
            StorageTier::Warm.typical_access_time(),
            Duration::from_millis(1)
        );
        assert_eq!(
            StorageTier::Cool.typical_access_time(),
            Duration::from_millis(10)
        );
        assert_eq!(
            StorageTier::Cold.typical_access_time(),
            Duration::from_millis(100)
        );
        assert_eq!(
            StorageTier::Frozen.typical_access_time(),
            Duration::from_secs(10)
        );
    }

    #[test]
    fn test_storage_tier_access_time_ordering() {
        assert!(StorageTier::Hot.typical_access_time() < StorageTier::Warm.typical_access_time());
        assert!(StorageTier::Warm.typical_access_time() < StorageTier::Cool.typical_access_time());
        assert!(StorageTier::Cool.typical_access_time() < StorageTier::Cold.typical_access_time());
        assert!(
            StorageTier::Cold.typical_access_time() < StorageTier::Frozen.typical_access_time()
        );
    }

    #[test]
    fn test_storage_tier_custom_ext() {
        let custom = StorageTier::Custom("edge-cache".to_string());
        assert_eq!(custom.priority(), 5);
        assert_eq!(custom.typical_access_time(), Duration::from_millis(50));
    }

    #[test]
    fn test_storage_tier_default_is_hot() {
        assert_eq!(StorageTier::default(), StorageTier::Hot);
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

    #[test]
    fn test_cache_policy_serde_roundtrip() {
        for policy in [
            CachePolicy::None,
            CachePolicy::ReadOnly,
            CachePolicy::WriteThrough,
            CachePolicy::WriteBack,
        ] {
            let json = serde_json::to_string(&policy).expect("serialize CachePolicy");
            let back: CachePolicy = serde_json::from_str(&json).expect("deserialize CachePolicy");
            assert_eq!(policy, back);
        }
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
        // Default Hot tier = 100µs. Running average: (old + new) / 2
        // After 10ms: (100µs + 10ms) / 2 = 5050µs
        stats.record_access_time(StorageTier::Hot, Duration::from_millis(10));
        // After 20ms: (5050µs + 20ms) / 2 = 12525µs
        stats.record_access_time(StorageTier::Hot, Duration::from_millis(20));

        let avg = stats
            .tier_access_times
            .get(&StorageTier::Hot)
            .expect("Should have access time");
        assert_eq!(*avg, Duration::from_micros(12525));
    }

    #[test]
    fn test_cache_stats_record_access_time_tier_not_in_defaults() {
        let mut stats = CacheStats::default();
        assert!(!stats.tier_access_times.contains_key(&StorageTier::Cold));
        stats.record_access_time(StorageTier::Cold, Duration::from_millis(3));
        assert_eq!(
            *stats
                .tier_access_times
                .get(&StorageTier::Cold)
                .expect("cold tier inserted"),
            Duration::from_millis(3)
        );
    }

    #[test]
    fn test_cache_stats_serde_roundtrip() {
        let stats = CacheStats {
            hits: 1,
            misses: 2,
            hot_tier_items: 3,
            warm_tier_items: 4,
            cold_tier_items: 5,
            hot_tier_size_bytes: 6,
            warm_tier_size_bytes: 7,
            cold_tier_size_bytes: 8,
            hot_tier_evictions: 9,
            warm_tier_evictions: 10,
            cold_tier_evictions: 11,
            tier_access_times: HashMap::from([(StorageTier::Hot, Duration::from_nanos(1))]),
            efficiency_metrics: EfficiencyMetrics::default(),
        };
        let json = serde_json::to_string(&stats).expect("serialize CacheStats");
        let back: CacheStats = serde_json::from_str(&json).expect("deserialize CacheStats");
        assert_eq!(stats.hits, back.hits);
        assert_eq!(stats.misses, back.misses);
        assert_eq!(stats.tier_access_times, back.tier_access_times);
        assert_eq!(
            stats.efficiency_metrics.moving_hit_ratio,
            back.efficiency_metrics.moving_hit_ratio
        );
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

    #[test]
    fn test_efficiency_metrics_serde_roundtrip() {
        let mut metrics = EfficiencyMetrics::default();
        metrics.update_hit();
        metrics.update_miss();
        let json = serde_json::to_string(&metrics).expect("serialize EfficiencyMetrics");
        let back: EfficiencyMetrics = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(metrics.moving_hit_ratio, back.moving_hit_ratio);
        assert_eq!(metrics.peak_hit_ratio, back.peak_hit_ratio);
        assert_eq!(metrics.effectiveness_score, back.effectiveness_score);
    }

    #[test]
    fn test_efficiency_metrics_consistency_bonus_at_100_operations() {
        let mut metrics = EfficiencyMetrics::default();
        for _ in 0..100 {
            metrics.update_hit();
        }
        assert_eq!(metrics.last_operations.len(), 100);
        assert!((metrics.effectiveness_score - 100.0).abs() < f64::EPSILON);
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
    fn test_cache_entry_new_empty_data() {
        let entry = CacheEntry::new("k".to_string(), Vec::new(), StorageTier::Warm);
        assert_eq!(entry.size, 0);
        assert!(entry.data.is_empty());
    }

    #[test]
    fn test_cache_entry_is_expired_past_ttl() {
        let mut entry = CacheEntry::new("key".to_string(), vec![1], StorageTier::Hot);
        entry.created_at = chrono::DateTime::from_timestamp(0, 0).expect("epoch");
        entry.ttl = Some(Duration::from_secs(1));
        assert!(entry.is_expired());
    }

    #[test]
    fn test_cache_entry_serde_roundtrip() {
        let entry = CacheEntry::new("k".to_string(), vec![0, 255], StorageTier::Cool);
        let json = serde_json::to_string(&entry).expect("serialize CacheEntry");
        let back: CacheEntry = serde_json::from_str(&json).expect("deserialize CacheEntry");
        assert_eq!(entry.key, back.key);
        assert_eq!(entry.data, back.data);
        assert_eq!(entry.size, back.size);
        assert_eq!(entry.tier, back.tier);
        assert_eq!(entry.ttl, back.ttl);
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
