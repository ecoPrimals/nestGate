// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Performance analytics HTTP handlers backed by Linux `/proc` metrics.

mod alerts;
mod collector;
mod config;
mod recommendations;
mod types;

use std::time::SystemTime;

use axum::{http::StatusCode, response::Json};
use serde_json::{Value, json};
use tracing::warn;

pub use types::{
    PerformanceAlert, PerformanceAnalyzerState, PerformanceMetricsResponse,
    PerformanceRecommendation,
};

#[cfg(test)]
pub use types::AnalysisConfig;

fn analytics_error(err: impl std::fmt::Display) -> (StatusCode, Json<Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "status": "error",
            "message": err.to_string(),
        })),
    )
}

/// Retrieve current system performance metrics from `/proc`.
///
/// # Errors
///
/// Returns HTTP 500 with a structured JSON body when metric collection fails.
pub async fn get_performance_metrics()
-> Result<Json<PerformanceMetricsResponse>, (StatusCode, Json<Value>)> {
    match collector::collect_performance_metrics().await {
        Ok(metrics) => Ok(Json(PerformanceMetricsResponse {
            metrics,
            timestamp: SystemTime::now(),
        })),
        Err(err) => {
            warn!(error = %err, "performance metrics collection failed");
            Err(analytics_error(err))
        }
    }
}

/// Retrieve active performance alerts based on configured thresholds.
///
/// # Errors
///
/// Returns HTTP 500 with a structured JSON body when metrics cannot be collected.
pub async fn get_performance_alerts()
-> Result<Json<Vec<PerformanceAlert>>, (StatusCode, Json<Value>)> {
    match alerts::generate_performance_alerts().await {
        Ok(alerts) => Ok(Json(alerts)),
        Err(err) => {
            warn!(error = %err, "performance alert generation failed");
            Err(analytics_error(err))
        }
    }
}

/// Retrieve performance optimization recommendations derived from current metrics.
///
/// # Errors
///
/// Returns HTTP 500 with a structured JSON body when metrics cannot be collected.
pub async fn get_performance_recommendations()
-> Result<Json<Vec<PerformanceRecommendation>>, (StatusCode, Json<Value>)> {
    match recommendations::generate_performance_recommendations().await {
        Ok(recommendations) => Ok(Json(recommendations)),
        Err(err) => {
            warn!(error = %err, "performance recommendation generation failed");
            Err(analytics_error(err))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[tokio::test]
    async fn get_performance_metrics_returns_not_implemented() {
        let result = get_performance_metrics().await;
        assert!(
            result.is_ok(),
            "expected successful metrics collection: {result:?}"
        );
        let response = result.expect("metrics handler should succeed");
        assert!(response.0.metrics.contains_key("cpu_usage"));
        assert!(response.0.metrics.contains_key("memory_usage"));
        assert!(response.0.metrics.contains_key("disk_read_latency_ms"));
        assert!(response.0.metrics.contains_key("disk_write_latency_ms"));
    }

    #[tokio::test]
    async fn get_performance_alerts_returns_active_alerts_list() {
        let result = get_performance_alerts().await;
        assert!(
            result.is_ok(),
            "expected successful alert generation: {result:?}"
        );
    }

    #[tokio::test]
    async fn get_performance_recommendations_returns_recommendations_list() {
        let result = get_performance_recommendations().await;
        assert!(
            result.is_ok(),
            "expected successful recommendation generation: {result:?}"
        );
    }

    #[test]
    fn performance_analyzer_state_default() {
        let state = PerformanceAnalyzerState::default();
        assert!(state.last_analysis.is_none());
    }

    #[test]
    fn analysis_config_default() {
        let config = super::types::AnalysisConfig::default();
        assert_eq!(config.interval_seconds, 0);
        assert!(!config.predictive_enabled);
    }

    #[test]
    fn performance_metrics_response_serialization() {
        let mut metrics = HashMap::new();
        metrics.insert("test_metric".into(), 42.0);
        let response = PerformanceMetricsResponse {
            metrics,
            timestamp: SystemTime::now(),
        };
        assert!(serde_json::to_string(&response).is_ok());
    }

    #[test]
    fn performance_alert_serialization() {
        let alert = PerformanceAlert {
            id: "test_alert".into(),
            message: "Test message".into(),
            severity: "critical".into(),
            timestamp: SystemTime::now(),
        };
        assert!(serde_json::to_string(&alert).is_ok());
    }

    #[test]
    fn performance_recommendation_serialization() {
        let rec = PerformanceRecommendation {
            id: "test_rec".into(),
            title: "Test".into(),
            description: "Test desc".into(),
            impact: "Test impact".into(),
            priority: 3,
        };
        assert!(serde_json::to_string(&rec).is_ok());
    }
}
