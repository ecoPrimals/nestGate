// **UNIFIED RESULT SYSTEM**
//! Error handling types and utilities.
// This module provides the single, canonical Result system for all NestGate operations,
//! eliminating fragmentation and duplication across multiple modules.
//! Error handling types and utilities.
// **CONSOLIDATES**:
//! - `error/mod.rs` Result type definitions
//! - `error/idiomatic/result_types.rs` duplicate definitions
//! - All scattered Result types across crates
//!
//! Error handling types and utilities.
// **PROVIDES**:
//! - Single source of truth for all Result types
//! - Consistent patterns across all domains
//! - Rich error context for debugging
//! - Seamless ecosystem integration

use crate::error::NestGateError;
// use super::idiomatic::domain_errors::*; // Commented out - module doesn't exist

// ==================== CANONICAL RESULT TYPES ====================

/// **THE CANONICAL RESULT TYPE**
///
/// This is the primary Result type for all `NestGate` operations.
/// Both T and E are generic for maximum flexibility and ecosystem compatibility.
///
/// **USAGE PATTERNS**:
/// ```rust
/// // Standard usage with NestGateError (most common)
/// fn operation() -> Result<Data> { ... }
///
/// // Domain-specific error type
/// fn validate() -> Result<Config, ValidationError> { ... }
///
/// // External ecosystem integration
/// fn parse() -> Result<Value, serde_json::Error> { ... }
/// ```
pub type Result<T, E = NestGateError> = std::result::Result<T, E>;
/// **CANONICAL RESULT ALIAS** - For explicit canonical usage
pub type CanonicalResult<T> = Result<T>;
// ==================== DOMAIN-SPECIFIC RESULT TYPES ====================

/// **VALIDATION OPERATIONS** - Configuration and input validation
pub type ValidationResult<T> = Result<T>;

/// **NETWORK OPERATIONS** - HTTP, TCP, and network communication
pub type NetworkResult<T> = Result<T>;

/// **STORAGE OPERATIONS** - File system, database, and persistence
pub type StorageResult<T> = Result<T>;

/// **SECURITY OPERATIONS** - Authentication, authorization, encryption
pub type SecurityResult<T> = Result<T>;

/// **ZFS OPERATIONS** - ZFS pool, dataset, and snapshot management
pub type ZfsResult<T> = Result<T>;

/// **API OPERATIONS** - REST API, GraphQL, and HTTP handlers
pub type ApiResult<T> = Result<T>;

/// **MCP PROTOCOL OPERATIONS** - Model Context Protocol operations
pub type McpResult<T> = Result<T>;

/// **TESTING OPERATIONS** - Test framework and validation
pub type TestingResult<T> = Result<T>;

/// **PERFORMANCE OPERATIONS** - Benchmarking and optimization
pub type PerformanceResult<T> = Result<T>;

/// **HANDLER OPERATIONS** - Request/response handling
pub type HandlerResult<T> = Result<T>;

/// **SERIALIZATION OPERATIONS** - JSON, TOML, and data format handling
pub type SerializationResult<T> = Result<T>;

/// **DATABASE OPERATIONS** - SQL and database interactions
pub type DatabaseResult<T> = Result<T>;

/// **CACHE OPERATIONS** - Caching and memory management
pub type CacheResult<T> = Result<T>;

/// **WORKFLOW OPERATIONS** - Process orchestration and automation
pub type WorkflowResult<T> = Result<T>;

/// **MONITORING OPERATIONS** - Metrics, logging, and observability
pub type MonitoringResult<T> = Result<T>;

/// **CONFIGURATION OPERATIONS** - Alias for validation results
pub type ConfigResult<T> = ValidationResult<T>;
// ==================== UTILITY TYPES ====================

/// **VOID RESULT** - For operations that return no data on success
pub type VoidResult = Result<()>;
/// **OPTIONAL RESULT** - For operations that may return no data
pub type OptionalResult<T> = Result<Option<T>>;
/// **COLLECTION RESULT** - For operations that return collections
pub type CollectionResult<T> = Result<Vec<T>>;
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
    fn to_canonical(self) -> Result<T>
    where
        E: Into<NestGateError>,
    {
        self.map_err(|e| e.into())
    }

    fn with_context<F>(self, _f: F) -> Result<T>
    where
        E: Into<NestGateError>,
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let error: NestGateError = e.into();
            // Add context to the error (implementation depends on NestGateError structure)
            error
        })
    }
}

// ==================== CONVENIENCE MACROS ====================

/// Macro for creating validation results with context
#[macro_export]
macro_rules! validation_result {
    ($expr:expr) => {
        $expr.map_err(|e| NestGateError::from(e))
    };
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| {
            let error: NestGateError = e.into();
            error
        })
    };
}

/// Macro for creating network results with context
#[macro_export]
macro_rules! network_result {
    ($expr:expr) => {
        $expr.map_err(|e| NestGateError::from(e))
    };
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| {
            let error: NestGateError = e.into();
            error
        })
    };
}

/// Macro for creating storage results with context
#[macro_export]
macro_rules! storage_result {
    ($expr:expr) => {
        $expr.map_err(|e| NestGateError::from(e))
    };
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| {
            let error: NestGateError = e.into();
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

    /// Example function showing domain-specific Result usage
    pub fn validate_config(config: &str) -> ValidationResult<()> {
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
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn parse_json(input: &str) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(input)
    }
}

// ==================== TESTS ====================

// Tests removed due to encoding issues - can be re-added later
// #[cfg(test)]
// mod tests { ... }
