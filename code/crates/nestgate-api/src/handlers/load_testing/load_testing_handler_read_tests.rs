// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! HTTP integration tests for load-test read endpoints: results, history, baselines.

#![cfg(test)]

use super::*;
use axum::Router;
use axum_test::TestServer;

fn create_test_router() -> Router {
    use axum::routing::{get, post};

    Router::new()
        .route("/load-test/start", post(start_load_test))
        .route("/load-test/results", get(get_load_test_results))
        .route("/load-test/history", get(get_load_test_history))
        .route("/load-test/baselines", get(get_performance_baselines))
}

// ==================== GET LOAD TEST RESULTS HANDLER TESTS ====================

#[tokio::test]
async fn test_get_load_test_results_success() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/results").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let results = body.as_array().expect("Test setup failed");
    assert!(!results.is_empty());

    // Check first result structure
    let first_result = &results[0];
    assert!(first_result.get("success").is_some());
    assert!(first_result.get("duration_seconds").is_some());
    assert!(first_result.get("total_requests").is_some());
    assert!(first_result.get("successful_requests").is_some());
    assert!(first_result.get("failed_requests").is_some());
    assert!(first_result.get("avg_response_time_ms").is_some());
}

#[tokio::test]
async fn test_get_load_test_results_returns_multiple() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/results").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let results = body.as_array().expect("Test setup failed");
    assert!(results.len() >= 2, "Should return multiple results");
}

#[tokio::test]
async fn test_get_load_test_results_validates_data() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/results").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let results = body.as_array().expect("Test setup failed");

    for result in results {
        let total = result["total_requests"]
            .as_u64()
            .expect("Test setup failed");
        let successful = result["successful_requests"]
            .as_u64()
            .expect("Test setup failed");
        let failed = result["failed_requests"]
            .as_u64()
            .expect("Test setup failed");

        // Validate totals match
        assert_eq!(total, successful + failed, "Request counts should match");

        // Validate response time is reasonable
        let avg_time = result["avg_response_time_ms"]
            .as_f64()
            .expect("Test setup failed");
        assert!(avg_time > 0.0, "Response time should be positive");
    }
}

// ==================== GET LOAD TEST HISTORY HANDLER TESTS ====================

#[tokio::test]
async fn test_get_load_test_history_success() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/history").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let history = body.as_array().expect("Test setup failed");
    assert!(!history.is_empty());

    // Check first history entry
    let first_entry = &history[0];
    assert!(first_entry.get("test_id").is_some());
    assert!(first_entry.get("test_name").is_some());
    assert!(first_entry.get("executed_at").is_some());
    assert!(first_entry.get("duration_seconds").is_some());
    assert!(first_entry.get("result").is_some());
}

#[tokio::test]
async fn test_get_load_test_history_validates_structure() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/history").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let history = body.as_array().expect("Test setup failed");

    for entry in history {
        // Validate test_id format
        let test_id = entry["test_id"].as_str().expect("Test setup failed");
        assert!(
            test_id.starts_with("test_"),
            "Test ID should have expected prefix"
        );

        // Validate test_name is not empty
        let test_name = entry["test_name"].as_str().expect("Test setup failed");
        assert!(!test_name.is_empty(), "Test name should not be empty");

        // Validate duration is positive
        let duration = entry["duration_seconds"]
            .as_u64()
            .expect("Test setup failed");
        assert!(duration > 0, "Duration should be positive");

        // Validate result contains expected fields
        let result = &entry["result"];
        assert!(result.get("success").is_some());
        assert!(result.get("total_requests").is_some());
    }
}

// ==================== GET PERFORMANCE BASELINES HANDLER TESTS ====================

#[tokio::test]
async fn test_get_performance_baselines_success() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/baselines").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let baselines = body.as_array().expect("Test setup failed");
    assert!(!baselines.is_empty());

    // Check first baseline
    let first_baseline = &baselines[0];
    assert!(first_baseline.get("baseline_id").is_some());
    assert!(first_baseline.get("name").is_some());
    assert!(first_baseline.get("expected_response_time_ms").is_some());
    assert!(first_baseline.get("expected_throughput_rps").is_some());
    assert!(first_baseline.get("max_error_rate_percent").is_some());
}

#[tokio::test]
async fn test_get_performance_baselines_multiple() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/baselines").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let baselines = body.as_array().expect("Test setup failed");
    assert!(baselines.len() >= 2, "Should return multiple baselines");
}

#[tokio::test]
async fn test_get_performance_baselines_validates_metrics() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/baselines").await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let baselines = body.as_array().expect("Test setup failed");

    for baseline in baselines {
        // Validate baseline_id format
        let baseline_id = baseline["baseline_id"].as_str().expect("Test setup failed");
        assert!(
            baseline_id.starts_with("baseline_"),
            "Baseline ID should have expected prefix"
        );

        // Validate metrics are positive
        let response_time = baseline["expected_response_time_ms"]
            .as_f64()
            .expect("Test setup failed");
        assert!(response_time > 0.0, "Response time should be positive");

        let throughput = baseline["expected_throughput_rps"]
            .as_f64()
            .expect("Test setup failed");
        assert!(throughput > 0.0, "Throughput should be positive");

        // Validate error rate is a valid percentage (0-100)
        let error_rate = baseline["max_error_rate_percent"]
            .as_f64()
            .expect("Test setup failed");
        assert!(
            (0.0..=100.0).contains(&error_rate),
            "Error rate should be 0-100%"
        );
    }
}
