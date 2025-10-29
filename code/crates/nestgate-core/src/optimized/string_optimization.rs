use std::collections::HashMap;
//
// This module provides practical string optimization patterns to reduce memory
// allocations and improve performance across the NestGate codebase.

use std::borrow::Cow;
use std::sync::Arc;
use parking_lot::RwLock;
use std::sync::OnceLock;

// **MIGRATED**: Using canonical constants instead of local definitions
use crate::constants::canonical::{
    api::{CONFIG_API, CONFIG_ZFS, CONFIG_NETWORK, CONFIG_SECURITY, CONFIG_MONITORING},
    operations::OP_READ,
};

// **CANONICAL CONSTANTS MIGRATION COMPLETE**
// All constants now imported from the canonical constants system

/// **SHARED STRING POOL**
/// Thread-safe pool for frequently used strings
static STRING_POOL: OnceLock<RwLock<HashMap<String, Arc<str>>>> = OnceLock::new();
/// **STRING OPTIMIZATION UTILITIES**
pub struct StringOptimizer;
impl StringOptimizer {
    /// Get or create a shared string reference
    /// This is useful for strings that are used multiple times across the application
    pub fn get_shared_string(s: &str) -> Arc<str> {
        let pool = STRING_POOL.get_or_init(|| RwLock::new(HashMap::new()));
        
        // First try to read from the pool
        {
            let pool_read = pool.read();
            if let Some(shared) = pool_read.get(s) {
                return Arc::clone(shared);
            }
        }
        
        // If not found, create and insert
        let mut pool_write = pool.write();
        // Double-check in case another thread added it
        if let Some(shared) = pool_write.get(s) {
            return Arc::clone(shared);
        }
        
        let shared: Arc<str> = Arc::from(s);
        pool_write.insert(s.to_string(), Arc::clone(&shared));
        shared
    }
    
    /// Create a flexible string that can be borrowed or owned
    /// Use this when you sometimes need ownership and sometimes don't
    pub fn flexible_string(input: &str, needs_ownership: bool) -> Cow<'_, str> {
        if needs_ownership {
            Cow::Owned(input.to_string())
        } else {
            Cow::Borrowed(input)
        }
    }
    
    /// Efficient string concatenation for multiple parts
    pub fn concat_strings(parts: &[&str]) -> String {
        let total_len: usize = parts.iter().map(|s| s.len()).sum();
        let mut result = String::with_capacity(total_len);
        for part in parts {
            result.push_str(part);
        }
        result
    }
    
    /// Create a string with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> String {
        String::with_capacity(capacity)
    }
    
    /// Efficient string formatting with pre-allocation
    pub fn format_with_capacity(capacity: usize, args: std::fmt::Arguments) -> String {
        let mut result = String::with_capacity(capacity);
        std::fmt::write(&mut result, args).expect("String formatting should not fail");
        result
    }
}

/// **OPTIMIZED STRING BUILDER**
/// More efficient than repeated string concatenation
pub struct OptimizedStringBuilder {
    buffer: String,
}
impl OptimizedStringBuilder {
    /// Create a new builder with estimated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
        }
    }
    
    /// Create a new builder with default capacity
    pub fn new() -> Self {
        Self::with_capacity(256) // Reasonable default
    }
    
    /// Add a string slice to the builder
    pub fn push_str(&mut self, s: &str) -> &mut Self {
        self.buffer.push_str(s);
        self
    }
    
    /// Add a character to the builder
    pub fn push_char(&mut self, c: char) -> &mut Self {
        self.buffer.push(c);
        self
    }
    
    /// Add a formatted string to the builder
    pub fn push_fmt(&mut self, args: std::fmt::Arguments) -> &mut Self {
        std::fmt::write(&mut self.buffer, args).expect("String formatting should not fail");
        self
    }
    
    /// Get the current length
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    
    /// Check if the builder is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
    
    /// Build the final string
    pub fn build(self) -> String {
        self.buffer
    }
    
    /// Build and return as an Arc<str> for sharing
    pub fn build_shared(self) -> Arc<str> {
        Arc::from(self.buffer.as_str())
    }
}

impl Default for OptimizedStringBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// **ZERO-ALLOCATION STRING UTILITIES**
pub struct ZeroAllocString;
impl ZeroAllocString {
    /// Check if a string matches a constant without allocation
    pub fn matches_constant(input: &str, constant: &'static str) -> bool {
        input == constant
    }
    
    /// Get a string slice from a larger string without allocation
    pub fn substring(input: &str, start: usize, len: usize) -> &str {
        &input[start..start.min(input.len()).saturating_add(len).min(input.len())]
    }
    
    /// Split a string and process parts without collecting
    pub fn split_and_process<F>(input: &str, delimiter: char, mut processor: F)
    where
        F: FnMut(&str),
    {
        for part in input.split(delimiter) {
            processor(part);
        }
    }
}

/// **MEMORY-EFFICIENT ERROR MESSAGES**
/// Pre-allocated error message templates
pub struct ErrorMessages;
impl ErrorMessages {
        StringOptimizer::format_with_capacity(
            operation.len() + details.len() + 32,
            format_args!("Network error during {operation}: {details}")
        )
    }
    
        StringOptimizer::format_with_capacity(
            operation.len() + path.len() + 32,
        )
    }
    
    pub fn config_error(field: &str, reason: &str) -> String {
        StringOptimizer::format_with_capacity(
            field.len() + reason.len() + 32,
            format_args!("Configuration error in field '{field}': {reason}")
        )
    }
}

/// **PERFORMANCE MACROS**
/// Convenient macros for common string optimization patterns
/// Efficiently concatenate string literals and variables
#[macro_export]
macro_rules! concat_strings {
    ($($part:expr),+ $(,)?) => {{
        let parts = &[$($part),+];
        $crate::optimized::string_optimization::StringOptimizer::concat_strings(parts)
    };
}
/// Create a shared string that can be reused
#[macro_export]
macro_rules! shared_string {
    ($s:expr) => {
        $crate::optimized::string_optimization::StringOptimizer::get_shared_string($s)
    };
}
/// Build a string efficiently with known parts
#[macro_export]
macro_rules! build_string {
    ($capacity:expr; $($part:expr),+ $(,)?) => {{
        let mut builder = $crate::optimized::string_optimization::OptimizedStringBuilder::with_capacity($capacity);
        $(builder.push_str($part);)+
        builder.build()
    };
    ($($part:expr),+ $(,)?) => {{
        let mut builder = $crate::optimized::string_optimization::OptimizedStringBuilder::new();
        $(builder.push_str($part);)+
        builder.build()
    };
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_constants() {
        assert_eq!(StringConstants::CONFIG_API, "api");
        assert_eq!(StringConstants::OP_READ, "read");
        assert_eq!(StringConstants::STATUS_SUCCESS, "success");
    }
    
    #[test]
    fn test_shared_string_pool() {
        let s1 = StringOptimizer::get_shared_string("test");
        let s2 = StringOptimizer::get_shared_string("test");
        
        // Should be the same Arc (pointer equality)
        assert!(Arc::ptr_eq(&s1, &s2));
    }
    
    #[test]
    fn test_flexible_string() {
        let borrowed = StringOptimizer::flexible_string("test", false);
        let owned = StringOptimizer::flexible_string("test", true);
        
        assert!(matches!(borrowed, Cow::Borrowed(_)));
        assert!(matches!(owned, Cow::Owned(_)));
    }
    
    #[test]
    fn test_string_builder() {
        // **CANONICAL MODERNIZATION** - Fixed builder pattern
        let mut builder = OptimizedStringBuilder::with_capacity(20);
        builder.push_str("Hello");
        builder.push_char(' ');
        builder.push_str("World");
        let result = builder.build();
        
        assert_eq!(result, "Hello World");
    }
    
    #[test]
    fn test_concat_strings() {
        let result = StringOptimizer::concat_strings(&["Hello", " ", "World"]);
        assert_eq!(result, "Hello World");
    }
    
    #[test]
    fn test_zero_alloc_utilities() {
        assert!(ZeroAllocString::matches_constant("api", StringConstants::CONFIG_API));
        
        let substring = ZeroAllocString::substring("Hello World", 6, 5);
        assert_eq!(substring, "World");
        
        let mut parts: Vec<String> = Vec::new();
        ZeroAllocString::split_and_process("a,b,c", ',', |part| {
            parts.push(part.to_string());
        );
        assert_eq!(parts, vec!["a", "b", "c"]);
    }
    
    #[test]
    fn test_error_messages() {
        let error = ErrorMessages::network_error("connect", "timeout", None);
        assert!(error.contains("Network error during connect: timeout"));
        
        let storage_error = ErrorMessages::storage_error("read", "/tmp/file", None);
    }
    
    #[test]
    fn test_macros() {
        let result = concat_strings!("Hello", " ", "World");
        assert_eq!(result, "Hello World");
        
        let shared = shared_string!("test");
        assert_eq!(shared.as_ref(), "test");
        
        let built = build_string!("Hello", " ", "World");
        assert_eq!(built, "Hello World");
        
        let built_with_capacity = build_string!(20; "Hello", " ", "World");
        assert_eq!(built_with_capacity, "Hello World");
    }
}