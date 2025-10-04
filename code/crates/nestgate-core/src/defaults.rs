//! # Default Constants for NestGate
//!
//! This module provides centralized default values to eliminate hardcoding
//! throughout the codebase while maintaining sovereignty principles.

use std::time::Duration;

/// **NETWORK DEFAULTS**
pub mod network {
    /// Default API port - can be overridden with `NESTGATE_API_PORT`
    pub const DEFAULT_API_PORT: u16 = 8080;

    /// Default bind address - can be overridden with `NESTGATE_BIND_ADDRESS`  
    pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";

    /// Default hostname for development - can be overridden with `NESTGATE_HOSTNAME`
    pub const DEFAULT_HOSTNAME: &str = "localhost";

    /// Default WebSocket port - can be overridden with `NESTGATE_WS_PORT`
    pub const DEFAULT_WS_PORT: u16 = 8081;

    /// Default health check port - can be overridden with `NESTGATE_HEALTH_PORT`
    pub const DEFAULT_HEALTH_PORT: u16 = 8082;
}

/// **DATABASE DEFAULTS**
pub mod database {
    /// Default `PostgreSQL` port - can be overridden with `NESTGATE_DB_PORT`
    pub const DEFAULT_POSTGRES_PORT: u16 = 5432;

    /// Default Redis port - can be overridden with `NESTGATE_REDIS_PORT`
    pub const DEFAULT_REDIS_PORT: u16 = 6379;

    /// Default database host - can be overridden with `NESTGATE_DB_HOST`
    pub const DEFAULT_DB_HOST: &str = "localhost";
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
    use std::env;

    /// Get API port from environment or default
    #[must_use]
    pub fn api_port() -> u16 {
        // ✅ KEPT CONST: Returns primitive, no allocation
        env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(super::network::DEFAULT_API_PORT)
    }

    /// Get bind address from environment or default
    /// ✅ MODERNIZED: Removed const - returns String (requires allocation)
    #[must_use]
    pub fn bind_address() -> String {
        env::var("NESTGATE_BIND_ADDRESS")
            .unwrap_or_else(|_| super::network::DEFAULT_BIND_ADDRESS.to_string())
    }

    /// Get hostname from environment or default
    /// ✅ MODERNIZED: Removed const - returns String (requires allocation)
    #[must_use]
    pub fn hostname() -> String {
        env::var("NESTGATE_HOSTNAME")
            .unwrap_or_else(|_| super::network::DEFAULT_HOSTNAME.to_string())
    }

    /// Get database port from environment or default
    #[must_use]
    pub fn db_port() -> u16 {
        // ✅ KEPT CONST: Returns primitive, no allocation
        env::var("NESTGATE_DB_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(super::database::DEFAULT_POSTGRES_PORT)
    }

    /// Get metrics port from environment or default
    #[must_use]
    pub fn metrics_port() -> u16 {
        // ✅ KEPT CONST: Returns primitive, no allocation
        env::var("NESTGATE_METRICS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(super::monitoring::DEFAULT_METRICS_PORT)
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
        let port = std::env::var("NESTGATE_WS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(super::network::DEFAULT_WS_PORT);
        format!("ws://{}:{}", env_helpers::hostname(), port)
    }

    /// Build health check URL with environment-aware host and port
    /// ✅ MODERNIZED: Removed const - uses format! macro
    #[must_use]
    pub fn health_url() -> String {
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

    #[test]
    fn test_default_constants() {
        assert_eq!(network::DEFAULT_API_PORT, 8080);
        assert_eq!(network::DEFAULT_BIND_ADDRESS, "0.0.0.0");
        assert_eq!(database::DEFAULT_POSTGRES_PORT, 5432);
    }

    #[test]
    fn test_environment_override() {
        env::set_var("NESTGATE_API_PORT", "9999");
        assert_eq!(env_helpers::api_port(), 9999);
        env::remove_var("NESTGATE_API_PORT");
        assert_eq!(env_helpers::api_port(), 8080);
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
}
