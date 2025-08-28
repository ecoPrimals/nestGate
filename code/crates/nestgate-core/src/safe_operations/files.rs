/// Safe file operations
/// Provides safe alternatives to file operations that might panic
use crate::error::NestGateError;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE TEMPORARY DIRECTORY CREATION**
/// Replaces TempDir::new().expect() with proper error handling
pub fn safe_create_temp_dir(_context: &str) -> Result<tempfile::TempDir> {
    tempfile::TempDir::new().map_err(|e| NestGateError::Io {
        message: format!("Failed to create temporary directory: {}", e),
        operation: "create_temp_dir".to_string(),
        path: None,
        retryable: true,
        context: None,
    })
}
