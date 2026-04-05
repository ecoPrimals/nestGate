// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **API ERROR UTILITIES**
//! API-specific error types and handling for the `NestGate` system.
// API-specific error handling utilities and convenience functions.

use std::borrow::Cow;

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a new API error
    pub fn api(message: impl Into<Cow<'static, str>>) -> Self {
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
    pub fn api_with_status(message: impl Into<Cow<'static, str>>, status_code: u16) -> Self {
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
        message: impl Into<Cow<'static, str>>,
        status_code: Option<u16>,
        request_id: Option<Cow<'static, str>>,
        endpoint: Option<Cow<'static, str>>,
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
    pub fn service_unavailable(message: impl Into<Cow<'static, str>>) -> Self {
        let msg = message.into();
        Self::Api(Box::new(
            crate::error::variants::core_errors::ApiErrorDetails {
                message: format!("Service unavailable: {msg}").into(),
                status_code: Some(503),
                request_id: None,
                endpoint: None,
                context: None,
            },
        ))
    }

    /// Create a not found error
    pub fn not_found(message: impl Into<Cow<'static, str>>) -> Self {
        let msg = message.into();
        Self::Api(Box::new(
            crate::error::variants::core_errors::ApiErrorDetails {
                message: format!("Not found: {msg}").into(),
                status_code: Some(404),
                request_id: None,
                endpoint: None,
                context: None,
            },
        ))
    }

    /// Create an invalid input error with field
    pub fn invalid_input_with_field(
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        let msg = message.into();
        Self::Validation(Box::new(
            crate::error::variants::core_errors::ValidationErrorDetails {
                message: format!("Invalid input: {msg}").into(),
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
    /// use nestgate_types::error::NestGateError;
    /// let error = NestGateError::config("Invalid configuration");
    /// ```
    pub fn config(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Configuration(Box::new(
            crate::error::variants::core_errors::ConfigurationErrorDetails {
                field: Cow::Borrowed(""),
                message: message.into(),
                currentvalue: None,
                expected: None,
                user_error: false,
            },
        ))
    }

    /// Create a configuration error with field
    pub fn config_with_field(
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
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
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message.as_ref(), "Test API error");
            assert_eq!(details.status_code, None);
            assert_eq!(details.request_id, None);
            assert_eq!(details.endpoint, None);
            assert!(details.context.is_none());
        }
    }

    #[test]
    fn test_api_error_with_string() {
        let error = NestGateUnifiedError::api("API failure".to_string());
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message, "API failure");
        }
    }

    #[test]
    fn test_api_with_status() {
        let error = NestGateUnifiedError::api_with_status("Bad request", 400);
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message.as_ref(), "Bad request");
            assert_eq!(details.status_code, Some(400));
            assert_eq!(details.request_id, None);
            assert_eq!(details.endpoint, None);
            assert!(details.context.is_none());
        }
    }

    #[test]
    fn test_api_with_status_string_message() {
        let error = NestGateUnifiedError::api_with_status("Internal error".to_string(), 500);
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message.as_ref(), "Internal error");
            assert_eq!(details.status_code, Some(500));
        }
    }

    #[test]
    fn test_api_with_context_full() {
        let error = NestGateUnifiedError::api_with_context(
            "Context error",
            Some(422),
            Some("req-123".into()),
            Some("/api/v1/test".into()),
        );
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message.as_ref(), "Context error");
            assert_eq!(details.status_code, Some(422));
            assert_eq!(
                details.request_id.as_ref().map(std::borrow::Cow::as_ref),
                Some("req-123")
            );
            assert_eq!(
                details.endpoint.as_ref().map(std::borrow::Cow::as_ref),
                Some("/api/v1/test")
            );
            assert!(details.context.is_none());
        }
    }

    #[test]
    fn test_api_with_context_partial() {
        let error = NestGateUnifiedError::api_with_context(
            "Partial context",
            Some(401),
            None,
            Some("/api/auth".into()),
        );
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message.as_ref(), "Partial context");
            assert_eq!(details.status_code, Some(401));
            assert_eq!(details.request_id, None);
            assert_eq!(
                details.endpoint.as_ref().map(std::borrow::Cow::as_ref),
                Some("/api/auth")
            );
        }
    }

    #[test]
    fn test_api_with_context_minimal() {
        let error = NestGateUnifiedError::api_with_context("Minimal context", None, None, None);
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message.as_ref(), "Minimal context");
            assert_eq!(details.status_code, None);
            assert_eq!(details.request_id, None);
            assert_eq!(details.endpoint, None);
        }
    }

    #[test]
    fn test_service_unavailable() {
        let error = NestGateUnifiedError::service_unavailable("Database down");
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(
                details.message.as_ref(),
                "Service unavailable: Database down"
            );
            assert_eq!(details.status_code, Some(503));
            assert_eq!(details.request_id, None);
            assert_eq!(details.endpoint, None);
            assert!(details.context.is_none());
        }
    }

    #[test]
    fn test_service_unavailable_string() {
        let error = NestGateUnifiedError::service_unavailable("Cache unavailable".to_string());
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(
                details.message.as_ref(),
                "Service unavailable: Cache unavailable"
            );
            assert_eq!(details.status_code, Some(503));
        }
    }

    #[test]
    fn test_not_found() {
        let error = NestGateUnifiedError::not_found("Resource missing");
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message.as_ref(), "Not found: Resource missing");
            assert_eq!(details.status_code, Some(404));
            assert_eq!(details.request_id, None);
            assert_eq!(details.endpoint, None);
            assert!(details.context.is_none());
        }
    }

    #[test]
    fn test_not_found_string() {
        let error = NestGateUnifiedError::not_found("User not found".to_string());
        assert!(matches!(&error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = error {
            assert_eq!(details.message.as_ref(), "Not found: User not found");
            assert_eq!(details.status_code, Some(404));
        }
    }

    #[test]
    fn test_invalid_input_with_field() {
        let error = NestGateUnifiedError::invalid_input_with_field("email", "Invalid format");
        assert!(matches!(&error, NestGateUnifiedError::Validation(_)));
        if let NestGateUnifiedError::Validation(details) = error {
            assert_eq!(details.message.as_ref(), "Invalid input: Invalid format");
            assert_eq!(
                details.field.as_ref().map(std::borrow::Cow::as_ref),
                Some("email")
            );
            assert_eq!(details.expected, None);
            assert_eq!(details.actual, None);
            assert!(details.context.is_none());
        }
    }

    #[test]
    fn test_invalid_input_with_field_strings() {
        let error = NestGateUnifiedError::invalid_input_with_field(
            "password".to_string(),
            "Too short".to_string(),
        );
        assert!(matches!(&error, NestGateUnifiedError::Validation(_)));
        if let NestGateUnifiedError::Validation(details) = error {
            assert_eq!(details.message.as_ref(), "Invalid input: Too short");
            assert_eq!(
                details.field.as_ref().map(std::borrow::Cow::as_ref),
                Some("password")
            );
        }
    }

    #[test]
    fn test_error_variant_distinction() {
        let api_error = NestGateUnifiedError::api("API error");
        let validation_error = NestGateUnifiedError::invalid_input_with_field("field", "Invalid");

        assert!(matches!(&api_error, NestGateUnifiedError::Api(_)));
        assert!(matches!(
            &validation_error,
            NestGateUnifiedError::Validation(_)
        ));
    }

    #[test]
    fn test_config_constructors() {
        let e = NestGateUnifiedError::config("bad");
        assert!(matches!(e, NestGateUnifiedError::Configuration(_)));
        let e2 = NestGateUnifiedError::config_with_field("port", "invalid");
        assert!(matches!(e2, NestGateUnifiedError::Configuration(_)));
        if let NestGateUnifiedError::Configuration(d) = e2 {
            assert_eq!(d.field.as_ref(), "port");
        }
    }

    #[test]
    fn test_status_codes() {
        let service_error = NestGateUnifiedError::service_unavailable("test");
        let not_found_error = NestGateUnifiedError::not_found("test");
        let custom_error = NestGateUnifiedError::api_with_status("test", 418);

        assert!(matches!(&service_error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = service_error {
            assert_eq!(details.status_code, Some(503));
        }

        assert!(matches!(&not_found_error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = not_found_error {
            assert_eq!(details.status_code, Some(404));
        }

        assert!(matches!(&custom_error, NestGateUnifiedError::Api(_)));
        if let NestGateUnifiedError::Api(details) = custom_error {
            assert_eq!(details.status_code, Some(418));
        }
    }
}
