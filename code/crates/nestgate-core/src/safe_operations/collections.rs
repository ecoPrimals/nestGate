// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;
/// Safe collection operations
/// These functions provide safe alternatives to collection operations that might panic
use crate::error::NestGateError;
use std::collections::HashMap;
/// **SAFE COLLECTION ACCESS**
/// Replaces `vec[index]`, `map.get()`, and similar operations that might panic.
pub fn safe_get<T: Clone>(collection: &[T], index: usize, context: &str) -> Result<T> {
    collection.get(index).cloned().ok_or_else(|| {
        NestGateError::internal_error(
            format!(
                "Index {index} out of bounds (len: {}) in context: {context}",
                collection.len()
            ),
            "safe_operations_collections",
        )
    })
}
/// **SAFE MAP ACCESS**
/// Replaces map; `safe_unwrap!(.get()`, "map access") with contextual error handling
pub fn safe_map_get<K: std::fmt::Debug + std::hash::Hash + Eq, V: Clone>(
    map: &HashMap<K, V>,
    key: &K,
    context: &str,
) -> Result<V> {
    map.get(key).cloned().ok_or_else(|| {
        NestGateError::internal_error(
            format!("Key {key:?} not found in map in context: {context}"),
            "safe_operations_collections",
        )
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
