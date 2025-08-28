/// Safe memory operations
/// Provides safe alternatives to memory operations that might panic
use crate::error::NestGateError;

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
            component: "safe_operations_memory".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            is_bug: false,
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "safe_buffer_read".to_string(),
                component: "safe_memory".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("details".to_string(), "Buffer underflow".to_string());
                    map.insert("requested_count".to_string(), count.to_string());
                    map.insert("buffer_len".to_string(), buffer.len().to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                retry_info: None,
                recovery_suggestions: vec!["Check buffer size before reading".to_string()],
                    performance_metrics: None,
                    environment: None,
            }),
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
            message: format!("Memory allocation failed: buffer size {} exceeds capacity {}", buffer.len(), buffer.capacity() * 2),
            resource_type: "memory".to_string(),
            current_usage: Some(buffer.len() as u64),
            max_capacity: Some((buffer.capacity() * 2) as u64),
            context: None,
        })
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
            component: "safe_operations_memory".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            is_bug: false,
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "safe_buffer_reserve".to_string(),
                component: "safe_memory".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("context".to_string(), context.to_string());
                    map.insert("additional_requested".to_string(), additional.to_string());
                    map.insert("allocation_error".to_string(), e.to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                retry_info: None,
                recovery_suggestions: vec!["Free unused memory and retry".to_string()],
                    performance_metrics: None,
                    environment: None,
            }),
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
            value: Some(len.to_string()),
            current_value: Some(len.to_string()),
            expected: Some(format!("Length <= {}", buffer.len())),
            context: None,
        });
    }

    buffer.truncate(len);
    Ok(())
}
