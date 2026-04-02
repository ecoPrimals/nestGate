// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **UNIFIED**: Use the main Result type from error module
// This eliminates duplicate Result type definitions
//! Safe Operations module

pub use crate::Result;
// **REMOVED**: Deprecated SafeResult<T> type alias eliminated
// Use unified Result<T> type directly

// Submodules organized by functionality
/// Safe async operation utilities and patterns
pub mod async_ops;
/// Safe collection manipulation helpers
pub mod collections;
/// Safe file operations with proper error handling
pub mod files;
/// Safety-focused macros for common patterns
pub mod macros;
/// Memory safety utilities and zero-copy patterns
pub mod memory;
/// Safe mutex wrappers with timeout support
pub mod mutexes;
/// Network safety helpers and timeout patterns
pub mod network;
/// Safe Option handling utilities
pub mod options;
/// Safe Result handling and error composition utilities
pub mod results;
/// Safe serialization helpers with validation
pub mod serialization;
/// Service safety utilities and health checking
pub mod services;
#[cfg(any(test, feature = "dev-stubs"))]
pub mod testing;
/// Thread safety utilities and synchronization patterns
pub mod threading;

// Re-export commonly used functions
pub use async_ops::*;
pub use collections::*;
pub use files::*;
pub use memory::*;
pub use mutexes::*;
pub use network::*;
pub use options::*;
pub use results::*;
pub use serialization::*;
pub use services::*;
#[cfg(any(test, feature = "dev-stubs"))]
pub use testing::*;
pub use threading::*;

// Safe adapter initialization helper
// Handles adapter initialization with proper error handling and logging
/// Initialize an adapter with safe error handling and logging
///
/// Wraps adapter initialization to provide consistent error handling and optional fallback.
///
/// # Arguments
///
/// * `init_result` - The result of the adapter initialization attempt
/// * `adapter_name` - Name of the adapter for logging and error messages
///
/// # Returns
///
/// * `Ok(Some(T))` - Adapter initialized successfully
/// * `Ok(None)` - Adapter initialization failed but was handled gracefully
/// * `Err(_)` - Critical error that should propagate
///
/// # Examples
///
/// ```ignore
/// let adapter = safe_adapter_init(
///     StorageAdapter::new(&config),
///     "storage"
/// )?;
/// ```
pub fn safe_adapter_init<T>(init_result: Result<T>, adapter_name: &str) -> Result<Option<T>> {
    match init_result {
        Ok(adapter) => {
            tracing::info!("✅ {} initialized successfully", adapter_name);
            Ok(Some(adapter))
        }
        Err(e) => {
            tracing::warn!("⚠️ {} initialization failed: {}", adapter_name, e);
            // Return None instead of error to allow graceful degradation
            Ok(None)
        }
    }
}

/// Safe connection pool return operation
///
/// Wraps connection pool return operations with proper error handling
/// to prevent panics and provide meaningful error messages.
pub fn safe_connection_pool_return<T>(result: Result<T>, operation: &str) -> Result<Result<T>> {
    match result {
        Ok(value) => Ok(Ok(value)),
        Err(e) => {
            tracing::warn!("Connection pool operation '{}' failed: {}", operation, e);
            Ok(Err(e))
        }
    }
}
