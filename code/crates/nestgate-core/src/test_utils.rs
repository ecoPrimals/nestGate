use crate::error::NestGateError;
//
// Common utilities and helpers for testing NestGate components.

use crate::{Result};
use std::path::PathBuf;

/// Create a temporary directory for testing
pub fn create_temp_dir(prefix: &str) -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir().join("nestgate-tests").join(format!(
        "{}_{}",
        prefix,
        uuid::Uuid::new_v4()
    ));

    std::fs::create_dir_all(&temp_dir).map_err(|e| NestGateError::Internal {
        message: format!("Failed to create test directory: {e}"),
        location: Some("test_utils.rs:17".to_string()),
        location: Some("test_setup".to_string()),
        is_bug: false,
    })?;

    Ok(temp_dir)
}

/// Create an internal error for testing
pub fn internal_error(message: &str, context: &str) -> NestGateError {
    NestGateError::Internal {
        message: message.to_string(),
        location: Some("test_utils.rs:28".to_string()),
        location: Some(context.to_string()),
        is_bug: false,
    }
}
