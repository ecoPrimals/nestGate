/// Safe collection operations
/// These functions provide safe alternatives to collection operations that might panic
use crate::error::NestGateError;
use std::collections::HashMap;

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE COLLECTION ACCESS**
/// Replaces vec[index], map; safe_unwrap!(.get(), "map access") etc.
pub fn safe_get<T: Clone>(collection: &[T], index: usize, context: &str) -> Result<T> {
    collection
        .get(index)
        .cloned()
        .ok_or_else(|| NestGateError::Internal {
            message: format!(
                "Index {index} out of bounds (len: {}) in context: {context}",
                collection.len()
            ),
            component: "safe_operations_collections".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            is_bug: false,
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "safe_collection_get".to_string(),
                component: "safe_operations".to_string(),
                metadata: {
                    let mut map = std::collections::HashMap::new();
                    map.insert("context".to_string(), context.to_string());
                    map.insert("index".to_string(), index.to_string());
                    map.insert("collection_len".to_string(), collection.len().to_string());
                    map
                },
                timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec![],
                    performance_metrics: None,
                    environment: None,
            }),
        })
}

/// **SAFE MAP ACCESS**
/// Replaces map; safe_unwrap!(.get(), "map access") with contextual error handling
pub fn safe_map_get<K: std::fmt::Debug + std::hash::Hash + Eq, V: Clone>(
    map: &HashMap<K, V>,
    key: &K,
    context: &str,
) -> Result<V> {
    map.get(key)
        .cloned()
        .ok_or_else(|| NestGateError::Internal {
            message: format!("Key {key:?} not found in map in context: {context}"),
            component: "safe_operations_collections".to_string(),
            location: Some(format!("{}:{}", file!(), line!())),
            is_bug: false,
            context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
                operation: "safe_map_get".to_string(),
                component: "safe_operations".to_string(),
                metadata: {
                    let mut metadata = std::collections::HashMap::new();
                    metadata.insert("context".to_string(), context.to_string());
                    metadata.insert("map_size".to_string(), map.len().to_string());
                    metadata
                },
                timestamp: std::time::SystemTime::now(),
                    retry_info: None,
                    recovery_suggestions: vec![],
                    performance_metrics: None,
                    environment: None,
            }),
        })
}

/// **SAFE VECTOR INDEX ACCESS WITH RECOVERY**
/// Enhanced version that provides fallback strategies
pub fn safe_get_with_default<T: Clone + Default>(
    collection: &[T],
    index: usize,
    context: &str,
) -> T {
    collection.get(index).cloned().unwrap_or_else(|| {
        tracing::warn!(
            "Index {} out of bounds in context '{}', using default",
            index,
            context
        );
        T::default()
    })
}
