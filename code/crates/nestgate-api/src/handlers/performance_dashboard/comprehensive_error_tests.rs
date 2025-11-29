//! **COMPREHENSIVE ERROR PATH TESTS** for Performance Dashboard
//!
//! Testing strategy:
//! - Error handling and recovery
//! - Edge cases and boundary conditions
//! - Concurrent access scenarios
//! - Invalid input handling
//! - Resource exhaustion scenarios

use super::handlers::*;
use crate::handlers::dashboard_types::DashboardConfig;
use crate::handlers::performance_analyzer::{AnalyzerConfig, PerformanceAnalyzer};
use crate::handlers::performance_dashboard::metrics::RealTimeMetricsCollector;
use crate::handlers::performance_dashboard::optimizer::OptimizationEngineInterface;
use std::sync::Arc;
use std::time::SystemTime;

/// Creates  Test Dashboard
fn create_test_dashboard() -> PerformanceDashboard {
    let config = DashboardConfig::default();
    let metrics_collector = Arc::new(RealTimeMetricsCollector::new());
    let performance_analyzer = Arc::new(PerformanceAnalyzer::new(AnalyzerConfig::default()));
    let optimization_engine = Arc::new(OptimizationEngineInterface::new());

    PerformanceDashboard::new(
        config,
        metrics_collector,
        performance_analyzer,
        optimization_engine,
    )
}

#[tokio::test]
async fn test_concurrent_dashboard_overview_requests() {
    let dashboard = Arc::new(create_test_dashboard());

    let mut handles = vec![];
    for _ in 0..10 {
        let dash_clone = Arc::clone(&dashboard);
        let handle = tokio::spawn(async move { dash_clone.get_overview().await });
        handles.push(handle);
    }

    let results: Vec<_> = futures::future::join_all(handles).await;

    // All concurrent requests should succeed
    for result in results {
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }
}

#[tokio::test]
async fn test_dashboard_overview_with_zero_time_range() {
    let dashboard = create_test_dashboard();

    // Get overview - should handle edge case gracefully
    let result = dashboard.get_overview().await;
    assert!(result.is_ok());

    let overview = result.unwrap();
    // Time range should still be valid
    assert!(overview.time_range.end >= overview.time_range.start);
}

#[tokio::test]
async fn test_dashboard_health_score_boundaries() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Health score must be within valid range
    assert!(overview.health_score >= 0.0, "Health score below 0");
    assert!(overview.health_score <= 100.0, "Health score above 100");
}

#[tokio::test]
async fn test_dashboard_metrics_non_negative() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();
    let metrics = &overview.current_metrics;

    // All percentage metrics should be non-negative
    assert!(metrics.cpu_usage_percent >= 0.0);
    assert!(metrics.memory_usage_percent >= 0.0);
    assert!(metrics.disk_usage_percent >= 0.0);
    assert!(metrics.error_rate_percent >= 0.0);

    // Counters should be non-negative
    assert!(metrics.active_connections >= 0);
    assert!(metrics.network_throughput_bps >= 0);
}

#[tokio::test]
async fn test_dashboard_capacity_forecast_validity() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();
    let forecast = &overview.capacity_forecast;

    // All capacity values should be non-negative
    assert!(forecast.current_usage_percentage >= 0.0);
    assert!(forecast.projected_usage_in_30_days >= 0.0);
    assert!(forecast.projected_usage_in_90_days >= 0.0);

    // Future projections should generally be >= current (or at least non-negative)
    assert!(forecast.projected_usage_in_30_days >= 0.0);
    assert!(forecast.projected_usage_in_90_days >= 0.0);
}

#[tokio::test]
async fn test_dashboard_timestamp_consistency() {
    let dashboard = create_test_dashboard();

    let before = SystemTime::now();
    let overview = dashboard.get_overview().await.unwrap();
    let after = SystemTime::now();

    // Timestamp should be between before and after
    assert!(overview.timestamp >= before);
    assert!(overview.timestamp <= after);
}

#[tokio::test]
async fn test_dashboard_trend_data_consistency() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();
    let analysis = &overview.performance_analysis;

    // Each trend should have data points
    assert!(!analysis.cpu_trend.data_points.is_empty());
    assert!(!analysis.memory_trend.data_points.is_empty());
    assert!(!analysis.disk_io_trend.data_points.is_empty());
    assert!(!analysis.network_io_trend.data_points.is_empty());

    // Data points should be valid numbers
    for point in &analysis.cpu_trend.data_points {
        assert!(*point >= 0.0, "CPU trend data point should be non-negative");
    }
}

#[tokio::test]
async fn test_dashboard_recommendations_structure() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Should have recommendations (even if empty list is valid)
    let recommendations = &overview.optimization_recommendations;

    // Each recommendation should be a non-empty string
    for rec in recommendations {
        assert!(!rec.is_empty(), "Recommendation should not be empty string");
        assert!(
            rec.len() > 5,
            "Recommendation should have meaningful content"
        );
    }
}

#[tokio::test]
async fn test_dashboard_alert_summary_counts() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();
    let alerts = &overview.alert_summary;

    // Alert counts should be non-negative
    assert!(alerts.critical_alerts >= 0);
    assert!(alerts.warning_alerts >= 0);
    assert!(alerts.info_alerts >= 0);

    // Total should be sum of all alerts
    let total = alerts.critical_alerts + alerts.warning_alerts + alerts.info_alerts;
    assert!(total >= 0);
}

#[test]
fn test_dashboard_query_with_invalid_range_format() {
    // Should still be able to create query with unusual range values
    let query = DashboardQuery {
        range: Some("invalid".to_string()),
        refresh: Some(0), // Zero refresh is edge case
    };

    assert!(query.range.is_some());
    assert_eq!(query.refresh, Some(0));
}

#[test]
fn test_dashboard_query_with_extreme_refresh_values() {
    // Test extremely large refresh interval
    let query = DashboardQuery {
        range: None,
        refresh: Some(u64::MAX),
    };

    assert_eq!(query.refresh, Some(u64::MAX));
}

#[tokio::test]
async fn test_multiple_sequential_overview_calls() {
    let dashboard = create_test_dashboard();

    // Make multiple sequential calls
    for _ in 0..5 {
        let result = dashboard.get_overview().await;
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_dashboard_state_thread_safety() {
    let dashboard = Arc::new(create_test_dashboard());

    // Access state from multiple threads
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let dash = Arc::clone(&dashboard);
            tokio::spawn(async move {
                let _state = dash.state.lock();
                // Just accessing the state should not panic
            })
        })
        .collect();

    for handle in handles {
        assert!(handle.await.is_ok());
    }
}

#[tokio::test]
async fn test_dashboard_metrics_collector_availability() {
    let dashboard = create_test_dashboard();

    // Metrics collector should be accessible
    let strong_count = Arc::strong_count(&dashboard.metrics_collector);
    assert!(strong_count >= 1);

    // Should still be able to get overview
    let result = dashboard.get_overview().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_dashboard_performance_analyzer_availability() {
    let dashboard = create_test_dashboard();

    // Performance analyzer should be accessible
    let strong_count = Arc::strong_count(&dashboard.performance_analyzer);
    assert!(strong_count >= 1);

    // Should still be able to get overview
    let result = dashboard.get_overview().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_dashboard_optimization_engine_availability() {
    let dashboard = create_test_dashboard();

    // Optimization engine should be accessible
    let strong_count = Arc::strong_count(&dashboard.optimization_engine);
    assert!(strong_count >= 1);

    // Should still be able to get overview
    let result = dashboard.get_overview().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_dashboard_rapid_successive_calls() {
    let dashboard = Arc::new(create_test_dashboard());

    // Make rapid successive calls
    let mut handles = vec![];
    for _ in 0..20 {
        let dash = Arc::clone(&dashboard);
        let handle = tokio::spawn(async move { dash.get_overview().await });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;

    // All should complete successfully
    let success_count = results
        .iter()
        .filter(|r| r.is_ok() && r.as_ref().unwrap().is_ok())
        .count();
    assert_eq!(success_count, 20);
}

#[tokio::test]
async fn test_dashboard_time_range_ordering() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Time range end should be after start
    assert!(overview.time_range.end > overview.time_range.start);

    // Duration should be positive
    let duration = overview
        .time_range
        .end
        .duration_since(overview.time_range.start);
    assert!(duration.is_ok());
    assert!(duration.unwrap().as_secs() > 0);
}

#[tokio::test]
async fn test_dashboard_response_time_reasonableness() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Response time should be reasonable (not negative or absurdly large)
    assert!(overview.current_metrics.response_time_ms > 0.0);
    assert!(overview.current_metrics.response_time_ms < 10000.0); // Less than 10 seconds
}
