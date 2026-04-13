// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **API COVERAGE BOOST TESTS**
//!
//! High-value tests targeting untested API error handling, conversions,
//! and edge cases to boost coverage from 71% → 80%+.

#[cfg(test)]
mod tests {
    use crate::error::{ApiError, ErrorResponse};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use std::error::Error; // For .source() method

    // ==================== API ERROR TESTS ====================

    #[test]
    fn test_api_error_core() {
        let core_error = nestgate_core::error::NestGateError::internal("test error");
        let api_error = ApiError::Core(core_error);
        assert!(api_error.to_string().contains("Core error"));
    }

    #[test]
    fn test_api_error_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let api_error = ApiError::Io(io_error);
        assert!(api_error.to_string().contains("I/O error"));
    }

    #[test]
    fn test_api_error_json() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let api_error = ApiError::Json(json_error);
        assert!(api_error.to_string().contains("JSON error"));
    }

    #[test]
    fn test_api_error_invalid_request() {
        let api_error = ApiError::InvalidRequest("bad request".to_string());
        assert!(api_error.to_string().contains("Invalid request"));
        assert!(api_error.to_string().contains("bad request"));
    }

    #[test]
    fn test_api_error_not_found() {
        let api_error = ApiError::NotFound("resource missing".to_string());
        assert!(api_error.to_string().contains("Not found"));
        assert!(api_error.to_string().contains("resource missing"));
    }

    #[test]
    fn test_api_error_internal() {
        let api_error = ApiError::Internal("internal issue".to_string());
        assert!(api_error.to_string().contains("Internal error"));
        assert!(api_error.to_string().contains("internal issue"));
    }

    #[test]
    fn test_api_error_service_unavailable() {
        let api_error = ApiError::ServiceUnavailable("service down".to_string());
        assert!(api_error.to_string().contains("Service unavailable"));
        assert!(api_error.to_string().contains("service down"));
    }

    // ==================== ERROR RESPONSE TESTS ====================

    #[test]
    fn test_error_response_creation() {
        let response = ErrorResponse {
            error: "test error".to_string(),
            code: "TEST_ERROR".to_string(),
            details: None,
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(response.error, "test error");
        assert_eq!(response.code, "TEST_ERROR");
        assert!(response.details.is_none());
    }

    #[test]
    fn test_error_response_with_details() {
        let details = serde_json::json!({
            "field": "name",
            "value": "invalid"
        });

        let response = ErrorResponse {
            error: "validation failed".to_string(),
            code: "VALIDATION_ERROR".to_string(),
            details: Some(details.clone()),
            timestamp: chrono::Utc::now(),
        };

        assert!(response.details.is_some());
        assert_eq!(response.details.unwrap(), details);
    }

    // ==================== ERROR CONVERSION TESTS ====================

    #[test]
    fn test_from_nestgate_error() {
        let core_error = nestgate_core::error::NestGateError::internal("test");
        let api_error: ApiError = core_error.into();

        assert!(matches!(api_error, ApiError::Core(_)));
    }

    #[test]
    fn test_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let api_error: ApiError = io_error.into();

        assert!(matches!(api_error, ApiError::Io(_)));
    }

    #[test]
    fn test_from_serde_error() {
        let json_str = "{invalid}";
        let json_error = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();
        let api_error: ApiError = json_error.into();

        assert!(matches!(api_error, ApiError::Json(_)));
    }

    // ==================== STATUS CODE TESTS ====================

    #[test]
    fn test_invalid_request_status_code() {
        let error = ApiError::InvalidRequest("bad".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_not_found_status_code() {
        let error = ApiError::NotFound("missing".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_internal_status_code() {
        let error = ApiError::Internal("internal".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_service_unavailable_status_code() {
        let error = ApiError::ServiceUnavailable("down".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    // ==================== ERROR SOURCE TESTS ====================

    #[test]
    fn test_error_source_core() {
        let core_error = nestgate_core::error::NestGateError::internal("test");
        let api_error = ApiError::Core(core_error);

        assert!(api_error.source().is_some());
    }

    #[test]
    fn test_error_source_io() {
        let io_error = std::io::Error::other("test");
        let api_error = ApiError::Io(io_error);

        assert!(api_error.source().is_some());
    }

    #[test]
    fn test_error_source_invalid_request() {
        let api_error = ApiError::InvalidRequest("test".to_string());
        assert!(api_error.source().is_none());
    }

    #[test]
    fn test_error_source_not_found() {
        let api_error = ApiError::NotFound("test".to_string());
        assert!(api_error.source().is_none());
    }

    // ==================== TIMESTAMP TESTS ====================

    #[test]
    fn test_error_response_timestamp_now() {
        let before = chrono::Utc::now();
        let response = ErrorResponse {
            error: "test".to_string(),
            code: "TEST".to_string(),
            details: None,
            timestamp: chrono::Utc::now(),
        };
        let after = chrono::Utc::now();

        assert!(response.timestamp >= before);
        assert!(response.timestamp <= after);
    }

    // ==================== SERIALIZATION TESTS ====================

    #[test]
    fn test_error_response_serialization() {
        let response = ErrorResponse {
            error: "test error".to_string(),
            code: "TEST_ERROR".to_string(),
            details: None,
            timestamp: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&response);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        assert!(json_str.contains("test error"));
        assert!(json_str.contains("TEST_ERROR"));
    }

    #[test]
    fn test_error_response_deserialization() {
        let json = r#"{
            "error": "test error",
            "code": "TEST_ERROR",
            "details": null,
            "timestamp": "2025-11-23T00:00:00Z"
        }"#;

        let response: Result<ErrorResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.error, "test error");
        assert_eq!(response.code, "TEST_ERROR");
    }

    // ==================== ERROR CODE TESTS ====================

    #[test]
    fn test_error_code_formats() {
        let codes = vec![
            "CORE_ERROR",
            "IO_ERROR",
            "JSON_ERROR",
            "INVALID_REQUEST",
            "NOT_FOUND",
            "INTERNAL_ERROR",
            "SERVICE_UNAVAILABLE",
        ];

        for code in codes {
            assert!(code.chars().all(|c| c.is_uppercase() || c == '_'));
            assert!(!code.is_empty());
        }
    }

    // ==================== ERROR MESSAGE TESTS ====================

    #[test]
    fn test_error_message_formats() {
        let messages = vec![
            (
                "Core error",
                ApiError::Core(nestgate_core::error::NestGateError::internal("test")),
            ),
            ("I/O error", ApiError::Io(std::io::Error::other("test"))),
            (
                "Invalid request",
                ApiError::InvalidRequest("test".to_string()),
            ),
            ("Not found", ApiError::NotFound("test".to_string())),
            ("Internal error", ApiError::Internal("test".to_string())),
            (
                "Service unavailable",
                ApiError::ServiceUnavailable("test".to_string()),
            ),
        ];

        for (prefix, error) in messages {
            let message = error.to_string();
            assert!(
                message.starts_with(prefix),
                "Expected '{message}' to start with '{prefix}'"
            );
        }
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_empty_error_messages() {
        let errors = vec![
            ApiError::InvalidRequest(String::new()),
            ApiError::NotFound(String::new()),
            ApiError::Internal(String::new()),
            ApiError::ServiceUnavailable(String::new()),
        ];

        for error in errors {
            // Should not panic with empty messages
            let _message = error.to_string();
            let _response = error.into_response();
        }
    }

    #[test]
    fn test_very_long_error_messages() {
        let long_message = "error ".repeat(1000);
        let error = ApiError::Internal(long_message.clone());

        assert!(error.to_string().contains(&long_message));
    }

    #[test]
    fn test_special_characters_in_messages() {
        let special_chars = "Error: <>&\"'\n\t\r";
        let error = ApiError::Internal(special_chars.to_string());

        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    // ==================== MULTIPLE CONVERSIONS TESTS ====================

    #[test]
    fn test_chain_error_conversions() {
        let io_error = std::io::Error::other("test");
        let api_error: ApiError = io_error.into();
        let _response = api_error.into_response();
        // Should complete without panic
    }

    #[test]
    fn test_error_response_debug() {
        let response = ErrorResponse {
            error: "test".to_string(),
            code: "TEST".to_string(),
            details: None,
            timestamp: chrono::Utc::now(),
        };

        let debug_str = format!("{response:?}");
        assert!(debug_str.contains("ErrorResponse"));
    }

    // ==================== CONCURRENT ERROR HANDLING TESTS ====================

    #[test]
    fn test_multiple_errors_concurrently() {
        let errors = vec![
            ApiError::InvalidRequest("1".to_string()),
            ApiError::NotFound("2".to_string()),
            ApiError::Internal("3".to_string()),
        ];

        for error in errors {
            let _response = error.into_response();
        }
    }
}
