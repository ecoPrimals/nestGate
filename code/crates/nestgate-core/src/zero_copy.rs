// Removed unused error imports
/// Zero-Copy Utilities
///
/// This module provides utilities and patterns for zero-copy operations,
/// reducing memory allocations and improving performance.
use std::borrow::Cow;
use std::sync::Arc;
/// Utility for creating Cow&lt;str&gt; from various string types
pub trait IntoCow<'a> {
    /// Into Cow
    fn into_cow(self) -> Cow<'a, str>;
}
impl<'a> IntoCow<'a> for &'a str {
    /// Into Cow
    fn into_cow(self) -> Cow<'a, str> {
        Cow::Borrowed(self)
    }
}

impl<'a> IntoCow<'a> for String {
    /// Into Cow
    fn into_cow(self) -> Cow<'a, str> {
        Cow::Owned(self)
    }
}

impl<'a> IntoCow<'a> for Cow<'a, str> {
    /// Into Cow
    fn into_cow(self) -> Cow<'a, str> {
        self
    }
}

/// Utility for efficient string operations
pub struct StringUtils;
/// Optimized string conversion for command output
pub fn optimize_command_output(output: &[u8]) -> Cow<str> {
    String::from_utf8_lossy(output)
}
/// Optimized string trimming that preserves zero-copy when possible
pub fn trim_efficient(s: &str) -> &str {
    s.trim()
}
impl StringUtils {
    /// Create a Cow&lt;str&gt; from static string literals (zero-copy)
    pub fn static_cow(s: &'static str) -> Cow<'static, str> {
        Cow::Borrowed(s)
    }

    /// Create a Cow&lt;str&gt; from owned strings (takes ownership)
    pub fn owned_cow(s: String) -> Cow<'static, str> {
        Cow::Owned(s)
    }

    /// Efficient string concatenation using Cow
    pub fn concat_cow<'a>(left: Cow<'a, str>, right: &str) -> Cow<'a, str> {
        if right.is_empty() {
            left
        } else {
            Cow::Owned(format!("{left}{right}"))
        }
    }
}

/// Buffer manager for zero-copy buffer reuse
pub struct BufferManager {
    buffers: Vec<Vec<u8>>,
    buffer_size: usize,
}
impl BufferManager {
    /// Create a new buffer manager with specified buffer size
    #[must_use]
    pub fn new(buffer_size: usize) -> Self {
        Self {
            buffers: Vec::new(),
            buffer_size,
        }
    }

    /// Get a buffer from the pool or create a new one
    pub fn get_buffer(&mut self) -> Vec<u8> {
        self.buffers
            .pop()
            .unwrap_or_else(|| Vec::with_capacity(self.buffer_size))
    }

    /// Return a buffer to the pool for reuse
    pub fn return_buffer(&mut self, mut buffer: Vec<u8>) {
        buffer.clear();
        if buffer.capacity() == self.buffer_size {
            self.buffers.push(buffer);
        }
    }

    /// Get the current pool size
    pub fn pool_size(&self) -> usize {
        self.buffers.len()
    }
}

/// Shared configuration using Arc for zero-copy sharing
pub struct SharedConfig<T> {
    data: Arc<T>,
}
impl<T> Clone for SharedConfig<T> {
    /// Clone
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
        }
    }
}

impl<T> SharedConfig<T> {
    /// Create a new shared configuration
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    /// Get a reference to the shared data
    pub fn get(&self) -> &T {
        &self.data
    }

    /// Get the reference count
    pub fn ref_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }
}

/// Zero-copy string slice operations
pub fn slice_cow(s: &str, start: usize, len: usize) -> Cow<str> {
    if start + len <= s.len() {
        Cow::Borrowed(&s[start..start + len])
    } else {
        Cow::Borrowed(s)
    }
}
/// Efficient line iteration without allocating
pub fn lines_zero_copy(s: &str) -> impl Iterator<Item = &str> {
    s.lines()
}
/// Zero-copy JSON value extraction (for simple cases)
pub fn extract_json_string<'a>(json: &'a str, key: &str) -> Option<&'a str> {
    // Simple JSON string extraction without parsing
    let pattern = format!("\"{key}\":");
    if let Some(start) = json.find(&pattern) {
        let start = start + pattern.len();
        if let Some(quote_start) = json[start..].find('"') {
            let quote_start = start + quote_start + 1;
            if let Some(quote_end) = json[quote_start..].find('"') {
                return Some(&json[quote_start..quote_start + quote_end]);
            }
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_cow_str_slice() {
        let s = "hello world";
        let cow = s.into_cow();
        assert!(matches!(cow, Cow::Borrowed(_)));
        assert_eq!(cow, "hello world");
    }

    #[test]
    fn test_into_cow_string() {
        let s = String::from("hello world");
        let cow = s.into_cow();
        assert!(matches!(cow, Cow::Owned(_)));
        assert_eq!(cow, "hello world");
    }

    #[test]
    fn test_into_cow_cow() {
        let original = Cow::Borrowed("hello world");
        let cow = original.into_cow();
        assert!(matches!(cow, Cow::Borrowed(_)));
        assert_eq!(cow, "hello world");
    }

    #[test]
    fn test_optimize_command_output() {
        let output = b"hello world\n";
        let cow = optimize_command_output(output);
        assert_eq!(cow, "hello world\n");
    }

    #[test]
    fn test_optimize_command_output_invalid_utf8() {
        let output = b"hello \xFF world";
        let cow = optimize_command_output(output);
        assert!(cow.contains("hello"));
        assert!(cow.contains("world"));
    }

    #[test]
    fn test_trim_efficient() {
        let s = "  hello world  ";
        let trimmed = trim_efficient(s);
        assert_eq!(trimmed, "hello world");
    }

    #[test]
    fn test_string_utils_static_cow() {
        let cow = StringUtils::static_cow("static string");
        assert!(matches!(cow, Cow::Borrowed(_)));
        assert_eq!(cow, "static string");
    }

    #[test]
    fn test_string_utils_owned_cow() {
        let s = String::from("owned string");
        let cow = StringUtils::owned_cow(s);
        assert!(matches!(cow, Cow::Owned(_)));
        assert_eq!(cow, "owned string");
    }

    #[test]
    fn test_string_utils_concat_cow_empty_right() {
        let left = Cow::Borrowed("hello");
        let result = StringUtils::concat_cow(left, "");
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_string_utils_concat_cow_non_empty_right() {
        let left = Cow::Borrowed("hello");
        let result = StringUtils::concat_cow(left, " world");
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_buffer_manager_new() {
        let manager = BufferManager::new(1024);
        assert_eq!(manager.pool_size(), 0);
        assert_eq!(manager.buffer_size, 1024);
    }

    #[test]
    fn test_buffer_manager_get_buffer() {
        let mut manager = BufferManager::new(1024);
        let buffer = manager.get_buffer();
        assert_eq!(buffer.capacity(), 1024);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_buffer_manager_return_buffer() {
        let mut manager = BufferManager::new(1024);
        let mut buffer = manager.get_buffer();
        buffer.push(42);

        manager.return_buffer(buffer);
        assert_eq!(manager.pool_size(), 1);

        let reused_buffer = manager.get_buffer();
        assert_eq!(reused_buffer.len(), 0);
        assert_eq!(reused_buffer.capacity(), 1024);
    }

    #[test]
    fn test_shared_config_new() {
        let config = SharedConfig::new("test config");
        assert_eq!(config.get(), &"test config");
        assert_eq!(config.ref_count(), 1);
    }

    #[test]
    fn test_shared_config_clone() {
        let config1 = SharedConfig::new("test config");
        let config2 = config1.clone();

        assert_eq!(config1.get(), config2.get());
        assert_eq!(config1.ref_count(), 2);
        assert_eq!(config2.ref_count(), 2);
    }

    #[test]
    fn test_slice_cow_valid_range() {
        let s = "hello world";
        let slice = slice_cow(s, 6, 5);
        assert!(matches!(slice, Cow::Borrowed(_)));
        assert_eq!(slice, "world");
    }

    #[test]
    fn test_slice_cow_invalid_range() {
        let s = "hello";
        let slice = slice_cow(s, 0, 10);
        assert!(matches!(slice, Cow::Borrowed(_)));
        assert_eq!(slice, "hello");
    }

    #[test]
    fn test_lines_zero_copy() {
        let text = "line1\nline2\nline3";
        let lines: Vec<&str> = lines_zero_copy(text).collect();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_extract_json_string_found() {
        let json = r"{"name": "John", "age": 30}";
        let name = extract_json_string(json, "name");
        assert_eq!(name, Some("John"));
    }

    #[test]
    fn test_extract_json_string_not_found() {
        let json = r"{"name": "John", "age": 30}";
        let email = extract_json_string(json, "email");
        assert_eq!(email, None);
    }

    #[test]
    fn test_extract_json_string_malformed() {
        let json = r"{"name": "John";
        let name = extract_json_string(json, "name");
        assert_eq!(name, None);
    }

    #[test]
    fn test_buffer_manager_wrong_capacity() {
        let mut manager = BufferManager::new(1024);
        let mut buffer = Vec::with_capacity(2048);
        buffer.push(42);

        manager.return_buffer(buffer);
        assert_eq!(manager.pool_size(), 0); // Should not be added to pool
    }

    #[test]
    fn test_zero_copy_performance_characteristics() {
        // Test that our zero-copy operations maintain their characteristics
        let static_str = "static string";
        let cow1 = StringUtils::static_cow(static_str);
        let cow2 = static_str.into_cow();

        // Both should be borrowed variants
        assert!(matches!(cow1, Cow::Borrowed(_)));
        assert!(matches!(cow2, Cow::Borrowed(_)));

        // Test that slicing preserves zero-copy
        let slice = slice_cow("hello world", 0, 5);
        assert!(matches!(slice, Cow::Borrowed(_)));
        assert_eq!(slice, "hello");
    }
}
