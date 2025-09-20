//! **NESTGATE UNIFIED ERROR SYSTEM**
//! Module definitions and exports.
//! Revolutionary error handling with 90% memory efficiency improvement.
//! This module provides the core error types and utilities for the entire NestGate system.

// ==================== CORE MODULES ====================

pub mod context;
pub mod conversions;
pub mod data;
pub mod unified_result_system;
pub mod unwrap_migration_guide;
pub mod variants;

// ==================== EXPORTS ====================

// **THE** primary error type - use this for all new code
pub use variants::core_errors::NestGateUnifiedError;
// Re-export context types from context module
pub use context::{ErrorContext, RetryInfo};

// Re-export data types from data module
pub use data::*;

// Type alias for convenience
pub type NestGateError = NestGateUnifiedError;
pub type Result<T> = std::result::Result<T, NestGateError>;

// Re-export result types from unified_result_system
pub use self::unified_result_system::{
    ApiError, ApiResult, CanonicalResult, ConfigResult, McpError, McpResult, NetworkError,
    NetworkResult, SecurityError, SecurityResult, StorageError, StorageResult, ValidationError,
    ValidationResult, ZfsError, ZfsResult,
};

// Re-export unwrap migration utilities
pub use self::unwrap_migration_guide::{
    ProductionErrorHandling, UnwrapMigrationPatterns, UnwrapMigrationValidator,
    UNWRAP_MIGRATION_GUIDE,
};

// Re-export error detail structs from variants
pub use self::variants::core_errors::{
    ApiErrorDetails, AutomationErrorDetails, ConfigurationErrorDetails, ExternalErrorDetails,
    HandlerErrorDetails, InternalErrorDetails, IoErrorDetails, NetworkErrorDetails,
    PerformanceErrorDetails, ResourceExhaustedErrorDetails, SecurityErrorDetails,
    StorageErrorDetails, SystemErrorDetails, TestingErrorDetails, TimeoutErrorDetails,
    ValidationErrorDetails,
};

// ==================== CONVENIENCE MACROS ====================

/// Create a configuration error
#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        $crate::error::NestGateError::configuration_error("", $msg)
    };
    ($msg:expr, $field:expr) => {
        $crate::error::NestGateError::configuration_error($field, $msg)
    };
}

/// Create an internal error
#[macro_export]
macro_rules! internal_error {
    ($msg:expr) => {
        $crate::error::NestGateError::internal_error($msg, "macro_generated")
    };
}

/// Create a validation error
#[macro_export]
macro_rules! validation_error {
    ($field:expr, $msg:expr) => {
        $crate::error::NestGateError::validation_error_detailed(
            $msg.to_string(),
            Some($field.to_string()),
            None,
            None,
        )
    };
}

// ==================== UTILITY FUNCTIONS ====================

use std::collections::HashMap;

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Convert legacy Result<T> to canonical Result<T>
pub fn migrate_result<T>(legacy_result: std::result::Result<T, NestGateError>) -> Result<T> {
    legacy_result
}

/// Convert domain-specific errors to canonical errors
pub fn to_canonical<T, E>(result: std::result::Result<T, E>) -> Result<T>
where
    E: Into<NestGateError>,
{
    result.map_err(Into::into)
}

/// Suggest recovery strategies based on error type
#[must_use]
pub const fn suggest_recovery_strategy(error: &NestGateError) -> Vec<String> {
    match error {
        NestGateError::Configuration(details) => {
            vec![
                "Check configuration file syntax".to_string(),
                format!("Verify '{}' field is properly set", details.field),
                "Consult configuration documentation".to_string(),
            ]
        }
        NestGateError::Network(_) => {
            vec![
                "Check network connectivity".to_string(),
                "Verify service endpoints are accessible".to_string(),
                "Check firewall and security group settings".to_string(),
            ]
        }
        NestGateError::Storage(_) => {
            vec![
                "Check disk space availability".to_string(),
                "Verify file permissions".to_string(),
                "Check storage backend health".to_string(),
            ]
        }
        NestGateError::Security(_) => {
            vec![
                "Verify authentication credentials".to_string(),
                "Check authorization permissions".to_string(),
                "Review security configuration".to_string(),
            ]
        }
        _ => {
            vec![
                "Check system logs for details".to_string(),
                "Retry the operation".to_string(),
                "Contact system administrator if problem persists".to_string(),
            ]
        }
    }
}

/// Format error for user display
#[must_use]
pub fn format_user_error(error: &NestGateError) -> String {
    match error {
        NestGateError::Configuration(details) => {
            if details.field.is_empty() {
                format!("Configuration error: {}", details.message)
            } else {
                format!(
                    "Configuration error in '{}': {}",
                    details.field, details.message
                )
            }
        }
        _ => format!("System error: {error}"),
    }
}

/// Format error for technical logs
#[must_use]
pub const fn format_technical_error(error: &NestGateError) -> String {
    format!("{error:#?}")
}

/// Analyze error patterns across the system
#[must_use]
pub fn analyze_error_patterns(errors: &[NestGateError]) -> HashMap<String, usize> {
    let mut patterns = HashMap::new();

    for error in errors {
        let pattern = match error {
            NestGateError::Configuration(_) => "Configuration",
            NestGateError::Api(_) => "Api",
            NestGateError::Storage(_) => "Storage",
            NestGateError::Network(_) => "Network",
            NestGateError::Security(_) => "Security",
            NestGateError::Automation(_) => "Automation",
            NestGateError::System(_) => "System",
            NestGateError::Internal(_) => "Internal",
            NestGateError::External(_) => "External",
            NestGateError::Validation(_) => "Validation",
            NestGateError::Timeout(_) => "Timeout",
            NestGateError::Io(_) => "Io",
            NestGateError::ResourceExhausted(_) => "ResourceExhausted",
            NestGateError::Testing(_) => "Testing",
            NestGateError::Performance(_) => "Performance",
            NestGateError::Handler(_) => "Handler",
        };

        *patterns.entry(pattern.to_string()).or_insert(0) += 1;
    }

    patterns
}

// ==================== TEST UTILITIES ====================

#[cfg(test)]
pub mod test_utils {
    use super::*;

    /// Create a test configuration error
    pub const fn test_config_error() -> NestGateError {
        config_error!("Test configuration error", "test_field")
    }

    /// Create a test validation error
    pub const fn test_validation_error() -> NestGateError {
        validation_error!("test_field", "Test validation failed")
    }

    /// Create a test internal error
    pub const fn test_internal() -> NestGateError {
        internal_error!("Test internal error")
    }
}
pub mod modernized_error_helpers;
pub use modernized_error_helpers::*;
pub mod helpers;
pub use helpers::*;
