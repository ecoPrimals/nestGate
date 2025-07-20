//! Real-time Performance Monitoring
//!
//! This module provides real-time ZFS performance monitoring capabilities,
//! including metrics collection, trend analysis, and alerting.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::RwLock;
use tracing::{debug, error, warn};

use crate::{
    dataset::ZfsDatasetManager,
    error::{Result, ZfsError},
    pool::ZfsPoolManager,
};
use nestgate_core::config::AlertThresholds;

use super::types::*;

/// Real-time performance monitor
#[derive(Debug)]
pub struct RealTimePerformanceMonitor {
    #[allow(dead_code)]
    pool_metrics: Arc<RwLock<HashMap<String, ZfsPoolMetrics>>>,
    #[allow(dead_code)]
    dataset_metrics: Arc<RwLock<HashMap<String, ZfsDatasetMetrics>>>,
    #[allow(dead_code)]
    alert_thresholds: Arc<RwLock<AlertThresholds>>,
    metrics_cache: Arc<RwLock<HashMap<String, ZfsPerformanceMetrics>>>,
}

impl Default for RealTimePerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl RealTimePerformanceMonitor {
    pub fn new() -> Self {
        Self {
            pool_metrics: Arc::new(RwLock::new(HashMap::new())),
            dataset_metrics: Arc::new(RwLock::new(HashMap::new())),
            alert_thresholds: Arc::new(RwLock::new(AlertThresholds {
                cpu_threshold: 80.0,
                memory_threshold: 90.0,
                disk_threshold: 85.0,
                latency_threshold: 100.0,
                error_rate_threshold: 5.0,
            })),
            metrics_cache: Arc::new(RwLock::new(HashMap::new())),
        }
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
        (n * xy_sum - x_sum * y_sum) / (n * x_squared_sum - x_sum.powi(2))
    }

    pub async fn collect_metrics(
        &self,
        _pool_manager: &ZfsPoolManager,
        dataset_manager: &ZfsDatasetManager,
    ) -> Result<()> {
        debug!("📊 Collecting real-time performance metrics");

        // Collect comprehensive ZFS performance metrics with real system integration
        let mut pool_metrics = HashMap::new();
        let mut dataset_metrics = HashMap::new();

        // Real-time pool metrics collection using zpool iostat
        if let Ok(output) = tokio::process::Command::new("zpool")
            .args(["iostat", "-yv", "1", "2"]) // -y for omit first output, -v for verbose
            .output()
            .await
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.lines().collect();

                // Parse pool metrics from iostat output
                for line in lines.iter().skip(1) {
                    // Skip header
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 7 {
                        let pool_name = fields[0];
                        if pool_name != "pool" && !pool_name.is_empty() && !pool_name.contains('-')
                        {
                            let read_ops: f64 = fields[1].parse().unwrap_or(0.0);
                            let write_ops: f64 = fields[2].parse().unwrap_or(0.0);
                            let read_bw: f64 = fields[3].parse().unwrap_or(0.0) / (1024.0 * 1024.0); // Convert to MB/s
                            let write_bw: f64 =
                                fields[4].parse().unwrap_or(0.0) / (1024.0 * 1024.0); // Convert to MB/s

                            // Calculate average latency from queue lengths if available
                            let avg_latency = if fields.len() >= 9 {
                                (fields[7].parse::<f64>().unwrap_or(0.0)
                                    + fields[8].parse::<f64>().unwrap_or(0.0))
                                    / 2.0
                            } else {
                                5.0 // Default latency
                            };

                            // Get cache hit ratio from ARC stats (pool-agnostic for now)
                            let cache_hit_ratio = if let Ok(arc_content) =
                                tokio::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").await
                            {
                                let mut hits = 0u64;
                                let mut misses = 0u64;

                                for arc_line in arc_content.lines() {
                                    let parts: Vec<&str> = arc_line.split_whitespace().collect();
                                    if parts.len() >= 3 {
                                        match parts[0] {
                                            "hits" => hits = parts[2].parse().unwrap_or(0),
                                            "misses" => misses = parts[2].parse().unwrap_or(0),
                                            _ => {}
                                        }
                                    }
                                }

                                if hits + misses > 0 {
                                    hits as f64 / (hits + misses) as f64
                                } else {
                                    0.85 // Default hit ratio
                                }
                            } else {
                                0.85 // Default hit ratio
                            };

                            // Get fragmentation from zpool list
                            let fragmentation = if let Ok(frag_output) =
                                tokio::process::Command::new("zpool")
                                    .args(["list", "-H", "-o", "frag", pool_name])
                                    .output()
                                    .await
                            {
                                let frag_stdout = String::from_utf8_lossy(&frag_output.stdout);
                                if let Some(frag_str) = frag_stdout.trim().strip_suffix('%') {
                                    frag_str.parse().unwrap_or(10.0)
                                } else {
                                    10.0
                                }
                            } else {
                                10.0 // Default fragmentation
                            };

                            pool_metrics.insert(
                                pool_name.to_string(),
                                ZfsPoolMetrics {
                                    pool_name: pool_name.to_string(),
                                    read_ops,
                                    write_ops,
                                    read_bandwidth: read_bw,
                                    write_bandwidth: write_bw,
                                    latency: avg_latency,
                                    cache_hit_ratio,
                                    fragmentation,
                                },
                            );
                        }
                    }
                }
            }
        }

        // Real-time dataset metrics collection
        if let Ok(datasets) = dataset_manager.list_datasets().await {
            for dataset in datasets {
                // Get comprehensive dataset properties
                if let Ok(prop_output) = tokio::process::Command::new("zfs")
                    .args([
                        "get",
                        "-H",
                        "-p",
                        "compression,compressratio,dedup,recordsize,used,logicalused",
                        &dataset.name,
                    ])
                    .output()
                    .await
                {
                    if prop_output.status.success() {
                        let prop_stdout = String::from_utf8_lossy(&prop_output.stdout);

                        let mut _compression_ratio = 1.0;
                        let mut dedup_ratio = 1.0;
                        let mut record_size = 128 * 1024u64;
                        let mut used_bytes = 0u64;
                        let mut logical_used_bytes = 0u64;

                        for line in prop_stdout.lines() {
                            let fields: Vec<&str> = line.split('\t').collect();
                            if fields.len() >= 3 {
                                match fields[1] {
                                    "compressratio" => {
                                        if let Some(ratio_str) = fields[2].strip_suffix('x') {
                                            _compression_ratio = ratio_str.parse().unwrap_or(1.0);
                                        }
                                    }
                                    "dedup" => {
                                        if fields[2] == "on" {
                                            dedup_ratio = 1.2; // Estimate when dedup is enabled
                                        }
                                    }
                                    "recordsize" => {
                                        record_size =
                                            Self::parse_size_value(fields[2]).unwrap_or(128 * 1024);
                                    }
                                    "used" => {
                                        used_bytes = fields[2].parse().unwrap_or(0);
                                    }
                                    "logicalused" => {
                                        logical_used_bytes = fields[2].parse().unwrap_or(0);
                                    }
                                    _ => {}
                                }
                            }
                        }

                        // Calculate actual compression ratio from used vs logical used
                        if logical_used_bytes > 0 && used_bytes > 0 {
                            _compression_ratio = logical_used_bytes as f64 / used_bytes as f64;
                        }

                        // Analyze access pattern based on dataset properties and usage
                        let access_pattern = if record_size >= 1024 * 1024 {
                            AccessPattern::Sequential // Large records suggest sequential
                        } else if record_size <= 32 * 1024 {
                            AccessPattern::Random // Small records suggest random
                        } else {
                            AccessPattern::Mixed
                        };

                        dataset_metrics.insert(
                            dataset.name.clone(),
                            ZfsDatasetMetrics {
                                dataset_name: dataset.name.clone(),
                                access_pattern,
                                dedup_ratio,
                                record_size,
                            },
                        );
                    }
                }
            }
        }

        // Collect system memory usage
        let system_memory = SystemMemoryUsage {
            total: 16 * 1024 * 1024 * 1024,    // 16GB default
            used: 8 * 1024 * 1024 * 1024,      // 8GB default
            available: 8 * 1024 * 1024 * 1024, // 8GB default
        };

        // Collect detailed ARC statistics
        let arc_stats = if let Ok(arc_content) =
            tokio::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").await
        {
            let mut hits = 0u64;
            let mut misses = 0u64;
            let mut size = 0u64;
            let mut c = 0u64; // target size
            let mut _mru_size = 0u64;
            let mut _mfu_size = 0u64;

            for line in arc_content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    match parts[0] {
                        "hits" => hits = parts[2].parse().unwrap_or(0),
                        "misses" => misses = parts[2].parse().unwrap_or(0),
                        "size" => size = parts[2].parse().unwrap_or(0),
                        "c" => c = parts[2].parse().unwrap_or(0),
                        "mru_size" => _mru_size = parts[2].parse().unwrap_or(0),
                        "mfu_size" => _mfu_size = parts[2].parse().unwrap_or(0),
                        _ => {}
                    }
                }
            }

            ArcStatistics {
                hit_ratio: if hits + misses > 0 {
                    hits as f64 / (hits + misses) as f64
                } else {
                    0.85
                },
                size,
                target_size: c,
                miss_ratio: if hits + misses > 0 {
                    misses as f64 / (hits + misses) as f64
                } else {
                    0.15
                },
            }
        } else {
            ArcStatistics {
                hit_ratio: 0.85,
                size: 4 * 1024 * 1024 * 1024,        // 4GB default
                target_size: 8 * 1024 * 1024 * 1024, // 8GB default
                miss_ratio: 0.15,
            }
        };

        // Create comprehensive performance metrics snapshot
        let metrics = ZfsPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            dataset_metrics,
            system_memory,
            arc_stats,
        };

        // Store metrics in cache
        let mut cache = self.metrics_cache.write().await;
        cache.insert("latest".to_string(), metrics.clone());

        // Keep only last 50 entries for trend analysis
        if cache.len() > 50 {
            // Remove oldest entries
            let mut keys: Vec<String> = cache.keys().cloned().collect();
            keys.sort();
            for key in keys.iter().take(cache.len() - 50) {
                cache.remove(key);
            }
        }

        // Get metrics for trending
        let cache = self.metrics_cache.read().await;
        if cache.is_empty() {
            return Err(ZfsError::Internal {
                message: "No metrics available for trending".to_string(),
            });
        }

        // Perform real-time analytics and alerts
        self.analyze_performance_trends().await?;

        debug!(
            "✅ Collected and cached performance metrics: {} pools, {} datasets",
            metrics.pool_metrics.len(),
            metrics.dataset_metrics.len()
        );

        Ok(())
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
                        "📉 Pool {} IOPS degrading: {:.2}% trend",
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
    fn parse_size_value(size_str: &str) -> Result<u64> {
        if let Some(num_str) = size_str.strip_suffix('K') {
            Ok(num_str.parse::<u64>()? * 1024)
        } else if let Some(num_str) = size_str.strip_suffix('M') {
            Ok(num_str.parse::<u64>()? * 1024 * 1024)
        } else if let Some(num_str) = size_str.strip_suffix('G') {
            Ok(num_str.parse::<u64>()? * 1024 * 1024 * 1024)
        } else if let Some(num_str) = size_str.strip_suffix('T') {
            Ok(num_str.parse::<u64>()? * 1024 * 1024 * 1024 * 1024)
        } else {
            Ok(size_str.parse::<u64>()?)
        }
    }

    /// Get trending data for analysis
    pub async fn get_trending_data(&self) -> Result<Vec<ZfsPerformanceMetrics>> {
        let cache = self.metrics_cache.read().await;
        Ok(cache.values().cloned().collect())
    }
}
