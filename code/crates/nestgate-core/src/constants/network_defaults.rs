//! Network Default Values - Centralized Configuration
//!
//! This module provides centralized default values for network configuration.
//! All hardcoded network values should be replaced with these constants
//! and made configurable via environment variables or configuration files.

// ==================== IPv4 DEFAULTS ====================

/// Default localhost IPv4 address
///
/// **Environment Variable**: `NESTGATE_DEFAULT_IPV4`  
/// **Usage**: Development, testing, default fallback
pub const DEFAULT_LOCALHOST_IPV4: &str = "127.0.0.1";

/// Default bind address for all interfaces
///
/// **Environment Variable**: `NESTGATE_BIND_ADDRESS`  
/// **Usage**: Server binding, wildcard listening
pub const DEFAULT_BIND_ALL_IPV4: &str = "0.0.0.0";

/// Default private network range start (10.0.0.0/8)
///
/// **Usage**: Private network detection, validation
pub const PRIVATE_NETWORK_10_START: &str = "10.0.0.0";

/// Default private network range (172.16.0.0/12)
///
/// **Usage**: Private network detection, validation
pub const PRIVATE_NETWORK_172_START: &str = "172.16.0.0";

/// Default private network range (192.168.0.0/16)
///
/// **Usage**: Private network detection, validation  
pub const PRIVATE_NETWORK_192_START: &str = "192.168.0.0";

// ==================== IPv6 DEFAULTS ====================

/// Default localhost IPv6 address
///
/// **Environment Variable**: `NESTGATE_DEFAULT_IPV6`  
/// **Usage**: IPv6 localhost, development
pub const DEFAULT_LOCALHOST_IPV6: &str = "::1";

/// Default bind address for all IPv6 interfaces
///
/// **Environment Variable**: `NESTGATE_BIND_ADDRESS_IPV6`  
/// **Usage**: IPv6 server binding
pub const DEFAULT_BIND_ALL_IPV6: &str = "::";

// ==================== HOSTNAME DEFAULTS ====================

/// Default hostname - localhost
///
/// **Environment Variable**: `NESTGATE_DEFAULT_HOSTNAME`  
/// **Usage**: Development, testing, default fallback
pub const DEFAULT_HOSTNAME: &str = "localhost";

// ==================== HELPER FUNCTIONS ====================

/// Get default localhost address (IPv4 by default)
///
/// Reads from environment variable `NESTGATE_DEFAULT_IP` or falls back to IPv4 localhost
#[must_use]
pub fn get_default_localhost() -> &'static str {
    std::env::var("NESTGATE_DEFAULT_IP")
        .ok()
        .map(|s| {
            if s == "ipv6" {
                DEFAULT_LOCALHOST_IPV6
            } else {
                DEFAULT_LOCALHOST_IPV4
            }
        })
        .unwrap_or(DEFAULT_LOCALHOST_IPV4)
}

/// Get default bind address (IPv4 by default)
///
/// Reads from environment variable `NESTGATE_BIND_ADDRESS` or falls back to 0.0.0.0
#[must_use]
pub fn get_default_bind_address() -> &'static str {
    std::env::var("NESTGATE_BIND_ADDRESS")
        .ok()
        .map(|s| {
            if s == "ipv6" {
                DEFAULT_BIND_ALL_IPV6
            } else {
                DEFAULT_BIND_ALL_IPV4
            }
        })
        .unwrap_or(DEFAULT_BIND_ALL_IPV4)
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_defined() {
        assert_eq!(DEFAULT_LOCALHOST_IPV4, "127.0.0.1");
        assert_eq!(DEFAULT_LOCALHOST_IPV6, "::1");
        assert_eq!(DEFAULT_BIND_ALL_IPV4, "0.0.0.0");
        assert_eq!(DEFAULT_HOSTNAME, "localhost");
    }

    #[test]
    fn test_get_default_localhost() {
        // Should return IPv4 by default
        let localhost = get_default_localhost();
        assert!(
            localhost == DEFAULT_LOCALHOST_IPV4 || localhost == DEFAULT_LOCALHOST_IPV6,
            "Localhost should be either IPv4 or IPv6"
        );
    }

    #[test]
    fn test_get_default_bind_address() {
        // Should return IPv4 by default
        let bind = get_default_bind_address();
        assert!(
            bind == DEFAULT_BIND_ALL_IPV4 || bind == DEFAULT_BIND_ALL_IPV6,
            "Bind address should be either IPv4 or IPv6"
        );
    }
}
