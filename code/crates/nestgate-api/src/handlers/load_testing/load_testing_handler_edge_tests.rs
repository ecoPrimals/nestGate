// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Edge cases, HTTP error paths, and response validation for load testing handlers.

#![cfg(test)]

use super::*;
use axum::{Router, http::StatusCode};
use axum_test::TestServer;
use serde_json::json;

fn create_test_router() -> Router {
    use axum::routing::{get, post};

    Router::new()
        .route("/load-test/start", post(start_load_test))
        .route("/load-test/results", get(get_load_test_results))
        .route("/load-test/history", get(get_load_test_history))
        .route("/load-test/baselines", get(get_performance_baselines))
}

// ==================== EDGE CASE TESTS ====================

#[tokio::test]
async fn test_start_load_test_with_zero_duration() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 0,
        "concurrent_users": 10,
        "requests_per_second": 5.0,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/test"],
        "test_data": {
            "payload_size_bytes": 1024,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 1000.0,
            "max_p95_response_time_ms": 2000.0,
            "min_success_rate": 0.95,
            "max_error_rate": 0.05
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    // Should accept but might have special handling
    response.assert_status_ok();
}

#[tokio::test]
async fn test_start_load_test_with_empty_endpoints() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 60,
        "concurrent_users": 10,
        "requests_per_second": 5.0,
        "scenario": "ConstantLoad",
        "endpoints": [],
        "test_data": {
            "payload_size_bytes": 1024,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 1000.0,
            "max_p95_response_time_ms": 2000.0,
            "min_success_rate": 0.95,
            "max_error_rate": 0.05
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    // Should accept empty endpoints list
    response.assert_status_ok();
}

#[tokio::test]
async fn test_start_load_test_with_large_payload() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 60,
        "concurrent_users": 10,
        "requests_per_second": 5.0,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/upload"],
        "test_data": {
            "payload_size_bytes": 10_485_760, // 10 MB
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 5000.0,
            "max_p95_response_time_ms": 10000.0,
            "min_success_rate": 0.95,
            "max_error_rate": 0.05
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let payload_size = body["config"]["test_data"]["payload_size_bytes"]
        .as_u64()
        .expect("Test setup failed");
    assert_eq!(payload_size, 10_485_760);
}

#[tokio::test]
async fn test_start_load_test_with_very_low_rps() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 60,
        "concurrent_users": 1,
        "requests_per_second": 0.1,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/slow"],
        "test_data": {
            "payload_size_bytes": 1024,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 1000.0,
            "max_p95_response_time_ms": 2000.0,
            "min_success_rate": 0.95,
            "max_error_rate": 0.05
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_start_load_test_with_fractional_rps() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 60,
        "concurrent_users": 5,
        "requests_per_second": 2.5,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/test"],
        "test_data": {
            "payload_size_bytes": 1024,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 1000.0,
            "max_p95_response_time_ms": 2000.0,
            "min_success_rate": 0.95,
            "max_error_rate": 0.05
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let rps = body["config"]["requests_per_second"]
        .as_f64()
        .expect("Test setup failed");
    assert_eq!(rps, 2.5);
}

#[tokio::test]
async fn test_start_load_test_with_single_user() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 60,
        "concurrent_users": 1,
        "requests_per_second": 1.0,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/test"],
        "test_data": {
            "payload_size_bytes": 1024,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 1000.0,
            "max_p95_response_time_ms": 2000.0,
            "min_success_rate": 0.95,
            "max_error_rate": 0.05
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();
}

// ==================== ERROR PATH TESTS ====================

#[tokio::test]
async fn test_start_load_test_malformed_json() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server
        .post("/load-test/start")
        .content_type("application/json")
        .text("{invalid json}")
        .await;

    // Should fail - exact status code may vary by framework version
    let status = response.status_code();
    assert!(
        status.is_client_error()
            || status == StatusCode::BAD_REQUEST
            || status == StatusCode::UNPROCESSABLE_ENTITY,
        "Expected client error, got: {:?}",
        status
    );
}

#[tokio::test]
async fn test_start_load_test_missing_required_fields() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 60
        // Missing required fields
    });

    let response = server.post("/load-test/start").json(&config).await;

    // Should fail with validation error
    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_start_load_test_wrong_content_type() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server
        .post("/load-test/start")
        .content_type("text/plain")
        .text("not json")
        .await;

    // Should fail with unsupported media type
    assert_eq!(response.status_code(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
}

#[tokio::test]
async fn test_get_methods_with_post() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    // GET endpoints should not accept POST
    let response = server.post("/load-test/results").await;

    assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_post_method_with_get() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    // POST endpoint should not accept GET
    let response = server.get("/load-test/start").await;

    assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_nonexistent_endpoint() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/nonexistent").await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}

// ==================== RESPONSE VALIDATION TESTS ====================

#[tokio::test]
async fn test_responses_have_correct_content_type() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let response = server.get("/load-test/results").await;

    response.assert_status_ok();

    // Verify response is valid JSON
    let _body: serde_json::Value = response.json();
}

#[tokio::test]
async fn test_start_load_test_response_structure() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 60,
        "concurrent_users": 10,
        "requests_per_second": 5.0,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/test"],
        "test_data": {
            "payload_size_bytes": 1024,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 1000.0,
            "max_p95_response_time_ms": 2000.0,
            "min_success_rate": 0.95,
            "max_error_rate": 0.05
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();

    // Verify all required fields present
    assert!(body.get("test_id").is_some(), "Missing test_id");
    assert!(body.get("config").is_some(), "Missing config");
    assert!(body.get("started_at").is_some(), "Missing started_at");

    // Verify config matches request
    let returned_config = &body["config"];
    assert_eq!(returned_config["duration_seconds"], 60);
    assert_eq!(returned_config["concurrent_users"], 10);
    assert_eq!(returned_config["requests_per_second"], 5.0);
}
