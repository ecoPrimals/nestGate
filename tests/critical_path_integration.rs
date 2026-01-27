//! Critical Path Integration Tests
//!
//! Tests for critical system paths and workflows

use nestgate_core::response::{
    error_response::ErrorResponseFactory, success_response::SuccessResponseFactory,
};

/// Test critical path: Error response creation
#[test]
fn test_error_responses() {
    let bad_request = ErrorResponseFactory::bad_request("Invalid input");
    assert_eq!(bad_request.status, 400);
    assert_eq!(bad_request.code, "BAD_REQUEST");

    let not_found = ErrorResponseFactory::not_found("/api/resource");
    assert_eq!(not_found.status, 404);
    assert!(not_found.message.contains("not found"));

    let internal = ErrorResponseFactory::internal("Server error");
    assert_eq!(internal.status, 500);

    let unauthorized = ErrorResponseFactory::unauthorized("access");
    assert_eq!(unauthorized.status, 401);

    let forbidden = ErrorResponseFactory::forbidden("resource");
    assert_eq!(forbidden.status, 403);
}

/// Test critical path: Success response creation
#[test]
fn test_success_responses() {
    let created = SuccessResponseFactory::created("user", "user-123");
    assert!(created.message.contains("created successfully"));
    assert!(created.data.is_some());

    let updated = SuccessResponseFactory::updated("dataset", "ds-456");
    assert!(updated.message.contains("updated successfully"));

    let deleted = SuccessResponseFactory::deleted("snapshot");
    assert!(deleted.message.contains("deleted successfully"));

    let retrieved = SuccessResponseFactory::retrieved("pools", 10);
    assert!(retrieved.message.contains("retrieved successfully"));
}

/// Test critical path: Response serialization
#[test]
fn test_response_serialization() {
    let response = SuccessResponseFactory::created("resource", "id-123");
    let json = serde_json::to_string(&response).expect("Should serialize");
    assert!(json.contains("created successfully"));

    let error = ErrorResponseFactory::bad_request("Test error");
    let error_json = serde_json::to_string(&error).expect("Should serialize");
    assert!(error_json.contains("BAD_REQUEST"));
}

/// Test critical path: Response builder pattern
#[test]
fn test_response_builders() {
    use nestgate_core::response::success_response::SuccessResponse;

    let response = SuccessResponse::new("Test")
        .add_data("id", serde_json::json!("123"))
        .add_metadata("version", serde_json::json!("1.0"));

    assert!(response.data.is_some());
    assert!(!response.metadata.is_empty());
}

/// Test critical path: Error response with details
#[test]
fn test_error_with_details() {
    use std::collections::HashMap;

    let mut details = HashMap::new();
    details.insert("field".to_string(), serde_json::json!("email"));

    let error = ErrorResponseFactory::validation_error("email", "invalid").with_details(details);

    assert!(error.details.is_some());
    assert_eq!(error.status, 400);
}

/// Test critical path: Multiple response types
#[test]
fn test_multiple_response_types() {
    let responses = vec![
        SuccessResponseFactory::created("user", "1"),
        SuccessResponseFactory::updated("user", "1"),
        SuccessResponseFactory::retrieved("users", 10),
        SuccessResponseFactory::deleted("user"),
    ];

    for response in responses {
        assert!(!response.message.is_empty());
        assert!(!response.timestamp.is_empty());
    }
}

/// Test critical path: All error types
#[test]
fn test_all_error_types() {
    let errors = [
        ErrorResponseFactory::bad_request("test"),
        ErrorResponseFactory::not_found("test"),
        ErrorResponseFactory::unauthorized("test"),
        ErrorResponseFactory::forbidden("test"),
        ErrorResponseFactory::conflict("test"),
        ErrorResponseFactory::service_unavailable("test"),
        ErrorResponseFactory::timeout("test"),
    ];

    let expected_statuses = [400, 404, 401, 403, 409, 503, 408];

    for (error, expected) in errors.iter().zip(expected_statuses.iter()) {
        assert_eq!(error.status, *expected);
    }
}

/// Test critical path: Response correlation IDs
#[test]
fn test_correlation_ids() {
    let response = SuccessResponseFactory::created("resource", "id")
        .with_correlation_id("corr-123".to_string());

    assert_eq!(response.correlation_id, Some("corr-123".to_string()));

    let error =
        ErrorResponseFactory::bad_request("test").with_correlation_id("corr-456".to_string());

    assert_eq!(error.correlation_id, Some("corr-456".to_string()));
}

/// Test critical path: Response metadata
#[test]
fn test_response_metadata() {
    let response = SuccessResponseFactory::created("resource", "id")
        .add_metadata("version", serde_json::json!("1.0"))
        .add_metadata("region", serde_json::json!("us-west"));

    assert_eq!(response.metadata.len(), 2);
    assert!(response.metadata.contains_key("version"));
    assert!(response.metadata.contains_key("region"));
}

/// Test critical path: Performance - fast operations
#[test]
fn test_performance() {
    use std::time::Instant;

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = SuccessResponseFactory::created("resource", "id");
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 100,
        "Response creation should be fast"
    );
}

/// Test critical path: Stress test - many responses
#[test]
fn stress_test_responses() {
    for i in 0..1000 {
        let _ = SuccessResponseFactory::created("resource", &format!("id-{}", i));
        let _ = ErrorResponseFactory::bad_request(&format!("error-{}", i));
    }
}

/// Test critical path: Health check responses
#[test]
fn test_health_check_responses() {
    let health = SuccessResponseFactory::health_check("api", "healthy");
    assert!(health.message.contains("healthy"));
    assert!(health.data.is_some());

    let health2 = SuccessResponseFactory::health_check("database", "degraded");
    assert!(health2.message.contains("healthy"));
}

/// Test critical path: Config update responses
#[test]
fn test_config_update_responses() {
    let response = SuccessResponseFactory::config_updated("security", 5);
    assert!(response.message.contains("configuration updated"));
    assert!(response.data.is_some());
}

/// Test critical path: Rate limit errors
#[test]
fn test_rate_limit_errors() {
    let rate_limit = ErrorResponseFactory::rate_limited(Some(60));
    assert_eq!(rate_limit.status, 429);
    assert!(rate_limit.details.is_some());

    let rate_limit_no_retry = ErrorResponseFactory::rate_limited(None);
    assert_eq!(rate_limit_no_retry.status, 429);
}

/// Test critical path: Processing responses
#[test]
fn test_processing_responses() {
    let started = SuccessResponseFactory::processing_started("job-123", "data_migration");
    assert!(started.message.contains("Processing started"));
    assert!(started.data.is_some());

    let completed = SuccessResponseFactory::processing_completed(
        "job-123",
        serde_json::json!({"status": "success"}),
    );
    assert!(completed.message.contains("Processing completed"));
}
