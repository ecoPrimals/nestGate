use crate::error::NestGateError;
/// **CLONE OPTIMIZATION MODULE**
///
/// This module provides patterns and utilities to minimize unnecessary cloning
/// and maximize zero-copy performance throughout the NestGate system.
///
/// ## Performance Impact
/// - **Current**: 304 files using .clone() - potential performance bottleneck
/// - **Target**: Reduce cloning by 60%+ through borrowing and Arc patterns
/// - **Strategy**: Smart reference patterns, Arc sharing, and Cow types
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;
/// **SMART REFERENCE PATTERNS**
///
/// Utilities for choosing optimal reference types based on usage patterns
pub struct SmartRef;
impl SmartRef {
    /// Choose between owned and borrowed based on lifetime requirements
    pub fn choose_ref<'a, T: Clone>(data: &'a T, needs_ownership: bool) -> Cow<'a, T> {
        if needs_ownership {
            Cow::Owned(data.clone())
        } else {
            Cow::Borrowed(data)
        }
    }

    /// Create Arc for multi-threaded sharing without cloning data
    pub fn share_threaded<T>(data: T) -> Arc<T> {
        Arc::new(data)
    }

    /// Create Rc for single-threaded sharing without cloning data
    pub fn share_local<T>(data: T) -> Rc<T> {
        Rc::new(data)
    }

    /// Extract from Arc without cloning if possible
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn try_unwrap_arc<T>(arc: Arc<T>) -> Result<T, Arc<T>>  {
        Arc::try_unwrap(arc)
    }
}

/// **CONFIGURATION SHARING PATTERNS**
///
/// Optimized patterns for sharing configuration data without cloning
#[derive(Debug)]
pub struct SharedConfiguration<T> {
    data: Arc<T>,
}
impl<T> Clone for SharedConfiguration<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data), // Arc clone is cheap (just reference count)
        }
    }
}

impl<T> SharedConfiguration<T> {
    /// Create new shared configuration
    pub fn new(config: T) -> Self {
        Self {
            data: Arc::new(config),
        }
    }

    /// Get reference to configuration (zero-copy)
    pub fn get(&self) -> &T {
        &self.data
    }

    /// Check if this is the only reference
    pub fn is_unique(&self) -> bool {
        Arc::strong_count(&self.data) == 1
    }

    /// Get mutable reference if unique, otherwise clone
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn get_mut(&mut self) -> Result<&mut T, NestGateError>  {
        Arc::get_mut(&mut self.data).ok_or_else(|| NestGateError::internal_error(
    }
}

/// **STRING OPTIMIZATION PATTERNS**
///
/// Utilities for efficient string handling without unnecessary cloning
pub struct StringOptimizer;
impl StringOptimizer {
    /// Create Cow string that borrows when possible
    pub fn efficient_string<'a>(s: &'a str, needs_modification: bool) -> Cow<'a, str> {
        if needs_modification {
            Cow::Owned(s.to_string())
        } else {
            Cow::Borrowed(s)
        }
    }

    /// Concatenate strings efficiently using Cow
    pub fn concat_efficient<'a>(base: Cow<'a, str>, suffix: &str) -> Cow<'a, str> {
        if suffix.is_empty() {
            base
        } else {
            match base {
                Cow::Borrowed(s) => Cow::Owned(format!("{s}{suffix}")),
                Cow::Owned(mut s) => {
                    s.push_str(suffix);
                    Cow::Owned(s)
                }
            }
        }
    }

    /// Trim string without cloning when possible
    pub fn trim_efficient(s: &str) -> &str {
        s.trim()
    }

    /// Split string into parts without cloning
    pub fn split_efficient(s: &str, delimiter: char) -> Vec<&str> {
        s.split(delimiter).collect()
    }
}

/// **COLLECTION OPTIMIZATION PATTERNS**
///
/// Utilities for efficient collection operations
pub struct CollectionOptimizer;
impl CollectionOptimizer {
    /// Share vector data using Arc
    pub fn share_vec<T>(vec: Vec<T>) -> Arc<Vec<T>> {
        Arc::new(vec)
    }

    /// Convert between owned and borrowed slices efficiently
    pub fn slice_cow<'a, T: Clone>(data: &'a [T], needs_ownership: bool) -> Cow<'a, [T]> {
        if needs_ownership {
            Cow::Owned(data.to_vec())
        } else {
            Cow::Borrowed(data)
        }
    }

    /// Filter without cloning elements when possible
    pub fn filter_refs<T>(data: &[T], predicate: impl Fn(&T) -> bool) -> Vec<&T> {
        data.iter().filter(|item| predicate(item)).collect()
    }

    /// Map over references to avoid cloning
    pub fn map_refs<'a, T, U>(data: &'a [T], mapper: impl Fn(&'a T) -> U) -> Vec<U> {
        data.iter().map(mapper).collect()
    }
}

/// **ZERO-COPY RESULT PATTERNS**
///
/// Patterns for returning results without unnecessary cloning
pub struct ZeroCopyResults;
impl ZeroCopyResults {
    /// Return reference when possible, owned when necessary
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn result_ref_or_owned<'a, T: Clone>(
        data: &'a T,
        condition: bool,
    ) -> Result<Cow<'a, T>, NestGateError>  {
        if condition {
            Ok(Cow::Borrowed(data))
        } else {
            Ok(Cow::Owned(data.clone()))
        }
    }

    /// Share result across multiple consumers
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn shared_result<T>(result: Result<T, NestGateError>) -> Result<Arc<T>, NestGateError>  {
        result.map(Arc::new)
    }

    /// Convert owned result to borrowed when safe
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn borrow_result<T>(result: &Result<T, NestGateError>) -> Result<&T, &NestGateError>  {
        result.as_ref()
    }
}

/// **PERFORMANCE MEASUREMENT**
///
/// Utilities to measure clone reduction impact
pub struct CloneMetrics {
    pub clones_avoided: u64,
    pub memory_saved_bytes: u64,
    pub performance_improvement_percent: f64,
}
impl Default for CloneMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl CloneMetrics {
    pub fn new() -> Self {
        Self {
            clones_avoided: 0,
            memory_saved_bytes: 0,
            performance_improvement_percent: 0.0,
        }
    }

    pub fn record_avoided_clone(&mut self, size_bytes: u64) {
        self.clones_avoided += 1;
        self.memory_saved_bytes += size_bytes;
    }

    pub fn calculate_improvement(&mut self, baseline_ns: u64, optimized_ns: u64) {
        if baseline_ns > 0 {
            self.performance_improvement_percent =
                ((baseline_ns - optimized_ns) as f64 / baseline_ns as f64) * 100.0;
        }
    }
}

/// **MIGRATION HELPERS**
///
/// Utilities to help migrate existing code to zero-copy patterns
pub struct MigrationHelper;
impl MigrationHelper {
    /// Replace .clone() with efficient alternative
    pub fn replace_clone<'a, T: Clone>(
        data: &'a T,
        usage_pattern: CloneUsage,
    ) -> OptimizedReference<'a, T> {
        match usage_pattern {
            CloneUsage::SingleUse => OptimizedReference::Borrowed(data),
            CloneUsage::MultipleReads => OptimizedReference::Shared(Arc::new(data.clone())),
            CloneUsage::Modification => OptimizedReference::Owned(data.clone()),
        }
    }
}

/// Usage patterns for clone optimization
pub enum CloneUsage {
    /// Single read access - use borrowing
    SingleUse,
    /// Multiple read access - use Arc sharing
    MultipleReads,
    /// Needs modification - must clone
    Modification,
}
/// Optimized reference types
pub enum OptimizedReference<'a, T> {
    Borrowed(&'a T),
    Shared(Arc<T>),
    Owned(T),
}
impl<'a, T> OptimizedReference<'a, T> {
    /// Get reference regardless of storage type
    pub fn get_ref(&self) -> &T {
        match self {
            Self::Borrowed(r) => r,
            Self::Shared(arc) => arc,
            Self::Owned(owned) => owned,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_configuration() {
        let config = SharedConfiguration::new("test_config".to_string());
        let config_clone = config.clone();

        assert_eq!(config.get(), config_clone.get());
        assert_eq!(Arc::strong_count(&config.data), 2);
    }

    #[test]
    fn test_string_optimization() -> anyhow::Result<()> {
        // Test that we can pass string references efficiently
        let owned_string = "test_string".to_string();

        // Test reference passing
        let result = process_string_reference(&owned_string);
        assert_eq!(result, "processed: test_string");

        // Test owned string handling
        let result = process_owned_string(owned_string.clone());
        assert_eq!(result, "processed: test_string");

        // Test error case
        if owned_string != "expected_owned" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Expected owned string".to_string(),
            )
            .into());
        }

        Ok(())
    }

    #[test]
    fn test_clone_metrics() {
        let mut metrics = CloneMetrics::new();
        metrics.record_avoided_clone(1024);
        metrics.calculate_improvement(1000, 800);

        assert_eq!(metrics.clones_avoided, 1);
        assert_eq!(metrics.memory_saved_bytes, 1024);
        assert_eq!(metrics.performance_improvement_percent, 20.0);
    }

    // Helper functions for testing
    fn process_string_reference(s: &str) -> String {
        format!("processed: {s}")
    }

    fn process_owned_string(s: String) -> String {
        format!("processed: {s}")
    }
}
