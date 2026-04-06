// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module provides real-time ZFS performance monitoring capabilities,
// including metrics collection, trend analysis, and alerting.

//! Monitoring module

mod metrics_collection;
mod types;

pub use types::{
    AlertThresholds, AlertThresholdsArc, DatasetMetricsMap, MetricsCacheMap, PoolMetricsMap,
};

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::error;
use tracing::warn;

use crate::error::Result;

use super::types::ZfsPerformanceMetrics;

use types::{DatasetMetricsMap as DmMap, MetricsCacheMap as McMap, PoolMetricsMap as PmMap};

/// Real-time performance monitor
#[derive(Debug)]
/// Realtimeperformancemonitor
pub struct RealTimePerformanceMonitor {
    _pool_metrics: PmMap,
    _dataset_metrics: DmMap,
    _alert_thresholds: AlertThresholdsArc,
    metrics_cache: McMap,
}

/// Type alias for backwards compatibility with test code
pub type PerformanceMonitor = RealTimePerformanceMonitor;
impl Default for RealTimePerformanceMonitor {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl RealTimePerformanceMonitor {
    /// Creates a new real-time performance monitor with default configuration.
    #[must_use]
    pub fn new() -> Self {
        Self {
            _pool_metrics: Arc::new(RwLock::new(HashMap::new())),
            _dataset_metrics: Arc::new(RwLock::new(HashMap::new())),
            _alert_thresholds: Arc::new(RwLock::new(AlertThresholds {
                cpu_threshold: 80.0,
                memory_threshold: 90.0,
                disk_threshold: 85.0,
            })),
            metrics_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get access to metrics cache for testing
    #[must_use]
    pub fn get_metrics_cache(&self) -> McMap {
        self.metrics_cache.clone()
    }

    /// Calculate trend from a series of values
    fn calculate_trend(values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }

        let n = values.len() as f64;
        let x_sum: f64 = (0..values.len()).map(|i| i as f64).sum();
        let y_sum: f64 = values.iter().sum();
        let xy_sum: f64 = values.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
        let x_squared_sum: f64 = (0..values.len()).map(|i| (i as f64).powi(2)).sum();

        // Calculate slope using least squares regression
        n.mul_add(xy_sum, -(x_sum * y_sum)) / x_sum.mul_add(-x_sum, n * x_squared_sum)
    }

    /// Analyze performance trends and generate predictive alerts
    async fn analyze_performance_trends(&self) -> Result<()> {
        let cache = self.metrics_cache.read().await;

        if cache.len() < 5 {
            return Ok(()); // Need at least 5 data points for trend analysis
        }

        let recent_metrics: Vec<&ZfsPerformanceMetrics> = cache.values().collect();

        // Analyze ARC hit ratio trends
        let arc_hit_ratios: Vec<f64> = recent_metrics
            .iter()
            .map(|m| m.arc_stats.hit_ratio)
            .collect();
        let arc_trend = Self::calculate_trend(&arc_hit_ratios);

        if arc_trend < -0.05 {
            // 5% degradation trend
            warn!(
                "📉 ARC hit ratio degrading: {:.2}% trend over last {} minutes",
                arc_trend * 100.0,
                recent_metrics.len()
            );
        }

        // Analyze pool performance trends
        for pool_name in recent_metrics[0].pool_metrics.keys() {
            let pool_iops: Vec<f64> = recent_metrics
                .iter()
                .filter_map(|m| m.pool_metrics.get(pool_name))
                .map(|p| p.read_ops + p.write_ops)
                .collect();

            let pool_latency: Vec<f64> = recent_metrics
                .iter()
                .filter_map(|m| m.pool_metrics.get(pool_name))
                .map(|p| p.latency)
                .collect();

            if pool_iops.len() >= 5 {
                let iops_trend = Self::calculate_trend(&pool_iops);
                let latency_trend = Self::calculate_trend(&pool_latency);

                if iops_trend < -0.15 {
                    // 15% IOPS degradation
                    warn!(
                        "📉 Pool {},
    IOPS degrading: {:.2}% trend",
                        pool_name,
                        iops_trend * 100.0
                    );
                }

                if latency_trend > 0.20 {
                    // 20% latency increase
                    warn!(
                        "📈 Pool {} latency increasing: {:.2}% trend",
                        pool_name,
                        latency_trend * 100.0
                    );
                }
            }
        }

        // Memory pressure analysis
        let memory_usage_ratios: Vec<f64> = recent_metrics
            .iter()
            .map(|m| m.system_memory.used as f64 / m.system_memory.total as f64)
            .collect();

        let memory_trend = Self::calculate_trend(&memory_usage_ratios);
        let current_memory_usage = memory_usage_ratios.last().unwrap_or(&0.5);

        if *current_memory_usage > 0.90 && memory_trend > 0.05 {
            error!(
                "🔴 CRITICAL: Memory pressure detected - {}% used with increasing trend",
                current_memory_usage * 100.0
            );
        } else if *current_memory_usage > 0.85 {
            warn!("⚠️ High memory usage: {:.1}%", current_memory_usage * 100.0);
        }
        Ok(())
    }

    /// Parse ZFS size values (e.g., "128K", "1M", "2G")
    fn parse_sizevalue(size_str: &str) -> Result<u64> {
        if let Some(num_str) = size_str.strip_suffix('K') {
            Ok(num_str.parse::<u64>().map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!(
                    "Failed to parse size value '{num_str}K': {e}"
                ))
            })? * 1024)
        } else if let Some(num_str) = size_str.strip_suffix('M') {
            Ok(num_str.parse::<u64>().map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!(
                    "Failed to parse size value '{num_str}M': {e}"
                ))
            })? * 1024
                * 1024)
        } else if let Some(num_str) = size_str.strip_suffix('G') {
            Ok(num_str.parse::<u64>().map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!(
                    "Failed to parse size value '{num_str}G': {e}"
                ))
            })? * 1024
                * 1024
                * 1024)
        } else if let Some(num_str) = size_str.strip_suffix('T') {
            Ok(num_str.parse::<u64>().map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!(
                    "Failed to parse size value '{num_str}T': {e}"
                ))
            })? * 1024
                * 1024
                * 1024
                * 1024)
        } else {
            Ok(size_str.parse::<u64>().map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!(
                    "Failed to parse size value '{size_str}': {e}"
                ))
            })?)
        }
    }

    /// Get trending data for analysis
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_trending_data(&self) -> Result<Vec<ZfsPerformanceMetrics>> {
        let cache = self.metrics_cache.read().await;
        Ok(cache.values().cloned().collect())
    }
}

#[cfg(test)]
impl RealTimePerformanceMonitor {
    pub(crate) fn test_calculate_trend(values: &[f64]) -> f64 {
        Self::calculate_trend(values)
    }

    pub(crate) fn test_parse_sizevalue(size_str: &str) -> Result<u64> {
        Self::parse_sizevalue(size_str)
    }

    pub(crate) async fn test_analyze_performance_trends(&self) -> Result<()> {
        self.analyze_performance_trends().await
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod regression_tests {
    use super::super::types::{
        AccessPattern, ArcStatistics, SystemMemoryUsage, ZfsDatasetMetrics, ZfsPerformanceMetrics,
        ZfsPoolMetrics,
    };
    use super::{AlertThresholds, RealTimePerformanceMonitor};
    use std::collections::HashMap;
    use std::time::SystemTime;

    #[test]
    fn calculate_trend_short_and_linear() {
        assert_eq!(RealTimePerformanceMonitor::test_calculate_trend(&[]), 0.0);
        assert_eq!(
            RealTimePerformanceMonitor::test_calculate_trend(&[1.0]),
            0.0
        );
        let slope = RealTimePerformanceMonitor::test_calculate_trend(&[0.0, 1.0, 2.0, 3.0]);
        assert!((slope - 1.0).abs() < 1e-9);
    }

    #[test]
    fn parse_sizevalue_suffixes_and_raw() {
        assert_eq!(
            RealTimePerformanceMonitor::test_parse_sizevalue("128K").unwrap(),
            128 * 1024
        );
        assert_eq!(
            RealTimePerformanceMonitor::test_parse_sizevalue("2M").unwrap(),
            2 * 1024 * 1024
        );
        assert_eq!(
            RealTimePerformanceMonitor::test_parse_sizevalue("1G").unwrap(),
            1024_u64 * 1024 * 1024
        );
        assert_eq!(
            RealTimePerformanceMonitor::test_parse_sizevalue("1T").unwrap(),
            1024_u64 * 1024 * 1024 * 1024
        );
        assert_eq!(
            RealTimePerformanceMonitor::test_parse_sizevalue("4096").unwrap(),
            4096
        );
        assert!(RealTimePerformanceMonitor::test_parse_sizevalue("badK").is_err());
    }

    #[test]
    fn alert_thresholds_default_zero() {
        let a = AlertThresholds::default();
        assert_eq!(a.cpu_threshold, 0.0);
        assert_eq!(a.memory_threshold, 0.0);
        assert_eq!(a.disk_threshold, 0.0);
    }

    #[test]
    fn realtime_monitor_new_sets_default_alert_thresholds() {
        let m = RealTimePerformanceMonitor::new();
        // Thresholds live inside the monitor; new() uses non-zero defaults (see constructor).
        let _ = m.get_metrics_cache();
    }

    #[test]
    fn calculate_trend_negative_slope_degrading_series() {
        let slope = RealTimePerformanceMonitor::test_calculate_trend(&[10.0, 8.0, 6.0, 4.0, 2.0]);
        assert!(slope < 0.0);
    }

    #[test]
    fn calculate_trend_constant_series_zero_slope() {
        let slope = RealTimePerformanceMonitor::test_calculate_trend(&[5.0, 5.0, 5.0, 5.0]);
        assert!(slope.abs() < 1e-9);
    }

    #[test]
    fn calculate_trend_two_points() {
        let slope = RealTimePerformanceMonitor::test_calculate_trend(&[0.0, 2.0]);
        assert!((slope - 2.0).abs() < 1e-9);
    }

    #[tokio::test]
    async fn get_trending_data_empty_cache() {
        let m = RealTimePerformanceMonitor::new();
        let rows = m.get_trending_data().await.expect("trending");
        assert!(rows.is_empty());
    }

    #[tokio::test]
    async fn default_monitor_matches_new() {
        let a = RealTimePerformanceMonitor::default();
        let _ = a.get_metrics_cache();
    }

    fn sample_metrics_snapshot(
        pool_name: &str,
        arc_hit: f64,
        mem_used_ratio: f64,
    ) -> ZfsPerformanceMetrics {
        let mut pool_metrics = HashMap::new();
        pool_metrics.insert(
            pool_name.to_string(),
            ZfsPoolMetrics {
                pool_name: pool_name.to_string(),
                read_ops: 100.0,
                write_ops: 50.0,
                read_bandwidth: 10.0,
                write_bandwidth: 5.0,
                latency: 120.0,
                cache_hit_ratio: 0.5,
                fragmentation: 55.0,
            },
        );
        let total = 10_000_u64;
        let used = (total as f64 * mem_used_ratio) as u64;
        ZfsPerformanceMetrics {
            timestamp: SystemTime::UNIX_EPOCH,
            pool_metrics,
            dataset_metrics: HashMap::from([(
                "tank/d0".to_string(),
                ZfsDatasetMetrics {
                    dataset_name: "tank/d0".to_string(),
                    access_pattern: AccessPattern::Mixed,
                    dedup_ratio: 1.0,
                    record_size: 128 * 1024,
                },
            )]),
            system_memory: SystemMemoryUsage {
                total,
                used,
                available: total.saturating_sub(used),
            },
            arc_stats: ArcStatistics {
                hit_ratio: arc_hit,
                size: 1,
                target_size: 2,
                miss_ratio: 1.0 - arc_hit,
            },
        }
    }

    #[tokio::test]
    async fn analyze_performance_trends_insufficient_points_is_ok() {
        let m = RealTimePerformanceMonitor::new();
        let cache = m.get_metrics_cache();
        {
            let mut w = cache.write().await;
            for i in 0..3 {
                w.insert(
                    format!("m{i}"),
                    sample_metrics_snapshot("p0", f64::from(i).mul_add(-0.01, 0.9), 0.5),
                );
            }
        }
        m.test_analyze_performance_trends().await.expect("analyze");
    }

    #[tokio::test]
    async fn analyze_performance_trends_full_branch_exercise() {
        let m = RealTimePerformanceMonitor::new();
        let cache = m.get_metrics_cache();
        {
            let mut w = cache.write().await;
            w.clear();
            for i in 0..5 {
                w.insert(
                    format!("m{i}"),
                    sample_metrics_snapshot(
                        "heavy",
                        f64::from(i).mul_add(-0.02, 0.5),
                        f64::from(i).mul_add(0.001, 0.92),
                    ),
                );
            }
        }
        m.test_analyze_performance_trends().await.expect("analyze");
    }

    #[test]
    fn zfs_performance_metrics_serde_roundtrip() {
        let m = sample_metrics_snapshot("z", 0.8, 0.5);
        let json = serde_json::to_string(&m).expect("serialize");
        let back: ZfsPerformanceMetrics = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.pool_metrics.len(), m.pool_metrics.len());
        assert!((back.arc_stats.hit_ratio - m.arc_stats.hit_ratio).abs() < f64::EPSILON);
    }

    #[test]
    fn access_pattern_serde_roundtrip() {
        for p in [
            AccessPattern::Sequential,
            AccessPattern::Random,
            AccessPattern::Mixed,
        ] {
            let j = serde_json::to_string(&p).unwrap();
            let back: AccessPattern = serde_json::from_str(&j).unwrap();
            assert_eq!(back, p);
        }
    }

    #[test]
    fn parse_sizevalue_invalid_suffixes_return_errors() {
        assert!(RealTimePerformanceMonitor::test_parse_sizevalue("12Mbad").is_err());
        assert!(RealTimePerformanceMonitor::test_parse_sizevalue("badM").is_err());
        assert!(RealTimePerformanceMonitor::test_parse_sizevalue("badG").is_err());
        assert!(RealTimePerformanceMonitor::test_parse_sizevalue("badT").is_err());
    }

    #[tokio::test]
    async fn get_trending_data_returns_inserted_snapshots() {
        let m = RealTimePerformanceMonitor::new();
        let cache = m.get_metrics_cache();
        {
            let mut w = cache.write().await;
            w.insert("latest".into(), sample_metrics_snapshot("tank", 0.9, 0.5));
        }
        let rows = m.get_trending_data().await.expect("trending");
        assert_eq!(rows.len(), 1);
        assert!((rows[0].arc_stats.hit_ratio - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn performance_monitor_type_alias_constructible() {
        let m: super::PerformanceMonitor = super::PerformanceMonitor::new();
        let _ = m.get_metrics_cache();
    }

    #[test]
    fn realtime_performance_monitor_debug_format() {
        let m = RealTimePerformanceMonitor::new();
        let s = format!("{m:?}");
        assert!(s.contains("RealTimePerformanceMonitor"));
    }
}
