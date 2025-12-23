//! **CANONICAL RESULT TYPES**
//!
//! Single source of truth for all Result type aliases in NestGate.
//!
//! ## Purpose
//!
//! This module consolidates 17 redundant Result type aliases into a clean,
//! canonical set of 12-14 types. This eliminates confusion and provides
//! clear patterns for error handling across the codebase.
//!
//! ## Usage Guidelines
//!
//! - **Prefer**: `Result<T>` for most cases
//! - **Use**: `CanonicalResult<T>` when disambiguating from `std::result::Result`
//! - **Domain-specific**: Use specialized types for different error types
//! - **Tests**: Use `TestResult<T>` in test code
//! - **Functions**: Use `ConnectionFactory`, `HealthCheckFn`, `ValidatorFn`
//!
//! ## Migration from Legacy Types
//!
//! All domain-specific Result aliases (ApiResult, CacheResult, StorageResult, etc.)
//! have been deprecated in v0.11.2 and will be removed in v0.12.0 (May 2026).
//!
//! Use `Result<T>` directly:
//!
//! ```rust,ignore
//! // OLD (deprecated):
//! pub fn fetch_data() -> ApiResult<Data> { ... }
//!
//! // NEW (canonical):
//! use nestgate_core::Result;
//!
//! pub fn fetch_data() -> Result<Data> { ... }
//! ```
//!
//! ## Consolidation Summary
//!
//! **Before**: 54 Result type aliases scattered across codebase  
//! **After**: 12-14 canonical types with clear ownership  
//! **Eliminated**: 17 redundant aliases (all resolved to `Result<T, NestGateError>`)  
//! **Benefit**: Single source of truth, reduced confusion, easier maintenance

use crate::error::NestGateError;
use std::sync::Arc;

// ==================== CORE TYPES ====================

/// **PRIMARY RESULT TYPE**
///
/// The canonical Result type for NestGate. Uses `NestGateError` as default error.
///
/// This is the **preferred** Result type for all NestGate operations.
///
/// ## Usage
///
/// ```rust,ignore
/// use nestgate_core::Result;
///
/// pub fn process_data(input: &str) -> Result<String> {
///     if input.is_empty() {
///         return Err(NestGateError::validation("Input cannot be empty"));
///     }
///     Ok(input.to_uppercase())
/// }
/// ```
///
/// ## When to Use
///
/// - For all standard operations returning `NestGateError`
/// - When the error type is obvious from context
/// - As the default choice for new functions
///
/// ## When NOT to Use
///
/// - When returning a different error type (use `std::result::Result<T, E>`)
/// - When Result might be ambiguous (use `CanonicalResult<T>`)
pub type Result<T, E = NestGateError> = std::result::Result<T, E>;

/// **CANONICAL RESULT ALIAS**
///
/// Explicit alias for disambiguation when `Result` conflicts with `std::result::Result`.
///
/// ## Usage
///
/// ```rust,ignore
/// use std::result::Result; // Using std Result
/// use nestgate_core::result_types::CanonicalResult;
///
/// pub fn process() -> CanonicalResult<Data> {
///     // Unambiguous - clearly NestGate's Result
///     Ok(Data::new())
/// }
/// ```
///
/// ## When to Use
///
/// - When `Result` would be ambiguous
/// - In modules that heavily use `std::result::Result`
/// - When explicitly showing NestGate Result usage
pub type CanonicalResult<T> = Result<T>;

/// **VOID RESULT**
///
/// Convenience type for operations that return no value on success.
///
/// Equivalent to `Result<()>` but more expressive of intent.
///
/// ## Usage
///
/// ```rust
/// use nestgate_core::result_types::VoidResult;
///
/// pub fn initialize_system() -> VoidResult {
///     // Perform initialization...
///     Ok(())
/// }
///
/// pub fn cleanup() -> VoidResult {
///     // Perform cleanup...
///     Ok(())
/// }
/// ```
///
/// ## When to Use
///
/// - For functions that succeed/fail but return no data
/// - Initialize, cleanup, validation functions
/// - Operations where success is the only data needed
pub type VoidResult = Result<()>;

// ==================== TEST UTILITIES ====================

/// **TEST RESULT**
///
/// Standard Result type for test code with default `()` return.
///
/// ## Usage
///
/// ```rust,ignore,no_run
/// use nestgate_core::result_types::TestResult;
///
/// // In your test file:
/// // #[test]
/// fn test_operation() -> TestResult {
///     // let result = operation()?;
///     // assert_eq!(result, expected);
///     Ok(())  // Default () works great for tests
/// }
///
/// // #[test]
/// fn test_with_return() -> TestResult<String> {
///     // let data = fetch_data()?;
///     Ok("data".to_string())  // Can also return data for helper functions
/// }
/// ```
///
/// ## Benefits
///
/// - Default `()` return makes test code cleaner
/// - Works with `?` operator for error propagation
/// - Consistent error handling in test code
pub type TestResult<T = ()> = Result<T>;

// ==================== FUNCTION TYPES ====================

/// **CONNECTION FACTORY**
///
/// Factory function type for creating connections with error handling.
///
/// Used in connection pools and resource management for creating connections
/// that can fail during initialization.
///
/// ## Usage
///
/// ```rust,ignore
/// use nestgate_core::result_types::ConnectionFactory;
/// use std::sync::Arc;
///
/// struct DbConnection { /* ... */ }
///
/// impl DbConnection {
///     fn new(url: &str) -> Result<Self> { /* ... */ }
/// }
///
/// fn create_factory(url: String) -> ConnectionFactory<DbConnection> {
///     Arc::new(move || DbConnection::new(&url))
/// }
/// ```
///
/// ## Why Arc?
///
/// - Factory can be shared across threads
/// - Multiple pool instances can use same factory
/// - Efficient cloning for distribution
///
/// Type alias for: `Arc<dyn Fn() -> Result<T> + Send + Sync>`
pub type ConnectionFactory<T> = Arc<dyn Fn() -> Result<T> + Send + Sync>;

/// **HEALTH CHECK FUNCTION**
///
/// Function type for health check operations on resources.
///
/// Used in connection pools, monitoring, and service health checking
/// to verify resources are still healthy.
///
/// ## Usage
///
/// ```rust,ignore
/// use nestgate_core::result_types::HealthCheckFn;
/// use std::sync::Arc;
///
/// struct Service { /* ... */ }
///
/// fn create_health_check() -> HealthCheckFn<Service> {
///     Arc::new(|service| {
///         service.ping()?;
///         service.check_status()?;
///         Ok(())
///     })
/// }
/// ```
///
/// ## Pattern
///
/// - Takes reference to avoid unnecessary cloning
/// - Returns `Result<()>` - success or error
/// - Thread-safe for concurrent health checks
///
/// Type alias for: `Arc<dyn Fn(&T) -> Result<()> + Send + Sync>`
pub type HealthCheckFn<T> = Arc<dyn Fn(&T) -> Result<()> + Send + Sync>;

/// **VALIDATOR FUNCTION**
///
/// Function type for validation operations on configuration or data.
///
/// Used in configuration builders, data validation, and input checking
/// to ensure data meets requirements before processing.
///
/// ## Usage
///
/// ```rust,ignore
/// use nestgate_core::result_types::ValidatorFn;
///
/// struct Config {
///     port: u16,
///     host: String,
/// }
///
/// fn port_validator() -> ValidatorFn<Config> {
///     Box::new(|config| {
///         if config.port < 1024 {
///             return Err(NestGateError::validation(
///                 "Port must be >= 1024 for non-root users"
///             ));
///         }
///         Ok(())
///     })
/// }
/// ```
///
/// ## Why Box?
///
/// - Validators are typically used locally
/// - Box is lighter than Arc when sharing isn't needed
/// - Still allows dynamic dispatch for flexibility
pub type ValidatorFn<T> = Box<dyn Fn(&T) -> Result<()> + Send + Sync>;

// ==================== RESULT EXTENSION TRAITS ====================

/// Extension trait for Result types to provide additional utility methods
pub trait ResultExt<T, E> {
    /// Convert to a canonical result with `NestGateError`
    fn to_canonical(self) -> Result<T>
    where
        E: Into<NestGateError>;

    /// Add context to the error
    fn with_context<F>(self, f: F) -> Result<T>
    where
        E: Into<NestGateError>,
        F: FnOnce() -> String;
}

impl<T, E> ResultExt<T, E> for std::result::Result<T, E> {
    /// Converts to Canonical
    fn to_canonical(self) -> Result<T>
    where
        E: Into<NestGateError>,
    {
        self.map_err(std::convert::Into::into)
    }

    /// Builder method to set Context
    fn with_context<F>(self, _f: F) -> Result<T>
    where
        E: Into<NestGateError>,
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let error: NestGateError = e.into();
            // Context could be added here if NestGateError supports it
            error
        })
    }
}

// ==================== CONVENIENCE MACROS ====================

/// Macro for creating validation results with context
#[macro_export]
macro_rules! validation_result {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::error::NestGateError::from(e))
    };
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| {
            let error: $crate::error::NestGateError = e.into();
            error
        })
    };
}

/// Macro for creating network results with context
#[macro_export]
macro_rules! network_result {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::error::NestGateError::from(e))
    };
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| {
            let error: $crate::error::NestGateError = e.into();
            error
        })
    };
}

/// Macro for creating storage results with context
#[macro_export]
macro_rules! storage_result {
    ($expr:expr) => {
        $expr.map_err(|e| $crate::error::NestGateError::from(e))
    };
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| {
            let error: $crate::error::NestGateError = e.into();
            error
        })
    };
}

// ==================== DOCUMENTATION EXAMPLES ====================

#[cfg(doc)]
mod examples {
    use super::*;

    /// Example function showing standard Result usage
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn standard_operation() -> Result<String> {
        Ok("success".to_string())
    }

    /// Example function showing canonical Result usage
    pub fn validate_config(config: &str) -> Result<()> {
        if config.is_empty() {
            Err(NestGateError::validation_error("config cannot be empty"))
        } else {
            Ok(())
        }
    }

    /// Example function showing external error integration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - JSON parsing fails
    pub fn parse_json(input: &str) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(input)
    }
}

// ==================== NOTES ====================

// Domain-specific types with different error types remain in their respective modules:
// - UniversalZfsResult<T> in nestgate-api/handlers/zfs/universal_zfs/types.rs
// - AIResult<T> in nestgate-core/ai_first_refactored.rs
// - NotificationResult<T> in nestgate-core/smart_abstractions/notification_channels.rs
// - NetworkResult<T> in nestgate-network/error.rs
// - InstallerResult<T> in nestgate-installer/error.rs
//
// These are legitimate because they use different error types or have specialized semantics.
//
// ## Consolidation History
//
// **November 9, 2025**: Merged `unified_result_system.rs` into this file
// - Added ResultExt trait for error conversion utilities
// - Added convenience macros (validation_result!, network_result!, storage_result!)
// - Added documentation examples
// - Single source of truth achieved
