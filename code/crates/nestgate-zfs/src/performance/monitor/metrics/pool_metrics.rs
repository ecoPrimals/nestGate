// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! ZFS pool-level metrics: `zpool iostat`, pool properties.

use std::sync::Arc;

use crate::pool::ZfsPoolManager;
use nestgate_core::{NestGateError, Result as CoreResult};
use tracing::{debug, warn};

use super::parsing::parse_zpool_get_pool_properties;
use crate::performance::types::{
    IoStatsSummary, PoolPerformanceMetrics, PoolProperties, ZfsPerformanceMonitor,
};

impl ZfsPerformanceMonitor {
    /// Collect real ZFS pool performance metrics
    pub(crate) async fn collect_pool_metrics(
        pool_manager: &Arc<ZfsPoolManager>,
    ) -> CoreResult<PoolPerformanceMetrics> {
        debug!("Collecting ZFS pool metrics");

        // Execute zpool iostat to get real I/O statistics
        let iostat_output = tokio::process::Command::new("zpool")
            .args(["iostat", "-v", "-y", "1", "1"])
            .output()
            .await
            .map_err(|_e| {
                NestGateError::internal_error("Failed to execute zpool iostat command", "unknown")
            })?;

        if !iostat_output.status.success() {
            warn!("zpool iostat failed, using fallback metrics");
            return Ok(PoolPerformanceMetrics::default());
        }

        let iostat_str = String::from_utf8_lossy(&iostat_output.stdout);
        let parsed_metrics = Self::parse_zpool_iostat(&iostat_str)?;

        // Get pool status information
        let pools = pool_manager.list_pools().await.unwrap_or_default();
        let mut total_size = 0u64;
        let mut total_free = 0u64;
        let mut fragmentation_sum = 0.0;
        let mut compression_sum = 0.0;
        let mut dedup_sum = 0.0;
        let pool_count = pools.len() as f64;

        for pool in &pools {
            // Get detailed pool information
            if let Ok(pool_info) = pool_manager.get_pool_info(&pool.name).await {
                let total_bytes = pool_info.capacity.total_bytes;
                let available_bytes = pool_info.capacity.available_bytes;

                total_size += total_bytes;
                total_free += available_bytes;

                // Collect additional pool properties
                if let Ok(properties) = Self::get_pool_properties(&pool.name).await {
                    fragmentation_sum += properties.fragmentation;
                    compression_sum += properties.compression_ratio;
                    dedup_sum += properties.dedup_ratio;
                }
            }
        }

        let utilization_percent = if total_size > 0 {
            ((total_size - total_free) as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        Ok(PoolPerformanceMetrics {
            total_iops: parsed_metrics.read_ops as f64 + parsed_metrics.write_ops as f64,
            total_throughput_mbs: parsed_metrics.read_throughput_mbs
                + parsed_metrics.write_throughput_mbs,
            avg_latency_ms: f64::midpoint(
                parsed_metrics.read_latency_ms,
                parsed_metrics.write_latency_ms,
            ),
            utilization_percent,
            fragmentation_percent: if pool_count > 0.0 {
                fragmentation_sum / pool_count
            } else {
                0.0
            },
            compression_ratio: if pool_count > 0.0 {
                compression_sum / pool_count
            } else {
                1.0
            },
            dedup_ratio: if pool_count > 0.0 {
                dedup_sum / pool_count
            } else {
                1.0
            },
        })
    }

    /// Parse zpool iostat output into structured metrics
    pub(crate) fn parse_zpool_iostat(output: &str) -> CoreResult<IoStatsSummary> {
        let mut read_ops = 0u64;
        let mut write_ops = 0u64;
        let mut read_bytes = 0u64;
        let mut write_bytes = 0u64;

        // Parse iostat output - looking for lines with pool statistics
        for line in output.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 7 && !line.starts_with('-') && !line.contains("pool") {
                // Format: pool alloc free read write read write
                // Fields: [0]  [1]   [2]  [3]  [4]   [5]  [6]
                if let (Ok(r_ops), Ok(w_ops), Ok(r_bw), Ok(w_bw)) = (
                    fields[3].parse::<u64>(),
                    fields[4].parse::<u64>(),
                    fields[5].parse::<u64>(),
                    fields[6].parse::<u64>(),
                ) {
                    read_ops += r_ops;
                    write_ops += w_ops;
                    read_bytes += r_bw;
                    write_bytes += w_bw;
                }
            }
        }

        Ok(IoStatsSummary {
            read_ops,
            write_ops,

            read_throughput_mbs: read_bytes as f64 / (1024.0 * 1024.0),
            write_throughput_mbs: write_bytes as f64 / (1024.0 * 1024.0),
            read_latency_ms: 0.0,
            write_latency_ms: 0.0,
        })
    }

    /// Get pool properties for monitoring
    pub(crate) async fn get_pool_properties(pool_name: &str) -> CoreResult<PoolProperties> {
        let output = tokio::process::Command::new("zpool")
            .args(["get", "all", pool_name])
            .output()
            .await
            .map_err(|_e| {
                NestGateError::internal_error("Failed to execute zpool get command", "unknown")
            })?;

        if !output.status.success() {
            return Ok(PoolProperties::default());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(parse_zpool_get_pool_properties(&output_str))
    }
}
