// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Environment-Driven Network Configuration
//!
//! **Philosophy**: Configuration flexibility without hardcoding
//!
//! This module provides environment-driven alternatives to hardcoded network values.
//! Each function checks environment variables first, then falls back to sensible defaults.
//!
//! ## Evolution from Hardcoding
//!
//! **Before** (hardcoded):
//! ```rust,ignore
//! const API_PORT: u16 = 8080; // HARDCODED!
//! ```
//!
//! **After** (environment-driven):
//! ```rust,ignore
//! use nestgate_core::constants::network_environment::api_port;
//!
//! let port = api_port(); // Checks NESTGATE_API_PORT, defaults to 8080
//! ```
//!
//! ## Benefits
//!
//! - ✅ **Deployment Flexibility**: Different ports per environment
//! - ✅ **Zero Configuration**: Sensible defaults just work
//! - ✅ **Runtime Discovery**: Integrates with capability-based discovery
//! - ✅ **Testability**: Override via environment in tests
//!
//! ## Environment Variables
//!
//! | Variable | Default | Purpose |
//! |----------|---------|---------|
//! | `NESTGATE_API_PORT` | 8080 | Main API server |
//! | `NESTGATE_ADMIN_PORT` | 8081 | Admin interface |
//! | `NESTGATE_METRICS_PORT` | 9090 | Prometheus metrics |
//! | `NESTGATE_HEALTH_PORT` | 8082 | Health check endpoint |
//! | `NESTGATE_WEBSOCKET_PORT` | 8081 | WebSocket connections |
//! | `NESTGATE_DEV_PORT` | 3000 | Development server |
//! | `NESTGATE_POSTGRES_PORT` | 5432 | PostgreSQL database |
//!
//! ## Future Evolution
//!
//! These will integrate with NestGate's service metadata for full capability-based discovery:
//! ```rust,ignore
//! // Future: Dynamic discovery
//! let services = nestgate::find_by_capability("api").await?;
//! let endpoint = &services[0].virtual_endpoint;
//! ```

use std::env;

use super::hardcoding::addresses;
use super::hardcoding::runtime_fallback_ports as fallback_ports;
use super::hardcoding::timeouts as fallback_timeouts;
use super::port_defaults::{
    DEFAULT_ADMIN_PORT, DEFAULT_API_PORT, DEFAULT_DEV_ALT_PORT, DEFAULT_DEV_PORT,
    DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT, DEFAULT_POSTGRES_PORT,
};
use super::timeouts::DEFAULT_KEEPALIVE_SECS;

// ==================== PORT CONFIGURATION ====================

/// Get API server port from environment or use default (8080)
///
/// **Environment**: `NESTGATE_API_PORT`\
/// **Default**: 8080\
/// **Usage**: Main API server, HTTP/REST endpoints
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::constants::network_environment::api_port;
///
/// // Uses NESTGATE_API_PORT if set, otherwise 8080
/// let port = api_port();
/// assert!(port > 0);
/// ```
#[must_use]
pub fn api_port() -> u16 {
    env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_API_PORT)
}

/// Get admin interface port from environment or use default (8081)
///
/// **Environment**: `NESTGATE_ADMIN_PORT`\
/// **Default**: 8081\
/// **Usage**: Admin interface, management endpoints
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::constants::network_environment::admin_port;
///
/// let port = admin_port();
/// assert!(port > 0);
/// ```
#[must_use]
pub fn admin_port() -> u16 {
    env::var("NESTGATE_ADMIN_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_ADMIN_PORT)
}

/// Get metrics port from environment or use default (9090)
///
/// **Environment**: `NESTGATE_METRICS_PORT`\
/// **Default**: 9090 (Prometheus standard)\
/// **Usage**: Prometheus metrics, monitoring exporters
///
/// # Rationale
///
/// Port 9090 is the de facto standard for Prometheus metrics exporters.
/// Using this default enables zero-configuration monitoring integration.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::constants::network_environment::metrics_port;
///
/// let port = metrics_port();
/// assert_eq!(port, 9090); // Default Prometheus port
/// ```
#[must_use]
pub fn metrics_port() -> u16 {
    env::var("NESTGATE_METRICS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_METRICS_PORT)
}

/// Get health check port from environment or use default (8082)
///
/// **Environment**: `NESTGATE_HEALTH_PORT`\
/// **Default**: 8082\
/// **Usage**: Health check endpoint, load balancer probes
///
/// # Rationale
///
/// Separate port from main API allows:
/// - Load balancer health checks without API authentication
/// - Monitoring when API is overloaded
/// - Independent firewall rules
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::constants::network_environment::health_port;
///
/// let port = health_port();
/// assert!(port > 0);
/// ```
#[must_use]
pub fn health_port() -> u16 {
    env::var("NESTGATE_HEALTH_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_HEALTH_PORT)
}

/// Get WebSocket port from environment or use default (8081)
///
/// **Environment**: `NESTGATE_WEBSOCKET_PORT`\
/// **Default**: 8081\
/// **Usage**: WebSocket connections, real-time updates
#[must_use]
pub fn websocket_port() -> u16 {
    env::var("NESTGATE_WEBSOCKET_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_ADMIN_PORT)
}

/// Get development server port from environment or use default (3000)
///
/// **Environment**: `NESTGATE_DEV_PORT`\
/// **Default**: 3000\
/// **Usage**: Development mode, local testing
#[must_use]
pub fn dev_port() -> u16 {
    env::var("NESTGATE_DEV_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_DEV_PORT)
}

/// Get alternative development port from environment or use default (5000)
///
/// **Environment**: `NESTGATE_DEV_ALT_PORT`\
/// **Default**: 5000\
/// **Usage**: Alternative dev server, parallel instances
#[must_use]
pub fn dev_alt_port() -> u16 {
    env::var("NESTGATE_DEV_ALT_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_DEV_ALT_PORT)
}

/// Get `PostgreSQL` port from environment or use default (5432)
///
/// **Environment**: `NESTGATE_POSTGRES_PORT`\
/// **Default**: 5432 (`PostgreSQL` standard)\
/// **Usage**: `PostgreSQL` database connections
#[must_use]
pub fn postgres_port() -> u16 {
    env::var("NESTGATE_POSTGRES_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_POSTGRES_PORT)
}

/// Get HTTPS port from environment or use default (8443)
///
/// **Environment**: `NESTGATE_HTTPS_PORT`\
/// **Default**: 8443\
/// **Usage**: HTTPS/TLS endpoints
#[must_use]
pub fn https_port() -> u16 {
    env::var("NESTGATE_HTTPS_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback_ports::HTTPS)
}

// ==================== ADDRESS CONFIGURATION ====================

/// Get bind address from environment or use default (0.0.0.0)
///
/// **Environment**: `NESTGATE_BIND_ADDRESS`\
/// **Default**: "0.0.0.0" (all interfaces)\
/// **Usage**: Server bind address
///
/// # Security Note
///
/// Default binds to all interfaces (0.0.0.0) for deployment flexibility.
/// Override with "127.0.0.1" for localhost-only in development.
#[must_use]
pub fn bind_address() -> String {
    env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| addresses::BIND_ALL_IPV4.to_string())
}

/// Get localhost address (always 127.0.0.1 for consistency)
///
/// **Usage**: Client connections to local services
///
/// # Note
///
/// This is intentionally NOT environment-driven as localhost
/// should always resolve to 127.0.0.1 for local connections.
#[must_use]
pub const fn localhost_ipv4() -> &'static str {
    addresses::LOCALHOST_IPV4
}

/// Get localhost name (always "localhost" for consistency)
///
/// **Usage**: Hostname for local service connections
#[must_use]
pub const fn localhost_name() -> &'static str {
    addresses::LOCALHOST_NAME
}

// ==================== TIMEOUT CONFIGURATION ====================

/// Get connection timeout from environment or use default (5000ms)
///
/// **Environment**: `NESTGATE_CONNECT_TIMEOUT_MS`\
/// **Default**: 5000 (5 seconds)\
/// **Usage**: Connection establishment timeout
#[must_use]
pub fn connect_timeout_ms() -> u64 {
    env::var("NESTGATE_CONNECT_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback_timeouts::CONNECT_MS)
}

/// Get request timeout from environment or use default (30000ms)
///
/// **Environment**: `NESTGATE_REQUEST_TIMEOUT_MS`\
/// **Default**: 30000 (30 seconds)\
/// **Usage**: Individual request timeout
#[must_use]
pub fn request_timeout_ms() -> u64 {
    env::var("NESTGATE_REQUEST_TIMEOUT_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback_timeouts::REQUEST_MS)
}

/// Get keep-alive interval from environment or use default (60000ms)
///
/// **Environment**: `NESTGATE_KEEPALIVE_MS`\
/// **Default**: 60000 (60 seconds)\
/// **Usage**: TCP keep-alive interval
#[must_use]
pub fn keepalive_ms() -> u64 {
    env::var("NESTGATE_KEEPALIVE_MS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_KEEPALIVE_SECS * 1000)
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::hardcoding::addresses;
    use crate::constants::hardcoding::timeouts as fallback_timeouts;
    use crate::constants::timeouts::DEFAULT_KEEPALIVE_SECS;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn test_api_port_default() {
        let orig = env::var("NESTGATE_API_PORT").ok();
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_API_PORT");
        let port = api_port();
        match orig {
            Some(v) => crate::env_process::set_var("NESTGATE_API_PORT", v),
            None => {}
        }
        assert_eq!(port, DEFAULT_API_PORT);
    }

    #[test]
    #[serial]
    fn test_api_port_environment() {
        let orig = env::var("NESTGATE_API_PORT").ok();
        // SAFETY: single-threaded test context.
        crate::env_process::set_var("NESTGATE_API_PORT", "9999");
        let port = api_port();
        match orig {
            Some(v) => crate::env_process::set_var("NESTGATE_API_PORT", v),
            None => crate::env_process::remove_var("NESTGATE_API_PORT"),
        }
        assert_eq!(port, 9999);
    }

    #[test]
    fn test_metrics_port_default() {
        let orig = env::var("NESTGATE_METRICS_PORT").ok();
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_METRICS_PORT");
        let port = metrics_port();
        match orig {
            Some(v) => crate::env_process::set_var("NESTGATE_METRICS_PORT", v),
            None => {}
        }
        assert_eq!(port, DEFAULT_METRICS_PORT);
    }

    #[test]
    fn test_bind_address_default() {
        let orig = env::var("NESTGATE_BIND_ADDRESS").ok();
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_BIND_ADDRESS");
        let addr = bind_address();
        match orig {
            Some(v) => crate::env_process::set_var("NESTGATE_BIND_ADDRESS", v),
            None => {}
        }
        assert_eq!(addr, addresses::BIND_ALL_IPV4);
    }

    #[test]
    fn test_bind_address_environment() {
        let orig = env::var("NESTGATE_BIND_ADDRESS").ok();
        // SAFETY: single-threaded test context.
        crate::env_process::set_var("NESTGATE_BIND_ADDRESS", "127.0.0.1");
        let addr = bind_address();
        match orig {
            Some(v) => crate::env_process::set_var("NESTGATE_BIND_ADDRESS", v),
            None => crate::env_process::remove_var("NESTGATE_BIND_ADDRESS"),
        }
        assert_eq!(addr, addresses::LOCALHOST_IPV4);
    }

    #[test]
    fn test_localhost_constants() {
        assert_eq!(localhost_ipv4(), addresses::LOCALHOST_IPV4);
        assert_eq!(localhost_name(), addresses::LOCALHOST_NAME);
    }

    #[test]
    fn test_timeout_defaults() {
        let orig_connect = env::var("NESTGATE_CONNECT_TIMEOUT_MS").ok();
        let orig_request = env::var("NESTGATE_REQUEST_TIMEOUT_MS").ok();
        let orig_keepalive = env::var("NESTGATE_KEEPALIVE_MS").ok();
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_CONNECT_TIMEOUT_MS");
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_REQUEST_TIMEOUT_MS");
        // SAFETY: single-threaded test context.
        crate::env_process::remove_var("NESTGATE_KEEPALIVE_MS");
        let connect = connect_timeout_ms();
        let request = request_timeout_ms();
        let keepalive = keepalive_ms();
        match orig_connect {
            Some(v) => crate::env_process::set_var("NESTGATE_CONNECT_TIMEOUT_MS", v),
            None => {}
        }
        match orig_request {
            Some(v) => crate::env_process::set_var("NESTGATE_REQUEST_TIMEOUT_MS", v),
            None => {}
        }
        match orig_keepalive {
            Some(v) => crate::env_process::set_var("NESTGATE_KEEPALIVE_MS", v),
            None => {}
        }
        assert_eq!(connect, fallback_timeouts::CONNECT_MS);
        assert_eq!(request, fallback_timeouts::REQUEST_MS);
        assert_eq!(keepalive, DEFAULT_KEEPALIVE_SECS * 1000);
    }
}
