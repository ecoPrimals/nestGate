// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Port configuration module
//!
//! Provides thread-safe configuration for all service ports including API, metrics,
//! database connections, and monitoring services. Ports are loaded from environment
//! variables with sensible defaults.
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::constants::port_defaults_config::PortConfig;
//!
//! // Load from environment
//! let config = PortConfig::from_env();
//! println!("API port: {}", config.get_api_port());
//! println!("Metrics port: {}", config.get_metrics_port());
//!
//! // Or build for testing
//! let test_config = PortConfig::new()
//!     .with_api_port(9999)
//!     .with_metrics_port(8888);
//! ```
use super::port_defaults::{
    DEFAULT_ADMIN_PORT, DEFAULT_API_PORT, DEFAULT_DEV_PORT, DEFAULT_GRAFANA_PORT,
    DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT, DEFAULT_POSTGRES_PORT, DEFAULT_PROMETHEUS_PORT,
    DEFAULT_REDIS_PORT,
};
use std::sync::Arc;

/// Thread-safe configuration for port defaults
///
/// Captures environment variables at initialization to prevent race conditions.
/// All port values are immutable after construction, making this safe to share across threads.
#[derive(Debug, Clone)]
pub struct PortConfig {
    // NestGate service ports
    api_port: Option<u16>,
    admin_port: Option<u16>,
    metrics_port: Option<u16>,
    health_port: Option<u16>,

    // Development ports
    dev_port: Option<u16>,

    // Database ports
    postgres_port: Option<u16>,
    redis_port: Option<u16>,

    // Monitoring ports
    prometheus_port: Option<u16>,
    grafana_port: Option<u16>,
}

/// Shared immutable reference to `PortConfig`
pub type SharedPortConfig = Arc<PortConfig>;

impl PortConfig {
    /// Create a new empty configuration (all values None, will use hardcoded defaults)
    #[must_use]
    pub const fn new() -> Self {
        Self {
            api_port: None,
            admin_port: None,
            metrics_port: None,
            health_port: None,
            dev_port: None,
            postgres_port: None,
            redis_port: None,
            prometheus_port: None,
            grafana_port: None,
        }
    }

    /// Create configuration from current environment variables
    /// This captures env vars at initialization time, making it thread-safe
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            api_port: std::env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            admin_port: std::env::var("NESTGATE_ADMIN_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            metrics_port: std::env::var("NESTGATE_METRICS_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            health_port: std::env::var("NESTGATE_HEALTH_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            dev_port: std::env::var("NESTGATE_DEV_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            postgres_port: std::env::var("NESTGATE_POSTGRES_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            redis_port: std::env::var("NESTGATE_REDIS_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            prometheus_port: std::env::var("NESTGATE_PROMETHEUS_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
            grafana_port: std::env::var("NESTGATE_GRAFANA_PORT")
                .ok()
                .and_then(|s| s.parse().ok()),
        }
    }

    // Port getters with fallback to hardcoded defaults

    /// Gets Api Port
    #[must_use]
    pub fn get_api_port(&self) -> u16 {
        self.api_port.unwrap_or(DEFAULT_API_PORT)
    }

    /// Gets Admin Port
    #[must_use]
    pub fn get_admin_port(&self) -> u16 {
        self.admin_port.unwrap_or(DEFAULT_ADMIN_PORT)
    }

    /// Gets Metrics Port
    #[must_use]
    pub fn get_metrics_port(&self) -> u16 {
        self.metrics_port.unwrap_or(DEFAULT_METRICS_PORT)
    }

    /// Gets Health Port
    #[must_use]
    pub fn get_health_port(&self) -> u16 {
        self.health_port.unwrap_or(DEFAULT_HEALTH_PORT)
    }

    /// Gets Dev Port
    #[must_use]
    pub fn get_dev_port(&self) -> u16 {
        self.dev_port.unwrap_or(DEFAULT_DEV_PORT)
    }

    /// Gets Postgres Port
    #[must_use]
    pub fn get_postgres_port(&self) -> u16 {
        self.postgres_port.unwrap_or(DEFAULT_POSTGRES_PORT)
    }

    /// Gets Redis Port
    #[must_use]
    pub fn get_redis_port(&self) -> u16 {
        self.redis_port.unwrap_or(DEFAULT_REDIS_PORT)
    }

    /// Gets Prometheus Port
    #[must_use]
    pub fn get_prometheus_port(&self) -> u16 {
        self.prometheus_port.unwrap_or(DEFAULT_PROMETHEUS_PORT)
    }

    /// Gets Grafana Port
    #[must_use]
    pub fn get_grafana_port(&self) -> u16 {
        self.grafana_port.unwrap_or(DEFAULT_GRAFANA_PORT)
    }

    // Builder methods for tests

    /// Builder method to set Api Port
    #[must_use]
    pub const fn with_api_port(mut self, port: u16) -> Self {
        self.api_port = Some(port);
        self
    }

    /// Builder method to set Admin Port
    #[must_use]
    pub const fn with_admin_port(mut self, port: u16) -> Self {
        self.admin_port = Some(port);
        self
    }

    /// Builder method to set Metrics Port
    #[must_use]
    pub const fn with_metrics_port(mut self, port: u16) -> Self {
        self.metrics_port = Some(port);
        self
    }

    /// Builder method to set Health Port
    #[must_use]
    pub const fn with_health_port(mut self, port: u16) -> Self {
        self.health_port = Some(port);
        self
    }

    /// Builder method to set Dev Port
    #[must_use]
    pub const fn with_dev_port(mut self, port: u16) -> Self {
        self.dev_port = Some(port);
        self
    }

    /// Builder method to set Postgres Port
    #[must_use]
    pub const fn with_postgres_port(mut self, port: u16) -> Self {
        self.postgres_port = Some(port);
        self
    }

    /// Builder method to set Redis Port
    #[must_use]
    pub const fn with_redis_port(mut self, port: u16) -> Self {
        self.redis_port = Some(port);
        self
    }

    /// Builder method to set Prometheus Port
    #[must_use]
    pub const fn with_prometheus_port(mut self, port: u16) -> Self {
        self.prometheus_port = Some(port);
        self
    }

    /// Builder method to set Grafana Port
    #[must_use]
    pub const fn with_grafana_port(mut self, port: u16) -> Self {
        self.grafana_port = Some(port);
        self
    }
}

impl Default for PortConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_config_new() {
        let config = PortConfig::new();

        // Should use hardcoded defaults
        assert_eq!(config.get_api_port(), 8080);
        assert_eq!(config.get_metrics_port(), 9090);
        assert_eq!(config.get_postgres_port(), 5432);
        assert_eq!(config.get_redis_port(), 6379);
    }

    #[test]
    fn test_port_config_builder() {
        let config = PortConfig::new()
            .with_api_port(9999)
            .with_metrics_port(8888)
            .with_postgres_port(5433);

        assert_eq!(config.get_api_port(), 9999);
        assert_eq!(config.get_metrics_port(), 8888);
        assert_eq!(config.get_postgres_port(), 5433);

        // Unchanged ports should use defaults
        assert_eq!(config.get_redis_port(), 6379);
    }

    #[test]
    fn test_port_config_all_defaults() {
        let config = PortConfig::new();

        // All ports should have valid defaults
        assert!(config.get_api_port() > 0);
        assert!(config.get_admin_port() > 0);
        assert!(config.get_metrics_port() > 0);
        assert!(config.get_health_port() > 0);
        assert!(config.get_dev_port() > 0);
        assert!(config.get_postgres_port() > 0);
        assert!(config.get_redis_port() > 0);
        assert!(config.get_prometheus_port() > 0);
        assert!(config.get_grafana_port() > 0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_port_config_access() {
        // Create two different configurations
        let config1 = Arc::new(PortConfig::new().with_api_port(5000));
        let config2 = Arc::new(PortConfig::new().with_api_port(6000));

        // Spawn concurrent tasks accessing different configs
        let handle1 = {
            let config = Arc::clone(&config1);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_api_port(), 5000);
                }
            })
        };

        let handle2 = {
            let config = Arc::clone(&config2);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_api_port(), 6000);
                }
            })
        };

        handle1.await.unwrap();
        handle2.await.unwrap();
    }

    #[test]
    fn test_port_config_specific_ports() {
        let config = PortConfig::new()
            .with_api_port(8080)
            .with_admin_port(8081)
            .with_metrics_port(9090)
            .with_health_port(8082)
            .with_dev_port(3000)
            .with_postgres_port(5432)
            .with_redis_port(6379)
            .with_prometheus_port(9090)
            .with_grafana_port(3001);

        assert_eq!(config.get_api_port(), 8080);
        assert_eq!(config.get_admin_port(), 8081);
        assert_eq!(config.get_metrics_port(), 9090);
        assert_eq!(config.get_health_port(), 8082);
        assert_eq!(config.get_dev_port(), 3000);
        assert_eq!(config.get_postgres_port(), 5432);
        assert_eq!(config.get_redis_port(), 6379);
        assert_eq!(config.get_prometheus_port(), 9090);
        assert_eq!(config.get_grafana_port(), 3001);
    }
}
