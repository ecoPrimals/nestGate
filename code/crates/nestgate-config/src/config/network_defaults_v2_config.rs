// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Configuration for network defaults module
//!
//! This module provides immutable configuration for network defaults,
//! eliminating runtime `env::var()` calls and enabling concurrent-safe testing.

use std::sync::Arc;

use nestgate_types::{EnvSource, ProcessEnv, env_parsed, env_var_or_default};

/// Configuration for network defaults
///
/// This struct captures all environment variables at initialization time,
/// eliminating the need for runtime `env::var()` calls.
#[derive(Debug, Clone)]
/// Configuration for `NetworkDefaultsV2`
pub struct NetworkDefaultsV2Config {
    // API Server
    api_host: String,
    api_port: u16,
    api_bind: Option<String>,
    api_url: Option<String>,

    // Metrics
    metrics_port: u16,
    metrics_bind: Option<String>,

    // WebSocket
    ws_port: u16,
    ws_bind: Option<String>,
    ws_url: Option<String>,

    // Health Checks
    health_port: u16,
    health_bind: Option<String>,
    health_url: Option<String>,

    // Storage
    storage_port: u16,
    storage_bind: Option<String>,

    // Timeouts
    connect_timeout_ms: u64,
    request_timeout_ms: u64,
    long_op_timeout_ms: u64,
}

/// Shared, thread-safe configuration
pub type SharedNetworkDefaultsV2Config = Arc<NetworkDefaultsV2Config>;

impl NetworkDefaultsV2Config {
    /// Create a new configuration with default values (no env vars)
    #[must_use]
    pub fn new() -> Self {
        // ✅ MIGRATED: Now uses centralized environment-driven functions
        use crate::constants::hardcoding::addresses;
        use crate::constants::{
            get_admin_port, get_api_port, get_dev_port, get_health_port, get_metrics_port,
        };

        Self {
            // API Server defaults
            api_host: addresses::LOCALHOST_IPV4.to_string(),
            api_port: get_api_port(),
            api_bind: None,
            api_url: None,

            // Metrics defaults
            metrics_port: get_metrics_port(),
            metrics_bind: None,

            // WebSocket defaults
            ws_port: get_admin_port(), // WebSocket uses admin port standard
            ws_bind: None,
            ws_url: None,

            // Health checks defaults
            health_port: get_health_port(),
            health_bind: None,
            health_url: None,

            // Storage defaults
            storage_port: get_dev_port(), // Storage uses dev port standard
            storage_bind: None,

            // Timeout defaults
            connect_timeout_ms: crate::constants::hardcoding::timeouts::CONNECT_MS,
            request_timeout_ms: crate::constants::hardcoding::timeouts::REQUEST_MS,
            long_op_timeout_ms: crate::constants::hardcoding::timeouts::LONG_OPERATION_MS,
        }
    }

    /// Create configuration from an injectable environment source
    #[must_use]
    pub fn from_env_source(env: &(impl EnvSource + ?Sized)) -> Self {
        let api_host_env = env_var_or_default(env, "NESTGATE_API_HOST", "");
        let api_host = if api_host_env.is_empty() {
            crate::constants::hardcoding::addresses::LOCALHOST_IPV4.to_string()
        } else {
            api_host_env
        };

        let api_port = env
            .get("NESTGATE_API_PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(crate::constants::get_api_port);

        Self {
            api_host,
            api_port,
            api_bind: env.get("NESTGATE_API_BIND"),
            api_url: env.get("NESTGATE_API_URL"),

            metrics_port: env
                .get("NESTGATE_METRICS_PORT")
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(crate::constants::get_metrics_port),
            metrics_bind: env.get("NESTGATE_METRICS_BIND"),

            ws_port: env_parsed(
                env,
                "NESTGATE_WS_PORT",
                crate::constants::hardcoding::runtime_fallback_ports::WEBSOCKET,
            ),
            ws_bind: env.get("NESTGATE_WS_BIND"),
            ws_url: env.get("NESTGATE_WS_URL"),

            health_port: env
                .get("NESTGATE_HEALTH_PORT")
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(crate::constants::get_health_port),
            health_bind: env.get("NESTGATE_HEALTH_BIND"),
            health_url: env.get("NESTGATE_HEALTH_URL"),

            storage_port: env_parsed(
                env,
                "NESTGATE_STORAGE_PORT",
                crate::constants::hardcoding::runtime_fallback_ports::STORAGE,
            ),
            storage_bind: env.get("NESTGATE_STORAGE_BIND"),

            connect_timeout_ms: env_parsed(
                env,
                "NESTGATE_CONNECT_TIMEOUT_MS",
                crate::constants::hardcoding::timeouts::CONNECT_MS,
            ),
            request_timeout_ms: env_parsed(
                env,
                "NESTGATE_REQUEST_TIMEOUT_MS",
                crate::constants::hardcoding::timeouts::REQUEST_MS,
            ),
            long_op_timeout_ms: env_parsed(
                env,
                "NESTGATE_LONG_OP_TIMEOUT_MS",
                crate::constants::hardcoding::timeouts::LONG_OPERATION_MS,
            ),
        }
    }

    /// Create configuration from environment variables
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_source(&ProcessEnv)
    }

    // ==================== GETTERS ====================

    /// Get API server host
    #[must_use]
    pub fn api_host(&self) -> String {
        self.api_host.clone()
    }

    /// Get API server port
    #[must_use]
    pub const fn api_port(&self) -> u16 {
        self.api_port
    }

    /// Get API bind address
    #[must_use]
    pub fn api_bind_address(&self) -> String {
        self.api_bind
            .clone()
            .unwrap_or_else(|| format!("{}:{}", self.api_host, self.api_port))
    }

    /// Get API URL
    #[must_use]
    pub fn api_url(&self) -> String {
        self.api_url
            .clone()
            .unwrap_or_else(|| format!("http://{}:{}", self.api_host, self.api_port))
    }

    /// Get metrics port
    #[must_use]
    pub const fn metrics_port(&self) -> u16 {
        self.metrics_port
    }

    /// Get metrics bind address
    #[must_use]
    pub fn metrics_bind_address(&self) -> String {
        self.metrics_bind
            .clone()
            .unwrap_or_else(|| format!("{}:{}", self.api_host, self.metrics_port))
    }

    /// Get WebSocket port
    #[must_use]
    pub const fn websocket_port(&self) -> u16 {
        self.ws_port
    }

    /// Get WebSocket bind address
    #[must_use]
    pub fn websocket_bind_address(&self) -> String {
        self.ws_bind
            .clone()
            .unwrap_or_else(|| format!("{}:{}", self.api_host, self.ws_port))
    }

    /// Get WebSocket URL
    #[must_use]
    pub fn websocket_url(&self) -> String {
        self.ws_url
            .clone()
            .unwrap_or_else(|| format!("ws://{}:{}", self.api_host, self.ws_port))
    }

    /// Get health check port
    #[must_use]
    pub const fn health_port(&self) -> u16 {
        self.health_port
    }

    /// Get health check bind address
    #[must_use]
    pub fn health_bind_address(&self) -> String {
        self.health_bind
            .clone()
            .unwrap_or_else(|| format!("{}:{}", self.api_host, self.health_port))
    }

    /// Get health check URL
    #[must_use]
    pub fn health_url(&self) -> String {
        self.health_url
            .clone()
            .unwrap_or_else(|| format!("http://{}:{}/health", self.api_host, self.health_port))
    }

    /// Get storage port
    #[must_use]
    pub const fn storage_port(&self) -> u16 {
        self.storage_port
    }

    /// Get storage bind address
    #[must_use]
    pub fn storage_bind_address(&self) -> String {
        self.storage_bind
            .clone()
            .unwrap_or_else(|| format!("{}:{}", self.api_host, self.storage_port))
    }

    /// Get connection timeout in milliseconds
    #[must_use]
    pub const fn connect_timeout_ms(&self) -> u64 {
        self.connect_timeout_ms
    }

    /// Get request timeout in milliseconds
    #[must_use]
    pub const fn request_timeout_ms(&self) -> u64 {
        self.request_timeout_ms
    }

    /// Get long operation timeout in milliseconds
    #[must_use]
    pub const fn long_operation_timeout_ms(&self) -> u64 {
        self.long_op_timeout_ms
    }

    // ==================== BUILDERS ====================

    /// Builder: Set API host
    #[must_use]
    pub fn with_api_host(mut self, host: String) -> Self {
        self.api_host = host;
        self
    }

    /// Builder: Set API port
    #[must_use]
    pub const fn with_api_port(mut self, port: u16) -> Self {
        self.api_port = port;
        self
    }

    /// Builder: Set metrics port
    #[must_use]
    pub const fn with_metrics_port(mut self, port: u16) -> Self {
        self.metrics_port = port;
        self
    }

    /// Builder: Set WebSocket port
    #[must_use]
    pub const fn with_websocket_port(mut self, port: u16) -> Self {
        self.ws_port = port;
        self
    }

    /// Builder: Set health port
    #[must_use]
    pub const fn with_health_port(mut self, port: u16) -> Self {
        self.health_port = port;
        self
    }

    /// Builder: Set storage port
    #[must_use]
    pub const fn with_storage_port(mut self, port: u16) -> Self {
        self.storage_port = port;
        self
    }

    /// Builder: Set connection timeout
    #[must_use]
    pub const fn with_connect_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.connect_timeout_ms = timeout_ms;
        self
    }

    /// Builder: Set request timeout
    #[must_use]
    pub const fn with_request_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.request_timeout_ms = timeout_ms;
        self
    }

    /// Builder: Set long operation timeout
    #[must_use]
    pub const fn with_long_operation_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.long_op_timeout_ms = timeout_ms;
        self
    }
}

impl Default for NetworkDefaultsV2Config {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

    #[test]
    fn test_from_env_source_map_env() {
        let env = MapEnv::from([
            ("NESTGATE_API_HOST", "10.0.0.1"),
            ("NESTGATE_API_PORT", "9000"),
            ("NESTGATE_METRICS_PORT", "9100"),
        ]);
        let config = NetworkDefaultsV2Config::from_env_source(&env);
        assert_eq!(config.api_host(), "10.0.0.1");
        assert_eq!(config.api_port(), 9000);
        assert_eq!(config.metrics_port(), 9100);
    }

    #[test]
    fn test_config_defaults() {
        let config = NetworkDefaultsV2Config::new();
        assert_eq!(config.api_host(), "127.0.0.1");
        assert_eq!(config.api_port(), 8080);
        assert_eq!(config.api_bind_address(), "127.0.0.1:8080");
        assert_eq!(config.api_url(), "http://127.0.0.1:8080");
        assert_eq!(config.metrics_port(), 9090);
        assert_eq!(config.websocket_port(), 8081); // Uses ADMIN_PORT (8081)
        assert_eq!(config.health_port(), 8082); // Uses HEALTH_PORT (8082)
        assert_eq!(config.storage_port(), 3000); // Uses DEV_PORT (3000)
        assert_eq!(config.connect_timeout_ms(), 5000);
        assert_eq!(config.request_timeout_ms(), 30000);
        assert_eq!(config.long_operation_timeout_ms(), 300_000);
    }

    #[test]
    fn test_config_builders() {
        let config = NetworkDefaultsV2Config::new()
            .with_api_host("0.0.0.0".to_string())
            .with_api_port(9000)
            .with_metrics_port(9091)
            .with_websocket_port(9092)
            .with_health_port(9093)
            .with_storage_port(9094)
            .with_connect_timeout_ms(10000)
            .with_request_timeout_ms(60000)
            .with_long_operation_timeout_ms(600_000);

        assert_eq!(config.api_host(), "0.0.0.0");
        assert_eq!(config.api_port(), 9000);
        assert_eq!(config.api_bind_address(), "0.0.0.0:9000");
        assert_eq!(config.api_url(), "http://0.0.0.0:9000");
        assert_eq!(config.metrics_port(), 9091);
        assert_eq!(config.websocket_port(), 9092);
        assert_eq!(config.health_port(), 9093);
        assert_eq!(config.storage_port(), 9094);
        assert_eq!(config.connect_timeout_ms(), 10000);
        assert_eq!(config.request_timeout_ms(), 60000);
        assert_eq!(config.long_operation_timeout_ms(), 600_000);
    }

    #[test]
    fn test_config_arc() {
        let config = Arc::new(NetworkDefaultsV2Config::new());
        assert_eq!(config.api_host(), "127.0.0.1");
        assert_eq!(config.api_port(), 8080);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            NetworkDefaultsV2Config::new()
                .with_api_host("192.168.1.100".to_string())
                .with_api_port(7070),
        );

        let mut handles = vec![];
        for _ in 0..100 {
            let config_clone = Arc::clone(&config);
            let handle = tokio::spawn(async move {
                assert_eq!(config_clone.api_host(), "192.168.1.100");
                assert_eq!(config_clone.api_port(), 7070);
                assert_eq!(config_clone.api_bind_address(), "192.168.1.100:7070");
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}
