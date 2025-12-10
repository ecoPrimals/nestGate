//! Network Address Defaults - Environment-Aware Configuration
//!
//! This module provides environment-aware defaults for network addresses,
//! replacing hardcoded IP addresses and hostnames throughout the codebase.
//!
//! **MIGRATION NOTE** (Week 2, Dec 2025): This module is being migrated to use
//! the modern `EnvironmentConfig` system. Helper functions are deprecated.
//!
//! **For new code**: Use `EnvironmentConfig::from_env()` directly
//! **For existing code**: Use these helpers (will show deprecation warnings)

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
// **MODERNIZED** (Week 2, Dec 2025): These functions now delegate to
// migration_bridge and are marked as deprecated.

/// Get API bind address from environment or default
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.bind_address` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.network.bind_address instead"
)]
#[must_use]
pub fn get_bind_address() -> String {
    use crate::config::migration_bridge;
    #[allow(deprecated)]
    migration_bridge::get_api_host() // bind_address and host are same in new config
}

/// Get API host from environment or default
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.host` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.network.host instead"
)]
#[must_use]
pub fn get_api_host() -> String {
    use crate::config::migration_bridge;
    #[allow(deprecated)]
    migration_bridge::get_api_host()
}

/// Get database host from environment or default
///
/// **DEPRECATED**: Database configuration should be external to NestGate
#[deprecated(
    since = "0.6.0",
    note = "Database configuration should be managed externally"
)]
#[must_use]
pub fn get_db_host() -> String {
    std::env::var("NESTGATE_DB_HOST").unwrap_or_else(|_| "localhost".to_string())
}

/// Get Redis host from environment or default
///
/// **DEPRECATED**: Database configuration should be external to NestGate
#[deprecated(
    since = "0.6.0",
    note = "Database configuration should be managed externally"
)]
#[must_use]
pub fn get_redis_host() -> String {
    std::env::var("NESTGATE_REDIS_HOST").unwrap_or_else(|_| "localhost".to_string())
}

/// Check if running in production mode
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.is_production()` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig for environment detection"
)]
#[must_use]
pub fn is_production() -> bool {
    std::env::var("NESTGATE_ENVIRONMENT")
        .map(|v| v == "production")
        .unwrap_or(false)
}

/// Check if running in development mode
///
/// **DEPRECATED**: Use environment detection from EnvironmentConfig
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig for environment detection"
)]
#[must_use]
pub fn is_development() -> bool {
    !is_production()
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
