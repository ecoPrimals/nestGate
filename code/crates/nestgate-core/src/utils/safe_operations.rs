//! Error handling improvements - Replace unwraps with proper Result propagation
//!
//! This module provides utilities to migrate from unwrap() to proper error handling.

use nestgate_core::error::NestGateError;
use nestgate_core::Result;

/// Parse environment variable with proper error context
pub fn parse_env_var<T>(key: &str) -> Result<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    std::env::var(key)
        .map_err(|e| {
            NestGateError::configuration_error(
                key,
                &format!("Environment variable '{}' not found: {}", key, e),
            )
        })?
        .parse()
        .map_err(|e| {
            NestGateError::configuration_error(
                key,
                &format!("Failed to parse '{}': {}", key, e),
            )
        })
}

/// Parse optional environment variable
pub fn parse_env_var_optional<T>(key: &str) -> Result<Option<T>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match std::env::var(key) {
        Ok(value) => value
            .parse()
            .map(Some)
            .map_err(|e| {
                NestGateError::configuration_error(
                    key,
                    &format!("Failed to parse '{}': {}", key, e),
                )
            }),
        Err(_) => Ok(None),
    }
}

/// Safe string operations with error context
pub trait SafeStringExt {
    fn safe_parse<T>(&self) -> Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display;
}

impl SafeStringExt for str {
    fn safe_parse<T>(&self) -> Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        self.parse().map_err(|e| {
            NestGateError::validation_error(&format!("Failed to parse '{}': {}", self, e))
        })
    }
}

/// Safe collection operations
pub trait SafeCollectionExt<T> {
    fn safe_get(&self, index: usize) -> Result<&T>;
    fn safe_first(&self) -> Result<&T>;
    fn safe_last(&self) -> Result<&T>;
}

impl<T> SafeCollectionExt<T> for [T] {
    fn safe_get(&self, index: usize) -> Result<&T> {
        self.get(index).ok_or_else(|| {
            NestGateError::validation_error(&format!(
                "Index {} out of bounds (len: {})",
                index,
                self.len()
            ))
        })
    }

    fn safe_first(&self) -> Result<&T> {
        self.first().ok_or_else(|| {
            NestGateError::validation_error("Cannot get first element: collection is empty")
        })
    }

    fn safe_last(&self) -> Result<&T> {
        self.last().ok_or_else(|| {
            NestGateError::validation_error("Cannot get last element: collection is empty")
        })
    }
}

// Also implement for Vec<T> for convenience
impl<T> SafeCollectionExt<T> for Vec<T> {
    fn safe_get(&self, index: usize) -> Result<&T> {
        self.as_slice().safe_get(index)
    }

    fn safe_first(&self) -> Result<&T> {
        self.as_slice().safe_first()
    }

    fn safe_last(&self) -> Result<&T> {
        self.as_slice().safe_last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env_var() {
        std::env::set_var("TEST_PORT", "8080");
        let port: u16 = parse_env_var("TEST_PORT").unwrap();
        assert_eq!(port, 8080);
        std::env::remove_var("TEST_PORT");
    }

    #[test]
    fn test_parse_env_var_missing() {
        std::env::remove_var("MISSING_VAR");
        let result: Result<String> = parse_env_var("MISSING_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_env_var_invalid() {
        std::env::set_var("INVALID_PORT", "not_a_number");
        let result: Result<u16> = parse_env_var("INVALID_PORT");
        assert!(result.is_err());
        std::env::remove_var("INVALID_PORT");
    }

    #[test]
    fn test_safe_string_parse() {
        let result: Result<i32> = "42".safe_parse();
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_safe_string_parse_invalid() {
        let result: Result<i32> = "invalid".safe_parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_get() {
        let vec = vec![1, 2, 3];
        assert_eq!(*vec.safe_get(1).unwrap(), 2);
    }

    #[test]
    fn test_safe_get_out_of_bounds() {
        let vec = vec![1, 2, 3];
        let result = vec.safe_get(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_first() {
        let vec = vec!["a", "b", "c"];
        assert_eq!(*vec.safe_first().unwrap(), "a");
    }

    #[test]
    fn test_safe_first_empty() {
        let vec: Vec<i32> = vec![];
        let result = vec.safe_first();
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_last() {
        let vec = vec![10, 20, 30];
        assert_eq!(*vec.safe_last().unwrap(), 30);
    }

    #[test]
    fn test_safe_last_empty() {
        let vec: Vec<String> = vec![];
        let result = vec.safe_last();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_env_var_optional_present() {
        std::env::set_var("OPTIONAL_VAR", "123");
        let result: Result<Option<i32>> = parse_env_var_optional("OPTIONAL_VAR");
        assert_eq!(result.unwrap(), Some(123));
        std::env::remove_var("OPTIONAL_VAR");
    }

    #[test]
    fn test_parse_env_var_optional_absent() {
        std::env::remove_var("ABSENT_VAR");
        let result: Result<Option<String>> = parse_env_var_optional("ABSENT_VAR");
        assert_eq!(result.unwrap(), None);
    }
}

