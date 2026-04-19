// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Named network address literals (IPv4/IPv6 loopback and “bind all”).
//!
//! These are stable identifiers, not magic numbers scattered through the codebase.
//! Prefer importing these constants when the semantic is “localhost” vs “all interfaces”.

/// IPv4 loopback (`127.0.0.1`). Use for same-host-only listeners unless configured otherwise.
pub const LOCALHOST_IPV4: &str = "127.0.0.1";

/// IPv6 loopback (`::1`). Use for same-host-only listeners on IPv6 stacks.
pub const LOCALHOST_IPV6: &str = "::1";

/// Conventional hostname for the local machine (`localhost`). Not a substitute for IP literals in binds.
pub const LOCALHOST_NAME: &str = "localhost";

/// Bind to all IPv4 interfaces (`0.0.0.0`). Use only when public or multi-interface binding is intentional.
pub const BIND_ALL_IPV4: &str = "0.0.0.0";

/// Bind to all IPv6 interfaces (`::`). Use only when public or multi-interface binding is intentional.
pub const BIND_ALL_IPV6: &str = "::";

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn localhost_ipv4_parses_as_loopback() {
        let ip = IpAddr::from_str(LOCALHOST_IPV4);
        assert!(ip.is_ok_and(|a| a.is_loopback()));
    }

    #[test]
    fn localhost_ipv6_parses_as_loopback() {
        let ip = IpAddr::from_str(LOCALHOST_IPV6);
        assert!(ip.is_ok_and(|a| a.is_loopback()));
    }

    #[test]
    fn bind_all_ipv4_is_unspecified_v4() {
        let ip = IpAddr::from_str(BIND_ALL_IPV4);
        assert!(ip.is_ok_and(|a| match a {
            IpAddr::V4(v4) => v4.is_unspecified(),
            IpAddr::V6(_) => false,
        }));
    }

    #[test]
    fn bind_all_ipv6_is_unspecified_v6() {
        let ip = IpAddr::from_str(BIND_ALL_IPV6);
        assert!(ip.is_ok_and(|a| match a {
            IpAddr::V6(v6) => v6.is_unspecified(),
            IpAddr::V4(_) => false,
        }));
    }

    #[test]
    fn localhost_name_is_non_empty() {
        assert!(!LOCALHOST_NAME.is_empty());
        assert_ne!(LOCALHOST_NAME, BIND_ALL_IPV4);
    }
}
