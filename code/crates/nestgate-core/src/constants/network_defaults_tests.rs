//! Comprehensive tests for network_defaults module
//!
//! **MODERNIZED FOR CONCURRENT TESTING:**
//! - Uses `NetworkDefaultsConfig` for dependency injection
//! - No environment variable pollution
//! - All tests run in parallel safely
//! - Removed all `#[serial_test::serial]` attributes

#[cfg(test)]
mod tests {
    use crate::constants::network_defaults::*;
    use crate::constants::network_defaults_config::NetworkDefaultsConfig;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_localhost_ipv4_constant() {
        assert_eq!(LOCALHOST_IPV4, Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(LOCALHOST_IPV4, Ipv4Addr::LOCALHOST);
    }

    #[test]
    fn test_localhost_ipv6_constant() {
        assert_eq!(LOCALHOST_IPV6, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(LOCALHOST_IPV6, Ipv6Addr::LOCALHOST);
    }

    #[test]
    fn test_bind_all_ipv4_constant() {
        assert_eq!(BIND_ALL_IPV4, Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(BIND_ALL_IPV4, Ipv4Addr::UNSPECIFIED);
    }

    #[test]
    fn test_bind_all_ipv6_constant() {
        assert_eq!(BIND_ALL_IPV6, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
        assert_eq!(BIND_ALL_IPV6, Ipv6Addr::UNSPECIFIED);
    }

    #[test]
    fn test_localhost_name_constant() {
        assert_eq!(LOCALHOST_NAME, "localhost");
        // LOCALHOST_NAME is a const str, always non-empty at compile time
    }

    #[test]
    fn test_default_bind_address_constant() {
        assert_eq!(DEFAULT_BIND_ADDRESS, "0.0.0.0");
        // DEFAULT_BIND_ADDRESS is a const str, always non-empty at compile time
    }

    #[test]
    fn test_get_bind_address_default() {
        // ✅ MODERNIZED: Use config instead of env vars (concurrent-safe!)
        let config = NetworkDefaultsConfig::new();
        let addr = config.get_bind_address();
        assert_eq!(addr, "0.0.0.0");
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_get_bind_address_with_custom() {
        // ✅ MODERNIZED: Inject config (no env pollution!)
        let config = NetworkDefaultsConfig::new().with_bind_address("127.0.0.1".to_string());
        let addr = config.get_bind_address();
        assert_eq!(addr, "127.0.0.1");
    }

    #[test]
    fn test_get_bind_address_with_custom_ip() {
        // ✅ MODERNIZED: Builder pattern (parallel-safe!)
        let config = NetworkDefaultsConfig::new().with_bind_address("192.168.1.100".to_string());
        let addr = config.get_bind_address();
        assert_eq!(addr, "192.168.1.100");
    }

    #[test]
    fn test_get_api_host_default() {
        // ✅ MODERNIZED: Concurrent-safe config pattern
        let config = NetworkDefaultsConfig::new();
        let host = config.get_api_host();
        assert_eq!(host, "localhost");
        assert!(!host.is_empty());
    }

    #[test]
    fn test_get_api_host_with_custom() {
        // ✅ MODERNIZED: No env vars = no race conditions
        let config = NetworkDefaultsConfig::new().with_api_host("api.example.com".to_string());
        let host = config.get_api_host();
        assert_eq!(host, "api.example.com");
    }

    #[test]
    fn test_get_api_host_with_ip() {
        // ✅ MODERNIZED: Builder pattern for all variations
        let config = NetworkDefaultsConfig::new().with_api_host("10.0.0.5".to_string());
        let host = config.get_api_host();
        assert_eq!(host, "10.0.0.5");
    }

    #[test]
    fn test_get_db_host_default() {
        // ✅ MODERNIZED: Config injection pattern
        let config = NetworkDefaultsConfig::new();
        let host = config.get_db_host();
        assert_eq!(host, "localhost");
    }

    #[test]
    fn test_get_db_host_with_custom() {
        // ✅ MODERNIZED: Concurrent-safe
        let config = NetworkDefaultsConfig::new().with_db_host("db.example.com".to_string());
        let host = config.get_db_host();
        assert_eq!(host, "db.example.com");
    }

    #[test]
    fn test_get_db_host_with_ip() {
        // ✅ MODERNIZED: Parallel-safe testing
        let config = NetworkDefaultsConfig::new().with_db_host("172.16.0.10".to_string());
        let host = config.get_db_host();
        assert_eq!(host, "172.16.0.10");
    }

    #[test]
    fn test_get_redis_host_default() {
        // ✅ MODERNIZED: Config-based testing
        let config = NetworkDefaultsConfig::new();
        let host = config.get_redis_host();
        assert_eq!(host, "localhost");
    }

    #[test]
    fn test_get_redis_host_with_custom() {
        // ✅ MODERNIZED: No serial needed
        let config = NetworkDefaultsConfig::new().with_redis_host("redis.example.com".to_string());
        let host = config.get_redis_host();
        assert_eq!(host, "redis.example.com");
    }

    #[test]
    fn test_get_redis_host_with_ip() {
        // ✅ MODERNIZED: Truly concurrent
        let config = NetworkDefaultsConfig::new().with_redis_host("10.1.2.3".to_string());
        let host = config.get_redis_host();
        assert_eq!(host, "10.1.2.3");
    }

    #[test]
    fn test_is_production_true() {
        // ✅ MODERNIZED: Config-based environment detection
        let config = NetworkDefaultsConfig::new().with_environment("production".to_string());
        assert!(config.is_production());
    }

    #[test]
    fn test_is_production_true_uppercase() {
        // ✅ MODERNIZED: Case-insensitive check
        let config = NetworkDefaultsConfig::new().with_environment("PRODUCTION".to_string());
        assert!(config.is_production());
    }

    #[test]
    fn test_is_production_true_short() {
        // ✅ MODERNIZED: Short form support
        let config = NetworkDefaultsConfig::new().with_environment("prod".to_string());
        assert!(config.is_production());
    }

    #[test]
    fn test_is_production_false_development() {
        // ✅ MODERNIZED: Development detection
        let config = NetworkDefaultsConfig::new().with_environment("development".to_string());
        assert!(!config.is_production());
    }

    #[test]
    fn test_is_production_false_default() {
        // ✅ MODERNIZED: Default environment
        let config = NetworkDefaultsConfig::new();
        assert!(!config.is_production()); // Default is not production
    }

    #[test]
    fn test_is_development_true() {
        // ✅ MODERNIZED: Explicit development config
        let config = NetworkDefaultsConfig::new().with_environment("development".to_string());
        assert!(config.is_development());
    }

    #[test]
    fn test_is_development_true_uppercase() {
        // ✅ MODERNIZED: Case-insensitive
        let config = NetworkDefaultsConfig::new().with_environment("DEVELOPMENT".to_string());
        assert!(config.is_development());
    }

    #[test]
    fn test_is_development_true_short() {
        // ✅ MODERNIZED: Short form
        let config = NetworkDefaultsConfig::new().with_environment("dev".to_string());
        assert!(config.is_development());
    }

    #[test]
    fn test_is_development_true_default() {
        // ✅ MODERNIZED: Safe default
        let config = NetworkDefaultsConfig::new();
        assert!(config.is_development()); // Default is development for safety
    }

    #[test]
    fn test_is_development_false_production() {
        // ✅ MODERNIZED: Mutual exclusivity
        let config = NetworkDefaultsConfig::new().with_environment("production".to_string());
        assert!(!config.is_development());
    }

    #[test]
    fn test_production_and_development_mutually_exclusive() {
        // ✅ MODERNIZED: Test both states without env pollution
        let prod_config = NetworkDefaultsConfig::new().with_environment("production".to_string());
        assert!(prod_config.is_production());
        assert!(!prod_config.is_development());

        let dev_config = NetworkDefaultsConfig::new().with_environment("development".to_string());
        assert!(!dev_config.is_production());
        assert!(dev_config.is_development());
    }

    #[test]
    fn test_localhost_constants_are_distinct() {
        // IPv4 and IPv6 localhost should be different types
        assert_ne!(LOCALHOST_IPV4.to_string(), LOCALHOST_IPV6.to_string());
    }

    #[test]
    fn test_bind_all_constants_are_distinct() {
        // IPv4 and IPv6 bind-all should be different types
        assert_ne!(BIND_ALL_IPV4.to_string(), BIND_ALL_IPV6.to_string());
    }

    #[test]
    fn test_ipv4_constants_format() {
        assert_eq!(LOCALHOST_IPV4.to_string(), "127.0.0.1");
        assert_eq!(BIND_ALL_IPV4.to_string(), "0.0.0.0");
    }

    #[test]
    fn test_ipv6_constants_format() {
        assert_eq!(LOCALHOST_IPV6.to_string(), "::1");
        assert_eq!(BIND_ALL_IPV6.to_string(), "::");
    }

    #[test]
    fn test_all_host_getters_return_valid_strings() {
        // ✅ MODERNIZED: Default config validation
        let config = NetworkDefaultsConfig::new();

        assert!(!config.get_api_host().is_empty());
        assert!(!config.get_db_host().is_empty());
        assert!(!config.get_redis_host().is_empty());
        assert!(!config.get_bind_address().is_empty());
    }

    #[test]
    fn test_environment_edge_cases() {
        // ✅ MODERNIZED: Test edge cases concurrently
        // Test with unusual but valid environment values
        let staging_config = NetworkDefaultsConfig::new().with_environment("staging".to_string());
        assert!(!staging_config.is_production()); // Not production
        assert!(!staging_config.is_development()); // Not development

        let test_config = NetworkDefaultsConfig::new().with_environment("test".to_string());
        assert!(!test_config.is_production());
        assert!(!test_config.is_development());

        // Empty string is neither production nor development
        let empty_config = NetworkDefaultsConfig::new().with_environment("".to_string());
        assert!(!empty_config.is_production());
        assert!(!empty_config.is_development()); // Empty string doesn't match "dev" or "development"
    }
}
