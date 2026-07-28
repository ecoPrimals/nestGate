// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, SystemTime};

use nestgate_core::Result;
use tokio::sync::broadcast;
use tracing::{debug, info, warn};

use crate::handlers::dashboard_types::{DashboardEvent, DashboardTimeRange};

use super::history::{
    DEFAULT_COLLECTION_INTERVAL, DEFAULT_HISTORY_CAPACITY, MetricsHistoryBuffer, MetricsSnapshot,
    capacity_metrics_history, comprehensive_metrics_history, io_metrics_history,
    pool_metrics_history,
};
use super::linux_proc;
use super::types::{PoolMetrics, RealTimeMetrics, SystemSnapshot};

/// Real-time metrics collection engine backed by /proc filesystem reads.
#[derive(Debug, Clone)]
pub struct RealTimeMetricsCollector {
    history: Arc<MetricsHistoryBuffer>,
    collection_interval: Duration,
    collection_started: Arc<AtomicBool>,
}

impl RealTimeMetricsCollector {
    /// Create a new metrics collector with default history capacity and interval.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(DEFAULT_HISTORY_CAPACITY, DEFAULT_COLLECTION_INTERVAL)
    }

    /// Create a collector with a custom ring-buffer capacity and collection interval.
    #[must_use]
    pub fn with_config(capacity: usize, collection_interval: Duration) -> Self {
        Self {
            history: Arc::new(MetricsHistoryBuffer::with_capacity(capacity)),
            collection_interval,
            collection_started: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start real-time metrics collection with event broadcasting.
    ///
    /// Spawns a background task on the current Tokio runtime that collects metrics
    /// on a fixed interval and appends each snapshot to the in-memory ring buffer.
    pub fn start_collection(&self, broadcaster: Arc<broadcast::Sender<DashboardEvent>>) {
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            warn!("start_collection: no tokio runtime; background collection not started");
            return;
        };

        if self
            .collection_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            debug!("start_collection: background collection already running");
            return;
        }

        let collector = self.clone();
        handle.spawn(async move {
            collector.run_collection_loop(broadcaster).await;
        });
    }

    async fn run_collection_loop(&self, broadcaster: Arc<broadcast::Sender<DashboardEvent>>) {
        info!(
            "Starting background metrics collection (interval {:?})",
            self.collection_interval
        );
        let mut interval = tokio::time::interval(self.collection_interval);

        loop {
            interval.tick().await;

            match self.capture_snapshot().await {
                Ok(snapshot) => {
                    self.history.push(snapshot.clone());
                    let event = DashboardEvent {
                        event_type: "metrics_update".into(),
                        data: serde_json::json!({
                            "timestamp": snapshot.metrics.timestamp,
                            "pool_count": snapshot.metrics.pool_metrics.len(),
                            "total_throughput": snapshot.metrics.total_throughput,
                        }),
                        timestamp: snapshot.metrics.timestamp,
                    };
                    let _ = broadcaster.send(event);
                }
                Err(error) => {
                    warn!("Background metrics collection failed: {error}");
                }
            }
        }
    }

    async fn capture_snapshot(&self) -> Result<MetricsSnapshot> {
        let metrics = self.get_current_metrics().await?;
        let (arc_size, l2arc_size) = linux_proc::arc_and_l2arc_sizes().await.unwrap_or((0, 0));
        Ok(MetricsSnapshot {
            metrics,
            arc_size,
            l2arc_size,
        })
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
        info!("Collecting real-time system and storage metrics");

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
    /// Returns snapshots from the in-memory ring buffer filtered by `time_range`.
    /// When collection has not started, returns an empty vec.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_historical_data(
        &self,
        pool_name: &str,
        time_range: &DashboardTimeRange,
    ) -> Result<Vec<PoolMetrics>> {
        debug!(pool = pool_name, "Historical pool metrics requested");
        let snapshots = self.history.snapshots_in_range(time_range);
        Ok(pool_metrics_history(&snapshots, pool_name))
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

        let (memory_total_gb, memory_used_gb) = {
            let total_bytes = nestgate_platform::linux_proc::total_memory_bytes().unwrap_or(0);
            let used_bytes = nestgate_platform::linux_proc::used_memory_bytes().unwrap_or(0);
            let total_gb = u32::try_from(total_bytes / (1024 * 1024 * 1024)).unwrap_or(u32::MAX);
            let used_gb = u32::try_from(used_bytes / (1024 * 1024 * 1024)).unwrap_or(u32::MAX);
            (total_gb, used_gb)
        };

        let cpu_usage =
            nestgate_platform::linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0);

        let (disk_total_gb, disk_used_gb) =
            nestgate_platform::linux_proc::statvfs_space(std::path::Path::new("/"))
                .map(|(total, used)| (total / (1024 * 1024 * 1024), used / (1024 * 1024 * 1024)))
                .unwrap_or((0, 0));

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
    /// Collects live metrics via `zpool list -H -p` and returns them keyed
    /// by pool name for dashboard and monitoring consumers.
    ///
    /// # Errors
    ///
    /// Returns an error if ZFS pool enumeration fails.
    pub async fn get_all_pool_metrics(&self) -> Result<HashMap<String, PoolMetrics>> {
        let pools = linux_proc::collect_zfs_pool_metrics()
            .await
            .unwrap_or_else(|_| vec![]);
        let map = pools.into_iter().map(|p| (p.name.clone(), p)).collect();
        Ok(map)
    }

    /// I/O performance over time from the in-memory ring buffer.
    ///
    /// Returns an empty vec when collection has not started.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_io_historical_data(
        &self,
        time_range: &DashboardTimeRange,
    ) -> Result<Vec<super::types::IOMetricsPoint>> {
        let snapshots = self.history.snapshots_in_range(time_range);
        Ok(io_metrics_history(&snapshots))
    }

    /// ZFS ARC / L2ARC cache performance — current snapshot.
    ///
    /// Returns a single-element vec with the current ARC/L2ARC state read from
    /// `/proc/spl/kstat/zfs/arcstats`. Historical trends require a time-series
    /// capability provider.
    ///
    /// # Errors
    ///
    /// Returns an error if ARC stats cannot be read.
    pub async fn get_cache_metrics(&self) -> Result<Vec<super::types::CacheMetricsPoint>> {
        let (arc_hit_ratio, l2arc_hit_ratio, _compression) =
            linux_proc::collect_zfs_cache_stats().await?;

        let (arc_size, l2arc_size) = linux_proc::arc_and_l2arc_sizes().await.unwrap_or((0, 0));

        Ok(vec![super::types::CacheMetricsPoint {
            timestamp: SystemTime::now(),
            arc_hit_ratio,
            l2arc_hit_ratio,
            arc_size,
            l2arc_size,
        }])
    }

    /// Comprehensive combined metrics over time from the ring buffer.
    ///
    /// Returns an empty vec when collection has not started.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_comprehensive_historical_data(
        &self,
    ) -> Result<Vec<super::types::ComprehensiveMetricsPoint>> {
        let snapshots = self.history.all_snapshots();
        Ok(comprehensive_metrics_history(&snapshots))
    }

    /// Storage capacity trends over time from the ring buffer.
    ///
    /// Returns an empty vec when collection has not started.
    ///
    /// # Errors
    ///
    /// Returns an error if metric retrieval fails.
    pub fn get_capacity_historical_data(
        &self,
        time_range: &DashboardTimeRange,
    ) -> Result<Vec<super::types::CapacityMetricsPoint>> {
        let snapshots = self.history.snapshots_in_range(time_range);
        Ok(capacity_metrics_history(&snapshots))
    }

    /// Push a snapshot directly (test helper).
    #[cfg(test)]
    pub(crate) fn push_test_snapshot(&self, snapshot: MetricsSnapshot) {
        self.history.push(snapshot);
    }
}

impl Default for RealTimeMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
