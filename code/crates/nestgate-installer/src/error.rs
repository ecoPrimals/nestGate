// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// ==================== SECTION: CANONICAL ERROR TYPES ====================

// CANONICAL MODERNIZATION: Consolidate installer error types
// USE CANONICAL TYPES:
//! Error module

pub use nestgate_core::error::{NestGateError, Result};

/// [`Result`] specialized with the canonical [`NestGateError`] for installer operations.
pub type InstallerResult<T> = Result<T>;

// ==================== SECTION ====================
use nestgate_core::error::{ConfigurationErrorDetails, InternalErrorDetails, SystemErrorDetails};

/// Installer-specific error type
pub struct InstallerError;

/// Installer-specific result type
pub type InstallResult<T> = std::result::Result<T, NestGateError>;

/// Installer-specific error utilities
impl InstallerError {
    /// Create configuration error using unified system
    pub fn configuration(message: impl Into<String>) -> NestGateError {
        NestGateError::Configuration(Box::new(ConfigurationErrorDetails {
            field: "field".into(),
            message: message.into().into(),
            currentvalue: None,
            expected: None,
            user_error: true,
        }))
    }

    /// Create installation failure error using unified system
    pub fn installation_error(message: impl Into<String>) -> NestGateError {
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message: message.into().into(),
            component: "nestgate-installer".into(),
            location: Some("installation".into()),
            is_bug: false,
            context: None,
        }))
    }

    /// Create system requirement error using unified system
    pub fn system_requirement(message: impl Into<String>) -> NestGateError {
        NestGateError::System(Box::new(SystemErrorDetails {
            message: message.into().into(),
            component: "system-requirements".into(),
            operation: Some("validation".into()),
            context: None,
        }))
    }

    /// Create permission error using unified system
    pub fn permission_error(message: impl Into<String>) -> NestGateError {
        NestGateError::System(Box::new(SystemErrorDetails {
            message: message.into().into(),
            component: "permissions".into(),
            operation: Some("access".into()),
            context: None,
        }))
    }
}

// CANONICAL MODERNIZATION: Removed orphan trait implementations
// These From implementations violate Rust's orphan rules since NestGateError
// is defined in nestgate_core and std::io::Error/serde_json::Error are external.
// Use helper functions instead for error conversion.

/// Create an installation error
pub fn installation_error(message: impl Into<String>) -> NestGateError {
    NestGateError::internal_error(
        format!("Installation error: {}", message.into()),
        "installer",
    )
}
/// Create an installation error from an IO error
pub fn from_io_error(error: std::io::Error, _b_operation: impl Into<String>) -> NestGateError {
    NestGateError::internal_error(format!("IO error: {error}"), "installer")
}
/// Create a validation error
pub fn validation(_message: impl Into<String>) -> NestGateError {
    NestGateError::validation("Validation error occurred during installation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::error::NestGateError;

    #[test]
    fn installer_error_configuration_maps_to_nest_gate_error() {
        let e = InstallerError::configuration("bad config");
        assert!(matches!(e, NestGateError::Configuration(_)));
    }

    #[test]
    fn installer_error_installation_and_system_helpers() {
        let e = InstallerError::installation_error("failed");
        assert!(matches!(e, NestGateError::Internal(_)));
        let e = InstallerError::system_requirement("need more ram");
        assert!(matches!(e, NestGateError::System(_)));
        let e = InstallerError::permission_error("root only");
        assert!(matches!(e, NestGateError::System(_)));
    }

    #[test]
    fn installation_error_helper_contains_prefix() {
        let e = installation_error("disk full");
        assert!(e.to_string().contains("Installation"));
    }

    #[test]
    fn from_io_error_maps_io() {
        let io = std::io::Error::other("oops");
        let e = from_io_error(io, "copy");
        assert!(e.to_string().contains("IO"));
    }

    #[test]
    fn validation_helper_returns_validation_variant() {
        let e = validation("x");
        assert!(matches!(e, NestGateError::Validation(_)));
    }
}
