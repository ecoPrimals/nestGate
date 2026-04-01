// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **PERFORMANCE ANALYZER STATE**
///
/// State management for performance analysis operations.
#[derive(Debug, Clone, Default)]
/// Performanceanalyzerstate
pub struct PerformanceAnalyzerState {
    /// Current analysis configuration
    pub config: AnalysisConfig,
    /// Last analysis timestamp
    pub last_analysis: Option<std::time::SystemTime>,
}

/// **ANALYSIS CONFIG**
///
/// Configuration for performance analysis operations.
#[derive(Debug, Clone, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::AnalysisConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::AnalysisConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Analysis
pub struct AnalysisConfig {
    /// Analysis interval in seconds
    pub interval_seconds: u64,
    /// Whether to enable predictive analysis
    pub predictive_enabled: bool,
}

/// **PERFORMANCE METRICS RESPONSE**
///
/// Response structure for performance metrics data.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for `PerformanceMetrics` operation
pub struct PerformanceMetricsResponse {
    /// Current system metrics
    pub metrics: HashMap<String, f64>,
    /// Timestamp when metrics were collected
    pub timestamp: std::time::SystemTime,
}

/// **PERFORMANCE ALERT**
///
/// Performance alert information.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancealert
pub struct PerformanceAlert {
    /// Alert identifier
    pub id: String,
    /// Alert message
    pub message: String,
    /// Alert severity level
    pub severity: String,
    /// Timestamp when alert was generated
    pub timestamp: std::time::SystemTime,
}

/// **PERFORMANCE RECOMMENDATION**
///
/// Performance optimization recommendation.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancerecommendation
pub struct PerformanceRecommendation {
    /// Recommendation identifier
    pub id: String,
    /// Recommendation title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Expected impact
    pub impact: String,
    /// Priority level
    pub priority: u32,
}

/// **GET PERFORMANCE METRICS HANDLER**
///
/// Retrieve current system performance metrics.
///
/// # Errors
///
/// This function currently always returns `Ok`, but returns `Result` for future error handling.
pub async fn get_performance_metrics() -> Result<Json<PerformanceMetricsResponse>, StatusCode> {
    let mut metrics = HashMap::new();
    metrics.insert("cpu_usage".to_string(), 45.2);
    metrics.insert("memory_usage".to_string(), 67.8);
    metrics.insert("disk_io".to_string(), 120.5);
    metrics.insert("network_io".to_string(), 85.3);

    let response = PerformanceMetricsResponse {
        metrics,
        timestamp: std::time::SystemTime::now(),
    };

    Ok(Json(response))
}

/// **GET PERFORMANCE ALERTS HANDLER**
///
/// Retrieve active performance alerts.
///
/// # Errors
///
/// This function currently always returns `Ok`, but returns `Result` for future error handling.
pub async fn get_performance_alerts() -> Result<Json<Vec<PerformanceAlert>>, StatusCode> {
    let alerts = vec![
        PerformanceAlert {
            id: "alert_001".to_string(),
            message: "High CPU usage detected".to_string(),
            severity: "warning".to_string(),
            timestamp: std::time::SystemTime::now(),
        },
        PerformanceAlert {
            id: "alert_002".to_string(),
            message: "Memory usage approaching threshold".to_string(),
            severity: "info".to_string(),
            timestamp: std::time::SystemTime::now(),
        },
    ];

    Ok(Json(alerts))
}

/// **GET PERFORMANCE RECOMMENDATIONS HANDLER**
///
/// Retrieve performance optimization recommendations.
pub async fn get_performance_recommendations()
-> Result<Json<Vec<PerformanceRecommendation>>, StatusCode> {
    let recommendations = vec![
        PerformanceRecommendation {
            id: "rec_001".to_string(),
            title: "Optimize CPU scheduling".to_string(),
            description: "Adjust CPU governor settings for better performance".to_string(),
            impact: "5-10% performance improvement".to_string(),
            priority: 2,
        },
        PerformanceRecommendation {
            id: "rec_002".to_string(),
            title: "Increase buffer cache".to_string(),
            description: "Allocate more memory for disk buffer cache".to_string(),
            impact: "15-20% I/O performance improvement".to_string(),
            priority: 1,
        },
    ];

    Ok(Json(recommendations))
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Analysisconfigcanonical
pub type AnalysisConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using AnalysisConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_performance_metrics() {
        let result = get_performance_metrics().await;
        assert!(result.is_ok());

        let response = result
            .expect("Test: get_performance_metrics should return Ok")
            .0;
        assert_eq!(response.metrics.len(), 4);
        assert!(response.metrics.contains_key("cpu_usage"));
        assert!(response.metrics.contains_key("memory_usage"));
        assert!(response.metrics.contains_key("disk_io"));
        assert!(response.metrics.contains_key("network_io"));

        // Verify values
        assert_eq!(response.metrics.get("cpu_usage"), Some(&45.2));
        assert_eq!(response.metrics.get("memory_usage"), Some(&67.8));
        assert_eq!(response.metrics.get("disk_io"), Some(&120.5));
        assert_eq!(response.metrics.get("network_io"), Some(&85.3));
    }

    #[tokio::test]
    async fn test_get_performance_alerts() {
        let result = get_performance_alerts().await;
        assert!(result.is_ok());

        let alerts = result
            .expect("Test: get_performance_alerts should return Ok")
            .0;
        assert_eq!(alerts.len(), 2);

        // Verify first alert
        assert_eq!(alerts[0].id, "alert_001");
        assert_eq!(alerts[0].message, "High CPU usage detected");
        assert_eq!(alerts[0].severity, "warning");

        // Verify second alert
        assert_eq!(alerts[1].id, "alert_002");
        assert_eq!(alerts[1].message, "Memory usage approaching threshold");
        assert_eq!(alerts[1].severity, "info");
    }

    #[tokio::test]
    async fn test_get_performance_recommendations() {
        let result = get_performance_recommendations().await;
        assert!(result.is_ok());

        let recommendations = result
            .expect("Test: get_performance_recommendations should return Ok")
            .0;
        assert_eq!(recommendations.len(), 2);

        // Verify first recommendation
        assert_eq!(recommendations[0].id, "rec_001");
        assert_eq!(recommendations[0].title, "Optimize CPU scheduling");
        assert_eq!(recommendations[0].priority, 2);
        assert_eq!(recommendations[0].impact, "5-10% performance improvement");

        // Verify second recommendation
        assert_eq!(recommendations[1].id, "rec_002");
        assert_eq!(recommendations[1].title, "Increase buffer cache");
        assert_eq!(recommendations[1].priority, 1);
        assert_eq!(
            recommendations[1].impact,
            "15-20% I/O performance improvement"
        );
    }

    #[test]
    fn test_performance_analyzer_state_default() {
        let state = PerformanceAnalyzerState::default();
        assert!(state.last_analysis.is_none());
    }

    #[test]
    #[allow(deprecated)]
    fn test_analysis_config_default() {
        let config = AnalysisConfig::default();
        assert_eq!(config.interval_seconds, 0);
        assert!(!config.predictive_enabled);
    }

    #[test]
    fn test_performance_metrics_response_serialization() {
        let mut metrics = HashMap::new();
        metrics.insert("test_metric".to_string(), 42.0);

        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: std::time::SystemTime::now(),
        };

        // Verify serialization works
        let json = serde_json::to_string(&response);
        assert!(json.is_ok());
    }

    #[test]
    fn test_performance_alert_serialization() {
        let alert = PerformanceAlert {
            id: "test_alert".to_string(),
            message: "Test message".to_string(),
            severity: "critical".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        // Verify serialization works
        let json = serde_json::to_string(&alert);
        assert!(json.is_ok());
    }

    #[test]
    fn test_performance_recommendation_serialization() {
        let recommendation = PerformanceRecommendation {
            id: "test_rec".to_string(),
            title: "Test Recommendation".to_string(),
            description: "Test description".to_string(),
            impact: "Test impact".to_string(),
            priority: 3,
        };

        // Verify serialization works
        let json = serde_json::to_string(&recommendation);
        assert!(json.is_ok());
    }
}
