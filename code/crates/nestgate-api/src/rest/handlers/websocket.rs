// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Pure data layer WebSocket handlers for real-time data streams.
// These handlers provide live data feeds for management dashboards
// without any authentication or user management overhead.

//! Websocket module

use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{Query, State, WebSocketUpgrade},
    response::Response,
};
use serde::Deserialize;
use std::hash::{DefaultHasher, Hash, Hasher};
use tokio::time::{Duration, interval};
use tracing::{debug, error, info};

use crate::rest::ApiState;
use crate::rest::models::types::ZfsMetrics;
use crate::rest::models::{DiskIoMetrics, NetworkIoMetrics, SystemMetrics};

// ==================== SECTION ====================
// WEBSOCKET DATA HANDLERS
// ==================== SECTION ====================

/// Query parameters for WebSocket connections
#[derive(Debug, Deserialize)]
/// Websocketquery
pub struct WebSocketQuery {
    /// Update interval in seconds
    pub interval: Option<u64>,
    /// Log level filter (for logs endpoint)
    pub level: Option<String>,
}
/// Live metrics WebSocket stream
/// GET /ws/metrics
pub async fn metrics_websocket(
    ws: WebSocketUpgrade,
    State(state): State<ApiState>,
    Query(query): Query<WebSocketQuery>,
) -> Response {
    debug!("Upgrading to metrics WebSocket connection");
    ws.on_upgrade(move |socket| handle_metrics_websocket(socket, state, query))
}

/// Live logs WebSocket stream\
/// GET /ws/logs
pub async fn logs_websocket(
    ws: WebSocketUpgrade,
    State(state): State<ApiState>,
    Query(query): Query<WebSocketQuery>,
) -> Response {
    debug!("Upgrading to logs WebSocket connection");
    ws.on_upgrade(move |socket| handle_logs_websocket(socket, state, query))
}

/// System events WebSocket stream
/// GET /ws/events
pub async fn events_websocket(
    ws: WebSocketUpgrade,
    State(state): State<ApiState>,
    Query(query): Query<WebSocketQuery>,
) -> Response {
    debug!("Upgrading to events WebSocket connection");
    ws.on_upgrade(move |socket| handle_events_websocket(socket, state, query))
}

// ==================== SECTION ====================
// WEBSOCKET HANDLERS
// ==================== SECTION ====================

/// Handle metrics WebSocket connection
async fn handle_metrics_websocket(mut socket: WebSocket, state: ApiState, query: WebSocketQuery) {
    info!("Metrics WebSocket connection established");
    let update_interval = Duration::from_secs(query.interval.unwrap_or(5));
    let mut ticker = interval(update_interval);

    loop {
        ticker.tick().await;

        // Get current metrics
        let metrics = match get_current_metrics(&state).await {
            Ok(metrics) => metrics,
            Err(e) => {
                error!("Failed to get metrics: {}", e);
                continue;
            }
        };

        // Send metrics as JSON
        let message = match serde_json::to_string(&metrics) {
            Ok(json) => Message::Text(json),
            Err(e) => {
                error!("Failed to serialize metrics: {}", e);
                continue;
            }
        };

        if socket.send(message).await.is_err() {
            debug!("Metrics WebSocket connection closed");
            break;
        }
    }

    info!("Metrics WebSocket connection terminated");
}

/// Handle logs WebSocket connection
async fn handle_logs_websocket(mut socket: WebSocket, _state: ApiState, query: WebSocketQuery) {
    info!("Logs WebSocket connection established");
    let level_filter = query.level.unwrap_or_else(|| "info".to_string());
    let update_interval = Duration::from_secs(query.interval.unwrap_or(1));
    let mut ticker = interval(update_interval);

    loop {
        ticker.tick().await;

        // Generate sample log entries (in production, would stream real logs)
        let log_entry = generate_sample_log_entry(&level_filter);

        let message = match serde_json::to_string(&log_entry) {
            Ok(json) => Message::Text(json),
            Err(e) => {
                error!("Failed to serialize log entry: {}", e);
                continue;
            }
        };

        if socket.send(message).await.is_err() {
            debug!("Logs WebSocket connection closed");
            break;
        }
    }

    info!("Logs WebSocket connection terminated");
}

/// Handle events WebSocket connection
async fn handle_events_websocket(mut socket: WebSocket, state: ApiState, query: WebSocketQuery) {
    info!("Events WebSocket connection established");
    let update_interval = Duration::from_secs(query.interval.unwrap_or(10));
    let mut ticker = interval(update_interval);

    loop {
        ticker.tick().await;

        // Generate sample system events (in production, would stream real events)
        let event = generate_sample_system_event(&state).await;

        let message = match serde_json::to_string(&event) {
            Ok(json) => Message::Text(json),
            Err(e) => {
                error!("Failed to serialize event: {}", e);
                continue;
            }
        };

        if socket.send(message).await.is_err() {
            debug!("Events WebSocket connection closed");
            break;
        }
    }

    info!("Events WebSocket connection terminated");
}

// ==================== SECTION ====================
// HELPER FUNCTIONS
// ==================== SECTION ====================

#[cfg(target_os = "linux")]
fn arcstats_named(content: &str, name: &str) -> u64 {
    for line in content.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        if f.len() >= 3 && f[0] == name {
            return f[2].parse().unwrap_or(0);
        }
    }
    0
}

/// ARC hit ratio (0–1), current size, target `c` from `/proc/spl/kstat/zfs/arcstats` when present.
fn zfs_arc_snapshot() -> (f64, u64, u64) {
    #[cfg(target_os = "linux")]
    {
        let Ok(content) = std::fs::read_to_string("/proc/spl/kstat/zfs/arcstats") else {
            return (0.0, 0, 0);
        };
        let hits = arcstats_named(&content, "hits");
        let misses = arcstats_named(&content, "misses");
        let size = arcstats_named(&content, "size");
        let c = arcstats_named(&content, "c");
        let total = hits.saturating_add(misses);
        let ratio = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };
        (ratio, size, c)
    }
    #[cfg(not(target_os = "linux"))]
    {
        (0.0, 0, 0)
    }
}

/// Get current system metrics
#[expect(
    clippy::unused_async,
    reason = "cfg(test) awaits this helper; metrics assembly is synchronous"
)]
async fn get_current_metrics(state: &ApiState) -> Result<SystemMetrics, String> {
    let total_datasets = u32::try_from(state.zfs_engines.len()).unwrap_or(u32::MAX);
    let total_snapshots = u32::try_from(super::zfs::helpers::get_snapshot_count_from_engine_impl())
        .unwrap_or(u32::MAX);
    // No aggregated ZFS-used-bytes path on this REST surface; report 0 instead of inventing totals.
    let total_used_bytes = 0_u64;
    let (arc_hit_ratio, arc_size, arc_target) = zfs_arc_snapshot();

    let load_average = std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|c| {
            c.split_whitespace()
                .next()
                .and_then(|s| s.parse::<f64>().ok())
        })
        .unwrap_or(0.0);

    let uptime_seconds = std::fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|c| {
            c.split_whitespace()
                .next()
                .and_then(|s| s.parse::<f64>().ok())
        })
        .map_or(0, |s| s.clamp(0.0, u64::MAX as f64) as u64);

    let cpu_usage_percent =
        nestgate_core::linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0);
    let memory_usage_percent = nestgate_core::linux_proc::memory_usage_percent().unwrap_or(0.0);

    Ok(SystemMetrics {
        cpu_usage_percent,
        memory_usage_percent,
        load_average,
        uptime_seconds,
        timestamp: chrono::Utc::now(),
        disk_io: DiskIoMetrics {
            // Per-second I/O is not computed here (would require sampled /proc deltas).
            read_bytes_per_sec: 0.0,
            write_bytes_per_sec: 0.0,
            read_ops_per_sec: 0.0,
            write_ops_per_sec: 0.0,
            read_mbps: 0.0,
            write_mbps: 0.0,
            read_iops: 0.0,
            write_iops: 0.0,
            avg_queue_depth: 0.0,
        },
        network_io: NetworkIoMetrics {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            rx_bytes_per_sec: 0.0,
            tx_bytes_per_sec: 0.0,
            rx_packets_per_sec: 0.0,
            tx_packets_per_sec: 0.0,
        },
        zfs_metrics: ZfsMetrics {
            arc_hit_ratio,
            arc_size_bytes: arc_size,
            arc_target_size_bytes: arc_target,
            read_throughput_mbps: 0.0,
            write_throughput_mbps: 0.0,
            compression_ratio: 1.0,
            deduplication_ratio: 1.0,
            total_datasets,
            total_snapshots,
            total_used_bytes,
        },
    })
}

/// Log entry structure for WebSocket streaming
#[derive(Debug, serde::Serialize)]
/// Logentry
pub struct LogEntry {
    /// Timestamp when the log entry was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Log level (DEBUG, INFO, WARN, ERROR)
    pub level: String,
    /// Log message content
    pub message: String,
    /// Module that generated the log entry
    pub module: String,
    /// Thread that generated the log entry
    pub thread: String,
}
/// Generate sample log entry
pub(crate) fn generate_sample_log_entry(level_filter: &str) -> LogEntry {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().timestamp_millis().hash(&mut hasher);
    let seed = hasher.finish();

    let levels = match level_filter {
        "debug" => vec!["DEBUG", "INFO", "WARN", "ERROR"],
        "info" => vec!["INFO", "WARN", "ERROR"],
        "warn" => vec!["WARN", "ERROR"],
        "error" => vec!["ERROR"],
        _ => vec!["INFO", "WARN"],
    };

    let level = levels[(seed % levels.len() as u64) as usize];

    let messages = [
        "ZFS dataset operation completed successfully",
        "Storage backend health check passed",
        "Snapshot created for dataset tank/data",
        "Compression ratio improved to 0.72",
        "Auto-configurator detected new storage",
        "Metrics collection interval updated",
        "WebSocket connection established",
        "System resources within normal limits",
        "Cache hit ratio: 94.2%",
        "Background cleanup task finished",
    ];

    let modules = [
        "nestgate::zfs",
        "nestgate::storage",
        "nestgate::monitoring",
        "nestgate::api",
        "nestgate::websocket",
    ];

    LogEntry {
        timestamp: chrono::Utc::now(),
        level: level.to_string(),
        message: messages[(seed % messages.len() as u64) as usize].to_string(),
        module: modules[((seed >> 8) % modules.len() as u64) as usize].to_string(),
        thread: format!("worker-{}", ((seed >> 16) % 8) + 1),
    }
}

/// System event structure for WebSocket streaming
#[derive(Debug, serde::Serialize)]
/// Systemevent
pub struct SystemEvent {
    /// Unique identifier for the event
    pub id: String,
    /// Timestamp when the event occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Type/category of the event
    pub event_type: String,
    /// Human-readable description of the event
    pub description: String,
    /// Additional event data in JSON format
    pub data: serde_json::Value,
    /// Event severity level
    pub severity: String,
}
/// Generate sample system event
#[expect(
    clippy::unused_async,
    reason = "cfg(test) awaits this helper; event construction is synchronous"
)]
async fn generate_sample_system_event(state: &ApiState) -> SystemEvent {
    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().timestamp_millis().hash(&mut hasher);
    let seed = hasher.finish();

    let dataset_count = state.zfs_engines.len();

    let event_types = [
        ("dataset_created", "info"),
        ("snapshot_taken", "info"),
        ("storage_scanned", "info"),
        ("metrics_updated", "debug"),
        ("threshold_exceeded", "warning"),
        ("system_startup", "info"),
    ];

    let (event_type, severity) = &event_types[(seed % event_types.len() as u64) as usize];

    let (description, data) = match *event_type {
        "dataset_created" => (
            "New ZFS dataset created".to_string(),
            serde_json::json!({
                "dataset_name": format!("tank/data_{}", ((seed >> 8) % 100)),
                "backend": "filesystem",
                "compression": true
            }),
        ),
        "snapshot_taken" => (
            "Automatic snapshot created".to_string(),
            serde_json::json!({
                "dataset": "tank/data",
                "snapshot_name": format!("auto-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")),
                "size_bytes": (seed % 1_000_000) + 1_000_000
            }),
        ),
        "storage_scanned" => (
            "Storage scan completed".to_string(),
            serde_json::json!({
                "backends_found": (seed % 5) + 2,
                "scan_duration_ms": (seed % 5000) + 1000
            }),
        ),
        "metrics_updated" => (
            "System metrics refreshed".to_string(),
            serde_json::json!({
                "datasets": dataset_count,
                "cpu_usage": nestgate_core::linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0),
                "memory_usage": nestgate_core::linux_proc::memory_usage_percent().unwrap_or(0.0)
            }),
        ),
        "threshold_exceeded" => (
            "Performance threshold exceeded".to_string(),
            serde_json::json!({
                "metric": "cpu_usage_percent",
                "threshold": 80.0,
                "currentvalue": nestgate_core::linux_proc::globalcpu_usage_percent_from_stat().unwrap_or(0.0)
            }),
        ),
        _ => (
            "System event occurred".to_string(),
            serde_json::json!({"info": "Generic system event"}),
        ),
    };

    SystemEvent {
        id: format!("event_{seed}"),
        timestamp: chrono::Utc::now(),
        event_type: (*event_type).to_string(),
        description,
        data,
        severity: (*severity).to_string(),
    }
}

// Test-only hash-based generators (production WebSocket metrics use `/proc` + `linux_proc`).
#[cfg(test)]
pub(crate) fn generate_realtimecpu_usage() -> f64 {
    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().timestamp_millis().hash(&mut hasher);
    let seed = hasher.finish();

    let base = 25.0;
    let variation = ((seed % 200) as f64) * 0.3; // More variation for real-time
    (base + variation).min(95.0)
}

/// Generate Realtime Memory Usage
#[cfg(test)]
pub(crate) fn generate_realtime_memory_usage() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 1).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 45.0;
    let variation = ((seed % 150) as f64) * 0.25;
    (base + variation).min(90.0)
}

/// Generate Realtime Disk Read
#[cfg(test)]
pub(crate) fn generate_realtime_disk_read() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 2).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 100.0;
    let variation = ((seed % 300) as f64) * 1.5;
    base + variation
}

/// Generate Realtime Disk Write
#[cfg(test)]
pub(crate) fn generate_realtime_disk_write() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 3).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 80.0;
    let variation = ((seed % 250) as f64) * 1.2;
    base + variation
}

/// Generate Realtime Read Iops
#[cfg(test)]
pub(crate) fn generate_realtime_read_iops() -> u64 {
    (generate_realtime_disk_read() * 120.0) as u64
}

/// Generate Realtime Write Iops
#[cfg(test)]
pub(crate) fn generate_realtime_write_iops() -> u64 {
    (generate_realtime_disk_write() * 110.0) as u64
}

/// Generate Realtime Queue Depth
#[cfg(test)]
pub(crate) fn generate_realtime_queue_depth() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 4).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 2.0;
    let variation = ((seed % 200) as f64) * 0.02;
    (base + variation).max(0.1)
}

/// Generate Realtime Network Rx
#[cfg(test)]
pub(crate) fn generate_realtime_network_rx() -> u64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 5).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 1024 * 1024; // 1MB/s base
    let variation = seed % (1024 * 1024); // Up to 1MB variation
    base + variation
}

/// Generate Realtime Network Tx
#[cfg(test)]
pub(crate) fn generate_realtime_network_tx() -> u64 {
    generate_realtime_network_rx() / 2
}

/// Generate Realtime Network Rx Packets
#[cfg(test)]
pub(crate) fn generate_realtime_network_rx_packets() -> u64 {
    generate_realtime_network_rx() / 1400
}

/// Generate Realtime Network Tx Packets
#[cfg(test)]
pub(crate) fn generate_realtime_network_tx_packets() -> u64 {
    generate_realtime_network_tx() / 1400
}

/// Generate Realtime Cache Hit Ratio
#[cfg(test)]
pub(crate) fn generate_realtime_cache_hit_ratio() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 6).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 0.85;
    let variation = ((seed % 100) as f64) * 0.002; // Small real-time variation
    (base + variation).min(0.99).max(0.70)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_api_state() -> ApiState {
        ApiState::new().expect("Failed to create test state")
    }

    #[test]
    fn test_websocket_query_debug() {
        let query = WebSocketQuery {
            interval: Some(5),
            level: Some("info".to_string()),
        };
        assert_eq!(query.interval, Some(5));
        assert_eq!(query.level.as_deref(), Some("info"));
    }

    #[test]
    fn test_log_entry_serialization() {
        let entry = LogEntry {
            timestamp: chrono::Utc::now(),
            level: "INFO".to_string(),
            message: "Test message".to_string(),
            module: "nestgate::test".to_string(),
            thread: "worker-1".to_string(),
        };
        let json = serde_json::to_string(&entry);
        assert!(json.is_ok());
        let parsed: serde_json::Value = serde_json::from_str(&json.unwrap()).unwrap();
        assert_eq!(parsed["level"], "INFO");
        assert_eq!(parsed["message"], "Test message");
    }

    #[test]
    fn test_system_event_serialization() {
        let event = SystemEvent {
            id: "evt_1".to_string(),
            timestamp: chrono::Utc::now(),
            event_type: "dataset_created".to_string(),
            description: "Test event".to_string(),
            data: serde_json::json!({"key": "value"}),
            severity: "info".to_string(),
        };
        let json = serde_json::to_string(&event);
        assert!(json.is_ok());
        let parsed: serde_json::Value = serde_json::from_str(&json.unwrap()).unwrap();
        assert_eq!(parsed["event_type"], "dataset_created");
    }

    #[test]
    fn test_generate_sample_log_entry() {
        let entry = generate_sample_log_entry("info");
        assert!(["DEBUG", "INFO", "WARN", "ERROR"].contains(&entry.level.as_str()));
        assert!(!entry.message.is_empty());
        assert!(!entry.module.is_empty());
    }

    #[tokio::test]
    async fn test_generate_sample_system_event() {
        let state = create_test_api_state();
        let event = generate_sample_system_event(&state).await;
        assert!(!event.id.is_empty());
        assert!(!event.event_type.is_empty());
        assert!(!event.description.is_empty());
    }

    #[tokio::test]
    async fn test_get_current_metrics() {
        let state = create_test_api_state();
        let result = get_current_metrics(&state).await;
        assert!(result.is_ok());
        let metrics = result.unwrap();
        assert!(metrics.cpu_usage_percent >= 0.0);
        assert!(metrics.memory_usage_percent >= 0.0);
    }

    #[test]
    fn generate_sample_log_entry_level_branches() {
        let d = generate_sample_log_entry("debug");
        assert!(["DEBUG", "INFO", "WARN", "ERROR"].contains(&d.level.as_str()));
        let w = generate_sample_log_entry("warn");
        assert!(["WARN", "ERROR"].contains(&w.level.as_str()));
        let e = generate_sample_log_entry("error");
        assert_eq!(e.level, "ERROR");
        let o = generate_sample_log_entry("verbose-unknown");
        assert!(["INFO", "WARN"].contains(&o.level.as_str()));
    }

    #[tokio::test]
    async fn generate_sample_system_event_covers_event_variants() {
        let state = create_test_api_state();
        for _ in 0..48 {
            let ev = generate_sample_system_event(&state).await;
            assert!(!ev.id.is_empty());
            assert!(!ev.event_type.is_empty());
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn arcstats_named_reads_known_line() {
        let content = "hits 4 100\nmisses 4 50\n";
        assert_eq!(super::arcstats_named(content, "hits"), 100);
        assert_eq!(super::arcstats_named(content, "nope"), 0);
    }
}
