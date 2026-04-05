// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

pub use super::Result;
/// **UNIFIED**: Use the main Result type from parent module
use crate::error::NestGateError;
/// Safe testing operations
/// Provides safe alternatives to testing operations that might panic
/// Create internal error (used by `safe_operations` functions)
#[must_use]
pub fn internal(message: &str, context: &str) -> NestGateError {
    NestGateError::internal_error(format!("{context}: {message}"), "safe_operations_testing")
}
