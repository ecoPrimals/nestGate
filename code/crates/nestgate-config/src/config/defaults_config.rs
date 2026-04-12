// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::sync::Arc;

use nestgate_types::{EnvSource, ProcessEnv};

/// Thread-safe configuration for network port defaults
/// Captures environment variables at initialization to prevent race conditions
#[derive(Debug, Clone)]
/// Configuration for `NetworkDefaults`
pub struct NetworkDefaultsConfig {
    // Port configurations
    api_port: Option<u16>,
    websocket_port: Option<u16>,
    http_port: Option<u16>,
    nas_http_port: Option<u16>,
    dev_server_port: Option<u16>,
    metrics_port: Option<u16>,
    health_port: Option<u16>,
    orchestrator_port: Option<u16>,

    // Address configurations
    bind_address: Option<String>,
    dev_bind_address: Option<String>,
    hostname: Option<String>,
    external_hostname: Option<String>,

    // URL configurations
    websocket_base_url: Option<String>,
    api_base_url: Option<String>,

    // Timeout configurations
    connection_timeout_ms: Option<u64>,
    request_timeout_ms: Option<u64>,
}

/// Shared immutable reference to `NetworkDefaultsConfig`
pub type SharedNetworkDefaultsConfig = Arc<NetworkDefaultsConfig>;

impl NetworkDefaultsConfig {
    /// Create a new empty configuration (all values None, will use hardcoded defaults)
    #[must_use]
    pub const fn new() -> Self {
        Self {
            api_port: None,
            websocket_port: None,
            http_port: None,
            nas_http_port: None,
            dev_server_port: None,
            metrics_port: None,
            health_port: None,
            orchestrator_port: None,
            bind_address: None,
            dev_bind_address: None,
            hostname: None,
            external_hostname: None,
            websocket_base_url: None,
            api_base_url: None,
            connection_timeout_ms: None,
            request_timeout_ms: None,
        }
    }

    /// Create configuration from an injectable environment source
    #[must_use]
    pub fn from_env_source(env: &(impl EnvSource + ?Sized)) -> Self {
        Self {
            api_port: env.get("NESTGATE_API_PORT").and_then(|s| s.parse().ok()),
            websocket_port: env
                .get("NESTGATE_WEBSOCKET_PORT")
                .and_then(|s| s.parse().ok()),
            http_port: env.get("NESTGATE_HTTP_PORT").and_then(|s| s.parse().ok()),
            nas_http_port: env
                .get("NESTGATE_NAS_HTTP_PORT")
                .and_then(|s| s.parse().ok()),
            dev_server_port: env
                .get("NESTGATE_DEV_SERVER_PORT")
                .and_then(|s| s.parse().ok()),
            metrics_port: env
                .get("NESTGATE_METRICS_PORT")
                .and_then(|s| s.parse().ok()),
            health_port: env.get("NESTGATE_HEALTH_PORT").and_then(|s| s.parse().ok()),
            orchestrator_port: env
                .get("NESTGATE_ORCHESTRATOR_PORT")
                .and_then(|s| s.parse().ok()),
            bind_address: env.get("NESTGATE_BIND_ADDRESS"),
            dev_bind_address: env.get("NESTGATE_DEV_BIND_ADDRESS"),
            hostname: env.get("NESTGATE_HOSTNAME"),
            external_hostname: env.get("NESTGATE_EXTERNAL_HOSTNAME"),
            websocket_base_url: env.get("NESTGATE_WS_BASE_URL"),
            api_base_url: env.get("NESTGATE_API_BASE_URL"),
            connection_timeout_ms: env
                .get("NESTGATE_CONNECTION_TIMEOUT_MS")
                .and_then(|s| s.parse().ok()),
            request_timeout_ms: env
                .get("NESTGATE_REQUEST_TIMEOUT_MS")
                .and_then(|s| s.parse().ok()),
        }
    }

    /// Create configuration from current environment variables
    /// This captures env vars at initialization time, making it thread-safe
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_source(&ProcessEnv)
    }

    // Port getters with fallback to established defaults

    /// Gets Api Port
    #[must_use]
    pub fn get_api_port(&self) -> u16 {
        self.api_port.unwrap_or(3000) // API default (Node.js convention)
    }

    /// Gets Websocket Port
    #[must_use]
    pub fn get_websocket_port(&self) -> u16 {
        self.websocket_port.unwrap_or(8082) // WebSocket default
    }

    /// Gets Http Port
    #[must_use]
    pub fn get_http_port(&self) -> u16 {
        self.http_port.unwrap_or(8080) // HTTP default
    }

    /// Gets Nas Http Port
    #[must_use]
    pub fn get_nas_http_port(&self) -> u16 {
        self.nas_http_port.unwrap_or(8080) // HTTP default for NAS
    }

    /// Gets Dev Server Port
    #[must_use]
    pub fn get_dev_server_port(&self) -> u16 {
        self.dev_server_port.unwrap_or(3000) // Development server default
    }

    /// Gets Metrics Port
    #[must_use]
    pub fn get_metrics_port(&self) -> u16 {
        self.metrics_port.unwrap_or(9090) // Prometheus default
    }

    /// Gets Health Port
    #[must_use]
    pub fn get_health_port(&self) -> u16 {
        self.health_port.unwrap_or(8081) // Health check default
    }

    /// Gets Orchestrator Port
    #[must_use]
    pub fn get_orchestrator_port(&self) -> u16 {
        self.orchestrator_port.unwrap_or(8090) // Orchestrator default
    }

    // Address getters with fallback to defaults

    /// Gets Bind Address
    #[must_use]
    pub fn get_bind_address(&self) -> String {
        self.bind_address
            .clone()
            .unwrap_or_else(|| crate::constants::network_defaults::LOCALHOST_IPV4.to_string())
    }

    /// Gets Development Bind Address
    #[must_use]
    pub fn get_development_bind_address(&self) -> String {
        self.dev_bind_address
            .clone()
            .unwrap_or_else(|| crate::constants::network_defaults::BIND_ALL_IPV4.to_string())
    }

    /// Gets Hostname
    #[must_use]
    pub fn get_hostname(&self) -> String {
        self.hostname
            .clone()
            .unwrap_or_else(|| crate::constants::network_defaults::LOCALHOST_NAME.to_string())
    }

    /// Gets External Hostname
    #[must_use]
    pub fn get_external_hostname(&self) -> String {
        self.external_hostname
            .clone()
            .unwrap_or_else(|| crate::constants::network_defaults::LOCALHOST_NAME.to_string())
    }

    // URL getters with dynamic construction if not provided

    /// Gets Websocket Base Url
    #[must_use]
    pub fn get_websocket_base_url(&self) -> String {
        self.websocket_base_url.clone().unwrap_or_else(|| {
            let discovery_config =
                crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "ws://{}:{}",
                discovery_config.discovery_host,
                self.get_websocket_port()
            )
        })
    }

    /// Gets Api Base Url
    #[must_use]
    pub fn get_api_base_url(&self) -> String {
        self.api_base_url.clone().unwrap_or_else(|| {
            let discovery_config =
                crate::config::discovery_config::ServiceDiscoveryConfig::default();
            discovery_config.build_endpoint(self.get_api_port())
        })
    }

    // Timeout getters

    /// Gets Connection Timeout Ms
    #[must_use]
    pub fn get_connection_timeout_ms(&self) -> u64 {
        self.connection_timeout_ms.unwrap_or(3000)
    }

    /// Gets Request Timeout Ms
    #[must_use]
    pub fn get_request_timeout_ms(&self) -> u64 {
        self.request_timeout_ms.unwrap_or(30000)
    }

    // Builder methods for tests

    /// Builder method to set Api Port
    #[must_use]
    pub const fn with_api_port(mut self, port: u16) -> Self {
        self.api_port = Some(port);
        self
    }

    /// Builder method to set Websocket Port
    #[must_use]
    pub const fn with_websocket_port(mut self, port: u16) -> Self {
        self.websocket_port = Some(port);
        self
    }

    /// Builder method to set Http Port
    #[must_use]
    pub const fn with_http_port(mut self, port: u16) -> Self {
        self.http_port = Some(port);
        self
    }

    /// Builder method to set Bind Address
    #[must_use]
    pub fn with_bind_address(mut self, address: String) -> Self {
        self.bind_address = Some(address);
        self
    }

    /// Builder method to set Hostname
    #[must_use]
    pub fn with_hostname(mut self, hostname: String) -> Self {
        self.hostname = Some(hostname);
        self
    }

    /// Builder method to set Connection Timeout Ms
    #[must_use]
    pub const fn with_connection_timeout_ms(mut self, timeout: u64) -> Self {
        self.connection_timeout_ms = Some(timeout);
        self
    }
}

impl Default for NetworkDefaultsConfig {
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
            ("NESTGATE_API_PORT", "4000"),
            ("NESTGATE_HOSTNAME", "test-host"),
        ]);
        let config = NetworkDefaultsConfig::from_env_source(&env);
        assert_eq!(config.get_api_port(), 4000);
        assert_eq!(config.get_hostname(), "test-host");
    }

    #[test]
    fn test_defaults_config_new() {
        let config = NetworkDefaultsConfig::new();

        // Should use hardcoded defaults
        assert_eq!(config.get_api_port(), 3000); // API_DEFAULT = 3000
        assert_eq!(config.get_websocket_port(), 8082); // WEBSOCKET_DEFAULT = 8082
        assert_eq!(config.get_http_port(), 8080); // HTTP_DEFAULT = 8080
        assert_eq!(config.get_bind_address(), "127.0.0.1");
        assert_eq!(config.get_hostname(), "localhost");
    }

    #[test]
    fn test_defaults_config_builder() {
        let config = NetworkDefaultsConfig::new()
            .with_api_port(9999)
            .with_websocket_port(8888)
            .with_hostname("custom-host".to_string());

        assert_eq!(config.get_api_port(), 9999);
        assert_eq!(config.get_websocket_port(), 8888);
        assert_eq!(config.get_hostname(), "custom-host");
    }

    #[test]
    fn test_defaults_config_url_generation() {
        let config = NetworkDefaultsConfig::new()
            .with_api_port(7777)
            .with_websocket_port(6666);

        assert_eq!(config.get_api_base_url(), "http://127.0.0.1:7777");
        assert_eq!(config.get_websocket_base_url(), "ws://127.0.0.1:6666");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_config_access() {
        // Create two different configurations
        let config1 = Arc::new(NetworkDefaultsConfig::new().with_api_port(5000));
        let config2 = Arc::new(NetworkDefaultsConfig::new().with_api_port(6000));

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
    fn test_defaults_config_timeouts() {
        let config = NetworkDefaultsConfig::new().with_connection_timeout_ms(5000);

        assert_eq!(config.get_connection_timeout_ms(), 5000);
        assert_eq!(config.get_request_timeout_ms(), 30000); // default
    }
}
