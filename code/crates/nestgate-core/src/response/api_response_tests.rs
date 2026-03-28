// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for ApiResponse
//! Tests all builders, conversions, and response scenarios

use super::api_response::*;
use axum::response::IntoResponse;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct TestPayload {
    id: u32,
    message: String,
}

#[test]
fn test_success_response_creation() {
    let payload = TestPayload {
        id: 1,
        message: "test".to_string(),
    };
    let response = ApiResponse::success(payload.clone());

    assert!(response.success);
    assert_eq!(response.data, Some(payload));
    assert!(response.error.is_none());
    assert!(response.error_code.is_none());
    assert!(response.metadata.is_some());
}

#[test]
fn test_success_with_metadata() {
    let payload = TestPayload {
        id: 1,
        message: "test".to_string(),
    };
    let mut metadata = HashMap::new();
    metadata.insert("key".to_string(), serde_json::json!("value"));

    let response = ApiResponse::success_with_metadata(payload.clone(), metadata.clone());

    assert!(response.success);
    assert_eq!(response.data, Some(payload));
    assert_eq!(response.metadata, Some(metadata));
}

#[test]
fn test_error_response_creation() {
    let response: ApiResponse<TestPayload> = ApiResponse::error("Something went wrong".to_string());

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Something went wrong".to_string()));
    assert!(response.error_code.is_none());
}

#[test]
fn test_error_with_code() {
    let response: ApiResponse<TestPayload> =
        ApiResponse::error_with_code("Database error".to_string(), "DB_001".to_string());

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Database error".to_string()));
    assert_eq!(response.error_code, Some("DB_001".to_string()));
    assert!(response.metadata.is_some());
}

#[test]
fn test_new_success() {
    let payload = TestPayload {
        id: 42,
        message: "direct".to_string(),
    };
    let response = ApiResponse::new_success(payload.clone());

    assert!(response.success);
    assert_eq!(response.data, Some(payload));
    assert!(response.error.is_none());
}

#[test]
fn test_new_error() {
    let response: ApiResponse<TestPayload> = ApiResponse::new_error("Error message".to_string());

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Error message".to_string()));
}

#[test]
fn test_new_error_with_code() {
    let response: ApiResponse<TestPayload> =
        ApiResponse::new_error_with_code("Validation failed".to_string(), "VAL_001".to_string());

    assert!(!response.success);
    assert_eq!(response.error, Some("Validation failed".to_string()));
    assert_eq!(response.error_code, Some("VAL_001".to_string()));
}

#[test]
fn test_with_metadata_builder() {
    let payload = TestPayload {
        id: 1,
        message: "test".to_string(),
    };
    let mut metadata = HashMap::new();
    metadata.insert("trace_id".to_string(), serde_json::json!("abc-123"));

    let response = ApiResponse::success(payload).with_metadata(metadata.clone());

    assert_eq!(response.metadata, Some(metadata));
}

#[test]
fn test_with_meta_builder() {
    let payload = TestPayload {
        id: 1,
        message: "test".to_string(),
    };
    let response = ApiResponse::success(payload)
        .with_meta("user_id", serde_json::json!(123))
        .with_meta("session", serde_json::json!("xyz"));

    let metadata = response.metadata.unwrap();
    assert_eq!(metadata.get("user_id"), Some(&serde_json::json!(123)));
    assert_eq!(metadata.get("session"), Some(&serde_json::json!("xyz")));
}

#[test]
fn test_is_success() {
    let success_response = ApiResponse::success(TestPayload {
        id: 1,
        message: "ok".to_string(),
    });
    let error_response: ApiResponse<TestPayload> = ApiResponse::error("fail".to_string());

    assert!(success_response.is_success());
    assert!(!error_response.is_success());
}

#[test]
fn test_is_error() {
    let success_response = ApiResponse::success(TestPayload {
        id: 1,
        message: "ok".to_string(),
    });
    let error_response: ApiResponse<TestPayload> = ApiResponse::error("fail".to_string());

    assert!(!success_response.is_error());
    assert!(error_response.is_error());
}

#[test]
fn test_error_message() {
    let error_response: ApiResponse<TestPayload> = ApiResponse::error("Custom error".to_string());

    assert_eq!(error_response.error_message(), Some("Custom error"));
}

#[test]
fn test_error_code() {
    let error_response: ApiResponse<TestPayload> =
        ApiResponse::error_with_code("Error".to_string(), "ERR_123".to_string());

    assert_eq!(error_response.error_code(), Some("ERR_123"));
}

#[test]
fn test_into_response_success() {
    let payload = TestPayload {
        id: 1,
        message: "success".to_string(),
    };
    let api_response = ApiResponse::success(payload);
    let response = api_response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[test]
fn test_into_response_error() {
    let api_response: ApiResponse<TestPayload> = ApiResponse::error("fail".to_string());
    let response = api_response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[test]
fn test_default_api_response() {
    let response: ApiResponse<TestPayload> = ApiResponse::default();

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Unknown error".to_string()));
}

#[test]
fn test_empty_response_success() {
    let response = EmptyResponse::success_empty();

    assert!(response.success);
    assert!(response.message.is_none());
}

#[test]
fn test_empty_response_success_message() {
    let response = EmptyResponse::success_message("Operation completed");

    assert!(response.success);
    assert_eq!(response.message, Some("Operation completed".to_string()));
}

#[test]
fn test_empty_response_error() {
    let response = EmptyResponse::error("Operation failed");

    assert!(!response.success);
    assert_eq!(response.message, Some("Operation failed".to_string()));
}

#[test]
fn test_empty_response_into_response_success() {
    let empty = EmptyResponse::success_empty();
    let response = empty.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[test]
fn test_empty_response_into_response_error() {
    let empty = EmptyResponse::error("fail");
    let response = empty.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[test]
fn test_empty_response_default() {
    let response = EmptyResponse::default();

    assert!(response.success);
    assert!(response.message.is_none());
}

#[test]
fn test_request_id_is_unique() {
    let r1 = ApiResponse::success(TestPayload {
        id: 1,
        message: "a".to_string(),
    });
    let r2 = ApiResponse::success(TestPayload {
        id: 2,
        message: "b".to_string(),
    });

    assert_ne!(r1.request_id, r2.request_id);
}

#[test]
fn test_timestamp_is_set() {
    let response = ApiResponse::success(TestPayload {
        id: 1,
        message: "test".to_string(),
    });

    // Timestamp should be recent (within last 5 seconds)
    let now = chrono::Utc::now();
    let diff = now.signed_duration_since(response.timestamp);
    assert!(diff.num_seconds() < 5);
}

#[test]
fn test_processing_time_initialization() {
    let response = ApiResponse::success(TestPayload {
        id: 1,
        message: "test".to_string(),
    });

    assert_eq!(response.processing_time_ms, 0);
}

#[test]
fn test_chaining_with_meta() {
    let response = ApiResponse::success(TestPayload {
        id: 1,
        message: "test".to_string(),
    })
    .with_meta("key1", serde_json::json!("value1"))
    .with_meta("key2", serde_json::json!(42))
    .with_meta("key3", serde_json::json!(true));

    let metadata = response.metadata.unwrap();
    assert_eq!(metadata.len(), 3);
    assert!(metadata.contains_key("key1"));
    assert!(metadata.contains_key("key2"));
    assert!(metadata.contains_key("key3"));
}

#[test]
fn test_to_json() {
    let payload = TestPayload {
        id: 1,
        message: "json_test".to_string(),
    };
    let response = ApiResponse::success(payload);
    let json = response.to_json();

    // Json wrapper should exist
    // Can't easily test the contents without deserializing, but we verify it compiles
    let _ = json;
}

#[test]
fn test_serialization() {
    let payload = TestPayload {
        id: 1,
        message: "serialize".to_string(),
    };
    let response = ApiResponse::success(payload);

    let serialized = serde_json::to_string(&response);
    assert!(serialized.is_ok());

    let json_str = serialized.unwrap();
    assert!(json_str.contains("\"success\":true"));
    assert!(json_str.contains("\"id\":1"));
}

#[test]
fn test_deserialization() {
    let json_str = r#"{
        "request_id": "test-123",
        "status": "Success",
        "success": true,
        "data": {"id": 42, "message": "hello"},
        "error": null,
        "error_code": null,
        "timestamp": "2025-12-05T00:00:00Z",
        "metadata": null,
        "processing_time_ms": 0
    }"#;

    let result: Result<ApiResponse<TestPayload>, _> = serde_json::from_str(json_str);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.success);
    assert_eq!(response.data.unwrap().id, 42);
}

#[test]
fn test_error_response_fields() {
    let response: ApiResponse<TestPayload> =
        ApiResponse::error_with_code("Not found".to_string(), "404".to_string());

    assert_eq!(response.error_message(), Some("Not found"));
    assert_eq!(response.error_code(), Some("404"));
    assert!(response.is_error());
    assert!(!response.is_success());
}

#[test]
fn test_complex_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert(
        "nested".to_string(),
        serde_json::json!({
            "key1": "value1",
            "key2": [1, 2, 3],
            "key3": {"inner": true}
        }),
    );

    let response = ApiResponse::success(TestPayload {
        id: 1,
        message: "complex".to_string(),
    })
    .with_metadata(metadata);

    let meta = response.metadata.unwrap();
    assert!(meta.contains_key("nested"));
}

#[test]
fn test_empty_response_serialization() {
    let response = EmptyResponse::success_message("All good");
    let serialized = serde_json::to_string(&response);

    assert!(serialized.is_ok());
    let json = serialized.unwrap();
    assert!(json.contains("\"success\":true"));
    assert!(json.contains("All good"));
}

#[test]
fn test_multiple_responses_independence() {
    let r1 = ApiResponse::success(TestPayload {
        id: 1,
        message: "a".to_string(),
    });
    let r2: ApiResponse<TestPayload> = ApiResponse::error("Error".to_string());
    let r3 = EmptyResponse::success_empty();

    assert!(r1.is_success());
    assert!(r2.is_error());
    assert!(r3.success);
}
