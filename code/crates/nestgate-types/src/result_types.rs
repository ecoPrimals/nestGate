// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL RESULT TYPES**
//!
//! Single source of truth for all Result type aliases in `NestGate`.
//!
//! ## Purpose
//!
//! This module consolidates 17 redundant Result type aliases into a clean,
//! canonical set of 12-14 types. This eliminates confusion and provides
//! clear patterns for error handling across the codebase.
//!
//! ## Usage Guidelines
//!
//! - **Prefer**: `Result` with your value type for most cases
//! - **Use**: `crate::error::CanonicalResult` when disambiguating from `std::result::Result`
//! - **Domain-specific**: Use specialized types for different error types
//! - **Tests**: Use `crate::error::TestResult` in test code
//! - **Functions**: Use `ConnectionFactory`, `HealthCheckFn`, `ValidatorFn`
//!
//! ## Migration from Legacy Types
//!
//! All domain-specific Result aliases (`ApiResult`, `CacheResult`, `StorageResult`, etc.)
//! have been deprecated in v0.11.2 and will be removed in v0.12.0 (May 2026).
//!
//! Use `nestgate_types::Result` (or `nestgate_types::error::Result`) with your concrete `Ok` type directly:
//!
//! ```rust,ignore
//! // OLD (deprecated):
//! pub fn fetch_data() -> ApiResult<Data> { todo!() }
//!
//! // NEW (canonical):
//! use nestgate_types::Result;
//!
//! pub fn fetch_data() -> Result<Data> { todo!() }
//! ```
//!
//! ## Consolidation Summary
//!
//! **Before**: 54 Result type aliases scattered across codebase\
//! **After**: 12-14 canonical types with clear ownership\
//! **Eliminated**: 17 redundant aliases (all resolved to `Result` with `NestGateError`)\
//! **Benefit**: Single source of truth, reduced confusion, easier maintenance

use crate::error::NestGateError;
use std::sync::Arc;

// ==================== CORE TYPES ====================

/// **PRIMARY RESULT TYPE**
///
/// The canonical Result type for `NestGate`. Uses `NestGateError` as default error.
///
/// This is the **preferred** Result type for all `NestGate` operations.
///
/// ## Usage
///
/// ```rust,ignore
/// use nestgate_types::Result;
///
/// pub fn process_data(input: &str) -> Result<String> {
///     if input.is_empty() {
///         return Err(NestGateError::validation_error("Input cannot be empty"));
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
/// - When returning a different error type (use `std::result::Result` with your `E`)
/// - When Result might be ambiguous (use `crate::error::CanonicalResult`)
pub type Result<T, E = NestGateError> = std::result::Result<T, E>;

/// **VOID RESULT**
///
/// Convenience type for operations that return no value on success.
///
/// Equivalent to `Result` with unit (`()`) success type but more expressive of intent.
///
/// ## Usage
///
/// ```rust
/// use nestgate_types::result_types::VoidResult;
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
/// use nestgate_types::result_types::ConnectionFactory;
/// use nestgate_types::Result;
/// use std::sync::Arc;
///
/// struct DbConnection;
///
/// impl DbConnection {
///     fn new(_url: &str) -> Result<Self> {
///         Ok(Self)
///     }
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
/// Type alias for `Arc` wrapping a `Fn` that returns `Result` (see `ConnectionFactory` definition).
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
/// use nestgate_types::result_types::HealthCheckFn;
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
/// - Returns unit `Result` — success or error
/// - Thread-safe for concurrent health checks
///
/// Type alias for `Arc` wrapping a `Fn` that takes `&T` and returns unit `Result` (see `HealthCheckFn` definition).
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
/// use nestgate_types::result_types::ValidatorFn;
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
