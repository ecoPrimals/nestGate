//! Configuration for sovereignty helpers module
//!
//! This module provides immutable configuration for sovereignty-compliant helpers,
//! eliminating runtime `env::var()` calls and enabling concurrent-safe testing.

use std::sync::Arc;

use crate::error::utilities::safe_env_var_or_default;

/// Configuration for sovereignty helpers
///
/// This struct captures all environment variables at initialization time,
/// eliminating the need for runtime `env::var()` calls.
#[derive(Debug, Clone)]
/// Configuration for SovereigntyHelpers
pub struct SovereigntyHelpersConfig {
    // API configuration
    api_host: String,
    api_port: String,
    api_scheme: String,

    // WebSocket configuration
    ws_scheme: String,

    // Discovery configuration
    discovery_path: String,

    // Network configuration
    bind_address: String,

    // Database configuration
    db_host: String,
    db_port: String,

    // Development configuration
    dev_host: String,
    dev_port: String,
}

/// Shared, thread-safe configuration
pub type SharedSovereigntyHelpersConfig = Arc<SovereigntyHelpersConfig>;

impl SovereigntyHelpersConfig {
    /// Default API host
    pub const DEFAULT_API_HOST: &'static str = "127.0.0.1";
    /// Default API port
    pub const DEFAULT_API_PORT: &'static str = "8080";
    /// Default API scheme
    pub const DEFAULT_API_SCHEME: &'static str = "http";
    /// Default WebSocket scheme
    pub const DEFAULT_WS_SCHEME: &'static str = "ws";
    /// Default discovery path
    pub const DEFAULT_DISCOVERY_PATH: &'static str = "/discovery";
    /// Default bind address
    pub const DEFAULT_BIND_ADDRESS: &'static str = "0.0.0.0";
    /// Default database host
    pub const DEFAULT_DB_HOST: &'static str = "127.0.0.1";
    /// Default database port
    pub const DEFAULT_DB_PORT: &'static str = "5432";
    /// Default development host
    pub const DEFAULT_DEV_HOST: &'static str = "127.0.0.1";
    /// Default development port
    pub const DEFAULT_DEV_PORT: &'static str = "3000";

    /// Create a new configuration with default values (no env vars)
    #[must_use]
    pub fn new() -> Self {
        Self {
            api_host: Self::DEFAULT_API_HOST.to_string(),
            api_port: Self::DEFAULT_API_PORT.to_string(),
            api_scheme: Self::DEFAULT_API_SCHEME.to_string(),
            ws_scheme: Self::DEFAULT_WS_SCHEME.to_string(),
            discovery_path: Self::DEFAULT_DISCOVERY_PATH.to_string(),
            bind_address: Self::DEFAULT_BIND_ADDRESS.to_string(),
            db_host: Self::DEFAULT_DB_HOST.to_string(),
            db_port: Self::DEFAULT_DB_PORT.to_string(),
            dev_host: Self::DEFAULT_DEV_HOST.to_string(),
            dev_port: Self::DEFAULT_DEV_PORT.to_string(),
        }
    }

    /// Create configuration from environment variables
    #[must_use]
    pub fn from_env() -> Self {
        let api_host =
            safe_env_var_or_default("NESTGATE_API_HOST", Self::DEFAULT_API_HOST).to_string();

        let api_port =
            safe_env_var_or_default("NESTGATE_API_PORT", Self::DEFAULT_API_PORT).to_string();

        let api_scheme =
            safe_env_var_or_default("NESTGATE_API_SCHEME", Self::DEFAULT_API_SCHEME).to_string();

        let ws_scheme =
            safe_env_var_or_default("NESTGATE_WS_SCHEME", Self::DEFAULT_WS_SCHEME).to_string();

        let discovery_path =
            safe_env_var_or_default("NESTGATE_DISCOVERY_PATH", Self::DEFAULT_DISCOVERY_PATH)
                .to_string();

        let bind_address =
            safe_env_var_or_default("NESTGATE_BIND_ADDRESS", Self::DEFAULT_BIND_ADDRESS)
                .to_string();

        let db_host =
            safe_env_var_or_default("NESTGATE_DB_HOST", Self::DEFAULT_DB_HOST).to_string();

        let db_port =
            safe_env_var_or_default("NESTGATE_DB_PORT", Self::DEFAULT_DB_PORT).to_string();

        let dev_host =
            safe_env_var_or_default("NESTGATE_DEV_HOST", Self::DEFAULT_DEV_HOST).to_string();

        let dev_port =
            safe_env_var_or_default("NESTGATE_DEV_PORT", Self::DEFAULT_DEV_PORT).to_string();

        Self {
            api_host,
            api_port,
            api_scheme,
            ws_scheme,
            discovery_path,
            bind_address,
            db_host,
            db_port,
            dev_host,
            dev_port,
        }
    }

    // ==================== GETTERS ====================

    /// Get API host
    #[must_use]
    pub fn api_host(&self) -> String {
        self.api_host.clone()
    }

    /// Get API port (as string)
    #[must_use]
    pub fn api_port_string(&self) -> String {
        self.api_port.clone()
    }

    /// Get API port (as u16)
    #[must_use]
    pub fn api_port(&self) -> u16 {
        self.api_port.parse().unwrap_or(8080)
    }

    /// Get API scheme
    #[must_use]
    pub fn api_scheme(&self) -> String {
        self.api_scheme.clone()
    }

    /// Get WebSocket scheme
    #[must_use]
    pub fn ws_scheme(&self) -> String {
        self.ws_scheme.clone()
    }

    /// Get discovery path
    #[must_use]
    pub fn discovery_path(&self) -> String {
        self.discovery_path.clone()
    }

    /// Get bind address
    #[must_use]
    pub fn bind_address(&self) -> String {
        self.bind_address.clone()
    }

    /// Get database host
    #[must_use]
    pub fn db_host(&self) -> String {
        self.db_host.clone()
    }

    /// Get database port
    #[must_use]
    pub fn db_port(&self) -> String {
        self.db_port.clone()
    }

    /// Get development host
    #[must_use]
    pub fn dev_host(&self) -> String {
        self.dev_host.clone()
    }

    /// Get development port
    #[must_use]
    pub fn dev_port(&self) -> String {
        self.dev_port.clone()
    }

    /// Get API endpoint (host:port)
    #[must_use]
    pub fn api_endpoint(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }

    /// Get HTTP API endpoint (scheme://host:port)
    #[must_use]
    pub fn http_api_endpoint(&self) -> String {
        format!("{}://{}", self.api_scheme, self.api_endpoint())
    }

    /// Get WebSocket endpoint (ws://host:port/ws)
    #[must_use]
    pub fn websocket_endpoint(&self) -> String {
        format!("{}://{}/ws", self.ws_scheme, self.api_endpoint())
    }

    /// Get discovery endpoint (http://host:port/discovery)
    #[must_use]
    pub fn discovery_endpoint(&self) -> String {
        format!("{}{}", self.http_api_endpoint(), self.discovery_path)
    }

    /// Get database endpoint (host:port)
    #[must_use]
    pub fn database_endpoint(&self) -> String {
        format!("{}:{}", self.db_host, self.db_port)
    }

    /// Get development endpoint (host:port)
    #[must_use]
    pub fn dev_endpoint(&self) -> String {
        format!("{}:{}", self.dev_host, self.dev_port)
    }

    // ==================== BUILDERS ====================

    /// Builder: Set API host
    #[must_use]
    pub fn with_api_host(mut self, api_host: String) -> Self {
        self.api_host = api_host;
        self
    }

    /// Builder: Set API port
    #[must_use]
    pub fn with_api_port(mut self, api_port: String) -> Self {
        self.api_port = api_port;
        self
    }

    /// Builder: Set API scheme
    #[must_use]
    pub fn with_api_scheme(mut self, api_scheme: String) -> Self {
        self.api_scheme = api_scheme;
        self
    }

    /// Builder: Set WebSocket scheme
    #[must_use]
    pub fn with_ws_scheme(mut self, ws_scheme: String) -> Self {
        self.ws_scheme = ws_scheme;
        self
    }

    /// Builder: Set discovery path
    #[must_use]
    pub fn with_discovery_path(mut self, discovery_path: String) -> Self {
        self.discovery_path = discovery_path;
        self
    }

    /// Builder: Set bind address
    #[must_use]
    pub fn with_bind_address(mut self, bind_address: String) -> Self {
        self.bind_address = bind_address;
        self
    }

    /// Builder: Set database host
    #[must_use]
    pub fn with_db_host(mut self, db_host: String) -> Self {
        self.db_host = db_host;
        self
    }

    /// Builder: Set database port
    #[must_use]
    pub fn with_db_port(mut self, db_port: String) -> Self {
        self.db_port = db_port;
        self
    }

    /// Builder: Set development host
    #[must_use]
    pub fn with_dev_host(mut self, dev_host: String) -> Self {
        self.dev_host = dev_host;
        self
    }

    /// Builder: Set development port
    #[must_use]
    pub fn with_dev_port(mut self, dev_port: String) -> Self {
        self.dev_port = dev_port;
        self
    }
}

impl Default for SovereigntyHelpersConfig {
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
        let config = SovereigntyHelpersConfig::new();
        assert_eq!(config.api_host(), "127.0.0.1");
        assert_eq!(config.api_port(), 8080);
        assert_eq!(config.api_scheme(), "http");
        assert_eq!(config.ws_scheme(), "ws");
        assert_eq!(config.discovery_path(), "/discovery");
        assert_eq!(config.bind_address(), "0.0.0.0");
        assert_eq!(config.db_host(), "127.0.0.1");
        assert_eq!(config.db_port(), "5432");
        assert_eq!(config.dev_host(), "127.0.0.1");
        assert_eq!(config.dev_port(), "3000");
    }

    #[test]
    fn test_derived_endpoints() {
        let config = SovereigntyHelpersConfig::new();
        assert_eq!(config.api_endpoint(), "127.0.0.1:8080");
        assert_eq!(config.http_api_endpoint(), "http://127.0.0.1:8080");
        assert_eq!(config.websocket_endpoint(), "ws://127.0.0.1:8080/ws");
        assert_eq!(
            config.discovery_endpoint(),
            "http://127.0.0.1:8080/discovery"
        );
        assert_eq!(config.database_endpoint(), "127.0.0.1:5432");
        assert_eq!(config.dev_endpoint(), "127.0.0.1:3000");
    }

    #[test]
    fn test_config_builders() {
        let config = SovereigntyHelpersConfig::new()
            .with_api_host("example.com".to_string())
            .with_api_port("9000".to_string())
            .with_api_scheme("https".to_string())
            .with_ws_scheme("wss".to_string());

        assert_eq!(config.api_host(), "example.com");
        assert_eq!(config.api_port(), 9000);
        assert_eq!(config.http_api_endpoint(), "https://example.com:9000");
        assert_eq!(config.websocket_endpoint(), "wss://example.com:9000/ws");
    }

    #[test]
    fn test_config_arc() {
        let config = Arc::new(SovereigntyHelpersConfig::new());
        assert_eq!(config.api_host(), "127.0.0.1");
        assert_eq!(config.api_port(), 8080);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            SovereigntyHelpersConfig::new()
                .with_api_host("192.168.1.100".to_string())
                .with_api_port("7070".to_string()),
        );

        let mut handles = vec![];
        for _ in 0..100 {
            let config_clone = Arc::clone(&config);
            let handle = tokio::spawn(async move {
                assert_eq!(config_clone.api_host(), "192.168.1.100");
                assert_eq!(config_clone.api_port(), 7070);
                assert_eq!(config_clone.api_endpoint(), "192.168.1.100:7070");
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}
