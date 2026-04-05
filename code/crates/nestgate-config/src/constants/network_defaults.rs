// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Network address constants (IP and hostname defaults).
//!
//! For runtime configuration use `EnvironmentConfig::from_env()` directly.

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
}
