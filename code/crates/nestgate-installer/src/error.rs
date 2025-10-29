// ==================== SECTION: CANONICAL ERROR TYPES ====================

// CANONICAL MODERNIZATION: Consolidate installer error types
// REMOVED DUPLICATES:
// - pub type InstallerResult<T> = Result<T>;
// - pub type InstallResult<T> = IdioResult<T, NestGateError>;

// USE CANONICAL TYPES:
pub use nestgate_core::error::{NestGateError, Result};

// Convenience aliases for installer operations
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
            field: "field".to_string(),
            message: message.into(),
            currentvalue: None,
            expected: None,
            user_error: true,
        }))
    }

    /// Create installation failure error using unified system
    pub fn installation_error(message: impl Into<String>) -> NestGateError {
        NestGateError::Internal(Box::new(InternalErrorDetails {
            message: message.into(),
            component: "nestgate-installer".to_string(),
            location: Some("installation".to_string()),
            is_bug: false,
            context: None,
        }))
    }

    /// Create system requirement error using unified system
    pub fn system_requirement(message: impl Into<String>) -> NestGateError {
        NestGateError::System(Box::new(SystemErrorDetails {
            message: message.into(),
            component: "system-requirements".to_string(),
            operation: Some("validation".to_string()),
            context: None,
        }))
    }

    /// Create permission error using unified system
    pub fn permission_error(message: impl Into<String>) -> NestGateError {
        NestGateError::System(Box::new(SystemErrorDetails {
            message: message.into(),
            component: "permissions".to_string(),
            operation: Some("access".to_string()),
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
