/// Safe memory operations
/// Provides safe alternatives to memory operations that might panic
use crate::NestGateError;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE BUFFER TAKE**
/// Safely extract elements from a buffer without panicking
pub fn safe_buffer_take<T: Clone>(buffer: &[T], count: usize, context: &str) -> Result<Vec<T>> {
    if count > buffer.len() {
        return Err(NestGateError::Internal {
            message: format!(
                "Cannot take {count} items from buffer of length {} in context: {context}",
                buffer.len()
            ),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: Some("Buffer underflow".to_string()),
            is_bug: false,
        });
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
        return Err(NestGateError::ResourceExhausted {
            resource: "memory".to_string(),
            current: buffer.len() as u64,
            limit: (buffer.capacity() * 2) as u64,
            retry_after: None,
            scaling_suggestion: Some("Consider processing data in smaller chunks".to_string()),
        });
    }

    buffer.resize(new_size, T::default());
    Ok(())
}

/// **SAFE BUFFER RESERVE**
/// Safely reserve buffer capacity
pub fn safe_buffer_reserve<T>(buffer: &mut Vec<T>, additional: usize, context: &str) -> Result<()> {
    buffer
        .try_reserve(additional)
        .map_err(|e| NestGateError::Internal {
            message: format!("Buffer reservation failed in context: {context}"),
            location: Some(format!("{}:{}", file!(), line!())),
            debug_info: Some(format!("Allocation error: {e}")),
            is_bug: false,
        })
}

/// **SAFE BUFFER TRUNCATE**
/// Safely truncate buffer to specified length
pub fn safe_buffer_truncate<T>(buffer: &mut Vec<T>, len: usize, context: &str) -> Result<()> {
    if len > buffer.len() {
        return Err(NestGateError::Validation {
            field: "buffer_length".to_string(),
            message: format!(
                "Truncation length {} exceeds buffer size {} in {}",
                len,
                buffer.len(),
                context
            ),
            current_value: Some(len.to_string()),
            expected: Some(format!("Length <= {}", buffer.len())),
            user_error: false,
        });
    }

    buffer.truncate(len);
    Ok(())
}
