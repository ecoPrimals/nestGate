//! **NESTGATE UNIFIED ERROR SYSTEM**
//!
//! This module provides the single, authoritative error system for NestGate,
//! consolidating and replacing ALL fragmented error types across all crates.

// ==================== SECTION ====================

/// **PRIMARY**: The definitive unified error system
pub mod variants;

/// Supporting error types and utilities
pub mod core;
pub mod context;
pub mod data;
pub mod enhanced_ergonomics;
pub mod idiomatic_evolution;
pub mod phase4_ecosystem_adoption;
// unified_error_consolidation - REMOVED: Migration complete

// ==================== SECTION ====================

/// **THE** primary error type - use this for all new code
pub use variants::{NestGateUnifiedError};

// Re-export context types from core module
pub use core::{ErrorContext, RetryInfo};

// Re-export data types from data module
pub use data::{StorageErrorData, NetworkErrorData, SecurityErrorData, SecuritySeverity, AutomationErrorData};

// ==================== SECTION ====================

/// **THE** primary error type - canonical across all NestGate
pub type NestGateError = NestGateUnifiedError;

/// **CANONICAL IDIOMATIC RESULT** - The primary Result type for all NestGate code
/// 
/// This implements the idiomatic Rust Result<T, E> pattern where both T and E are generic.
/// Uses NestGateError as the default error type for seamless compatibility.
/// 
/// **USAGE PATTERNS**:
/// ```rust
/// // Unified error (most common) - uses NestGateError default
/// fn operation() -> Result<Data> { ... }
/// 
/// // Domain-specific error - explicit error type
/// fn validate() -> Result<Config, ValidationError> { ... }
/// 
/// // Ecosystem integration - external error type
/// fn parse() -> Result<Value, serde_json::Error> { ... }
/// ```
pub type Result<T, E = NestGateError> = std::result::Result<T, E>;

/// **CANONICAL RESULT ALIAS** - For explicit canonical usage
pub type CanonicalResult<T> = Result<T>;

/// **IDIOMATIC RESULT** - Explicit generic Result type for migration compatibility
pub type IdioResult<T, E = NestGateError> = std::result::Result<T, E>;

// ==================== SECTION ====================

/// Import domain-specific error types
pub use idiomatic_evolution::{
    ValidationError, NetworkError, StorageError, SecurityError,
    ZfsError, ApiError, McpError
};

/// **VALIDATION OPERATIONS** - Rich error context for validation
pub type ValidationResult<T> = IdioResult<T, ValidationError>;

/// **NETWORK OPERATIONS** - Rich error context for network operations
pub type NetworkResult<T> = IdioResult<T, NetworkError>;

/// **STORAGE OPERATIONS** - Rich error context for storage operations
pub type StorageResult<T> = IdioResult<T, StorageError>;

/// **SECURITY OPERATIONS** - Rich error context for security operations
pub type SecurityResult<T> = IdioResult<T, SecurityError>;

/// **ZFS OPERATIONS** - Rich error context for ZFS operations
pub type ZfsResult<T> = IdioResult<T, ZfsError>;

/// **API OPERATIONS** - Rich error context for API operations
pub type ApiResult<T> = IdioResult<T, ApiError>;

/// **MCP PROTOCOL OPERATIONS** - Rich error context for MCP operations
pub type McpResult<T> = IdioResult<T, McpError>;

/// **CONFIGURATION OPERATIONS** - Use validation errors for config
pub type ConfigResult<T> = ValidationResult<T>;

// ==================== SECTION ====================

/// Legacy error types - being phased out
pub use core::{
    NestGateLegacyError,
    TestErrorData
};

// ==================== SECTION ====================
// Error data types are now available directly from the error module
// Use: data::ZfsErrorData, data::NetworkErrorData, etc.

// ==================== SECTION ====================

use std::collections::HashMap;

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium, 
    /// High severity - error
    High,
    /// Critical severity - system failure
    Critical,
}

impl Default for ErrorSeverity {
    fn default() -> Self {
        Self::Medium
    }
}

// ==================== SECTION ====================

/// Migration helper for converting legacy Result types
pub struct ResultMigrationHelper;

impl ResultMigrationHelper {
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
}

// ==================== SECTION ====================

/// Extension trait for adding domain context to results
pub trait WithDomainContext<T> {
    /// Add domain-specific context to an error
    fn with_domain_context(self, domain: &str, operation: &str) -> Result<T>;
    
    /// Add ZFS-specific context
    fn with_zfs_context(self, operation: &str, pool: Option<&str>) -> Result<T>;
    
    /// Add network-specific context  
    fn with_network_context(self, operation: &str, endpoint: Option<&str>) -> Result<T>;
    
    /// Add API-specific context
    fn with_api_context(self, endpoint: &str, method: &str) -> Result<T>;
}

impl<T, E> WithDomainContext<T> for std::result::Result<T, E>
where
    E: Into<NestGateError>,
{
    fn with_domain_context(self, domain: &str, operation: &str) -> Result<T> {
        self.map_err(|e| {
            let mut error = e.into();
            // Add domain context to the error
            error
        })
    }
    
    fn with_zfs_context(self, operation: &str, pool: Option<&str>) -> Result<T> {
        self.with_domain_context("zfs", operation)
    }
    
    fn with_network_context(self, operation: &str, endpoint: Option<&str>) -> Result<T> {
        self.with_domain_context("network", operation)
    }
    
    fn with_api_context(self, endpoint: &str, method: &str) -> Result<T> {
        self.with_domain_context("api", &format!("{} {}", method, endpoint))
    }
}

// ==================== SECTION ====================

/// Create a configuration error
#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        $crate::error::NestGateError::Configuration {
            message: $msg.to_string(),
            field: None,
        }
    };
    ($msg:expr, $field:expr) => {
        $crate::error::NestGateError::Configuration {
            message: $msg.to_string(),
            field: Some($field.to_string()),
        }
    };
}

/// Create an internal error
#[macro_export]
macro_rules! internal_error {
    ($msg:expr) => {
        $crate::error::NestGateError::Internal {
            message: $msg.to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            is_bug: true,
        }
    };
}

/// Create a validation error
#[macro_export]
macro_rules! validation_error {
    ($field:expr, $msg:expr) => {
        $crate::error::NestGateError::Validation {
            field: $field.to_string(),
            message: $msg.to_string(),
            current_value: None,
            expected: None,
            user_error: true,
                context: None,
        }
    };
    ($field:expr, $msg:expr, $current:expr, $expected:expr) => {
        $crate::error::NestGateError::Validation {
            field: $field.to_string(),
            message: $msg.to_string(),
            current_value: Some($current.to_string()),
            expected: Some($expected.to_string()),
            user_error: true,
                context: None,
        }
    };
}

// ==================== SECTION ====================

/// Analyze error patterns across the system
pub fn analyze_error_patterns(errors: &[NestGateError]) -> HashMap<String, usize> {
    let mut patterns = HashMap::new();
    
    for error in errors {
        let pattern = match error {
            NestGateError::Configuration { .. } => "configuration",
            NestGateError::Validation { .. } => "validation", 
            NestGateError::Network { .. } => "network",
            NestGateError::Storage { .. } => "storage",
            NestGateError::Security { .. } => "security",
            NestGateError::Internal { .. } => "internal",
            _ => "other",
        };
        
        *patterns.entry(pattern.to_string()).or_insert(0) += 1;
    }
    
    patterns
}

/// Get error severity level
pub fn error_severity(error: &NestGateError) -> ErrorSeverity {
    match error {
        NestGateError::Internal { is_bug: true, .. } => ErrorSeverity::Critical,
        NestGateError::Security { .. } => ErrorSeverity::High,
        NestGateError::Network { .. } => ErrorSeverity::Medium,
        NestGateError::Storage { .. } => ErrorSeverity::Medium,
        NestGateError::Configuration { .. } => ErrorSeverity::Medium,
        NestGateError::Validation { .. } => ErrorSeverity::Low,
        _ => ErrorSeverity::Medium,
    }
}

/// Suggest recovery strategies for common errors
pub fn suggest_recovery_strategy(error: &NestGateError) -> Vec<String> {
    match error {
        NestGateError::Configuration { field, .. } => {
            vec![
                "Check configuration file syntax".to_string(),
                format!("Verify '{}' field is properly set", field),
                "Consult configuration documentation".to_string(),
            ]
        }
        NestGateError::Network { .. } => {
            vec![
                "Check network connectivity".to_string(),
                "Verify service endpoints are accessible".to_string(),
                "Check firewall and security group settings".to_string(),
            ]
        }
        NestGateError::Storage { .. } => {
            vec![
                "Check disk space availability".to_string(),
                "Verify file permissions".to_string(),
                "Check storage backend health".to_string(),
            ]
        }
        NestGateError::Security { .. } => {
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

// ==================== SECTION ====================

/// Format error for user display
pub fn format_user_error(error: &NestGateError) -> String {
    match error {
        NestGateError::Validation { field, message, .. } => {
            if field.is_empty() {
                format!("Validation error: {}", message)
            } else {
                format!("Invalid {}: {}", field, message)
            }
        }
        NestGateError::Configuration { message, field, .. } => {
            if field.is_empty() {
                format!("Configuration error: {}", message)
            } else {
                format!("Configuration error in '{}': {}", field, message)
            }
        }
        _ => format!("System error: {}", error),
    }
}

/// Format error for technical logs
pub fn format_technical_error(error: &NestGateError) -> String {
    format!("{:#?}", error)
}

// ==================== SECTION ====================

/// Create test errors for validation
#[cfg(test)]
pub mod test_utils {
    use super::*;
    
    /// Create a test configuration error
    pub fn test_config_error() -> NestGateError {
        config_error!("Test configuration error", "test_field")
    }
    
    /// Create a test validation error
    pub fn test_validation_error() -> NestGateError {
        validation_error!("test_field", "Test validation failed")
    }
    
    /// Create a test internal error
    pub fn test_internal_error() -> NestGateError {
        internal_error!("Test internal error")
    }
}
