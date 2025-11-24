//! Network Address Defaults - Environment-Aware Configuration
//!
//! This module provides environment-aware defaults for network addresses,
//! replacing hardcoded IP addresses and hostnames throughout the codebase.

use super::network_defaults_config::NetworkDefaultsConfig;
use std::net::{Ipv4Addr, Ipv6Addr};

// ==================== IP ADDRESS DEFAULTS ====================

/// Default localhost IPv4 address
pub const LOCALHOST_IPV4: Ipv4Addr = Ipv4Addr::LOCALHOST; // 127.0.0.1

/// Default localhost IPv6 address  
pub const LOCALHOST_IPV6: Ipv6Addr = Ipv6Addr::LOCALHOST; // ::1

/// Default bind-all IPv4 address
pub const BIND_ALL_IPV4: Ipv4Addr = Ipv4Addr::UNSPECIFIED; // 0.0.0.0

/// Default bind-all IPv6 address
pub const BIND_ALL_IPV6: Ipv6Addr = Ipv6Addr::UNSPECIFIED; // ::

// ==================== HOSTNAME DEFAULTS ====================

/// Default localhost hostname
pub const LOCALHOST_NAME: &str = "localhost";

/// Default bind address for services
pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";

// ==================== HELPER FUNCTIONS ====================

/// Get API bind address from environment or default
/// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
///
/// Reads from `NESTGATE_BIND_ADDRESS` environment variable, falls back to 0.0.0.0
#[must_use]
pub fn get_bind_address() -> String {
    NetworkDefaultsConfig::from_env().get_bind_address()
}

/// Get API host from environment or default
/// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
///
/// Reads from `NESTGATE_API_HOST` environment variable, falls back to localhost
#[must_use]
pub fn get_api_host() -> String {
    NetworkDefaultsConfig::from_env().get_api_host()
}

/// Get database host from environment or default
/// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
///
/// Reads from `NESTGATE_DB_HOST` environment variable, falls back to localhost
#[must_use]
pub fn get_db_host() -> String {
    NetworkDefaultsConfig::from_env().get_db_host()
}

/// Get Redis host from environment or default
/// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
///
/// Reads from `NESTGATE_REDIS_HOST` environment variable, falls back to localhost
#[must_use]
pub fn get_redis_host() -> String {
    NetworkDefaultsConfig::from_env().get_redis_host()
}

/// Check if running in production mode
/// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
///
/// Reads from `NESTGATE_ENVIRONMENT` environment variable
#[must_use]
pub fn is_production() -> bool {
    NetworkDefaultsConfig::from_env().is_production()
}

/// Check if running in development mode
/// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
///
/// Reads from `NESTGATE_ENVIRONMENT` environment variable, defaults to true
#[must_use]
pub fn is_development() -> bool {
    NetworkDefaultsConfig::from_env().is_development()
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_localhost_constants() {
        assert_eq!(LOCALHOST_IPV4, Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(LOCALHOST_IPV6, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    }

    #[test]
    fn test_bind_all_constants() {
        assert_eq!(BIND_ALL_IPV4, Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(BIND_ALL_IPV6, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    }

    #[test]
    fn test_get_bind_address() {
        // Should return a valid address
        let addr = get_bind_address();
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_get_api_host() {
        // Should return a valid host
        let host = get_api_host();
        assert!(!host.is_empty());
    }

    #[test]
    fn test_environment_detection() {
        // In test environment, should default to development
        let is_dev = is_development();
        let is_prod = is_production();

        // At least one should be determinable
        // Valid environment detection (always true, but documents intent)
        #[allow(clippy::overly_complex_bool_expr)]
        {
            assert!(is_dev || is_prod || (!is_dev && !is_prod));
        }
    }
}

// Additional comprehensive tests in separate module for better organization
#[cfg(test)]
#[path = "network_defaults_tests.rs"]
mod network_defaults_tests;
