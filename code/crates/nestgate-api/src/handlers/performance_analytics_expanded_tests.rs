// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **EXPANDED PERFORMANCE ANALYTICS TESTS**
//!
//! Comprehensive test coverage for `performance_analytics.rs` improving overall coverage.
//! Focus on real-world scenarios, data validation, and edge cases.

#[cfg(test)]
mod expanded_performance_tests {
    use super::super::performance_analytics::*;
    use std::collections::HashMap;
    use std::time::SystemTime;

    // ==================== PERFORMANCE METRICS RESPONSE TESTS ====================

    #[test]
    fn test_performance_metrics_response_creation() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_string(), 45.2);
        metrics.insert("memory_usage".to_string(), 67.8);
        metrics.insert("disk_io".to_string(), 120.5);

        let response = PerformanceMetricsResponse {
            metrics: metrics.clone(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(response.metrics.len(), 3);
        assert_eq!(response.metrics["cpu_usage"], 45.2);
        assert_eq!(response.metrics["memory_usage"], 67.8);
        assert_eq!(response.metrics["disk_io"], 120.5);
    }

    #[test]
    fn test_performance_metrics_response_empty() {
        let response = PerformanceMetricsResponse {
            metrics: HashMap::new(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(response.metrics.len(), 0);
        assert!(response.metrics.is_empty());
    }

    #[test]
    fn test_performance_metrics_response_many_metrics() {
        let mut metrics = HashMap::new();
        for i in 0..100 {
            metrics.insert(format!("metric_{i}"), f64::from(i));
        }

        let response = PerformanceMetricsResponse {
            metrics: metrics.clone(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(response.metrics.len(), 100);
        assert_eq!(response.metrics["metric_0"], 0.0);
        assert_eq!(response.metrics["metric_99"], 99.0);
    }

    #[test]
    fn test_performance_metrics_response_extreme_values() {
        let mut metrics = HashMap::new();
        metrics.insert("very_low".to_string(), 0.0);
        metrics.insert("very_high".to_string(), 999_999.99);
        metrics.insert("negative".to_string(), -100.0);

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: SystemTime::now(),
        };

        assert_eq!(response.metrics["very_low"], 0.0);
        assert_eq!(response.metrics["very_high"], 999_999.99);
        assert_eq!(response.metrics["negative"], -100.0);
    }

    #[test]
    fn test_performance_metrics_response_typical_system() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_string(), 35.5);
        metrics.insert("memory_usage".to_string(), 58.2);
        metrics.insert("disk_io".to_string(), 85.0);
        metrics.insert("network_io".to_string(), 42.7);
        metrics.insert("cache_hit_rate".to_string(), 92.5);

        let response = PerformanceMetricsResponse {
            metrics: metrics.clone(),
            timestamp: SystemTime::now(),
        };

        assert!(response.metrics["cpu_usage"] < 100.0);
        assert!(response.metrics["memory_usage"] < 100.0);
        assert!(response.metrics["cache_hit_rate"] > 90.0);
    }

    // ==================== PERFORMANCE ALERT TESTS ====================

    #[test]
    fn test_performance_alert_creation() {
        let alert = PerformanceAlert {
            id: "alert-001".to_string(),
            message: "High CPU usage detected".to_string(),
            severity: "warning".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(alert.id, "alert-001");
        assert_eq!(alert.message, "High CPU usage detected");
        assert_eq!(alert.severity, "warning");
    }

    #[test]
    fn test_performance_alert_severity_levels() {
        let severities = vec!["info", "warning", "error", "critical"];

        for severity in severities {
            let alert = PerformanceAlert {
                id: format!("alert-{severity}"),
                message: format!("{severity} level alert"),
                severity: severity.to_string(),
                timestamp: SystemTime::now(),
            };

            assert_eq!(alert.severity, severity);
            assert!(alert.message.contains(severity));
        }
    }

    #[test]
    fn test_performance_alert_info_level() {
        let alert = PerformanceAlert {
            id: "info-001".to_string(),
            message: "System performance is normal".to_string(),
            severity: "info".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(alert.severity, "info");
        assert!(alert.message.contains("normal"));
    }

    #[test]
    fn test_performance_alert_warning_level() {
        let alert = PerformanceAlert {
            id: "warn-001".to_string(),
            message: "Memory usage approaching 80%".to_string(),
            severity: "warning".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(alert.severity, "warning");
        assert!(alert.message.contains("Memory"));
    }

    #[test]
    fn test_performance_alert_error_level() {
        let alert = PerformanceAlert {
            id: "err-001".to_string(),
            message: "Disk I/O saturation detected".to_string(),
            severity: "error".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(alert.severity, "error");
        assert!(alert.message.contains("Disk"));
    }

    #[test]
    fn test_performance_alert_critical_level() {
        let alert = PerformanceAlert {
            id: "crit-001".to_string(),
            message: "System resources exhausted - immediate action required".to_string(),
            severity: "critical".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(alert.severity, "critical");
        assert!(alert.message.contains("immediate action"));
    }

    #[test]
    fn test_performance_alert_long_message() {
        let long_message = "This is a very long alert message that describes in detail the performance issue that was detected, including specific metrics, thresholds that were exceeded, and potential impact on system operations.".to_string();

        let alert = PerformanceAlert {
            id: "long-001".to_string(),
            message: long_message.clone(),
            severity: "warning".to_string(),
            timestamp: SystemTime::now(),
        };

        assert_eq!(alert.message, long_message);
        assert!(alert.message.len() > 100);
    }

    // ==================== PERFORMANCE RECOMMENDATION TESTS ====================

    #[test]
    fn test_performance_recommendation_creation() {
        let rec = PerformanceRecommendation {
            id: "rec-001".to_string(),
            title: "Increase Memory Allocation".to_string(),
            description: "Consider increasing memory allocation to improve cache performance"
                .to_string(),
            impact: "High - 30% performance improvement expected".to_string(),
            priority: 1,
        };

        assert_eq!(rec.id, "rec-001");
        assert_eq!(rec.title, "Increase Memory Allocation");
        assert_eq!(rec.priority, 1);
    }

    #[test]
    fn test_performance_recommendation_priorities() {
        let priorities = vec![
            (1, "Critical - Implement immediately"),
            (2, "High - Implement soon"),
            (3, "Medium - Schedule for next sprint"),
            (4, "Low - Consider when resources available"),
        ];

        for (priority, impact) in priorities {
            let rec = PerformanceRecommendation {
                id: format!("rec-{priority}"),
                title: format!("Priority {priority} Recommendation"),
                description: "Test description".to_string(),
                impact: impact.to_string(),
                priority,
            };

            assert_eq!(rec.priority, priority);
            assert!(rec.impact.contains(&priority.to_string()) || !rec.impact.is_empty());
        }
    }

    #[test]
    fn test_performance_recommendation_high_priority() {
        let rec = PerformanceRecommendation {
            id: "rec-high".to_string(),
            title: "Optimize Database Queries".to_string(),
            description:
                "Add indexes to frequently queried tables to reduce query time from 500ms to 50ms"
                    .to_string(),
            impact: "Very High - 90% query performance improvement".to_string(),
            priority: 1,
        };

        assert_eq!(rec.priority, 1);
        assert!(rec.impact.contains("90%"));
        assert!(rec.description.contains("500ms to 50ms"));
    }

    #[test]
    fn test_performance_recommendation_low_priority() {
        let rec = PerformanceRecommendation {
            id: "rec-low".to_string(),
            title: "Update Library Versions".to_string(),
            description:
                "Update to latest versions of non-critical libraries for minor performance gains"
                    .to_string(),
            impact: "Low - 2-5% improvement in specific edge cases".to_string(),
            priority: 5,
        };

        assert_eq!(rec.priority, 5);
        assert!(rec.impact.contains("Low"));
    }

    #[test]
    fn test_performance_recommendation_storage_optimization() {
        let rec = PerformanceRecommendation {
            id: "rec-storage".to_string(),
            title: "Implement Data Compression".to_string(),
            description: "Enable compression on ZFS datasets to reduce storage footprint by 40-60%"
                .to_string(),
            impact: "High - Significant storage savings with minimal CPU overhead".to_string(),
            priority: 2,
        };

        assert_eq!(rec.title, "Implement Data Compression");
        assert!(rec.description.contains("40-60%"));
    }

    #[test]
    fn test_performance_recommendation_network_optimization() {
        let rec = PerformanceRecommendation {
            id: "rec-network".to_string(),
            title: "Enable Network Bonding".to_string(),
            description:
                "Configure LACP network bonding to aggregate bandwidth across multiple interfaces"
                    .to_string(),
            impact: "Medium - 2x network throughput for large transfers".to_string(),
            priority: 3,
        };

        assert_eq!(rec.title, "Enable Network Bonding");
        assert!(rec.impact.contains("2x"));
    }

    #[test]
    fn test_performance_recommendation_caching() {
        let rec = PerformanceRecommendation {
            id: "rec-cache".to_string(),
            title: "Increase ARC Cache Size".to_string(),
            description: "Allocate additional RAM to ZFS ARC cache to improve read performance"
                .to_string(),
            impact: "High - Up to 50% improvement in read latency for cached data".to_string(),
            priority: 1,
        };

        assert_eq!(rec.title, "Increase ARC Cache Size");
        assert!(rec.description.contains("ARC cache"));
    }

    // ==================== SERIALIZATION TESTS ====================

    #[test]
    fn test_performance_metrics_response_serialization() {
        let mut metrics = HashMap::new();
        metrics.insert("cpu".to_string(), 50.0);
        metrics.insert("memory".to_string(), 60.0);

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: SystemTime::now(),
        };

        let json = serde_json::to_string(&response);
        assert!(json.is_ok());

        let deserialized: Result<PerformanceMetricsResponse, _> =
            serde_json::from_str(&json.expect("serialization failed"));
        assert!(deserialized.is_ok());

        let deserialized = deserialized.expect("deserialization failed");
        assert_eq!(deserialized.metrics.len(), 2);
    }

    #[test]
    fn test_performance_alert_serialization() {
        let alert = PerformanceAlert {
            id: "test-001".to_string(),
            message: "Test alert".to_string(),
            severity: "info".to_string(),
            timestamp: SystemTime::now(),
        };

        let json = serde_json::to_string(&alert);
        assert!(json.is_ok());

        let deserialized: Result<PerformanceAlert, _> =
            serde_json::from_str(&json.expect("serialization failed"));
        assert!(deserialized.is_ok());

        let deserialized = deserialized.expect("deserialization failed");
        assert_eq!(deserialized.id, "test-001");
        assert_eq!(deserialized.severity, "info");
    }

    #[test]
    fn test_performance_recommendation_serialization() {
        let rec = PerformanceRecommendation {
            id: "rec-test".to_string(),
            title: "Test Recommendation".to_string(),
            description: "Test description".to_string(),
            impact: "Test impact".to_string(),
            priority: 3,
        };

        let json = serde_json::to_string(&rec);
        assert!(json.is_ok());

        let deserialized: Result<PerformanceRecommendation, _> =
            serde_json::from_str(&json.expect("serialization failed"));
        assert!(deserialized.is_ok());

        let deserialized = deserialized.expect("deserialization failed");
        assert_eq!(deserialized.id, "rec-test");
        assert_eq!(deserialized.priority, 3);
    }

    // ==================== INTEGRATION SCENARIOS ====================

    #[test]
    fn test_performance_monitoring_workflow() {
        // Simulate a performance monitoring workflow

        // Step 1: Collect metrics
        let mut metrics = HashMap::new();
        metrics.insert("cpu_usage".to_string(), 85.0);
        metrics.insert("memory_usage".to_string(), 90.0);

        let response = PerformanceMetricsResponse {
            metrics: metrics.clone(),
            timestamp: SystemTime::now(),
        };

        // Step 2: Generate alert if threshold exceeded
        let alert = if response.metrics["cpu_usage"] > 80.0 {
            Some(PerformanceAlert {
                id: "cpu-high".to_string(),
                message: "CPU usage exceeds 80%".to_string(),
                severity: "warning".to_string(),
                timestamp: SystemTime::now(),
            })
        } else {
            None
        };

        // Step 3: Create recommendation
        let recommendation = PerformanceRecommendation {
            id: "rec-cpu".to_string(),
            title: "Add CPU Resources".to_string(),
            description: "Consider adding more CPU cores or reducing workload".to_string(),
            impact: "High - Will reduce CPU pressure".to_string(),
            priority: 2,
        };

        // Verify workflow
        assert!(alert.is_some());
        assert_eq!(recommendation.priority, 2);
        assert_eq!(response.metrics.len(), 2);
    }

    #[test]
    fn test_multi_severity_alert_handling() {
        let alerts = [
            PerformanceAlert {
                id: "a1".to_string(),
                message: "Minor issue".to_string(),
                severity: "info".to_string(),
                timestamp: SystemTime::now(),
            },
            PerformanceAlert {
                id: "a2".to_string(),
                message: "Attention needed".to_string(),
                severity: "warning".to_string(),
                timestamp: SystemTime::now(),
            },
            PerformanceAlert {
                id: "a3".to_string(),
                message: "Action required".to_string(),
                severity: "critical".to_string(),
                timestamp: SystemTime::now(),
            },
        ];

        assert_eq!(alerts.len(), 3);
        assert_eq!(alerts[0].severity, "info");
        assert_eq!(alerts[1].severity, "warning");
        assert_eq!(alerts[2].severity, "critical");
    }

    #[test]
    fn test_recommendation_prioritization() {
        let mut recommendations = [
            PerformanceRecommendation {
                id: "r1".to_string(),
                title: "Low priority".to_string(),
                description: "Can wait".to_string(),
                impact: "Low".to_string(),
                priority: 5,
            },
            PerformanceRecommendation {
                id: "r2".to_string(),
                title: "High priority".to_string(),
                description: "Do soon".to_string(),
                impact: "High".to_string(),
                priority: 1,
            },
            PerformanceRecommendation {
                id: "r3".to_string(),
                title: "Medium priority".to_string(),
                description: "Schedule".to_string(),
                impact: "Medium".to_string(),
                priority: 3,
            },
        ];

        // Sort by priority
        recommendations.sort_by_key(|r| r.priority);

        assert_eq!(recommendations[0].priority, 1);
        assert_eq!(recommendations[1].priority, 3);
        assert_eq!(recommendations[2].priority, 5);
    }
}
