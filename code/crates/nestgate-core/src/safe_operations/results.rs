// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **UNIFIED**: Use the main Result type from parent module  
pub use super::Result;
/// Result handling utilities
/// Core utilities for handling results safely without `unwrap()` calls
use crate::error::NestGateError;
use std::fmt::Debug;
/// **SAFE RESULT UNWRAP**
/// Provides safe result unwrapping with contextual error handling
pub fn safe_unwrap_result<T, E: Debug>(
    result: std::result::Result<T, E>,
    operation: &str,
    context: &str,
) -> Result<T> {
    result.map_err(|e| {
        NestGateError::internal_error(
            format!("Operation '{operation}' failed in context '{context}': {e:?}"),
            "safe_operations_results",
        )
    })
}
