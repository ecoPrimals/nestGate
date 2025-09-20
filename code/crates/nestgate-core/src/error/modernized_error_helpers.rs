//! 🔧 MODERNIZED ERROR HANDLING HELPERS
//! Error handling types and utilities.
//! Provides simplified error creation for common patterns

use crate::error::NestGateUnifiedError;

/// Create a storage error with modern pattern
pub const fn storage_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::storage_error(&message.into())
}

/// Create a configuration error with modern pattern  
pub const fn configuration_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::configuration_error("config", &message.into())
}

/// Create a validation error with modern pattern
pub const fn validation_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::validation(message.into())
}

/// Create an internal error with modern pattern
pub const fn internal(message: impl Into<String>, component: &str) -> NestGateUnifiedError {
    NestGateUnifiedError::internal_error(message.into(), component.to_string())
}
