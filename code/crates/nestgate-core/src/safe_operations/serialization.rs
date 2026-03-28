// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Safe serialization operations
// Replaces manual serde operations with proper error handling

use crate::error::{NestGateError, Result}; // Removed unused ErrorContext for pedantic perfection
use serde::{Deserialize, Serialize};

/// **SAFE JSON SERIALIZATION**
/// Replaces `crate::safe_operations::safe_to_json()`? with proper error handling
///
/// # Errors
///
/// This function will return an error if:
/// - JSON serialization fails due to invalid data structures
/// - Memory allocation fails during serialization
/// - The data contains non-serializable types
pub fn safe_to_json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(|e| {
        NestGateError::internal_error(
            format!("JSON serialization failed: {e}"),
            "safe_serialization",
        )
    })
}
/// **SAFE JSON DESERIALIZATION**
/// Replaces `crate::safe_operations::safe_from_json()`? with proper error handling
///
/// # Errors
///
/// This function will return an error if:
/// - The JSON string is malformed or invalid
/// - The JSON structure doesn't match the expected type T
/// - Memory allocation fails during deserialization
/// - Required fields are missing from the JSON
pub fn safe_from_json<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T> {
    serde_json::from_str(json).map_err(|e| {
        NestGateError::internal_error(
            format!("JSON deserialization failed: {e}"),
            "safe_serialization",
        )
    })
}
/// **SAFE JSON PRETTY SERIALIZATION**
/// For configuration and debugging output
///
/// # Errors
///
/// This function will return an error if:
/// - JSON serialization fails due to invalid data structures
/// - Memory allocation fails during pretty formatting
/// - The data contains non-serializable types
pub fn safe_to_json_pretty<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string_pretty(value).map_err(|e| {
        NestGateError::internal_error(
            format!("JSON pretty serialization failed: {e}"),
            "safe_serialization",
        )
    })
}
