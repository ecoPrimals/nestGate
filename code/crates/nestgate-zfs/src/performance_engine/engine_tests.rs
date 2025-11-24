//! Comprehensive tests for PerformanceOptimizationEngine

use super::engine::PerformanceOptimizationEngine;
use super::types::{AlertSeverity, AlertType, PerformanceAlert};
use crate::{config::ZfsConfig, dataset::ZfsDatasetManager, pool::ZfsPoolManager};
use std::sync::Arc;

// ==================== HELPER FUNCTIONS ====================

/// Create a test engine with default configuration
async fn create_test_engine() -> PerformanceOptimizationEngine {
    let config = ZfsConfig::default();
    let pool_manager = Arc::new(ZfsPoolManager::new(&config).await.unwrap());
    let dataset_manager = Arc::new(ZfsDatasetManager::new(config.clone(), pool_manager.clone()));

    PerformanceOptimizationEngine::new(config, pool_manager, dataset_manager)
}

// ==================== CONSTRUCTOR TESTS ====================

#[tokio::test]
async fn test_engine_new_creates_valid_instance() {
    let engine = create_test_engine().await;
    drop(engine); // Successfully created
}

#[tokio::test]
async fn test_engine_can_be_cloned() {
    let engine = create_test_engine().await;
    let cloned = engine.clone();
    drop(engine);
    drop(cloned);
}

// ==================== TUNING TESTS ====================

#[tokio::test]
async fn test_tune_zfs_parameters_returns_result() {
    let engine = create_test_engine().await;
    let result = engine.tune_zfs_parameters("tank/dataset");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_tune_zfs_parameters_sets_recordsize() {
    let engine = create_test_engine().await;
    let result = engine.tune_zfs_parameters("tank/dataset").unwrap();
    assert!(result.parameter_changes.contains_key("recordsize"));
}

#[tokio::test]
async fn test_tune_zfs_parameters_sets_compression() {
    let engine = create_test_engine().await;
    let result = engine.tune_zfs_parameters("tank/dataset").unwrap();
    assert!(result.parameter_changes.contains_key("compression"));
}

// ==================== ALERT HANDLING TESTS ====================

#[tokio::test]
async fn test_handle_performance_alert() {
    let engine = create_test_engine().await;
    let alert = PerformanceAlert {
        alert_type: AlertType::ThresholdExceeded,
        severity: AlertSeverity::Critical,
        pool_name: "tank".to_string(),
        dataset_name: None,
        description: "High latency detected".to_string(),
        timestamp: std::time::SystemTime::now(),
    };
    let response = engine.handle_performance_alert(alert);
    assert!(response.is_ok());
}

// ==================== ASYNC TESTS ====================

#[tokio::test]
async fn test_get_trending_data() {
    let engine = create_test_engine().await;
    let result = engine.get_trending_data().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_optimize_performance() {
    let engine = create_test_engine().await;
    let result = engine.optimize_performance().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_engine() {
    let mut engine = create_test_engine().await;
    let result = engine.start().await;
    assert!(result.is_ok());
}

// ==================== EDGE CASE TESTS ====================

#[tokio::test]
async fn test_tune_empty_dataset_name() {
    let engine = create_test_engine().await;
    let result = engine.tune_zfs_parameters("");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_tune_long_dataset_name() {
    let engine = create_test_engine().await;
    let long_name = "a".repeat(1000);
    let result = engine.tune_zfs_parameters(&long_name);
    assert!(result.is_ok());
}

// ==================== STRESS TESTS ====================

#[tokio::test]
async fn test_multiple_tuning_operations() {
    let engine = create_test_engine().await;
    for i in 0..20 {
        let dataset = format!("tank/dataset{}", i);
        let result = engine.tune_zfs_parameters(&dataset);
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_multiple_alert_handling() {
    let engine = create_test_engine().await;
    for i in 0..10 {
        let alert = PerformanceAlert {
            alert_type: AlertType::ThresholdExceeded,
            severity: AlertSeverity::Warning,
            pool_name: format!("pool{}", i),
            dataset_name: None,
            description: format!("Alert {}", i),
            timestamp: std::time::SystemTime::now(),
        };
        let response = engine.handle_performance_alert(alert);
        assert!(response.is_ok());
    }
}

// ==================== INTEGRATION TESTS ====================

#[tokio::test]
async fn test_full_optimization_cycle() {
    let mut engine = create_test_engine().await;

    // Start engine
    let start_result = engine.start().await;
    assert!(start_result.is_ok());

    // Get trending data
    let trending_result = engine.get_trending_data().await;
    assert!(trending_result.is_ok());

    // Run optimization
    let opt_result = engine.optimize_performance().await;
    assert!(opt_result.is_ok());
}

#[tokio::test]
async fn test_engine_lifecycle() {
    let engine = create_test_engine().await;

    // Tune parameters
    let tune_result = engine.tune_zfs_parameters("tank/data");
    assert!(tune_result.is_ok());

    // Handle alert
    let alert = PerformanceAlert {
        alert_type: AlertType::ThresholdExceeded,
        severity: AlertSeverity::Warning,
        pool_name: "tank".to_string(),
        dataset_name: None,
        description: "Test alert".to_string(),
        timestamp: std::time::SystemTime::now(),
    };
    let alert_result = engine.handle_performance_alert(alert);
    assert!(alert_result.is_ok());

    // Clone engine
    let cloned = engine.clone();
    drop(cloned);

    // Original engine should still work
    let tune_result2 = engine.tune_zfs_parameters("tank/data2");
    assert!(tune_result2.is_ok());
}
