//! Network configuration defaults with environment variable support
//!
//! This module provides sovereignty-compliant network configuration that:
//! - Defaults to safe development values
//! - Allows complete override via environment variables
//! - Supports runtime discovery via Infant Discovery
//! - Zero hardcoded production assumptions
//!
//! # Examples
//!
//! ```rust
//! use nestgate_core::config::network_defaults;
//!
//! // Get API server host (defaults to 127.0.0.1, overridable via NESTGATE_API_HOST)
//! let host = network_defaults::api_host();
//!
//! // Get API server port (defaults to 8080, overridable via NESTGATE_API_PORT)
//! let port = network_defaults::api_port();
//!
//! // Get full bind address
//! let bind = network_defaults::api_bind_address();
//! ```
//!
//! # Environment Variables
//!
//! All network configuration can be overridden via environment variables:
//!
//! ## API Server
//! - `NESTGATE_API_HOST` - API server hostname (default: "127.0.0.1")
//! - `NESTGATE_API_PORT` - API server port (default: 8080)
//! - `NESTGATE_API_BIND` - Full bind address (default: "127.0.0.1:8080")
//!
//! ## Metrics
//! - `NESTGATE_METRICS_PORT` - Metrics/monitoring port (default: 9090)
//! - `NESTGATE_METRICS_BIND` - Metrics bind address (default: "127.0.0.1:9090")
//!
//! ## WebSocket
//! - `NESTGATE_WS_PORT` - WebSocket port (default: 8081)
//! - `NESTGATE_WS_BIND` - WebSocket bind address (default: "127.0.0.1:8081")
//!
//! ## Health Checks
//! - `NESTGATE_HEALTH_PORT` - Health check port (default: 8082)
//! - `NESTGATE_HEALTH_BIND` - Health check bind address (default: "127.0.0.1:8082")
//!
//! ## Storage
//! - `NESTGATE_STORAGE_PORT` - Storage metrics port (default: 5000)
//! - `NESTGATE_STORAGE_BIND` - Storage bind address (default: "127.0.0.1:5000")
//!
//! # Sovereignty Compliance
//!
//! This module ensures sovereignty compliance by:
//! - **No hardcoded endpoints**: All values configurable
//! - **Environment-first**: Always checks env vars first
//! - **Safe defaults**: Development-safe fallbacks
//! - **Discovery-ready**: Integrates with Infant Discovery
//! - **Zero vendor lock-in**: No platform assumptions

use std::env;
use crate::constants::hardcoding::{addresses, ports, timeouts};

// ==================== API SERVER ====================

/// Default API server host (overridable via `NESTGATE_API_HOST`)
///
/// # Environment Variable
/// - `NESTGATE_API_HOST`: Custom API hostname
///
/// # Default
/// Returns `"127.0.0.1"` if not set (safe for development)
#[must_use]
pub fn api_host() -> String {
    env::var("NESTGATE_API_HOST").unwrap_or_else(|_| addresses::LOCALHOST_IPV4.to_string())
}

/// Default API server port (overridable via `NESTGATE_API_PORT`)
///
/// # Environment Variable
/// - `NESTGATE_API_PORT`: Custom API port number
///
/// # Default
/// Returns `8080` if not set
#[must_use]
pub fn api_port() -> u16 {
    env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(ports::HTTP_DEFAULT)
}

/// Full API bind address (overridable via `NESTGATE_API_BIND`)
///
/// # Environment Variable
/// - `NESTGATE_API_BIND`: Full bind address (e.g., "0.0.0.0:8080")
///
/// # Default
/// Returns `"127.0.0.1:8080"` constructed from `api_host()` and `api_port()`
#[must_use]
pub fn api_bind_address() -> String {
    env::var("NESTGATE_API_BIND").unwrap_or_else(|_| format!("{}:{}", api_host(), api_port()))
}

/// API server URL (e.g., "http://127.0.0.1:8080")
///
/// # Environment Variable
/// - `NESTGATE_API_URL`: Full API URL
///
/// # Default
/// Constructs from `api_host()` and `api_port()`
#[must_use]
pub fn api_url() -> String {
    env::var("NESTGATE_API_URL").unwrap_or_else(|_| format!("http://{}:{}", api_host(), api_port()))
}

// ==================== METRICS ====================

/// Default metrics/monitoring port (overridable via `NESTGATE_METRICS_PORT`)
///
/// # Environment Variable
/// - `NESTGATE_METRICS_PORT`: Custom metrics port
///
/// # Default
/// Returns `9090` if not set
#[must_use]
pub fn metrics_port() -> u16 {
    env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(ports::METRICS_DEFAULT)
}

/// Metrics bind address (overridable via `NESTGATE_METRICS_BIND`)
///
/// # Environment Variable
/// - `NESTGATE_METRICS_BIND`: Full metrics bind address
///
/// # Default
/// Returns `"127.0.0.1:9090"`
#[must_use]
pub fn metrics_bind_address() -> String {
    env::var("NESTGATE_METRICS_BIND")
        .unwrap_or_else(|_| format!("{}:{}", api_host(), metrics_port()))
}

// ==================== WEBSOCKET ====================

/// Default WebSocket port (overridable via `NESTGATE_WS_PORT`)
///
/// # Environment Variable
/// - `NESTGATE_WS_PORT`: Custom WebSocket port
///
/// # Default
/// Returns `8082` if not set
#[must_use]
pub fn websocket_port() -> u16 {
    env::var("NESTGATE_WS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(ports::WEBSOCKET_DEFAULT)
}

/// WebSocket bind address (overridable via `NESTGATE_WS_BIND`)
///
/// # Environment Variable
/// - `NESTGATE_WS_BIND`: Full WebSocket bind address
///
/// # Default
/// Returns `"127.0.0.1:8081"`
#[must_use]
pub fn websocket_bind_address() -> String {
    env::var("NESTGATE_WS_BIND").unwrap_or_else(|_| format!("{}:{}", api_host(), websocket_port()))
}

/// WebSocket URL (e.g., "ws://127.0.0.1:8081")
///
/// # Environment Variable
/// - `NESTGATE_WS_URL`: Full WebSocket URL
///
/// # Default
/// Constructs from `api_host()` and `websocket_port()`
#[must_use]
pub fn websocket_url() -> String {
    env::var("NESTGATE_WS_URL")
        .unwrap_or_else(|_| format!("ws://{}:{}", api_host(), websocket_port()))
}

// ==================== HEALTH CHECKS ====================

/// Default health check port (overridable via `NESTGATE_HEALTH_PORT`)
///
/// # Environment Variable
/// - `NESTGATE_HEALTH_PORT`: Custom health check port
///
/// # Default
/// Returns `8081` if not set
#[must_use]
pub fn health_port() -> u16 {
    env::var("NESTGATE_HEALTH_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(ports::HEALTH_CHECK)
}

/// Health check bind address (overridable via `NESTGATE_HEALTH_BIND`)
///
/// # Environment Variable
/// - `NESTGATE_HEALTH_BIND`: Full health check bind address
///
/// # Default
/// Returns `"127.0.0.1:8082"`
#[must_use]
pub fn health_bind_address() -> String {
    env::var("NESTGATE_HEALTH_BIND").unwrap_or_else(|_| format!("{}:{}", api_host(), health_port()))
}

/// Health check URL (e.g., "http://127.0.0.1:8082/health")
///
/// # Environment Variable
/// - `NESTGATE_HEALTH_URL`: Full health check URL
///
/// # Default
/// Constructs from `api_host()` and `health_port()`
#[must_use]
pub fn health_url() -> String {
    env::var("NESTGATE_HEALTH_URL")
        .unwrap_or_else(|_| format!("http://{}:{}/health", api_host(), health_port()))
}

// ==================== STORAGE ====================

/// Default storage metrics port (overridable via `NESTGATE_STORAGE_PORT`)
///
/// # Environment Variable
/// - `NESTGATE_STORAGE_PORT`: Custom storage port
///
/// # Default
/// Returns `5000` if not set
#[must_use]
pub fn storage_port() -> u16 {
    env::var("NESTGATE_STORAGE_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(ports::STORAGE_DEFAULT)
}

/// Storage bind address (overridable via `NESTGATE_STORAGE_BIND`)
///
/// # Environment Variable
/// - `NESTGATE_STORAGE_BIND`: Full storage bind address
///
/// # Default
/// Returns `"127.0.0.1:5000"`
#[must_use]
pub fn storage_bind_address() -> String {
    env::var("NESTGATE_STORAGE_BIND")
        .unwrap_or_else(|_| format!("{}:{}", api_host(), storage_port()))
}

// ==================== TIMEOUTS ====================

/// Default connection timeout in milliseconds (overridable via `NESTGATE_CONNECT_TIMEOUT_MS`)
///
/// # Environment Variable
/// - `NESTGATE_CONNECT_TIMEOUT_MS`: Connection timeout in milliseconds
///
/// # Default
/// Returns `5000` ms (5 seconds) if not set
#[must_use]
pub fn connect_timeout_ms() -> u64 {
    env::var("NESTGATE_CONNECT_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(timeouts::CONNECT_MS)
}

/// Default request timeout in milliseconds (overridable via `NESTGATE_REQUEST_TIMEOUT_MS`)
///
/// # Environment Variable
/// - `NESTGATE_REQUEST_TIMEOUT_MS`: Request timeout in milliseconds
///
/// # Default
/// Returns `30000` ms (30 seconds) if not set
#[must_use]
pub fn request_timeout_ms() -> u64 {
    env::var("NESTGATE_REQUEST_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(timeouts::REQUEST_MS)
}

/// Default long operation timeout in milliseconds (overridable via `NESTGATE_LONG_OP_TIMEOUT_MS`)
///
/// # Environment Variable
/// - `NESTGATE_LONG_OP_TIMEOUT_MS`: Long operation timeout in milliseconds
///
/// # Default
/// Returns `300000` ms (5 minutes) if not set
#[must_use]
pub fn long_operation_timeout_ms() -> u64 {
    env::var("NESTGATE_LONG_OP_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(timeouts::LONG_OPERATION_MS)
}

// ==================== BIND ADDRESSES ====================

/// Get bind address for all interfaces (0.0.0.0)
///
/// This is useful for production deployments where you want to listen on all interfaces.
/// Always check security implications before using this in production.
///
/// # Security Note
/// Binding to 0.0.0.0 exposes the service to all network interfaces.
/// Use with caution and proper firewall configuration.
#[must_use]
pub fn bind_all_interfaces(port: u16) -> String {
    format!("{}:{port}", addresses::BIND_ALL_IPV4)
}

/// Get localhost bind address
///
/// This is the safe default for development.
#[must_use]
pub fn bind_localhost(port: u16) -> String {
    format!("{}:{port}", addresses::LOCALHOST_IPV4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_api_defaults() {
        // Clear env vars
        env::remove_var("NESTGATE_API_HOST");
        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_API_BIND");

        assert_eq!(api_host(), "127.0.0.1");
        assert_eq!(api_port(), 8080);
        assert_eq!(api_bind_address(), "127.0.0.1:8080");
        assert_eq!(api_url(), "http://127.0.0.1:8080");
    }

    #[test]
    #[serial]
    fn test_api_env_override() {
        env::set_var("NESTGATE_API_HOST", "0.0.0.0");
        env::set_var("NESTGATE_API_PORT", "9000");

        assert_eq!(api_host(), "0.0.0.0");
        assert_eq!(api_port(), 9000);
        assert_eq!(api_bind_address(), "0.0.0.0:9000");

        // Cleanup
        env::remove_var("NESTGATE_API_HOST");
        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    #[serial]
    fn test_metrics_defaults() {
        env::remove_var("NESTGATE_METRICS_PORT");
        env::remove_var("NESTGATE_METRICS_BIND");

        assert_eq!(metrics_port(), 9090);
        assert!(metrics_bind_address().contains("9090"));
    }

    #[test]
    #[serial]
    fn test_websocket_defaults() {
        env::remove_var("NESTGATE_WS_PORT");
        env::remove_var("NESTGATE_WS_BIND");

        assert_eq!(websocket_port(), 8081);
        assert!(websocket_url().starts_with("ws://"));
    }

    #[test]
    #[serial]
    fn test_timeout_defaults() {
        env::remove_var("NESTGATE_CONNECT_TIMEOUT_MS");
        env::remove_var("NESTGATE_REQUEST_TIMEOUT_MS");
        env::remove_var("NESTGATE_LONG_OP_TIMEOUT_MS");

        assert_eq!(connect_timeout_ms(), 5_000);
        assert_eq!(request_timeout_ms(), 30_000);
        assert_eq!(long_operation_timeout_ms(), 300_000);
    }

    #[test]
    fn test_bind_helpers() {
        assert_eq!(bind_all_interfaces(8080), "0.0.0.0:8080");
        assert_eq!(bind_localhost(8080), "127.0.0.1:8080");
    }

    // ==================== ADDITIONAL COMPREHENSIVE TESTS ====================

    #[test]
    #[serial]
    fn test_api_url_format() {
        env::remove_var("NESTGATE_API_URL");
        env::remove_var("NESTGATE_API_HOST");
        env::remove_var("NESTGATE_API_PORT");

        let url = api_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains(":8080"));
    }

    #[test]
    #[serial]
    fn test_custom_api_url() {
        env::set_var("NESTGATE_API_URL", "https://api.example.com");

        assert_eq!(api_url(), "https://api.example.com");

        env::remove_var("NESTGATE_API_URL");
    }

    #[test]
    #[serial]
    fn test_health_port_default() {
        env::remove_var("NESTGATE_HEALTH_PORT");

        assert_eq!(health_port(), 8082);
    }

    #[test]
    #[serial]
    fn test_health_port_custom() {
        env::set_var("NESTGATE_HEALTH_PORT", "9999");

        assert_eq!(health_port(), 9999);

        env::remove_var("NESTGATE_HEALTH_PORT");
    }

    #[test]
    #[serial]
    fn test_health_url_format() {
        env::remove_var("NESTGATE_HEALTH_URL");

        let url = health_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains(":8082"));
    }

    #[test]
    #[serial]
    fn test_storage_port_default() {
        env::remove_var("NESTGATE_STORAGE_PORT");

        assert_eq!(storage_port(), 5000);
    }

    #[test]
    #[serial]
    fn test_storage_port_custom() {
        env::set_var("NESTGATE_STORAGE_PORT", "6000");

        assert_eq!(storage_port(), 6000);

        env::remove_var("NESTGATE_STORAGE_PORT");
    }

    #[test]
    #[serial]
    fn test_websocket_url_format() {
        env::remove_var("NESTGATE_WS_URL");

        let url = websocket_url();
        assert!(url.starts_with("ws://"));
        assert!(url.contains(":8081"));
    }

    #[test]
    #[serial]
    fn test_custom_websocket_url() {
        env::set_var("NESTGATE_WS_URL", "wss://ws.example.com");

        assert_eq!(websocket_url(), "wss://ws.example.com");

        env::remove_var("NESTGATE_WS_URL");
    }

    #[test]
    #[serial]
    fn test_connect_timeout_custom() {
        env::set_var("NESTGATE_CONNECT_TIMEOUT_MS", "10000");

        assert_eq!(connect_timeout_ms(), 10_000);

        env::remove_var("NESTGATE_CONNECT_TIMEOUT_MS");
    }

    #[test]
    #[serial]
    fn test_request_timeout_custom() {
        env::set_var("NESTGATE_REQUEST_TIMEOUT_MS", "60000");

        assert_eq!(request_timeout_ms(), 60_000);

        env::remove_var("NESTGATE_REQUEST_TIMEOUT_MS");
    }

    #[test]
    #[serial]
    fn test_long_operation_timeout_custom() {
        env::set_var("NESTGATE_LONG_OP_TIMEOUT_MS", "600000");

        assert_eq!(long_operation_timeout_ms(), 600_000);

        env::remove_var("NESTGATE_LONG_OP_TIMEOUT_MS");
    }

    #[test]
    #[serial]
    fn test_invalid_port_falls_back_to_default() {
        env::set_var("NESTGATE_API_PORT", "invalid");

        // Should fallback to default 8080
        assert_eq!(api_port(), 8080);

        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    #[serial]
    fn test_empty_port_falls_back_to_default() {
        env::set_var("NESTGATE_API_PORT", "");

        // Should fallback to default 8080
        assert_eq!(api_port(), 8080);

        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    #[serial]
    fn test_out_of_range_port_parsing() {
        env::set_var("NESTGATE_API_PORT", "70000"); // Out of u16 range

        // Should fallback to default
        assert_eq!(api_port(), 8080);

        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    #[serial]
    fn test_metrics_bind_custom() {
        env::set_var("NESTGATE_METRICS_BIND", "0.0.0.0:9091");

        assert_eq!(metrics_bind_address(), "0.0.0.0:9091");

        env::remove_var("NESTGATE_METRICS_BIND");
    }

    #[test]
    #[serial]
    fn test_websocket_bind_custom() {
        env::set_var("NESTGATE_WS_BIND", "0.0.0.0:8082");

        assert!(websocket_bind_address().contains("8082"));

        env::remove_var("NESTGATE_WS_BIND");
    }

    #[test]
    #[serial]
    fn test_health_bind_custom() {
        env::set_var("NESTGATE_HEALTH_BIND", "0.0.0.0:8083");

        assert!(health_bind_address().contains("8083"));

        env::remove_var("NESTGATE_HEALTH_BIND");
    }

    #[test]
    #[serial]
    fn test_storage_bind_custom() {
        env::set_var("NESTGATE_STORAGE_BIND", "0.0.0.0:5001");

        assert!(storage_bind_address().contains("5001"));

        env::remove_var("NESTGATE_STORAGE_BIND");
    }

    #[test]
    fn test_bind_all_interfaces_various_ports() {
        assert_eq!(bind_all_interfaces(80), "0.0.0.0:80");
        assert_eq!(bind_all_interfaces(443), "0.0.0.0:443");
        assert_eq!(bind_all_interfaces(3000), "0.0.0.0:3000");
        assert_eq!(bind_all_interfaces(65535), "0.0.0.0:65535");
    }

    #[test]
    fn test_bind_localhost_various_ports() {
        assert_eq!(bind_localhost(80), "127.0.0.1:80");
        assert_eq!(bind_localhost(443), "127.0.0.1:443");
        assert_eq!(bind_localhost(3000), "127.0.0.1:3000");
        assert_eq!(bind_localhost(65535), "127.0.0.1:65535");
    }

    #[test]
    #[serial]
    fn test_all_defaults_when_no_env_vars() {
        // Clear all environment variables
        env::remove_var("NESTGATE_API_HOST");
        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_METRICS_PORT");
        env::remove_var("NESTGATE_WS_PORT");
        env::remove_var("NESTGATE_HEALTH_PORT");
        env::remove_var("NESTGATE_STORAGE_PORT");
        env::remove_var("NESTGATE_CONNECT_TIMEOUT_MS");
        env::remove_var("NESTGATE_REQUEST_TIMEOUT_MS");
        env::remove_var("NESTGATE_LONG_OP_TIMEOUT_MS");

        // Verify all defaults
        assert_eq!(api_host(), "127.0.0.1");
        assert_eq!(api_port(), 8080);
        assert_eq!(metrics_port(), 9090);
        assert_eq!(websocket_port(), 8081);
        assert_eq!(health_port(), 8082);
        assert_eq!(storage_port(), 5000);
        assert_eq!(connect_timeout_ms(), 5_000);
        assert_eq!(request_timeout_ms(), 30_000);
        assert_eq!(long_operation_timeout_ms(), 300_000);
    }

    #[test]
    #[serial]
    fn test_sovereignty_compliance_no_hardcoded_values() {
        // This test validates sovereignty: all values must be overridable
        env::set_var("NESTGATE_API_HOST", "custom.host");
        env::set_var("NESTGATE_API_PORT", "12345");
        env::set_var("NESTGATE_METRICS_PORT", "54321");

        assert_eq!(api_host(), "custom.host");
        assert_eq!(api_port(), 12345);
        assert_eq!(metrics_port(), 54321);

        // Cleanup
        env::remove_var("NESTGATE_API_HOST");
        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_METRICS_PORT");
    }

    #[test]
    #[serial]
    fn test_api_bind_respects_full_override() {
        env::set_var("NESTGATE_API_BIND", "192.168.1.100:9999");

        assert_eq!(api_bind_address(), "192.168.1.100:9999");

        env::remove_var("NESTGATE_API_BIND");
    }

    #[test]
    #[serial]
    fn test_timeout_values_are_reasonable() {
        env::remove_var("NESTGATE_CONNECT_TIMEOUT_MS");
        env::remove_var("NESTGATE_REQUEST_TIMEOUT_MS");
        env::remove_var("NESTGATE_LONG_OP_TIMEOUT_MS");

        // Connect timeout should be short (< 10s)
        assert!(connect_timeout_ms() < 10_000);

        // Request timeout should be medium (< 1 minute)
        assert!(request_timeout_ms() < 60_000);

        // Long operation timeout should be substantial but not infinite
        assert!(long_operation_timeout_ms() > 60_000);
        assert!(long_operation_timeout_ms() < 600_000); // Less than 10 minutes
    }

    #[test]
    #[serial]
    fn test_invalid_timeout_falls_back() {
        env::set_var("NESTGATE_CONNECT_TIMEOUT_MS", "not_a_number");

        assert_eq!(connect_timeout_ms(), 5_000);

        env::remove_var("NESTGATE_CONNECT_TIMEOUT_MS");
    }
}
