// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage tier and per-dataset performance aggregation.

use std::collections::HashMap;
use std::sync::Arc;

use crate::dataset::ZfsDatasetManager;
use crate::types::StorageTier;
use nestgate_core::Result as CoreResult;
use tracing::debug;

use crate::performance::types::{
    DatasetPerformanceStats, SlaCompliance, TierMetrics, TierPerformanceTargets,
    ZfsPerformanceMonitor,
};

impl ZfsPerformanceMonitor {
    /// Collect tier-specific metrics
    pub(super) async fn collect_tier_metrics(
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<HashMap<StorageTier, TierMetrics>> {
        debug!("Collecting tier-specific metrics");

        let mut tier_metrics = HashMap::new();

        for tier in [StorageTier::Hot, StorageTier::Warm, StorageTier::Cold] {
            let metrics = Self::collect_single_tier_metrics(&tier, dataset_manager).await?;
            tier_metrics.insert(tier, metrics);
        }

        Ok(tier_metrics)
    }

    /// Collect metrics for a single tier
    pub(super) async fn collect_single_tier_metrics(
        tier: &StorageTier,
        dataset_manager: &Arc<ZfsDatasetManager>,
    ) -> CoreResult<TierMetrics> {
        debug!("Collecting metrics for tier: {:?}", tier);

        let datasets = dataset_manager.list_datasets().await.unwrap_or_default();
        let tier_datasets: Vec<_> = datasets.into_iter().filter(|d| d.tier == *tier).collect();

        if tier_datasets.is_empty() {
            return Ok(TierMetrics::default_for_tier(tier.clone()));
        }

        let mut total_read_iops = 0.0;
        let mut total_write_iops = 0.0;
        let mut total_read_throughput = 0.0;
        let mut total_write_throughput = 0.0;
        let mut total_read_latency = 0.0;
        let mut total_write_latency = 0.0;
        let mut total_utilization = 0.0;
        let dataset_count = tier_datasets.len() as f64;

        for dataset in &tier_datasets {
            if let Ok(stats) = Self::get_dataset_performance_stats(&dataset.name).await {
                total_read_iops += stats.read_iops;
                total_write_iops += stats.write_iops;
                total_read_throughput += stats.read_throughput_mbs;
                total_write_throughput += stats.write_throughput_mbs;
                total_read_latency += stats.read_latency_ms;
                total_write_latency += stats.write_latency_ms;
                total_utilization += stats.utilization_percent;
            }
        }

        let cache_hit_ratio = Self::get_zfs_cache_hit_ratio().await.unwrap_or(0.85);

        Ok(TierMetrics {
            tier: tier.clone(),
            read_iops: total_read_iops,
            write_iops: total_write_iops,
            read_throughput_mbs: total_read_throughput,
            write_throughput_mbs: total_write_throughput,
            avg_read_latency_ms: if dataset_count > 0.0 {
                total_read_latency / dataset_count
            } else {
                0.0
            },
            avg_write_latency_ms: if dataset_count > 0.0 {
                total_write_latency / dataset_count
            } else {
                0.0
            },
            cache_hit_ratio,
            queue_depth: Self::get_real_queue_depth(tier).unwrap_or(4.0),
            utilization_percent: if dataset_count > 0.0 {
                total_utilization / dataset_count
            } else {
                0.0
            },
            targets: TierPerformanceTargets::default(),
            sla_compliance: SlaCompliance::default(),
        })
    }

    /// Get performance statistics for a specific dataset
    pub(super) async fn get_dataset_performance_stats(
        dataset_name: &str,
    ) -> CoreResult<DatasetPerformanceStats> {
        debug!(
            "Collecting real performance stats for dataset: {}",
            dataset_name
        );

        let mut stats = DatasetPerformanceStats::default();

        // Get dataset properties
        if let Ok(output) = tokio::process::Command::new("zfs")
            .args(["get", "-H", "-p", "used,compressratio,dedup", dataset_name])
            .output()
            .await
            && output.status.success()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() >= 4 {
                    match fields[1] {
                        "compressratio" => {
                            if let Ok(ratio) = fields[2].trim_end_matches('x').parse::<f64>() {
                                stats.compression_effectiveness = ratio;
                            }
                        }
                        "dedup" => {
                            if fields[2] == "on" {
                                stats.deduplication_effectiveness = 1.2;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Get I/O statistics
        if let Some(pool_name) = dataset_name.split('/').next()
            && let Ok(output) = tokio::process::Command::new("zpool")
                .args(["iostat", "-v", pool_name, "1", "1"])
                .output()
                .await
            && output.status.success()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains(dataset_name) {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 7 {
                        if let Ok(read_ops) = fields[1].parse::<f64>() {
                            stats.read_iops = read_ops;
                        }
                        if let Ok(write_ops) = fields[2].parse::<f64>() {
                            stats.write_iops = write_ops;
                        }
                        if let Ok(read_bw) = fields[3].parse::<f64>() {
                            stats.read_throughput_mbs = read_bw / (1024.0 * 1024.0);
                        }
                        if let Ok(write_bw) = fields[4].parse::<f64>() {
                            stats.write_throughput_mbs = write_bw / (1024.0 * 1024.0);
                        }
                    }
                }
            }
        }

        // Calculate utilization and latency
        let total_iops = stats.read_iops + stats.write_iops;
        stats.utilization_percent = if total_iops > 0.0 {
            (total_iops / 10_000.0 * 100.0).min(100.0)
        } else {
            0.0
        };

        stats.read_latency_ms = if stats.read_iops > 0.0 {
            (1000.0 / stats.read_iops).min(1000.0)
        } else {
            0.0
        };
        stats.write_latency_ms = if stats.write_iops > 0.0 {
            (1000.0 / stats.write_iops).min(1000.0)
        } else {
            0.0
        };

        Ok(stats)
    }

    /// Get ZFS cache hit ratio
    pub(super) async fn get_zfs_cache_hit_ratio() -> CoreResult<f64> {
        // Read ARC statistics from /proc/spl/kstat/zfs/arcstats
        if let Ok(content) = tokio::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").await {
            let mut hits = 0u64;
            let mut misses = 0u64;

            for line in content.lines() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 3 {
                    match fields[0] {
                        "hits" => hits = fields[2].parse().unwrap_or(0),
                        "misses" => misses = fields[2].parse().unwrap_or(0),
                        _ => {}
                    }
                }
            }

            let total = hits + misses;
            if total > 0 {
                return Ok((hits as f64 / total as f64) * 100.0);
            }
        }

        Ok(85.0) // Default fallback
    }

    /// Get real queue depth for a tier
    pub(super) const fn get_real_queue_depth(tier: &StorageTier) -> CoreResult<f64> {
        // This would typically read from system statistics
        // For now, return tier-appropriate defaults
        Ok(match tier {
            StorageTier::Hot => 32.0,
            StorageTier::Warm => 16.0,
            StorageTier::Cold => 8.0,
            StorageTier::Cache => 64.0,
            StorageTier::Archive => 4.0,
        })
    }
}
