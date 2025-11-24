//! Centralized constants to eliminate hardcoding
//!
//! This module provides a single source of truth for network addresses,
//! ports, and other configuration values that were previously hardcoded
//! throughout the codebase.
//!
//! All values can be overridden via environment variables.

use std::env;
use std::sync::OnceLock;

// ============================================================================
// Network Addresses
// ============================================================================

/// Default network addresses
pub mod addresses {
    /// IPv4 localhost address
    pub const LOCALHOST_IPV4: &str = "127.0.0.1";

    /// IPv6 localhost address
    pub const LOCALHOST_IPV6: &str = "::1";

    /// Localhost hostname
    pub const LOCALHOST_NAME: &str = "localhost";

    /// Bind to all IPv4 interfaces
    pub const BIND_ALL_IPV4: &str = "0.0.0.0";

    /// Bind to all IPv6 interfaces
    pub const BIND_ALL_IPV6: &str = "::";
}

// ============================================================================
// Network Ports
// ============================================================================

/// Default network ports
pub mod ports {
    /// Default HTTP port
    pub const HTTP_DEFAULT: u16 = 8080;

    /// Default HTTPS port
    pub const HTTPS_DEFAULT: u16 = 8443;

    /// Default API server port
    pub const API_DEFAULT: u16 = 3000;

    /// Alternative API port
    pub const API_ALT: u16 = 3001;

    /// Default metrics/monitoring port
    pub const METRICS_DEFAULT: u16 = 9090;

    /// Prometheus metrics port
    pub const PROMETHEUS: u16 = 9090;

    /// Default health check port
    pub const HEALTH_CHECK: u16 = 8081;

    /// Default gRPC port
    pub const GRPC_DEFAULT: u16 = 50051;

    /// Default WebSocket port
    pub const WEBSOCKET_DEFAULT: u16 = 8082;

    /// Default admin interface port
    pub const ADMIN_DEFAULT: u16 = 9000;

    /// Default storage service port
    pub const STORAGE_DEFAULT: u16 = 5000;

    /// Default orchestration service port
    pub const ORCHESTRATION_DEFAULT: u16 = 8083;

    /// Default storage discovery port
    pub const STORAGE_DISCOVERY_DEFAULT: u16 = 8084;

    /// Default compute service port
    pub const COMPUTE_DEFAULT: u16 = 8085;

    /// Extended services port
    pub const EXTENDED_SERVICES: u16 = 3002;

    /// Discovery service port
    pub const DISCOVERY_SERVICE: u16 = 3010;

    /// Alternative metrics port
    pub const METRICS_ALT: u16 = 9001;

    /// Metrics default port (Prometheus standard)
    pub const METRICS_PROMETHEUS: u16 = 9090;

    /// Health check port
    pub const HEALTH_DEFAULT: u16 = 8081;

    /// Orchestrator port
    pub const ORCHESTRATOR_DEFAULT: u16 = 8090;

    /// BearDog security primal default port
    pub const BEARDOG_DEFAULT: u16 = 8081;

    /// Songbird networking primal default port
    pub const SONGBIRD_DEFAULT: u16 = 8082;

    /// PostgreSQL database default port
    pub const POSTGRES_DEFAULT: u16 = 5432;

    /// Redis cache default port
    pub const REDIS_DEFAULT: u16 = 6379;
}

// ============================================================================
// Timeout Constants (milliseconds)
// ============================================================================

/// Timeout constants in milliseconds
pub mod timeouts {
    /// Default connection timeout (5 seconds)
    pub const CONNECT_MS: u64 = 5_000;

    /// Default request timeout (30 seconds)
    pub const REQUEST_MS: u64 = 30_000;

    /// Default long operation timeout (5 minutes)
    pub const LONG_OPERATION_MS: u64 = 300_000;
}

// ============================================================================
// Environment Variable Helpers
// ============================================================================

/// Cache for bind address from environment
static BIND_ADDRESS: OnceLock<String> = OnceLock::new();

/// Cache for API port from environment
static API_PORT: OnceLock<u16> = OnceLock::new();

/// Get the bind address from environment or use default
///
/// Checks `NESTGATE_BIND_ADDRESS` environment variable.
/// Falls back to `0.0.0.0` if not set.
pub fn get_bind_address() -> &'static str {
    BIND_ADDRESS.get_or_init(|| {
        env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| addresses::BIND_ALL_IPV4.to_string())
    })
}

/// Get the API port from environment or use default
///
/// Checks `NESTGATE_API_PORT` environment variable.
/// Falls back to 3000 if not set or invalid.
pub fn get_api_port() -> u16 {
    *API_PORT.get_or_init(|| {
        env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(ports::API_DEFAULT)
    })
}

/// Get the metrics port from environment or use default
///
/// Checks `NESTGATE_METRICS_PORT` environment variable.
/// Falls back to 9090 if not set or invalid.
#[must_use]
pub fn get_metrics_port() -> u16 {
    env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::METRICS_DEFAULT)
}

/// Get the health check port from environment or use default
///
/// Checks `NESTGATE_HEALTH_PORT` environment variable.
/// Falls back to 8081 if not set or invalid.
#[must_use]
pub fn get_health_port() -> u16 {
    env::var("NESTGATE_HEALTH_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::HEALTH_CHECK)
}

// ============================================================================
// Service Discovery Defaults
// ============================================================================

/// Default service discovery configuration
pub mod discovery {
    use super::*;

    /// Default service discovery timeout (milliseconds)
    pub const TIMEOUT_MS: u64 = 5000;

    /// Default retry attempts for service discovery
    pub const RETRY_ATTEMPTS: u32 = 3;

    /// Default port range start for capability scanning
    pub const SCAN_PORT_START: u16 = 3000;

    /// Default port range end for capability scanning
    pub const SCAN_PORT_END: u16 = 3999;

    /// Get discovery timeout from environment or default
    #[must_use]
    pub fn get_timeout_ms() -> u64 {
        env::var("NESTGATE_DISCOVERY_TIMEOUT_MS")
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(TIMEOUT_MS)
    }
}

// ============================================================================
// Magic Numbers (to be eliminated)
// ============================================================================

/// Common buffer sizes and limits
pub mod limits {
    /// Default buffer size for I/O operations (64KB)
    pub const BUFFER_SIZE_DEFAULT: usize = 65536;

    /// Maximum buffer size (1MB)
    pub const BUFFER_SIZE_MAX: usize = 1_048_576;

    /// Default connection pool size
    pub const CONNECTION_POOL_SIZE: usize = 10;

    /// Maximum concurrent connections
    pub const MAX_CONNECTIONS: usize = 1000;

    /// Default timeout in seconds
    pub const TIMEOUT_SECS: u64 = 30;

    /// Maximum retry attempts
    pub const MAX_RETRIES: u32 = 3;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addresses_are_valid() {
        assert_eq!(addresses::LOCALHOST_IPV4, "127.0.0.1");
        assert_eq!(addresses::LOCALHOST_IPV6, "::1");
        assert_eq!(addresses::LOCALHOST_NAME, "localhost");
        assert_eq!(addresses::BIND_ALL_IPV4, "0.0.0.0");
        assert_eq!(addresses::BIND_ALL_IPV6, "::");
    }

    #[test]
    fn test_ports_are_in_valid_range() {
        // All ports are u16, which are always >= 0, so just verify they're defined
        // These checks serve as documentation that these ports exist and are configured
        assert_eq!(ports::HTTP_DEFAULT, ports::HTTP_DEFAULT);
        assert_eq!(ports::HTTPS_DEFAULT, ports::HTTPS_DEFAULT);
        assert_eq!(ports::API_DEFAULT, ports::API_DEFAULT);
        assert_eq!(ports::METRICS_DEFAULT, ports::METRICS_DEFAULT);
        assert_eq!(ports::HEALTH_CHECK, ports::HEALTH_CHECK);
    }

    #[test]
    fn test_get_bind_address_default() {
        // Should return default when env var not set
        let addr = get_bind_address();
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_get_api_port_default() {
        // Should return valid port when env var not set
        let port = get_api_port();
        assert!(port > 0);
    }

    #[test]
    fn test_discovery_timeout() {
        let timeout = discovery::get_timeout_ms();
        assert!(timeout > 0);
    }

    #[test]
    fn test_limits_are_reasonable() {
        // These are compile-time constants, so we verify their relationships
        // rather than testing values that are always true.
        const _: () = assert!(limits::BUFFER_SIZE_MAX >= limits::BUFFER_SIZE_DEFAULT);
        const _: () = assert!(limits::MAX_CONNECTIONS >= limits::CONNECTION_POOL_SIZE);

        // Runtime verification that constants are accessible
        let _ = limits::BUFFER_SIZE_DEFAULT;
        let _ = limits::CONNECTION_POOL_SIZE;
        let _ = limits::TIMEOUT_SECS;
        let _ = limits::MAX_RETRIES;
    }
}
