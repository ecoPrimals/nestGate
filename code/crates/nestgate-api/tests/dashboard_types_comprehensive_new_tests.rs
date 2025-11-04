//! Comprehensive tests for dashboard_types module
//!
//! Tests cover:
//! - DashboardTimeRange creation, validation, and methods
//! - DashboardState lifecycle and operations
//! - PerformanceAlert structure and behavior
//! - AlertSeverity enum and ordering
//! - DashboardEvent handling
//! - DashboardConfig defaults and customization

use nestgate_api::handlers::dashboard_types::*;
use std::time::{Duration, SystemTime};

// ==================== DASHBOARD TIME RANGE TESTS ====================

#[test]
fn test_dashboard_time_range_new() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let granularity = Duration::from_secs(60);

    let range = DashboardTimeRange::new(start, end, granularity);

    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
    assert_eq!(range.granularity, granularity);
}

#[test]
fn test_dashboard_time_range_last_hours() {
    let range = DashboardTimeRange::last_hours(24);

    assert!(range.is_valid());
    assert_eq!(range.granularity, Duration::from_secs(300)); // 5 minutes

    // Should be approximately 24 hours
    let duration = range.duration();
    assert!(duration.as_secs() >= 24 * 3600 - 5);
    assert!(duration.as_secs() <= 24 * 3600 + 5);
}

#[test]
fn test_dashboard_time_range_last_hours_various() {
    for hours in [1, 6, 12, 24, 48] {
        let range = DashboardTimeRange::last_hours(hours);
        assert!(range.is_valid());

        let duration = range.duration();
        let expected = hours * 3600;
        assert!(duration.as_secs() >= expected - 5);
        assert!(duration.as_secs() <= expected + 5);
    }
}

#[test]
fn test_dashboard_time_range_last_days() {
    let range = DashboardTimeRange::last_days(7);

    assert!(range.is_valid());
    assert_eq!(range.granularity, Duration::from_secs(3600)); // 1 hour

    // Should be approximately 7 days
    let duration = range.duration();
    let expected = 7 * 24 * 3600;
    assert!(duration.as_secs() >= expected - 5);
    assert!(duration.as_secs() <= expected + 5);
}

#[test]
fn test_dashboard_time_range_last_days_various() {
    for days in [1, 3, 7, 14, 30] {
        let range = DashboardTimeRange::last_days(days);
        assert!(range.is_valid());

        let duration = range.duration();
        let expected = days * 24 * 3600;
        assert!(duration.as_secs() >= expected - 5);
        assert!(duration.as_secs() <= expected + 5);
    }
}

#[test]
fn test_dashboard_time_range_duration() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(7200); // 2 hours
    let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

    let duration = range.duration();
    assert_eq!(duration, Duration::from_secs(7200));
}

#[test]
fn test_dashboard_time_range_is_valid_true() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

    assert!(range.is_valid());
}

#[test]
fn test_dashboard_time_range_is_valid_false_reversed() {
    let start = SystemTime::now();
    let end = start - Duration::from_secs(3600); // End before start
    let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

    assert!(!range.is_valid());
}

#[test]
fn test_dashboard_time_range_is_valid_false_zero_granularity() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let range = DashboardTimeRange::new(start, end, Duration::ZERO);

    assert!(!range.is_valid());
}

#[test]
fn test_dashboard_time_range_data_points() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600); // 1 hour
    let range = DashboardTimeRange::new(start, end, Duration::from_secs(60)); // 1 minute

    assert_eq!(range.data_points(), 60); // 60 data points
}

#[test]
fn test_dashboard_time_range_data_points_invalid() {
    let start = SystemTime::now();
    let end = start - Duration::from_secs(3600); // Invalid
    let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

    assert_eq!(range.data_points(), 0);
}

#[test]
fn test_dashboard_time_range_data_points_zero_granularity() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let range = DashboardTimeRange::new(start, end, Duration::ZERO);

    assert_eq!(range.data_points(), 0);
}

#[test]
fn test_dashboard_time_range_intervals() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(300); // 5 minutes
    let range = DashboardTimeRange::new(start, end, Duration::from_secs(60)); // 1 minute

    let intervals = range.intervals();
    assert_eq!(intervals.len(), 5); // 5 intervals

    // Check first interval
    assert_eq!(intervals[0].0, start);
    assert_eq!(intervals[0].1, start + Duration::from_secs(60));
}

#[test]
fn test_dashboard_time_range_intervals_partial() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(250); // 4 minutes 10 seconds
    let range = DashboardTimeRange::new(start, end, Duration::from_secs(60)); // 1 minute

    let intervals = range.intervals();
    assert_eq!(intervals.len(), 5); // 4 full + 1 partial

    // Last interval should be partial
    let last = intervals.last().unwrap();
    assert_eq!(last.1, end);
}

#[test]
fn test_dashboard_time_range_clone() {
    let range = DashboardTimeRange::last_hours(1);
    let cloned = range.clone();

    assert_eq!(range.granularity, cloned.granularity);
}

#[test]
fn test_dashboard_time_range_serialization() {
    let range = DashboardTimeRange::last_hours(1);
    let json = serde_json::to_string(&range).expect("Serialization failed");
    let deserialized: DashboardTimeRange =
        serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(range.granularity, deserialized.granularity);
}

// ==================== DASHBOARD STATE TESTS ====================

#[test]
fn test_dashboard_state_new() {
    let state = DashboardState::new();

    assert_eq!(state.active_connections, 0);
    assert!(state.cached_metrics.is_empty());
    assert!(state.active_alerts.is_empty());
}

#[test]
fn test_dashboard_state_default() {
    let state = DashboardState::default();

    assert_eq!(state.active_connections, 0);
    assert!(state.cached_metrics.is_empty());
}

#[test]
fn test_dashboard_state_update_metrics() {
    let mut state = DashboardState::new();
    let value = serde_json::json!({"cpu": 50.0});

    state.update_metrics("system".to_string(), value.clone());

    assert_eq!(state.cached_metrics.len(), 1);
    assert_eq!(state.cached_metrics.get("system"), Some(&value));
}

#[test]
fn test_dashboard_state_update_multiple_metrics() {
    let mut state = DashboardState::new();

    state.update_metrics("cpu".to_string(), serde_json::json!(50.0));
    state.update_metrics("memory".to_string(), serde_json::json!(75.0));
    state.update_metrics("disk".to_string(), serde_json::json!(60.0));

    assert_eq!(state.cached_metrics.len(), 3);
}

#[test]
fn test_dashboard_state_add_alert() {
    let mut state = DashboardState::new();
    let alert = PerformanceAlert {
        id: "alert-1".to_string(),
        severity: AlertSeverity::Warning,
        title: "Test Alert".to_string(),
        message: "Test message".to_string(),
        timestamp: SystemTime::now(),
        resolved: false,
        metric_name: "cpu".to_string(),
        currentvalue: 85.0,
        threshold: 80.0,
    };

    state.add_alert(alert);

    assert_eq!(state.active_alerts.len(), 1);
}

#[test]
fn test_dashboard_state_add_multiple_alerts() {
    let mut state = DashboardState::new();

    for i in 0..5 {
        let alert = PerformanceAlert {
            id: format!("alert-{}", i),
            severity: AlertSeverity::Info,
            title: "Test".to_string(),
            message: "Test".to_string(),
            timestamp: SystemTime::now(),
            resolved: false,
            metric_name: "test".to_string(),
            currentvalue: 50.0,
            threshold: 40.0,
        };
        state.add_alert(alert);
    }

    assert_eq!(state.active_alerts.len(), 5);
}

#[test]
fn test_dashboard_state_clear_resolved_alerts() {
    let mut state = DashboardState::new();

    // Add resolved alert
    let mut alert1 = PerformanceAlert {
        id: "alert-1".to_string(),
        severity: AlertSeverity::Info,
        title: "Resolved".to_string(),
        message: "Test".to_string(),
        timestamp: SystemTime::now(),
        resolved: true,
        metric_name: "test".to_string(),
        currentvalue: 50.0,
        threshold: 40.0,
    };
    state.add_alert(alert1);

    // Add unresolved alert
    let alert2 = PerformanceAlert {
        id: "alert-2".to_string(),
        severity: AlertSeverity::Warning,
        title: "Active".to_string(),
        message: "Test".to_string(),
        timestamp: SystemTime::now(),
        resolved: false,
        metric_name: "test".to_string(),
        currentvalue: 90.0,
        threshold: 80.0,
    };
    state.add_alert(alert2);

    state.clear_resolved_alerts();

    assert_eq!(state.active_alerts.len(), 1);
    assert_eq!(state.active_alerts[0].id, "alert-2");
}

// ==================== PERFORMANCE ALERT TESTS ====================

#[test]
fn test_performance_alert_creation() {
    let alert = PerformanceAlert {
        id: "alert-123".to_string(),
        severity: AlertSeverity::Critical,
        title: "High CPU".to_string(),
        message: "CPU usage exceeds threshold".to_string(),
        timestamp: SystemTime::now(),
        resolved: false,
        metric_name: "cpu_usage".to_string(),
        currentvalue: 95.0,
        threshold: 80.0,
    };

    assert_eq!(alert.id, "alert-123");
    assert_eq!(alert.severity, AlertSeverity::Critical);
    assert!(!alert.resolved);
}

#[test]
fn test_performance_alert_clone() {
    let alert = PerformanceAlert {
        id: "alert-123".to_string(),
        severity: AlertSeverity::Warning,
        title: "Test".to_string(),
        message: "Test".to_string(),
        timestamp: SystemTime::now(),
        resolved: false,
        metric_name: "test".to_string(),
        currentvalue: 50.0,
        threshold: 40.0,
    };

    let cloned = alert.clone();
    assert_eq!(alert.id, cloned.id);
}

#[test]
fn test_performance_alert_serialization() {
    let alert = PerformanceAlert {
        id: "alert-123".to_string(),
        severity: AlertSeverity::Emergency,
        title: "Critical".to_string(),
        message: "System critical".to_string(),
        timestamp: SystemTime::now(),
        resolved: false,
        metric_name: "system".to_string(),
        currentvalue: 99.0,
        threshold: 95.0,
    };

    let json = serde_json::to_string(&alert).expect("Serialization failed");
    let deserialized: PerformanceAlert =
        serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(alert.id, deserialized.id);
    assert_eq!(alert.severity, deserialized.severity);
}

// ==================== ALERT SEVERITY TESTS ====================

#[test]
fn test_alert_severity_variants() {
    let info = AlertSeverity::Info;
    let warning = AlertSeverity::Warning;
    let critical = AlertSeverity::Critical;
    let emergency = AlertSeverity::Emergency;

    // Verify all variants exist
    assert_ne!(info, warning);
    assert_ne!(warning, critical);
    assert_ne!(critical, emergency);
}

#[test]
fn test_alert_severity_ordering() {
    assert!(AlertSeverity::Info < AlertSeverity::Warning);
    assert!(AlertSeverity::Warning < AlertSeverity::Critical);
    assert!(AlertSeverity::Critical < AlertSeverity::Emergency);
}

#[test]
fn test_alert_severity_equality() {
    assert_eq!(AlertSeverity::Info, AlertSeverity::Info);
    assert_ne!(AlertSeverity::Info, AlertSeverity::Emergency);
}

#[test]
fn test_alert_severity_clone() {
    let severity = AlertSeverity::Critical;
    let cloned = severity.clone();
    assert_eq!(severity, cloned);
}

#[test]
fn test_alert_severity_serialization() {
    for severity in [
        AlertSeverity::Info,
        AlertSeverity::Warning,
        AlertSeverity::Critical,
        AlertSeverity::Emergency,
    ] {
        let json = serde_json::to_string(&severity).expect("Serialization failed");
        let deserialized: AlertSeverity =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(severity, deserialized);
    }
}

// ==================== DASHBOARD EVENT TESTS ====================

#[test]
fn test_dashboard_event_creation() {
    let event = DashboardEvent {
        event_type: "metric_update".to_string(),
        data: serde_json::json!({"cpu": 50.0}),
        timestamp: SystemTime::now(),
    };

    assert_eq!(event.event_type, "metric_update");
}

#[test]
fn test_dashboard_event_clone() {
    let event = DashboardEvent {
        event_type: "test".to_string(),
        data: serde_json::json!({"value": 123}),
        timestamp: SystemTime::now(),
    };

    let cloned = event.clone();
    assert_eq!(event.event_type, cloned.event_type);
}

#[test]
fn test_dashboard_event_serialization() {
    let event = DashboardEvent {
        event_type: "alert".to_string(),
        data: serde_json::json!({"severity": "critical"}),
        timestamp: SystemTime::now(),
    };

    let json = serde_json::to_string(&event).expect("Serialization failed");
    let deserialized: DashboardEvent = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(event.event_type, deserialized.event_type);
}

// ==================== DASHBOARD CONFIG TESTS ====================

#[test]
fn test_dashboard_config_default() {
    let config = DashboardConfig::default();

    assert!(config.enable_real_time);
    assert_eq!(config.update_interval, Duration::from_secs(1));
    assert_eq!(config.max_history_points, 1000);
    assert!(config.enable_predictions);
    assert!(config.alert_thresholds.contains_key("cpu_usage"));
}

#[test]
fn test_dashboard_config_default_thresholds() {
    let config = DashboardConfig::default();

    assert_eq!(config.alert_thresholds.get("cpu_usage"), Some(&80.0));
    assert_eq!(config.alert_thresholds.get("memory_usage"), Some(&85.0));
    assert_eq!(config.alert_thresholds.get("disk_usage"), Some(&90.0));
    assert_eq!(config.alert_thresholds.get("latency_ms"), Some(&1000.0));
    assert_eq!(config.alert_thresholds.get("error_rate"), Some(&5.0));
}

#[test]
fn test_dashboard_config_clone() {
    let config = DashboardConfig::default();
    let cloned = config.clone();

    assert_eq!(config.enable_real_time, cloned.enable_real_time);
    assert_eq!(config.max_history_points, cloned.max_history_points);
}

#[test]
fn test_dashboard_config_custom() {
    let mut config = DashboardConfig::default();
    config.enable_real_time = false;
    config.update_interval = Duration::from_secs(5);
    config.max_history_points = 500;
    config.enable_predictions = false;

    assert!(!config.enable_real_time);
    assert_eq!(config.update_interval, Duration::from_secs(5));
    assert_eq!(config.max_history_points, 500);
    assert!(!config.enable_predictions);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_full_dashboard_workflow() {
    // Create config
    let config = DashboardConfig::default();
    assert!(config.enable_real_time);

    // Create state
    let mut state = DashboardState::new();

    // Add metrics
    state.update_metrics("cpu".to_string(), serde_json::json!(85.0));

    // Create and add alert
    let alert = PerformanceAlert {
        id: "cpu-alert".to_string(),
        severity: AlertSeverity::Warning,
        title: "High CPU".to_string(),
        message: "CPU above threshold".to_string(),
        timestamp: SystemTime::now(),
        resolved: false,
        metric_name: "cpu_usage".to_string(),
        currentvalue: 85.0,
        threshold: *config.alert_thresholds.get("cpu_usage").unwrap(),
    };
    state.add_alert(alert);

    // Verify state
    assert_eq!(state.cached_metrics.len(), 1);
    assert_eq!(state.active_alerts.len(), 1);
}

#[test]
fn test_time_range_with_state() {
    let range = DashboardTimeRange::last_hours(24);
    let mut state = DashboardState::new();

    // Use range to determine data points
    let points = range.data_points();
    assert!(points > 0);

    // Update metrics
    state.update_metrics("range_points".to_string(), serde_json::json!(points));
    assert!(state.cached_metrics.contains_key("range_points"));
}

#[test]
fn test_alert_severity_comparison() {
    let mut severities = vec![
        AlertSeverity::Emergency,
        AlertSeverity::Info,
        AlertSeverity::Critical,
        AlertSeverity::Warning,
    ];

    severities.sort();

    assert_eq!(severities[0], AlertSeverity::Info);
    assert_eq!(severities[1], AlertSeverity::Warning);
    assert_eq!(severities[2], AlertSeverity::Critical);
    assert_eq!(severities[3], AlertSeverity::Emergency);
}
