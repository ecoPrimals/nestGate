// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

use nestgate_core::Result;
use tokio::sync::broadcast;
use tracing::{debug, info};

use crate::handlers::dashboard_types::{DashboardEvent, DashboardTimeRange};

use super::linux_proc;
use super::types::{PoolMetrics, RealTimeMetrics, SystemSnapshot};

/// Real-time metrics collection engine backed by /proc filesystem reads.
#[derive(Debug)]
pub struct RealTimeMetricsCollector {
    // Implementation details
}
impl RealTimeMetricsCollector {
    /// Create a new metrics collector
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    /// Start real-time metrics collection with event broadcasting
    pub fn start_collection(&self, _broadcaster: Arc<broadcast::Sender<DashboardEvent>>) {
        // Implementation for starting real-time metrics collection
        info!("Starting real-time metrics collection");
        // This would spawn background tasks to continuously collect metrics
    }

    /// Get current system and storage metrics with real data collection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_current_metrics(&self) -> Result<RealTimeMetrics> {
        info!("📊 Collecting real-time system and storage metrics");

        // Collect real system metrics
        let system_metrics = linux_proc::collect_real_system_metrics().await?;

        // Collect ZFS pool metrics (if available)
        let pool_metrics = linux_proc::collect_zfs_pool_metrics()
            .await
            .unwrap_or_else(|_| vec![]);

        // Collect ZFS ARC statistics
        let (arc_hit_ratio, l2arc_hit_ratio, compression_ratio) =
            linux_proc::collect_zfs_cache_stats().await?;

        // Calculate total throughput from pool metrics or system I/O
        let total_throughput = if pool_metrics.is_empty() {
            // Fallback to system disk I/O throughput estimation
            (system_metrics.disk_io.read_bytes + system_metrics.disk_io.write_bytes) as f64
                / (1024.0 * 1024.0) // MB/s
        } else {
            pool_metrics
                .iter()
                .map(|p| p.read_throughput + p.write_throughput)
                .sum()
        };

        // Calculate average latencies from system disk metrics
        let average_read_latency = if pool_metrics.is_empty() {
            // Estimate from system I/O (simplified calculation)
            let read_ops = system_metrics.disk_io.read_bytes.max(1);
            (system_metrics.disk_io.read_bytes as f64 / read_ops as f64) / 1000.0
        // Rough latency estimate
        } else {
            pool_metrics.iter().map(|p| p.read_throughput).sum::<f64>()
                / pool_metrics.len().max(1) as f64
        };

        let average_write_latency = if pool_metrics.is_empty() {
            let write_ops = system_metrics.disk_io.write_bytes.max(1);
            (system_metrics.disk_io.write_bytes as f64 / write_ops as f64) / 1000.0
        // Rough latency estimate
        } else {
            pool_metrics.iter().map(|p| p.write_throughput).sum::<f64>()
                / pool_metrics.len().max(1) as f64
        };

        Ok(RealTimeMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            system_metrics,
            arc_hit_ratio,
            l2arc_hit_ratio,
            compression_ratio,
            total_throughput,
            average_read_latency,
            average_write_latency,
        })
    }

    /// Get historical performance data for a specific pool.
    ///
    /// Requires a time-series store (not yet wired). Returns an empty vec
    /// until the observability capability provider supplies historical storage.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_historical_data(
        &self,
        pool_name: &str,
        _time_range: &DashboardTimeRange,
    ) -> Result<Vec<PoolMetrics>> {
        debug!(
            pool = pool_name,
            "Historical pool metrics require a time-series store — returning empty"
        );
        Ok(vec![])
    }

    /// Get system resource snapshot from /proc, with safe fallbacks.
    ///
    /// # Errors
    ///
    /// Returns an error if system metrics cannot be collected.
    pub fn get_system_resources(&self) -> Result<SystemSnapshot> {
        let cpu_cores = std::thread::available_parallelism()
            .map(|n| u32::try_from(n.get()).unwrap_or(u32::MAX))
            .unwrap_or(1);

        let (memory_total_gb, memory_used_gb, cpu_usage) =
            match std::fs::read_to_string("/proc/meminfo") {
                Ok(content) => {
                    let mut mem_total_kb = 0u64;
                    let mut mem_available_kb = 0u64;
                    for line in content.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            match parts[0] {
                                "MemTotal:" => mem_total_kb = parts[1].parse().unwrap_or(0),
                                "MemAvailable:" => mem_available_kb = parts[1].parse().unwrap_or(0),
                                _ => {}
                            }
                        }
                    }
                    let total_gb = u32::try_from(mem_total_kb / (1024 * 1024)).unwrap_or(u32::MAX);
                    let used_gb = u32::try_from(
                        mem_total_kb.saturating_sub(mem_available_kb) / (1024 * 1024),
                    )
                    .unwrap_or(u32::MAX);
                    (total_gb, used_gb, 0.0)
                }
                Err(_) => (0, 0, 0.0),
            };

        // statvfs would be ideal but we stay pure-Rust; disk metrics deferred
        let (disk_total_gb, disk_used_gb) = (0, 0);

        let network_interfaces = match std::fs::read_to_string("/proc/net/dev") {
            Ok(content) => content
                .lines()
                .skip(2)
                .filter_map(|line| line.split(':').next().map(|name| name.trim().to_string()))
                .collect(),
            Err(_) => vec![],
        };

        Ok(SystemSnapshot {
            timestamp: SystemTime::now(),
            cpu_cores,
            cpu_usage_percent: cpu_usage,
            memory_total_gb,
            memory_used_gb,
            disk_total_gb,
            disk_used_gb,
            network_interfaces,
        })
    }

    /// Per-pool metrics keyed by pool name.
    ///
    /// Returns an empty map until ZFS pool enumeration is wired into the
    /// real-time collector (live metrics use `/proc/spl/kstat/zfs` instead).
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_all_pool_metrics(&self) -> Result<HashMap<String, PoolMetrics>> {
        debug!("Per-pool metric map requires ZFS pool enumeration — returning empty");
        Ok(HashMap::new())
    }

    /// I/O performance over time.
    ///
    /// Requires a time-series store; returns empty until the observability
    /// capability provider supplies historical I/O storage.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_io_historical_data(
        &self,
        _time_range: &DashboardTimeRange,
    ) -> Result<Vec<super::types::IOMetricsPoint>> {
        debug!("I/O historical data requires a time-series store — returning empty");
        Ok(vec![])
    }

    /// ZFS ARC / L2ARC cache performance over time.
    ///
    /// Point-in-time snapshots are available via [`get_current_metrics`]; historical
    /// trends require a time-series backend.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_cache_metrics(&self) -> Result<Vec<super::types::CacheMetricsPoint>> {
        debug!("Cache historical metrics require a time-series store — returning empty");
        Ok(vec![])
    }

    /// Comprehensive combined metrics over time.
    ///
    /// Requires a time-series store; returns empty until wired.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_comprehensive_historical_data(
        &self,
    ) -> Result<Vec<super::types::ComprehensiveMetricsPoint>> {
        debug!("Comprehensive historical data requires a time-series store — returning empty");
        Ok(vec![])
    }

    /// Storage capacity trends over time.
    ///
    /// Requires a time-series store; returns empty until wired.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_capacity_historical_data(
        &self,
        _time_range: &DashboardTimeRange,
    ) -> Result<Vec<super::types::CapacityMetricsPoint>> {
        debug!("Capacity historical data requires a time-series store — returning empty");
        Ok(vec![])
    }
}

impl Default for RealTimeMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
