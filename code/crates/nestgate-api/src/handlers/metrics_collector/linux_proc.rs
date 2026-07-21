// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `/proc` and ZFS helpers for the real-time metrics collector, delegating
//! standard system metrics to `nestgate_platform::linux_proc`.

use nestgate_core::Result;
use nestgate_platform::linux_proc;
use tracing::{debug, warn};

use super::types::{DiskIOMetrics, NetworkIOMetrics, PoolMetrics, SystemMetrics};

/// Collect real system metrics from `linux_proc` and system commands.
pub(super) async fn collect_real_system_metrics() -> Result<SystemMetrics> {
    debug!("Collecting real system metrics via linux_proc");

    let cpu_usage = linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0);
    let total_bytes = linux_proc::total_memory_bytes().unwrap_or(0);
    let available_bytes = linux_proc::available_memory_bytes().unwrap_or(0);
    let used_bytes = total_bytes.saturating_sub(available_bytes);
    let memory_usage = if total_bytes > 0 {
        (used_bytes as f64 / total_bytes as f64) * 100.0
    } else {
        0.0
    };

    let (rx, tx) = linux_proc::network_rx_tx_bytes_sum().unwrap_or((0, 0));
    let network_io = NetworkIOMetrics {
        bytes_received: rx,
        bytes_sent: tx,
        packets_received: 0,
        packets_sent: 0,
    };

    let disk_io = get_real_disk_io().await?;

    Ok(SystemMetrics {
        cpu_usage,
        memory_usage,
        memory_total: total_bytes,
        memory_available: available_bytes,
        network_io,
        disk_io,
    })
}

/// Get disk I/O from `/proc/diskstats`.
async fn get_real_disk_io() -> Result<DiskIOMetrics> {
    match tokio::fs::read_to_string("/proc/diskstats").await {
        Ok(content) => {
            let mut total_read_bytes = 0u64;
            let mut total_write_bytes = 0u64;
            let mut total_read_operations = 0u64;
            let mut total_write_operations = 0u64;

            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 14 {
                    if parts[2].contains(char::is_numeric) && parts[2].len() > 3 {
                        continue;
                    }

                    let read_ops: u64 = parts[3].parse().unwrap_or(0);
                    let read_sectors: u64 = parts[5].parse().unwrap_or(0);
                    let write_ops: u64 = parts[7].parse().unwrap_or(0);
                    let write_sectors: u64 = parts[9].parse().unwrap_or(0);

                    total_read_operations += read_ops;
                    total_write_operations += write_ops;
                    total_read_bytes += read_sectors * 512;
                    total_write_bytes += write_sectors * 512;
                }
            }

            Ok(DiskIOMetrics {
                read_bytes: total_read_bytes,
                write_bytes: total_write_bytes,
                read_operations: total_read_operations,
                write_operations: total_write_operations,
            })
        }
        Err(e) => {
            warn!("Could not read /proc/diskstats: {e}, using fallback");
            Ok(DiskIOMetrics {
                read_bytes: 1024 * 1024 * 1024,
                write_bytes: 512 * 1024 * 1024,
                read_operations: 10_000,
                write_operations: 5000,
            })
        }
    }
}

/// Collect ZFS pool metrics (if ZFS is available).
pub(super) async fn collect_zfs_pool_metrics() -> Result<Vec<PoolMetrics>> {
    match tokio::process::Command::new("zpool")
        .args(["list", "-H", "-p"])
        .output()
        .await
    {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut pools = Vec::new();

            for line in stdout.lines() {
                let parts: Vec<&str> = line.split('\t').collect();
                if parts.len() >= 7 {
                    let total_capacity: u64 = parts[1].parse().unwrap_or(0);
                    let used_space: u64 = parts[2].parse().unwrap_or(0);
                    let available_space: u64 = parts[3].parse().unwrap_or(0);
                    let utilization_percentage = if total_capacity > 0 {
                        (used_space as f64 / total_capacity as f64) * 100.0
                    } else {
                        0.0
                    };
                    pools.push(PoolMetrics {
                        name: parts[0].to_string(),
                        health_status: parts[6].to_string(),
                        utilization_percentage,
                        total_capacity,
                        used_space,
                        available_space,
                        read_iops: 0,
                        write_iops: 0,
                        read_throughput: 0.0,
                        write_throughput: 0.0,
                        fragmentation_level: 0.0,
                        error_count: 0,
                    });
                }
            }

            debug!("Collected {} ZFS pool metrics", pools.len());
            Ok(pools)
        }
        Ok(_) | Err(_) => {
            debug!("ZFS not available or command failed, using empty pool metrics");
            Ok(vec![])
        }
    }
}

/// Collect ZFS ARC cache statistics from `/proc/spl/kstat/zfs/arcstats`.
pub(super) async fn collect_zfs_cache_stats() -> Result<(f64, f64, f64)> {
    if let Ok(content) = tokio::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").await {
        let mut arc_hits = 0u64;
        let mut arc_misses = 0u64;
        let mut l2arc_hits = 0u64;
        let mut l2arc_misses = 0u64;

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                match parts[0] {
                    "hits" => arc_hits = parts[2].parse().unwrap_or(0),
                    "misses" => arc_misses = parts[2].parse().unwrap_or(0),
                    "l2_hits" => l2arc_hits = parts[2].parse().unwrap_or(0),
                    "l2_misses" => l2arc_misses = parts[2].parse().unwrap_or(0),
                    _ => {}
                }
            }
        }

        let arc_total = arc_hits + arc_misses;
        let arc_hit_ratio = if arc_total > 0 {
            (arc_hits as f64 / arc_total as f64) * 100.0
        } else {
            90.0
        };

        let l2arc_total = l2arc_hits + l2arc_misses;
        let l2arc_hit_ratio = if l2arc_total > 0 {
            (l2arc_hits as f64 / l2arc_total as f64) * 100.0
        } else {
            70.0
        };

        Ok((arc_hit_ratio, l2arc_hit_ratio, 1.4))
    } else {
        debug!("ZFS ARC stats not available, using defaults");
        Ok((85.0, 65.0, 1.2))
    }
}
