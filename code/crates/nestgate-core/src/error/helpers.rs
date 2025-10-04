//! **ERROR HANDLING HELPERS**
//!
//! Helper functions for common error handling patterns to reduce unwrap usage.

use super::NestGateError;

/// Safe string conversion that never panics
pub fn safe_to_string<T: std::fmt::Display>(value: T) -> String {
    format!("{value}")
}

/// Safe environment variable access
pub fn safe_env_var(key: &str) -> Result<String, NestGateError> {
    std::env::var(key).map_err(|e| {
        NestGateError::configuration_error(key, &format!("Environment variable not found: {e}"))
    })
}

/// Safe environment variable access with default
pub fn safe_env_var_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Safe file operations
pub fn safe_read_to_string(path: &std::path::Path) -> Result<String, NestGateError> {
    std::fs::read_to_string(path).map_err(|e| {
        NestGateError::io_error(format!("Failed to read file {}: {}", path.display(), e))
    })
}

/// Safe JSON parsing
pub fn safe_json_parse<T: serde::de::DeserializeOwned>(content: &str) -> Result<T, NestGateError> {
    serde_json::from_str(content)
        .map_err(|e| NestGateError::validation_error(&format!("JSON parsing failed: {e}")))
}

/// Safe mutex lock (returns error instead of poisoning)
pub fn safe_lock<T>(
    mutex: &std::sync::Mutex<T>,
) -> Result<std::sync::MutexGuard<'_, T>, NestGateError> {
    mutex
        .lock()
        .map_err(|e| NestGateError::internal_error(format!("Mutex lock failed: {e}"), "mutex"))
}

/// Safe channel send
pub fn safe_send<T>(sender: &std::sync::mpsc::Sender<T>, value: T) -> Result<(), NestGateError> {
    sender
        .send(value)
        .map_err(|e| NestGateError::internal_error(format!("Channel send failed: {e}"), "channel"))
}
