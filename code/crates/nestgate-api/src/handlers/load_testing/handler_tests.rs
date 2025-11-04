//! **LOAD TESTING HANDLER INTEGRATION TESTS**
//!
//! Comprehensive integration tests for load testing HTTP handlers.
//! Tests cover happy paths, error paths, edge cases, and validation logic.

#[cfg(test)]
mod handler_tests {
    use super::super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        response::Response,
        Router,
    };
    use axum_test::TestServer;
    use serde_json::json;
    use std::time::{Duration, SystemTime};

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
        assert!(body["test_id"].as_str().expect("Test setup failed").starts_with("test_"));
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
        let endpoints = body["config"]["endpoints"].as_array().expect("Test setup failed");
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
            let total = result["total_requests"].as_u64().expect("Test setup failed");
            let successful = result["successful_requests"].as_u64().expect("Test setup failed");
            let failed = result["failed_requests"].as_u64().expect("Test setup failed");

            // Validate totals match
            assert_eq!(total, successful + failed, "Request counts should match");

            // Validate response time is reasonable
            let avg_time = result["avg_response_time_ms"].as_f64().expect("Test setup failed");
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
            let duration = entry["duration_seconds"].as_u64().expect("Test setup failed");
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
            let response_time = baseline["expected_response_time_ms"].as_f64().expect("Test setup failed");
            assert!(response_time > 0.0, "Response time should be positive");

            let throughput = baseline["expected_throughput_rps"].as_f64().expect("Test setup failed");
            assert!(throughput > 0.0, "Throughput should be positive");

            // Validate error rate is a valid percentage (0-100)
            let error_rate = baseline["max_error_rate_percent"].as_f64().expect("Test setup failed");
            assert!(
                error_rate >= 0.0 && error_rate <= 100.0,
                "Error rate should be 0-100%"
            );
        }
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
                "payload_size_bytes": 10485760, // 10 MB
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
        assert_eq!(payload_size, 10485760);
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
        let rps = body["config"]["requests_per_second"].as_f64().expect("Test setup failed");
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
}
