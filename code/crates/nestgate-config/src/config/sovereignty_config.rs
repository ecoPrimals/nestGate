// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::env;
use std::sync::Arc;

/// Configuration for sovereignty-compliant settings, providing environment variable
/// overrides and safe fallbacks.
///
/// This config eliminates hardcoded infrastructure assumptions, ensuring users
/// maintain full control over their systems.
#[derive(Debug, Clone)]
/// Configuration for `SovereigntyRuntime`
pub struct SovereigntyRuntimeConfig {
    api_endpoint: Option<String>,
    api_host: Option<String>,
    api_port: u16,
    bind_address: String,
    ws_endpoint: Option<String>,
    discovery_endpoint: Option<String>,
    orchestration_endpoint: Option<String>,
    test_endpoint: Option<String>,
}

/// Type alias for Sharedsovereigntyruntimeconfig
pub type SharedSovereigntyRuntimeConfig = Arc<SovereigntyRuntimeConfig>;

impl SovereigntyRuntimeConfig {
    /// Creates a new `SovereigntyRuntimeConfig` with default values from environment.
    ///
    /// Environment variables:
    /// - `NESTGATE_API_PORT`: API port (default: from `EnvironmentConfig`)
    /// - `NESTGATE_BIND_ADDRESS`: Bind address (default: from `EnvironmentConfig`)
    #[must_use]
    pub fn new() -> Self {
        use crate::config::environment::EnvironmentConfig;

        // Load environment configuration with proper fallbacks - Modern idiomatic pattern
        let env_config = EnvironmentConfig::from_env().unwrap_or_default();

        Self {
            api_endpoint: None,
            api_host: None,
            api_port: env_config.network.port.get(),
            bind_address: env_config.network.host,
            ws_endpoint: None,
            discovery_endpoint: None,
            orchestration_endpoint: None,
            test_endpoint: None,
        }
    }

    /// Creates a new `SovereigntyRuntimeConfig` by reading environment variables
    /// or using default values.
    #[must_use]
    #[expect(
        deprecated,
        reason = "NESTGATE_API_PORT fallback uses runtime_fallback_ports until canonical env resolution"
    )]
    pub fn from_env() -> Self {
        let api_endpoint = env::var("NESTGATE_API_ENDPOINT").ok();
        let api_host = env::var("NESTGATE_API_HOST").ok();
        let api_port = env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(crate::constants::hardcoding::runtime_fallback_ports::HTTP);
        let bind_address = env::var("NESTGATE_BIND_ADDRESS")
            .unwrap_or_else(|_| crate::constants::LOCALHOST.to_string()); // Idiomatic: computed default
        let ws_endpoint = env::var("NESTGATE_WS_ENDPOINT").ok();
        let discovery_endpoint = env::var("NESTGATE_DISCOVERY_ENDPOINT").ok();
        let orchestration_endpoint = env::var("NESTGATE_ORCHESTRATION_ENDPOINT").ok();
        let test_endpoint = env::var("NESTGATE_TEST_ENDPOINT").ok();

        Self {
            api_endpoint,
            api_host,
            api_port,
            bind_address,
            ws_endpoint,
            discovery_endpoint,
            orchestration_endpoint,
            test_endpoint,
        }
    }

    // Builder methods for testing

    /// Builder method to set the API endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The full API endpoint URL (e.g., "<http://localhost:8080>")
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use nestgate_core::config::sovereignty_config::SovereigntyRuntimeConfig;
    ///
    /// let config = SovereigntyRuntimeConfig::default()
    ///     .with_api_endpoint("http://api.example.com".to_string());
    /// ```
    #[must_use]
    pub fn with_api_endpoint(mut self, endpoint: String) -> Self {
        self.api_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Api Host
    #[must_use]
    pub fn with_api_host(mut self, host: String) -> Self {
        self.api_host = Some(host);
        self
    }

    /// Builder method to set Api Port
    #[must_use]
    pub const fn with_api_port(mut self, port: u16) -> Self {
        self.api_port = port;
        self
    }

    /// Builder method to set Bind Address
    #[must_use]
    pub fn with_bind_address(mut self, address: String) -> Self {
        self.bind_address = address;
        self
    }

    /// Builder method to set Ws Endpoint
    #[must_use]
    pub fn with_ws_endpoint(mut self, endpoint: String) -> Self {
        self.ws_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Discovery Endpoint
    #[must_use]
    pub fn with_discovery_endpoint(mut self, endpoint: String) -> Self {
        self.discovery_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Orchestration Endpoint
    #[must_use]
    pub fn with_orchestration_endpoint(mut self, endpoint: String) -> Self {
        self.orchestration_endpoint = Some(endpoint);
        self
    }

    /// Builder method to set Test Endpoint
    #[must_use]
    pub fn with_test_endpoint(mut self, endpoint: String) -> Self {
        self.test_endpoint = Some(endpoint);
        self
    }

    // Getter methods

    /// Get API endpoint with fallback logic
    #[must_use]
    pub fn api_endpoint(&self) -> String {
        if let Some(endpoint) = &self.api_endpoint {
            return endpoint.clone();
        }

        if let Some(host) = &self.api_host {
            return format!("http://{}:{}", host, self.api_port);
        }

        format!("http://{}:{}", self.bind_address, self.api_port)
    }

    /// Api Port
    #[must_use]
    pub const fn api_port(&self) -> u16 {
        self.api_port
    }

    /// Bind Address
    #[must_use]
    pub fn bind_address(&self) -> String {
        self.bind_address.clone()
    }

    /// Get WebSocket endpoint with fallback logic
    #[must_use]
    pub fn websocket_endpoint(&self) -> String {
        if let Some(endpoint) = &self.ws_endpoint {
            return endpoint.clone();
        }

        format!("ws://{}:{}/ws", self.bind_address, self.api_port)
    }

    /// Get discovery endpoint with fallback logic
    #[must_use]
    pub fn discovery_endpoint(&self) -> String {
        if let Some(endpoint) = &self.discovery_endpoint {
            return endpoint.clone();
        }

        format!("{}/discovery", self.api_endpoint())
    }

    /// Get orchestration endpoint with fallback logic
    #[must_use]
    pub fn orchestration_endpoint(&self) -> String {
        if let Some(endpoint) = &self.orchestration_endpoint {
            return endpoint.clone();
        }

        self.api_endpoint()
    }

    /// Get test endpoint with fallback logic
    #[must_use]
    pub fn test_endpoint(&self) -> String {
        if let Some(endpoint) = &self.test_endpoint {
            return endpoint.clone();
        }

        self.api_endpoint()
    }
}

impl Default for SovereigntyRuntimeConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::hardcoding::runtime_fallback_ports;

    #[test]
    #[expect(
        deprecated,
        reason = "default API port aligns with runtime_fallback_ports when env matches"
    )]
    fn test_default_config() {
        let config = SovereigntyRuntimeConfig::new();
        assert_eq!(config.api_port(), runtime_fallback_ports::HTTP);
        assert_eq!(config.bind_address(), crate::constants::LOCALHOST);
        assert!(config.api_endpoint().contains("127.0.0.1"));
        assert!(config.websocket_endpoint().starts_with("ws://"));
    }

    #[test]
    #[expect(
        deprecated,
        reason = "builder tests use runtime_fallback_ports for numeric parity"
    )]
    fn test_builder_pattern() {
        let config = SovereigntyRuntimeConfig::new()
            .with_api_port(runtime_fallback_ports::METRICS)
            .with_bind_address("0.0.0.0".to_string())
            .with_api_endpoint(format!("http://custom:{}", runtime_fallback_ports::METRICS));

        assert_eq!(config.api_port(), runtime_fallback_ports::METRICS);
        assert_eq!(config.bind_address(), "0.0.0.0");
        assert_eq!(
            config.api_endpoint(),
            format!("http://custom:{}", runtime_fallback_ports::METRICS)
        );
    }

    #[test]
    fn test_fallback_logic() {
        let config = SovereigntyRuntimeConfig::new()
            .with_api_host("myhost".to_string())
            .with_api_port(8888);

        assert_eq!(config.api_endpoint(), "http://myhost:8888");
        assert_eq!(config.websocket_endpoint(), "ws://127.0.0.1:8888/ws");
        assert_eq!(config.discovery_endpoint(), "http://myhost:8888/discovery");
    }

    #[test]
    fn test_orchestration_endpoint_fallback() {
        let config = SovereigntyRuntimeConfig::new();
        assert_eq!(config.orchestration_endpoint(), config.api_endpoint());
    }

    #[test]
    fn test_test_endpoint_fallback() {
        let config = SovereigntyRuntimeConfig::new();
        assert_eq!(config.test_endpoint(), config.api_endpoint());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(SovereigntyRuntimeConfig::new());
        let handles: Vec<_> = (0..100)
            .map(|_| {
                let cfg = config.clone();
                tokio::spawn(async move {
                    let _ = cfg.api_endpoint();
                    let _ = cfg.websocket_endpoint();
                    let _ = cfg.discovery_endpoint();
                    let _ = cfg.orchestration_endpoint();
                    let _ = cfg.test_endpoint();
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task should complete successfully");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    #[expect(
        deprecated,
        reason = "concurrent test uses runtime_fallback_ports for numeric parity"
    )]
    async fn test_concurrent_different_configs() {
        let config1 = Arc::new(
            SovereigntyRuntimeConfig::new()
                .with_api_port(runtime_fallback_ports::HTTP)
                .with_bind_address("127.0.0.1".to_string()),
        );
        let config2 = Arc::new(
            SovereigntyRuntimeConfig::new()
                .with_api_port(runtime_fallback_ports::METRICS)
                .with_bind_address("0.0.0.0".to_string()),
        );

        let handle1 = tokio::spawn({
            let cfg = config1.clone();
            async move { cfg.api_endpoint() }
        });
        let handle2 = tokio::spawn({
            let cfg = config2.clone();
            async move { cfg.api_endpoint() }
        });

        let endpoint1 = handle1.await.unwrap();
        let endpoint2 = handle2.await.unwrap();

        assert!(endpoint1.contains(&format!("127.0.0.1:{}", runtime_fallback_ports::HTTP)));
        assert!(endpoint2.contains(&format!("0.0.0.0:{}", runtime_fallback_ports::METRICS)));
        assert_ne!(endpoint1, endpoint2);
    }
}
