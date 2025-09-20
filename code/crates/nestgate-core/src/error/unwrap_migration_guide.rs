//! **`UNWRAP()` MIGRATION GUIDE AND UTILITIES**
//!
//! Comprehensive guide for migrating `unwrap()` calls to production-ready error handling.
//!
//! **ELIMINATES**:
//! - panic!() calls in production code paths
//! - Unsafe `unwrap()` operations that can crash the system
//! - Hidden error conditions that cause unexpected failures
//!
//! **PROVIDES**:
//! - Safe error handling patterns with descriptive context
//! - Migration utilities for automated `unwrap()` replacement
//! - Production-ready error recovery strategies
//! - Comprehensive error reporting and logging

use crate::error::{NestGateError, Result};
use std::fmt;

// ==================== UNWRAP MIGRATION PATTERNS ====================

/// **UNWRAP MIGRATION PATTERNS**
///
/// Safe alternatives to `unwrap()` calls with proper error handling
pub struct UnwrapMigrationPatterns;

impl UnwrapMigrationPatterns {
    /// Replace `option.unwrap()` with safe error handling
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn safe_option_unwrap<T>(option: Option<T>, context: &str) -> Result<T>  {
        option.ok_or_else(|| {
            NestGateError::internal_error(
                format!("Missing required value: {context}"),
                "unwrap_migration",
            )
        })
    }

    /// Replace `result.unwrap()` with safe error handling
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn safe_result_unwrap<T, E: fmt::Display>(
        result: std::result::Result<T, E>,
        context: &str,
    ) -> Result<T>  {
        result.map_err(|e| {
            NestGateError::internal_error(
                format!("Operation failed in {context}: {e}"),
                "unwrap_migration",
            )
        })
    }

    /// Replace `option.unwrap_or_default()` with explicit default handling
    pub fn safe_option_with_default<T: Default>(option: Option<T>, context: &str) -> T {
        option.unwrap_or_else(|| {
            tracing::warn!("Using default value for missing {}", context);
            T::default()
        })
    }

    /// Replace `unwrap()` in tests with descriptive `expect()`
    pub fn test_expect<T, E: fmt::Debug>(
        result: std::result::Result<T, E>,
        test_context: &str,
    ) -> T {
        result.unwrap_or_else(|e| panic!("Test failure in {test_context}: {e:?}"))
    }
}

// ==================== MIGRATION UTILITIES ====================

/// **UNWRAP MIGRATION GUIDE**
pub const UNWRAP_MIGRATION_GUIDE: &str = r#"
🔄 UNWRAP() MIGRATION GUIDE

## Before (Unsafe unwrap() calls)
```rust
let value = option.unwrap();                    // ❌ Can panic
let result = operation().unwrap();              // ❌ Can panic
let config = env::var("KEY").unwrap();          // ❌ Can panic
```

## After (Safe error handling)
```rust
let value = UnwrapMigrationPatterns::safe_option_unwrap(
    option, 
    "configuration_value"
)?;                                             // ✅ Safe with context

let result = UnwrapMigrationPatterns::safe_result_unwrap(
    operation(), 
    "critical_operation"
)?;                                             // ✅ Safe with context

let config = env::var("KEY").map_err(|e| {
    NestGateError::configuration_error(
        format!("Missing required environment variable KEY: {e}")
    )
})?;                                            // ✅ Safe with detailed error
```

## Test Code (Use expect with context)
```rust
let adapter = UniversalAdapter::new()
    .expect("Failed to create adapter in test");   // ✅ Safe for tests
```

## Performance Benefits
- ✅ Eliminates unexpected panics in production
- ✅ Provides clear error context for debugging
- ✅ Enables graceful error recovery
- ✅ Improves system reliability and stability
"#;

// ==================== PRODUCTION ERROR HANDLING PATTERNS ====================

/// **PRODUCTION-READY ERROR HANDLING**
///
/// Safe wrappers for common `unwrap()` patterns in production code
pub struct ProductionErrorHandling;

impl ProductionErrorHandling {
    /// Safe environment variable access
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn safe_env_var(key: &str) -> Result<String>  {
        std::env::var(key).map_err(|e| {
            NestGateError::configuration_error(
                key,
                &format!("Missing required environment variable {key}: {e}"),
            )
        })
    }

    /// Safe environment variable with default
    #[must_use]
    pub const fn safe_env_var_with_default(key: &str, default: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| {
            tracing::info!("Using default value for {}: {}", key, default);
            default.to_string()
        })
    }

    /// Safe JSON parsing
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn safe_json_parse<T: serde::de::DeserializeOwned>(
        json_str: &str,
        context: &str,
    ) -> Result<T>  {
        serde_json::from_str(json_str).map_err(|e| {
            NestGateError::validation_error(&format!("Failed to parse JSON in {context}: {e}"))
        })
    }

    /// Safe file operations
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn safe_file_read(path: &std::path::Path) -> Result<String> {
        std::fs::read_to_string(path).map_err(|e| {
            NestGateError::internal_error(
                format!("Failed to read file {}: {}", path.display(), e),
                "file_operations",
            )
        })
    }

    /// Safe network address parsing
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn safe_socket_addr_parse(addr_str: &str) -> Result<std::net::SocketAddr>  {
        addr_str.parse().map_err(|e| {
            NestGateError::network_error(&format!("Invalid socket address {addr_str}: {e}"))
        })
    }
}

// ==================== AUTOMATED MIGRATION MACROS ====================

/// Macro for safe option unwrapping with context
#[macro_export]
macro_rules! safe_unwrap_option {
    ($option:expr, $context:expr) => {
        $crate::error::unwrap_migration_guide::UnwrapMigrationPatterns::safe_option_unwrap(
            $option, $context,
        )
    };
}

/// Macro for safe result unwrapping with context
#[macro_export]
macro_rules! safe_unwrap_result {
    ($result:expr, $context:expr) => {
        $crate::error::unwrap_migration_guide::UnwrapMigrationPatterns::safe_result_unwrap(
            $result, $context,
        )
    };
}

/// Macro for test expectations with context
#[macro_export]
macro_rules! test_expect {
    ($result:expr, $context:expr) => {
        $crate::error::unwrap_migration_guide::UnwrapMigrationPatterns::test_expect(
            $result, $context,
        )
    };
}

// ==================== MIGRATION VALIDATION ====================

/// **UNWRAP MIGRATION VALIDATOR**
///
/// Tools for validating that `unwrap()` calls have been properly migrated
pub struct UnwrapMigrationValidator;

impl UnwrapMigrationValidator {
    /// Check if code contains unsafe `unwrap()` patterns
    #[must_use]
    pub const fn has_unsafe_unwraps(code: &str) -> bool {
        // Look for unwrap() calls outside of test contexts
        let unwrap_count = code.matches(".unwrap()").count();
        let test_unwrap_count = code
            .lines()
            .filter(|line| line.contains("[test]") || line.contains("[tokio::test]"))
            .map(|_| code.matches(".unwrap()").count())
            .sum::<usize>();

        unwrap_count > test_unwrap_count
    }

    /// Generate migration suggestions for `unwrap()` patterns
    #[must_use]
    pub fn generate_migration_suggestions(code: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        if code.contains(".unwrap()") {
            suggestions
                .push("Replace .unwrap() with proper error handling using ? operator".to_string());
            suggestions.push("Use .expect() with descriptive messages in test code".to_string());
            suggestions.push("Consider .unwrap_or_default() for non-critical values".to_string());
        }

        if code.contains("env::var(") && code.contains(".unwrap()") {
            suggestions.push(
                "Use ProductionErrorHandling::safe_env_var() for environment variables".to_string(),
            );
        }

        if code.contains("serde_json::from_str(") && code.contains(".unwrap()") {
            suggestions.push(
                "Use ProductionErrorHandling::safe_json_parse() for JSON parsing".to_string(),
            );
        }

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_option_unwrap() {
        let some_value = Some("test");
        let result = UnwrapMigrationPatterns::safe_option_unwrap(some_value, "test value");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");

        let none_value: Option<&str> = None;
        let result = UnwrapMigrationPatterns::safe_option_unwrap(none_value, "missing value");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_result_unwrap() {
        let ok_result: std::result::Result<i32, &str> = Ok(42);
        let result = UnwrapMigrationPatterns::safe_result_unwrap(ok_result, "test operation");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        let err_result: std::result::Result<i32, &str> = Err("test error");
        let result = UnwrapMigrationPatterns::safe_result_unwrap(err_result, "failing operation");
        assert!(result.is_err());
    }

    #[test]
    fn test_production_error_handling() {
        // Test safe environment variable access
        std::env::set_var("TEST_VAR", "test_value");
        let result = ProductionErrorHandling::safe_env_var("TEST_VAR");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");

        // Test safe environment variable with default
        let value =
            ProductionErrorHandling::safe_env_var_with_default("NONEXISTENT_VAR", "default");
        assert_eq!(value, "default");
    }
}
