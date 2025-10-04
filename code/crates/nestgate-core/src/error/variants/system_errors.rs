// **SYSTEM ERROR UTILITIES**
//! System-level error types and handling for the `NestGate` system.
// System and internal error handling utilities.

use super::core_errors::NestGateUnifiedError;

impl NestGateUnifiedError {
    /// Create a new system error
    pub fn system(message: impl Into<String>, component: impl Into<String>) -> Self {
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
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(Box::new(
            crate::error::variants::core_errors::InternalErrorDetails {
                message: message.into(),
                component: "unknown".to_string(),
                location: None,
                context: None,
                is_bug: false,
            },
        ))
    }

    /// Create an internal error with component context
    pub fn internal_with_component(
        message: impl Into<String>,
        component: impl Into<String>,
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

    /// Legacy compatibility method for internal_error calls
    /// This method provides backward compatibility for the old internal_error signature
    pub fn internal_error(message: impl Into<String>, component: impl Into<String>) -> Self {
        Self::internal_with_component(message, component)
    }

    /// Create an external service error
    pub fn external_service_unavailable(
        service: impl Into<String>,
        message: impl Into<String>,
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
    pub fn validation(message: impl Into<String>) -> Self {
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
    pub fn io_error(message: impl Into<String>) -> Self {
        Self::System(Box::new(
            crate::error::variants::core_errors::SystemErrorDetails {
                message: message.into(),
                component: "io".to_string(),
                operation: Some("io".to_string()),
                context: None,
            },
        ))
    }

    /// Create an internal error with debug context
    pub fn internal_error_with_debug_context(
        message: impl Into<String>,
        debug_info: impl Into<String>,
    ) -> Self {
        Self::Internal(Box::new(
            crate::error::variants::core_errors::InternalErrorDetails {
                message: format!("{} (debug: {})", message.into(), debug_info.into()),
                component: "debug".to_string(),
                location: None,
                context: None,
                is_bug: false,
            },
        ))
    }
}
