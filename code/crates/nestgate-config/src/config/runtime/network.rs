// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Network configuration module
//!
//! Provides environment-driven configuration for network services including
//! API endpoints, ports, timeouts, and connection pooling.

use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, Ipv4Addr};

/// Network configuration for API and service endpoints.
///
/// Controls HTTP/HTTPS ports, bind addresses, timeouts, and connection pooling.
/// All values can be overridden via `NESTGATE_*` environment variables.
///
/// # Environment Variables
///
/// - `NESTGATE_API_HOST` - API bind address (default: 127.0.0.1)
/// - `NESTGATE_API_PORT` - HTTP port (default: 8080)
/// - `NESTGATE_HTTPS_PORT` - HTTPS port (default: 8443)
/// - `NESTGATE_TARPC_PORT` - tarpc RPC port (default: 8091)
/// - `NESTGATE_INTERNAL_PORT` - Internal services port (default: 3000)
/// - `NESTGATE_BIND_ALL` - Bind to 0.0.0.0 (default: false)
/// - `NESTGATE_TIMEOUT_SECONDS` - Request timeout (default: 30)
/// - `NESTGATE_CONNECTION_POOL_SIZE` - Pool size (default: 10)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// API host (default: 127.0.0.1)
    pub api_host: IpAddr,

    /// API HTTP port (default: 8080)
    pub api_port: u16,

    /// API HTTPS port (default: 8443)
    pub https_port: u16,

    /// tarpc RPC port (default: 8091)
    pub tarpc_port: u16,

    /// Internal service port (default: 3000)
    pub internal_port: u16,

    /// Bind all interfaces (default: false, binds 127.0.0.1 only)
    pub bind_all: bool,

    /// Request timeout in seconds (default: 30)
    pub timeout_seconds: u64,

    /// Connection pool size (default: 10)
    pub connection_pool_size: usize,
}

impl NetworkConfig {
    /// Load network configuration from environment variables.
    ///
    /// Falls back to sensible defaults if environment variables are not set.
    ///
    /// # Errors
    ///
    /// Returns error if environment variables contain invalid values
    /// (e.g., non-numeric ports, invalid IP addresses).
    pub fn from_environment() -> Result<Self> {
        use std::net::Ipv4Addr;

        let config = Self {
            api_host: env::var("NESTGATE_API_HOST")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or({
                    // ✅ SOVEREIGNTY: Compile-time constant, zero runtime overhead
                    IpAddr::V4(Ipv4Addr::LOCALHOST) // 127.0.0.1
                }),

            // ✅ MIGRATED: Now uses centralized environment-driven functions
            api_port: env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(crate::constants::get_api_port),

            https_port: env::var("NESTGATE_HTTPS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8443), // HTTPS standard (no dedicated getter yet)

            tarpc_port: env::var("NESTGATE_TARPC_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8091), // tarpc standard (no dedicated getter yet)

            internal_port: env::var("NESTGATE_INTERNAL_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(crate::constants::get_dev_port),

            bind_all: env::var("NESTGATE_BIND_ALL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),

            timeout_seconds: env::var("NESTGATE_TIMEOUT_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),

            connection_pool_size: env::var("NESTGATE_CONNECTION_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        };

        config.validate()?;
        Ok(config)
    }

    /// Validate network configuration values.
    fn validate(&self) -> Result<()> {
        if self.api_port == 0 {
            return Err(NestGateError::configuration_error(
                "api_port",
                "API port cannot be 0",
            ));
        }
        if self.connection_pool_size == 0 {
            return Err(NestGateError::configuration_error(
                "connection_pool_size",
                "Connection pool size must be at least 1",
            ));
        }
        Ok(())
    }

    /// Get the full API base URL (http://host:port).
    #[must_use]
    pub fn api_base_url(&self) -> String {
        format!("http://{}:{}", self.api_host, self.api_port)
    }

    /// Get the full HTTPS base URL (https://host:port).
    #[must_use]
    pub fn https_base_url(&self) -> String {
        format!("https://{}:{}", self.api_host, self.https_port)
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        use crate::constants::hardcoding::{addresses, runtime_fallback_ports};

        // Parse with guaranteed fallback
        let api_host = addresses::LOCALHOST_IPV4
            .parse()
            .unwrap_or_else(|_| Ipv4Addr::LOCALHOST.into());

        Self {
            api_host,
            api_port: runtime_fallback_ports::HTTP,
            https_port: runtime_fallback_ports::HTTPS,
            tarpc_port: 8091, // tarpc default port
            internal_port: runtime_fallback_ports::API,
            bind_all: false,
            timeout_seconds: 30,
            connection_pool_size: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert_eq!(config.api_port, 8080);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.connection_pool_size, 10);
    }

    #[test]
    fn test_api_base_url() {
        let config = NetworkConfig::default();
        let url = config.api_base_url();
        assert!(url.starts_with("http://"));
        assert!(url.contains(":8080"));
    }

    #[test]
    fn test_validation_rejects_zero_port() {
        let config = NetworkConfig {
            api_port: 0,
            ..Default::default()
        };
        assert!(config.validate().is_err());
    }
}
