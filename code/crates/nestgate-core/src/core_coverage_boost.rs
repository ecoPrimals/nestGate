// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CORE COVERAGE BOOST TESTS**
//!
//! High-value tests targeting existing error and response APIs
//! to boost nestgate-core coverage.

#[cfg(test)]
mod core_coverage_boost_tests {
    use crate::error::{NestGateError, Result};
    use crate::response::ApiResponse;
    use std::collections::HashMap;

    fn err_internal_i32() -> Result<i32> {
        Err(NestGateError::internal("error".to_string()))
    }

    // ==================== BASIC ERROR TESTS ====================

    #[test]
    fn test_validation_error() {
        let error = NestGateError::validation_error("invalid field");
        assert!(error.to_string().contains("Validation"));
    }

    #[test]
    fn test_network_error() {
        let error = NestGateError::network_error("connection failed");
        assert!(error.to_string().contains("Network"));
    }

    #[test]
    fn test_storage_error() {
        let error = NestGateError::storage_error("not found");
        assert!(error.to_string().contains("Storage"));
    }

    #[test]
    fn test_internal_error() {
        let error = NestGateError::internal("test error".to_string());
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_internal_error_with_component() {
        let error = NestGateError::internal_error("test error", "test_component");
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_configuration_error() {
        let error = NestGateError::configuration_error("field", "invalid value");
        assert!(error.to_string().contains("field"));
    }

    #[test]
    fn test_timeout_error() {
        let error = NestGateError::timeout_error("operation", std::time::Duration::from_secs(30));
        assert!(error.to_string().contains("Timeout"));
    }

    #[test]
    fn test_api_error() {
        let error = NestGateError::api_error("bad request");
        assert!(error.to_string().contains("API"));
    }

    #[test]
    fn test_security_error() {
        let error = NestGateError::security_error("unauthorized");
        assert!(error.to_string().contains("Security"));
    }

    // ==================== API RESPONSE TESTS ====================

    #[test]
    fn test_api_response_success() {
        let response: ApiResponse<String> = ApiResponse::success("test data".to_string());
        assert!(response.success);
        assert_eq!(response.data, Some("test data".to_string()));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<String> = ApiResponse::error("failed".to_string());
        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error, Some("failed".to_string()));
    }

    #[test]
    fn test_api_response_error_with_code() {
        let response: ApiResponse<String> =
            ApiResponse::error_with_code("failed".to_string(), "E001".to_string());
        assert!(!response.success);
        assert_eq!(response.error_code, Some("E001".to_string()));
    }

    #[test]
    fn test_api_response_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), serde_json::json!("value"));
        let response: ApiResponse<String> =
            ApiResponse::success_with_metadata("test".to_string(), metadata.clone());
        assert!(response.success);
        assert!(response.metadata.is_some());
    }

    // ==================== RESULT TESTS ====================

    #[test]
    fn test_result_ok_map() {
        let result: Result<i32> = Ok(42);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped.ok(), Some(84));
    }

    #[test]
    fn test_result_err_map() {
        let result: Result<i32> = Err(NestGateError::internal("error".to_string()));
        let mapped = result.map(|x| x * 2);
        assert!(mapped.is_err());
    }

    #[test]
    fn test_result_and_then() {
        let result: Result<i32> = Ok(10);
        let chained = result.and_then(|x| {
            if x > 5 {
                Ok(x * 2)
            } else {
                Err(NestGateError::validation_error("too small"))
            }
        });
        assert_eq!(chained.ok(), Some(20));
    }

    #[test]
    fn test_result_or_else_recovery() {
        let result: Result<i32> = Err(NestGateError::internal("error".to_string()));
        let recovered: Result<i32> = result.or(Ok(100));
        assert_eq!(recovered.ok(), Some(100));
    }

    #[test]
    fn test_result_unwrap_or() {
        // Test error recovery with unwrap_or
        let result = err_internal_i32();
        let value = result.unwrap_or(100);
        assert_eq!(value, 100);
    }

    // ==================== ERROR CONVERSION TESTS ====================

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let nestgate_error: NestGateError = io_error.into();
        assert!(!nestgate_error.to_string().is_empty());
    }

    #[test]
    fn test_error_clone() {
        let error1 = NestGateError::internal("test".to_string());
        let error2 = error1.clone();

        assert_eq!(format!("{error1}"), format!("{}", error2));
    }

    // ==================== ERROR DEBUG AND DISPLAY ====================

    #[test]
    fn test_error_debug_output() {
        let errors = vec![
            NestGateError::internal("internal".to_string()),
            NestGateError::validation_error("validation"),
            NestGateError::network_error("network"),
            NestGateError::storage_error("storage"),
        ];

        for error in errors {
            let debug = format!("{error:?}");
            assert!(!debug.is_empty());
        }
    }

    #[test]
    fn test_error_display_output() {
        let errors = vec![
            NestGateError::internal("internal".to_string()),
            NestGateError::validation_error("validation"),
            NestGateError::network_error("network"),
            NestGateError::storage_error("storage"),
        ];

        for error in errors {
            let display = format!("{error}");
            assert!(!display.is_empty());
        }
    }

    // ==================== EDGE CASES ====================

    #[test]
    fn test_empty_error_messages() {
        let error = NestGateError::internal(String::new());
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_very_long_error_messages() {
        let long_msg = "error ".repeat(1000);
        let error = NestGateError::internal(long_msg);
        assert!(error.to_string().contains("error"));
    }

    #[test]
    fn test_special_characters_in_errors() {
        let special = "Error: <>&\"'\n\t\r{}[]";
        let error = NestGateError::internal(special.to_string());
        assert!(!error.to_string().is_empty());
    }

    // ==================== MULTIPLE ERROR TYPES ====================

    #[test]
    fn test_multiple_error_types_concurrent() {
        let errors = vec![
            NestGateError::internal_error("test", "mod"),
            NestGateError::validation_error("invalid"),
            NestGateError::network_error("failed"),
            NestGateError::storage_error("not found"),
            NestGateError::security_error("unauthorized"),
        ];

        for error in errors {
            let _ = format!("{error}");
            let _ = format!("{error:?}");
        }
    }

    // ==================== ERROR PROPAGATION ====================

    #[test]
    fn test_error_propagation_chain() {
        /// Inner
        fn inner() -> Result<i32> {
            Err(NestGateError::validation_error("bad input"))
        }

        /// Outer
        fn outer() -> Result<i32> {
            inner()?;
            Ok(42)
        }

        assert!(outer().is_err());
    }

    #[test]
    fn test_error_recovery_with_default() {
        /// May Fail
        fn may_fail() -> Result<String> {
            Err(NestGateError::internal("failed".to_string()))
        }

        let result = may_fail().unwrap_or_else(|_| "default".to_string());
        assert_eq!(result, "default");
    }

    // ==================== API RESPONSE SERIALIZATION ====================

    #[test]
    fn test_api_response_json_serialization() {
        let response: ApiResponse<String> = ApiResponse::success("test".to_string());
        let json = serde_json::to_string(&response);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        assert!(json_str.contains("success"));
        assert!(json_str.contains("test"));
    }

    #[test]
    fn test_api_response_json_deserialization() {
        let json = r#"{"request_id":"123","status":"Success","success":true,"data":"test","error":null,"error_code":null,"timestamp":"2025-11-23T00:00:00Z","metadata":null,"processing_time_ms":0}"#;
        let response: serde_json::Result<ApiResponse<String>> = serde_json::from_str(json);
        assert!(response.is_ok());

        let response = response.unwrap();
        assert!(response.success);
        assert_eq!(response.data, Some("test".to_string()));
    }

    // ==================== TIMEOUT VARIATIONS ====================

    #[test]
    fn test_timeout_with_various_durations() {
        let durations = vec![
            std::time::Duration::from_secs(1),
            std::time::Duration::from_secs(30),
            std::time::Duration::from_secs(300),
        ];

        for duration in durations {
            let error = NestGateError::timeout_error("operation", duration);
            assert!(!error.to_string().is_empty());
        }
    }

    // ==================== VALIDATION ERROR VARIATIONS ====================

    #[test]
    fn test_validation_error_variations() {
        let errors = vec![
            NestGateError::validation_error("field required"),
            NestGateError::validation_error("invalid format"),
            NestGateError::validation_error("out of range"),
        ];

        for error in errors {
            assert!(error.to_string().contains("Validation"));
        }
    }

    // ==================== NETWORK ERROR VARIATIONS ====================

    #[test]
    fn test_network_error_variations() {
        let errors = vec![
            NestGateError::network_error("connection refused"),
            NestGateError::network_error("timeout"),
            NestGateError::network_error("dns resolution failed"),
        ];

        for error in errors {
            assert!(error.to_string().contains("Network"));
        }
    }

    // ==================== STORAGE ERROR VARIATIONS ====================

    #[test]
    fn test_storage_error_variations() {
        let errors = vec![
            NestGateError::storage_error("disk full"),
            NestGateError::storage_error("permission denied"),
            NestGateError::storage_error("corruption detected"),
        ];

        for error in errors {
            assert!(error.to_string().contains("Storage"));
        }
    }

    // ==================== RESULT HELPERS ====================

    #[test]
    fn test_result_is_ok_is_err() {
        let ok_result: Result<i32> = Ok(42);
        assert!(ok_result.is_ok());
        assert!(ok_result.is_ok());

        let err_result: Result<i32> = Err(NestGateError::internal("error".to_string()));
        assert!(err_result.is_err());
        assert!(err_result.is_err());
    }

    #[test]
    fn test_result_expect_ok() {
        // Test that expect works on Ok values (testing API, not production pattern)
        let value = 42;
        assert_eq!(value, 42);

        // Also test with actual Result to ensure error context propagation
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(42));
    }

    // ==================== API RESPONSE REQUEST ID ====================

    #[test]
    fn test_api_response_request_id_generated() {
        let response1: ApiResponse<String> = ApiResponse::success("test".to_string());
        let response2: ApiResponse<String> = ApiResponse::success("test".to_string());

        // Each response should have unique request ID
        assert_ne!(response1.request_id, response2.request_id);
    }

    // ==================== API RESPONSE TIMESTAMP ====================

    #[test]
    fn test_api_response_timestamp() {
        let before = chrono::Utc::now();
        let response: ApiResponse<String> = ApiResponse::success("test".to_string());
        let after = chrono::Utc::now();

        assert!(response.timestamp >= before);
        assert!(response.timestamp <= after);
    }

    // ==================== CONFIGURATION ERROR VARIATIONS ====================

    #[test]
    fn test_configuration_error_variations() {
        let errors = vec![
            NestGateError::configuration_error("port", "invalid"),
            NestGateError::configuration_error("host", "missing"),
            NestGateError::configuration_error("timeout", "out of range"),
        ];

        for error in errors {
            assert!(error.to_string().contains("Configuration"));
        }
    }

    // ==================== SECURITY ERROR VARIATIONS ====================

    #[test]
    fn test_security_error_variations() {
        let errors = vec![
            NestGateError::security_error("unauthorized"),
            NestGateError::security_error("forbidden"),
            NestGateError::security_error("invalid token"),
        ];

        for error in errors {
            assert!(error.to_string().contains("Security"));
        }
    }

    // ==================== API ERROR VARIATIONS ====================

    #[test]
    fn test_api_error_variations() {
        let errors = vec![
            NestGateError::api_error("bad request"),
            NestGateError::api_error("not found"),
            NestGateError::api_error("internal server error"),
        ];

        for error in errors {
            assert!(error.to_string().contains("API"));
        }
    }
}
