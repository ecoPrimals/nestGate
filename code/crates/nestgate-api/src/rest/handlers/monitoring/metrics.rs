// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
    let total_datasets = u32::try_from(state.zfs_engines.len()).unwrap_or(u32::MAX);

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
        let cpu = nestgate_core::linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0);
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
            arc_hit_ratio: calculate_real_zfs_cache_hit_ratio().await.unwrap_or(0.0),
            arc_size_bytes: 0,
            arc_target_size_bytes: 0,
            read_throughput_mbps: 0.0,
            write_throughput_mbps: 0.0,
            compression_ratio: 0.0,
            deduplication_ratio: 0.0,
            total_datasets,
            total_snapshots: 0,
            total_used_bytes: 0,
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
    State(_state): State<ApiState>,
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

    // TSDB not yet wired — return empty history rather than fabricated data.
    // When a time-series backend (Prometheus/VictoriaMetrics) is integrated,
    // this endpoint will query stored samples for the requested range.
    let metrics_history: Vec<SystemMetrics> = Vec::new();

    info!(
        "Metrics history requested ({} to {}, interval {}m) — TSDB not yet wired, returning empty",
        start_time, end_time, interval_minutes
    );
    Ok(Json(DataResponse::new(metrics_history)))
}
