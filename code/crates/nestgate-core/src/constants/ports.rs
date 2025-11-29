//! Port Configuration Constants
//!
//! Centralized port definitions for all NestGate services.
//! These can be overridden via environment variables.
//!
//! # Environment Variables
//!
//! Each port constant can be overridden using its corresponding environment variable:
//! - `NESTGATE_API_PORT` - Main API server port
//! - `NESTGATE_DEV_PORT` - Development server port
//! - `NESTGATE_METRICS_PORT` - Prometheus metrics port
//! - `POSTGRES_PORT` - PostgreSQL database port
//! - `REDIS_PORT` - Redis cache port
//! - `MONGODB_PORT` - MongoDB database port
//! - `PRIMAL_DISCOVERY_PORT` - Primal discovery service port
//!
//! # Usage
//!
//! ```rust
//! use nestgate_core::constants::ports;
//! use nestgate_core::config::migration_helpers::get_port;
//!
//! // Get port with environment variable fallback
//! let api_port = get_port(
//!     "NESTGATE_API_PORT",
//!     None,
//!     ports::API_SERVER_DEFAULT
//! );
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

/// Default port for Songbird (networking primal)
///
/// **Environment Variable**: `SONGBIRD_PORT`  
/// **Default**: 9091  
/// **Usage**: Songbird networking primal communication
pub const SONGBIRD_DEFAULT: u16 = 9091;

/// Default port for BearDog (security primal)
///
/// **Environment Variable**: `BEARDOG_PORT`  
/// **Default**: 9092  
/// **Usage**: BearDog security primal communication
pub const BEARDOG_DEFAULT: u16 = 9092;

// ==================== HELPER FUNCTIONS ====================

/// Get API server port with environment variable support
///
/// Checks `NESTGATE_API_PORT` environment variable, falls back to default
pub fn api_server_port() -> u16 {
    std::env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(API_SERVER_DEFAULT)
}

/// Get development server port with environment variable support
///
/// Checks `NESTGATE_DEV_PORT` environment variable, falls back to default
pub fn dev_server_port() -> u16 {
    std::env::var("NESTGATE_DEV_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEV_SERVER_DEFAULT)
}

/// Get metrics server port with environment variable support
///
/// Checks `NESTGATE_METRICS_PORT` environment variable, falls back to default
pub fn metrics_server_port() -> u16 {
    std::env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(METRICS_SERVER_DEFAULT)
}

/// Get PostgreSQL port with environment variable support
///
/// Checks `POSTGRES_PORT` environment variable, falls back to default
pub fn postgres_port() -> u16 {
    std::env::var("POSTGRES_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(POSTGRES_DEFAULT)
}

/// Get Redis port with environment variable support
///
/// Checks `REDIS_PORT` environment variable, falls back to default
pub fn redis_port() -> u16 {
    std::env::var("REDIS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(REDIS_DEFAULT)
}

/// Get MongoDB port with environment variable support
///
/// Checks `MONGODB_PORT` environment variable, falls back to default
pub fn mongodb_port() -> u16 {
    std::env::var("MONGODB_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(MONGODB_DEFAULT)
}

/// Get Primal Discovery port with environment variable support
///
/// Checks `PRIMAL_DISCOVERY_PORT` environment variable, falls back to default
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
    fn test_port_helper_functions() {
        // Test without environment variables (should return defaults)
        assert_eq!(api_server_port(), API_SERVER_DEFAULT);
        assert_eq!(dev_server_port(), DEV_SERVER_DEFAULT);
        assert_eq!(postgres_port(), POSTGRES_DEFAULT);
    }

    #[test]
    fn test_port_with_env_override() {
        // Save original value if it exists
        let original = std::env::var("NESTGATE_API_PORT").ok();

        // Set environment variable
        std::env::set_var("NESTGATE_API_PORT", "9999");
        assert_eq!(api_server_port(), 9999);

        // Clean up - restore original or remove
        match original {
            Some(val) => std::env::set_var("NESTGATE_API_PORT", val),
            None => std::env::remove_var("NESTGATE_API_PORT"),
        }

        // Verify it returns to expected value after cleanup
        let expected = std::env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(API_SERVER_DEFAULT);
        assert_eq!(api_server_port(), expected);
    }

    #[test]
    fn test_invalid_env_port_falls_back() {
        // Set invalid port value
        std::env::set_var("NESTGATE_API_PORT", "invalid");
        assert_eq!(api_server_port(), API_SERVER_DEFAULT);
        std::env::remove_var("NESTGATE_API_PORT");
    }
}
