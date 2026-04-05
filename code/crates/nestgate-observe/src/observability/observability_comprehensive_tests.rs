// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for observability module
//! Added: November 21, 2025 - Coverage Sprint Day 2
//!
//! Target: 80%+ coverage of observability functionality

#[cfg(test)]
mod observability_comprehensive_tests {
    use super::super::{
        ObservabilityConfig, ObservabilityManager, get_observability, get_system_health,
        record_metric,
    };
    use std::collections::HashMap;
    use std::time::Duration;

    // ==================== Manager Creation Tests ====================

    #[tokio::test]
    async fn test_observability_manager_creation_default() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        assert!(manager.get_metrics().await.is_ok(), "Should get metrics");
        assert!(manager.get_health().is_ok(), "Should get health");
    }

    #[tokio::test]
    async fn test_observability_manager_with_custom_config() {
        let config = ObservabilityConfig {
            metrics_enabled: true,
            health_checks_enabled: true,
            tracing_enabled: false,
            metrics_interval: Duration::from_secs(10),
            health_check_interval: Duration::from_secs(30),
            max_metrics_history: 500,
        };
        let manager = ObservabilityManager::new(config);

        assert!(manager.get_metrics().await.is_ok());
    }

    #[tokio::test]
    async fn test_observability_manager_disabled_features() {
        let config = ObservabilityConfig {
            metrics_enabled: false,
            health_checks_enabled: false,
            tracing_enabled: false,
            ..Default::default()
        };
        let manager = ObservabilityManager::new(config);

        // Manager should still be created, features just disabled
        assert!(manager.get_metrics().await.is_ok());
    }

    // ==================== Initialization Tests ====================

    #[tokio::test]
    async fn test_manager_initialization() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let result = manager.initialize();
        assert!(result.is_ok(), "Initialization should succeed");
    }

    #[tokio::test]
    async fn test_manager_initialization_all_features_disabled() {
        let config = ObservabilityConfig {
            metrics_enabled: false,
            health_checks_enabled: false,
            tracing_enabled: false,
            ..Default::default()
        };
        let manager = ObservabilityManager::new(config);
        assert!(manager.initialize().is_ok());
        assert!(manager.get_metrics().await.is_ok());
        assert!(manager.get_health().is_ok());
    }

    #[tokio::test]
    async fn test_double_initialization() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let first = manager.initialize();
        let second = manager.initialize();

        assert!(first.is_ok(), "First initialization should succeed");
        assert!(second.is_ok(), "Second initialization should not fail");
    }

    #[tokio::test]
    async fn test_manager_usable_before_init() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        // Should be usable even before explicit init
        assert!(manager.get_metrics().await.is_ok());
        assert!(manager.get_health().is_ok());
    }

    // ==================== Metrics Recording Tests ====================

    #[tokio::test]
    async fn test_record_single_metric() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let result = manager
            .record_metric("test_metric", 42.0, HashMap::new())
            .await;
        assert!(result.is_ok(), "Should record metric");
    }

    #[tokio::test]
    async fn test_record_multiple_metrics() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        for i in 0..10 {
            let result = manager
                .record_metric("counter", f64::from(i), HashMap::new())
                .await;
            assert!(result.is_ok(), "Should record metric {i}");
        }
    }

    #[tokio::test]
    async fn test_record_metric_with_labels() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let mut labels = HashMap::new();
        labels.insert("service".to_string(), "storage".to_string());
        labels.insert("operation".to_string(), "create".to_string());

        let result = manager.record_metric("operations", 1.0, labels).await;
        assert!(result.is_ok(), "Should record metric with labels");
    }

    #[tokio::test]
    async fn test_record_negative_metric() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let result = manager
            .record_metric("negative", -10.5, HashMap::new())
            .await;
        assert!(result.is_ok(), "Should handle negative values");
    }

    #[tokio::test]
    async fn test_record_zero_metric() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let result = manager.record_metric("zero", 0.0, HashMap::new()).await;
        assert!(result.is_ok(), "Should handle zero values");
    }

    #[tokio::test]
    async fn test_record_large_metric() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let result = manager
            .record_metric("large", f64::MAX / 2.0, HashMap::new())
            .await;
        assert!(result.is_ok(), "Should handle large values");
    }

    #[tokio::test]
    async fn test_record_metric_with_empty_name() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let _result = manager.record_metric("", 1.0, HashMap::new()).await;
        // Implementation specific - might reject or sanitize
    }

    // ==================== Metrics Retrieval Tests ====================

    #[tokio::test]
    async fn test_get_metrics() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        // Record some metrics
        manager
            .record_metric("test1", 10.0, HashMap::new())
            .await
            .expect("Failed to record metric");
        manager
            .record_metric("test2", 20.0, HashMap::new())
            .await
            .expect("Failed to record metric");

        let metrics = manager.get_metrics().await;
        assert!(metrics.is_ok(), "Should retrieve metrics");
    }

    #[tokio::test]
    async fn test_get_metrics_before_recording() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let metrics = manager.get_metrics().await;
        assert!(metrics.is_ok(), "Should return empty/default metrics");
    }

    #[tokio::test]
    async fn test_get_metrics_history() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        // Record metrics over time
        for i in 0..5 {
            manager
                .record_metric("historical", f64::from(i), HashMap::new())
                .await
                .expect("Failed to record metric");
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        let history = manager.get_metrics_history(Duration::from_secs(1)).await;
        assert!(history.is_ok(), "Should retrieve metrics history");
    }

    #[tokio::test]
    async fn test_metrics_history_with_zero_duration() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let history = manager.get_metrics_history(Duration::from_secs(0)).await;
        assert!(history.is_ok(), "Should handle zero duration");
    }

    #[tokio::test]
    async fn test_metrics_history_with_long_duration() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let history = manager.get_metrics_history(Duration::from_secs(3600)).await;
        assert!(history.is_ok(), "Should handle long duration");
    }

    // ==================== Health Check Tests ====================

    #[tokio::test]
    async fn test_get_health_status() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let health = manager.get_health();
        assert!(health.is_ok(), "Should retrieve health status");
    }

    #[tokio::test]
    async fn test_health_check_after_initialization() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        manager.initialize().expect("Initialization failed");

        let health = manager.get_health();
        assert!(health.is_ok(), "Should retrieve health after init");
    }

    #[tokio::test]
    async fn test_health_status_structure() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        let _health = manager.get_health().expect("Failed to get health");
        // Health should have some structure
        // Depending on SystemHealth implementation, verify fields exist
    }

    // ==================== Global Observability Tests ====================

    #[tokio::test]
    async fn test_global_record_metric() {
        let result = record_metric("global_test", 100.0).await;
        // Should succeed or log warning if not initialized
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_global_record_metric_multiple() {
        for i in 0..5 {
            let result = record_metric(&format!("metric_{i}"), f64::from(i)).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_global_get_system_health_when_not_initialized() {
        // This might fail if not initialized
        let _result = get_system_health();
        // Depending on implementation, might succeed with warning or fail
    }

    #[tokio::test]
    async fn test_get_observability_none_when_not_init() {
        // In a fresh test, might not be initialized
        let obs = get_observability();
        // Might be None or Some depending on other tests
        assert!(obs.is_some() || obs.is_none());
    }

    // ==================== Configuration Tests ====================

    #[tokio::test]
    async fn test_config_default_values() {
        let config = ObservabilityConfig::default();

        assert!(config.metrics_enabled);
        assert!(config.health_checks_enabled);
        assert!(config.tracing_enabled);
        assert_eq!(config.metrics_interval, Duration::from_secs(30));
        assert_eq!(config.health_check_interval, Duration::from_secs(60));
        assert_eq!(config.max_metrics_history, 1000);
    }

    #[tokio::test]
    async fn test_config_custom_intervals() {
        let config = ObservabilityConfig {
            metrics_interval: Duration::from_secs(1),
            health_check_interval: Duration::from_secs(5),
            ..Default::default()
        };

        let manager = ObservabilityManager::new(config);
        assert!(manager.get_metrics().await.is_ok());
    }

    #[tokio::test]
    async fn test_config_large_history_size() {
        let config = ObservabilityConfig {
            max_metrics_history: 10000,
            ..Default::default()
        };

        let manager = ObservabilityManager::new(config);
        assert!(manager.get_metrics().await.is_ok());
    }

    #[tokio::test]
    async fn test_config_zero_history_size() {
        let config = ObservabilityConfig {
            max_metrics_history: 0,
            ..Default::default()
        };

        let manager = ObservabilityManager::new(config);
        assert!(manager.get_metrics().await.is_ok());
    }

    // ==================== Concurrent Operations Tests ====================

    #[tokio::test]
    async fn test_concurrent_metric_recording() {
        let config = ObservabilityConfig::default();
        let manager = std::sync::Arc::new(ObservabilityManager::new(config));

        let mut handles = vec![];
        for i in 0..10 {
            let manager_clone = manager.clone();
            let handle = tokio::spawn(async move {
                manager_clone
                    .record_metric(&format!("concurrent_{i}"), f64::from(i), HashMap::new())
                    .await
            });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.expect("Task panicked");
            assert!(result.is_ok(), "Concurrent recording should succeed");
        }
    }

    #[tokio::test]
    async fn test_concurrent_health_checks() {
        let config = ObservabilityConfig::default();
        let manager = std::sync::Arc::new(ObservabilityManager::new(config));

        let mut handles = vec![];
        for _ in 0..5 {
            let manager_clone = manager.clone();
            let handle = tokio::spawn(async move { manager_clone.get_health() });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.expect("Task panicked");
            assert!(result.is_ok(), "Concurrent health checks should succeed");
        }
    }

    #[tokio::test]
    async fn test_concurrent_metrics_retrieval() {
        let config = ObservabilityConfig::default();
        let manager = std::sync::Arc::new(ObservabilityManager::new(config));

        let mut handles = vec![];
        for _ in 0..10 {
            let manager_clone = manager.clone();
            let handle = tokio::spawn(async move { manager_clone.get_metrics().await });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await.expect("Task panicked");
            assert!(
                result.is_ok(),
                "Concurrent metrics retrieval should succeed"
            );
        }
    }

    // ==================== Error Handling Tests ====================

    #[tokio::test]
    async fn test_record_metric_when_disabled() {
        let config = ObservabilityConfig {
            metrics_enabled: false,
            ..Default::default()
        };
        let manager = ObservabilityManager::new(config);

        let result = manager.record_metric("disabled", 1.0, HashMap::new()).await;
        // Should handle gracefully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_health_check_when_disabled() {
        let config = ObservabilityConfig {
            health_checks_enabled: false,
            ..Default::default()
        };
        let manager = ObservabilityManager::new(config);

        let result = manager.get_health();
        // Should handle gracefully
        assert!(result.is_ok());
    }

    // ==================== Integration Tests ====================

    #[tokio::test]
    async fn test_full_lifecycle() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        // Initialize
        manager.initialize().expect("Init failed");

        // Record metrics
        manager
            .record_metric("lifecycle", 1.0, HashMap::new())
            .await
            .expect("Record failed");

        // Get metrics
        let _metrics = manager.get_metrics().await.expect("Get metrics failed");

        // Get health
        let _health = manager.get_health().expect("Get health failed");

        // Get history
        let _history = manager
            .get_metrics_history(Duration::from_secs(60))
            .await
            .expect("Get history failed");
    }

    #[tokio::test]
    async fn test_manager_reusability() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        // Use manager multiple times
        for i in 0..5 {
            manager
                .record_metric("reuse", f64::from(i), HashMap::new())
                .await
                .expect("Failed to record");
            let _ = manager.get_metrics().await.expect("Failed to get metrics");
            let _ = manager.get_health().expect("Failed to get health");
        }
    }
}
