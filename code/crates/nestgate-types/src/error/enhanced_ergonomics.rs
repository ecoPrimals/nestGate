// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Enhanced ergonomics for the unified error system (macros and helpers).

use crate::error::NestGateError;
use crate::error::Result;

/// **ERGONOMIC ERROR CREATION MACROS**
/// These macros make error creation more convenient while preserving all the
/// rich context and debugging information of our unified error system.
/// Create a simple error with automatic location tracking
#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        $crate::error::NestGateError::simple($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::NestGateError::simple(format!($fmt, $($arg)*))
    };
}

/// **DEPRECATED**: Use the idiomatic `config_error!` macro from unified module
///
/// This macro has been moved to the unified error system.
/// Use `NestGateError::config_error()` method instead.
// Removed duplicate config_error macro - use unified error system methods

/// **DEPRECATED**: Use the idiomatic `network_error!` macro from idiomatic_evolution module
#[macro_export]
macro_rules! legacy_network_error {
    ($operation:expr, $details:expr) => {
        $crate::error::NestGateError::network_error($operation, $details)
    };
    ($operation:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::error::NestGateError::network_error($operation, format!($fmt, $($arg)*))
    };
}

/// Create a storage error with operation context (LEGACY - Use idiomatic `storage_error!` instead)
/// **DEPRECATED**: Use the idiomatic `storage_error!` macro from idiomatic_evolution module
#[macro_export]
macro_rules! legacy_storage_error {
    ($operation:expr, $details:expr) => {
        $crate::error::NestGateError::storage_error($operation, $details)
    };
    ($operation:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::error::NestGateError::storage_error($operation, format!($fmt, $($arg)*))
    };
}

/// **RESULT EXTENSION TRAITS FOR ENHANCED ERGONOMICS**
/// Enhanced result extensions for better error handling ergonomics
pub trait EnhancedResultExt<T> {
    /// Convert to `NestGateError` with context string
    fn with_context(self, context: impl Into<String>) -> Result<T>
    where
        Self: Sized;

    /// Map error to `NestGateError`
    fn map_nestgate_err<F>(self, f: F) -> Result<T>
    where
        Self: Sized,
        F: FnOnce() -> NestGateError;

    /// Add operation context to error
    fn with_operation(self, operation: &str) -> Result<T>
    where
        Self: Sized;

    /// Add resource context to error
    fn with_resource(self, path: &str) -> Result<T>
    where
        Self: Sized;
}

impl<T, E> EnhancedResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_context(self, context: impl Into<String>) -> Result<T> {
        let ctx = context.into();
        self.map_err(|e| NestGateError::validation_error(&format!("{ctx}: {e}")))
    }

    fn map_nestgate_err<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> NestGateError,
    {
        self.map_err(|_| f())
    }

    fn with_operation(self, operation: &str) -> Result<T> {
        self.with_context(format!("Operation '{operation}' failed"))
    }

    fn with_resource(self, path: &str) -> Result<T> {
        self.with_context(format!("Resource '{path}' error"))
    }
}

/// **SAFE OPERATION UTILITIES**
/// These utilities help eliminate unsafe patterns like `.unwrap()` and `.expect("Operation failed")`
/// Safely extract a value from a mutex, recovering from poisoning
pub fn safe_mutex_lock<'a, T>(
    mutex: &'a std::sync::Mutex<T>,
    context: &str,
) -> Result<std::sync::MutexGuard<'a, T>> {
    mutex.lock().map_err(|_poisoned| {
        tracing::warn!("Mutex poisoned in {context}, recovering gracefully");
        NestGateError::internal_error(
            format!("Mutex poisoned in {context}"),
            "enhanced_ergonomics".to_string(),
        )
    })
}

/// Safely extract a value from a `RwLock`, recovering from poisoning
pub fn safe_rwlock_read<'a, T>(
    rwlock: &'a std::sync::RwLock<T>,
    context: &str,
) -> Result<std::sync::RwLockReadGuard<'a, T>> {
    rwlock.read().map_err(|_poisoned| {
        tracing::warn!("RwLock poisoned in {context}, recovering gracefully");
        NestGateError::internal_error(
            format!("RwLock poisoned in {context}"),
            "enhanced_ergonomics".to_string(),
        )
    })
}

/// Safely extract a write guard from a `RwLock`, recovering from poisoning
pub fn safe_rwlock_write<'a, T>(
    rwlock: &'a std::sync::RwLock<T>,
    context: &str,
) -> Result<std::sync::RwLockWriteGuard<'a, T>> {
    rwlock.write().map_err(|_poisoned| {
        tracing::warn!("RwLock write poisoned in {context}, recovering gracefully");
        NestGateError::internal_error(
            format!("RwLock write poisoned in {context}"),
            "enhanced_ergonomics".to_string(),
        )
    })
}

/// **ENHANCED ERROR DISPLAY UTILITIES**
/// Format an error chain for user-friendly display
pub fn format_error_chain(error: &NestGateError) -> String {
    let mut chain = Vec::new();
    let mut current_error: &dyn std::error::Error = error;
    loop {
        chain.push(current_error.to_string());

        match current_error.source() {
            Some(source) => current_error = source,
            None => break,
        }
    }

    if chain.len() == 1 {
        chain[0].clone()
    } else {
        format!("{}\nCaused by:\n{}", chain[0], chain[1..].join("\n  "))
    }
}

/// Create a development-friendly error report
pub fn debug_error_report(error: &NestGateError) -> String {
    format!(
        "🚨 ERROR REPORT 🚨\n\
         Error: {error}\n\
         Type: {}\n\
         Chain: {}\n\
         Debug Info: Available in structured logs",
        std::any::type_name::<NestGateError>(),
        format_error_chain(error)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_macros() {
        let simple_error = error!("test error");
        assert!(simple_error.to_string().contains("test error"));

        let formatted_error = error!("test error with value: {}", 42);
        assert!(formatted_error.to_string().contains("42"));
    }

    #[test]
    fn test_result_extensions() {
        let result: std::result::Result<i32, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));

        let with_context = result.with_context("loading configuration file");
        assert!(with_context.is_err());
        assert!(with_context
            .expect_err("expected err")
            .to_string()
            .contains("loading configuration file"));
    }

    #[test]
    fn test_error_formatting() {
        let error = NestGateError::simple("test error");
        let formatted = format_error_chain(&error);
        assert!(formatted.contains("test error"));

        let report = debug_error_report(&error);
        assert!(report.contains("ERROR REPORT"));
    }
}
