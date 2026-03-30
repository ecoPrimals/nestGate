// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Per-pool I/O statistics and bandwidth parsing.

use std::sync::Arc;

use crate::numeric::f64_to_u64_saturating;
use crate::pool::ZfsPoolManager;
use nestgate_core::{NestGateError, Result as CoreResult};
use tracing::debug;

use crate::performance::types::{PoolIoStats, ZfsPerformanceMonitor};

impl ZfsPerformanceMonitor {
    /// Collect I/O statistics for pools
    pub(super) async fn collect_io_statistics(
        pool_manager: &Arc<ZfsPoolManager>,
    ) -> CoreResult<Vec<PoolIoStats>> {
        debug!("Collecting I/O statistics");

        let pools = pool_manager.list_pools().await.unwrap_or_default();
        let mut io_stats = Vec::new();

        for pool in pools {
            let stats = Self::get_pool_iostat_data(&pool.name).await?;
            io_stats.push(stats);
        }
        Ok(io_stats)
    }

    /// Get detailed I/O statistics for a specific pool
    pub(super) async fn get_pool_iostat_data(pool_name: &str) -> CoreResult<PoolIoStats> {
        debug!("Getting I/O statistics for pool: {}", pool_name);

        let output = tokio::process::Command::new("zpool")
            .args(["iostat", "-v", pool_name, "1", "1"])
            .output()
            .await
            .map_err(|_e| {
                NestGateError::internal_error("Failed to execute zpool iostat command", "unknown")
            })?;

        if !output.status.success() {
            return Ok(PoolIoStats::default());
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut pool_stats = PoolIoStats::default();

        for line in output_str.lines() {
            if line.contains(pool_name) && !line.starts_with("pool:") {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 7 {
                    if let Ok(read_ops) = fields[3].parse::<u64>() {
                        pool_stats.read_ops = read_ops;
                    }
                    if let Ok(write_ops) = fields[4].parse::<u64>() {
                        pool_stats.write_ops = write_ops;
                    }
                    if let Ok(read_bw) = Self::parse_iostat_bandwidth(fields[5]) {
                        pool_stats.bytes_read = read_bw;
                    }
                    if let Ok(write_bw) = Self::parse_iostat_bandwidth(fields[6]) {
                        pool_stats.bytes_written = write_bw;
                    }
                }
                break;
            }
        }

        Ok(pool_stats)
    }

    /// Parse bandwidth values from iostat output
    pub(super) fn parse_iostat_bandwidth(value: &str) -> Result<u64, std::num::ParseFloatError> {
        let value = value.trim();
        if value.is_empty() || value == "-" {
            return Ok(0);
        }

        let (number, multiplier) = if value.ends_with('K') {
            (value.trim_end_matches('K'), 1024_u64)
        } else if value.ends_with('M') {
            (value.trim_end_matches('M'), 1024_u64 * 1024)
        } else if value.ends_with('G') {
            (value.trim_end_matches('G'), 1024_u64 * 1024 * 1024)
        } else {
            (value, 1_u64)
        };

        let number: f64 = number.parse()?;
        Ok(f64_to_u64_saturating(number * multiplier as f64))
    }
}
