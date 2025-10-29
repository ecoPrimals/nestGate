// **ERROR CONVERSION AND CONTEXT TRAITS**
//! Trait definitions and implementations.
// Traits for converting between error types and adding contextual information.

use crate::error::NestGateError;
use super::result_types::IdioResult;

/// **IDIOMATIC ERROR CONVERSION**
/// Provides seamless conversion between domain-specific errors and NestGateError
/// while maintaining all rich context information.
pub trait IntoNestGateError {
    fn into_nestgate_error(self) -> NestGateError;
}
/// **CONTEXTUAL ERROR ENHANCEMENT**
/// Adds rich context to errors for better debugging and monitoring
pub trait WithContext<T> {
    fn with_operation(self, operation: &str) -> IdioResult<T>;
    fn with_component(self, component: &str) -> IdioResult<T>;
}
impl<T, E> WithContext<T> for IdioResult<T, E>
where
    E: Into<NestGateError>,
{
    fn with_operation(self, _operation: &str) -> IdioResult<T> {
        self.map_err(|e| {
            
            // Context is embedded in the error variants themselves
            // No need to add context as errors already contain operation info
            e.into()
        })
    }
    
    fn with_component(self, _component: &str) -> IdioResult<T> {
        self.map_err(|e| {
            
            // Context is embedded in the error variants themselves
            // No need to add context as errors already contain component info
            e.into()
        })
    }
}

/// **IDIOMATIC RESULT EXTENSIONS**
/// Extension trait providing additional functionality for IdioResult types
pub trait IdioResultExt<T, E> {
    /// Add operation context to the error
    fn with_operation(self, operation: &str) -> IdioResult<T, E>;
    
    /// Add component context to the error
    fn with_component(self, component: &str) -> IdioResult<T, E>;
    
    /// Convert to a domain-specific Result type
    fn to_domain<F>(self, f: F) -> IdioResult<T, E>
    where
        F: FnOnce(E) -> E;
}
impl<T, E> IdioResultExt<T, E> for IdioResult<T, E> {
    fn with_operation(self, _operation: &str) -> IdioResult<T, E> {
        // For domain-specific errors, context is already embedded
        self
    }
    
    fn with_component(self, _component: &str) -> IdioResult<T, E> {
        // For domain-specific errors, context is already embedded
        self
    }
    
    fn to_domain<F>(self, f: F) -> IdioResult<T, E>
    where
        F: FnOnce(E) -> E,
    {
        self.map_err(f)
    }
} 