// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Safe service operations

use crate::Result;
use crate::error::NestGateError;
use tracing::warn;

/// Safely call a service operation with error handling
pub fn safe_service_call<T, F>(operation_name: &str, operation: F) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    match operation() {
        Ok(result) => Ok(result),
        Err(e) => {
            warn!(operation = operation_name, "Service operation failed: {e}");
            Err(e)
        }
    }
}
/// Create a service error with context
#[must_use]
pub fn create_service_error(operation_name: &str, message: &str) -> NestGateError {
    NestGateError::internal_error(
        format!("Service operation '{operation_name}' failed: {message}"),
        "safe_operations",
    )
}
