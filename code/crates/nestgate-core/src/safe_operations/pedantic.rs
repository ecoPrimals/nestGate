// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use crate::NestGateError;
/// **ULTRA-PEDANTIC SAFE OPERATIONS**
///
/// This module provides the most rigorous, panic-free operation utilities
/// for achieving absolute perfection in production code.
use std::alloc::Layout;
use std::collections::HashMap;
use std::net::IpAddr;

/// **UNIFIED**: Use the main Result type from parent module
/// **PEDANTIC IP PARSING**
/// Absolutely safe IP address parsing with comprehensive error context
#[must_use]
/// # Errors
/// Returns an error if the operation fails
pub fn pedantic_parse_ip(ip_str: &str, context: &str) -> Result<IpAddr, NestGateError> {
    ip_str.parse().map_err(|_e| {
        NestGateError::configuration(format!(
            "Invalid IP address format in {}: '{}' (expected format like 127.0.0.1 or ::1)",
            context, ip_str
        ))
    })
}

/// **PEDANTIC LAYOUT CREATION**
/// Absolutely safe memory layout creation with alignment validation
pub fn pedantic_layout_from_size_align(size: usize, align: usize, context: &str) -> Result<Layout, NestGateError> {
    Layout::from_size_align(size, align).map_err(|e| {
        NestGateError::internal_error_with_debug(
            format!("Invalid memory layout in {}: size={}, align={}, error={}", context, size, align, e)
        )
    })
}

/// **PEDANTIC VECTOR ACCESS**
/// Absolutely safe vector element access with bounds checking
pub fn pedantic_get_vector_element<T>(vec: &[T], index: usize, context: &str) -> Result<&T, NestGateError> {
    vec.get(index).ok_or_else(|| {
        let mut details = HashMap::new();
        details.insert("index".to_string(), index.to_string());
        details.insert("vector_length".to_string(), vec.len().to_string());
        details.insert("context".to_string(), context.to_string());
        NestGateError::internal_error_with_debug(
            format!("Vector index {} out of bounds (length: {}) in {}", index, vec.len(), context)
        )
    })
}

/// **PEDANTIC MACROS FOR ULTRA-SAFE OPERATIONS**
/// Replace ? with context-aware error handling
#[macro_export]
macro_rules! pedantic_unwrap {
    ($expr:expr, $context:expr) => {
        $expr.map_err(|e| $crate::error::NestGateError::internal_error_with_debug(
            format!("PEDANTIC: Operation failed in {}: {:?}", $context, e)
        ))?
    };
}

/// Replace .expect() with detailed error context
#[macro_export]
macro_rules! pedantic_expect {
    ($expr:expr, $context:expr) => {
        $expr.ok_or_else(|| $crate::error::NestGateError::internal_error_with_debug(
            format!("PEDANTIC: Expected value in {$context}")
        ))?
    };
}

/// **PEDANTIC CONSTANTS**
pub const PEDANTIC_MAX_RETRIES: usize = 3;
/// Pedantic Timeout Ms
pub const PEDANTIC_TIMEOUT_MS: u64 = 5000;
/// Pedantic Buffer Size
pub const PEDANTIC_BUFFER_SIZE: usize = 4096;

/// **PEDANTIC VALIDATION FUNCTIONS**
pub fn validate_non_empty_string(s: &str, field_name: &str) -> Result<(), NestGateError> {
    if s.is_empty() {
        Err(NestGateError::configuration(format!(
            "Field '{}' cannot be empty", field_name
        )))
    } else {
        Ok(())
    }
}

/// Validates  Positive Number
pub fn validate_positive_number(n: i64, field_name: &str) -> Result<(), NestGateError> {
    if n <= 0 {
        Err(NestGateError::configuration(format!(
            "Field '{}' must be positive, got: {}", field_name, n
        )))
    } else {
        Ok(())
    }
}

/// **PEDANTIC ERROR CONTEXT BUILDER**
pub struct PedanticErrorContext {
    context: String,
    details: HashMap<String, String>,
}

impl PedanticErrorContext {
    #[must_use]
    pub fn new(context: &str) -> Self {
        Self {
            context: context.to_string(),
            details: HashMap::new(),
        }
    }

    #[must_use]
    pub fn add_detail(mut self, key: &str, value: &str) -> Self {
        self.details.insert(key.to_string(), value.to_string());
        self
    }

    /// Builds the final instance
    pub fn build_error(self, message: &str) -> NestGateError {
        NestGateError::internal_error_with_debug(
            format!("{} in {}: {:?}", message, self.context, self.details)
        )
    }
} 