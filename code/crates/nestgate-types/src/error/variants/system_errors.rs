// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **SYSTEM ERROR UTILITIES**
//! System-level error types and handling for the `NestGate` system.
// System and internal error handling utilities.

use std::borrow::Cow;

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a new system error
    pub fn system(
        message: impl Into<Cow<'static, str>>,
        component: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::System(Box::new(
            crate::error::variants::core_errors::SystemErrorDetails {
                message: message.into(),
                component: component.into(),
                operation: None,
                context: None,
            },
        ))
    }

    /// Create an internal error
    pub fn internal(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Internal(Box::new(
            crate::error::variants::core_errors::InternalErrorDetails {
                message: message.into(),
                component: Cow::Borrowed("unknown"),
                location: None,
                context: None,
                is_bug: false,
            },
        ))
    }

    /// Create an internal error with component context
    pub fn internal_with_component(
        message: impl Into<Cow<'static, str>>,
        component: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::Internal(Box::new(
            crate::error::variants::core_errors::InternalErrorDetails {
                message: message.into(),
                component: component.into(),
                location: None,
                context: None,
                is_bug: false,
            },
        ))
    }

    /// Legacy compatibility method for `internal_error` calls
    /// This method provides backward compatibility for the old `internal_error` signature
    pub fn internal_error(
        message: impl Into<Cow<'static, str>>,
        component: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::internal_with_component(message, component)
    }

    /// Create an external service error
    pub fn external_service_unavailable(
        service: impl Into<Cow<'static, str>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::External(Box::new(
            crate::error::variants::core_errors::ExternalErrorDetails {
                message: message.into(),
                service: service.into(),
                retryable: true,
                context: None,
            },
        ))
    }

    /// Create a validation error
    pub fn validation(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Validation(Box::new(
            crate::error::variants::core_errors::ValidationErrorDetails {
                message: message.into(),
                field: None,
                expected: None,
                actual: None,
                context: None,
            },
        ))
    }

    /// Create an I/O error
    pub fn io_error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::System(Box::new(
            crate::error::variants::core_errors::SystemErrorDetails {
                message: message.into(),
                component: Cow::Borrowed("io"),
                operation: Some(Cow::Borrowed("io")),
                context: None,
            },
        ))
    }

    /// Create an internal error with debug context
    pub fn internal_error_with_debug_context(
        message: impl Into<Cow<'static, str>>,
        debug_info: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::Internal(Box::new(
            crate::error::variants::core_errors::InternalErrorDetails {
                message: format!("{} (debug: {})", message.into(), debug_info.into()).into(),
                component: Cow::Borrowed("debug"),
                location: None,
                context: None,
                is_bug: false,
            },
        ))
    }

    /// Create a not implemented error
    pub fn not_implemented(message: impl Into<Cow<'static, str>>) -> Self {
        Self::NotImplemented(Box::new(
            crate::error::variants::core_errors::NotImplementedErrorDetails {
                feature: message.into(),
                message: None,
                planned_version: None,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::error::variants::core_errors::NestGateUnifiedError;

    #[test]
    fn system_and_internal_constructors_smoke() {
        let e = NestGateUnifiedError::system("m", "c");
        assert!(matches!(e, NestGateUnifiedError::System(_)));
        let e = NestGateUnifiedError::internal("i");
        assert!(matches!(e, NestGateUnifiedError::Internal(_)));
        let e = NestGateUnifiedError::internal_with_component("i", "comp");
        assert!(matches!(e, NestGateUnifiedError::Internal(_)));
        let e = NestGateUnifiedError::internal_error("i", "c");
        assert!(matches!(e, NestGateUnifiedError::Internal(_)));
        let e = NestGateUnifiedError::external_service_unavailable("svc", "down");
        assert!(matches!(e, NestGateUnifiedError::External(_)));
        let e = NestGateUnifiedError::validation("bad");
        assert!(matches!(e, NestGateUnifiedError::Validation(_)));
        let e = NestGateUnifiedError::io_error("read fail");
        assert!(matches!(e, NestGateUnifiedError::System(_)));
        let e = NestGateUnifiedError::internal_error_with_debug_context("m", "dbg");
        assert!(matches!(e, NestGateUnifiedError::Internal(_)));
        let e = NestGateUnifiedError::not_implemented("feature-x");
        assert!(matches!(e, NestGateUnifiedError::NotImplemented(_)));
    }
}
