// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(deprecated, reason = "migration to RuntimePortResolver in progress")]

//! Configuration for defaults module
//!
//! This module provides immutable configuration for default values,
//! eliminating runtime `env::var()` calls and enabling concurrent-safe testing.

use std::sync::Arc;

use nestgate_types::{EnvSource, ProcessEnv, env_parsed, env_var_or_default};

/// Configuration for defaults
///
/// This struct captures all environment variables at initialization time,
/// eliminating the need for runtime `env::var()` calls.
#[derive(Debug, Clone)]
/// Configuration for `DefaultsV2`
pub struct DefaultsV2Config {
    // Network settings
    api_port: u16,
    bind_address: String,
    hostname: String,
    ws_port: u16,
    health_port: u16,

    // Database settings
    db_port: u16,

    // Monitoring settings
    metrics_port: u16,
}

/// Shared, thread-safe configuration
pub type SharedDefaultsV2Config = Arc<DefaultsV2Config>;

impl DefaultsV2Config {
    /// Default API port (see [`crate::constants::hardcoding::get_api_port`].)
    pub const DEFAULT_API_PORT: u16 = crate::constants::hardcoding::runtime_fallback_ports::HTTP;
    /// Default bind address
    pub const DEFAULT_BIND_ADDRESS: &'static str =
        crate::constants::hardcoding::addresses::BIND_ALL_IPV4;
    /// Default hostname
    pub const DEFAULT_HOSTNAME: &'static str =
        crate::constants::hardcoding::addresses::LOCALHOST_NAME;
    /// Default WebSocket port
    pub const DEFAULT_WS_PORT: u16 =
        crate::constants::hardcoding::runtime_fallback_ports::WEBSOCKET;
    /// Default health port
    pub const DEFAULT_HEALTH_PORT: u16 =
        crate::constants::hardcoding::runtime_fallback_ports::HEALTH;
    /// Default database port
    pub const DEFAULT_DB_PORT: u16 = crate::constants::hardcoding::runtime_fallback_ports::POSTGRES;
    /// Default metrics port
    pub const DEFAULT_METRICS_PORT: u16 =
        crate::constants::hardcoding::runtime_fallback_ports::METRICS;

    /// Create a new configuration with default values (no env vars)
    #[must_use]
    pub fn new() -> Self {
        Self {
            api_port: Self::DEFAULT_API_PORT,
            bind_address: Self::DEFAULT_BIND_ADDRESS.to_string(),
            hostname: Self::DEFAULT_HOSTNAME.to_string(),
            ws_port: Self::DEFAULT_WS_PORT,
            health_port: Self::DEFAULT_HEALTH_PORT,
            db_port: Self::DEFAULT_DB_PORT,
            metrics_port: Self::DEFAULT_METRICS_PORT,
        }
    }

    /// Create configuration from environment variables
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_source(&ProcessEnv)
    }

    /// Like [`Self::from_env`], but reads from an injectable [`EnvSource`].
    #[must_use]
    pub fn from_env_source(env: &(impl EnvSource + ?Sized)) -> Self {
        let api_port = env_parsed(env, "NESTGATE_API_PORT", Self::DEFAULT_API_PORT);

        let bind_address =
            env_var_or_default(env, "NESTGATE_BIND_ADDRESS", Self::DEFAULT_BIND_ADDRESS);

        let hostname = env_var_or_default(env, "NESTGATE_HOSTNAME", Self::DEFAULT_HOSTNAME);

        let ws_port = env_parsed(env, "NESTGATE_WS_PORT", Self::DEFAULT_WS_PORT);

        let health_port = env_parsed(env, "NESTGATE_HEALTH_PORT", Self::DEFAULT_HEALTH_PORT);

        let db_port = env_parsed(env, "NESTGATE_DB_PORT", Self::DEFAULT_DB_PORT);

        let metrics_port = env_parsed(env, "NESTGATE_METRICS_PORT", Self::DEFAULT_METRICS_PORT);

        Self {
            api_port,
            bind_address,
            hostname,
            ws_port,
            health_port,
            db_port,
            metrics_port,
        }
    }

    // ==================== GETTERS ====================

    /// Get API port
    #[must_use]
    pub const fn api_port(&self) -> u16 {
        self.api_port
    }

    /// Get bind address
    #[must_use]
    pub fn bind_address(&self) -> String {
        self.bind_address.clone()
    }

    /// Get hostname
    #[must_use]
    pub fn hostname(&self) -> String {
        self.hostname.clone()
    }

    /// Get WebSocket port
    #[must_use]
    pub const fn ws_port(&self) -> u16 {
        self.ws_port
    }

    /// Get health port
    #[must_use]
    pub const fn health_port(&self) -> u16 {
        self.health_port
    }

    /// Get database port
    #[must_use]
    pub const fn db_port(&self) -> u16 {
        self.db_port
    }

    /// Get metrics port
    #[must_use]
    pub const fn metrics_port(&self) -> u16 {
        self.metrics_port
    }

    /// Get API URL
    #[must_use]
    pub fn api_url(&self) -> String {
        format!("http://{}:{}", self.hostname, self.api_port)
    }

    /// Get WebSocket URL
    #[must_use]
    pub fn websocket_url(&self) -> String {
        format!("ws://{}:{}", self.hostname, self.ws_port)
    }

    /// Get health URL
    #[must_use]
    pub fn health_url(&self) -> String {
        format!("http://{}:{}/health", self.hostname, self.health_port)
    }

    // ==================== BUILDERS ====================

    /// Builder: Set API port
    #[must_use]
    pub const fn with_api_port(mut self, api_port: u16) -> Self {
        self.api_port = api_port;
        self
    }

    /// Builder: Set bind address
    #[must_use]
    pub fn with_bind_address(mut self, bind_address: String) -> Self {
        self.bind_address = bind_address;
        self
    }

    /// Builder: Set hostname
    #[must_use]
    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = hostname;
        self
    }

    /// Builder: Set WebSocket port
    #[must_use]
    pub const fn with_ws_port(mut self, ws_port: u16) -> Self {
        self.ws_port = ws_port;
        self
    }

    /// Builder: Set health port
    #[must_use]
    pub const fn with_health_port(mut self, health_port: u16) -> Self {
        self.health_port = health_port;
        self
    }

    /// Builder: Set database port
    #[must_use]
    pub const fn with_db_port(mut self, db_port: u16) -> Self {
        self.db_port = db_port;
        self
    }

    /// Builder: Set metrics port
    #[must_use]
    pub const fn with_metrics_port(mut self, metrics_port: u16) -> Self {
        self.metrics_port = metrics_port;
        self
    }
}

impl Default for DefaultsV2Config {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        use crate::constants::hardcoding::runtime_fallback_ports;
        let config = DefaultsV2Config::new();
        assert_eq!(config.api_port(), runtime_fallback_ports::HTTP);
        assert_eq!(config.bind_address(), "0.0.0.0");
        assert_eq!(config.hostname(), "localhost");
        assert_eq!(config.ws_port(), runtime_fallback_ports::WEBSOCKET);
        assert_eq!(config.health_port(), runtime_fallback_ports::HEALTH);
        assert_eq!(config.db_port(), runtime_fallback_ports::POSTGRES);
        assert_eq!(config.metrics_port(), runtime_fallback_ports::METRICS);
    }

    #[test]
    fn test_derived_urls() {
        let config = DefaultsV2Config::new();
        // Verify URLs are properly formed with ServiceDiscoveryConfig
        assert!(config.api_url().starts_with("http://"));
        assert!(config.websocket_url().starts_with("ws://"));
        assert!(config.health_url().contains("/health"));
    }

    #[test]
    fn test_config_builders() {
        let config = DefaultsV2Config::new()
            .with_hostname("example.com".to_string())
            .with_api_port(9000)
            .with_ws_port(9001)
            .with_health_port(9002);

        assert_eq!(config.hostname(), "example.com");
        assert_eq!(config.api_port(), 9000);
        assert_eq!(config.api_url(), "http://example.com:9000");
        assert_eq!(config.websocket_url(), "ws://example.com:9001");
        assert_eq!(config.health_url(), "http://example.com:9002/health");
    }

    #[test]
    fn test_config_arc() {
        let config = Arc::new(DefaultsV2Config::new());
        assert_eq!(
            config.api_port(),
            crate::constants::hardcoding::runtime_fallback_ports::HTTP
        );
        assert_eq!(config.hostname(), "localhost");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            DefaultsV2Config::new()
                .with_hostname("test.local".to_string())
                .with_api_port(7070),
        );

        let mut handles = vec![];
        for _ in 0..100 {
            let config_clone = Arc::clone(&config);
            let handle = tokio::spawn(async move {
                assert_eq!(config_clone.hostname(), "test.local");
                assert_eq!(config_clone.api_port(), 7070);
                assert_eq!(config_clone.api_url(), "http://test.local:7070");
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}
