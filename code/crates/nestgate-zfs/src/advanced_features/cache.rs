// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// ZFS cache (ARC/L2ARC) analysis and performance metrics

use nestgate_core::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[cfg(target_os = "linux")]
fn read_arcstats_text() -> Option<String> {
    std::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").ok()
}

#[cfg(target_os = "linux")]
fn arcstats_named_u64(content: &str, name: &str) -> u64 {
    for line in content.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() >= 3 && f[0] == name {
            return f[2].parse().unwrap_or(0);
        }
    }
    0
}

/// Advanced cache analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cacheanalytics
pub struct CacheAnalytics {
    /// ARC statistics
    pub arc_stats: ArcStats,
    /// L2ARC statistics
    pub l2arc_stats: L2arcStats,
    /// Cache efficiency metrics
    pub efficiency: CacheEfficiency,
}
impl CacheAnalytics {
    /// Analyze cache performance
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn analyze_cache_performance(pool: &str) -> Result<Self> {
        debug!("Analyzing cache performance for pool: {}", pool);

        // Collect ARC statistics
        let arc_stats = ArcStats::collect()?;

        // Collect L2ARC statistics
        let l2arc_stats = L2arcStats::collect()?;

        // Calculate efficiency
        let efficiency = CacheEfficiency::calculate(&arc_stats, &l2arc_stats);

        Ok(Self {
            arc_stats,
            l2arc_stats,
            efficiency,
        })
    }
}

/// ARC (Adaptive Replacement Cache) statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Arcstats
pub struct ArcStats {
    /// ARC size
    pub size: u64,
    /// ARC hit ratio
    pub hit_ratio: f64,
    /// ARC miss ratio
    pub miss_ratio: f64,
}
impl ArcStats {
    /// Collect ARC statistics from `/proc/spl/kstat/zfs/arcstats` when ZFS is present; otherwise zeros.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn collect() -> Result<Self> {
        #[cfg(target_os = "linux")]
        {
            let Some(content) = read_arcstats_text() else {
                return Ok(Self {
                    size: 0,
                    hit_ratio: 0.0,
                    miss_ratio: 0.0,
                });
            };
            let hits = arcstats_named_u64(&content, "hits");
            let misses = arcstats_named_u64(&content, "misses");
            let size = arcstats_named_u64(&content, "size");
            let total = hits.saturating_add(misses);
            let hit_ratio = if total > 0 {
                hits as f64 / total as f64
            } else {
                0.0
            };
            let miss_ratio = if total > 0 {
                misses as f64 / total as f64
            } else {
                0.0
            };
            Ok(Self {
                size,
                hit_ratio,
                miss_ratio,
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            Ok(Self {
                size: 0,
                hit_ratio: 0.0,
                miss_ratio: 0.0,
            })
        }
    }
}

/// L2ARC (Level 2 Adaptive Replacement Cache) statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// `L2Arcstats`
pub struct L2arcStats {
    /// L2ARC size
    pub size: u64,
    /// L2ARC hit ratio
    pub hit_ratio: f64,
    /// L2ARC miss ratio
    pub miss_ratio: f64,
}
impl L2arcStats {
    /// Collect L2ARC statistics from `/proc/spl/kstat/zfs/arcstats` when ZFS is present; otherwise zeros.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn collect() -> Result<Self> {
        #[cfg(target_os = "linux")]
        {
            let Some(content) = read_arcstats_text() else {
                return Ok(Self {
                    size: 0,
                    hit_ratio: 0.0,
                    miss_ratio: 0.0,
                });
            };
            let l2_hits = arcstats_named_u64(&content, "l2_hits");
            let l2_misses = arcstats_named_u64(&content, "l2_misses");
            let size = arcstats_named_u64(&content, "l2_asize");
            let total = l2_hits.saturating_add(l2_misses);
            let hit_ratio = if total > 0 {
                l2_hits as f64 / total as f64
            } else {
                0.0
            };
            let miss_ratio = if total > 0 {
                l2_misses as f64 / total as f64
            } else {
                0.0
            };
            Ok(Self {
                size,
                hit_ratio,
                miss_ratio,
            })
        }
        #[cfg(not(target_os = "linux"))]
        {
            Ok(Self {
                size: 0,
                hit_ratio: 0.0,
                miss_ratio: 0.0,
            })
        }
    }
}

/// Cache efficiency metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cacheefficiency
pub struct CacheEfficiency {
    /// Overall cache efficiency
    pub overall_efficiency: f64,
    /// ARC efficiency
    pub arc_efficiency: f64,
    /// L2ARC efficiency
    pub l2arc_efficiency: f64,
}
impl Default for CacheEfficiency {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl CacheEfficiency {
    /// Creates a new cache efficiency tracker
    #[must_use]
    pub const fn new() -> Self {
        Self {
            overall_efficiency: 0.0,
            arc_efficiency: 0.0,
            l2arc_efficiency: 0.0,
        }
    }

    /// Calculate cache efficiency
    #[must_use]
    pub fn calculate(arc_stats: &ArcStats, l2arc_stats: &L2arcStats) -> Self {
        let arc_efficiency = arc_stats.hit_ratio * 100.0;
        let l2arc_efficiency = l2arc_stats.hit_ratio * 100.0;
        let overall_efficiency = f64::midpoint(arc_efficiency, l2arc_efficiency);

        Self {
            overall_efficiency,
            arc_efficiency: arc_stats.hit_ratio,
            l2arc_efficiency: l2arc_stats.hit_ratio,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arc_and_l2arc_collect_and_cache_efficiency_calculate() {
        let arc = ArcStats::collect().expect("arc");
        let l2 = L2arcStats::collect().expect("l2");
        let eff = CacheEfficiency::calculate(&arc, &l2);
        assert!(eff.overall_efficiency >= 0.0);
        assert_eq!(eff.arc_efficiency, arc.hit_ratio);
        assert_eq!(eff.l2arc_efficiency, l2.hit_ratio);
        let d = CacheEfficiency::default();
        assert_eq!(d.overall_efficiency, 0.0);
        let n = CacheEfficiency::new();
        assert_eq!(n.overall_efficiency, d.overall_efficiency);
        assert_eq!(n.arc_efficiency, d.arc_efficiency);
        assert_eq!(n.l2arc_efficiency, d.l2arc_efficiency);
    }

    #[test]
    fn analyze_cache_performance_roundtrip_serde() {
        let a = CacheAnalytics::analyze_cache_performance("tank").expect("analyze");
        let json = serde_json::to_string(&a).expect("ser");
        let back: CacheAnalytics = serde_json::from_str(&json).expect("de");
        assert_eq!(back.arc_stats.size, a.arc_stats.size);
        assert!(
            (back.efficiency.overall_efficiency - a.efficiency.overall_efficiency).abs() < 1e-9
        );
    }
}
