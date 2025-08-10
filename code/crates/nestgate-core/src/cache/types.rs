/// Cache Types and Configuration
/// Core types, enums, and configuration structures for the caching system.
use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Import EvictionPolicy from services module

// Internal re-exports will be handled by the module system

/// Storage tier for caching
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageTier {
    /// Fast access, limited capacity (SSD, RAM)
    Hot,
    /// Moderate access, larger capacity (fast HDD)
    Warm,
    /// Slow access, unlimited capacity (archive storage)
    Cold,
}

impl StorageTier {
    /// Get tier priority (lower number = higher priority)
    pub fn priority(&self) -> u8 {
        match self {
            StorageTier::Hot => 0,
            StorageTier::Warm => 1,
            StorageTier::Cold => 2,
        }
    }

    /// Get typical access time for this tier
    pub fn typical_access_time(&self) -> Duration {
        match self {
            StorageTier::Hot => Duration::from_millis(1),
            StorageTier::Warm => Duration::from_millis(10),
            StorageTier::Cold => Duration::from_millis(100),
        }
    }
}

/// Cache policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CachePolicy {
    /// No caching
    None,
    /// Read-only caching
    ReadOnly,
    /// Write-through caching (writes go to both cache and backing store)
    #[default]
    WriteThrough,
    /// Write-back caching (writes go to cache, then are flushed to backing store)
    WriteBack,
}

impl std::fmt::Display for CachePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CachePolicy::None => write!(f, "none"),
            CachePolicy::ReadOnly => write!(f, "read-only"),
            CachePolicy::WriteThrough => write!(f, "write-through"),
            CachePolicy::WriteBack => write!(f, "write-back"),
        }
    }
}

/// Use this instead of the deprecated CacheConfig
impl crate::unified_types::UnifiedCacheConfig {
    /// High-performance cache configuration optimized for production workloads
    pub fn high_performance() -> Self {
        Self {
            name: "high-perf-cache".to_string(),
            max_size: 2 * 1024 * 1024 * 1024, // 2GB
            eviction_policy: "lru".to_string(),
            ttl_seconds: 1800, // 30 minutes
            cache_dir: "/tmp/nestgate-high-perf-cache".to_string(),
            policy: "write-back".to_string(),
            hot_tier_size: 500_000_000,
            warm_tier_size: 1_000_000_000,
            cold_tier_unlimited: true,
            enable_compression: true,
            compression_level: 3,
            default_ttl_seconds: 1800,
            enable_metrics: true,
            metrics_interval_seconds: 30,
            enable_persistence: false,
            persistence_interval_seconds: 60,
            max_memory_percent: 90.0,
            enable_lru: true,
            concurrent_threads: (num_cpus::get() * 2) as u32,
            ..Default::default()
        }
    }

    /// Development-friendly cache configuration with debugging features
    pub fn development() -> Self {
        Self {
            name: "dev-cache".to_string(),
            max_size: 50 * 1024 * 1024, // 50MB
            eviction_policy: "lru".to_string(),
            ttl_seconds: 7200, // 2 hours
            cache_dir: "/tmp/nestgate-dev-cache".to_string(),
            policy: "write-through".to_string(),
            hot_tier_size: 10_000_000,
            warm_tier_size: 30_000_000,
            cold_tier_unlimited: false,
            enable_compression: false,
            compression_level: 6,
            default_ttl_seconds: 7200,
            enable_metrics: true,
            metrics_interval_seconds: 120,
            enable_persistence: true,
            persistence_path: "/tmp/nestgate-dev-cache.db".to_string(),
            persistence_interval_seconds: 600,
            max_memory_percent: 60.0,
            enable_lru: true,
            concurrent_threads: 2,
            ..Default::default()
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn hit_ratio(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Get total number of items across all tiers
    pub fn total_items(&self) -> usize {
        self.hot_tier_items + self.warm_tier_items + self.cold_tier_items
    }

    /// Get total size across all tiers
    pub fn total_size_bytes(&self) -> u64 {
        self.hot_tier_size_bytes + self.warm_tier_size_bytes + self.cold_tier_size_bytes
    }

    /// Get total evictions across all tiers
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

    fn add_operation(&mut self, is_hit: bool) {
        if self.last_operations.len() >= self.max_operations_tracked {
            self.last_operations.remove(0);
        }
        self.last_operations.push(is_hit);
    }

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

        let mean = window_hit_ratios.iter().sum::<f64>() / window_hit_ratios.len() as f64;
        let variance = window_hit_ratios
            .iter()
            .map(|ratio| (ratio - mean).powi(2))
            .sum::<f64>()
            / window_hit_ratios.len() as f64;

        variance
    }
}

/// Cache entry with data and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn age(&self) -> chrono::Duration {
        chrono::Utc::now() - self.created_at
    }
}
