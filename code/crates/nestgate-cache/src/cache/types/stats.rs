// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Cache statistics and efficiency metrics.

use super::tier::{CacheStorageTierExt, StorageTier};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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

#[cfg(test)]
impl EfficiencyMetrics {
    /// Length of the sliding window of hit/miss operations (tests only).
    #[must_use]
    pub fn last_operations_len(&self) -> usize {
        self.last_operations.len()
    }
}
