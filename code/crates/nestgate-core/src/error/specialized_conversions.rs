//! **SPECIALIZED ERROR CONVERSIONS**
//!
//! This module provides From implementations for specialized error types,
//! enabling automatic conversion to NestGateUnifiedError.
//!
//! **Phase 2 of Error Consolidation** - Specialized Errors
//!
//! This consolidates specialized error types from various modules:
//! - CircuitBreakerError → System
//! - AuthError → Security
//! - SimdError → Performance
//! - CapabilityRoutingError → Internal
//! - RateLimitError → Security
//! - UniversalSecurityError → Security
//! - InputValidationError → Validation
//! - NotificationError → External

use crate::error::{
    NestGateError, NestGateUnifiedError,
    SystemErrorDetails, SecurityErrorDetails, PerformanceErrorDetails,
    InternalErrorDetails, ValidationErrorDetails, ExternalErrorDetails,
};

// ==================== CIRCUIT BREAKER ERROR ====================

use crate::resilience::circuit_breaker::CircuitBreakerError;

impl From<CircuitBreakerError> for NestGateError {
    fn from(err: CircuitBreakerError) -> Self {
        match err {
            CircuitBreakerError::CircuitOpen { name } => {
                NestGateError::System(Box::new(SystemErrorDetails {
                    message: format!("Circuit breaker '{name}' is open - requests blocked"),
                    component: "circuit_breaker".to_string(),
                    operation: Some("request_blocked".to_string()),
                    context: None,
                }))
            },
            CircuitBreakerError::Configuration { message } => {
                NestGateError::System(Box::new(SystemErrorDetails {
                    message: format!("Circuit breaker configuration error: {message}"),
                    component: "circuit_breaker".to_string(),
                    operation: Some("configure".to_string()),
                    context: None,
                }))
            },
            CircuitBreakerError::Internal { message } => {
                NestGateError::System(Box::new(SystemErrorDetails {
                    message: format!("Circuit breaker internal error: {message}"),
                    component: "circuit_breaker".to_string(),
                    operation: None,
                    context: None,
                }))
            },
        }
    }
}

// ==================== AUTH ERROR ====================

use crate::services::auth::types::AuthError;

impl From<AuthError> for NestGateError {
    fn from(err: AuthError) -> Self {
        let message = match err {
            AuthError::InvalidCredentials => "Invalid username or password",
            AuthError::AccountLocked => "Account is temporarily locked",
            AuthError::MfaRequired => "Multi-factor authentication required",
            AuthError::InvalidMfaCode => "Invalid MFA code",
            AuthError::TokenExpired => "Authentication token has expired",
            AuthError::SessionExpired => "Session has expired",
            AuthError::InsufficientPermissions => "Insufficient permissions",
            AuthError::UserNotFound => "User not found",
            AuthError::UserAlreadyExists => "User already exists",
            AuthError::WeakPassword => "Password does not meet security requirements",
            AuthError::InvalidEmail => "Invalid email address",
        };

        NestGateError::Security(Box::new(SecurityErrorDetails {
            message: message.to_string(),
            operation: Some("authenticate".to_string()),
            principal: None,
            security_data: None,
            context: None,
        }))
    }
}

// ==================== SIMD ERROR ====================

use crate::simd::types::SimdError;

impl From<SimdError> for NestGateError {
    fn from(err: SimdError) -> Self {
        let message = match err {
            SimdError::LengthMismatch => "Input and output arrays have different lengths",
            SimdError::UnsupportedOperation => "Operation not supported on current hardware",
            SimdError::InvalidAlignment => "Data not properly aligned for SIMD operations",
            SimdError::BufferTooLarge => "Buffer size exceeds maximum supported size",
        };

        NestGateError::Performance(Box::new(PerformanceErrorDetails {
            message: message.to_string(),
            operation: "simd_operation".to_string(),
            metric: None,
            expected: None,
            actual: None,
            unit: None,
            performance_data: None,
            context: None,
        }))
    }
}

// ==================== CAPABILITY ROUTING ERROR ====================

use crate::ecosystem_integration::capability_router::CapabilityRoutingError;

impl From<CapabilityRoutingError> for NestGateError {
    fn from(err: CapabilityRoutingError) -> Self {
        let message = format!("{err}");
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message,
            component: "capability_router".to_string(),
            location: None,
            is_bug: false,
            context: None,
        }))
    }
}

// ==================== RATE LIMIT ERROR ====================

use crate::security::rate_limiter::RateLimitError;

impl From<RateLimitError> for NestGateError {
    fn from(err: RateLimitError) -> Self {
        let message = format!("{err}");
        NestGateError::Security(Box::new(SecurityErrorDetails {
            message,
            operation: Some("rate_limit_check".to_string()),
            principal: None,
            security_data: None,
            context: None,
        }))
    }
}

// ==================== UNIVERSAL SECURITY ERROR ====================

use crate::universal_security_client::client::UniversalSecurityError;

impl From<UniversalSecurityError> for NestGateError {
    fn from(err: UniversalSecurityError) -> Self {
        let message = format!("{err}");
        NestGateError::Security(Box::new(SecurityErrorDetails {
            message,
            operation: Some("security_client".to_string()),
            principal: None,
            security_data: None,
            context: None,
        }))
    }
}

// ==================== INPUT VALIDATION ERROR ====================

use crate::security::input_validation::InputValidationError;

impl From<InputValidationError> for NestGateError {
    fn from(err: InputValidationError) -> Self {
        let message = format!("{err}");
        NestGateError::Validation(Box::new(ValidationErrorDetails {
            message,
            field: None,
            expected: None,
            actual: None,
            context: None,
        }))
    }
}

// ==================== NOTIFICATION ERROR ====================

use crate::smart_abstractions::notification_channels::NotificationError;

impl From<NotificationError> for NestGateError {
    fn from(err: NotificationError) -> Self {
        let message = format!("{err}");
        NestGateError::External(Box::new(ExternalErrorDetails {
            message,
            service: "notification_system".to_string(),
            retryable: true,
            context: None,
        }))
    }
}

// ==================== ZERO COST ERROR ====================

use crate::zero_cost::types::ZeroCostError;

impl From<ZeroCostError> for NestGateError {
    fn from(err: ZeroCostError) -> Self {
        let message = format!("{err}");
        NestGateError::Performance(Box::new(PerformanceErrorDetails {
            message,
            operation: "zero_cost_operation".to_string(),
            metric: None,
            expected: None,
            actual: None,
            unit: None,
            performance_data: None,
            context: None,
        }))
    }
} 