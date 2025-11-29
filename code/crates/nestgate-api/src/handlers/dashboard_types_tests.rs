//! Unit tests for dashboard types
//!
//! These tests cover dashboard data structures and time range logic.

#[cfg(test)]
mod tests {
    use super::super::dashboard_types::*;
    use std::time::{Duration, SystemTime};

    // ==================== DashboardTimeRange Tests ====================

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

        assert!(range.is_valid());
        assert_eq!(range.granularity, Duration::from_secs(300)); // 5 minutes
        assert_eq!(range.duration().as_secs(), 2 * 3600);
    }

    #[test]
    fn test_time_range_last_days() {
        let range = DashboardTimeRange::last_days(7);

        assert!(range.is_valid());
        assert_eq!(range.granularity, Duration::from_secs(3600)); // 1 hour
        assert_eq!(range.duration().as_secs(), 7 * 24 * 3600);
    }

    #[test]
    fn test_time_range_duration() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(3600);
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

        assert_eq!(range.duration(), Duration::from_secs(3600));
    }

    #[test]
    fn test_time_range_is_valid() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(3600);

        // Valid range
        let valid_range = DashboardTimeRange::new(start, end, Duration::from_secs(60));
        assert!(valid_range.is_valid());

        // Invalid range (end before start)
        let invalid_range = DashboardTimeRange::new(end, start, Duration::from_secs(60));
        assert!(!invalid_range.is_valid());

        // Invalid range (zero granularity)
        let zero_granularity = DashboardTimeRange::new(start, end, Duration::ZERO);
        assert!(!zero_granularity.is_valid());
    }

    #[test]
    fn test_time_range_data_points() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(3600);
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

        assert_eq!(range.data_points(), 60);
    }

    #[test]
    fn test_time_range_data_points_invalid() {
        let start = SystemTime::now();
        let end = start;
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(60));

        assert_eq!(range.data_points(), 0);
    }

    #[test]
    fn test_time_range_intervals() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(300);
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(100));

        let intervals = range.intervals();
        assert_eq!(intervals.len(), 3);
    }

    #[test]
    fn test_time_range_intervals_exact_fit() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(600);
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(100));

        let intervals = range.intervals();
        assert_eq!(intervals.len(), 6);
    }

    #[test]
    fn test_time_range_clone() {
        let range1 = DashboardTimeRange::last_hours(1);
        let range2 = range1.clone();

        assert_eq!(range1.granularity, range2.granularity);
    }

    #[test]
    fn test_time_range_serialization() {
        let range = DashboardTimeRange::last_hours(1);
        let json = serde_json::to_string(&range).expect("Failed to serialize");

        assert!(json.contains("start"));
        assert!(json.contains("end"));
        assert!(json.contains("granularity"));
    }

    // ==================== DashboardState Tests ====================

    #[test]
    fn test_dashboard_state_creation() {
        let state = DashboardState::new();

        assert_eq!(state.active_connections, 0);
        assert_eq!(state.cached_metrics.len(), 0);
        assert_eq!(state.active_alerts.len(), 0);
    }

    #[test]
    fn test_dashboard_state_default() {
        let state = DashboardState::default();

        assert_eq!(state.active_connections, 0);
        assert_eq!(state.cached_metrics.len(), 0);
    }

    #[test]
    fn test_dashboard_state_update_metrics() {
        let mut state = DashboardState::new();
        let initial_update = state.last_update;

        std::thread::sleep(Duration::from_millis(10));

        state.update_metrics("cpu_usage".to_string(), serde_json::json!(45.5));

        assert_eq!(state.cached_metrics.len(), 1);
        assert!(state.last_update > initial_update);
    }

    #[test]
    fn test_dashboard_state_multiple_metrics() {
        let mut state = DashboardState::new();

        state.update_metrics("cpu_usage".to_string(), serde_json::json!(45.5));
        state.update_metrics("memory_usage".to_string(), serde_json::json!(65.0));
        state.update_metrics("disk_io".to_string(), serde_json::json!(1000));

        assert_eq!(state.cached_metrics.len(), 3);
        assert_eq!(
            state.cached_metrics.get("cpu_usage").unwrap(),
            &serde_json::json!(45.5)
        );
    }

    #[test]
    fn test_dashboard_state_add_alert() {
        let mut state = DashboardState::new();
        let alert = create_test_alert("alert1", false);

        state.add_alert(alert);

        assert_eq!(state.active_alerts.len(), 1);
    }

    #[test]
    fn test_dashboard_state_clear_resolved_alerts() {
        let mut state = DashboardState::new();

        state.add_alert(create_test_alert("alert1", false));
        state.add_alert(create_test_alert("alert2", true));
        state.add_alert(create_test_alert("alert3", false));

        assert_eq!(state.active_alerts.len(), 3);

        state.clear_resolved_alerts();

        assert_eq!(state.active_alerts.len(), 2);
    }

    #[test]
    fn test_dashboard_state_debug_format() {
        let state = DashboardState::new();
        let debug_str = format!("{state:?}");

        assert!(debug_str.contains("DashboardState"));
        assert!(debug_str.contains("active_connections"));
    }

    // ==================== PerformanceAlert Tests ====================

    #[test]
    fn test_performance_alert_creation() {
        let alert = create_test_alert("test_alert", false);

        assert_eq!(alert.id, "test_alert");
        assert!(!alert.resolved);
    }

    #[test]
    fn test_performance_alert_clone() {
        let alert1 = create_test_alert("alert1", false);
        let alert2 = alert1.clone();

        assert_eq!(alert1.id, alert2.id);
        assert_eq!(alert1.resolved, alert2.resolved);
    }

    #[test]
    fn test_performance_alert_serialization() {
        let alert = create_test_alert("test_alert", false);
        let json = serde_json::to_string(&alert).expect("Failed to serialize");

        assert!(json.contains("test_alert"));
        assert!(json.contains("resolved"));
    }

    #[test]
    fn test_performance_alert_deserialization() {
        let alert = create_test_alert("test_alert", true);
        let json = serde_json::to_string(&alert).unwrap();

        let deserialized: PerformanceAlert =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(alert.id, deserialized.id);
        assert_eq!(alert.resolved, deserialized.resolved);
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_dashboard_workflow() {
        let mut state = DashboardState::new();

        // Add metrics
        state.update_metrics("cpu".to_string(), serde_json::json!(50.0));
        state.update_metrics("memory".to_string(), serde_json::json!(70.0));

        // Add alerts
        state.add_alert(create_test_alert("cpu_high", false));
        state.add_alert(create_test_alert("memory_high", true));

        assert_eq!(state.cached_metrics.len(), 2);
        assert_eq!(state.active_alerts.len(), 2);

        // Clear resolved alerts
        state.clear_resolved_alerts();

        assert_eq!(state.active_alerts.len(), 1);
    }

    #[test]
    fn test_time_range_workflow() {
        // Create hourly range
        let hourly_range = DashboardTimeRange::last_hours(1);
        assert!(hourly_range.is_valid());
        assert!(hourly_range.data_points() > 0);

        // Create daily range
        let daily_range = DashboardTimeRange::last_days(1);
        assert!(daily_range.is_valid());
        assert!(daily_range.data_points() > 0);

        // Daily range should have more data points
        assert!(daily_range.duration() > hourly_range.duration());
    }

    // ==================== Edge Cases ====================

    #[test]
    fn test_time_range_zero_duration() {
        let now = SystemTime::now();
        let range = DashboardTimeRange::new(now, now, Duration::from_secs(60));

        assert!(!range.is_valid());
        assert_eq!(range.data_points(), 0);
    }

    #[test]
    fn test_time_range_reversed() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(3600);
        let range = DashboardTimeRange::new(end, start, Duration::from_secs(60));

        assert!(!range.is_valid());
    }

    #[test]
    fn test_dashboard_state_empty_metrics() {
        let state = DashboardState::new();
        assert!(state.cached_metrics.is_empty());
    }

    #[test]
    fn test_dashboard_state_overwrite_metric() {
        let mut state = DashboardState::new();

        state.update_metrics("cpu".to_string(), serde_json::json!(50.0));
        state.update_metrics("cpu".to_string(), serde_json::json!(60.0));

        assert_eq!(state.cached_metrics.len(), 1);
        assert_eq!(
            state.cached_metrics.get("cpu").unwrap(),
            &serde_json::json!(60.0)
        );
    }

    #[test]
    fn test_dashboard_state_all_alerts_resolved() {
        let mut state = DashboardState::new();

        state.add_alert(create_test_alert("alert1", true));
        state.add_alert(create_test_alert("alert2", true));

        state.clear_resolved_alerts();

        assert_eq!(state.active_alerts.len(), 0);
    }

    #[test]
    fn test_time_range_large_granularity() {
        let start = SystemTime::now();
        let end = start + Duration::from_secs(3600);
        let range = DashboardTimeRange::new(start, end, Duration::from_secs(7200));

        assert!(range.is_valid());
        assert_eq!(range.data_points(), 1);
    }

    #[test]
    fn test_time_range_intervals_empty() {
        let now = SystemTime::now();
        let range = DashboardTimeRange::new(now, now, Duration::from_secs(60));

        let intervals = range.intervals();
        assert_eq!(intervals.len(), 0);
    }

    // ==================== Helper Functions ====================

    /// Creates  Test Alert
    fn create_test_alert(id: &str, resolved: bool) -> PerformanceAlert {
        PerformanceAlert {
            id: id.to_string(),
            severity: AlertSeverity::Warning,
            title: format!("Test Alert: {id}"),
            message: format!("Test alert: {id}"),
            timestamp: SystemTime::now(),
            resolved,
            metric_name: "test_metric".to_string(),
            currentvalue: 85.0,
            threshold: 80.0,
        }
    }
}
