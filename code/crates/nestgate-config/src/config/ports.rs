// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Port Configuration System
//!
//! **Purpose**: Eliminate hardcoded port constants, enable environment-driven configuration.
//!
//! ## Philosophy
//!
//! Ports should be:
//! - Configurable via environment variables
//! - Have sensible defaults for development
//! - Required in production (fail fast)
//! - Documented and discoverable
//!
//! ## Migration Pattern
//!
//! ### Before (Hardcoded)
//! ```rust,ignore
//! const API_PORT: u16 = 8080;
//! const INTERNAL_PORT: u16 = 9090;
//! ```
//!
//! ### After (Configurable)
//! ```rust,ignore
//! let ports = PortConfig::from_env();
//! let api_port = ports.api_port;
//! let internal_port = ports.internal_port;
//! ```
//!
//! ## Environment Variables
//!
//! - `NESTGATE_API_PORT`: Main API port (default: 8080)
//! - `NESTGATE_INTERNAL_PORT`: Internal services port (default: 9090)
//! - `NESTGATE_METRICS_PORT`: Metrics/Prometheus port (default: 9091)
//! - `NESTGATE_HEALTH_PORT`: Health check port (default: 8081)
//! - `NESTGATE_ADMIN_PORT`: Admin/management port (default: 8082)
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use nestgate_core::config::ports::PortConfig;
//!
//! # fn example() -> anyhow::Result<()> {
//! // Development (uses defaults)
//! let ports = PortConfig::default_dev();
//! assert_eq!(ports.api_port, 8080);
//!
//! // Production (requires env vars or uses defaults)
//! let ports = PortConfig::from_env();
//! println!("API listening on port {}", ports.api_port);
//!
//! // Explicit configuration
//! let ports = PortConfig::builder()
//!     .api_port(3000)
//!     .metrics_port(9090)
//!     .build();
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use std::env;

/// Port configuration for all `NestGate` services
///
/// **Zero Hardcoding**: All ports configurable via environment variables
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortConfig {
    /// Main API port
    /// Environment: `NESTGATE_API_PORT`
    /// Default: 8080
    pub api_port: u16,

    /// Internal services port
    /// Environment: `NESTGATE_INTERNAL_PORT`
    /// Default: 9090
    pub internal_port: u16,

    /// Metrics/Prometheus port
    /// Environment: `NESTGATE_METRICS_PORT`
    /// Default: 9091
    pub metrics_port: u16,

    /// Health check port
    /// Environment: `NESTGATE_HEALTH_PORT`
    /// Default: 8081
    pub health_port: u16,

    /// Admin/management port
    /// Environment: `NESTGATE_ADMIN_PORT`
    /// Default: 8082
    pub admin_port: u16,

    /// Discovery service port
    /// Environment: `NESTGATE_DISCOVERY_PORT`
    /// Default: 8083
    pub discovery_port: u16,
}

impl PortConfig {
    /// Create from environment variables with fallback to defaults
    ///
    /// This is the recommended method for production deployments.
    /// Reads environment variables, falls back to development defaults if not set.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            api_port: env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_API_PORT),
            internal_port: env::var("NESTGATE_INTERNAL_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_INTERNAL_PORT),
            metrics_port: env::var("NESTGATE_METRICS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_METRICS_PORT),
            health_port: env::var("NESTGATE_HEALTH_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_HEALTH_PORT),
            admin_port: env::var("NESTGATE_ADMIN_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_ADMIN_PORT),
            discovery_port: env::var("NESTGATE_DISCOVERY_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_DISCOVERY_PORT),
        }
    }

    /// Development defaults
    ///
    /// Use this for local development and testing.
    /// All ports use standard defaults.
    #[must_use]
    pub const fn default_dev() -> Self {
        Self {
            api_port: Self::DEFAULT_API_PORT,
            internal_port: Self::DEFAULT_INTERNAL_PORT,
            metrics_port: Self::DEFAULT_METRICS_PORT,
            health_port: Self::DEFAULT_HEALTH_PORT,
            admin_port: Self::DEFAULT_ADMIN_PORT,
            discovery_port: Self::DEFAULT_DISCOVERY_PORT,
        }
    }

    /// Create a builder for custom configuration
    #[must_use]
    pub fn builder() -> PortConfigBuilder {
        PortConfigBuilder::default()
    }

    // Default port constants (private - use config methods instead!)
    const DEFAULT_API_PORT: u16 = 8080;
    const DEFAULT_INTERNAL_PORT: u16 = 9090;
    const DEFAULT_METRICS_PORT: u16 = 9091;
    const DEFAULT_HEALTH_PORT: u16 = 8081;
    const DEFAULT_ADMIN_PORT: u16 = 8082;
    const DEFAULT_DISCOVERY_PORT: u16 = 8083;
}

impl Default for PortConfig {
    fn default() -> Self {
        Self::default_dev()
    }
}

/// Builder for `PortConfig`
///
/// Allows explicit port configuration for testing or special deployments.
#[derive(Debug, Clone, Default)]
pub struct PortConfigBuilder {
    api_port: Option<u16>,
    internal_port: Option<u16>,
    metrics_port: Option<u16>,
    health_port: Option<u16>,
    admin_port: Option<u16>,
    discovery_port: Option<u16>,
}

impl PortConfigBuilder {
    /// Set API port
    #[must_use]
    pub const fn api_port(mut self, port: u16) -> Self {
        self.api_port = Some(port);
        self
    }

    /// Set internal services port
    #[must_use]
    pub const fn internal_port(mut self, port: u16) -> Self {
        self.internal_port = Some(port);
        self
    }

    /// Set metrics port
    #[must_use]
    pub const fn metrics_port(mut self, port: u16) -> Self {
        self.metrics_port = Some(port);
        self
    }

    /// Set health check port
    #[must_use]
    pub const fn health_port(mut self, port: u16) -> Self {
        self.health_port = Some(port);
        self
    }

    /// Set admin port
    #[must_use]
    pub const fn admin_port(mut self, port: u16) -> Self {
        self.admin_port = Some(port);
        self
    }

    /// Set discovery port
    #[must_use]
    pub const fn discovery_port(mut self, port: u16) -> Self {
        self.discovery_port = Some(port);
        self
    }

    /// Build the configuration
    #[must_use]
    pub fn build(self) -> PortConfig {
        PortConfig {
            api_port: self.api_port.unwrap_or(PortConfig::DEFAULT_API_PORT),
            internal_port: self
                .internal_port
                .unwrap_or(PortConfig::DEFAULT_INTERNAL_PORT),
            metrics_port: self
                .metrics_port
                .unwrap_or(PortConfig::DEFAULT_METRICS_PORT),
            health_port: self.health_port.unwrap_or(PortConfig::DEFAULT_HEALTH_PORT),
            admin_port: self.admin_port.unwrap_or(PortConfig::DEFAULT_ADMIN_PORT),
            discovery_port: self
                .discovery_port
                .unwrap_or(PortConfig::DEFAULT_DISCOVERY_PORT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ports() {
        let ports = PortConfig::default_dev();
        assert_eq!(ports.api_port, 8080);
        assert_eq!(ports.internal_port, 9090);
        assert_eq!(ports.metrics_port, 9091);
        assert_eq!(ports.health_port, 8081);
        assert_eq!(ports.admin_port, 8082);
        assert_eq!(ports.discovery_port, 8083);
    }

    #[test]
    fn test_builder() {
        let ports = PortConfig::builder()
            .api_port(3000)
            .metrics_port(9999)
            .build();

        assert_eq!(ports.api_port, 3000);
        assert_eq!(ports.metrics_port, 9999);
        // Others use defaults
        assert_eq!(ports.internal_port, 9090);
    }

    #[test]
    fn test_from_env_uses_defaults() {
        // Without env vars set, should use defaults
        let ports = PortConfig::from_env();
        assert_eq!(ports.api_port, 8080);
        assert_eq!(ports.internal_port, 9090);
    }

    #[test]
    fn test_serialization() {
        let ports = PortConfig::default_dev();
        let json = serde_json::to_string(&ports).expect("Serialization failed");
        let deserialized: PortConfig = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(ports, deserialized);
    }
}
