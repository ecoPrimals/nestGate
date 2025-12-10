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

use crate::constants::hardcoding::addresses;

// Import the configuration module for concurrent-safe access
use super::network_defaults_v2_config::NetworkDefaultsV2Config;

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
    NetworkDefaultsV2Config::from_env().api_host()
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
    NetworkDefaultsV2Config::from_env().api_port()
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
    NetworkDefaultsV2Config::from_env().api_bind_address()
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
    NetworkDefaultsV2Config::from_env().api_url()
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
    NetworkDefaultsV2Config::from_env().metrics_port()
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
    NetworkDefaultsV2Config::from_env().metrics_bind_address()
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
    NetworkDefaultsV2Config::from_env().websocket_port()
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
    NetworkDefaultsV2Config::from_env().websocket_bind_address()
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
    NetworkDefaultsV2Config::from_env().websocket_url()
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
    NetworkDefaultsV2Config::from_env().health_port()
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
    NetworkDefaultsV2Config::from_env().health_bind_address()
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
    NetworkDefaultsV2Config::from_env().health_url()
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
    NetworkDefaultsV2Config::from_env().storage_port()
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
    NetworkDefaultsV2Config::from_env().storage_bind_address()
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
    NetworkDefaultsV2Config::from_env().connect_timeout_ms()
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
    NetworkDefaultsV2Config::from_env().request_timeout_ms()
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
    NetworkDefaultsV2Config::from_env().long_operation_timeout_ms()
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

    // **MODERN CONCURRENT-SAFE TESTS**
    // All tests use NetworkDefaultsV2Config directly instead of polluting env vars

    #[test]
    fn test_api_defaults() {
        let config = NetworkDefaultsV2Config::new();
        assert_eq!(config.api_host(), "127.0.0.1");
        assert_eq!(config.api_port(), 8080);
        assert_eq!(config.api_bind_address(), "127.0.0.1:8080");
        assert_eq!(config.api_url(), "http://127.0.0.1:8080");
    }

    #[test]
    fn test_api_custom_config() {
        let config = NetworkDefaultsV2Config::new()
            .with_api_host("0.0.0.0".to_string())
            .with_api_port(9000);

        assert_eq!(config.api_host(), "0.0.0.0");
        assert_eq!(config.api_port(), 9000);
        assert_eq!(config.api_bind_address(), "0.0.0.0:9000");
    }

    #[test]
    fn test_metrics_defaults() {
        let config = NetworkDefaultsV2Config::new();
        assert_eq!(config.metrics_port(), 9090);
        assert!(config.metrics_bind_address().contains("9090"));
    }

    #[test]
    fn test_websocket_defaults() {
        let config = NetworkDefaultsV2Config::new();
        assert_eq!(config.websocket_port(), 8082);
        assert!(config.websocket_url().starts_with("ws://"));
    }

    #[test]
    fn test_timeout_defaults() {
        let config = NetworkDefaultsV2Config::new();
        assert_eq!(config.connect_timeout_ms(), 5_000);
        assert_eq!(config.request_timeout_ms(), 30_000);
        assert_eq!(config.long_operation_timeout_ms(), 300_000);
    }

    #[test]
    fn test_bind_helpers() {
        assert_eq!(bind_all_interfaces(8080), "0.0.0.0:8080");
        assert_eq!(bind_localhost(8080), "127.0.0.1:8080");
    }

    // ==================== ADDITIONAL COMPREHENSIVE TESTS ====================

    #[test]
    fn test_api_url_format() {
        let config = NetworkDefaultsV2Config::new();
        let url = config.api_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains(":8080"));
    }

    #[test]
    fn test_health_port_default() {
        let config = NetworkDefaultsV2Config::new();
        assert_eq!(config.health_port(), 8081);
    }

    #[test]
    fn test_health_port_custom() {
        let config = NetworkDefaultsV2Config::new().with_health_port(9999);
        assert_eq!(config.health_port(), 9999);
    }

    #[test]
    fn test_health_url_format() {
        let config = NetworkDefaultsV2Config::new();
        let url = config.health_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains(":8081"));
    }

    #[test]
    fn test_storage_port_default() {
        let config = NetworkDefaultsV2Config::new();
        assert_eq!(config.storage_port(), 5000);
    }

    #[test]
    fn test_storage_port_custom() {
        let config = NetworkDefaultsV2Config::new().with_storage_port(6000);
        assert_eq!(config.storage_port(), 6000);
    }

    #[test]
    fn test_websocket_url_format() {
        let config = NetworkDefaultsV2Config::new();
        let url = config.websocket_url();
        assert!(url.starts_with("ws://"));
        assert!(url.contains(":8082"));
    }

    #[test]
    fn test_connect_timeout_custom() {
        let config = NetworkDefaultsV2Config::new().with_connect_timeout_ms(10_000);
        assert_eq!(config.connect_timeout_ms(), 10_000);
    }

    #[test]
    fn test_request_timeout_custom() {
        let config = NetworkDefaultsV2Config::new().with_request_timeout_ms(60_000);
        assert_eq!(config.request_timeout_ms(), 60_000);
    }

    #[test]
    fn test_long_operation_timeout_custom() {
        let config = NetworkDefaultsV2Config::new().with_long_operation_timeout_ms(600_000);
        assert_eq!(config.long_operation_timeout_ms(), 600_000);
    }

    #[test]
    fn test_port_type_safety() {
        // Config uses u16, ensuring type safety at compile time
        let config = NetworkDefaultsV2Config::new();
        assert_eq!(config.api_port(), 8080);
    }

    #[test]
    fn test_metrics_bind_custom() {
        let config = NetworkDefaultsV2Config::new()
            .with_api_host("0.0.0.0".to_string())
            .with_metrics_port(9091);
        let bind = config.metrics_bind_address();
        assert!(bind.contains("9091"));
    }

    #[test]
    fn test_websocket_bind_custom() {
        let config = NetworkDefaultsV2Config::new()
            .with_api_host("0.0.0.0".to_string())
            .with_websocket_port(8082);
        assert!(config.websocket_bind_address().contains("8082"));
    }

    #[test]
    fn test_health_bind_custom() {
        let config = NetworkDefaultsV2Config::new()
            .with_api_host("0.0.0.0".to_string())
            .with_health_port(8083);
        assert!(config.health_bind_address().contains("8083"));
    }

    #[test]
    fn test_storage_bind_custom() {
        let config = NetworkDefaultsV2Config::new()
            .with_api_host("0.0.0.0".to_string())
            .with_storage_port(5001);
        assert!(config.storage_bind_address().contains("5001"));
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
    fn test_all_defaults_when_no_env_vars() {
        // Test defaults via config (no env pollution)
        let config = NetworkDefaultsV2Config::new();

        // Verify all defaults
        assert_eq!(config.api_host(), "127.0.0.1");
        assert_eq!(config.api_port(), 8080);
        assert_eq!(metrics_port(), 9090);
        assert_eq!(websocket_port(), 8082); // WEBSOCKET_DEFAULT = 8082
        assert_eq!(health_port(), 8081); // HEALTH_CHECK = 8081
        assert_eq!(storage_port(), 5000);
        assert_eq!(connect_timeout_ms(), 5_000);
        assert_eq!(request_timeout_ms(), 30_000);
        assert_eq!(long_operation_timeout_ms(), 300_000);
    }

    #[test]
    fn test_sovereignty_compliance_no_hardcoded_values() {
        // This test validates sovereignty: all values must be overridable
        let config = NetworkDefaultsV2Config::new()
            .with_api_host("custom.host".to_string())
            .with_api_port(12345)
            .with_metrics_port(54321);

        assert_eq!(config.api_host(), "custom.host");
        assert_eq!(config.api_port(), 12345);
        assert_eq!(config.metrics_port(), 54321);
    }

    #[test]
    fn test_api_bind_respects_full_override() {
        let config = NetworkDefaultsV2Config::new()
            .with_api_host("192.168.1.100".to_string())
            .with_api_port(9999);
        assert_eq!(config.api_bind_address(), "192.168.1.100:9999");
    }

    #[test]
    fn test_timeout_values_are_reasonable() {
        let config = NetworkDefaultsV2Config::new();

        // Connect timeout should be short (< 10s)
        assert!(config.connect_timeout_ms() < 10_000);

        // Request timeout should be medium (< 1 minute)
        assert!(config.request_timeout_ms() < 60_000);

        // Long operation timeout should be substantial but not infinite
        assert!(config.long_operation_timeout_ms() > 60_000);
        assert!(config.long_operation_timeout_ms() < 600_000); // Less than 10 minutes
    }
}

#[cfg(test)]
#[path = "network_defaults_tests.rs"]
mod network_defaults_tests;
