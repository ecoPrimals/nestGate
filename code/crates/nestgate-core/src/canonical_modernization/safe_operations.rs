use crate::NestGateError;
//
// **CANONICAL MODERNIZATION COMPLETE** - Safe parsing and operation utilities
// that provide error-free operations with fallback handling.

use crate::{Result, NestGateError};
use std::str::FromStr;

/// Result type for safe operations
pub type SafeOperationResult<T> = Result<T>;

/// Safely parse a string to any type that implements FromStr
pub fn safe_parse<T>(input: &str) -> SafeOperationResult<T>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    input.parse().map_err(|e| {
        NestGateError::validation_error(
            "parse_operation",
            format!("Failed to parse '{}': {}", input, e),
        )
    })
}

/// Safely convert between types
pub fn safe_convert<T, U>(value: T) -> SafeOperationResult<U>
where
    T: TryInto<U>,
    T::Error: std::fmt::Display,
{
    value.try_into().map_err(|e| {
        NestGateError::validation_error(
            "convert_operation",
            format!("Failed to convert value: {}", e),
        )
    })
}

/// Safely parse with fallback value
pub fn safe_parse_with_fallback<T>(input: &str, fallback: T) -> T
where
    T: FromStr + Clone,
{
    input.parse().unwrap_or(fallback)
}

/// Safely parse IP address with fallback
pub fn safe_parse_ip_with_fallback(input: &str, fallback: &str) -> String {
    use std::net::IpAddr;
    
    if input.parse::<IpAddr>().is_ok() {
        input.to_string()
    } else {
        fallback.to_string()
    }
}

/// Safely parse port with fallback
pub fn safe_parse_port_with_fallback(input: &str, fallback: u16) -> u16 {
    input.parse().unwrap_or(fallback)
}

/// Safely parse string with fallback
pub fn safe_parse_string_with_fallback(input: Option<&str>, fallback: &str) -> String {
    input.unwrap_or(fallback).to_string()
}

/// Safely parse boolean with fallback
pub fn safe_parse_bool(input: &str, fallback: bool) -> bool {
    match input.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => fallback,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_parse_success() {
        let result: SafeOperationResult<i32> = safe_parse("42");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_safe_parse_failure() {
        let result: SafeOperationResult<i32> = safe_parse("not_a_number");
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_parse_with_fallback() {
        assert_eq!(safe_parse_with_fallback("42", 0), 42);
        assert_eq!(safe_parse_with_fallback("not_a_number", 99), 99);
    }

    #[test]
    fn test_safe_parse_bool() {
        assert_eq!(safe_parse_bool("true", false), true);
        assert_eq!(safe_parse_bool("false", true), false);
        assert_eq!(safe_parse_bool("invalid", true), true);
    }
} 