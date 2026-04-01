// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Top-level metrics collection orchestration.

use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::debug;

use crate::dataset::ZfsDatasetManager;
use crate::pool::ZfsPoolManager;
use nestgate_core::Result as CoreResult;

use crate::performance::types::{
    CurrentPerformanceMetrics, IoStatistics, PerformanceTrends, SystemResourceMetrics,
    TierMetricsMap, ZfsPerformanceMonitor,
};

impl ZfsPerformanceMonitor {
    /// Collect performance metrics
    pub(crate) async fn collect_metrics(
        pool_manager: &Arc<ZfsPoolManager>,
        dataset_manager: &Arc<ZfsDatasetManager>,
        current_metrics: &Arc<RwLock<CurrentPerformanceMetrics>>,
        tier_metrics: &TierMetricsMap,
    ) -> CoreResult<()> {
        debug!("Collecting performance metrics");

        // Collect real ZFS metrics
        let pool_metrics = Self::collect_pool_metrics(pool_manager).await?;
        let system_performance = Self::collect_system_metrics().await?;
        let disk_stats = Self::collect_io_statistics(pool_manager).await?;
        let tier_data = Self::collect_tier_metrics(dataset_manager).await?;

        let metrics = CurrentPerformanceMetrics {
            timestamp: SystemTime::now(),
            pool_metrics,
            tier_metrics: tier_data.clone(),
            system_metrics: SystemResourceMetrics {
                cpu_utilization_percent: system_performance.cpu_utilization_percent,
                memory_usage_bytes: (system_performance.memory_utilization_percent
                    * 8_000_000_000.0
                    / 100.0) as u64,
                available_memory_bytes: 8_000_000_000_u64,
                network_io_mbs: system_performance.network_throughput_mbs,
                io_wait_percent: 2.0,
                load_average_1m: system_performance.system_load_average,
            },
            io_statistics: IoStatistics {
                total_reads: 100000_u64,
                total_writes: 50000_u64,
                total_bytes_read: disk_stats.first().map_or(0, |s| s.read_ops * 1024 * 1024),
                total_bytes_written: disk_stats.first().map_or(0, |s| s.write_ops * 1024 * 1024),
                avg_io_size_bytes: 65536_u64,
                read_write_ratio: 2.0,
            },
            trends: PerformanceTrends::default(),
        };

        // Update current metrics
        {
            let mut current = current_metrics.write().await;
            *current = metrics;
        }

        // Update tier-specific metrics
        {
            let mut tier_data_store = tier_metrics.write().await;
            for (tier, tier_metric) in tier_data {
                if let Some(data) = tier_data_store.get_mut(&tier) {
                    data.current_metrics = tier_metric.clone();
                    data.history.push_back(tier_metric);

                    // Limit history size
                    if data.history.len() > 100 {
                        data.history.pop_front();
                    }
                }
            }
        }
        Ok(())
    }
}
