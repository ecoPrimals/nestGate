// Memory optimization utilities to reduce allocations and improve performance
//! Memory Optimization functionality and utilities.
// This module provides utilities and patterns for reducing memory allocations
//! and improving performance through zero-copy patterns and efficient string handling.

use std::borrow::Cow;
use std::collections::HashMap;

/// Efficient string constants for common operations
pub mod constants {
    /// Common operation names as static strings to avoid allocations
    pub const GET_METADATA: &str = "get_metadata";
    pub const CREATE_FILE: &str = "create_file";
    pub const READ_FILE: &str = "read_file";
    pub const WRITE_FILE: &str = "write_file";
    pub const DELETE_FILE: &str = "delete_file";
    pub const CREATE_DIRECTORY: &str = "create_directory";
    pub const DELETE_DIRECTORY: &str = "delete_directory";
    pub const READ_DIRECTORY: &str = "read_directory";
    pub const ATOMIC_WRITE: &str = "atomic_write";
    pub const SYNC_FILE: &str = "sync_file";
    /// Common status strings
    pub const INITIALIZED: &str = "initialized";
    pub const RUNNING: &str = "running";
    pub const STOPPED: &str = "stopped";
    pub const FAILED: &str = "failed";

    /// Common error messages
    pub const FILE_NOT_FOUND: &str = "File not found";
    pub const PERMISSION_DENIED: &str = "Permission denied";
    pub const DISK_FULL: &str = "Disk full";
    pub const INVALID_PATH: &str = "Invalid path";
}

/// Efficient error message builder that avoids unnecessary allocations
pub struct ErrorMessageBuilder {
    base_message: &'static str,
    details: Option<String>,
}
impl ErrorMessageBuilder {
    /// Create a new error message builder with a static base message
    pub const fn new(base_message: &'static str) -> Self {
        Self {
            base_message,
            details: None,
        }
    }

    /// Add dynamic details to the error message
    pub fn with_details<S: Into<String>>(mut self, details: S) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Build the final error message, avoiding allocation if no details
    pub const fn build(self) -> Cow<'static, str> {
        match self.details {
            None => Cow::Borrowed(self.base_message),
            Some(details) => Cow::Owned(format!("{}: {}", self.base_message, details),
        }
    }
}

/// Efficient string map builder for common configuration patterns
pub struct StringMapBuilder {
    map: HashMap<&'static str, String>,
}
impl StringMapBuilder {
    /// Create a new string map builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Add a static key-value pair (no allocation for key)
    #[must_use]
    pub fn insert_static(mut self, key: &'static str, value: impl Into<String>) -> Self {
        self.map.insert(key, value.into());
        self
    }

    /// Build the final map
    pub const fn build(self) -> HashMap<String, String> {
        self.map
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    }

    /// Build the final map with static keys (more efficient)
    pub const fn build_static_keys(self) -> HashMap<&'static str, String> {
        self.map
    }
}

impl Default for StringMapBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for creating efficient error messages with static base
#[macro_export]
macro_rules! efficient_error {
    ($base:expr) => {
        $crate::utils::memory_optimization::ErrorMessageBuilder::new($base).build()
    };
    ($base:expr, $details:expr) => {
        $crate::utils::memory_optimization::ErrorMessageBuilder::new($base)
            .with_details($details)
            .build()
    };
}
/// Macro for creating string maps with static keys
#[macro_export]
macro_rules! static_string_map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut builder = $crate::utils::memory_optimization::StringMapBuilder::new();
            $(
                builder = builder.insert_static($key, $value);
            )*
            builder.build_static_keys()
        }
    };
}
/// Efficient path validation that avoids string allocations
    if path.is_empty() {
        return Err("Path cannot be empty");
    }
    if path.starts_with('/') {
        return Err("Absolute paths are not allowed");
    }

    if path.contains("..") {
        return Err("Path traversal is not allowed");
    }

    if path.contains('\0') {
        return Err("Null bytes in path are not allowed");
    }

    Ok(())
}

/// Efficient content type detection using static strings
pub const fn detect_content_type_efficient(extension: &str) -> &'static str {
    match extension {
        "txt" => "text/plain",
        "json" => "application/json",
        "xml" => "application/xml",
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        _ => "application/octet-stream",
    }
}
/// Pool of reusable string buffers for temporary operations
pub struct StringBufferPool {
    buffers: Vec<String>,
    max_size: usize,
}
impl StringBufferPool {
    /// Create a new string buffer pool
    pub const fn new(max_size: usize) -> Self {
        Self {
            buffers: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Get a buffer from the pool or create a new one
    pub fn get_buffer(&mut self) -> String {
        self.buffers
            .pop()
            .unwrap_or_else(|| String::with_capacity(1024))
    }

    /// Return a buffer to the pool
    pub fn return_buffer(&mut self, mut buffer: String) {
        if self.buffers.len() < self.max_size {
            buffer.clear();
            self.buffers.push(buffer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message_builder() {
        let static_msg = ErrorMessageBuilder::new("Static error").build();
        assert_eq!(static_msg, "Static error");

        let dynamic_msg = ErrorMessageBuilder::new("Dynamic error")
            .with_details("additional info")
            .build();
        assert_eq!(dynamic_msg, "Dynamic error: additional info");
    }

    #[test]
    fn test_path_validation() {
        assert!(validate_path_efficient("valid/path").is_ok());
        assert!(validate_path_efficient("").is_err());
        assert!(validate_path_efficient("/absolute").is_err());
        assert!(validate_path_efficient("../traversal").is_err());
        assert!(validate_path_efficient("null\0byte").is_err());
    }

    #[test]
    fn test_content_type_detection() {
        assert_eq!(detect_content_type_efficient("txt"), "text/plain");
        assert_eq!(detect_content_type_efficient("json"), "application/json");
        assert_eq!(
            detect_content_type_efficient("unknown"),
            "application/octet-stream"
        );
    }
}
