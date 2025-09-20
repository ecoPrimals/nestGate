//! Error conversion implementations
//! Error handling types and utilities.
//! This module provides conversion implementations from standard library
//! and third-party error types to `NestGate` errors.

use super::variants::core_errors::*;

/// Convert standard I/O errors to `NestGate` errors
impl From<std::io::Error> for NestGateUnifiedError {
    fn from(error: std::io::Error) -> Self {
        Self::internal_error(format!("I/O error: {error}"), "io_operation")
    }
}

/// Convert serde JSON errors to `NestGate` errors
impl From<serde_json::Error> for NestGateUnifiedError {
    fn from(error: serde_json::Error) -> Self {
        Self::validation_error(&format!("JSON error: {error}"))
    }
}

/// Convert string errors to `NestGate` errors
impl From<String> for NestGateUnifiedError {
    fn from(error: String) -> Self {
        Self::internal_error(error, "string_conversion")
    }
}

/// Convert &str errors to `NestGate` errors
impl From<&str> for NestGateUnifiedError {
    fn from(error: &str) -> Self {
        Self::internal_error(error, "str_conversion")
    }
}
