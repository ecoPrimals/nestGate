//! Comprehensive tests for network_defaults module
//!
//! Tests verify environment variable handling, constants, and edge cases

#[cfg(test)]
mod tests {
    use crate::constants::network_defaults::*;
    use std::env;
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
    #[serial_test::serial]
    fn test_get_bind_address_default() {
        env::remove_var("NESTGATE_BIND_ADDRESS");
        let addr = get_bind_address();
        assert_eq!(addr, "0.0.0.0");
        assert!(!addr.is_empty());
    }

    #[test]
    #[serial_test::serial]
    fn test_get_bind_address_with_env() {
        env::set_var("NESTGATE_BIND_ADDRESS", "127.0.0.1");
        let addr = get_bind_address();
        assert_eq!(addr, "127.0.0.1");
        env::remove_var("NESTGATE_BIND_ADDRESS");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_bind_address_with_custom_ip() {
        env::set_var("NESTGATE_BIND_ADDRESS", "192.168.1.100");
        let addr = get_bind_address();
        assert_eq!(addr, "192.168.1.100");
        env::remove_var("NESTGATE_BIND_ADDRESS");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_api_host_default() {
        env::remove_var("NESTGATE_API_HOST");
        let host = get_api_host();
        assert_eq!(host, "localhost");
        assert!(!host.is_empty());
    }

    #[test]
    #[serial_test::serial]
    fn test_get_api_host_with_env() {
        env::set_var("NESTGATE_API_HOST", "api.example.com");
        let host = get_api_host();
        assert_eq!(host, "api.example.com");
        env::remove_var("NESTGATE_API_HOST");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_api_host_with_ip() {
        env::set_var("NESTGATE_API_HOST", "10.0.0.5");
        let host = get_api_host();
        assert_eq!(host, "10.0.0.5");
        env::remove_var("NESTGATE_API_HOST");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_db_host_default() {
        env::remove_var("NESTGATE_DB_HOST");
        let host = get_db_host();
        assert_eq!(host, "localhost");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_db_host_with_env() {
        env::set_var("NESTGATE_DB_HOST", "db.example.com");
        let host = get_db_host();
        assert_eq!(host, "db.example.com");
        env::remove_var("NESTGATE_DB_HOST");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_db_host_with_ip() {
        env::set_var("NESTGATE_DB_HOST", "172.16.0.10");
        let host = get_db_host();
        assert_eq!(host, "172.16.0.10");
        env::remove_var("NESTGATE_DB_HOST");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_redis_host_default() {
        env::remove_var("NESTGATE_REDIS_HOST");
        let host = get_redis_host();
        assert_eq!(host, "localhost");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_redis_host_with_env() {
        env::set_var("NESTGATE_REDIS_HOST", "redis.example.com");
        let host = get_redis_host();
        assert_eq!(host, "redis.example.com");
        env::remove_var("NESTGATE_REDIS_HOST");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_redis_host_with_ip() {
        env::set_var("NESTGATE_REDIS_HOST", "10.1.2.3");
        let host = get_redis_host();
        assert_eq!(host, "10.1.2.3");
        env::remove_var("NESTGATE_REDIS_HOST");
    }

    #[test]
    #[serial_test::serial]
    fn test_is_production_true() {
        env::set_var("NESTGATE_ENVIRONMENT", "production");
        assert!(is_production());
        env::remove_var("NESTGATE_ENVIRONMENT");
    }

    #[test]
    #[serial_test::serial]
    fn test_is_production_true_uppercase() {
        env::set_var("NESTGATE_ENVIRONMENT", "PRODUCTION");
        assert!(is_production());
        env::remove_var("NESTGATE_ENVIRONMENT");
    }

    #[test]
    #[serial_test::serial]
    fn test_is_production_true_short() {
        env::set_var("NESTGATE_ENVIRONMENT", "prod");
        assert!(is_production());
        env::remove_var("NESTGATE_ENVIRONMENT");
    }

    #[test]
    #[serial_test::serial]
    fn test_is_production_false_development() {
        env::set_var("NESTGATE_ENVIRONMENT", "development");
        assert!(!is_production());
        env::remove_var("NESTGATE_ENVIRONMENT");
    }

    #[test]
    #[serial_test::serial]
    fn test_is_production_false_default() {
        env::remove_var("NESTGATE_ENVIRONMENT");
        assert!(!is_production()); // Default is not production
    }

    #[test]
    #[serial_test::serial]
    fn test_is_development_true() {
        env::set_var("NESTGATE_ENVIRONMENT", "development");
        assert!(is_development());
        env::remove_var("NESTGATE_ENVIRONMENT");
    }

    #[test]
    #[serial_test::serial]
    fn test_is_development_true_uppercase() {
        env::set_var("NESTGATE_ENVIRONMENT", "DEVELOPMENT");
        assert!(is_development());
        env::remove_var("NESTGATE_ENVIRONMENT");
    }

    #[test]
    #[serial_test::serial]
    fn test_is_development_true_short() {
        env::set_var("NESTGATE_ENVIRONMENT", "dev");
        assert!(is_development());
        env::remove_var("NESTGATE_ENVIRONMENT");
    }

    #[test]
    #[serial_test::serial]
    fn test_is_development_true_default() {
        env::remove_var("NESTGATE_ENVIRONMENT");
        assert!(is_development()); // Default is development for safety
    }

    #[test]
    #[serial_test::serial]
    fn test_is_development_false_production() {
        env::set_var("NESTGATE_ENVIRONMENT", "production");
        assert!(!is_development());
        env::remove_var("NESTGATE_ENVIRONMENT");
    }

    #[test]
    #[serial_test::serial]
    fn test_production_and_development_mutually_exclusive() {
        env::set_var("NESTGATE_ENVIRONMENT", "production");
        assert!(is_production());
        assert!(!is_development());
        env::remove_var("NESTGATE_ENVIRONMENT");

        env::set_var("NESTGATE_ENVIRONMENT", "development");
        assert!(!is_production());
        assert!(is_development());
        env::remove_var("NESTGATE_ENVIRONMENT");
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
    #[serial_test::serial]
    fn test_all_host_getters_return_valid_strings() {
        env::remove_var("NESTGATE_API_HOST");
        env::remove_var("NESTGATE_DB_HOST");
        env::remove_var("NESTGATE_REDIS_HOST");

        assert!(!get_api_host().is_empty());
        assert!(!get_db_host().is_empty());
        assert!(!get_redis_host().is_empty());
        assert!(!get_bind_address().is_empty());
    }

    #[test]
    #[serial_test::serial]
    fn test_environment_edge_cases() {
        // Test with unusual but valid environment values
        env::set_var("NESTGATE_ENVIRONMENT", "staging");
        assert!(!is_production()); // Not production
        assert!(!is_development()); // Not development
        env::remove_var("NESTGATE_ENVIRONMENT");

        env::set_var("NESTGATE_ENVIRONMENT", "test");
        assert!(!is_production());
        assert!(!is_development());
        env::remove_var("NESTGATE_ENVIRONMENT");

        // Empty string is neither production nor development
        env::set_var("NESTGATE_ENVIRONMENT", "");
        assert!(!is_production());
        assert!(!is_development()); // Empty string doesn't match "dev" or "development"
        env::remove_var("NESTGATE_ENVIRONMENT");
    }
}
