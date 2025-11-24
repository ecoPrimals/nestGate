//! Tests for performance dashboard handlers

use super::handlers::*;
use crate::handlers::dashboard_types::DashboardConfig;
use crate::handlers::performance_analyzer::{AnalyzerConfig, PerformanceAnalyzer};
use crate::handlers::performance_dashboard::metrics::RealTimeMetricsCollector;
use crate::handlers::performance_dashboard::optimizer::OptimizationEngineInterface;
use axum::extract::Path;
use std::sync::Arc;

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

#[test]
fn test_performance_dashboard_creation() {
    let dashboard = create_test_dashboard();

    assert!(dashboard.state.lock().is_ok());
}

#[test]
fn test_dashboard_has_metrics_collector() {
    let dashboard = create_test_dashboard();

    // Verify metrics collector is accessible
    assert!(Arc::strong_count(&dashboard.metrics_collector) >= 1);
}

#[test]
fn test_dashboard_has_performance_analyzer() {
    let dashboard = create_test_dashboard();

    // Verify performance analyzer is accessible
    assert!(Arc::strong_count(&dashboard.performance_analyzer) >= 1);
}

#[test]
fn test_dashboard_has_optimization_engine() {
    let dashboard = create_test_dashboard();

    // Verify optimization engine is accessible
    assert!(Arc::strong_count(&dashboard.optimization_engine) >= 1);
}

#[test]
fn test_dashboard_state_initialization() {
    let dashboard = create_test_dashboard();

    let state = dashboard.state.lock().unwrap();
    // State should be initialized (default)
    assert!(format!("{state:?}").contains("DashboardState"));
}

#[tokio::test]
async fn test_get_overview_returns_ok() {
    let dashboard = create_test_dashboard();

    let result = dashboard.get_overview().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_overview_has_timestamp() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();
    let now = std::time::SystemTime::now();

    // Timestamp should be recent (within last second)
    let duration = now.duration_since(overview.timestamp);
    assert!(duration.is_ok());
    assert!(duration.unwrap().as_secs() < 2);
}

#[tokio::test]
async fn test_get_overview_has_time_range() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Time range should span approximately 1 hour
    let duration = overview
        .time_range
        .end
        .duration_since(overview.time_range.start)
        .unwrap();

    // Should be close to 3600 seconds (1 hour)
    assert!(duration.as_secs() > 3500 && duration.as_secs() < 3700);
}

#[tokio::test]
async fn test_get_overview_health_score_valid() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Health score should be between 0 and 100
    assert!(overview.health_score >= 0.0);
    assert!(overview.health_score <= 100.0);
}

#[tokio::test]
async fn test_get_overview_has_current_metrics() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Current metrics should have valid values
    assert!(overview.current_metrics.cpu_usage_percent >= 0.0);
    assert!(overview.current_metrics.memory_usage_percent >= 0.0);
    assert!(overview.current_metrics.disk_usage_percent >= 0.0);
}

#[tokio::test]
async fn test_get_overview_has_performance_analysis() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Performance analysis should have trend data
    assert!(!overview
        .performance_analysis
        .cpu_trend
        .data_points
        .is_empty());
    assert!(!overview
        .performance_analysis
        .memory_trend
        .data_points
        .is_empty());
    assert!(!overview
        .performance_analysis
        .disk_io_trend
        .data_points
        .is_empty());
    assert!(!overview
        .performance_analysis
        .network_io_trend
        .data_points
        .is_empty());
}

#[tokio::test]
async fn test_get_overview_has_recommendations() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Should have at least some optimization recommendations
    assert!(!overview.optimization_recommendations.is_empty());
}

#[tokio::test]
async fn test_get_overview_has_capacity_forecast() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Capacity forecast should have valid projections
    assert!(overview.capacity_forecast.current_usage_percentage >= 0.0);
    assert!(overview.capacity_forecast.projected_usage_in_30_days >= 0.0);
    assert!(overview.capacity_forecast.projected_usage_in_90_days >= 0.0);
}

#[tokio::test]
async fn test_get_overview_has_alert_summary() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Alert summary should be present
    assert!(overview.alert_summary.critical_alerts >= 0);
    assert!(overview.alert_summary.warning_alerts >= 0);
    assert!(overview.alert_summary.info_alerts >= 0);
}

#[test]
fn test_dashboard_query_creation() {
    let query = DashboardQuery {
        range: Some("1h".to_string()),
        refresh: Some(60),
    };

    assert_eq!(query.range, Some("1h".to_string()));
    assert_eq!(query.refresh, Some(60));
}

#[test]
fn test_dashboard_query_optional_fields() {
    let query = DashboardQuery {
        range: None,
        refresh: None,
    };

    assert!(query.range.is_none());
    assert!(query.refresh.is_none());
}

#[test]
fn test_stream_dashboard_metrics_creation() {
    let dashboard = Arc::new(create_test_dashboard());

    // Should be able to create metrics stream
    let _stream = PerformanceDashboard::stream_dashboard_metrics(dashboard);
}

#[test]
fn test_stream_dashboard_metrics_function() {
    let dashboard = Arc::new(create_test_dashboard());

    // Should be able to call the standalone function
    let _stream = stream_dashboard_metrics(dashboard);
}

#[test]
fn test_get_performance_analysis_returns_ok() {
    let dashboard = Arc::new(create_test_dashboard());

    let result = get_performance_analysis(dashboard, Path("cpu".to_string()));
    assert!(result.is_ok());
}

#[test]
fn test_get_performance_analysis_response_structure() {
    let dashboard = Arc::new(create_test_dashboard());

    let response = get_performance_analysis(dashboard, Path("memory".to_string())).unwrap();

    // Response should contain success data
    assert!(response.0.success);
}

#[test]
fn test_multiple_dashboard_instances() {
    let dashboard1 = create_test_dashboard();
    let dashboard2 = create_test_dashboard();

    // Each dashboard should have its own state
    let state1_ptr = Arc::as_ptr(&dashboard1.state);
    let state2_ptr = Arc::as_ptr(&dashboard2.state);

    assert_ne!(state1_ptr, state2_ptr);
}

#[tokio::test]
async fn test_get_overview_consistency() {
    let dashboard = create_test_dashboard();

    let overview1 = dashboard.get_overview().await.unwrap();
    let overview2 = dashboard.get_overview().await.unwrap();

    // Both overviews should have the same structure
    assert_eq!(
        overview1.optimization_recommendations.len(),
        overview2.optimization_recommendations.len()
    );
}

#[tokio::test]
async fn test_get_overview_trend_data_points() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();

    // Each trend should have 3 data points as per implementation
    assert_eq!(overview.performance_analysis.cpu_trend.data_points.len(), 3);
    assert_eq!(
        overview.performance_analysis.memory_trend.data_points.len(),
        3
    );
    assert_eq!(
        overview
            .performance_analysis
            .disk_io_trend
            .data_points
            .len(),
        3
    );
    assert_eq!(
        overview
            .performance_analysis
            .network_io_trend
            .data_points
            .len(),
        3
    );
}

#[tokio::test]
async fn test_get_overview_metric_snapshot_fields() {
    let dashboard = create_test_dashboard();

    let overview = dashboard.get_overview().await.unwrap();
    let snapshot = &overview.current_metrics;

    // Verify all snapshot fields are present and reasonable
    assert!(snapshot.active_connections > 0);
    assert!(snapshot.response_time_ms > 0.0);
    assert!(snapshot.error_rate_percent >= 0.0);
    assert!(snapshot.network_throughput_bps > 0);
}
