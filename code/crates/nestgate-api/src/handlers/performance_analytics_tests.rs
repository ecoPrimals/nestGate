// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Integration tests for performance analytics handlers.

use super::performance_analytics::*;

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== STATE TESTS ====================

    #[test]
    fn test_performance_analyzer_state_creation() {
        let state = PerformanceAnalyzerState::default();

        assert_eq!(state.config.interval_seconds, 0);
        assert!(!state.config.predictive_enabled);
        assert!(state.last_analysis.is_none());
    }

    #[test]
    fn test_performance_analyzer_state_with_config() {
        let config = AnalysisConfig {
            interval_seconds: 60,
            predictive_enabled: true,
        };

        let state = PerformanceAnalyzerState {
            config,
            last_analysis: Some(std::time::SystemTime::now()),
        };

        assert_eq!(state.config.interval_seconds, 60);
        assert!(state.config.predictive_enabled);
        assert!(state.last_analysis.is_some());
    }

    #[test]
    fn test_performance_analyzer_state_clone() {
        let state1 = PerformanceAnalyzerState::default();
        let state2 = state1.clone();

        assert_eq!(
            state1.config.interval_seconds,
            state2.config.interval_seconds
        );
        assert_eq!(
            state1.config.predictive_enabled,
            state2.config.predictive_enabled
        );
    }

    #[test]
    fn test_performance_analyzer_state_debug() {
        let state = PerformanceAnalyzerState::default();
        let debug_str = format!("{:?}", state);

        assert!(debug_str.contains("PerformanceAnalyzerState"));
    }

    // ==================== CONFIG TESTS ====================

    #[test]
    fn test_analysis_config_default() {
        let config = AnalysisConfig::default();

        assert_eq!(config.interval_seconds, 0);
        assert!(!config.predictive_enabled);
    }

    #[test]
    fn test_analysis_config_creation() {
        let config = AnalysisConfig {
            interval_seconds: 300,
            predictive_enabled: true,
        };

        assert_eq!(config.interval_seconds, 300);
        assert!(config.predictive_enabled);
    }

    #[test]
    fn test_analysis_config_intervals() {
        let intervals = vec![30, 60, 120, 300, 600, 1800, 3600];

        for interval in intervals {
            let config = AnalysisConfig {
                interval_seconds: interval,
                predictive_enabled: false,
            };

            assert_eq!(config.interval_seconds, interval);
        }
    }

    #[test]
    fn test_analysis_config_predictive_flags() {
        let config_enabled = AnalysisConfig {
            interval_seconds: 60,
            predictive_enabled: true,
        };
        assert!(config_enabled.predictive_enabled);

        let config_disabled = AnalysisConfig {
            interval_seconds: 60,
            predictive_enabled: false,
        };
        assert!(!config_disabled.predictive_enabled);
    }

    #[test]
    fn test_analysis_config_clone() {
        let config1 = AnalysisConfig {
            interval_seconds: 120,
            predictive_enabled: true,
        };
        let config2 = config1.clone();

        assert_eq!(config1.interval_seconds, config2.interval_seconds);
        assert_eq!(config1.predictive_enabled, config2.predictive_enabled);
    }

    // ==================== METRICS RESPONSE TESTS ====================

    #[test]
    fn test_performance_metrics_response_creation() {
        let mut metrics = std::collections::HashMap::new();
        metrics.insert("cpu".to_string(), 50.0);

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(response.metrics.len(), 1);
        assert_eq!(
            response.metrics.get("cpu").expect("Test setup failed"),
            &50.0
        );
    }

    #[test]
    fn test_performance_metrics_response_multiple_metrics() {
        let mut metrics = std::collections::HashMap::new();
        metrics.insert("cpu_usage".to_string(), 45.2);
        metrics.insert("memory_usage".to_string(), 67.8);
        metrics.insert("disk_io".to_string(), 120.5);

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(response.metrics.len(), 3);
        assert!(response.metrics.contains_key("cpu_usage"));
        assert!(response.metrics.contains_key("memory_usage"));
        assert!(response.metrics.contains_key("disk_io"));
    }

    #[test]
    fn test_performance_metrics_response_serialization() {
        let mut metrics = std::collections::HashMap::new();
        metrics.insert("test_metric".to_string(), 100.0);

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&response).expect("Test setup failed");
        assert!(json.contains("test_metric"));
        assert!(json.contains("100"));
    }

    #[test]
    fn test_performance_metrics_response_clone() {
        let mut metrics = std::collections::HashMap::new();
        metrics.insert("metric".to_string(), 75.5);

        let response1 = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };
        let response2 = response1.clone();

        assert_eq!(response1.metrics.len(), response2.metrics.len());
    }

    // ==================== ALERT TESTS ====================

    #[test]
    fn test_performance_alert_creation() {
        let alert = PerformanceAlert {
            id: "alert_001".to_string(),
            message: "Test alert".to_string(),
            severity: "warning".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(alert.id, "alert_001");
        assert_eq!(alert.message, "Test alert");
        assert_eq!(alert.severity, "warning");
    }

    #[test]
    fn test_performance_alert_severity_levels() {
        let severities = vec!["info", "warning", "error", "critical"];

        for severity in severities {
            let alert = PerformanceAlert {
                id: format!("alert_{severity}"),
                message: format!("{} alert", severity),
                severity: severity.to_string(),
                timestamp: std::time::SystemTime::now(),
            };

            assert_eq!(alert.severity, severity);
        }
    }

    #[test]
    fn test_performance_alert_serialization() {
        let alert = PerformanceAlert {
            id: "alert_test".to_string(),
            message: "Serialization test".to_string(),
            severity: "info".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        let json = serde_json::to_string(&alert).expect("Test setup failed");
        assert!(json.contains("alert_test"));
        assert!(json.contains("Serialization test"));
        assert!(json.contains("info"));
    }

    #[test]
    fn test_performance_alert_clone() {
        let alert1 = PerformanceAlert {
            id: "alert_001".to_string(),
            message: "Clone test".to_string(),
            severity: "warning".to_string(),
            timestamp: std::time::SystemTime::now(),
        };
        let alert2 = alert1.clone();

        assert_eq!(alert1.id, alert2.id);
        assert_eq!(alert1.message, alert2.message);
        assert_eq!(alert1.severity, alert2.severity);
    }

    #[test]
    fn test_performance_alert_debug() {
        let alert = PerformanceAlert {
            id: "debug_test".to_string(),
            message: "Debug output test".to_string(),
            severity: "info".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        let debug_str = format!("{:?}", alert);
        assert!(debug_str.contains("PerformanceAlert"));
        assert!(debug_str.contains("debug_test"));
    }

    // ==================== RECOMMENDATION TESTS ====================

    #[test]
    fn test_performance_recommendation_creation() {
        let rec = PerformanceRecommendation {
            id: "rec_001".to_string(),
            title: "Test Recommendation".to_string(),
            description: "Test description".to_string(),
            impact: "High impact".to_string(),
            priority: 1,
        };

        assert_eq!(rec.id, "rec_001");
        assert_eq!(rec.title, "Test Recommendation");
        assert_eq!(rec.priority, 1);
    }

    #[test]
    fn test_performance_recommendation_priorities() {
        let priorities = vec![1, 2, 3, 4, 5];

        for priority in priorities {
            let rec = PerformanceRecommendation {
                id: format!("rec_{priority}"),
                title: format!("Priority {priority}"),
                description: "Description".to_string(),
                impact: "Impact".to_string(),
                priority,
            };

            assert_eq!(rec.priority, priority);
        }
    }

    #[test]
    fn test_performance_recommendation_serialization() {
        let rec = PerformanceRecommendation {
            id: "rec_test".to_string(),
            title: "Test Title".to_string(),
            description: "Test Description".to_string(),
            impact: "Medium".to_string(),
            priority: 2,
        };

        let json = serde_json::to_string(&rec).expect("Test setup failed");
        assert!(json.contains("rec_test"));
        assert!(json.contains("Test Title"));
        assert!(json.contains("priority"));
    }

    #[test]
    fn test_performance_recommendation_clone() {
        let rec1 = PerformanceRecommendation {
            id: "rec_001".to_string(),
            title: "Clone Test".to_string(),
            description: "Description".to_string(),
            impact: "High".to_string(),
            priority: 1,
        };
        let rec2 = rec1.clone();

        assert_eq!(rec1.id, rec2.id);
        assert_eq!(rec1.title, rec2.title);
        assert_eq!(rec1.priority, rec2.priority);
    }

    #[test]
    fn test_performance_recommendation_debug() {
        let rec = PerformanceRecommendation {
            id: "debug_rec".to_string(),
            title: "Debug Test".to_string(),
            description: "Debug description".to_string(),
            impact: "Low".to_string(),
            priority: 3,
        };

        let debug_str = format!("{:?}", rec);
        assert!(debug_str.contains("PerformanceRecommendation"));
        assert!(debug_str.contains("debug_rec"));
    }

    // ==================== HANDLER TESTS ====================

    #[tokio::test]
    async fn test_get_performance_metrics_handler() {
        let result = get_performance_metrics().await;

        assert!(result.is_ok());
        let response = result.expect("Test setup failed");

        assert!(response.0.metrics.contains_key("cpu_usage"));
        assert!(response.0.metrics.contains_key("memory_usage"));
    }

    #[tokio::test]
    async fn test_get_performance_alerts_handler() {
        let result = get_performance_alerts().await;

        assert!(result.is_ok());
        let alerts = result.expect("Test setup failed");

        assert_eq!(alerts.0.len(), 2);
        assert_eq!(alerts.0[0].id, "alert_001");
        assert_eq!(alerts.0[1].id, "alert_002");
    }

    #[tokio::test]
    async fn test_get_performance_recommendations_handler() {
        let result = get_performance_recommendations().await;

        assert!(result.is_ok());
        let recommendations = result.expect("Test setup failed");

        assert_eq!(recommendations.0.len(), 2);
        assert_eq!(recommendations.0[0].id, "rec_001");
        assert_eq!(recommendations.0[1].id, "rec_002");
    }

    #[tokio::test]
    async fn test_metrics_handler_consistency() {
        // Call handler multiple times
        for _ in 0..5 {
            let result = get_performance_metrics().await;
            assert!(result.is_ok());

            let response = result.expect("Test setup failed");
            assert_eq!(response.0.metrics.len(), 4); // Always returns 4 metrics
        }
    }

    #[tokio::test]
    async fn test_alerts_handler_consistency() {
        // Call handler multiple times
        for _ in 0..5 {
            let result = get_performance_alerts().await;
            assert!(result.is_ok());

            let alerts = result.expect("Test setup failed");
            assert_eq!(alerts.0.len(), 2); // Always returns 2 alerts
        }
    }

    #[tokio::test]
    async fn test_recommendations_handler_consistency() {
        // Call handler multiple times
        for _ in 0..5 {
            let result = get_performance_recommendations().await;
            assert!(result.is_ok());

            let recommendations = result.expect("Test setup failed");
            assert_eq!(recommendations.0.len(), 2); // Always returns 2 recommendations
        }
    }

    // ==================== INTEGRATION TESTS ====================

    #[tokio::test]
    async fn test_full_performance_workflow() {
        // Get metrics
        let metrics_result = get_performance_metrics().await;
        assert!(metrics_result.is_ok());

        // Get alerts
        let alerts_result = get_performance_alerts().await;
        assert!(alerts_result.is_ok());

        // Get recommendations
        let recs_result = get_performance_recommendations().await;
        assert!(recs_result.is_ok());

        // Verify all succeeded - results are already validated by is_ok() above
        let _metrics = metrics_result.expect("Test setup failed");
        let _alerts = alerts_result.expect("Test setup failed");
        let _recs = recs_result.expect("Test setup failed");
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_analysis_config_zero_interval() {
        let config = AnalysisConfig {
            interval_seconds: 0,
            predictive_enabled: false,
        };

        assert_eq!(config.interval_seconds, 0);
    }

    #[test]
    fn test_analysis_config_large_interval() {
        let config = AnalysisConfig {
            interval_seconds: u64::MAX,
            predictive_enabled: true,
        };

        assert_eq!(config.interval_seconds, u64::MAX);
    }

    #[test]
    fn test_performance_alert_empty_message() {
        let alert = PerformanceAlert {
            id: "empty_test".to_string(),
            message: String::new(),
            severity: "info".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        assert!(alert.message.is_empty());
    }

    #[test]
    fn test_performance_recommendation_zero_priority() {
        let rec = PerformanceRecommendation {
            id: "zero_priority".to_string(),
            title: "Zero Priority Test".to_string(),
            description: "Test".to_string(),
            impact: "None".to_string(),
            priority: 0,
        };

        assert_eq!(rec.priority, 0);
    }

    #[test]
    fn test_performance_recommendation_max_priority() {
        let rec = PerformanceRecommendation {
            id: "max_priority".to_string(),
            title: "Max Priority Test".to_string(),
            description: "Test".to_string(),
            impact: "Critical".to_string(),
            priority: u32::MAX,
        };

        assert_eq!(rec.priority, u32::MAX);
    }

    // ==================== TIMESTAMP TESTS ====================

    #[test]
    fn test_metrics_response_timestamp() {
        let metrics = std::collections::HashMap::new();
        let before = std::time::SystemTime::now();

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        let after = std::time::SystemTime::now();

        // Timestamp should be between before and after
        assert!(response.timestamp >= before);
        assert!(response.timestamp <= after);
    }

    #[test]
    fn test_alert_timestamp() {
        let before = std::time::SystemTime::now();

        let alert = PerformanceAlert {
            id: "time_test".to_string(),
            message: "Timestamp test".to_string(),
            severity: "info".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        let after = std::time::SystemTime::now();

        assert!(alert.timestamp >= before);
        assert!(alert.timestamp <= after);
    }
}
