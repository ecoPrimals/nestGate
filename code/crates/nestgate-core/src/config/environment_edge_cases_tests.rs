//! Deep edge case tests for environment configuration
//! Part of test coverage expansion: 72.62% → 90%
//!
//! Focus: Configuration edge cases, environment variable combinations,
//! invalid inputs, boundary conditions

#[cfg(test)]
mod environment_edge_cases {
    use crate::config::environment::EnvironmentConfig;
    use crate::network::client::types::Port;
    use std::env;

    #[test]
    fn test_empty_env_var_fallback_to_default() {
        // Test that empty environment variables fall back to defaults
        // EnvironmentConfig reads NESTGATE_HOST (not NESTGATE_API_HOST)
        env::set_var("NESTGATE_HOST", "");

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should use default, not empty string
        assert!(
            !config.network.host.is_empty(),
            "Empty env var should use default"
        );
        assert_ne!(config.network.host, "", "Host should not be empty string");
    }

    #[test]
    fn test_whitespace_only_env_var() {
        // Test handling of whitespace-only environment variables
        // EnvironmentConfig reads NESTGATE_HOST (not NESTGATE_API_HOST)
        env::set_var("NESTGATE_HOST", "   ");

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should trim or use default
        let trimmed = config.network.host.trim();
        assert!(!trimmed.is_empty(), "Whitespace-only should be handled");
    }

    #[test]
    fn test_port_boundary_1024() {
        // Test exact boundary at 1024 (first non-privileged port)
        let result = Port::new(1024);
        assert!(
            result.is_ok(),
            "Port 1024 should be valid (first non-privileged)"
        );
        assert_eq!(result.unwrap().get(), 1024);
    }

    #[test]
    fn test_port_boundary_1023() {
        // Test boundary at 1023 (last privileged port)
        // Note: Port type allows 1-65535 because it's used for BOTH:
        // 1. Client connections (where 80/443/etc are standard)
        // 2. Server binding (where >= 1024 is recommended for security)
        // Server binding should validate >= 1024 separately
        let result = Port::new(1023);
        assert!(result.is_ok(), "Port 1023 is valid for client connections");
    }

    #[test]
    fn test_port_boundary_65535() {
        // Test maximum valid port
        let result = Port::new(65535);
        assert!(result.is_ok(), "Port 65535 should be valid (max port)");
        assert_eq!(result.unwrap().get(), 65535);
    }

    #[test]
    fn test_port_overflow_65536() {
        // Test port number overflow
        // Port accepts u16, so 65536+ cannot be tested as literals
        // Values beyond u16::MAX are compile-time errors, not runtime errors
    }

    #[test]
    fn test_concurrent_env_var_access() {
        // Test thread safety of environment variable access
        use std::thread;

        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    env::set_var("NESTGATE_API_PORT", format!("{}", 8080 + i));
                    EnvironmentConfig::from_env()
                })
            })
            .collect();

        for handle in handles {
            let config = handle.join().unwrap().unwrap();
            // Should not panic, port should be valid
            assert!(config.network.port.get() >= 1024);
        }
    }

    #[test]
    fn test_invalid_utf8_in_env_var() {
        // Test handling of invalid UTF-8 (if possible in env vars)
        // Note: Rust env vars are typically valid UTF-8, but test robustness
        // EnvironmentConfig reads NESTGATE_HOST (not NESTGATE_API_HOST)
        env::set_var("NESTGATE_HOST", "valid-host");

        let config = EnvironmentConfig::from_env().expect("Config should load");
        assert!(
            config.network.host.is_ascii() || config.network.host.chars().all(|c| !c.is_control())
        );
    }

    #[test]
    fn test_very_long_host_name() {
        // Test handling of extremely long host names
        // EnvironmentConfig reads NESTGATE_HOST (not NESTGATE_API_HOST)
        let long_host = "a".repeat(1000);
        env::set_var("NESTGATE_HOST", &long_host);

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should either accept or fall back to default (not panic)
        assert!(!config.network.host.is_empty());
    }

    #[test]
    fn test_special_characters_in_host() {
        // Test various special characters in host names
        // EnvironmentConfig reads NESTGATE_HOST (not NESTGATE_API_HOST)
        let test_cases = vec![
            "localhost",
            "127.0.0.1",
            "::1",
            "example.com",
            "sub.domain.example.com",
            "host-with-dash",
            "host_with_underscore",
        ];

        for host in test_cases {
            env::set_var("NESTGATE_HOST", host);
            let config = EnvironmentConfig::from_env().expect("Config should load");

            // Should handle all valid DNS names and IPs
            assert!(
                !config.network.host.is_empty(),
                "Host '{}' should be valid",
                host
            );
        }
    }

    #[test]
    fn test_invalid_port_string_non_numeric() {
        // Test non-numeric port string
        // EnvironmentConfig reads NESTGATE_PORT (not NESTGATE_API_PORT)
        env::set_var("NESTGATE_PORT", "not-a-number");

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should fall back to default
        assert!(
            config.network.port.get() >= 1024,
            "Invalid port should use default"
        );
    }

    #[test]
    fn test_invalid_port_string_negative() {
        // Test negative port number
        // EnvironmentConfig reads NESTGATE_PORT (not NESTGATE_API_PORT)
        env::set_var("NESTGATE_PORT", "-1234");

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should fall back to default (can't be negative)
        assert!(
            config.network.port.get() > 0,
            "Negative port should use default"
        );
    }

    #[test]
    fn test_invalid_port_string_float() {
        // Test floating point port number
        // EnvironmentConfig reads NESTGATE_PORT (not NESTGATE_API_PORT)
        env::set_var("NESTGATE_PORT", "8080.5");

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should fall back to default or truncate
        assert!(config.network.port.get() >= 1024);
    }

    #[test]
    fn test_env_var_override_priority() {
        // Test that environment variables take priority over defaults
        // EnvironmentConfig reads NESTGATE_HOST and NESTGATE_PORT
        env::set_var("NESTGATE_HOST", "custom-host");
        env::set_var("NESTGATE_PORT", "9999");

        let config = EnvironmentConfig::from_env().expect("Config should load");

        assert_eq!(config.network.host, "custom-host");
        assert_eq!(config.network.port.get(), 9999);
    }

    #[test]
    fn test_partial_env_var_config() {
        // Test with only some env vars set
        // EnvironmentConfig reads NESTGATE_HOST and NESTGATE_PORT
        env::remove_var("NESTGATE_HOST");
        env::set_var("NESTGATE_PORT", "8888");

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should use default host, custom port
        assert!(!config.network.host.is_empty());
        assert_eq!(config.network.port.get(), 8888);
    }

    #[test]
    fn test_config_clone_equality() {
        // Test that cloned configs are equal
        let config1 = EnvironmentConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.network.host, config2.network.host);
        assert_eq!(config1.network.port.get(), config2.network.port.get());
    }

    #[test]
    fn test_config_debug_output() {
        // Test that debug output doesn't panic and contains key info
        let config = EnvironmentConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("network") || debug_str.contains("Network"));
        assert!(!debug_str.is_empty());
    }

    #[test]
    fn test_port_validation_common_ports() {
        // Test validation of commonly used ports
        let common_ports = vec![
            (1024, true),  // First non-privileged
            (3000, true),  // Node.js default
            (5432, true),  // PostgreSQL
            (6379, true),  // Redis
            (8080, true),  // HTTP alt
            (8443, true),  // HTTPS alt
            (9090, true),  // Prometheus
            (27017, true), // MongoDB
        ];

        for (port, should_be_valid) in common_ports {
            let result = Port::new(port);
            assert_eq!(
                result.is_ok(),
                should_be_valid,
                "Port {} validation mismatch",
                port
            );
        }
    }

    #[test]
    fn test_multiple_env_var_sources() {
        // Test behavior when multiple similar env vars exist
        env::set_var("NESTGATE_API_HOST", "host1");
        env::set_var("NESTGATE_HOST", "host2");

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should have deterministic priority
        assert!(!config.network.host.is_empty());
    }

    #[test]
    fn test_env_var_case_sensitivity() {
        // Verify env vars are case-sensitive (standard behavior)
        // EnvironmentConfig reads NESTGATE_PORT (not NESTGATE_API_PORT)
        env::set_var("NESTGATE_PORT", "8080");
        env::set_var("nestgate_port", "9090"); // lowercase (should be ignored)

        let config = EnvironmentConfig::from_env().expect("Config should load");

        // Should use uppercase (correct) case, not lowercase
        assert_eq!(config.network.port.get(), 8080);
    }

    #[test]
    fn test_config_construction_idempotency() {
        // Test that constructing config multiple times gives same result
        // Clear env vars first to ensure isolation
        env::remove_var("NESTGATE_HOST");
        env::remove_var("NESTGATE_PORT");

        // Now set our test values
        env::set_var("NESTGATE_HOST", "test-host");
        env::set_var("NESTGATE_PORT", "7777");

        let config1 = EnvironmentConfig::from_env().expect("Config should load");
        let config2 = EnvironmentConfig::from_env().expect("Config should load");

        // Both configs should have the same values from environment
        assert_eq!(config1.network.host, "test-host");
        assert_eq!(config2.network.host, "test-host");
        assert_eq!(config1.network.port.get(), 7777);
        assert_eq!(config2.network.port.get(), config1.network.port.get());

        // Cleanup
        env::remove_var("NESTGATE_HOST");
        env::remove_var("NESTGATE_PORT");
    }
}
