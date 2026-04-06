// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ERROR HANDLING UTILITIES**
//!
//! Consolidated error handling helpers and utilities.
//! This module combines safe operation wrappers and error constructor patterns.
//!
//! **Replaces** (REMOVED November 10, 2025):
//! - `error/helpers.rs` (safe operation wrappers) ✅
//! - `error/modernized_error_helpers.rs` (error constructors) ✅

use super::{NestGateError, NestGateUnifiedError};
use crate::{EnvSource, ProcessEnv, env_var_or_default};

// ==================== SAFE OPERATION WRAPPERS ====================

/// Safe string conversion that never panics
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::safe_to_string;
/// let result = safe_to_string(42);
/// assert_eq!(result, "42");
/// ```
#[must_use]
pub fn safe_to_string<T: std::fmt::Display>(value: T) -> String {
    format!("{value}")
}

/// Safe environment variable access
///
/// Returns an error if the environment variable is not found.
///
/// # Errors
///
/// Returns `NestGateError::Configuration` if the environment variable doesn't exist.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::safe_env_var;
/// let home = safe_env_var("HOME")?;
/// ```
pub fn safe_env_var(key: &str) -> Result<String, NestGateError> {
    safe_env_var_from_env_source(key, &ProcessEnv)
}

/// Like [`safe_env_var`], but reads from an injectable [`EnvSource`].
///
/// # Errors
///
/// Returns [`NestGateError::Configuration`] when `key` is not present in `env`.
pub fn safe_env_var_from_env_source(
    key: &str,
    env: &dyn EnvSource,
) -> Result<String, NestGateError> {
    env.get(key).ok_or_else(|| {
        NestGateError::configuration_error(
            key.to_string(),
            "Environment variable not found".to_string(),
        )
    })
}

/// Safe environment variable access with default value
///
/// Returns the default value if the environment variable is not found.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::safe_env_var_or_default;
/// let port = safe_env_var_or_default("PORT", "8080");
/// ```
#[must_use]
pub fn safe_env_var_or_default(key: &str, default: &str) -> String {
    safe_env_var_or_default_from_env_source(key, default, &ProcessEnv)
}

/// Like [`safe_env_var_or_default`], but reads from an injectable [`EnvSource`].
#[must_use]
pub fn safe_env_var_or_default_from_env_source(
    key: &str,
    default: &str,
    env: &dyn EnvSource,
) -> String {
    env_var_or_default(env, key, default)
}

/// Safe file read operation
///
/// Reads a file to string with proper error handling.
///
/// # Errors
///
/// Returns `NestGateError::Io` if the file cannot be read.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::safe_read_to_string;
/// use std::path::Path;
/// let content = safe_read_to_string(Path::new("config.toml"))?;
/// ```
pub fn safe_read_to_string(path: &std::path::Path) -> Result<String, NestGateError> {
    std::fs::read_to_string(path).map_err(|e| {
        NestGateError::io_error(format!("Failed to read file {}: {}", path.display(), e))
    })
}

/// Safe JSON parsing
///
/// Parses JSON with proper error handling.
///
/// # Errors
///
/// Returns `NestGateError::Validation` if JSON parsing fails.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::safe_json_parse;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Config { port: u16 }
///
/// let config: Config = safe_json_parse(r#"{"port": 8080}"#)?;
/// ```
pub fn safe_json_parse<T: serde::de::DeserializeOwned>(content: &str) -> Result<T, NestGateError> {
    serde_json::from_str(content)
        .map_err(|e| NestGateError::validation_error(format!("JSON parsing failed: {e}")))
}

/// Safe mutex lock (returns error instead of poisoning)
///
/// Acquires a mutex lock with proper error handling.
///
/// # Errors
///
/// Returns `NestGateError::Internal` if the mutex is poisoned.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::safe_lock;
/// use std::sync::Mutex;
///
/// let data = Mutex::new(42);
/// let guard = safe_lock(&data)?;
/// ```
pub fn safe_lock<T>(
    mutex: &std::sync::Mutex<T>,
) -> Result<std::sync::MutexGuard<'_, T>, NestGateError> {
    mutex
        .lock()
        .map_err(|e| NestGateError::internal_error(format!("Mutex lock failed: {e}"), "mutex"))
}

/// Safe channel send
///
/// Sends a value through a channel with proper error handling.
///
/// # Errors
///
/// Returns `NestGateError::Internal` if the channel is disconnected.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::safe_send;
/// use std::sync::mpsc::channel;
///
/// let (tx, rx) = channel();
/// safe_send(&tx, 42)?;
/// ```
pub fn safe_send<T>(sender: &std::sync::mpsc::Sender<T>, value: T) -> Result<(), NestGateError> {
    sender
        .send(value)
        .map_err(|e| NestGateError::internal_error(format!("Channel send failed: {e}"), "channel"))
}

// ==================== ERROR CONSTRUCTOR PATTERNS ====================

/// Create a storage error with modern pattern
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::storage_error;
/// return Err(storage_error("Failed to write data"));
/// ```
#[must_use]
pub fn storage_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::storage_error(message.into())
}

/// Create a configuration error with modern pattern
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::configuration_error;
/// return Err(configuration_error("Invalid port number"));
/// ```
#[must_use]
pub fn configuration_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::configuration_error("config", message.into())
}

/// Create a validation error with modern pattern
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::validation_error;
/// return Err(validation_error("Invalid email format"));
/// ```
#[must_use]
pub fn validation_error(message: impl Into<String>) -> NestGateUnifiedError {
    NestGateUnifiedError::validation(message.into())
}

/// Create an internal error with modern pattern
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::error::utilities::internal;
/// return Err(internal("Unexpected state", "auth_module"));
/// ```
#[must_use]
pub fn internal(message: impl Into<String>, component: &str) -> NestGateUnifiedError {
    NestGateUnifiedError::internal_error(message.into(), component.to_string())
}

// ==================== RE-EXPORTS FOR BACKWARD COMPATIBILITY ====================

// Note: Re-exports handled in mod.rs to avoid privacy issues

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_to_string() {
        assert_eq!(safe_to_string(42), "42");
        assert_eq!(safe_to_string("hello"), "hello");
    }

    #[test]
    fn test_safe_env_var_or_default() {
        let result = safe_env_var_or_default("NONEXISTENT_VAR_12345", "default");
        assert_eq!(result, "default");
    }

    #[test]
    fn test_error_constructors() {
        let err = storage_error("test");
        assert!(format!("{err:?}").contains("Storage"));

        let err = configuration_error("test");
        assert!(format!("{err:?}").contains("Configuration"));

        let err = validation_error("test");
        assert!(format!("{err:?}").contains("Validation"));

        let err = internal("test", "component");
        assert!(format!("{err:?}").contains("Internal"));
    }

    #[test]
    fn safe_json_parse_ok_and_err() {
        let v: serde_json::Value = safe_json_parse(r#"{"a":1}"#).expect("parse");
        assert_eq!(v["a"], 1);
        let err = safe_json_parse::<serde_json::Value>("not json").expect_err("bad json");
        assert!(err.to_string().contains("JSON") || err.to_string().contains("parsing"));
    }

    #[test]
    fn safe_lock_ok_and_poisoned() {
        let m = std::sync::Mutex::new(0i32);
        assert!(safe_lock(&m).is_ok());
        let m2 = std::sync::Mutex::new(());
        let _ = std::panic::catch_unwind(|| {
            let _g = m2.lock().unwrap();
            panic!("poison");
        });
        assert!(safe_lock(&m2).is_err());
    }

    #[test]
    fn safe_send_ok_and_disconnected() {
        let (tx, rx) = std::sync::mpsc::channel::<i32>();
        assert!(safe_send(&tx, 7).is_ok());
        drop(rx);
        assert!(safe_send(&tx, 1).is_err());
    }

    #[test]
    fn safe_read_to_string_ok_and_err() {
        let path = std::env::temp_dir().join(format!("nestgate_types_read_{}", std::process::id()));
        std::fs::write(&path, b"content").unwrap();
        assert_eq!(safe_read_to_string(&path).unwrap(), "content");
        std::fs::remove_file(&path).ok();
        let missing = path.join("definitely_missing");
        assert!(safe_read_to_string(&missing).is_err());
    }

    #[test]
    fn safe_env_var_reads_home_when_set() {
        if let Some(home) = crate::ProcessEnv.get("HOME") {
            assert_eq!(safe_env_var("HOME").unwrap(), home);
        }
    }
}
