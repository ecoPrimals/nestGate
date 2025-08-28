use nestgate_core::{NestGateError, Result, IdioResult};

/// **UNIFIED ERROR SYSTEM MIGRATION**
/// 
/// The installer now uses the unified NestGateError system for all operations,
/// eliminating the fragmented InstallerError type and providing better integration
/// with the rest of the NestGate ecosystem.

/// **CANONICAL RESULT TYPE** - Use unified error system
pub type InstallerResult<T> = Result<T>;

/// **DOMAIN-SPECIFIC RESULT TYPE** - For explicit installer context
pub type InstallResult<T> = IdioResult<T, NestGateError>;

// ==================== SECTION ====================

/// Helper functions for creating installer-specific errors using unified system
pub struct InstallerErrorHelper;

impl InstallerErrorHelper {
    /// Create configuration error using unified system
    pub fn configuration_error(message: impl Into<String>) -> NestGateError {
        NestGateError::Configuration {
            field: "installer_config".to_string(),
            message: message.into(),
            current_value: None,
            expected: None,
            user_error: true,
        }
    }

    /// Create installation failure error using unified system
    pub fn installation_error(message: impl Into<String>) -> NestGateError {
        NestGateError::Installation {
            message: message.into(),
            component: "nestgate-installer".to_string(),
            step: Some("installation".to_string()),
            retryable: true,
            installer_data: None,
            context: None,
        }
    }

    /// Create validation error using unified system
    pub fn validation_error(message: impl Into<String>) -> NestGateError {
        NestGateError::Validation {
            message: message.into(),
            field: "installer_input".to_string(),
            value: None,
            current_value: None,
            expected: None,
            context: None,
        }
    }

    /// Create system error using unified system
    pub fn system_error(message: impl Into<String>) -> NestGateError {
        NestGateError::Internal {
            message: message.into(),
            component: "installer_system".to_string(),
            location: None,
            bug_report: false,
            context: None,
        }
    }

    /// Convert I/O errors to unified system
    pub fn from_io_error(error: std::io::Error, operation: impl Into<String>) -> NestGateError {
        NestGateError::Io {
            message: error.to_string(),
            operation: operation.into(),
            path: None,
            retryable: false,
            context: None,
        }
    }
}
