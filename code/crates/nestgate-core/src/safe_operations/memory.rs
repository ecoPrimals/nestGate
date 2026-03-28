// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;
/// Safe memory operations
/// Provides safe alternatives to memory operations that might panic
use crate::error::NestGateError;
/// **SAFE BUFFER TAKE**
/// Safely extract elements from a buffer without panicking
pub fn safe_buffer_take<T: Clone>(buffer: &[T], count: usize, context: &str) -> Result<Vec<T>> {
    if count > buffer.len() {
        return Err(NestGateError::internal_error(
            format!(
                "Cannot take {count} items from buffer of length {} in context: {context}",
                buffer.len()
            ),
            "safe_operations_memory",
        ));
    }
    Ok(buffer[..count].to_vec())
}

/// **SAFE BUFFER RESIZE**
/// Safely resize a buffer with error handling
pub fn safe_buffer_resize<T: Clone + Default>(
    buffer: &mut Vec<T>,
    new_size: usize,
    _context: &str,
) -> Result<()> {
    // Safe resize operation with bounds checking
    if new_size > buffer.capacity() * 2 {
        // Prevent excessive memory allocation
        return Err(NestGateError::internal_error(
            format!(
                "Memory allocation failed: buffer size {} exceeds capacity {}",
                buffer.len(),
                buffer.capacity() * 2
            ),
            "safe_memory",
        ));
    }
    buffer.resize(new_size, T::default());
    Ok(())
}

/// **SAFE BUFFER RESERVE**
/// Safely reserve buffer capacity
pub fn safe_buffer_reserve<T>(buffer: &mut Vec<T>, additional: usize, context: &str) -> Result<()> {
    buffer.try_reserve(additional).map_err(|_e| {
        NestGateError::internal_error(
            format!("Buffer reservation failed in context: {context}"),
            "safe_operations_memory",
        )
    })
}
/// **SAFE BUFFER TRUNCATE**
/// Safely truncate buffer to specified length
pub fn safe_buffer_truncate<T>(buffer: &mut Vec<T>, len: usize, context: &str) -> Result<()> {
    if len > buffer.len() {
        return Err(NestGateError::validation_error(&format!(
            "Truncation length {} exceeds buffer size {} in {}",
            len,
            buffer.len(),
            context
        )));
    }
    buffer.truncate(len);
    Ok(())
}
