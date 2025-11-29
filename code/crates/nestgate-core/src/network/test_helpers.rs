//! **NETWORK TEST HELPERS**
//!
//! Common test utilities and helper functions for network tests.
//! This module reduces duplication and improves test maintainability.

use super::client::*;
use crate::config::port_config;

/// Create a test port using the configured API port
#[must_use]
pub fn test_api_port() -> Port {
    Port::new(port_config::api_port()).expect("Failed to create test API port")
}

/// Create a test port using the configured Grafana port
#[must_use]
pub fn test_grafana_port() -> Port {
    Port::new(port_config::grafana_port()).expect("Failed to create test Grafana port")
}

/// Create a test port using the configured admin port
#[must_use]
pub fn test_admin_port() -> Port {
    Port::new(port_config::admin_port()).expect("Failed to create test admin port")
}

/// Create a test endpoint with HTTP scheme
#[must_use]
pub fn test_http_endpoint(host: &str, port: Port) -> Endpoint {
    Endpoint::http(host.to_string(), port)
}

/// Create a test endpoint with HTTPS scheme
#[must_use]
pub fn test_https_endpoint(host: &str, port: Port) -> Endpoint {
    Endpoint::https(host.to_string(), port)
}

/// Create a simple GET request for testing
#[must_use]
pub fn test_get_request<'a>(path: &'a str) -> Request<'a> {
    Request::get(path)
}

/// Create a POST request with JSON body for testing
#[must_use]
pub fn test_post_json_request<'a>(path: &'a str, json_body: &'a str) -> Request<'a> {
    Request::post_json(path, json_body)
}

/// Assert that a port is in the valid range
pub fn assert_valid_port(port_num: u16) {
    assert!(port_num > 0, "Port must be greater than 0");
    // Note: u16 max is 65535, so this check is always true but kept for documentation
    #[allow(unused_comparisons)]
    assert!(port_num <= 65535, "Port must be <= 65535");
}

/// Assert that an endpoint URL matches expected format
pub fn assert_endpoint_format(endpoint: &Endpoint, expected_host: &str, expected_port: u16) {
    let url = endpoint.url();
    assert!(url.contains(expected_host), "URL should contain host");
    assert!(
        url.contains(&expected_port.to_string()),
        "URL should contain port"
    );
}

/// Create a test timeout in milliseconds
#[must_use]
pub fn test_timeout_ms(milliseconds: u64) -> TimeoutMs {
    TimeoutMs::new(milliseconds)
}

/// Create a test timeout in seconds (converted to ms)
#[must_use]
pub fn test_timeout_secs(seconds: u64) -> TimeoutMs {
    TimeoutMs::new(seconds * 1000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_test_api_port() {
        let port = test_api_port();
        assert_eq!(port.get(), port_config::api_port());
    }

    #[test]
    fn test_helper_test_http_endpoint() {
        let port = test_api_port();
        let endpoint = test_http_endpoint("api.example.com", port);
        assert!(endpoint.url().starts_with("http://"));
        assert!(endpoint.url().contains("api.example.com"));
    }

    #[test]
    fn test_helper_test_get_request() {
        let request = test_get_request("/api/status");
        // Request created successfully - basic smoke test
        assert_eq!(request.path, "/api/status");
    }

    #[test]
    fn test_helper_test_post_json_request() {
        let request = test_post_json_request("/api/data", r#"{"key": "value"}"#);
        // Request created successfully - basic smoke test
        assert_eq!(request.path, "/api/data");
    }

    #[test]
    fn test_helper_assert_valid_port() {
        assert_valid_port(80);
        assert_valid_port(443);
        assert_valid_port(port_config::api_port());
    }

    #[test]
    #[should_panic(expected = "Port must be greater than 0")]
    fn test_helper_assert_valid_port_zero() {
        assert_valid_port(0);
    }

    #[test]
    fn test_helper_timeout() {
        let timeout = test_timeout_ms(5000);
        assert_eq!(
            timeout.as_duration(),
            std::time::Duration::from_millis(5000)
        );

        let timeout = test_timeout_secs(5);
        assert_eq!(timeout.as_duration(), std::time::Duration::from_secs(5));
    }
}
