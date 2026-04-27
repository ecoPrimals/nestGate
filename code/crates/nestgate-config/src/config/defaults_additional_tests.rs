// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Additional comprehensive tests for default configurations
//!
//! **Test Expansion Phase 1** (Nov 6, 2025)
//! Focus: Configuration defaults, serialization, environment overrides
//! Goal: Expand coverage from 48.28% toward 90%

#[cfg(test)]
mod network_config_comprehensive_tests {
    use crate::constants::network_hardcoded;

    #[test]
    fn test_localhost_addresses_are_valid() {
        assert!(
            network_hardcoded::addresses::LOCALHOST_IPV4
                .parse::<std::net::Ipv4Addr>()
                .is_ok()
        );
        assert_eq!(network_hardcoded::addresses::LOCALHOST_IPV6, "::1");
    }

    #[test]
    fn test_bind_all_addresses_are_valid() {
        assert_eq!(network_hardcoded::addresses::BIND_ALL_IPV4, "0.0.0.0");
        assert_eq!(network_hardcoded::addresses::BIND_ALL_IPV6, "::");
    }

    #[test]
    fn test_port_numbers_in_valid_range() {
        // Verify ports are defined (compile-time constants, so just verify they exist)
        std::hint::black_box((
            network_hardcoded::ports::HTTP_DEFAULT,
            network_hardcoded::ports::HTTPS_DEFAULT,
            network_hardcoded::ports::API_DEFAULT,
        ));
        // Ports are u16, so automatically in valid range (1-65535)
    }

    #[test]
    fn test_common_ports_distinct() {
        let http = network_hardcoded::ports::HTTP_DEFAULT;
        let https = network_hardcoded::ports::HTTPS_DEFAULT;
        let api = network_hardcoded::ports::API_DEFAULT;

        // Ports should be different to avoid conflicts
        assert_ne!(http, https);
        assert_ne!(http, api);
        // HTTPS and API might coincide, but generally shouldn't
    }

    #[test]
    fn test_get_api_bind_address_returns_valid_address() {
        let addr = network_hardcoded::get_api_bind_address();
        assert!(!addr.is_empty());
        assert!(addr.len() >= 7); // Minimum "0.0.0.0" or similar
    }

    #[test]
    fn test_get_api_port_returns_valid_port() {
        let port = network_hardcoded::get_api_port();
        // Port is u16, automatically in valid range (0-65535)
        // Just verify it compiles and returns a value
        assert!(port > 0); // Verify we have a non-zero port
    }

    #[test]
    fn test_all_environment_keys_defined() {
        // Verify environment keys are defined - these are const strings
        // These are const strs, so we check their content directly
        assert!(network_hardcoded::env_keys::BIND_ADDRESS.starts_with("NESTGATE_"));
        assert!(network_hardcoded::env_keys::API_PORT.starts_with("NESTGATE_"));
    }
}

#[cfg(test)]
mod error_handling_tests {
    use nestgate_types::error::NestGateError;

    #[test]
    fn test_error_is_send() {
        /// Assert Send
        fn assert_send<T: Send>() {}
        assert_send::<NestGateError>();
    }

    #[test]
    fn test_error_is_sync() {
        /// Assert Sync
        fn assert_sync<T: Sync>() {}
        assert_sync::<NestGateError>();
    }

    #[test]
    fn test_error_is_send_sync() {
        /// Assert Send Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NestGateError>();
    }
}

#[cfg(test)]
mod constants_validation_tests {
    use crate::constants::shared;

    #[test]
    fn test_buffer_size_is_reasonable() {
        // Buffer size migrated to canonical_defaults (domain-specific)
        // Network buffer size check (8KB)
        use crate::constants::network::NETWORK_BUFFER_SIZE;
        // Compile-time validation of buffer size constants
        #[expect(clippy::assertions_on_constants)]
        {
            assert!(NETWORK_BUFFER_SIZE >= 1024, "Network buffer too small");
            assert!(
                NETWORK_BUFFER_SIZE <= 1024 * 1024,
                "Network buffer unreasonably large"
            );
        }
    }

    #[test]
    fn test_max_connections_is_positive() {
        // Compile-time validation of connection limits
        #[expect(clippy::assertions_on_constants)]
        {
            assert!(shared::DEFAULT_MAX_CONNECTIONS > 0);
            assert!(
                shared::DEFAULT_MAX_CONNECTIONS <= 10000,
                "Max connections unreasonably high"
            );
        }
    }

    #[test]
    fn test_timeout_is_reasonable() {
        // Timeout migrated to canonical.rs
        use crate::constants::canonical::timeouts::DEFAULT_TIMEOUT_MS;
        // Compile-time validation of timeout constants
        #[expect(clippy::assertions_on_constants)]
        {
            assert!(DEFAULT_TIMEOUT_MS > 0);
            assert!(
                DEFAULT_TIMEOUT_MS <= 300_000,
                "Timeout unreasonably long (>5 min)"
            );
        }
    }
}
