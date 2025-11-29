//! **NETWORK TEST MACROS**
//!
//! Macros to reduce boilerplate in network tests.
//! This significantly reduces code duplication.

/// Assert that a port creation succeeds
#[macro_export]
macro_rules! assert_port_valid {
    ($port_num:expr) => {
        let port = Port::new($port_num);
        assert!(port.is_ok(), "Port {} should be valid", $port_num);
    };
}

/// Assert that a port creation fails
#[macro_export]
macro_rules! assert_port_invalid {
    ($port_num:expr) => {
        let port = Port::new($port_num);
        assert!(port.is_err(), "Port {} should be invalid", $port_num);
    };
}

/// Test that an HTTP method is safe
#[macro_export]
macro_rules! assert_method_safe {
    ($method:expr) => {
        assert!($method.is_safe(), "{:?} should be safe", $method);
    };
}

/// Test that an HTTP method is unsafe
#[macro_export]
macro_rules! assert_method_unsafe {
    ($method:expr) => {
        assert!(!$method.is_safe(), "{:?} should be unsafe", $method);
    };
}

/// Test that a status code matches expected value
#[macro_export]
macro_rules! assert_status_code {
    ($status:expr, $expected:expr) => {
        assert_eq!($status.as_u16(), $expected, "Status code mismatch");
    };
}

#[cfg(test)]
mod tests {
    use super::super::client::{Method, Port, StatusCode};
use crate::config::port_config;

    #[test]
    fn test_macro_assert_port_valid() {
        assert_port_valid!(port_config::api_port());
    }

    #[test]
    fn test_macro_assert_port_invalid() {
        assert_port_invalid!(0);
    }

    #[test]
    fn test_macro_assert_method_safe() {
        assert_method_safe!(Method::Get);
    }

    #[test]
    fn test_macro_assert_method_unsafe() {
        assert_method_unsafe!(Method::Post);
    }

    #[test]
    fn test_macro_assert_status_code() {
        assert_status_code!(StatusCode::OK, 200);
    }
}
