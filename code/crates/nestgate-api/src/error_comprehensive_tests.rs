/// Comprehensive additional tests for error module
/// Targets edge cases, HTTP response paths, and error conversions
use super::*;
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[test]
fn test_error_response_with_details() {
    let error = ErrorResponse {
        error: "Validation failed".to_string(),
        code: "VALIDATION_ERROR".to_string(),
        details: Some(serde_json::json!({"field": "email", "reason": "invalid format"})),
        timestamp: chrono::Utc::now(),
    };

    assert_eq!(error.error, "Validation failed");
    assert!(error.details.is_some());
    let details = error.details.unwrap();
    assert_eq!(details["field"], "email");
}

#[test]
fn test_error_response_deserialization() {
    let json = r#"{
        "error": "Test error",
        "code": "TEST_CODE",
        "details": null,
        "timestamp": "2025-11-14T12:00:00Z"
    }"#;

    let error: ErrorResponse = serde_json::from_str(json).expect("Should deserialize");
    assert_eq!(error.error, "Test error");
    assert_eq!(error.code, "TEST_CODE");
}

#[test]
fn test_error_response_debug_format() {
    let error = ErrorResponse {
        error: "Debug test".to_string(),
        code: "DEBUG".to_string(),
        details: None,
        timestamp: chrono::Utc::now(),
    };

    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("Debug test"));
    assert!(debug_str.contains("DEBUG"));
}

#[test]
fn test_api_error_display_io() {
    let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
    let api_error = ApiError::Io(io_error);
    let display = format!("{api_error}");

    assert!(display.contains("I/O error"));
    assert!(display.contains("Access denied"));
}

#[test]
fn test_api_error_display_json() {
    let json_err = serde_json::from_str::<serde_json::Value>("{invalid}").unwrap_err();
    let api_error = ApiError::Json(json_err);
    let display = format!("{api_error}");

    assert!(display.contains("JSON error"));
}

#[test]
fn test_api_error_display_core() {
    let core_error = nestgate_core::error::NestGateError::Validation(Box::new(
        nestgate_core::error::ValidationErrorDetails {
            message: "Invalid input data".to_string(),
            field: None,
            expected: None,
            actual: None,
            context: None,
        },
    ));
    let api_error = ApiError::Core(core_error);
    let display = format!("{api_error}");

    assert!(display.contains("Core error"));
}

#[test]
fn test_api_error_debug_format() {
    let error = ApiError::InvalidRequest("Bad data".to_string());
    let debug_str = format!("{error:?}");

    assert!(debug_str.contains("InvalidRequest"));
    assert!(debug_str.contains("Bad data"));
}

#[test]
fn test_api_error_source_io() {
    use std::error::Error;
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "Not found");
    let api_error = ApiError::Io(io_error);

    assert!(api_error.source().is_some());
}

#[test]
fn test_api_error_source_json() {
    use std::error::Error;
    let json_err = serde_json::from_str::<serde_json::Value>("bad").unwrap_err();
    let api_error = ApiError::Json(json_err);

    assert!(api_error.source().is_some());
}

#[test]
fn test_api_error_source_core() {
    use std::error::Error;
    let core_error = nestgate_core::error::NestGateError::Validation(Box::new(
        nestgate_core::error::ValidationErrorDetails {
            message: "Test".to_string(),
            field: None,
            expected: None,
            actual: None,
            context: None,
        },
    ));
    let api_error = ApiError::Core(core_error);

    assert!(api_error.source().is_some());
}

#[test]
fn test_api_error_source_invalid_request() {
    use std::error::Error;
    let api_error = ApiError::InvalidRequest("Test".to_string());
    assert!(api_error.source().is_none());
}

#[test]
fn test_api_error_source_not_found() {
    use std::error::Error;
    let api_error = ApiError::NotFound("Test".to_string());
    assert!(api_error.source().is_none());
}

#[test]
fn test_api_error_source_internal() {
    use std::error::Error;
    let api_error = ApiError::Internal("Test".to_string());
    assert!(api_error.source().is_none());
}

#[test]
fn test_api_error_source_service_unavailable() {
    use std::error::Error;
    let api_error = ApiError::ServiceUnavailable("Test".to_string());
    assert!(api_error.source().is_none());
}

#[test]
fn test_api_error_into_response_invalid_request() {
    let error = ApiError::InvalidRequest("Missing field".to_string());
    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_api_error_into_response_not_found() {
    let error = ApiError::NotFound("/api/resource".to_string());
    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_api_error_into_response_internal() {
    let error = ApiError::Internal("Server error".to_string());
    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_api_error_into_response_service_unavailable() {
    let error = ApiError::ServiceUnavailable("Database down".to_string());
    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[test]
fn test_api_error_into_response_io() {
    let io_error = std::io::Error::other("IO failed");
    let error = ApiError::Io(io_error);
    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_api_error_into_response_json() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
    let error = ApiError::Json(json_err);
    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_api_error_into_response_core() {
    let core_error = nestgate_core::error::NestGateError::Validation(Box::new(
        nestgate_core::error::ValidationErrorDetails {
            message: "Core error".to_string(),
            field: None,
            expected: None,
            actual: None,
            context: None,
        },
    ));
    let error = ApiError::Core(core_error);
    let response = error.into_response();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_api_error_from_nestgate_error() {
    let core_error = nestgate_core::error::NestGateError::Internal(Box::new(
        nestgate_core::error::InternalErrorDetails {
            message: "test failed".to_string(),
            component: "test".to_string(),
            location: None,
            is_bug: false,
            context: None,
        },
    ));
    let api_error: ApiError = core_error.into();

    match api_error {
        ApiError::Core(_) => {} // Success
        _ => panic!("Expected ApiError::Core"),
    }
}

#[test]
fn test_api_error_from_io_different_kinds() {
    // Test different IO error kinds
    let errors = vec![
        std::io::ErrorKind::NotFound,
        std::io::ErrorKind::PermissionDenied,
        std::io::ErrorKind::ConnectionRefused,
        std::io::ErrorKind::TimedOut,
    ];

    for kind in errors {
        let io_error = std::io::Error::new(kind, "Test");
        let api_error: ApiError = io_error.into();

        match api_error {
            ApiError::Io(_) => {} // Success
            _ => panic!("Expected ApiError::Io"),
        }
    }
}

#[test]
fn test_api_error_from_json_syntax_error() {
    let json_err = serde_json::from_str::<serde_json::Value>("{bad json").unwrap_err();
    let api_error: ApiError = json_err.into();

    match api_error {
        ApiError::Json(_) => {} // Success
        _ => panic!("Expected ApiError::Json"),
    }
}

#[test]
fn test_error_response_timestamp_ordering() {
    let error1 = ErrorResponse {
        error: "First".to_string(),
        code: "FIRST".to_string(),
        details: None,
        timestamp: chrono::Utc::now(),
    };

    std::thread::sleep(std::time::Duration::from_millis(10));

    let error2 = ErrorResponse {
        error: "Second".to_string(),
        code: "SECOND".to_string(),
        details: None,
        timestamp: chrono::Utc::now(),
    };

    assert!(error2.timestamp > error1.timestamp);
}

#[test]
fn test_error_response_complex_details() {
    let details = serde_json::json!({
        "errors": [
            {"field": "email", "message": "invalid"},
            {"field": "password", "message": "too short"}
        ],
        "request_id": "req-123",
        "trace": ["step1", "step2", "step3"]
    });

    let error = ErrorResponse {
        error: "Multiple validation errors".to_string(),
        code: "VALIDATION_ERROR".to_string(),
        details: Some(details.clone()),
        timestamp: chrono::Utc::now(),
    };

    assert_eq!(error.details.unwrap(), details);
}

#[test]
fn test_api_error_invalid_request_empty_message() {
    let error = ApiError::InvalidRequest(String::new());
    let display = format!("{error}");
    assert_eq!(display, "Invalid request: ");
}

#[test]
fn test_api_error_not_found_with_path() {
    let error = ApiError::NotFound("/api/users/999".to_string());
    let display = format!("{error}");
    assert!(display.contains("/api/users/999"));
}

#[test]
fn test_api_error_internal_with_details() {
    let error = ApiError::Internal("Database connection pool exhausted".to_string());
    let display = format!("{error}");
    assert!(display.contains("Database connection pool exhausted"));
}

#[test]
fn test_api_error_service_unavailable_with_retry() {
    let error = ApiError::ServiceUnavailable("Retry after 60 seconds".to_string());
    let display = format!("{error}");
    assert!(display.contains("Retry after"));
}

#[test]
fn test_result_type_alias() {
    // Test that our Result type alias works correctly
    fn returns_result() -> Result<String> {
        Ok("success".to_string())
    }

    let result = returns_result();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[test]
fn test_result_with_api_error() {
    fn returns_error() -> Result<String> {
        Err(nestgate_core::error::NestGateError::Validation(Box::new(
            nestgate_core::error::ValidationErrorDetails {
                message: "Bad input".to_string(),
                field: None,
                expected: None,
                actual: None,
                context: None,
            },
        )))
    }

    let result = returns_error();
    assert!(result.is_err());
}

#[test]
fn test_error_response_serialization_round_trip() {
    let original = ErrorResponse {
        error: "Round trip test".to_string(),
        code: "TEST".to_string(),
        details: Some(serde_json::json!({"key": "value"})),
        timestamp: chrono::Utc::now(),
    };

    let json = serde_json::to_string(&original).expect("Should serialize");
    let deserialized: ErrorResponse = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(original.error, deserialized.error);
    assert_eq!(original.code, deserialized.code);
}

#[test]
fn test_api_error_chain_display() {
    // Test that we can display error chains properly
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let api_error = ApiError::Io(io_error);

    let display = format!("{api_error}");
    assert!(!display.is_empty());
    assert!(display.contains("error"));
}

#[test]
fn test_error_response_empty_code() {
    let error = ErrorResponse {
        error: "Test".to_string(),
        code: String::new(),
        details: None,
        timestamp: chrono::Utc::now(),
    };

    assert_eq!(error.code, "");
}

#[test]
fn test_error_response_very_long_message() {
    let long_message = "error ".repeat(1000);
    let error = ErrorResponse {
        error: long_message.clone(),
        code: "LONG_ERROR".to_string(),
        details: None,
        timestamp: chrono::Utc::now(),
    };

    assert_eq!(error.error.len(), long_message.len());
}
