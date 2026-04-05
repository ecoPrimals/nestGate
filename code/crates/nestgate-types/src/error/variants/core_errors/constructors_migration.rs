// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Ergonomic migration helpers and domain-specific factories for [`NestGateUnifiedError`](super::unified_enum::NestGateUnifiedError).

use std::borrow::Cow;
use std::time::Duration;

use super::details::{
    ApiErrorDetails, ConfigurationErrorDetails, InternalErrorDetails, NetworkErrorDetails,
    NotImplementedErrorDetails, ResourceExhaustedErrorDetails, SecurityErrorDetails,
    StorageErrorDetails, TimeoutErrorDetails, ValidationErrorDetails,
};
use super::unified_enum::NestGateUnifiedError;

impl NestGateUnifiedError {
    // ==================== ERGONOMIC MIGRATION HELPERS ====================
    // These helpers make migration from domain_errors.rs patterns easier
    // They use impl Into<String> for maximum ergonomics

    /// Create a network connection failure error - migration helper
    ///
    /// Replaces: `NetworkError::ConnectionFailed { address, port, error, timeout }`
    #[must_use]
    pub fn network_connection_failed(
        address: impl Into<Cow<'static, str>>,
        port: u16,
        reason: impl Into<Cow<'static, str>>,
    ) -> Self {
        let address = address.into();
        let reason = reason.into();
        Self::Network(Box::new(NetworkErrorDetails {
            message: format!("Connection failed: {address}:{port} - {reason}").into(),
            endpoint: Some(format!("{address}:{port}").into()),
            operation: Some(Cow::Borrowed("connect")),
            network_data: None,
            context: None,
        }))
    }

    /// Create a network timeout error - migration helper
    ///
    /// Replaces: `NetworkError::Timeout { url, timeout, method }`
    #[must_use]
    pub fn network_timeout(url: impl Into<Cow<'static, str>>, duration: Duration) -> Self {
        let url = url.into();
        Self::Timeout(Box::new(TimeoutErrorDetails {
            message: format!("Request timeout: {url} after {duration:?}").into(),
            operation: Some(Cow::Borrowed("network_request")),
            timeout: duration,
            retryable: true,
            context: None,
        }))
    }

    /// Create a storage file not found error - migration helper
    ///
    /// Replaces: `StorageError::FileNotFound { path, operation }`
    #[must_use]
    pub fn storage_not_found(path: impl Into<Cow<'static, str>>) -> Self {
        let path = path.into();
        Self::Storage(Box::new(StorageErrorDetails {
            message: format!("File not found: {path}").into(),
            resource: Some(path.clone()),
            operation: Some(Cow::Borrowed("read")),
            storage_data: None,
            context: None,
        }))
    }

    /// Create a storage permission denied error - migration helper
    ///
    /// Replaces: `StorageError::PermissionDenied { path, operation, required_permissions }`
    #[must_use]
    pub fn storage_permission_denied(
        path: impl Into<Cow<'static, str>>,
        operation: impl Into<Cow<'static, str>>,
    ) -> Self {
        let path = path.into();
        let operation = operation.into();
        Self::Storage(Box::new(StorageErrorDetails {
            message: format!("Permission denied: {path} for operation '{operation}'").into(),
            resource: Some(path),
            operation: Some(operation),
            storage_data: None,
            context: None,
        }))
    }

    /// Create a storage disk full error - migration helper
    ///
    /// Replaces: `StorageError::DiskFull { path, available, required }`
    #[must_use]
    pub fn storage_disk_full(
        path: impl Into<Cow<'static, str>>,
        required_bytes: u64,
        available_bytes: u64,
    ) -> Self {
        let path = path.into();
        Self::ResourceExhausted(Box::new(ResourceExhaustedErrorDetails {
            message: format!(
                "Disk full: {path} (required: {required_bytes} bytes, available: {available_bytes} bytes)"
            )
            .into(),
            resource: Cow::Borrowed("disk_space"),
            limit: Some(required_bytes),
            current: Some(available_bytes),
            context: None,
        }))
    }

    /// Create a validation field error - migration helper
    ///
    /// Replaces: `ValidationError::FieldValidation { field, message, constraint }`
    #[must_use]
    pub fn validation_field(
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        let field_str = field.into();
        let message_str = message.into();
        Self::Validation(Box::new(ValidationErrorDetails {
            message: format!("Field '{field_str}': {message_str}").into(),
            field: Some(field_str),
            expected: None,
            actual: None,
            context: None,
        }))
    }

    /// Create a validation schema error - migration helper
    ///
    /// Replaces: `ValidationError::SchemaValidation { schema, message, path }`
    #[must_use]
    pub fn validation_schema(
        schema: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
        path: Option<Cow<'static, str>>,
    ) -> Self {
        let schema = schema.into();
        let message = message.into();
        Self::Validation(Box::new(ValidationErrorDetails {
            message: format!("Schema validation failed ({schema}): {message}").into(),
            field: path,
            expected: Some(schema),
            actual: None,
            context: None,
        }))
    }

    /// Create a security authentication failed error - migration helper
    ///
    /// Replaces: `SecurityError::AuthenticationFailed { principal, reason }`
    #[must_use]
    pub fn security_authentication_failed(
        principal: impl Into<Cow<'static, str>>,
        reason: impl Into<Cow<'static, str>>,
    ) -> Self {
        let principal = principal.into();
        let reason = reason.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!("Authentication failed for '{principal}': {reason}").into(),
            operation: Some(Cow::Borrowed("authenticate")),
            principal: Some(principal),
            security_data: None,
            context: None,
        }))
    }

    /// Create a security authorization failed error - migration helper
    ///
    /// Replaces: `SecurityError::AuthorizationFailed { principal, action, resource }`
    #[must_use]
    pub fn security_authorization_failed(
        principal: impl Into<Cow<'static, str>>,
        action: impl Into<Cow<'static, str>>,
        resource: impl Into<Cow<'static, str>>,
    ) -> Self {
        let principal = principal.into();
        let action = action.into();
        let resource = resource.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!(
                "Authorization failed: '{principal}' cannot '{action}' on '{resource}'"
            )
            .into(),
            operation: Some(action),
            principal: Some(principal),
            security_data: None,
            context: None,
        }))
    }

    /// Create a security encryption error - migration helper
    ///
    /// Replaces: `SecurityError::EncryptionFailed { algorithm, reason }`
    #[must_use]
    pub fn security_encryption_failed(
        algorithm: impl Into<Cow<'static, str>>,
        reason: impl Into<Cow<'static, str>>,
    ) -> Self {
        let algorithm = algorithm.into();
        let reason = reason.into();
        Self::Security(Box::new(SecurityErrorDetails {
            message: format!("Encryption failed ({algorithm}): {reason}").into(),
            operation: Some(Cow::Borrowed("encrypt")),
            principal: None,
            security_data: None,
            context: None,
        }))
    }

    /// Create an API not found error - migration helper
    ///
    /// Replaces: `ApiError::NotFound { endpoint }`
    #[must_use]
    pub fn api_not_found(endpoint: impl Into<Cow<'static, str>>) -> Self {
        let endpoint = endpoint.into();
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Endpoint not found: {endpoint}").into(),
            status_code: Some(404),
            request_id: None,
            endpoint: Some(endpoint),
            context: None,
        }))
    }

    /// Create an API bad request error - migration helper
    ///
    /// Replaces: `ApiError::BadRequest { reason }`
    #[must_use]
    pub fn api_bad_request(reason: impl Into<Cow<'static, str>>) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Bad request: {}", reason.into()).into(),
            status_code: Some(400),
            request_id: None,
            endpoint: None,
            context: None,
        }))
    }

    /// Create an API internal server error - migration helper
    ///
    /// Replaces: `ApiError::InternalError { message }`
    #[must_use]
    pub fn api_internal_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: format!("Internal server error: {}", message.into()).into(),
            status_code: Some(500),
            request_id: None,
            endpoint: None,
            context: None,
        }))
    }

    /// Create a configuration invalid value error - migration helper
    ///
    /// Replaces: `ConfigurationError::InvalidValue { field, value, expected }`
    #[must_use]
    pub fn configuration_invalid_value(
        field: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
        expected: impl Into<Cow<'static, str>>,
    ) -> Self {
        let field = field.into();
        let value = value.into();
        let expected = expected.into();
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.clone(),
            message: format!("Invalid value for '{field}': got '{value}', expected '{expected}'")
                .into(),
            currentvalue: Some(value),
            expected: Some(expected),
            user_error: true,
        }))
    }

    /// Create a configuration missing required field error - migration helper
    ///
    /// Replaces: `ConfigurationError::MissingRequired { field }`
    #[must_use]
    pub fn configuration_missing_required(field: impl Into<Cow<'static, str>>) -> Self {
        let field = field.into();
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.clone(),
            message: format!("Missing required configuration field: '{field}'").into(),
            currentvalue: None,
            expected: Some(Cow::Borrowed("required value")),
            user_error: true,
        }))
    }

    /// Create a feature not enabled error
    ///
    /// Used when optional features are accessed but not enabled
    #[must_use]
    pub fn feature_not_enabled(
        feature: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::NotImplemented(Box::new(NotImplementedErrorDetails {
            feature: feature.into(),
            message: Some(message.into()),
            planned_version: None,
        }))
    }

    /// Create a storage operation error with operation name
    #[must_use]
    pub fn storage_operation(message: impl Into<Cow<'static, str>>, _recoverable: bool) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: message.into(),
            operation: Some(Cow::Borrowed("storage_operation")),
            resource: None,
            storage_data: None,
            context: None,
        }))
    }

    /// Create a simple error from a message string.
    ///
    /// Convenience constructor that wraps the message as an `Internal` error
    /// with automatic module tracking. Prefer domain-specific constructors
    /// (`configuration_error`, `network_error`, etc.) when the error category
    /// is known; use `simple` for quick prototyping and macro-generated errors.
    #[must_use]
    pub fn simple(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Internal(Box::new(InternalErrorDetails {
            message: message.into(),
            component: Cow::Borrowed("simple"),
            location: None,
            is_bug: false,
            context: None,
        }))
    }
}
