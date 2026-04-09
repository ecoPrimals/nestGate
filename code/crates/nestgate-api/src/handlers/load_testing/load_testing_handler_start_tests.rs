// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! HTTP integration tests for `POST /load-test/start` (scenarios, payloads, thresholds).

#![cfg(test)]

use super::*;
use axum::Router;
use axum_test::TestServer;
use serde_json::json;

/// Helper to create a test router with load testing endpoints
fn create_test_router() -> Router {
    use axum::routing::{get, post};

    Router::new()
        .route("/load-test/start", post(start_load_test))
        .route("/load-test/results", get(get_load_test_results))
        .route("/load-test/history", get(get_load_test_history))
        .route("/load-test/baselines", get(get_performance_baselines))
}

// ==================== START LOAD TEST HANDLER TESTS ====================

#[tokio::test]
async fn test_start_load_test_success() {
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
    assert!(body.get("test_id").is_some());
    assert!(body.get("config").is_some());
    assert!(body.get("started_at").is_some());
}

#[tokio::test]
async fn test_start_load_test_with_ramp_scenario() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 120,
        "concurrent_users": 50,
        "requests_per_second": 10.0,
        "scenario": {
            "Ramp": {
                "start_users": 10,
                "end_users": 100,
                "ramp_duration_seconds": 300
            }
        },
        "endpoints": ["/api/stress"],
        "test_data": {
            "payload_size_bytes": 2048,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 500.0,
            "max_p95_response_time_ms": 1500.0,
            "min_success_rate": 0.99,
            "max_error_rate": 0.01
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    assert!(
        body["test_id"]
            .as_str()
            .expect("Test setup failed")
            .starts_with("test_")
    );
}

#[tokio::test]
async fn test_start_load_test_with_spike_scenario() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 90,
        "concurrent_users": 30,
        "requests_per_second": 7.5,
        "scenario": {
            "Spike": {
                "baseline_users": 20,
                "spike_users": 200,
                "spike_duration_seconds": 60
            }
        },
        "endpoints": ["/api/spike-test"],
        "test_data": {
            "payload_size_bytes": 512,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 800.0,
            "max_p95_response_time_ms": 1800.0,
            "min_success_rate": 0.96,
            "max_error_rate": 0.04
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_start_load_test_with_step_scenario() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 180,
        "concurrent_users": 100,
        "requests_per_second": 15.0,
        "scenario": {
            "Step": {
                "max_users": 100,
                "step_users": 10,
                "step_duration_seconds": 30
            }
        },
        "endpoints": ["/api/step-test"],
        "test_data": {
            "payload_size_bytes": 4096,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 600.0,
            "max_p95_response_time_ms": 1400.0,
            "min_success_rate": 0.98,
            "max_error_rate": 0.02
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_start_load_test_with_custom_headers() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 45,
        "concurrent_users": 15,
        "requests_per_second": 3.0,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/auth-test"],
        "test_data": {
            "payload_size_bytes": 1024,
            "expected_response_size_bytes": 512,
            "custom_headers": {
                "Authorization": "Bearer test-token",
                "X-API-Key": "test-key"
            },
            "body_template": "{\"test\": \"data\"}"
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
    let test_data = &body["config"]["test_data"];
    assert_eq!(
        test_data["custom_headers"]["Authorization"],
        "Bearer test-token"
    );
    assert_eq!(test_data["custom_headers"]["X-API-Key"], "test-key");
}

#[tokio::test]
async fn test_start_load_test_minimal_config() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 30,
        "concurrent_users": 5,
        "requests_per_second": 1.0,
        "scenario": "ConstantLoad",
        "endpoints": ["/health"],
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
async fn test_start_load_test_multiple_endpoints() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 90,
        "concurrent_users": 25,
        "requests_per_second": 8.0,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/endpoint1", "/api/endpoint2", "/api/endpoint3"],
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
    let endpoints = body["config"]["endpoints"]
        .as_array()
        .expect("Test setup failed");
    assert_eq!(endpoints.len(), 3);
}

#[tokio::test]
async fn test_start_load_test_high_concurrency() {
    let server = TestServer::new(create_test_router()).expect("Test setup failed");

    let config = json!({
        "duration_seconds": 300,
        "concurrent_users": 1000,
        "requests_per_second": 100.0,
        "scenario": "ConstantLoad",
        "endpoints": ["/api/stress"],
        "test_data": {
            "payload_size_bytes": 1024,
            "expected_response_size_bytes": null,
            "custom_headers": {},
            "body_template": null
        },
        "thresholds": {
            "max_avg_response_time_ms": 2000.0,
            "max_p95_response_time_ms": 5000.0,
            "min_success_rate": 0.90,
            "max_error_rate": 0.10
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_start_load_test_strict_thresholds() {
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
            "max_avg_response_time_ms": 50.0,
            "max_p95_response_time_ms": 100.0,
            "min_success_rate": 0.999,
            "max_error_rate": 0.001
        }
    });

    let response = server.post("/load-test/start").json(&config).await;

    response.assert_status_ok();

    let body: serde_json::Value = response.json();
    let thresholds = &body["config"]["thresholds"];
    assert_eq!(thresholds["max_avg_response_time_ms"], 50.0);
    assert_eq!(thresholds["min_success_rate"], 0.999);
}
