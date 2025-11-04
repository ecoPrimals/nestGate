//! **COMPREHENSIVE DASHBOARD TYPES TESTS**
//!
//! Test coverage for dashboard_types.rs to increase overall coverage.
//! These tests cover time ranges, metrics, and dashboard data structures.

#[cfg(test)]
mod tests {
    use crate::handlers::dashboard_types::*;
    use std::time::{Duration, SystemTime};

    // ==================== DASHBOARD TIME RANGE TESTS ====================

    #[test]
    fn test_time_range_creation() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(3600);
        let granularity = Duration::from_secs(60);

        let range = DashboardTimeRange::new(start, end, granularity);

        assert_eq!(range.start, start);
        assert_eq!(range.end, end);
        assert_eq!(range.granularity, granularity);
    }

    #[test]
    fn test_time_range_last_hours() {
        let range = DashboardTimeRange::last_hours(2);

        // Should create a 2-hour range
        let duration = range.duration();
        assert!(
            duration.as_secs() >= 7140 && duration.as_secs() <= 7260,
            "Duration should be approximately 2 hours (7200 seconds), got {}",
            duration.as_secs()
        );

        // Granularity should be 5 minutes (300 seconds)
        assert_eq!(
            range.granularity.as_secs(),
            300,
            "Granularity should be 5 minutes"
        );
    }

    #[test]
    fn test_time_range_last_days() {
        let range = DashboardTimeRange::last_days(1);

        // Should create a 1-day range
        let duration = range.duration();
        assert!(
            duration.as_secs() >= 86340 && duration.as_secs() <= 86460,
            "Duration should be approximately 1 day (86400 seconds)"
        );

        // Granularity should be 1 hour (3600 seconds)
        assert_eq!(
            range.granularity.as_secs(),
            3600,
            "Granularity should be 1 hour"
        );
    }

    #[test]
    fn test_time_range_duration() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(7200); // 2 hours
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

        let duration = range.duration();
        assert_eq!(duration.as_secs(), 7200, "Duration should be 2 hours");
    }

    #[test]
    fn test_time_range_is_valid() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(3600);

        // Valid range
        let valid_range = DashboardTimeRange::new(start, end, Duration::from_secs(60));
        assert!(valid_range.is_valid(), "Valid range should return true");

        // Invalid range (end before start)
        let invalid_range = DashboardTimeRange::new(end, start, Duration::from_secs(60));
        assert!(
            !invalid_range.is_valid(),
            "Invalid range should return false"
        );

        // Invalid range (zero granularity)
        let invalid_granularity = DashboardTimeRange::new(start, end, Duration::ZERO);
        assert!(
            !invalid_granularity.is_valid(),
            "Zero granularity should be invalid"
        );
    }

    #[test]
    fn test_time_range_data_points() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(3600); // 1 hour
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(60)); // 1 minute granularity

        let points = range.data_points();
        assert_eq!(
            points, 60,
            "Should have 60 data points for 1 hour with 1-minute granularity"
        );
    }

    #[test]
    fn test_time_range_data_points_invalid() {
        let start = SystemTime::now();
        let end = start; // Same start and end
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

        let points = range.data_points();
        assert_eq!(points, 0, "Invalid range should have 0 data points");
    }

    #[test]
    fn test_time_range_intervals() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(600); // 10 minutes
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(120)); // 2-minute intervals

        let intervals = range.intervals();

        // Should have 5 intervals (600 / 120 = 5)
        assert_eq!(intervals.len(), 5, "Should split into 5 intervals");

        // Each interval should be a tuple of (start, end)
        for (i, &(interval_start, _interval_end)) in intervals.iter().enumerate() {
            let expected = start + Duration::from_secs(i as u64 * 120);
            // Allow small timing differences
            let diff = interval_start
                .duration_since(expected)
                .unwrap_or_else(|_| expected.duration_since(interval_start).expect("Test setup failed"));
            assert!(
                diff < Duration::from_millis(10),
                "Interval {} should be at expected time",
                i
            );
        }
    }

    #[test]
    fn test_time_range_serialization() {
        let start = SystemTime::UNIX_EPOCH + Duration::from_secs(1000000);
        let end = start + Duration::from_secs(3600);
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

        let serialized = serde_json::to_string(&range);
        assert!(serialized.is_ok(), "DashboardTimeRange should serialize");
    }

    #[test]
    fn test_time_range_clone() {
        let range1 = DashboardTimeRange::last_hours(1);
        let range2 = range1.clone();

        assert_eq!(
            range1.granularity, range2.granularity,
            "Cloned range should have same granularity"
        );
        assert!(
            range1.is_valid() == range2.is_valid(),
            "Cloned range should have same validity"
        );
    }

    // ==================== DASHBOARD STATE TESTS ====================

    #[test]
    fn test_dashboard_state_creation() {
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
        let old_update = state.last_update;

        // Wait a tiny bit
        std::thread::sleep(Duration::from_millis(10));

        state.update_metrics("cpu".to_string(), serde_json::json!(75.5));

        assert_eq!(state.cached_metrics.len(), 1);
        assert!(
            state.last_update > old_update,
            "Last update should be more recent"
        );
    }

    #[test]
    fn test_dashboard_state_add_alert() {
        let mut state = DashboardState::new();

        let alert = PerformanceAlert {
            id: "alert1".to_string(),
            severity: AlertSeverity::Warning,
            title: "High CPU".to_string(),
            message: "CPU usage is high".to_string(),
            timestamp: SystemTime::now(),
            resolved: false,
            metric_name: "cpu_usage".to_string(),
            currentvalue: 85.0,
            threshold: 80.0,
        };

        state.add_alert(alert);

        assert_eq!(state.active_alerts.len(), 1);
    }

    #[test]
    fn test_dashboard_state_clear_resolved_alerts() {
        let mut state = DashboardState::new();

        // Add unresolved alert
        state.add_alert(PerformanceAlert {
            id: "alert1".to_string(),
            severity: AlertSeverity::Warning,
            title: "Alert 1".to_string(),
            message: "Message 1".to_string(),
            timestamp: SystemTime::now(),
            resolved: false,
            metric_name: "metric1".to_string(),
            currentvalue: 90.0,
            threshold: 80.0,
        });

        // Add resolved alert
        state.add_alert(PerformanceAlert {
            id: "alert2".to_string(),
            severity: AlertSeverity::Info,
            title: "Alert 2".to_string(),
            message: "Message 2".to_string(),
            timestamp: SystemTime::now(),
            resolved: true,
            metric_name: "metric2".to_string(),
            currentvalue: 70.0,
            threshold: 80.0,
        });

        assert_eq!(state.active_alerts.len(), 2);

        state.clear_resolved_alerts();

        assert_eq!(state.active_alerts.len(), 1);
        assert!(!state.active_alerts[0].resolved);
    }

    // ==================== PERFORMANCE ALERT TESTS ====================

    #[test]
    fn test_performance_alert_creation() {
        let alert = PerformanceAlert {
            id: "alert1".to_string(),
            severity: AlertSeverity::Critical,
            title: "High Memory Usage".to_string(),
            message: "Memory usage exceeded threshold".to_string(),
            timestamp: SystemTime::now(),
            resolved: false,
            metric_name: "memory_usage".to_string(),
            currentvalue: 95.0,
            threshold: 90.0,
        };

        assert_eq!(alert.id, "alert1");
        assert_eq!(alert.severity, AlertSeverity::Critical);
        assert!(!alert.resolved);
    }

    #[test]
    fn test_alert_severity_ordering() {
        assert!(AlertSeverity::Info < AlertSeverity::Warning);
        assert!(AlertSeverity::Warning < AlertSeverity::Critical);
        assert!(AlertSeverity::Critical < AlertSeverity::Emergency);
    }

    #[test]
    fn test_performance_alert_serialization() {
        let alert = PerformanceAlert {
            id: "alert1".to_string(),
            severity: AlertSeverity::Warning,
            title: "Test Alert".to_string(),
            message: "Test message".to_string(),
            timestamp: SystemTime::now(),
            resolved: false,
            metric_name: "test_metric".to_string(),
            currentvalue: 85.0,
            threshold: 80.0,
        };

        let serialized = serde_json::to_string(&alert);
        assert!(serialized.is_ok(), "PerformanceAlert should serialize");
    }

    // ==================== DASHBOARD CONFIG TESTS ====================

    #[test]
    fn test_dashboard_config_default() {
        let config = DashboardConfig::default();

        assert!(config.enable_real_time);
        assert!(config.enable_predictions);
        assert_eq!(config.update_interval.as_secs(), 1);
        assert_eq!(config.max_history_points, 1000);
        assert!(!config.alert_thresholds.is_empty());
    }

    #[test]
    fn test_dashboard_config_alert_thresholds() {
        let config = DashboardConfig::default();

        assert!(config.alert_thresholds.contains_key("cpu_usage"));
        assert!(config.alert_thresholds.contains_key("memory_usage"));
        assert!(config.alert_thresholds.contains_key("disk_usage"));

        let cpu_threshold = config.alert_thresholds.get("cpu_usage");
        assert_eq!(cpu_threshold, Some(&80.0));
    }

    // ==================== DASHBOARD EVENT TESTS ====================

    #[test]
    fn test_dashboard_event_creation() {
        let event = DashboardEvent {
            event_type: "metric_updated".to_string(),
            data: serde_json::json!({"metric": "cpu", "value": 75}),
            timestamp: SystemTime::now(),
        };

        assert_eq!(event.event_type, "metric_updated");
    }

    #[test]
    fn test_dashboard_event_serialization() {
        let event = DashboardEvent {
            event_type: "alert_triggered".to_string(),
            data: serde_json::json!({"alert_id": "alert1"}),
            timestamp: SystemTime::now(),
        };

        let serialized = serde_json::to_string(&event);
        assert!(serialized.is_ok(), "DashboardEvent should serialize");
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_dashboard_time_range_with_state() {
        // Create a time range
        let range = DashboardTimeRange::last_hours(1);
        assert!(range.is_valid(), "Range should be valid");

        // Get intervals
        let intervals = range.intervals();
        assert!(!intervals.is_empty(), "Should have intervals");

        // Could create dashboard state with metrics for each interval
        let mut state = DashboardState::new();
        state.update_metrics(
            "interval_count".to_string(),
            serde_json::json!(intervals.len()),
        );

        assert_eq!(state.cached_metrics.len(), 1);
    }

    #[test]
    fn test_multiple_time_ranges() {
        let range_1h = DashboardTimeRange::last_hours(1);
        let range_24h = DashboardTimeRange::last_hours(24);
        let range_7d = DashboardTimeRange::last_days(7);

        // All should be valid
        assert!(range_1h.is_valid());
        assert!(range_24h.is_valid());
        assert!(range_7d.is_valid());

        // Longer ranges should have more data points
        assert!(range_24h.duration() > range_1h.duration());
        assert!(range_7d.duration() > range_24h.duration());
    }

    #[test]
    fn test_edge_case_very_short_range() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(10); // 10 seconds
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(1)); // 1 second granularity

        assert!(range.is_valid(), "Very short range should be valid");
        let points = range.data_points();
        assert_eq!(points, 10, "Should have 10 data points");
    }

    #[test]
    fn test_edge_case_very_long_range() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(365 * 24 * 3600); // 1 year
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(3600));

        assert!(range.is_valid(), "Very long range should be valid");
        let duration = range.duration();
        assert!(
            duration.as_secs() >= 365 * 24 * 3600 - 60,
            "Duration should be approximately 1 year"
        );
    }
}
