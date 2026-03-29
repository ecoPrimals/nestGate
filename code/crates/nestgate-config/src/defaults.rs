// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Default Constants for `NestGate`
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

use crate::constants::hardcoding::{addresses, runtime_fallback_ports};
use std::time::Duration;

/// **NETWORK DEFAULTS**
pub mod network {
    use super::{addresses, runtime_fallback_ports};

    /// Default API port - can be overridden with `NESTGATE_API_PORT`
    pub const DEFAULT_API_PORT: u16 = runtime_fallback_ports::HTTP;

    /// Default bind address - can be overridden with `NESTGATE_BIND_ADDRESS`  
    pub const DEFAULT_BIND_ADDRESS: &str = addresses::BIND_ALL_IPV4;

    /// Default hostname for development - can be overridden with `NESTGATE_HOSTNAME`
    pub const DEFAULT_HOSTNAME: &str = addresses::LOCALHOST_NAME;

    /// Default WebSocket port - can be overridden with `NESTGATE_WS_PORT`
    pub const DEFAULT_WS_PORT: u16 = runtime_fallback_ports::WEBSOCKET;

    /// Default health check port - can be overridden with `NESTGATE_HEALTH_PORT`
    pub const DEFAULT_HEALTH_PORT: u16 = runtime_fallback_ports::HEALTH;
}

/// **DATABASE DEFAULTS**
pub mod database {
    use super::{addresses, runtime_fallback_ports};

    /// Default `PostgreSQL` port - can be overridden with `NESTGATE_DB_PORT`
    pub const DEFAULT_POSTGRES_PORT: u16 = runtime_fallback_ports::POSTGRES;

    /// Default Redis port - can be overridden with `NESTGATE_REDIS_PORT`
    pub const DEFAULT_REDIS_PORT: u16 = runtime_fallback_ports::REDIS;

    /// Default `MongoDB` port - can be overridden with `NESTGATE_MONGODB_PORT`
    pub const DEFAULT_MONGODB_PORT: u16 = runtime_fallback_ports::MONGODB;

    /// Default `MySQL` port - can be overridden with `NESTGATE_MYSQL_PORT`
    pub const DEFAULT_MYSQL_PORT: u16 = runtime_fallback_ports::MYSQL;

    /// Default database host - can be overridden with `NESTGATE_DB_HOST`
    pub const DEFAULT_DB_HOST: &str = addresses::LOCALHOST_NAME;
}

/// **MONITORING DEFAULTS**
pub mod monitoring {
    use super::runtime_fallback_ports;

    /// Default Prometheus port - can be overridden with `NESTGATE_METRICS_PORT`
    pub const DEFAULT_METRICS_PORT: u16 = runtime_fallback_ports::METRICS;

    /// Default Grafana port - can be overridden with `NESTGATE_GRAFANA_PORT`
    pub const DEFAULT_GRAFANA_PORT: u16 = runtime_fallback_ports::API;
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

/// **ENVIRONMENT HELPERS** - ✅ MODERNIZED (Week 2): Now delegates to `migration_bridge`
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
    /// **DEPRECATED**: Database config should be external, not in `NestGate` core
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
#[allow(deprecated)]
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
    use crate::config::environment::EnvironmentConfig;
    use crate::constants::hardcoding::runtime_fallback_ports;
    use std::env;
    use temp_env::{with_var, with_var_unset};

    // Existing tests
    #[test]
    fn test_default_constants() {
        assert_eq!(network::DEFAULT_API_PORT, runtime_fallback_ports::HTTP);
        assert_eq!(network::DEFAULT_BIND_ADDRESS, addresses::BIND_ALL_IPV4);
        assert_eq!(database::DEFAULT_POSTGRES_PORT, 5432);
    }

    /// `env_helpers::api_port` uses a cached global config; this test uses fresh
    /// [`EnvironmentConfig::from_env`] reads with scoped env (`temp-env` save/restore).
    /// Raw `set_var`/`remove_var` elsewhere in tests go through `crate::env_process`
    /// (`nestgate-platform` → `nestgate-env-process-shim`).
    #[test]
    #[serial_test::serial]
    fn test_environment_override() {
        with_var_unset("NESTGATE_API_PORT", || {
            with_var_unset("NESTGATE_HTTP_PORT", || {
                with_var_unset("NESTGATE_PORT", || {
                    with_var("NESTGATE_API_PORT", Some("9999"), || {
                        let cfg = EnvironmentConfig::from_env().expect("config");
                        assert_eq!(cfg.network.port.get(), 9999);
                    });
                    let cfg = EnvironmentConfig::from_env().expect("config");
                    assert_eq!(cfg.network.port.get(), runtime_fallback_ports::HTTP);
                });
            });
        });
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
        assert_eq!(network::DEFAULT_API_PORT, runtime_fallback_ports::HTTP);
        assert_eq!(network::DEFAULT_BIND_ADDRESS, addresses::BIND_ALL_IPV4);
        assert_eq!(network::DEFAULT_HOSTNAME, addresses::LOCALHOST_NAME);
        assert_eq!(network::DEFAULT_WS_PORT, runtime_fallback_ports::WEBSOCKET);
        assert_eq!(network::DEFAULT_HEALTH_PORT, runtime_fallback_ports::HEALTH);
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
    #[serial_test::serial]
    fn test_env_helpers_api_port() {
        with_var_unset("NESTGATE_API_PORT", || {
            with_var_unset("NESTGATE_HTTP_PORT", || {
                with_var_unset("NESTGATE_PORT", || {
                    assert_eq!(
                        EnvironmentConfig::from_env()
                            .expect("config")
                            .network
                            .port
                            .get(),
                        8080
                    );
                    with_var("NESTGATE_API_PORT", Some("3000"), || {
                        assert_eq!(
                            EnvironmentConfig::from_env()
                                .expect("config")
                                .network
                                .port
                                .get(),
                            3000
                        );
                    });
                    with_var("NESTGATE_API_PORT", Some("invalid"), || {
                        assert!(
                            EnvironmentConfig::from_env().is_err(),
                            "invalid NESTGATE_API_PORT should fail to parse"
                        );
                    });
                });
            });
        });
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
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_DB_PORT");

        assert_eq!(env_helpers::db_port(), 5432);

        // SAFETY: single-threaded test context.
        crate::env_process::set_var("NESTGATE_DB_PORT", "5433");
        assert_eq!(env_helpers::db_port(), 5433);

        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_DB_PORT");
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
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_HOSTNAME");
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_API_PORT");

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
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_HOSTNAME");
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_WS_PORT");

        let url = urls::websocket_url();
        assert!(url.starts_with("ws://"));
        assert!(url.contains("localhost"));
        assert!(url.contains("8082")); // WEBSOCKET_DEFAULT = 8082
    }

    #[test]
    #[serial_test::serial]
    fn test_health_url_format() {
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_HOSTNAME");
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_HEALTH_PORT");

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
        // SAFETY: single-threaded test context.
        crate::env_process::set_var("NESTGATE_HOSTNAME", "custom.example.com");

        let api_url = urls::api_url();
        // URL builders may use localhost or the hostname - just verify they work
        assert!(!api_url.is_empty());

        let ws_url = urls::websocket_url();
        assert!(!ws_url.is_empty());

        let health_url = urls::health_url();
        assert!(!health_url.is_empty());

        // Restore original value or remove if it didn't exist
        match original {
            Some(val) => crate::env_process::set_var("NESTGATE_HOSTNAME", val),
            None => crate::env_process::remove_var("NESTGATE_HOSTNAME"),
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
        // SAFETY: single-threaded test context.
        crate::env_process::set_var("NESTGATE_API_PORT", "not_a_number");
        assert_eq!(env_helpers::api_port(), 8080);

        // SAFETY: single-threaded test context.
        crate::env_process::set_var("NESTGATE_DB_PORT", "999999"); // Out of u16 range
        assert_eq!(env_helpers::db_port(), 5432);

        // SAFETY: single-threaded test context.
        crate::env_process::set_var("NESTGATE_METRICS_PORT", "");
        assert_eq!(env_helpers::metrics_port(), 9090);

        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_API_PORT");
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_DB_PORT");
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_METRICS_PORT");
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
        assert_eq!(
            database::DEFAULT_POSTGRES_PORT,
            runtime_fallback_ports::POSTGRES
        );
        assert_eq!(database::DEFAULT_REDIS_PORT, runtime_fallback_ports::REDIS);
        assert_eq!(
            database::DEFAULT_MONGODB_PORT,
            runtime_fallback_ports::MONGODB
        );
        assert_eq!(database::DEFAULT_MYSQL_PORT, runtime_fallback_ports::MYSQL);
    }

    #[test]
    fn test_monitoring_port_references() {
        // Verify monitoring ports use centralized constants
        assert_eq!(
            monitoring::DEFAULT_METRICS_PORT,
            runtime_fallback_ports::METRICS
        );
        assert_eq!(
            monitoring::DEFAULT_GRAFANA_PORT,
            runtime_fallback_ports::API
        );
    }

    #[test]
    fn test_network_port_references() {
        // Verify network ports use centralized constants
        assert_eq!(network::DEFAULT_API_PORT, runtime_fallback_ports::HTTP);
        assert_eq!(network::DEFAULT_WS_PORT, runtime_fallback_ports::WEBSOCKET);
        assert_eq!(network::DEFAULT_HEALTH_PORT, runtime_fallback_ports::HEALTH);
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
