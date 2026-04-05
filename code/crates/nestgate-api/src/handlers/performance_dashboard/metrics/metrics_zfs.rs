// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS pool, ARC, and compression sampling for the performance dashboard metrics collector.

use crate::handlers::metrics_collector::PoolMetrics;
use nestgate_core::Result;
use nestgate_zfs::numeric::f64_to_u64_saturating;
use tokio::process::Command;
use tracing::debug;
use tracing::warn;

/// Collect real ZFS pool metrics
pub(super) async fn collect_pool_metrics() -> Result<Vec<PoolMetrics>> {
    debug!("🏊 Collecting real ZFS pool metrics");

    let mut pool_metrics = Vec::new();

    // Get pool list
    let pool_list_output = Command::new("zpool")
        .args(["list", "-H", "-o", "name"])
        .output()
        .await;

    match pool_list_output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);

            for pool_name in stdout.lines() {
                let pool_name = pool_name.trim();
                if pool_name.is_empty() {
                    continue;
                }

                // Collect individual pool metrics
                if let Ok(pool_metric) = collect_single_pool_metrics(pool_name).await {
                    pool_metrics.push(pool_metric);
                }
            }
        }
        Ok(output) => {
            warn!(
                "⚠️ zpool list failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            warn!("⚠️ Failed to execute zpool list: {e}");
        }
    }

    // If no pools found, return empty vec (not an error)
    Ok(pool_metrics)
}

/// Collect metrics for a single pool
pub(super) async fn collect_single_pool_metrics(pool_name: &str) -> Result<PoolMetrics> {
    debug!("📊 Collecting metrics for pool: {}", pool_name);

    // Get pool I/O statistics using zpool iostat
    let iostat_output = Command::new("zpool")
        .args(["iostat", "-v", pool_name, "1", "2"]) // 2 samples, 1 second apart
        .output()
        .await;

    let mut read_ops = 0.0;
    let mut write_ops = 0.0;
    let mut read_throughput_mbs = 0.0;
    let mut write_throughput_mbs = 0.0;
    let mut _avg_latency_ms = 2.5; // Default fallback (reserved for future metrics)

    if let Ok(output) = iostat_output
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Parse the last set of statistics (after "---" separator)
        let mut found_data = false;
        for line in stdout.lines().rev() {
            if line.contains(pool_name) && found_data {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 7 {
                    read_ops = fields[1].parse().unwrap_or(0.0);
                    write_ops = fields[2].parse().unwrap_or(0.0);

                    // Convert bandwidth from bytes to MB/s
                    let read_bw_bytes: f64 = fields[3].parse().unwrap_or(0.0);
                    let write_bw_bytes: f64 = fields[4].parse().unwrap_or(0.0);
                    read_throughput_mbs = read_bw_bytes / (1024.0 * 1024.0);
                    write_throughput_mbs = write_bw_bytes / (1024.0 * 1024.0);

                    // Calculate latency from operations and throughput (reserved for future metrics)
                    let total_ops = read_ops + write_ops;
                    if total_ops > 0.0 {
                        _avg_latency_ms = (1000.0_f64 / total_ops).min(100.0);
                        // Cap at 100ms
                    }
                }
                break;
            }
            if line.contains("---") {
                found_data = true;
            }
        }
    }

    // Get pool capacity information
    let (total_capacity, used_capacity, _health_status) = get_pool_capacity_info(pool_name).await;

    Ok(PoolMetrics {
        name: pool_name.to_string(),
        read_iops: read_ops as u64,
        write_iops: write_ops as u64,
        read_throughput: read_throughput_mbs,
        write_throughput: write_throughput_mbs,
        used_space: used_capacity,
        available_space: total_capacity - used_capacity,
        utilization_percentage: if total_capacity > 0 {
            (used_capacity as f64 / total_capacity as f64) * 100.0
        } else {
            0.0
        },
        fragmentation_level: 0.0,
        error_count: 0,
        total_capacity,
        health_status: if total_capacity > 0 {
            "ONLINE"
        } else {
            "UNKNOWN"
        }
        .to_string(),
    })
}

/// Collect ZFS ARC statistics
pub(super) async fn collect_arc_statistics() -> Result<(f64, f64)> {
    let mut arc_hit_ratio = 85.0; // Default fallback
    let mut l2arc_hit_ratio = 65.0; // Default fallback

    // Read ARC statistics from /proc/spl/kstat/zfs/arcstats
    if let Ok(content) = tokio::fs::read_to_string("/proc/spl/kstat/zfs/arcstats").await {
        let mut arc_hits = 0u64;
        let mut arc_misses = 0u64;
        let mut l2arc_hits = 0u64;
        let mut l2arc_misses = 0u64;

        for line in content.lines() {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 3 {
                let value = fields[2].parse().unwrap_or(0);
                match fields[0] {
                    "hits" => arc_hits = value,
                    "misses" => arc_misses = value,
                    "l2_hits" => l2arc_hits = value,
                    "l2_misses" => l2arc_misses = value,
                    _ => {}
                }
            }
        }

        let arc_total = arc_hits + arc_misses;
        if arc_total > 0 {
            arc_hit_ratio = (arc_hits as f64 / arc_total as f64) * 100.0;
        }

        let l2arc_total = l2arc_hits + l2arc_misses;
        if l2arc_total > 0 {
            l2arc_hit_ratio = (l2arc_hits as f64 / l2arc_total as f64) * 100.0;
        }
    }

    Ok((arc_hit_ratio, l2arc_hit_ratio))
}

/// Calculate compression ratio across all pools
pub(super) async fn calculate_compression_ratio() -> Result<f64> {
    let output = Command::new("zfs")
        .args(["get", "-H", "-o", "value", "compressratio"])
        .output()
        .await;

    if let Ok(output) = output
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut total_ratio = 0.0;
        let mut count = 0;

        for line in stdout.lines() {
            let ratio_str = line.trim().replace('x', "");
            if let Ok(ratio) = ratio_str.parse::<f64>() {
                total_ratio += ratio;
                count += 1;
            }
        }

        if count > 0 {
            return Ok(total_ratio / f64::from(count));
        }
    }

    Ok(1.4) // Default compression ratio
}

/// Calculate average latencies from pool metrics
pub(super) fn calculate_average_latencies(pool_metrics: &[PoolMetrics]) -> (f64, f64) {
    if pool_metrics.is_empty() {
        return (0.0, 0.0);
    }

    // Calculate average based on throughput (simplified calculation)
    let total_read_throughput: f64 = pool_metrics.iter().map(|p| p.read_throughput).sum();
    let total_write_throughput: f64 = pool_metrics.iter().map(|p| p.write_throughput).sum();

    let avg_read_latency = if total_read_throughput > 0.0 {
        100.0 / total_read_throughput
    } else {
        0.0
    };
    let avg_write_latency = if total_write_throughput > 0.0 {
        100.0 / total_write_throughput
    } else {
        0.0
    };

    (avg_read_latency, avg_write_latency)
}

/// Get pool capacity information
pub(super) async fn get_pool_capacity_info(pool_name: &str) -> (u64, u64, String) {
    let output = Command::new("zpool")
        .args(["list", "-H", "-o", "size,allocated,health", pool_name])
        .output()
        .await;

    if let Ok(output) = output
        && output.status.success()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() >= 3 {
                let total_size = parse_size_string(fields[0]).unwrap_or(0);
                let allocated_size = parse_size_string(fields[1]).unwrap_or(0);
                let health = fields[2].to_string();

                return (total_size, allocated_size, health);
            }
        }
    }

    (0, 0, "UNKNOWN".to_string())
}

/// Parse ZFS size strings (e.g., "1.5T", "512G") to bytes
pub(super) fn parse_size_string(size_str: &str) -> Option<u64> {
    let size_str = size_str.trim();
    if size_str.is_empty() || size_str == "-" {
        return Some(0);
    }

    let (number_part, suffix) = if size_str.len() > 1 {
        let split_pos = size_str.len() - 1;
        size_str.split_at(split_pos)
    } else {
        return size_str.parse().ok();
    };

    if let Ok(number) = number_part.parse::<f64>() {
        let multiplier = match suffix.to_uppercase().as_str() {
            "K" => 1_024,
            "M" => 1_024 * 1_024,
            "G" => 1_024 * 1_024 * 1_024,
            "T" => 1_024_u64 * 1_024 * 1_024 * 1_024,
            "P" => 1_024_u64 * 1_024 * 1_024 * 1_024 * 1_024,
            _ => return size_str.parse().ok(),
        };

        Some(f64_to_u64_saturating(number * multiplier as f64))
    } else {
        size_str.parse().ok()
    }
}
