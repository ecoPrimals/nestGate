// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Comprehensive tests for error response module
/// Tests error creation, builders, factory methods, and serialization
#[cfg(test)]
mod error_response_tests {
    use super::super::*;
    use serde_json;
    use std::collections::HashMap;

    #[test]
    fn test_simple_error_creation() {
        let error = UnifiedErrorResponse::simple(
            "Database connection failed",
            "DB_CONNECTION_ERROR",
            "database",
        );

        assert_eq!(error.message, "Database connection failed");
        assert_eq!(error.code, "DB_CONNECTION_ERROR");
        assert_eq!(error.component, "database");
        assert_eq!(error.status, 500);
        assert!(error.details.is_none());
        assert!(error.correlation_id.is_none());
        assert!(!error.timestamp.is_empty());
    }

    #[test]
    fn test_error_with_custom_status() {
        let error =
            UnifiedErrorResponse::with_status("Resource not found", "NOT_FOUND", "api", 404);

        assert_eq!(error.status, 404);
        assert_eq!(error.message, "Resource not found");
        assert_eq!(error.code, "NOT_FOUND");
        assert_eq!(error.component, "api");
    }

    #[test]
    fn test_error_with_details() {
        let mut details = HashMap::new();
        details.insert("field".to_string(), serde_json::json!("username"));
        details.insert("reason".to_string(), serde_json::json!("too short"));

        let error = UnifiedErrorResponse::simple("Validation failed", "VALIDATION", "validator")
            .with_details(details);

        assert!(error.details.is_some());
        let error_details = error.details.unwrap();
        assert_eq!(
            error_details.get("field").unwrap(),
            &serde_json::json!("username")
        );
        assert_eq!(
            error_details.get("reason").unwrap(),
            &serde_json::json!("too short")
        );
    }

    #[test]
    fn test_error_with_correlation_id() {
        let error = UnifiedErrorResponse::simple("Test error", "TEST", "test")
            .with_correlation_id("req-12345".to_string());

        assert_eq!(error.correlation_id, Some("req-12345".to_string()));
    }

    #[test]
    fn test_error_with_context() {
        let error = UnifiedErrorResponse::simple("Test error", "TEST", "test")
            .with_context("user_id", serde_json::json!("user-123"))
            .with_context("operation", serde_json::json!("update"));

        assert!(error.details.is_some());
        let details = error.details.unwrap();
        assert_eq!(
            details.get("user_id").unwrap(),
            &serde_json::json!("user-123")
        );
        assert_eq!(
            details.get("operation").unwrap(),
            &serde_json::json!("update")
        );
    }

    #[test]
    fn test_error_with_context_chain() {
        let error = UnifiedErrorResponse::simple("Test error", "TEST", "test")
            .with_context_chain("step1", serde_json::json!("validated"))
            .with_context_chain("step2", serde_json::json!("processed"))
            .with_context_chain("step3", serde_json::json!("failed"));

        assert!(error.details.is_some());
        let details = error.details.unwrap();
        assert_eq!(details.len(), 3);
        assert_eq!(details.get("step3").unwrap(), &serde_json::json!("failed"));
    }

    #[test]
    fn test_error_builder_chaining() {
        let error = UnifiedErrorResponse::with_status("Error", "ERR", "component", 400)
            .with_correlation_id("req-789".to_string())
            .with_context("key1", serde_json::json!("value1"))
            .with_context("key2", serde_json::json!(42));

        assert_eq!(error.status, 400);
        assert_eq!(error.correlation_id, Some("req-789".to_string()));
        assert!(error.details.is_some());
        let details = error.details.unwrap();
        assert_eq!(details.len(), 2);
    }

    #[test]
    fn test_status_code_conversion() {
        assert_eq!(
            UnifiedErrorResponse::with_status("", "", "", 200).to_status_code(),
            StatusCode::OK
        );
        assert_eq!(
            UnifiedErrorResponse::with_status("", "", "", 400).to_status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            UnifiedErrorResponse::with_status("", "", "", 404).to_status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            UnifiedErrorResponse::with_status("", "", "", 500).to_status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_custom_status_codes() {
        // Custom status codes (like 999) are supported by axum and stored correctly
        let error = UnifiedErrorResponse::with_status("Error", "ERR", "test", 999);
        assert_eq!(error.status, 999);
        // Axum's StatusCode::from_u16() accepts any u16 value
        let status = error.to_status_code();
        assert_eq!(status.as_u16(), 999);
    }

    #[test]
    fn test_factory_bad_request() {
        let error = ErrorResponseFactory::bad_request("Invalid request format");
        assert_eq!(error.status, 400);
        assert_eq!(error.code, "BAD_REQUEST");
        assert_eq!(error.component, "nestgate-core");
        assert_eq!(error.message, "Invalid request format");
    }

    #[test]
    fn test_factory_internal_error() {
        let error = ErrorResponseFactory::internal("Database crashed");
        assert_eq!(error.status, 500);
        assert_eq!(error.code, "INTERNAL_ERROR");
        assert_eq!(error.message, "Database crashed");
    }

    #[test]
    fn test_factory_not_found() {
        let error = ErrorResponseFactory::not_found("/api/users/999");
        assert_eq!(error.status, 404);
        assert_eq!(error.code, "NOT_FOUND");
        assert!(error.message.contains("/api/users/999"));
        assert!(error.message.contains("not found"));
    }

    #[test]
    fn test_factory_unauthorized() {
        let error = ErrorResponseFactory::unauthorized("delete user");
        assert_eq!(error.status, 401);
        assert_eq!(error.code, "UNAUTHORIZED");
        assert!(error.message.contains("delete user"));
        assert!(error.message.contains("Unauthorized"));
    }

    #[test]
    fn test_factory_forbidden() {
        let error = ErrorResponseFactory::forbidden("admin panel");
        assert_eq!(error.status, 403);
        assert_eq!(error.code, "FORBIDDEN");
        assert!(error.message.contains("admin panel"));
        assert!(error.message.contains("forbidden"));
    }

    #[test]
    fn test_factory_validation_error() {
        let error = ErrorResponseFactory::validation_error("email", "invalid format");
        assert_eq!(error.status, 400);
        assert_eq!(error.code, "VALIDATION_ERROR");
        assert!(error.message.contains("email"));
        assert!(error.message.contains("invalid format"));
        assert!(error.details.is_some());

        let details = error.details.unwrap();
        assert_eq!(details.get("field").unwrap(), &serde_json::json!("email"));
    }

    #[test]
    fn test_factory_conflict() {
        let error = ErrorResponseFactory::conflict("user account");
        assert_eq!(error.status, 409);
        assert_eq!(error.code, "CONFLICT");
        assert!(error.message.contains("user account"));
    }

    #[test]
    fn test_factory_rate_limited_without_retry() {
        let error = ErrorResponseFactory::rate_limited(None);
        assert_eq!(error.status, 429);
        assert_eq!(error.code, "RATE_LIMITED");
        assert_eq!(error.message, "Rate limit exceeded");
    }

    #[test]
    fn test_factory_rate_limited_with_retry() {
        let error = ErrorResponseFactory::rate_limited(Some(60));
        assert_eq!(error.status, 429);
        assert!(error.details.is_some());

        let details = error.details.unwrap();
        assert_eq!(
            details.get("retry_after_seconds").unwrap(),
            &serde_json::json!(60)
        );
    }

    #[test]
    fn test_factory_service_unavailable() {
        let error = ErrorResponseFactory::service_unavailable("authentication");
        assert_eq!(error.status, 503);
        assert_eq!(error.code, "SERVICE_UNAVAILABLE");
        assert!(error.message.contains("authentication"));
        assert!(error.message.contains("unavailable"));
    }

    #[test]
    fn test_factory_timeout() {
        let error = ErrorResponseFactory::timeout("database query");
        assert_eq!(error.status, 408);
        assert_eq!(error.code, "TIMEOUT");
        assert!(error.message.contains("database query"));
        assert!(error.message.contains("timed out"));
    }

    #[test]
    fn test_legacy_conversion() {
        let unified =
            UnifiedErrorResponse::with_status("Test error", "TEST_CODE", "test-component", 400);
        let timestamp = unified.timestamp.clone();
        let legacy: LegacyErrorResponse = unified.into();

        assert_eq!(legacy.error, "Test error");
        assert_eq!(legacy.code, Some("TEST_CODE".to_string()));
        assert_eq!(legacy.timestamp, timestamp);
    }

    #[test]
    fn test_legacy_conversion_preserves_message() {
        let unified = UnifiedErrorResponse::simple(
            "Complex error message with details",
            "COMPLEX",
            "component",
        );
        let legacy: LegacyErrorResponse = unified.into();

        assert_eq!(legacy.error, "Complex error message with details");
        assert_eq!(legacy.code, Some("COMPLEX".to_string()));
    }

    #[test]
    fn test_error_response_serialization() {
        let error = UnifiedErrorResponse::simple("Test", "TEST_CODE", "test");
        let json = serde_json::to_string(&error).expect("Should serialize");

        assert!(json.contains("\"message\":\"Test\""));
        assert!(json.contains("\"code\":\"TEST_CODE\""));
        assert!(json.contains("\"component\":\"test\""));
        assert!(json.contains("\"status\":500"));
    }

    #[test]
    fn test_error_response_deserialization() {
        let json = r#"{
            "message": "Test error",
            "code": "TEST",
            "component": "test-component",
            "status": 404,
            "details": null,
            "timestamp": "2025-11-14T12:00:00Z",
            "correlation_id": null
        }"#;

        let error: UnifiedErrorResponse = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(error.message, "Test error");
        assert_eq!(error.code, "TEST");
        assert_eq!(error.status, 404);
    }

    #[test]
    fn test_error_with_complex_details() {
        let mut details = HashMap::new();
        details.insert("array".to_string(), serde_json::json!([1, 2, 3]));
        details.insert("object".to_string(), serde_json::json!({"key": "value"}));
        details.insert("number".to_string(), serde_json::json!(42));
        details.insert("boolean".to_string(), serde_json::json!(true));

        let error = UnifiedErrorResponse::simple("Test", "TEST", "test").with_details(details);

        assert!(error.details.is_some());
        let error_details = error.details.unwrap();
        assert_eq!(error_details.len(), 4);
    }

    #[test]
    fn test_error_clone() {
        let error1 = UnifiedErrorResponse::with_status("Error", "ERR", "comp", 400)
            .with_correlation_id("req-123".to_string());
        let error2 = error1.clone();

        assert_eq!(error1.message, error2.message);
        assert_eq!(error1.code, error2.code);
        assert_eq!(error1.status, error2.status);
        assert_eq!(error1.correlation_id, error2.correlation_id);
    }

    #[test]
    fn test_legacy_error_clone() {
        let legacy1 = LegacyErrorResponse {
            error: "Test".to_string(),
            code: Some("TEST".to_string()),
            timestamp: "2025-11-14T12:00:00Z".to_string(),
        };
        let legacy2 = legacy1.clone();

        assert_eq!(legacy1.error, legacy2.error);
        assert_eq!(legacy1.code, legacy2.code);
        assert_eq!(legacy1.timestamp, legacy2.timestamp);
    }

    #[test]
    fn test_all_factory_methods_have_correct_components() {
        assert_eq!(
            ErrorResponseFactory::bad_request("").component,
            "nestgate-core"
        );
        assert_eq!(
            ErrorResponseFactory::internal("").component,
            "nestgate-core"
        );
        assert_eq!(
            ErrorResponseFactory::not_found("").component,
            "nestgate-core"
        );
        assert_eq!(
            ErrorResponseFactory::unauthorized("").component,
            "nestgate-core"
        );
        assert_eq!(
            ErrorResponseFactory::forbidden("").component,
            "nestgate-core"
        );
        assert_eq!(
            ErrorResponseFactory::validation_error("", "").component,
            "nestgate-core"
        );
        assert_eq!(
            ErrorResponseFactory::conflict("").component,
            "nestgate-core"
        );
        assert_eq!(
            ErrorResponseFactory::rate_limited(None).component,
            "nestgate-core"
        );
        assert_eq!(
            ErrorResponseFactory::service_unavailable("").component,
            "nestgate-core"
        );
        assert_eq!(ErrorResponseFactory::timeout("").component, "nestgate-core");
    }

    #[test]
    fn test_error_response_debug_format() {
        let error = UnifiedErrorResponse::simple("Debug test", "DEBUG", "test");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("Debug test"));
        assert!(debug_str.contains("DEBUG"));
    }

    #[test]
    fn test_legacy_error_debug_format() {
        let legacy = LegacyErrorResponse {
            error: "Legacy debug test".to_string(),
            code: Some("LEGACY".to_string()),
            timestamp: "2025-11-14T12:00:00Z".to_string(),
        };
        let debug_str = format!("{:?}", legacy);
        assert!(debug_str.contains("Legacy debug test"));
    }
}
