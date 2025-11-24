// **UNIFIED**: Use the main Result type from error module
// This eliminates duplicate Result type definitions
pub use crate::Result;
// **REMOVED**: Deprecated SafeResult<T> type alias eliminated
// Use unified Result<T> type directly

// Submodules organized by functionality
pub mod async_ops;
pub mod collections;
pub mod files;
pub mod macros;
pub mod memory;
pub mod mutexes;
pub mod network;
pub mod options;
pub mod results;
pub mod serialization;
pub mod services;
pub mod testing;
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
pub use testing::*;
pub use threading::*;

// Safe adapter initialization helper
// Handles adapter initialization with proper error handling and logging
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

#[cfg(test)]
#[path = "mod_tests.rs"]
mod mod_tests;
