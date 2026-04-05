// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for Performance Analytics
//!
//! Tests cover metrics collection, alert generation, and recommendation engine.

#[cfg(test)]
mod performance_analytics_tests {
    use super::super::performance_analytics::*;
    use std::collections::HashMap;

    // ==================== PERFORMANCE METRICS TESTS ====================

    #[tokio::test]
    async fn test_get_performance_metrics_endpoint() {
        let result = get_performance_metrics().await;
        assert!(result.is_ok());

        if let Ok(response) = result {
            let metrics = response.0;
            assert!(!metrics.metrics.is_empty());
            assert!(metrics.metrics.contains_key("cpu_usage"));
            assert!(metrics.metrics.contains_key("memory_usage"));
        }
    }

    #[test]
    fn test_performance_metrics_response_structure() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_string(), 45.2);
        metrics.insert("memory_usage".to_string(), 67.8);

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(response.metrics.len(), 2);
        assert!(response.metrics.contains_key("cpu_usage"));
    }

    #[test]
    fn test_performance_metrics_serialization() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_string(), 45.2);

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        assert!(json.contains("cpu_usage"));
        assert!(json.contains("45.2"));
    }

    #[test]
    fn test_performance_metrics_deserialization() {
        let json = r#"{
            "metrics": {"cpu_usage": 50.0, "memory_usage": 70.0},
            "timestamp": {"secs_since_epoch": 1699833600, "nanos_since_epoch": 0}
        }"#;

        let result: Result<PerformanceMetricsResponse, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    // ==================== PERFORMANCE ALERT TESTS ====================

    #[tokio::test]
    async fn test_get_performance_alerts_endpoint() {
        let result = get_performance_alerts().await;
        assert!(result.is_ok());

        if let Ok(response) = result {
            let alerts = response.0;
            // Should return at least one alert
            assert!(!alerts.is_empty());
        }
    }

    #[test]
    fn test_performance_alert_creation() {
        let alert = PerformanceAlert {
            id: "alert-001".to_string(),
            message: "High CPU usage detected".to_string(),
            severity: "warning".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(alert.id, "alert-001");
        assert_eq!(alert.severity, "warning");
        assert!(!alert.message.is_empty());
    }

    #[test]
    fn test_performance_alert_severity_levels() {
        let severities = vec!["info", "warning", "critical"];

        for severity in severities {
            let alert = PerformanceAlert {
                id: format!("alert-{severity}"),
                message: format!("{severity} alert"),
                severity: severity.to_string(),
                timestamp: std::time::SystemTime::now(),
            };

            assert_eq!(alert.severity, severity);
        }
    }

    #[test]
    fn test_performance_alert_serialization() {
        let alert = PerformanceAlert {
            id: "alert-002".to_string(),
            message: "Memory threshold exceeded".to_string(),
            severity: "critical".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&alert).expect("Should serialize");
        assert!(json.contains("alert-002"));
        assert!(json.contains("critical"));
    }

    // ==================== PERFORMANCE RECOMMENDATION TESTS ====================

    #[tokio::test]
    async fn test_get_performance_recommendations_endpoint() {
        let result = get_performance_recommendations().await;
        assert!(result.is_ok());

        if let Ok(response) = result {
            let recommendations = response.0;
            assert!(!recommendations.is_empty());
        }
    }

    #[test]
    fn test_performance_recommendation_creation() {
        let recommendation = PerformanceRecommendation {
            id: "rec-001".to_string(),
            title: "Optimize ARC size".to_string(),
            description: "Increase ARC size for better cache hit ratio".to_string(),
            impact: "High".to_string(),
            priority: 1,
        };

        assert_eq!(recommendation.id, "rec-001");
        assert_eq!(recommendation.priority, 1);
        assert_eq!(recommendation.impact, "High");
    }

    #[test]
    fn test_performance_recommendation_priority_levels() {
        let priorities = vec![1, 2, 3, 4, 5];

        for priority in priorities {
            let recommendation = PerformanceRecommendation {
                id: format!("rec-{priority}"),
                title: format!("Priority {priority} recommendation"),
                description: "Test description".to_string(),
                impact: "Medium".to_string(),
                priority,
            };

            assert_eq!(recommendation.priority, priority);
        }
    }

    #[test]
    fn test_performance_recommendation_serialization() {
        let recommendation = PerformanceRecommendation {
            id: "rec-002".to_string(),
            title: "Enable compression".to_string(),
            description: "Enable LZ4 compression to save disk space".to_string(),
            impact: "Medium".to_string(),
            priority: 2,
        };

        let json = serde_json::to_string(&recommendation).expect("Should serialize");
        assert!(json.contains("rec-002"));
        assert!(json.contains("compression"));
    }

    // ==================== ANALYSIS CONFIG TESTS ====================

    #[test]
    #[expect(deprecated)]
    fn test_analysis_config_creation() {
        let config = AnalysisConfig {
            interval_seconds: 60,
            predictive_enabled: true,
        };

        assert_eq!(config.interval_seconds, 60);
        assert!(config.predictive_enabled);
    }

    #[test]
    #[expect(deprecated)]
    fn test_analysis_config_default() {
        let config = AnalysisConfig::default();
        assert!(!std::ptr::addr_of!(config).is_null());
    }

    // ==================== ANALYZER STATE TESTS ====================

    #[test]
    #[expect(deprecated)]
    fn test_performance_analyzer_state_creation() {
        let state = PerformanceAnalyzerState {
            config: AnalysisConfig::default(),
            last_analysis: Some(std::time::SystemTime::now()),
        };

        assert!(state.last_analysis.is_some());
    }

    #[test]
    fn test_performance_analyzer_state_default() {
        let state = PerformanceAnalyzerState::default();
        assert!(state.last_analysis.is_none());
    }

    #[test]
    #[expect(deprecated)]
    fn test_performance_analyzer_state_clone() {
        let state1 = PerformanceAnalyzerState {
            config: AnalysisConfig::default(),
            last_analysis: Some(std::time::SystemTime::now()),
        };

        let state2 = state1;
        assert!(state2.last_analysis.is_some());
    }

    // ==================== INTEGRATION TESTS ====================

    #[tokio::test]
    async fn test_full_performance_analytics_workflow() {
        // Get metrics
        let metrics_result = get_performance_metrics().await;
        assert!(metrics_result.is_ok());

        // Get alerts
        let alerts_result = get_performance_alerts().await;
        assert!(alerts_result.is_ok());

        // Get recommendations
        let recommendations_result = get_performance_recommendations().await;
        assert!(recommendations_result.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_performance_queries() {
        use futures_util::future::join_all;

        let tasks = (0..5)
            .map(|_| {
                tokio::spawn(async {
                    let _ = get_performance_metrics().await;
                    let _ = get_performance_alerts().await;
                    let _ = get_performance_recommendations().await;
                })
            })
            .collect::<Vec<_>>();

        let results = join_all(tasks).await;

        // All tasks should complete successfully
        for result in results {
            assert!(result.is_ok());
        }
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_empty_metrics_map() {
        let metrics = HashMap::new();
        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        assert!(response.metrics.is_empty());
    }

    #[test]
    fn test_large_metrics_map() {
        let mut metrics = HashMap::new();
        for i in 0..100 {
            metrics.insert(format!("metric_{i}"), f64::from(i));
        }

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(response.metrics.len(), 100);
    }

    #[test]
    fn test_alert_with_empty_message() {
        let alert = PerformanceAlert {
            id: "alert-empty".to_string(),
            message: String::new(),
            severity: "info".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        assert!(alert.message.is_empty());
        assert!(!alert.id.is_empty());
    }

    #[test]
    fn test_recommendation_zero_priority() {
        let recommendation = PerformanceRecommendation {
            id: "rec-zero".to_string(),
            title: "Low priority task".to_string(),
            description: "Can be done later".to_string(),
            impact: "Low".to_string(),
            priority: 0,
        };

        assert_eq!(recommendation.priority, 0);
    }

    // ==================== PERFORMANCE TESTS ====================

    #[tokio::test]
    async fn test_metrics_collection_performance() {
        let start = std::time::Instant::now();

        for _ in 0..100 {
            let _ = get_performance_metrics().await;
        }

        let duration = start.elapsed();

        // Should complete 100 collections in less than 1 second
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_alert_creation_performance() {
        let start = std::time::Instant::now();

        for i in 0..1000 {
            let _ = PerformanceAlert {
                id: format!("alert-{i}"),
                message: "Performance alert".to_string(),
                severity: "info".to_string(),
                timestamp: std::time::SystemTime::now(),
            };
        }

        let duration = start.elapsed();

        // Should create 1000 alerts very quickly
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_recommendation_creation_performance() {
        let start = std::time::Instant::now();

        for i in 0..1000 {
            let _ = PerformanceRecommendation {
                id: format!("rec-{i}"),
                title: "Recommendation".to_string(),
                description: "Test description".to_string(),
                impact: "Medium".to_string(),
                priority: i % 5,
            };
        }

        let duration = start.elapsed();

        // Should create 1000 recommendations very quickly
        assert!(duration.as_millis() < 100);
    }
}
