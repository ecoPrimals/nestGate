// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::{
    MetricsHistoryQuery, calculate_real_zfs_cache_hit_ratio, get_alerts, get_metrics,
    get_metrics_history,
};
use crate::rest::ApiState;
use crate::rest::models::{AlertSeverity, AlertStatus, ComparisonOperator};
use axum::extract::{Query, State};
use dashmap::DashMap;
use nestgate_core::universal_storage::StorageDetector;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

/// Helper to create a test API state
fn create_test_api_state() -> ApiState {
    ApiState {
        zfs_engines: Arc::new(DashMap::new()),
        storage_detector: Arc::new(RwLock::new(StorageDetector::default())),
        auto_configurator: Arc::new(OnceLock::new()),
        rpc_manager: Arc::new(OnceLock::new()),
    }
}

#[tokio::test]
async fn test_get_metrics_returns_data() {
    let state = create_test_api_state();
    let result = get_metrics(State(state)).await;

    assert!(result.is_ok());
    let response = result.expect("Test: get_metrics should return Ok");
    let metrics = &response.0.data;

    // Verify basic metrics are present
    assert!(metrics.cpu_usage_percent >= 0.0);
    assert!(metrics.cpu_usage_percent <= 100.0);
    assert!(metrics.memory_usage_percent >= 0.0);
    assert!(metrics.memory_usage_percent <= 100.0);
    assert!(metrics.uptime_seconds > 0);
}

#[tokio::test]
async fn test_get_metrics_zfs_metrics() {
    let state = create_test_api_state();
    let result = get_metrics(State(state)).await;

    assert!(result.is_ok());
    let metrics = &result.expect("Test: get_metrics should return Ok").0.data;

    // ZFS metrics default to zero when no real ZFS backend is wired
    assert!(metrics.zfs_metrics.arc_hit_ratio >= 0.0);
    assert!(metrics.zfs_metrics.arc_hit_ratio <= 100.0);
    assert!(metrics.zfs_metrics.compression_ratio >= 0.0);
}

#[tokio::test]
async fn test_get_metrics_with_datasets() {
    let state = create_test_api_state();

    // Add test datasets
    state
        .zfs_engines
        .insert("dataset1".to_string(), "data1".to_string());

    let result = get_metrics(State(state)).await;
    assert!(result.is_ok());

    let metrics = &result.expect("Test: get_metrics should return Ok").0.data;
    assert!(metrics.zfs_metrics.total_datasets >= 1);
    // Just verify the metrics structure is returned correctly
}

#[tokio::test]
async fn test_get_metrics_disk_io() {
    let state = create_test_api_state();
    let result = get_metrics(State(state)).await;

    assert!(result.is_ok());
    let metrics = &result.expect("Test: get_metrics should return Ok").0.data;

    // Verify disk I/O metrics
    assert!(metrics.disk_io.read_bytes_per_sec >= 0.0);
    assert!(metrics.disk_io.write_bytes_per_sec >= 0.0);
    assert!(metrics.disk_io.read_mbps >= 0.0);
    assert!(metrics.disk_io.write_mbps >= 0.0);
    assert!(metrics.disk_io.avg_queue_depth >= 0.0);
}

#[tokio::test]
async fn test_get_metrics_network_io() {
    let state = create_test_api_state();
    let result = get_metrics(State(state)).await;

    assert!(result.is_ok());
    let metrics = &result.expect("Test: get_metrics should return Ok").0.data;

    // Verify network I/O struct is populated (values depend on system traffic)
    let _ = metrics.network_io.bytes_sent;
    let _ = metrics.network_io.bytes_received;
}

#[tokio::test]
async fn test_get_metrics_history_default_params() {
    let state = create_test_api_state();
    let query = MetricsHistoryQuery {
        start: None,
        end: None,
        interval: None,
        metrics: None,
    };

    let result = get_metrics_history(State(state), Query(query)).await;
    assert!(result.is_ok());

    let history = &result
        .expect("Test: get_metrics_history should return Ok")
        .0
        .data;
    // TSDB not yet wired — history returns empty
    assert!(
        history.is_empty(),
        "Without TSDB, metrics history should be empty"
    );
}

#[tokio::test]
async fn test_get_metrics_history_with_interval() {
    let state = create_test_api_state();

    for interval in &["1m", "5m", "15m", "1h", "1d"] {
        let query = MetricsHistoryQuery {
            start: None,
            end: None,
            interval: Some((*interval).to_string()),
            metrics: None,
        };

        let result = get_metrics_history(State(state.clone()), Query(query)).await;
        assert!(result.is_ok());

        let history = &result
            .expect("Test: get_metrics_history should return Ok")
            .0
            .data;
        // Without TSDB, all intervals return empty
        assert!(history.is_empty());
    }
}

#[tokio::test]
async fn test_get_metrics_history_with_time_range() {
    let state = create_test_api_state();

    let end = chrono::Utc::now();
    let start = end - chrono::Duration::hours(1);

    let query = MetricsHistoryQuery {
        start: Some(start.to_rfc3339()),
        end: Some(end.to_rfc3339()),
        interval: Some("5m".to_string()),
        metrics: None,
    };

    let result = get_metrics_history(State(state), Query(query)).await;
    assert!(result.is_ok());

    let history = &result
        .expect("Test: get_metrics_history should return Ok")
        .0
        .data;
    // Without TSDB, returns empty
    assert!(history.is_empty());
}

#[tokio::test]
async fn test_get_alerts_returns_data() {
    let state = create_test_api_state();
    let result = get_alerts(State(state)).await;

    assert!(result.is_ok());
    let _alerts = &result.expect("Test: get_alerts should return Ok").0.data;

    // Verify alert structure is valid (alerts may be empty if no alerts active)
    // Note: Length is always >= 0 by definition, but we're verifying the structure exists
}

#[tokio::test]
async fn test_get_alerts_with_many_datasets() {
    let state = create_test_api_state();

    // Add more than 10 datasets to trigger alert
    for i in 0..15 {
        state
            .zfs_engines
            .insert(format!("dataset{i}"), format!("data{i}"));
    }

    let result = get_alerts(State(state)).await;
    assert!(result.is_ok());

    let alerts = &result.expect("Test: get_alerts should return Ok").0.data;
    assert!(!alerts.is_empty(), "Should have high dataset count alert");

    // Find the high dataset count alert
    let dataset_alert = alerts.iter().find(|a| a.name == "High Dataset Count");
    assert!(dataset_alert.is_some());

    let alert = dataset_alert.expect("Test: dataset_alert should be Some");
    assert_eq!(alert.severity, AlertSeverity::Warning);
    assert_eq!(alert.status, AlertStatus::Active);
    assert!(!alert.suggested_actions.is_empty());
}

#[tokio::test]
async fn test_get_alerts_structure() {
    let state = create_test_api_state();

    // Add datasets to generate some alerts
    for i in 0..15 {
        state
            .zfs_engines
            .insert(format!("dataset{i}"), format!("data{i}"));
    }

    let result = get_alerts(State(state)).await;
    assert!(result.is_ok());

    let alerts = &result.expect("Test: get_alerts should return Ok").0.data;

    for alert in alerts {
        // Verify alert has all required fields
        assert!(!alert.id.is_empty());
        assert!(!alert.name.is_empty());
        assert!(!alert.message.is_empty());
        assert!(!alert.conditions.is_empty());

        // Verify timestamp logic
        assert!(alert.triggered_at >= alert.created_at);

        // Verify condition structure
        for condition in &alert.conditions {
            assert!(!condition.metric_name.is_empty());
            assert!(condition.duration_seconds > 0);
        }
    }
}

#[tokio::test]
async fn test_get_alerts_severity_levels() {
    let state = create_test_api_state();

    // Add datasets to generate alerts
    for i in 0..15 {
        state
            .zfs_engines
            .insert(format!("dataset{i}"), format!("data{i}"));
    }

    let result = get_alerts(State(state)).await;
    assert!(result.is_ok());

    let alerts = &result.expect("Test: get_alerts should return Ok").0.data;

    // Verify we have various severity levels
    let has_warning = alerts
        .iter()
        .any(|a| matches!(a.severity, AlertSeverity::Warning));
    assert!(has_warning, "Should have at least one warning alert");
}

#[tokio::test]
async fn test_calculate_real_zfs_cache_hit_ratio() {
    let result = calculate_real_zfs_cache_hit_ratio().await;

    // Should always return a valid ratio (default or actual)
    assert!(result.is_ok());
    let ratio = result.expect("Test: calculate_arc_hit_ratio should return Ok");
    assert!(ratio >= 0.0);
    assert!(ratio <= 100.0);
}

#[tokio::test]
async fn test_metrics_history_query_deserialization() {
    // Test that MetricsHistoryQuery can be deserialized
    let json = r#"{"start":"2025-01-01T00:00:00Z","end":"2025-01-01T01:00:00Z","interval":"5m"}"#;
    let query: std::result::Result<MetricsHistoryQuery, _> = serde_json::from_str(json);
    assert!(query.is_ok());

    let q = query.expect("Test: query deserialization should succeed");
    assert!(q.start.is_some());
    assert!(q.end.is_some());
    assert_eq!(q.interval, Some("5m".to_string()));
}

#[test]
fn test_alert_severity_variants() {
    // Verify AlertSeverity enum variants
    let _warning = AlertSeverity::Warning;
    let _critical = AlertSeverity::Critical;
    // Just checking these compile
}

#[test]
fn test_alert_status_variants() {
    // Verify AlertStatus enum variants
    let _active = AlertStatus::Active;
    let _resolved = AlertStatus::Resolved;
    // Just checking these compile
}

#[test]
fn test_comparison_operator_variants() {
    // Verify ComparisonOperator enum variants
    let _gt = ComparisonOperator::GreaterThan;
    // Just checking these compile
}
