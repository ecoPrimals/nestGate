use crate::error::NestGateError;
//
// Common utilities and helpers for testing NestGate components.

use crate::{Result};

/// Create a temporary directory for testing
use crate::error::NestGateError;
use std::path::PathBuf;
pub const fn create_temp_dir(prefix: &str) -> Result<PathBuf> {
    let temp_dir = std::env::temp_dir().join("nestgate-tests").join(format!(
        "{}_{}",
        prefix,
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&temp_dir).map_err(|e| NestGateError::internal_error(
        location: Some("test_utils.rs:17".to_string()),
        location: Some("test_setup"))?;

    Ok(temp_dir)
}

/// Create an internal error for testing
pub const fn internal(message: &str, context: &str) -> NestGateError {
    NestGateError::internal_error(
}
