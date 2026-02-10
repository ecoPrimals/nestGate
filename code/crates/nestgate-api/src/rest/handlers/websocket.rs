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
use tokio::time::{interval, Duration};
use tracing::{debug, error, info};

use crate::rest::models::types::ZfsMetrics;
use crate::rest::models::{DiskIoMetrics, NetworkIoMetrics, SystemMetrics};
use crate::rest::ApiState;

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
        let _message = match serde_json::to_string(&metrics) {
            Ok(json) => Message::Text(json),
            Err(e) => {
                error!("Failed to serialize metrics: {}", e);
                continue;
            }
        };

        if socket.send(_message).await.is_err() {
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

        let _message = match serde_json::to_string(&log_entry) {
            Ok(json) => Message::Text(json),
            Err(e) => {
                error!("Failed to serialize log entry: {}", e);
                continue;
            }
        };

        if socket.send(_message).await.is_err() {
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

/// Get current system metrics
async fn get_current_metrics(state: &ApiState) -> Result<SystemMetrics, String> {
    // Get ZFS metrics from engines
    let engines = state.zfs_engines.read().await;
    let mut total_datasets = 0;
    let total_snapshots = 0;
    let mut total_used_bytes = 0;
    let mut _total_available_bytes = 0;
    let mut compression_ratios = Vec::new();
    for (_name, _engine) in engines.iter() {
        // Placeholder stats - _engine is now just a String
        total_datasets += 1;
        // Use available stats instead of cow_stats
        total_used_bytes += 1024 * 1024; // Placeholder - 1MB per dataset
        _total_available_bytes += 1024 * 1024 * 1024; // 1GB per dataset

        // Access compression stats directly (not optional in ModernZfsStats)
        {
            // Placeholder compression stats
            compression_ratios.push(2.5); // Placeholder compression ratio
        }
    }

    let overall_compression_ratio = if compression_ratios.is_empty() {
        1.0
    } else {
        compression_ratios.iter().sum::<f64>() / (compression_ratios.len() as f64)
    };

    Ok(SystemMetrics {
        cpu_usage_percent: generate_realtime_cpu_usage(),
        memory_usage_percent: generate_realtime_memory_usage(),
        load_average: 0.5,    // Placeholder value
        uptime_seconds: 3600, // Placeholder value
        timestamp: chrono::Utc::now(),
        disk_io: DiskIoMetrics {
            read_bytes_per_sec: generate_realtime_disk_read() * 1024.0 * 1024.0, // Convert MB to bytes
            write_bytes_per_sec: generate_realtime_disk_write() * 1024.0 * 1024.0,
            read_ops_per_sec: generate_realtime_read_iops() as f64,
            write_ops_per_sec: generate_realtime_write_iops() as f64,
            read_mbps: generate_realtime_disk_read(),
            write_mbps: generate_realtime_disk_write(),
            read_iops: generate_realtime_read_iops() as f64,
            write_iops: generate_realtime_write_iops() as f64,
            avg_queue_depth: generate_realtime_queue_depth(),
        },
        network_io: NetworkIoMetrics {
            bytes_sent: (generate_realtime_network_tx() as f64 * 1024.0 * 1024.0) as u64, // Convert MB to bytes
            bytes_received: (generate_realtime_network_rx() as f64 * 1024.0 * 1024.0) as u64,
            packets_sent: generate_realtime_network_tx_packets(),
            packets_received: generate_realtime_network_rx_packets(),
            rx_bytes_per_sec: generate_realtime_network_rx() as f64,
            tx_bytes_per_sec: generate_realtime_network_tx() as f64,
            rx_packets_per_sec: generate_realtime_network_rx_packets() as f64,
            tx_packets_per_sec: generate_realtime_network_tx_packets() as f64,
        },
        zfs_metrics: ZfsMetrics {
            arc_hit_ratio: generate_realtime_cache_hit_ratio(),
            arc_size_bytes: 2_147_483_648, // 2GB
            arc_target_size_bytes: 2_147_483_648,
            read_throughput_mbps: 100.0,
            write_throughput_mbps: 50.0,
            compression_ratio: overall_compression_ratio,
            deduplication_ratio: 1.2,
            total_datasets,
            total_snapshots: total_snapshots.try_into().unwrap_or(0),
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
fn generate_sample_log_entry(level_filter: &str) -> LogEntry {
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

    let _messages = [
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
        message: _messages[(seed % _messages.len() as u64) as usize].to_string(),
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
async fn generate_sample_system_event(state: &ApiState) -> SystemEvent {
    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().timestamp_millis().hash(&mut hasher);
    let seed = hasher.finish();

    let engines = state.zfs_engines.read().await;
    let dataset_count = engines.len();

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
                "cpu_usage": generate_realtime_cpu_usage(),
                "memory_usage": generate_realtime_memory_usage()
            }),
        ),
        "threshold_exceeded" => (
            "Performance threshold exceeded".to_string(),
            serde_json::json!({
                "metric": "cpu_usage_percent",
                "threshold": 80.0,
                "currentvalue": generate_realtime_cpu_usage()
            }),
        ),
        _ => (
            "System event occurred".to_string(),
            serde_json::json!({"info": "Generic system event"}),
        ),
    };

    SystemEvent {
        id: "event_self.base_url".to_string(),
        timestamp: chrono::Utc::now(),
        event_type: (*event_type).to_string(),
        description,
        data,
        severity: (*severity).to_string(),
    }
}

// Real-time metric generators (with more variation than historical)
fn generate_realtime_cpu_usage() -> f64 {
    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().timestamp_millis().hash(&mut hasher);
    let seed = hasher.finish();

    let base = 25.0;
    let variation = ((seed % 200) as f64) * 0.3; // More variation for real-time
    (base + variation).min(95.0)
}

/// Generate Realtime Memory Usage
fn generate_realtime_memory_usage() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 1).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 45.0;
    let variation = ((seed % 150) as f64) * 0.25;
    (base + variation).min(90.0)
}

/// Generate Realtime Disk Read
fn generate_realtime_disk_read() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 2).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 100.0;
    let variation = ((seed % 300) as f64) * 1.5;
    base + variation
}

/// Generate Realtime Disk Write
fn generate_realtime_disk_write() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 3).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 80.0;
    let variation = ((seed % 250) as f64) * 1.2;
    base + variation
}

/// Generate Realtime Read Iops
fn generate_realtime_read_iops() -> u64 {
    (generate_realtime_disk_read() * 120.0) as u64
}

/// Generate Realtime Write Iops
fn generate_realtime_write_iops() -> u64 {
    (generate_realtime_disk_write() * 110.0) as u64
}

/// Generate Realtime Queue Depth
fn generate_realtime_queue_depth() -> f64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 4).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 2.0;
    let variation = ((seed % 200) as f64) * 0.02;
    (base + variation).max(0.1)
}

/// Generate Realtime Network Rx
fn generate_realtime_network_rx() -> u64 {
    let mut hasher = DefaultHasher::new();
    (chrono::Utc::now().timestamp_millis() + 5).hash(&mut hasher);
    let seed = hasher.finish();

    let base = 1024 * 1024; // 1MB/s base
    let variation = seed % (1024 * 1024); // Up to 1MB variation
    base + variation
}

/// Generate Realtime Network Tx
fn generate_realtime_network_tx() -> u64 {
    generate_realtime_network_rx() / 2
}

/// Generate Realtime Network Rx Packets
fn generate_realtime_network_rx_packets() -> u64 {
    generate_realtime_network_rx() / 1400
}

/// Generate Realtime Network Tx Packets
fn generate_realtime_network_tx_packets() -> u64 {
    generate_realtime_network_tx() / 1400
}

/// Generate Realtime Cache Hit Ratio
fn generate_realtime_cache_hit_ratio() -> f64 {
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
}
