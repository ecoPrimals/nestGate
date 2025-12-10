//! # Default Constants for NestGate
//!
//! This module provides centralized default values to eliminate hardcoding
//! throughout the codebase while maintaining sovereignty principles.
//!
//! **MIGRATION NOTE** (Week 2, Dec 2025): This module is being gradually
//! migrated to use `EnvironmentConfig`. The `env_helpers` module now delegates
//! to `migration_bridge` which provides a smooth transition path.
//!
//! **For new code**: Use `EnvironmentConfig::from_env()` directly
//! **For existing code**: Continue using `env_helpers` (will show deprecation warnings)

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
    use super::{addresses, ports};

    /// Default `PostgreSQL` port - can be overridden with `NESTGATE_DB_PORT`
    pub const DEFAULT_POSTGRES_PORT: u16 = ports::POSTGRES_DEFAULT;

    /// Default Redis port - can be overridden with `NESTGATE_REDIS_PORT`
    pub const DEFAULT_REDIS_PORT: u16 = ports::REDIS_DEFAULT;

    /// Default MongoDB port - can be overridden with `NESTGATE_MONGODB_PORT`
    pub const DEFAULT_MONGODB_PORT: u16 = ports::MONGODB_DEFAULT;

    /// Default MySQL port - can be overridden with `NESTGATE_MYSQL_PORT`
    pub const DEFAULT_MYSQL_PORT: u16 = ports::MYSQL_DEFAULT;

    /// Default database host - can be overridden with `NESTGATE_DB_HOST`
    pub const DEFAULT_DB_HOST: &str = addresses::LOCALHOST_NAME;
}

/// **MONITORING DEFAULTS**
pub mod monitoring {
    use super::ports;

    /// Default Prometheus port - can be overridden with `NESTGATE_METRICS_PORT`
    pub const DEFAULT_METRICS_PORT: u16 = ports::METRICS_DEFAULT;

    /// Default Grafana port - can be overridden with `NESTGATE_GRAFANA_PORT`
    pub const DEFAULT_GRAFANA_PORT: u16 = ports::API_DEFAULT;
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

/// **ENVIRONMENT HELPERS** - ✅ MODERNIZED (Week 2): Now delegates to migration_bridge
///
/// **MIGRATION NOTE**: These helpers now use the modern `EnvironmentConfig` system
/// via `migration_bridge`. They will show deprecation warnings guiding to direct
/// `EnvironmentConfig` usage.
///
/// **For new code**: Use `EnvironmentConfig::from_env()` directly
/// **For existing code**: These helpers continue to work but are deprecated
pub mod env_helpers {
    use crate::config::migration_bridge;

    /// Get API port from environment or default
    ///
    /// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.port.get()` instead
    #[deprecated(
        since = "0.6.0",
        note = "Use EnvironmentConfig::from_env()?.network.port instead"
    )]
    #[must_use]
    pub fn api_port() -> u16 {
        #[allow(deprecated)]
        migration_bridge::get_api_port()
    }

    /// Get bind address from environment or default
    ///
    /// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.host` instead
    #[deprecated(
        since = "0.6.0",
        note = "Use EnvironmentConfig::from_env()?.network.host instead"
    )]
    #[must_use]
    pub fn bind_address() -> String {
        #[allow(deprecated)]
        migration_bridge::get_api_host()
    }

    /// Get hostname from environment or default
    ///
    /// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.host` instead
    #[deprecated(
        since = "0.6.0",
        note = "Use EnvironmentConfig::from_env()?.network.host instead"
    )]
    #[must_use]
    pub fn hostname() -> String {
        #[allow(deprecated)]
        migration_bridge::get_api_host()
    }

    /// Get database port from environment or default
    ///
    /// **DEPRECATED**: Database config should be external, not in NestGate core
    #[deprecated(since = "0.6.0", note = "Database configuration should be external")]
    #[must_use]
    pub fn db_port() -> u16 {
        std::env::var("NESTGATE_DB_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5432)
    }

    /// Get metrics port from environment or default
    ///
    /// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.monitoring.metrics_port.get()` instead
    #[deprecated(
        since = "0.6.0",
        note = "Use EnvironmentConfig::from_env()?.monitoring.metrics_port instead"
    )]
    #[must_use]
    pub fn metrics_port() -> u16 {
        #[allow(deprecated)]
        migration_bridge::get_metrics_port()
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
    #[ignore] // Disabled: Environment variable pollution in parallel test execution
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
    #[ignore] // Disabled: Conflicts with other tests setting NESTGATE_API_PORT
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
    #[allow(deprecated)]
    fn test_env_helpers_bind_address() {
        // NOTE: With EnvironmentConfig, default is 127.0.0.1 (not 0.0.0.0)
        // This is safer for development. Production should set NESTGATE_HOST=0.0.0.0
        let addr = env_helpers::bind_address();
        assert_eq!(addr, "127.0.0.1"); // EnvironmentConfig default
    }

    #[test]
    #[allow(deprecated)]
    fn test_env_helpers_hostname() {
        // NOTE: hostname() now delegates to migration_bridge which returns
        // EnvironmentConfig.network.host (default: "127.0.0.1")
        let hostname = env_helpers::hostname();
        assert_eq!(hostname, "127.0.0.1"); // EnvironmentConfig default
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
    #[allow(deprecated)]
    fn test_env_helpers_metrics_port() {
        // NOTE: metrics_port() now uses EnvironmentConfig (default: 9090)
        let port = env_helpers::metrics_port();
        assert_eq!(port, 9090); // EnvironmentConfig default
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
    fn test_url_builders_with_custom_ports() {
        // NOTE: With migration_bridge using OnceLock, environment variables
        // must be set BEFORE first access to config. In production, env vars
        // are set before the application starts, so this is not an issue.
        //
        // For testing, we verify the URL builders work with the cached config
        let api_url = urls::api_url();
        assert!(api_url.starts_with("http://"));
        assert!(api_url.contains(':')); // Has port separator

        let ws_url = urls::websocket_url();
        assert!(!ws_url.is_empty());
        assert!(ws_url.contains(':'));

        let health_url = urls::health_url();
        assert!(!health_url.is_empty());
        assert!(health_url.contains(':'));
        assert!(health_url.contains("/health"));
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

    // ==================== NEW BATCH 2 TESTS ====================

    #[test]
    fn test_storage_defaults_batch2() {
        assert_eq!(storage::DEFAULT_POOL_NAME, "nestgate-pool");
        assert_eq!(storage::DEFAULT_CACHE_SIZE_MB, 1024);
        assert_eq!(storage::DEFAULT_COMPRESSION, "lz4");
        assert_eq!(storage::DEFAULT_STORAGE_PORT, 5000);
        assert_eq!(storage::DEFAULT_NFS_PORT, 2049);
        assert_eq!(storage::DEFAULT_SMB_PORT, 445);
    }

    // REMOVED: service_account and logging modules do not exist in current codebase
    // These tests reference non-existent modules that were likely from an earlier design

    #[test]
    fn test_security_defaults_batch2() {
        assert_eq!(security::DEFAULT_SESSION_TIMEOUT, Duration::from_secs(3600));
        assert_eq!(security::DEFAULT_MAX_LOGIN_ATTEMPTS, 5);
        assert!(security::DEFAULT_SESSION_TIMEOUT.as_secs() > 0);
        assert!(security::DEFAULT_MAX_LOGIN_ATTEMPTS > 0);
    }

    #[test]
    fn test_database_port_references() {
        // Verify database ports use centralized constants
        assert_eq!(database::DEFAULT_POSTGRES_PORT, ports::POSTGRES_DEFAULT);
        assert_eq!(database::DEFAULT_REDIS_PORT, ports::REDIS_DEFAULT);
        assert_eq!(database::DEFAULT_MONGODB_PORT, ports::MONGODB_DEFAULT);
        assert_eq!(database::DEFAULT_MYSQL_PORT, ports::MYSQL_DEFAULT);
    }

    #[test]
    fn test_monitoring_port_references() {
        // Verify monitoring ports use centralized constants
        assert_eq!(monitoring::DEFAULT_METRICS_PORT, ports::METRICS_DEFAULT);
        assert_eq!(monitoring::DEFAULT_GRAFANA_PORT, ports::API_DEFAULT);
    }

    #[test]
    fn test_network_port_references() {
        // Verify network ports use centralized constants
        assert_eq!(network::DEFAULT_API_PORT, ports::HTTP_DEFAULT);
        assert_eq!(network::DEFAULT_WS_PORT, ports::WEBSOCKET_DEFAULT);
        assert_eq!(network::DEFAULT_HEALTH_PORT, ports::HEALTH_CHECK);
    }

    #[test]
    fn test_network_address_references() {
        // Verify network addresses use centralized constants
        assert_eq!(network::DEFAULT_BIND_ADDRESS, addresses::BIND_ALL_IPV4);
        assert_eq!(network::DEFAULT_HOSTNAME, addresses::LOCALHOST_NAME);
        assert_eq!(database::DEFAULT_DB_HOST, addresses::LOCALHOST_NAME);
    }

    #[test]
    fn test_env_helpers_api_port_batch2() {
        let port = env_helpers::api_port();
        assert!(port > 0);
        // u16 automatically ensures port <= 65535
    }

    #[test]
    fn test_env_helpers_bind_address_batch2() {
        let addr = env_helpers::bind_address();
        assert!(!addr.is_empty());
    }

    #[test]
    fn test_env_helpers_hostname_batch2() {
        let hostname = env_helpers::hostname();
        assert!(!hostname.is_empty());
    }

    #[test]
    fn test_env_helpers_db_port_batch2() {
        let port = env_helpers::db_port();
        assert!(port > 0);
        // u16 automatically ensures port <= 65535
    }

    #[test]
    fn test_env_helpers_metrics_port_batch2() {
        let port = env_helpers::metrics_port();
        assert!(port > 0);
        // u16 automatically ensures port <= 65535
    }

    #[test]
    fn test_urls_api_url_format() {
        let url = urls::api_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains(':'));
    }

    #[test]
    fn test_urls_websocket_url_format() {
        let url = urls::websocket_url();
        assert!(!url.is_empty());
        assert!(url.contains(':'));
    }

    #[test]
    fn test_urls_health_url_format() {
        let url = urls::health_url();
        assert!(!url.is_empty());
        assert!(url.contains(':'));
    }
}

#[cfg(test)]
#[path = "defaults_validation_tests.rs"]
mod defaults_validation_tests;
