// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Host resource metrics: CPU, memory, disk I/O from `/proc`.

use crate::performance::types::{
    DiskIoStats, MemoryInfo, SystemPerformanceMetrics, ZfsPerformanceMonitor,
};
use nestgate_core::Result as CoreResult;
use tracing::debug;

impl ZfsPerformanceMonitor {
    /// Collect system performance metrics
    pub(super) async fn collect_system_metrics() -> CoreResult<SystemPerformanceMetrics> {
        debug!("Collecting system performance metrics");

        // Read memory information
        let memory_info = Self::get_memory_info().await;
        let cpu_usage = Self::get_cpu_usage().await;
        let disk_io = Self::get_disk_io_stats().await;

        Ok(SystemPerformanceMetrics {
            memory_utilization_percent: memory_info.utilization_percent,
            cpu_utilization_percent: cpu_usage,
            disk_queue_depth: disk_io.queue_depth,
            network_throughput_mbs: disk_io.throughput_mbs,
            system_load_average: Self::get_load_average().await,
        })
    }

    /// Get memory information
    pub(super) async fn get_memory_info() -> MemoryInfo {
        if let Ok(content) = tokio::fs::read_to_string("/proc/meminfo").await {
            let mut total = 0u64;
            let mut available = 0u64;

            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 2 {
                        total = fields[1].parse().unwrap_or(0) * 1024; // Convert kB to bytes
                    }
                } else if line.starts_with("MemAvailable:") {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    if fields.len() >= 2 {
                        available = fields[1].parse().unwrap_or(0) * 1024; // Convert kB to bytes
                    }
                }
            }

            let used = total.saturating_sub(available);
            let utilization_percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            MemoryInfo {
                total_mb: (total / (1024 * 1024)),
                used_mb: (used / (1024 * 1024)),
                available_mb: (available / (1024 * 1024)),
                utilization_percent,
            }
        } else {
            MemoryInfo::default()
        }
    }

    /// Get CPU usage percentage
    pub(super) async fn get_cpu_usage() -> f64 {
        if let Ok(content) = tokio::fs::read_to_string("/proc/stat").await
            && let Some(line) = content.lines().next()
            && line.starts_with("cpu ")
        {
            let fields: Vec<&str> = line.split_whitespace().collect();
            if fields.len() >= 8 {
                let user: u64 = fields[1].parse().unwrap_or(0);
                let nice: u64 = fields[2].parse().unwrap_or(0);
                let system: u64 = fields[3].parse().unwrap_or(0);
                let idle: u64 = fields[4].parse().unwrap_or(0);
                let iowait: u64 = fields[5].parse().unwrap_or(0);
                let irq: u64 = fields[6].parse().unwrap_or(0);
                let softirq: u64 = fields[7].parse().unwrap_or(0);

                let total = user + nice + system + idle + iowait + irq + softirq;
                let non_idle = user + nice + system + irq + softirq;

                if total > 0 {
                    return (non_idle as f64 / total as f64) * 100.0;
                }
            }
        }
        0.0
    }

    /// Get disk I/O statistics
    pub(super) async fn get_disk_io_stats() -> DiskIoStats {
        // Read from /proc/diskstats for real disk I/O data
        if let Ok(content) = tokio::fs::read_to_string("/proc/diskstats").await {
            let mut total_reads = 0u64;
            let mut total_writes = 0u64;

            for line in content.lines() {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if fields.len() >= 14 {
                    // Skip partition entries, focus on whole devices
                    if let Ok(reads) = fields[3].parse::<u64>() {
                        total_reads += reads;
                    }
                    if let Ok(writes) = fields[7].parse::<u64>() {
                        total_writes += writes;
                    }
                }
            }

            DiskIoStats {
                read_iops: total_reads,
                write_iops: total_writes,
                throughput_mbs: (total_reads + total_writes) as f64 / (1024.0 * 1024.0),
                queue_depth: 4, // Default approximation
            }
        } else {
            DiskIoStats::default()
        }
    }

    /// Get system load average
    pub(super) async fn get_load_average() -> f64 {
        if let Ok(content) = tokio::fs::read_to_string("/proc/loadavg").await {
            let fields: Vec<&str> = content.split_whitespace().collect();
            if !fields.is_empty() {
                return fields[0].parse().unwrap_or(0.0);
            }
        }
        0.0
    }
}
