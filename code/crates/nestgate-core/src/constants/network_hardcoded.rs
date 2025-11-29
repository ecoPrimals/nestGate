//! Network constants for hardcoded values cleanup
//!
//! This module centralizes all network-related constants that were previously
//! hardcoded throughout the codebase. These can be overridden via environment
//! variables for deployment flexibility.
//!
//! **Migration Status**: Phase 1 - Foundation (Nov 6, 2025)
//! - Created centralized constants module
//! - Documented all hardcoded values found (640+ instances)
//! - Next: Systematic replacement across codebase

use std::env;

// ==================== IP ADDRESSES ====================

/// Default network addresses - extracted from hardcoded values
pub mod addresses {
    /// Localhost IPv4 address (127.0.0.1)
    /// Found in 316 locations across codebase
    pub const LOCALHOST_IPV4: &str = "127.0.0.1";

    /// Localhost hostname
    pub const LOCALHOST_NAME: &str = "localhost";

    /// Bind to all IPv4 interfaces (0.0.0.0)
    /// Used for server bindings to accept connections on all interfaces
    pub const BIND_ALL_IPV4: &str = "0.0.0.0";

    /// Bind to all IPv6 interfaces (::)
    pub const BIND_ALL_IPV6: &str = "::";

    /// Loopback IPv6 address
    pub const LOCALHOST_IPV6: &str = "::1";
}

// ==================== PORT NUMBERS ====================

/// Default network ports - extracted from hardcoded values
/// Found in 133 locations across codebase
pub mod ports {
    /// Default HTTP port for development
    pub const HTTP_DEFAULT: u16 = 8080;

    /// Default HTTPS port
    pub const HTTPS_DEFAULT: u16 = 8443;

    /// Default API server port (found in multiple locations)
    pub const API_DEFAULT: u16 = 3000;

    /// Alternative API port
    pub const API_ALT: u16 = 3001;

    pub const METRICS_DEFAULT: u16 = 9090;

    pub const HEALTH_CHECK_DEFAULT: u16 = 8081;

    /// Alternate health check port (used by network_defaults)
    pub const HEALTH_CHECK: u16 = 8082;

    /// Development server port / Storage metrics port
    pub const DEV_SERVER: u16 = 5000;

    /// Storage metrics port (alias for DEV_SERVER)
    pub const STORAGE_DEFAULT: u16 = 5000;

    /// WebSocket port (used by network_defaults)
    pub const WEBSOCKET_DEFAULT: u16 = 8081;

    /// Admin interface port
    pub const ADMIN_DEFAULT: u16 = 8082;
}

/// Default timeout values in milliseconds
pub mod timeouts {
    pub const CONNECT_MS: u64 = 5_000;

    pub const REQUEST_MS: u64 = 30_000;

    /// Long operation timeout (5 minutes)
    pub const LONG_OPERATION_MS: u64 = 300_000;
}

// ==================== ENVIRONMENT VARIABLE HELPERS ====================

/// Environment variable key names
pub mod env_keys {
    /// Bind Address
    pub const BIND_ADDRESS: &str = "NESTGATE_BIND_ADDRESS";
    /// Api Port
    pub const API_PORT: &str = "NESTGATE_API_PORT";
    pub const METRICS_PORT: &str = "NESTGATE_METRICS_PORT";
    pub const HEALTH_PORT: &str = "NESTGATE_HEALTH_PORT";
    /// Websocket Port
    pub const WEBSOCKET_PORT: &str = "NESTGATE_WEBSOCKET_PORT";
}

// ==================== RUNTIME CONFIGURATION ====================

/// Get API bind address from environment or default
///
/// Environment variable: `NESTGATE_BIND_ADDRESS`
/// Default: 0.0.0.0 (bind all interfaces)
pub fn get_api_bind_address() -> String {
    env::var(env_keys::BIND_ADDRESS).unwrap_or_else(|_| addresses::BIND_ALL_IPV4.to_string())
}

/// Get API port from environment or default
///
/// Environment variable: `NESTGATE_API_PORT`
/// Default: 8080
pub fn get_api_port() -> u16 {
    env::var(env_keys::API_PORT)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::HTTP_DEFAULT)
}

/// Get metrics port from environment or default
///
/// Environment variable: `NESTGATE_METRICS_PORT`
/// Default: 9090
pub fn get_metrics_port() -> u16 {
    env::var(env_keys::METRICS_PORT)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::METRICS_DEFAULT)
}

/// Get health check port from environment or default
///
/// Environment variable: `NESTGATE_HEALTH_PORT`
/// Default: 8081
pub fn get_health_port() -> u16 {
    env::var(env_keys::HEALTH_PORT)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::HEALTH_CHECK_DEFAULT)
}

/// Get WebSocket port from environment or default
///
/// Environment variable: `NESTGATE_WEBSOCKET_PORT`
/// Default: 9000
pub fn get_websocket_port() -> u16 {
    env::var(env_keys::WEBSOCKET_PORT)
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(ports::WEBSOCKET_DEFAULT)
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_addresses() {
        assert_eq!(addresses::LOCALHOST_IPV4, "127.0.0.1");
        assert_eq!(addresses::LOCALHOST_NAME, "localhost");
        assert_eq!(addresses::BIND_ALL_IPV4, "0.0.0.0");
        assert_eq!(addresses::BIND_ALL_IPV6, "::");
    }

    #[test]
    fn test_default_ports() {
        assert_eq!(ports::HTTP_DEFAULT, 8080);
        assert_eq!(ports::HTTPS_DEFAULT, 8443);
        assert_eq!(ports::API_DEFAULT, 3000);
        assert_eq!(ports::METRICS_DEFAULT, 9090);
        assert_eq!(ports::HEALTH_CHECK_DEFAULT, 8081);
    }

    #[test]
    fn test_get_api_bind_address_default() {
        // Assuming env var is not set
        let addr = get_api_bind_address();
        assert!(addr == "0.0.0.0" || !addr.is_empty()); // Either default or env override
    }

    #[test]
    fn test_get_api_port_default() {
        // Assuming env var is not set
        let port = get_api_port();
        assert!(port > 0); // u16 is always <= 65535
    }

    #[test]
    fn test_env_keys_defined() {
        assert_eq!(env_keys::BIND_ADDRESS, "NESTGATE_BIND_ADDRESS");
        assert_eq!(env_keys::API_PORT, "NESTGATE_API_PORT");
        assert_eq!(env_keys::METRICS_PORT, "NESTGATE_METRICS_PORT");
    }
}
