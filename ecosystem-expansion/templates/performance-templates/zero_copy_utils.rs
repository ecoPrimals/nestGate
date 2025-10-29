/// Zero-copy string and data processing utilities for API compatibility
use std::borrow::Cow;

/// Zero-copy string processing utilities
pub struct StringUtils;

impl StringUtils {
    /// Create a static Cow string from a string slice
    pub fn static_cow(s: &str) -> Cow<'static, str> {
        Cow::Owned(s.to_string())
    }
}

/// Optimize command output with zero-copy patterns
pub fn optimize_command_output(output: &[u8]) -> Cow<str> {
    String::from_utf8_lossy(output)
}

/// Zero-copy line processing
pub fn lines_zero_copy(input: &str) -> impl Iterator<Item = &str> {
    input.lines()
}
