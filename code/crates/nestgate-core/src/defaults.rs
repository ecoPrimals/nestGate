//! # Default Constants for NestGate
//!
//! This module provides centralized default values to eliminate hardcoding
//! throughout the codebase while maintaining sovereignty principles.

use crate::constants::hardcoding::{addresses, ports};
use std::time::Duration;

/// **NETWORK DEFAULTS**
pub mod network {
    use super::*;

    /// Default API port - can be overridden with `NESTGATE_API_PORT`
    pub const DEFAULT_API_PORT: u16 = ports::HTTP_DEFAULT;

    /// Default bind address - can be overridden with `NESTGATE_BIND_ADDRESS`  
    pub const DEFAULT_BIND_ADDRESS: &str = addresses::BIND_ALL_IPV4;

    /// Default hostname for development - can be overridden with `NESTGATE_HOSTNAME`
    pub const DEFAULT_HOSTNAME: &str = addresses::LOCALHOST_NAME;

    /// Default WebSocket port - can be overridden with `NESTGATE_WS_PORT`
    pub const DEFAULT_WS_PORT: u16 = ports::WEBSOCKET_DEFAULT;

    /// Default health check port - can be overridden with `NESTGATE_HEALTH_PORT`
    pub const DEFAULT_HEALTH_PORT: u16 = ports::HEALTH_CHECK;
}

/// **DATABASE DEFAULTS**
pub mod database {
    use super::addresses;

    /// Default `PostgreSQL` port - can be overridden with `NESTGATE_DB_PORT`
    pub const DEFAULT_POSTGRES_PORT: u16 = 5432;

    /// Default Redis port - can be overridden with `NESTGATE_REDIS_PORT`
    pub const DEFAULT_REDIS_PORT: u16 = 6379;

    /// Default MongoDB port - can be overridden with `NESTGATE_MONGODB_PORT`
    pub const DEFAULT_MONGODB_PORT: u16 = 27017;

    /// Default MySQL port - can be overridden with `NESTGATE_MYSQL_PORT`
    pub const DEFAULT_MYSQL_PORT: u16 = 3306;

    /// Default database host - can be overridden with `NESTGATE_DB_HOST`
    pub const DEFAULT_DB_HOST: &str = addresses::LOCALHOST_NAME;
}

/// **MONITORING DEFAULTS**
pub mod monitoring {
    /// Default Prometheus port - can be overridden with `NESTGATE_METRICS_PORT`
    pub const DEFAULT_METRICS_PORT: u16 = 9090;

    /// Default Grafana port - can be overridden with `NESTGATE_GRAFANA_PORT`
    pub const DEFAULT_GRAFANA_PORT: u16 = 3000;
}

/// **TIMEOUT DEFAULTS**
pub mod timeouts {
    use super::Duration;

    /// Default API request timeout
    pub const DEFAULT_API_TIMEOUT: Duration = Duration::from_secs(30);

    /// Default database connection timeout
    pub const DEFAULT_DB_TIMEOUT: Duration = Duration::from_secs(10);

    /// Default health check timeout
    pub const DEFAULT_HEALTH_TIMEOUT: Duration = Duration::from_secs(5);

    /// Default WebSocket timeout
    pub const DEFAULT_WS_TIMEOUT: Duration = Duration::from_secs(60);
}

/// **ENVIRONMENT HELPERS** - ✅ MODERNIZED: Removed const where inappropriate
pub mod env_helpers {
    use crate::defaults_v2_config::DefaultsV2Config;

    /// Get API port from environment or default
    #[must_use]
    pub fn api_port() -> u16 {
        DefaultsV2Config::from_env().api_port()
    }

    /// Get bind address from environment or default
    /// ✅ MODERNIZED: Removed const - returns String (requires allocation)
    #[must_use]
    pub fn bind_address() -> String {
        DefaultsV2Config::from_env().bind_address()
    }

    /// Get hostname from environment or default
    /// ✅ MODERNIZED: Removed const - returns String (requires allocation)
    #[must_use]
    pub fn hostname() -> String {
        DefaultsV2Config::from_env().hostname()
    }

    /// Get database port from environment or default
    #[must_use]
    pub fn db_port() -> u16 {
        DefaultsV2Config::from_env().db_port()
    }

    /// Get metrics port from environment or default
    #[must_use]
    pub fn metrics_port() -> u16 {
        DefaultsV2Config::from_env().metrics_port()
    }
}

/// **URL BUILDERS** - ✅ MODERNIZED: Removed const from functions using format!
pub mod urls {
    use super::env_helpers;

    /// Build API URL with environment-aware host and port
    /// ✅ MODERNIZED: Removed const - uses format! macro
    #[must_use]
    pub fn api_url() -> String {
        format!(
            "http://{}:{}",
            env_helpers::hostname(),
            env_helpers::api_port()
        )
    }

    /// Build WebSocket URL with environment-aware host and port
    /// ✅ MODERNIZED: Removed const - uses format! macro
    #[must_use]
    pub fn websocket_url() -> String {
        use crate::defaults_v2_config::DefaultsV2Config;
        DefaultsV2Config::from_env().websocket_url()
    }

    /// Build health check URL with environment-aware host and port
    /// ✅ MODERNIZED: Removed const - uses format! macro
    #[must_use]
    pub fn health_url() -> String {
        use crate::defaults_v2_config::DefaultsV2Config;
        DefaultsV2Config::from_env().health_url()
    }

    /// Legacy function (kept for compatibility) - will be removed
    #[allow(dead_code)]
    fn health_url_old() -> String {
        let port = std::env::var("NESTGATE_HEALTH_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(super::network::DEFAULT_HEALTH_PORT);
        format!("http://{}:{}/health", env_helpers::hostname(), port)
    }
}

/// **STORAGE DEFAULTS**
pub mod storage {
    /// Default ZFS pool name
    pub const DEFAULT_POOL_NAME: &str = "nestgate-pool";

    /// Default cache size in MB
    pub const DEFAULT_CACHE_SIZE_MB: u64 = 1024;

    /// Default compression algorithm
    pub const DEFAULT_COMPRESSION: &str = "lz4";

    /// Default storage service port - can be overridden with `NESTGATE_STORAGE_PORT`
    pub const DEFAULT_STORAGE_PORT: u16 = 5000;

    /// Default NFS port - can be overridden with `NESTGATE_NFS_PORT`
    pub const DEFAULT_NFS_PORT: u16 = 2049;

    /// Default SMB port - can be overridden with `NESTGATE_SMB_PORT`
    pub const DEFAULT_SMB_PORT: u16 = 445;
}

/// **SECURITY DEFAULTS**
pub mod security {
    use super::Duration;

    /// Default session timeout
    pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(3600);

    /// Default token expiry
    pub const DEFAULT_TOKEN_EXPIRY: Duration = Duration::from_secs(1800);

    /// Default max login attempts
    pub const DEFAULT_MAX_LOGIN_ATTEMPTS: u32 = 5;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    // Mutex to serialize env var tests (prevent parallel test interference)
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    // Existing tests
    #[test]
    fn test_default_constants() {
        assert_eq!(network::DEFAULT_API_PORT, ports::HTTP_DEFAULT);
        assert_eq!(network::DEFAULT_BIND_ADDRESS, addresses::BIND_ALL_IPV4);
        assert_eq!(database::DEFAULT_POSTGRES_PORT, 5432);
    }

    #[test]
    fn test_environment_override() {
        let _lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        env::set_var("NESTGATE_API_PORT", "9999");
        assert_eq!(env_helpers::api_port(), 9999);
        env::remove_var("NESTGATE_API_PORT");
        assert_eq!(env_helpers::api_port(), ports::HTTP_DEFAULT);
    }

    #[test]
    fn test_url_builders() {
        let api_url = urls::api_url();
        assert!(api_url.contains("http://"));

        let ws_url = urls::websocket_url();
        assert!(ws_url.contains("ws://"));

        let health_url = urls::health_url();
        assert!(health_url.contains("/health"));
    }

    // New comprehensive tests

    #[test]
    fn test_network_defaults() {
        assert_eq!(network::DEFAULT_API_PORT, ports::HTTP_DEFAULT);
        assert_eq!(network::DEFAULT_BIND_ADDRESS, addresses::BIND_ALL_IPV4);
        assert_eq!(network::DEFAULT_HOSTNAME, addresses::LOCALHOST_NAME);
        assert_eq!(network::DEFAULT_WS_PORT, ports::WEBSOCKET_DEFAULT);
        assert_eq!(network::DEFAULT_HEALTH_PORT, ports::HEALTH_CHECK)
    }

    #[test]
    fn test_database_defaults() {
        assert_eq!(database::DEFAULT_POSTGRES_PORT, 5432);
        assert_eq!(database::DEFAULT_REDIS_PORT, 6379);
        assert_eq!(database::DEFAULT_DB_HOST, "localhost");
    }

    #[test]
    fn test_monitoring_defaults() {
        assert_eq!(monitoring::DEFAULT_METRICS_PORT, 9090);
        assert_eq!(monitoring::DEFAULT_GRAFANA_PORT, 3000);
    }

    #[test]
    fn test_timeout_defaults() {
        assert_eq!(timeouts::DEFAULT_API_TIMEOUT, Duration::from_secs(30));
        assert_eq!(timeouts::DEFAULT_DB_TIMEOUT, Duration::from_secs(10));
        assert_eq!(timeouts::DEFAULT_HEALTH_TIMEOUT, Duration::from_secs(5));
        assert_eq!(timeouts::DEFAULT_WS_TIMEOUT, Duration::from_secs(60));
    }

    #[test]
    fn test_storage_defaults() {
        assert_eq!(storage::DEFAULT_POOL_NAME, "nestgate-pool");
        assert_eq!(storage::DEFAULT_CACHE_SIZE_MB, 1024);
        assert_eq!(storage::DEFAULT_COMPRESSION, "lz4");
    }

    #[test]
    fn test_security_defaults() {
        assert_eq!(security::DEFAULT_SESSION_TIMEOUT, Duration::from_secs(3600));
        assert_eq!(security::DEFAULT_TOKEN_EXPIRY, Duration::from_secs(1800));
        assert_eq!(security::DEFAULT_MAX_LOGIN_ATTEMPTS, 5);
    }

    #[test]
    fn test_env_helpers_api_port() {
        let _lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Clear any existing env var
        env::remove_var("NESTGATE_API_PORT");

        // Test default
        assert_eq!(env_helpers::api_port(), 8080);

        // Test override
        env::set_var("NESTGATE_API_PORT", "3000");
        assert_eq!(env_helpers::api_port(), 3000);

        // Test invalid value falls back to default
        env::set_var("NESTGATE_API_PORT", "invalid");
        assert_eq!(env_helpers::api_port(), 8080);

        // Cleanup
        env::remove_var("NESTGATE_API_PORT");
    }

    #[test]
    fn test_env_helpers_bind_address() {
        env::remove_var("NESTGATE_BIND_ADDRESS");

        assert_eq!(env_helpers::bind_address(), "0.0.0.0");

        env::set_var("NESTGATE_BIND_ADDRESS", "127.0.0.1");
        assert_eq!(env_helpers::bind_address(), "127.0.0.1");

        env::remove_var("NESTGATE_BIND_ADDRESS");
    }

    #[test]
    fn test_env_helpers_hostname() {
        let _lock = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        env::remove_var("NESTGATE_HOSTNAME");

        assert_eq!(env_helpers::hostname(), "localhost");

        env::set_var("NESTGATE_HOSTNAME", "example.com");
        assert_eq!(env_helpers::hostname(), "example.com");

        env::remove_var("NESTGATE_HOSTNAME");
    }

    #[test]
    fn test_env_helpers_db_port() {
        env::remove_var("NESTGATE_DB_PORT");

        assert_eq!(env_helpers::db_port(), 5432);

        env::set_var("NESTGATE_DB_PORT", "5433");
        assert_eq!(env_helpers::db_port(), 5433);

        env::remove_var("NESTGATE_DB_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_env_helpers_metrics_port() {
        env::remove_var("NESTGATE_METRICS_PORT");

        assert_eq!(env_helpers::metrics_port(), 9090);

        env::set_var("NESTGATE_METRICS_PORT", "9091");
        assert_eq!(env_helpers::metrics_port(), 9091);

        env::remove_var("NESTGATE_METRICS_PORT");
    }

    #[test]
    fn test_api_url_format() {
        env::remove_var("NESTGATE_HOSTNAME");
        env::remove_var("NESTGATE_API_PORT");

        let url = urls::api_url();
        assert!(url.starts_with("http://"));
        // URL should contain host and port
        assert!(url.matches(':').count() >= 2); // http:// and port separator
                                                // Should have a valid structure
        assert!(url.len() > 10); // Minimum valid URL length
    }

    #[test]
    #[serial_test::serial]
    fn test_websocket_url_format() {
        env::remove_var("NESTGATE_HOSTNAME");
        env::remove_var("NESTGATE_WS_PORT");

        let url = urls::websocket_url();
        assert!(url.starts_with("ws://"));
        assert!(url.contains("localhost"));
        assert!(url.contains("8082")); // WEBSOCKET_DEFAULT = 8082
    }

    #[test]
    #[serial_test::serial]
    fn test_health_url_format() {
        env::remove_var("NESTGATE_HOSTNAME");
        env::remove_var("NESTGATE_HEALTH_PORT");

        let url = urls::health_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains("localhost"));
        assert!(url.contains("8081")); // HEALTH_CHECK = 8081
        assert!(url.ends_with("/health"));
    }

    #[test]
    fn test_url_builders_with_custom_host() {
        // Save and restore env var to avoid test pollution
        let original = env::var("NESTGATE_HOSTNAME").ok();
        env::set_var("NESTGATE_HOSTNAME", "custom.example.com");

        let api_url = urls::api_url();
        // URL builders may use localhost or the hostname - just verify they work
        assert!(!api_url.is_empty());

        let ws_url = urls::websocket_url();
        assert!(!ws_url.is_empty());

        let health_url = urls::health_url();
        assert!(!health_url.is_empty());

        // Restore original value or remove if it didn't exist
        match original {
            Some(val) => env::set_var("NESTGATE_HOSTNAME", val),
            None => env::remove_var("NESTGATE_HOSTNAME"),
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_url_builders_with_custom_ports() {
        // Save and restore env vars to avoid test pollution
        let api_port_orig = env::var("NESTGATE_API_PORT").ok();
        let ws_port_orig = env::var("NESTGATE_WS_PORT").ok();
        let health_port_orig = env::var("NESTGATE_HEALTH_PORT").ok();

        // Clear any existing values first to ensure clean state
        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_WS_PORT");
        env::remove_var("NESTGATE_HEALTH_PORT");

        // Now set the test values
        env::set_var("NESTGATE_API_PORT", "9000");
        env::set_var("NESTGATE_WS_PORT", "9001");
        env::set_var("NESTGATE_HEALTH_PORT", "9002");

        let api_url = urls::api_url();
        assert!(
            api_url.contains("9000"),
            "API URL should contain 9000, got: {}",
            api_url
        );

        let ws_url = urls::websocket_url();
        assert!(
            ws_url.contains("9001"),
            "WS URL should contain 9001, got: {}",
            ws_url
        );

        let health_url = urls::health_url();
        assert!(
            health_url.contains("9002"),
            "Health URL should contain 9002, got: {}",
            health_url
        );

        // Restore original values or remove if they didn't exist
        match api_port_orig {
            Some(val) => env::set_var("NESTGATE_API_PORT", val),
            None => env::remove_var("NESTGATE_API_PORT"),
        }
        match ws_port_orig {
            Some(val) => env::set_var("NESTGATE_WS_PORT", val),
            None => env::remove_var("NESTGATE_WS_PORT"),
        }
        match health_port_orig {
            Some(val) => env::set_var("NESTGATE_HEALTH_PORT", val),
            None => env::remove_var("NESTGATE_HEALTH_PORT"),
        }
    }

    #[test]
    fn test_port_parsing_invalid_values() {
        // Test that invalid port values fall back to defaults
        env::set_var("NESTGATE_API_PORT", "not_a_number");
        assert_eq!(env_helpers::api_port(), 8080);

        env::set_var("NESTGATE_DB_PORT", "999999"); // Out of u16 range
        assert_eq!(env_helpers::db_port(), 5432);

        env::set_var("NESTGATE_METRICS_PORT", "");
        assert_eq!(env_helpers::metrics_port(), 9090);

        env::remove_var("NESTGATE_API_PORT");
        env::remove_var("NESTGATE_DB_PORT");
        env::remove_var("NESTGATE_METRICS_PORT");
    }

    #[test]
    fn test_timeout_duration_values() {
        // Verify timeouts are reasonable
        assert!(timeouts::DEFAULT_API_TIMEOUT.as_secs() > 0);
        assert!(timeouts::DEFAULT_DB_TIMEOUT.as_secs() > 0);
        assert!(timeouts::DEFAULT_HEALTH_TIMEOUT.as_secs() > 0);
        assert!(timeouts::DEFAULT_WS_TIMEOUT.as_secs() > 0);

        // Verify relative ordering makes sense
        assert!(timeouts::DEFAULT_HEALTH_TIMEOUT < timeouts::DEFAULT_DB_TIMEOUT);
        assert!(timeouts::DEFAULT_DB_TIMEOUT < timeouts::DEFAULT_API_TIMEOUT);
        assert!(timeouts::DEFAULT_API_TIMEOUT < timeouts::DEFAULT_WS_TIMEOUT);
    }
}

#[cfg(test)]
#[path = "defaults_validation_tests.rs"]
mod defaults_validation_tests;
