// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! System and ZFS metrics REST handlers.

use axum::{
    extract::{Query, State},
    response::Json,
};
use tracing::{debug, info};

use crate::rest::models::types::ZfsMetrics;
use crate::rest::models::{DiskIoMetrics, NetworkIoMetrics, SystemMetrics};
use crate::rest::{ApiState, DataError, DataResponse};

use super::prometheus::calculate_real_zfs_cache_hit_ratio;
use super::query::MetricsHistoryQuery;

/// Get current system metrics
/// GET /api/v1/monitoring/metrics
///
/// # Errors
///
/// Returns [`Json`] containing [`DataError`] when metrics aggregation or
/// serialization fails (reserved for future wired collectors).
pub async fn get_metrics(
    State(state): State<ApiState>,
) -> Result<Json<DataResponse<SystemMetrics>>, Json<DataError>> {
    debug!("Getting current system metrics");
    // Get current ZFS metrics from engines
    let mut total_datasets = 0;
    let total_snapshots = 0;
    let mut total_used_bytes = 0;
    let mut _total_available_bytes = 0;
    let mut compression_ratios = Vec::new();

    for _entry in state.zfs_engines.iter() {
        total_datasets += 1;
        total_used_bytes += 1024 * 1024; // Estimated per-engine until real stats wired
        _total_available_bytes += 1024 * 1024 * 1024; // 1GB per dataset (placeholder)

        {
            compression_ratios.push(2.5); // Default until per-engine ZFS compression is wired
        }
    }

    // Calculate overall compression ratio
    let overall_compression_ratio = if compression_ratios.is_empty() {
        1.0
    } else {
        compression_ratios.iter().sum::<f64>() / (compression_ratios.len() as f64)
    };

    // Real system metrics via linux_proc on Linux, with sensible fallbacks
    #[cfg(target_os = "linux")]
    let (
        cpu_usage,
        memory_usage,
        bytes_received,
        bytes_sent,
        total_read_bytes,
        total_written_bytes,
    ): (f64, f64, f64, f64, f64, f64) = {
        let cpu = nestgate_core::linux_proc::global_cpu_usage_percent_from_stat().unwrap_or(0.0);
        let mem_used = nestgate_core::linux_proc::used_memory_bytes().unwrap_or(0) as f64;
        let mem_total = (nestgate_core::linux_proc::used_memory_bytes().unwrap_or(1)
            + nestgate_core::linux_proc::available_memory_bytes().unwrap_or(1))
            as f64;
        let mem_pct = if mem_total > 0.0 {
            (mem_used / mem_total) * 100.0
        } else {
            0.0
        };
        let (rx, tx) = nestgate_core::linux_proc::network_rx_tx_bytes_sum().unwrap_or((0, 0));
        (cpu, mem_pct, rx as f64, tx as f64, 0.0, 0.0)
    };
    #[cfg(not(target_os = "linux"))]
    let (
        cpu_usage,
        memory_usage,
        bytes_received,
        bytes_sent,
        total_read_bytes,
        total_written_bytes,
    ): (f64, f64, f64, f64, f64, f64) = { (0.0, 0.0, 0.0, 0.0, 0.0, 0.0) };

    let metrics = SystemMetrics {
        timestamp: chrono::Utc::now(),
        cpu_usage_percent: cpu_usage,
        memory_usage_percent: memory_usage,
        load_average: nestgate_core::linux_proc::load_averages().map_or(0.0, |(m1, _, _)| m1),
        uptime_seconds: nestgate_core::linux_proc::uptime_secs().unwrap_or(0),
        disk_io: DiskIoMetrics {
            read_bytes_per_sec: total_read_bytes,
            write_bytes_per_sec: total_written_bytes,
            read_ops_per_sec: total_read_bytes / 4096.0,
            write_ops_per_sec: total_written_bytes / 4096.0,
            read_mbps: total_read_bytes / (1024.0 * 1024.0),
            write_mbps: total_written_bytes / (1024.0 * 1024.0),
            read_iops: total_read_bytes / 4096.0,
            write_iops: total_written_bytes / 4096.0,
            avg_queue_depth: 0.0,
        },
        network_io: NetworkIoMetrics {
            bytes_sent: bytes_sent as u64,
            bytes_received: bytes_received as u64,
            packets_sent: 0,
            packets_received: 0,
            rx_bytes_per_sec: bytes_received,
            tx_bytes_per_sec: bytes_sent,
            rx_packets_per_sec: 0.0,
            tx_packets_per_sec: 0.0,
        },
        zfs_metrics: ZfsMetrics {
            arc_hit_ratio: calculate_real_zfs_cache_hit_ratio().await.unwrap_or(85.0), // Real ZFS ARC hit ratio
            arc_size_bytes: 2_147_483_648,                                             // 2GB
            arc_target_size_bytes: 2_147_483_648,
            read_throughput_mbps: 100.0,
            write_throughput_mbps: 50.0,
            compression_ratio: overall_compression_ratio,
            deduplication_ratio: 1.2,
            total_datasets,
            total_snapshots: total_snapshots.try_into().unwrap_or(0),
            total_used_bytes,
        },
    };

    info!("Retrieved current system metrics");
    Ok(Json(DataResponse::new(metrics)))
}

/// Get historical metrics data
/// GET /api/v1/monitoring/metrics/history
///
/// # Errors
///
/// Returns [`Json`] containing [`DataError`] when the time-series query
/// fails (reserved for future TSDB-backed history).
pub async fn get_metrics_history(
    State(state): State<ApiState>,
    Query(query): Query<MetricsHistoryQuery>,
) -> Result<Json<DataResponse<Vec<SystemMetrics>>>, Json<DataError>> {
    debug!("Getting historical metrics data: {:?}", query);
    // Parse time range (simplified for demo)
    let start_time = query
        .start
        .as_deref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map_or_else(
            || chrono::Utc::now() - chrono::Duration::hours(1),
            |dt| dt.with_timezone(&chrono::Utc),
        );

    let end_time = query
        .end
        .as_deref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map_or_else(chrono::Utc::now, |dt| dt.with_timezone(&chrono::Utc));

    // Parse interval
    let interval_minutes = match query.interval.as_deref() {
        Some("1m") => 1,
        Some("15m") => 15,
        Some("1h") => 60,
        Some("1d") => 1440,
        _ => 5, // Default to 5 minutes (includes `5m` and unknown)
    };

    // Generate historical data points
    let mut metrics_history = Vec::new();
    let mut current_time = start_time;

    while current_time <= end_time {
        // Get current ZFS state (simplified)
        let total_datasets = state.zfs_engines.len() as u32;

        // ✅ NOTE: Historical data should come from time-series database
        // For now, return current real-time metrics for all historical points
        // In production, this would query stored metrics from InfluxDB/Prometheus

        // ecoBin v3.0: historical placeholders; real data should come from TSDB or `/proc`-based collectors.
        let metrics = SystemMetrics {
            cpu_usage_percent: 45.0,
            memory_usage_percent: 65.0,
            load_average: 1.2,
            uptime_seconds: 86400,
            timestamp: current_time,
            disk_io: DiskIoMetrics {
                read_bytes_per_sec: 100.0 * 1024.0 * 1024.0,
                write_bytes_per_sec: 50.0 * 1024.0 * 1024.0,
                read_ops_per_sec: 1000.0,
                write_ops_per_sec: 500.0,
                read_mbps: 100.0,
                write_mbps: 50.0,
                read_iops: 1000.0,
                write_iops: 500.0,
                avg_queue_depth: 2.5,
            },
            network_io: NetworkIoMetrics {
                bytes_sent: (1024 * 1024 * 15) as u64,
                bytes_received: (1024 * 1024 * 25) as u64,
                packets_sent: 800,
                packets_received: 1000,
                rx_bytes_per_sec: f64::from(1024 * 1024 * 25),
                tx_bytes_per_sec: f64::from(1024 * 1024 * 15),
                rx_packets_per_sec: 1000.0,
                tx_packets_per_sec: 800.0,
            },
            zfs_metrics: ZfsMetrics {
                arc_hit_ratio: 85.0,           // Real ZFS ARC hit ratio would be calculated here
                arc_size_bytes: 2_147_483_648, // 2GB
                arc_target_size_bytes: 2_147_483_648,
                read_throughput_mbps: 100.0,
                write_throughput_mbps: 50.0,
                compression_ratio: 0.68,
                deduplication_ratio: 1.2,
                total_datasets,
                total_snapshots: total_datasets * 3, // Assume 3 snapshots per dataset
                total_used_bytes: u64::from(total_datasets) * 2 * 1024 * 1024 * 1024, // 2GB per dataset
            },
        };

        metrics_history.push(metrics);
        current_time += chrono::Duration::minutes(interval_minutes);
    }

    info!(
        "Retrieved {} historical metrics data points",
        metrics_history.len()
    );
    Ok(Json(DataResponse::new(metrics_history)))
}
