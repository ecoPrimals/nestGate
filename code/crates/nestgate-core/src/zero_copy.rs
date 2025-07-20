//! Zero-Copy Utilities
//!
//! This module provides utilities and patterns for zero-copy operations,
//! reducing memory allocations and improving performance.

use std::borrow::Cow;
use std::sync::Arc;

/// Utility for creating Cow&lt;str&gt; from various string types
pub trait IntoCow<'a> {
    fn into_cow(self) -> Cow<'a, str>;
}

impl<'a> IntoCow<'a> for &'a str {
    fn into_cow(self) -> Cow<'a, str> {
        Cow::Borrowed(self)
    }
}

impl<'a> IntoCow<'a> for String {
    fn into_cow(self) -> Cow<'a, str> {
        Cow::Owned(self)
    }
}

impl<'a> IntoCow<'a> for Cow<'a, str> {
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

    /// Create a Cow&lt;str&gt; from dynamic strings (clone on write)
    pub fn dynamic_cow(s: String) -> Cow<'static, str> {
        Cow::Owned(s)
    }

    /// Efficiently join strings with zero-copy when possible
    pub fn join_efficient(parts: &[&str], separator: &str) -> String {
        parts.join(separator)
    }
}

/// Shared configuration wrapper that reduces cloning
#[derive(Debug, Clone)]
pub struct SharedConfig<T> {
    inner: Arc<T>,
}

impl<T> SharedConfig<T> {
    pub fn new(config: T) -> Self {
        Self {
            inner: Arc::new(config),
        }
    }

    pub fn from_arc(arc: Arc<T>) -> Self {
        Self { inner: arc }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }

    pub fn clone_arc(&self) -> Arc<T> {
        Arc::clone(&self.inner)
    }
}

impl<T> std::ops::Deref for SharedConfig<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Efficient buffer management for zero-copy operations
pub struct BufferManager {
    buffers: Vec<Vec<u8>>,
    capacity: usize,
}

impl BufferManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffers: Vec::new(),
            capacity,
        }
    }

    pub fn get_buffer(&mut self, size: usize) -> Vec<u8> {
        if let Some(mut buffer) = self.buffers.pop() {
            if buffer.capacity() >= size {
                buffer.clear();
                buffer.resize(size, 0);
                return buffer;
            }
        }
        vec![0; size]
    }

    pub fn return_buffer(&mut self, buffer: Vec<u8>) {
        if self.buffers.len() < self.capacity {
            self.buffers.push(buffer);
        }
    }
}

/// Macro for creating efficient string maps
#[macro_export]
macro_rules! string_map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.to_string());
            )*
            map
        }
    };
}

/// Macro for creating efficient Cow string maps
#[macro_export]
macro_rules! cow_string_map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert(
                    std::borrow::Cow::Borrowed($key),
                    std::borrow::Cow::Borrowed($value)
                );
            )*
            map
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_config() {
        #[derive(Debug, Clone, PartialEq)]
        struct TestConfig {
            value: String,
        }

        let config = TestConfig {
            value: "test".to_string(),
        };

        let shared = SharedConfig::new(config.clone());
        assert_eq!(shared.get(), &config);

        let shared2 = SharedConfig::from_arc(shared.clone_arc());
        assert_eq!(shared2.get(), &config);
    }

    #[test]
    fn test_string_utils() {
        let static_cow = StringUtils::static_cow("hello");
        match static_cow {
            Cow::Borrowed(_) => (),
            Cow::Owned(_) => panic!("Expected borrowed"),
        }

        let dynamic_cow = StringUtils::dynamic_cow("world".to_string());
        match dynamic_cow {
            Cow::Owned(_) => (),
            Cow::Borrowed(_) => panic!("Expected owned"),
        }
    }

    #[test]
    fn test_buffer_manager() {
        let mut manager = BufferManager::new(2);

        let buffer1 = manager.get_buffer(1024);
        assert_eq!(buffer1.len(), 1024);

        manager.return_buffer(buffer1);

        let buffer2 = manager.get_buffer(1024);
        assert_eq!(buffer2.len(), 1024);
    }
}
