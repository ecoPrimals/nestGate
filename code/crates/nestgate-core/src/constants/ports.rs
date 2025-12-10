//! Port Configuration Constants
//!
//! Centralized port definitions for all NestGate services.
//! These can be overridden via environment variables.
//!
//! **MIGRATION NOTE** (Week 2, Dec 2025): This module is being migrated to use
//! the modern `EnvironmentConfig` system. Constants are kept for backward
//! compatibility but helper functions are deprecated.
//!
//! # Recommended Usage (Modern)
//!
//! ```rust
//! use nestgate_core::config::environment::EnvironmentConfig;
//!
//! let config = EnvironmentConfig::from_env()?;
//! let api_port = config.network.port.get();          // Type-safe port
//! let metrics_port = config.monitoring.metrics_port.get();
//! ```
//!
//! # Legacy Usage (Deprecated)
//!
//! ```rust
//! use nestgate_core::constants::ports;
//!
//! // These constants still work but are deprecated
//! let api_port = ports::API_SERVER_DEFAULT;  // Hardcoded, no env support
//!
//! // These functions work but show deprecation warnings
//! let port = ports::api_server_port();  // Deprecated
//! ```

/// Default port for the main NestGate API server
///
/// **Environment Variable**: `NESTGATE_API_PORT`  
/// **Default**: 8080  
/// **Usage**: Main HTTP API endpoints
pub const API_SERVER_DEFAULT: u16 = 8080;

/// Default port for development server
///
/// **Environment Variable**: `NESTGATE_DEV_PORT`  
/// **Default**: 3000  
/// **Usage**: Development and hot-reload server
pub const DEV_SERVER_DEFAULT: u16 = 3000;

/// Default port for Prometheus metrics
///
/// **Environment Variable**: `NESTGATE_METRICS_PORT`  
/// **Default**: 9090  
/// **Usage**: Prometheus metrics endpoint
pub const METRICS_SERVER_DEFAULT: u16 = 9090;

/// Default port for Grafana dashboard
///
/// **Environment Variable**: `GRAFANA_PORT`  
/// **Default**: 3001  
/// **Usage**: Grafana monitoring dashboard
pub const GRAFANA_DEFAULT: u16 = 3001;

// ==================== DATABASE PORTS ====================

/// Default port for PostgreSQL database
///
/// **Environment Variable**: `POSTGRES_PORT`  
/// **Default**: 5432  
/// **Usage**: PostgreSQL database connections
pub const POSTGRES_DEFAULT: u16 = 5432;

/// Default port for Redis cache
///
/// **Environment Variable**: `REDIS_PORT`  
/// **Default**: 6379  
/// **Usage**: Redis cache and session store
pub const REDIS_DEFAULT: u16 = 6379;

/// Default port for MongoDB database
///
/// **Environment Variable**: `MONGODB_PORT`  
/// **Default**: 27017  
/// **Usage**: MongoDB document database
pub const MONGODB_DEFAULT: u16 = 27017;

// ==================== PRIMAL DISCOVERY PORTS ====================

/// Default port for Primal Discovery service
///
/// **Environment Variable**: `PRIMAL_DISCOVERY_PORT`  
/// **Default**: 5000  
/// **Usage**: Infant Discovery architecture service discovery
pub const PRIMAL_DISCOVERY_DEFAULT: u16 = 5000;

/// Default port for networking service (capability-based discovery preferred)
///
/// **Environment Variable**: `NETWORKING_SERVICE_PORT`  
/// **Default**: 9091  
/// **Usage**: Generic networking service communication (use capability discovery)
pub const NETWORKING_SERVICE_DEFAULT: u16 = 9091;

/// Default port for security service (capability-based discovery preferred)
///
/// **Environment Variable**: `SECURITY_SERVICE_PORT`  
/// **Default**: 9092  
/// **Usage**: Generic security service communication (use capability discovery)
pub const SECURITY_SERVICE_DEFAULT: u16 = 9092;

// ==================== HELPER FUNCTIONS ====================
// **MODERNIZED** (Week 2, Dec 2025): These functions now delegate to
// migration_bridge and are marked as deprecated to guide migration.

/// Get API server port with environment variable support
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.network.port.get()` instead
///
/// # Migration Example
///
/// ```rust
/// // OLD
/// let port = ports::api_server_port();
///
/// // NEW
/// let config = EnvironmentConfig::from_env()?;
/// let port = config.network.port.get();
/// ```
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.network.port instead"
)]
pub fn api_server_port() -> u16 {
    use crate::config::migration_bridge;
    #[allow(deprecated)]
    migration_bridge::get_api_port()
}

/// Get development server port with environment variable support
///
/// **DEPRECATED**: Use `EnvironmentConfig` with custom port configuration
#[deprecated(
    since = "0.6.0",
    note = "Configure development port via NESTGATE_PORT environment variable"
)]
pub fn dev_server_port() -> u16 {
    std::env::var("NESTGATE_DEV_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEV_SERVER_DEFAULT)
}

/// Get metrics server port with environment variable support
///
/// **DEPRECATED**: Use `EnvironmentConfig::from_env()?.monitoring.metrics_port.get()` instead
#[deprecated(
    since = "0.6.0",
    note = "Use EnvironmentConfig::from_env()?.monitoring.metrics_port instead"
)]
pub fn metrics_server_port() -> u16 {
    use crate::config::migration_bridge;
    #[allow(deprecated)]
    migration_bridge::get_metrics_port()
}

/// Get PostgreSQL port with environment variable support
///
/// **DEPRECATED**: Database configuration should be external to NestGate
#[deprecated(
    since = "0.6.0",
    note = "Database configuration should be managed externally"
)]
pub fn postgres_port() -> u16 {
    std::env::var("POSTGRES_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(POSTGRES_DEFAULT)
}

/// Get Redis port with environment variable support
///
/// **DEPRECATED**: Database configuration should be external to NestGate
#[deprecated(
    since = "0.6.0",
    note = "Database configuration should be managed externally"
)]
pub fn redis_port() -> u16 {
    std::env::var("REDIS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(REDIS_DEFAULT)
}

/// Get MongoDB port with environment variable support
///
/// **DEPRECATED**: Database configuration should be external to NestGate
#[deprecated(
    since = "0.6.0",
    note = "Database configuration should be managed externally"
)]
pub fn mongodb_port() -> u16 {
    std::env::var("MONGODB_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(MONGODB_DEFAULT)
}

/// Get Primal Discovery port with environment variable support
///
/// **DEPRECATED**: Use capability-based discovery instead of hardcoded ports
#[deprecated(
    since = "0.6.0",
    note = "Use capability-based service discovery instead of port constants"
)]
pub fn primal_discovery_port() -> u16 {
    std::env::var("PRIMAL_DISCOVERY_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(PRIMAL_DISCOVERY_DEFAULT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ports() {
        assert_eq!(API_SERVER_DEFAULT, 8080);
        assert_eq!(DEV_SERVER_DEFAULT, 3000);
        assert_eq!(POSTGRES_DEFAULT, 5432);
        assert_eq!(REDIS_DEFAULT, 6379);
        assert_eq!(MONGODB_DEFAULT, 27017);
        assert_eq!(PRIMAL_DISCOVERY_DEFAULT, 5000);
    }

    #[test]
    #[allow(deprecated)]
    fn test_port_helper_functions() {
        // Test deprecated helper functions - they now use EnvironmentConfig
        let api_port = api_server_port();
        assert!(api_port > 0);

        let dev_port = dev_server_port();
        assert!(dev_port > 0);

        let postgres = postgres_port();
        assert_eq!(postgres, POSTGRES_DEFAULT);
    }

    #[test]
    #[allow(deprecated)]
    fn test_port_with_env_override() {
        // NOTE: With migration_bridge using OnceLock, environment variables
        // must be set BEFORE first access. In production, this is always the case.
        // For testing, we verify the helper functions work with cached config.
        let port = api_server_port();
        assert!(port > 0); // Valid port from cached config

        // Direct env var access still works for non-cached helpers
        let original = std::env::var("NESTGATE_DEV_PORT").ok();
        std::env::set_var("NESTGATE_DEV_PORT", "9999");
        assert_eq!(dev_server_port(), 9999);

        // Clean up immediately to avoid test pollution
        match original {
            Some(val) => std::env::set_var("NESTGATE_DEV_PORT", val),
            None => std::env::remove_var("NESTGATE_DEV_PORT"),
        }
    }

    #[test]
    fn test_invalid_env_port_falls_back() {
        // Save and clear any existing value to ensure clean test
        let original = std::env::var("NESTGATE_DEV_PORT").ok();

        // Clear environment variable first to prevent test pollution
        std::env::remove_var("NESTGATE_DEV_PORT");

        // Test that invalid port values fall back to defaults
        std::env::set_var("NESTGATE_DEV_PORT", "invalid");

        #[allow(deprecated)]
        let port = dev_server_port();
        assert_eq!(
            port, DEV_SERVER_DEFAULT,
            "Invalid port string should fall back to default"
        );

        // Clean up
        match original {
            Some(val) => std::env::set_var("NESTGATE_DEV_PORT", val),
            None => std::env::remove_var("NESTGATE_DEV_PORT"),
        }
    }
}
