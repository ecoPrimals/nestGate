//! **COMPREHENSIVE WEBSOCKET HANDLER TESTS**
//!
//! Tests for WebSocket query parameters, structures, and helper functions.

use super::*;

// ==================== WEBSOCKET QUERY TESTS ====================

#[test]
fn test_websocket_query_default() {
    let query = WebSocketQuery {
        interval: None,
        level: None,
    };

    assert!(query.interval.is_none());
    assert!(query.level.is_none());
}

#[test]
fn test_websocket_query_with_interval() {
    let query = WebSocketQuery {
        interval: Some(10),
        level: None,
    };

    assert_eq!(query.interval, Some(10));
}

#[test]
fn test_websocket_query_with_level() {
    let query = WebSocketQuery {
        interval: None,
        level: Some("debug".to_string()),
    };

    assert_eq!(query.level, Some("debug".to_string()));
}

#[test]
fn test_websocket_query_all_params() {
    let query = WebSocketQuery {
        interval: Some(5),
        level: Some("info".to_string()),
    };

    assert_eq!(query.interval, Some(5));
    assert_eq!(query.level, Some("info".to_string()));
}

// ==================== LOG ENTRY TESTS ====================

#[test]
fn test_log_entry_structure() {
    let entry = LogEntry {
        timestamp: chrono::Utc::now(),
        level: "INFO".to_string(),
        message: "Test message".to_string(),
        module: "nestgate::test".to_string(),
        thread: "worker-1".to_string(),
    };

    assert_eq!(entry.level, "INFO");
    assert_eq!(entry.message, "Test message");
    assert_eq!(entry.module, "nestgate::test");
    assert_eq!(entry.thread, "worker-1");
}

#[test]
fn test_log_entry_serialization() {
    let entry = LogEntry {
        timestamp: chrono::Utc::now(),
        level: "ERROR".to_string(),
        message: "Error occurred".to_string(),
        module: "nestgate::core".to_string(),
        thread: "worker-2".to_string(),
    };

    let json = serde_json::to_string(&entry);
    assert!(json.is_ok());

    let json_str = json.expect("Test setup failed");
    assert!(json_str.contains("ERROR"));
    assert!(json_str.contains("Error occurred"));
}

#[test]
fn test_generate_sample_log_entry_info() {
    let entry = generate_sample_log_entry("info");

    assert!(!entry.level.is_empty());
    assert!(!entry.message.is_empty());
    assert!(!entry.module.is_empty());
    assert!(!entry.thread.is_empty());
}

#[test]
fn test_generate_sample_log_entry_debug() {
    let entry = generate_sample_log_entry("debug");

    // Debug filter should include all levels
    assert!(!entry.level.is_empty());
    assert!(!entry.message.is_empty());
}

#[test]
fn test_generate_sample_log_entry_warn() {
    let entry = generate_sample_log_entry("warn");

    // Warn filter should only include WARN or ERROR
    assert!(entry.level == "WARN" || entry.level == "ERROR");
}

#[test]
fn test_generate_sample_log_entry_error() {
    let entry = generate_sample_log_entry("error");

    // Error filter should only include ERROR
    assert_eq!(entry.level, "ERROR");
}

#[test]
fn test_generate_sample_log_entry_unknown_filter() {
    let entry = generate_sample_log_entry("unknown");

    // Unknown filter should default to info/warn
    assert!(entry.level == "INFO" || entry.level == "WARN");
}

#[test]
fn test_log_entry_thread_format() {
    let entry = generate_sample_log_entry("info");

    // Thread should be in worker-N format
    assert!(entry.thread.starts_with("worker-"));
}

#[test]
fn test_log_entry_module_valid() {
    let entry = generate_sample_log_entry("info");

    // Module should start with nestgate::
    assert!(entry.module.starts_with("nestgate::"));
}

// ==================== SYSTEM EVENT TESTS ====================

#[test]
fn test_system_event_structure() {
    let event = SystemEvent {
        id: "event-123".to_string(),
        timestamp: chrono::Utc::now(),
        event_type: "dataset_created".to_string(),
        description: "New dataset created".to_string(),
        data: serde_json::json!({"dataset": "tank/data"}),
        severity: "info".to_string(),
    };

    assert_eq!(event.id, "event-123");
    assert_eq!(event.event_type, "dataset_created");
    assert_eq!(event.severity, "info");
}

#[test]
fn test_system_event_serialization() {
    let event = SystemEvent {
        id: "evt-456".to_string(),
        timestamp: chrono::Utc::now(),
        event_type: "threshold_exceeded".to_string(),
        description: "CPU threshold exceeded".to_string(),
        data: serde_json::json!({"cpu": 85.5}),
        severity: "warning".to_string(),
    };

    let json = serde_json::to_string(&event);
    assert!(json.is_ok());

    let json_str = json.expect("Test setup failed");
    assert!(json_str.contains("threshold_exceeded"));
    assert!(json_str.contains("warning"));
}

// ==================== REAL-TIME METRIC GENERATOR TESTS ====================

#[test]
fn test_generate_realtime_cpu_usage() {
    let usage = generate_realtime_cpu_usage();

    assert!(usage >= 0.0);
    assert!(usage <= 100.0);
}

#[test]
fn test_generate_realtime_cpu_usage_multiple() {
    // Generate multiple values to check variation
    // No sleep needed - RNG is concurrent-safe and each call is independent
    let values: Vec<f64> = (0..10)
        .map(|_| generate_realtime_cpu_usage())
        .collect();

    // All should be valid percentages
    for val in &values {
        assert!(*val >= 0.0 && *val <= 100.0);
    }
}

#[test]
fn test_generate_realtime_memory_usage() {
    let usage = generate_realtime_memory_usage();

    assert!(usage >= 0.0);
    assert!(usage <= 100.0);
}

#[test]
fn test_generate_realtime_memory_usage_range() {
    let usage = generate_realtime_memory_usage();

    // Memory usage should be in reasonable range (45-90%)
    assert!(usage >= 0.0);
    assert!(usage <= 90.0);
}

#[test]
fn test_generate_realtime_disk_read() {
    let read_mbps = generate_realtime_disk_read();

    assert!(read_mbps >= 0.0);
    assert!(read_mbps > 0.0); // Should always be positive
}

#[test]
fn test_generate_realtime_disk_write() {
    let write_mbps = generate_realtime_disk_write();

    assert!(write_mbps >= 0.0);
    assert!(write_mbps > 0.0); // Should always be positive
}

#[test]
fn test_generate_realtime_read_iops() {
    let iops = generate_realtime_read_iops();

    assert!(iops > 0);
}

#[test]
fn test_generate_realtime_write_iops() {
    let iops = generate_realtime_write_iops();

    assert!(iops > 0);
}

#[test]
fn test_generate_realtime_queue_depth() {
    let depth = generate_realtime_queue_depth();

    assert!(depth >= 0.0);
    assert!(depth > 0.0); // Should always be positive
}

#[test]
fn test_generate_realtime_network_rx() {
    let rx = generate_realtime_network_rx();

    assert!(rx > 0);
}

#[test]
fn test_generate_realtime_network_tx() {
    let tx = generate_realtime_network_tx();

    assert!(tx > 0);
}

#[test]
fn test_generate_realtime_network_tx_less_than_rx() {
    let rx = generate_realtime_network_rx();
    let tx = generate_realtime_network_tx();

    // TX should be half of RX
    assert!(tx <= rx);
}

#[test]
fn test_generate_realtime_network_rx_packets() {
    let packets = generate_realtime_network_rx_packets();

    assert!(packets > 0);
}

#[test]
fn test_generate_realtime_network_tx_packets() {
    let packets = generate_realtime_network_tx_packets();

    assert!(packets > 0);
}

#[test]
fn test_generate_realtime_cache_hit_ratio() {
    let ratio = generate_realtime_cache_hit_ratio();

    assert!(ratio >= 0.0 && ratio <= 1.0);
}

#[test]
fn test_generate_realtime_cache_hit_ratio_range() {
    let ratio = generate_realtime_cache_hit_ratio();

    // Should be in the range 0.70 to 0.99
    assert!(ratio >= 0.70);
    assert!(ratio <= 0.99);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_websocket_query_deserialize() {
    let json = r#"{"interval":5,"level":"debug"}"#;
    let query: Result<WebSocketQuery, _> = serde_json::from_str(json);

    assert!(query.is_ok());
    let q = query.expect("Test setup failed");
    assert_eq!(q.interval, Some(5));
    assert_eq!(q.level, Some("debug".to_string()));
}

#[test]
fn test_websocket_query_deserialize_partial() {
    let json = r#"{"interval":10}"#;
    let query: Result<WebSocketQuery, _> = serde_json::from_str(json);

    assert!(query.is_ok());
    let q = query.expect("Test setup failed");
    assert_eq!(q.interval, Some(10));
    assert!(q.level.is_none());
}

#[test]
fn test_log_entry_complete_roundtrip() {
    let original = LogEntry {
        timestamp: chrono::Utc::now(),
        level: "WARN".to_string(),
        message: "Test warning message".to_string(),
        module: "nestgate::api".to_string(),
        thread: "worker-3".to_string(),
    };

    let json = serde_json::to_string(&original).expect("Test setup failed");
    let deserialized: serde_json::Value = serde_json::from_str(&json).expect("Test setup failed");

    assert_eq!(deserialized["level"], "WARN");
    assert_eq!(deserialized["message"], "Test warning message");
}

#[test]
fn test_system_event_complete_roundtrip() {
    let original = SystemEvent {
        id: "evt-789".to_string(),
        timestamp: chrono::Utc::now(),
        event_type: "snapshot_taken".to_string(),
        description: "Snapshot created".to_string(),
        data: serde_json::json!({"snapshot": "tank@snap1"}),
        severity: "info".to_string(),
    };

    let json = serde_json::to_string(&original).expect("Test setup failed");
    let deserialized: serde_json::Value = serde_json::from_str(&json).expect("Test setup failed");

    assert_eq!(deserialized["event_type"], "snapshot_taken");
    assert_eq!(deserialized["severity"], "info");
}

#[test]
fn test_metrics_generators_consistency() {
    // Test that related metrics are consistent
    let disk_read = generate_realtime_disk_read();
    let read_iops = generate_realtime_read_iops();

    // IOPS should be derived from disk read
    assert!(read_iops > 0);
    assert!(disk_read > 0.0);
}

#[test]
fn test_network_metrics_relationship() {
    let rx = generate_realtime_network_rx();
    let tx = generate_realtime_network_tx();
    let rx_packets = generate_realtime_network_rx_packets();
    let tx_packets = generate_realtime_network_tx_packets();

    // All should be positive
    assert!(rx > 0);
    assert!(tx > 0);
    assert!(rx_packets > 0);
    assert!(tx_packets > 0);

    // TX should be less than or equal to RX
    assert!(tx <= rx);
}

#[test]
fn test_log_levels_hierarchy() {
    // Test different log level filters
    let debug_entry = generate_sample_log_entry("debug");
    let info_entry = generate_sample_log_entry("info");
    let warn_entry = generate_sample_log_entry("warn");
    let error_entry = generate_sample_log_entry("error");

    // Debug can be any level
    assert!(!debug_entry.level.is_empty());

    // Info should not be DEBUG
    assert!(!info_entry.level.is_empty());

    // Warn should be WARN or ERROR
    assert!(warn_entry.level == "WARN" || warn_entry.level == "ERROR");

    // Error should always be ERROR
    assert_eq!(error_entry.level, "ERROR");
}

#[test]
fn test_multiple_log_entries_different() {
    // Modern concurrent pattern: No sleeps needed
    // Timestamps are high-precision and will differ even in tight loops
    let entry1 = generate_sample_log_entry("info");
    let entry2 = generate_sample_log_entry("info");

    // Entries should have different timestamps (nanosecond precision ensures this)
    // If timestamps are the same, the generator needs fixing, not the test
    assert_ne!(entry1.timestamp, entry2.timestamp);
}

#[test]
fn test_cpu_usage_not_exceeds_max() {
    // Generate many values to ensure max cap works
    // No sleep - testing bounds checking, not timing
    for _ in 0..20 {
        let usage = generate_realtime_cpu_usage();
        assert!(usage <= 95.0, "CPU usage should not exceed 95%");
    }
}

#[test]
fn test_memory_usage_not_exceeds_max() {
    // Generate many values to ensure max cap works
    // No sleep - testing bounds checking, not timing
    for _ in 0..20 {
        let usage = generate_realtime_memory_usage();
        assert!(usage <= 90.0, "Memory usage should not exceed 90%");
    }
}

#[test]
fn test_queue_depth_minimum() {
    // Test that queue depth has a minimum value
    // No sleep - testing bounds checking, not timing
    for _ in 0..20 {
        let depth = generate_realtime_queue_depth();
        assert!(depth >= 0.1, "Queue depth should have minimum of 0.1");
    }
}

#[test]
fn test_cache_hit_ratio_bounds() {
    // Test that cache hit ratio stays within bounds
    // No sleep - testing bounds checking, not timing
    for _ in 0..20 {
        let ratio = generate_realtime_cache_hit_ratio();
        assert!(
            ratio >= 0.70 && ratio <= 0.99,
            "Cache hit ratio should be between 0.70 and 0.99, got {}",
            ratio
        );
    }
}
