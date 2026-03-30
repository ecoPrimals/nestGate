// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Primary and backward-compatible constructors for [`NestGateUnifiedError`](super::unified_enum::NestGateUnifiedError).

use std::borrow::Cow;

use super::details::{
    ApiErrorDetails, ConfigurationErrorDetails, NetworkErrorDetails, SecurityErrorDetails,
    StorageErrorDetails, TimeoutErrorDetails, ValidationErrorDetails,
};
use super::unified_enum::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a configuration error
    #[must_use]
    pub fn configuration_error(
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.into(),
            message: message.into(),
            currentvalue: None,
            expected: None,
            user_error: false,
        }))
    }

    /// Create an API error
    #[must_use]
    pub fn api_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: message.into(),
            status_code: None,
            request_id: None,
            endpoint: None,
            context: None,
        }))
    }

    /// Create a storage error
    #[must_use]
    pub fn storage_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: message.into(),
            resource: None,
            storage_data: None,
            operation: None,
            context: None,
        }))
    }

    /// Create a security error
    #[must_use]
    pub fn security_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Security(Box::new(SecurityErrorDetails {
            message: message.into(),
            operation: None,
            principal: None,
            security_data: None,
            context: None,
        }))
    }

    /// Create a network error
    #[must_use]
    pub fn network_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Network(Box::new(NetworkErrorDetails {
            message: message.into(),
            endpoint: None,
            network_data: None,
            operation: None,
            context: None,
        }))
    }

    /// Create a validation error
    #[must_use]
    pub fn validation_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Validation(Box::new(ValidationErrorDetails {
            message: message.into(),
            field: None,
            expected: None,
            actual: None,
            context: None,
        }))
    }

    // internal method moved to system_errors.rs to avoid duplication

    /// Create a timeout error
    #[must_use]
    pub fn timeout_error(
        operation: impl Into<Cow<'static, str>>,
        duration: std::time::Duration,
    ) -> Self {
        let operation = operation.into();
        Self::Timeout(Box::new(TimeoutErrorDetails {
            message: format!("Operation '{operation}' timed out after {duration:?}").into(),
            operation: Some(operation),
            timeout: duration,
            retryable: true,
            context: None,
        }))
    }

    // ==================== BACKWARD COMPATIBILITY CONSTRUCTORS ====================
    // These methods maintain the old struct-style construction patterns for easier migration

    /// Create configuration error with detailed fields (backward compatibility)
    #[must_use]
    pub fn configuration_error_detailed(
        field: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
        currentvalue: Option<Cow<'static, str>>,
        expected: Option<Cow<'static, str>>,
        user_error: bool,
    ) -> Self {
        Self::Configuration(Box::new(ConfigurationErrorDetails {
            field: field.into(),
            message: message.into(),
            currentvalue,
            expected,
            user_error,
        }))
    }

    /// Create API error with detailed fields (backward compatibility)
    #[must_use]
    pub fn api_error_detailed(
        message: impl Into<Cow<'static, str>>,
        status_code: Option<u16>,
        request_id: Option<Cow<'static, str>>,
        endpoint: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Api(Box::new(ApiErrorDetails {
            message: message.into(),
            status_code,
            request_id,
            endpoint,
            context: None,
        }))
    }

    /// Create storage error with detailed fields (backward compatibility)
    #[must_use]
    pub fn storage_error_detailed(
        message: impl Into<Cow<'static, str>>,
        operation: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Storage(Box::new(StorageErrorDetails {
            message: message.into(),
            operation,
            resource: None,
            storage_data: None,
            context: None,
        }))
    }

    /// Create network error with detailed fields (backward compatibility)
    #[must_use]
    pub fn network_error_detailed(
        message: impl Into<Cow<'static, str>>,
        operation: Option<Cow<'static, str>>,
        endpoint: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Network(Box::new(NetworkErrorDetails {
            message: message.into(),
            operation,
            endpoint,
            network_data: None,
            context: None,
        }))
    }

    /// Create validation error with detailed fields (backward compatibility)
    #[must_use]
    pub fn validation_error_detailed(
        message: impl Into<Cow<'static, str>>,
        field: Option<Cow<'static, str>>,
        expected: Option<Cow<'static, str>>,
        actual: Option<Cow<'static, str>>,
    ) -> Self {
        Self::Validation(Box::new(ValidationErrorDetails {
            message: message.into(),
            field,
            expected,
            actual,
            context: None,
        }))
    }
}
