// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **API ERROR UTILITIES**
//! API-specific error types and handling for the `NestGate` system.
// API-specific error handling utilities and convenience functions.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a new API error
    pub fn api(message: impl Into<String>) -> Self {
        Self::Api(Box::new(
            crate::error::variants::core_errors::ApiErrorDetails {
                message: message.into(),
                status_code: None,
                request_id: None,
                endpoint: None,
                context: None,
            },
        ))
    }

    /// Create an API error with status code
    pub fn api_with_status(message: impl Into<String>, status_code: u16) -> Self {
        Self::Api(Box::new(
            crate::error::variants::core_errors::ApiErrorDetails {
                message: message.into(),
                status_code: Some(status_code),
                request_id: None,
                endpoint: None,
                context: None,
            },
        ))
    }

    /// Create an API error with full context
    pub fn api_with_context(
        message: impl Into<String>,
        status_code: Option<u16>,
        request_id: Option<String>,
        endpoint: Option<String>,
    ) -> Self {
        Self::Api(Box::new(
            crate::error::variants::core_errors::ApiErrorDetails {
                message: message.into(),
                status_code,
                request_id,
                endpoint,
                context: None,
            },
        ))
    }

    /// Create a service unavailable error
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::Api(Box::new(
            crate::error::variants::core_errors::ApiErrorDetails {
                message: format!("Service unavailable: {}", message.into()),
                status_code: Some(503),
                request_id: None,
                endpoint: None,
                context: None,
            },
        ))
    }

    /// Create a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::Api(Box::new(
            crate::error::variants::core_errors::ApiErrorDetails {
                message: format!("Not found: {}", message.into()),
                status_code: Some(404),
                request_id: None,
                endpoint: None,
                context: None,
            },
        ))
    }

    /// Create an invalid input error with field
    pub fn invalid_input_with_field(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation(Box::new(
            crate::error::variants::core_errors::ValidationErrorDetails {
                message: format!("Invalid input: {}", message.into()),
                field: Some(field.into()),
                expected: None,
                actual: None,
                context: None,
            },
        ))
    }

    /// Create a configuration error (convenience constructor)
    ///
    /// # Example
    /// ```
    /// use nestgate_core::error::NestGateError;
    /// let error = NestGateError::config("Invalid configuration");
    /// ```
    pub fn config(message: impl Into<String>) -> Self {
        Self::Configuration(Box::new(
            crate::error::variants::core_errors::ConfigurationErrorDetails {
                field: String::new(),
                message: message.into(),
                currentvalue: None,
                expected: None,
                user_error: false,
            },
        ))
    }

    /// Create a configuration error with field
    pub fn config_with_field(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Configuration(Box::new(
            crate::error::variants::core_errors::ConfigurationErrorDetails {
                field: field.into(),
                message: message.into(),
                currentvalue: None,
                expected: None,
                user_error: false,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_basic() {
        let error = NestGateUnifiedError::api("Test API error");

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Test API error");
                assert_eq!(details.status_code, None);
                assert_eq!(details.request_id, None);
                assert_eq!(details.endpoint, None);
                assert!(details.context.is_none());
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_api_error_with_string() {
        let error = NestGateUnifiedError::api("API failure".to_string());

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "API failure");
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_api_with_status() {
        let error = NestGateUnifiedError::api_with_status("Bad request", 400);

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Bad request");
                assert_eq!(details.status_code, Some(400));
                assert_eq!(details.request_id, None);
                assert_eq!(details.endpoint, None);
                assert!(details.context.is_none());
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_api_with_status_string_message() {
        let error = NestGateUnifiedError::api_with_status("Internal error".to_string(), 500);

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Internal error");
                assert_eq!(details.status_code, Some(500));
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_api_with_context_full() {
        let error = NestGateUnifiedError::api_with_context(
            "Context error",
            Some(422),
            Some("req-123".to_string()),
            Some("/api/v1/test".to_string()),
        );

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Context error");
                assert_eq!(details.status_code, Some(422));
                assert_eq!(details.request_id, Some("req-123".to_string()));
                assert_eq!(details.endpoint, Some("/api/v1/test".to_string()));
                assert!(details.context.is_none());
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_api_with_context_partial() {
        let error = NestGateUnifiedError::api_with_context(
            "Partial context",
            Some(401),
            None,
            Some("/api/auth".to_string()),
        );

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Partial context");
                assert_eq!(details.status_code, Some(401));
                assert_eq!(details.request_id, None);
                assert_eq!(details.endpoint, Some("/api/auth".to_string()));
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_api_with_context_minimal() {
        let error = NestGateUnifiedError::api_with_context("Minimal context", None, None, None);

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Minimal context");
                assert_eq!(details.status_code, None);
                assert_eq!(details.request_id, None);
                assert_eq!(details.endpoint, None);
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_service_unavailable() {
        let error = NestGateUnifiedError::service_unavailable("Database down");

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Service unavailable: Database down");
                assert_eq!(details.status_code, Some(503));
                assert_eq!(details.request_id, None);
                assert_eq!(details.endpoint, None);
                assert!(details.context.is_none());
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_service_unavailable_string() {
        let error = NestGateUnifiedError::service_unavailable("Cache unavailable".to_string());

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Service unavailable: Cache unavailable");
                assert_eq!(details.status_code, Some(503));
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_not_found() {
        let error = NestGateUnifiedError::not_found("Resource missing");

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Not found: Resource missing");
                assert_eq!(details.status_code, Some(404));
                assert_eq!(details.request_id, None);
                assert_eq!(details.endpoint, None);
                assert!(details.context.is_none());
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_not_found_string() {
        let error = NestGateUnifiedError::not_found("User not found".to_string());

        match error {
            NestGateUnifiedError::Api(details) => {
                assert_eq!(details.message, "Not found: User not found");
                assert_eq!(details.status_code, Some(404));
            }
            _ => panic!("Expected Api error variant"),
        }
    }

    #[test]
    fn test_invalid_input_with_field() {
        let error = NestGateUnifiedError::invalid_input_with_field("email", "Invalid format");

        match error {
            NestGateUnifiedError::Validation(details) => {
                assert_eq!(details.message, "Invalid input: Invalid format");
                assert_eq!(details.field, Some("email".to_string()));
                assert_eq!(details.expected, None);
                assert_eq!(details.actual, None);
                assert!(details.context.is_none());
            }
            _ => panic!("Expected Validation error variant"),
        }
    }

    #[test]
    fn test_invalid_input_with_field_strings() {
        let error = NestGateUnifiedError::invalid_input_with_field(
            "password".to_string(),
            "Too short".to_string(),
        );

        match error {
            NestGateUnifiedError::Validation(details) => {
                assert_eq!(details.message, "Invalid input: Too short");
                assert_eq!(details.field, Some("password".to_string()));
            }
            _ => panic!("Expected Validation error variant"),
        }
    }

    #[test]
    fn test_error_variant_distinction() {
        let api_error = NestGateUnifiedError::api("API error");
        let validation_error = NestGateUnifiedError::invalid_input_with_field("field", "Invalid");

        // Ensure different error types are created
        match (api_error, validation_error) {
            (NestGateUnifiedError::Api(_), NestGateUnifiedError::Validation(_)) => {
                // Expected - different variants created
            }
            _ => panic!("Expected different error variants"),
        }
    }

    #[test]
    fn test_status_codes() {
        let service_error = NestGateUnifiedError::service_unavailable("test");
        let not_found_error = NestGateUnifiedError::not_found("test");
        let custom_error = NestGateUnifiedError::api_with_status("test", 418);

        match service_error {
            NestGateUnifiedError::Api(details) => assert_eq!(details.status_code, Some(503)),
            _ => panic!("Expected Api error"),
        }

        match not_found_error {
            NestGateUnifiedError::Api(details) => assert_eq!(details.status_code, Some(404)),
            _ => panic!("Expected Api error"),
        }

        match custom_error {
            NestGateUnifiedError::Api(details) => assert_eq!(details.status_code, Some(418)),
            _ => panic!("Expected Api error"),
        }
    }
}
