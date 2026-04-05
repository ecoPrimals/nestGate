// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Configuration for system constants module
//!
//! This module provides immutable configuration for system constants,
//! eliminating runtime `env::var()` calls and enabling concurrent-safe testing.

use std::sync::Arc;

/// Configuration for system constants
///
/// This struct captures all environment variables at initialization time,
/// eliminating the need for runtime `env::var()` calls.
#[derive(Debug, Clone)]
/// Configuration for System
pub struct SystemConfig {
    // System settings
    timeout_ms: u64,
    max_connections: usize,
    buffer_size: usize,
    retry_attempts: u32,
    health_check_interval: u64,

    // Network settings
    api_port: u16,
    bind_host: String,
    api_url: Option<String>,
}

/// Shared, thread-safe configuration
pub type SharedSystemConfig = Arc<SystemConfig>;

impl SystemConfig {
    /// Default timeout in milliseconds
    pub const DEFAULT_TIMEOUT_MS: u64 = 5000;
    /// Default maximum connections
    pub const MAX_CONNECTIONS: usize = 1000;
    /// Default buffer size
    pub const BUFFER_SIZE: usize = 8192;
    /// Default retry attempts
    pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
    /// Default health check interval in seconds
    pub const DEFAULT_HEALTH_CHECK_INTERVAL: u64 = 30;
    /// Default bind host
    pub const DEFAULT_BIND_HOST: &'static str = "127.0.0.1";
    /// Default API port
    pub const DEFAULT_API_PORT: u16 = 8080;

    /// Create a new configuration with default values (no env vars)
    #[must_use]
    pub fn new() -> Self {
        Self {
            timeout_ms: Self::DEFAULT_TIMEOUT_MS,
            max_connections: Self::MAX_CONNECTIONS,
            buffer_size: Self::BUFFER_SIZE,
            retry_attempts: Self::DEFAULT_RETRY_ATTEMPTS,
            health_check_interval: Self::DEFAULT_HEALTH_CHECK_INTERVAL,
            api_port: Self::DEFAULT_API_PORT,
            bind_host: Self::DEFAULT_BIND_HOST.to_string(),
            api_url: None,
        }
    }

    /// Create configuration from environment variables
    #[must_use]
    pub fn from_env() -> Self {
        let timeout_ms = std::env::var("NESTGATE_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::DEFAULT_TIMEOUT_MS);

        let max_connections = std::env::var("NESTGATE_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::MAX_CONNECTIONS);

        let buffer_size = std::env::var("NESTGATE_BUFFER_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::BUFFER_SIZE);

        let retry_attempts = std::env::var("NESTGATE_RETRY_ATTEMPTS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::DEFAULT_RETRY_ATTEMPTS);

        let health_check_interval = std::env::var("NESTGATE_HEALTH_CHECK_INTERVAL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::DEFAULT_HEALTH_CHECK_INTERVAL);

        let api_port = std::env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::DEFAULT_API_PORT);

        let bind_host = std::env::var("NESTGATE_BIND_HOST")
            .unwrap_or_else(|_| Self::DEFAULT_BIND_HOST.to_string());

        let api_url = std::env::var("NESTGATE_API_URL").ok();

        Self {
            timeout_ms,
            max_connections,
            buffer_size,
            retry_attempts,
            health_check_interval,
            api_port,
            bind_host,
            api_url,
        }
    }

    // ==================== GETTERS ====================

    /// Get timeout in milliseconds
    #[must_use]
    pub const fn timeout_ms(&self) -> u64 {
        self.timeout_ms
    }

    /// Get maximum connections
    #[must_use]
    pub const fn max_connections(&self) -> usize {
        self.max_connections
    }

    /// Get buffer size
    #[must_use]
    pub const fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    /// Get default retry attempts
    #[must_use]
    pub const fn retry_attempts(&self) -> u32 {
        self.retry_attempts
    }

    /// Get health check interval in seconds
    #[must_use]
    pub const fn health_check_interval(&self) -> u64 {
        self.health_check_interval
    }

    /// Get API port
    #[must_use]
    pub const fn api_port(&self) -> u16 {
        self.api_port
    }

    /// Get bind host
    #[must_use]
    pub fn bind_host(&self) -> String {
        self.bind_host.clone()
    }

    /// Get API URL
    #[must_use]
    pub fn api_url(&self) -> String {
        self.api_url
            .clone()
            .unwrap_or_else(|| format!("http://{}:{}", self.bind_host, self.api_port))
    }

    // ==================== BUILDERS ====================

    /// Builder: Set timeout in milliseconds
    #[must_use]
    pub const fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Builder: Set maximum connections
    #[must_use]
    pub const fn with_max_connections(mut self, max_connections: usize) -> Self {
        self.max_connections = max_connections;
        self
    }

    /// Builder: Set buffer size
    #[must_use]
    pub const fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    /// Builder: Set retry attempts
    #[must_use]
    pub const fn with_retry_attempts(mut self, retry_attempts: u32) -> Self {
        self.retry_attempts = retry_attempts;
        self
    }

    /// Builder: Set health check interval
    #[must_use]
    pub const fn with_health_check_interval(mut self, health_check_interval: u64) -> Self {
        self.health_check_interval = health_check_interval;
        self
    }

    /// Builder: Set API port
    #[must_use]
    pub const fn with_api_port(mut self, api_port: u16) -> Self {
        self.api_port = api_port;
        self
    }

    /// Builder: Set bind host
    #[must_use]
    pub fn with_bind_host(mut self, bind_host: String) -> Self {
        self.bind_host = bind_host;
        self
    }

    /// Builder: Set API URL
    #[must_use]
    pub fn with_api_url(mut self, api_url: String) -> Self {
        self.api_url = Some(api_url);
        self
    }
}

impl Default for SystemConfig {
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
        let config = SystemConfig::new();
        assert_eq!(config.timeout_ms(), 5000);
        assert_eq!(config.max_connections(), 1000);
        assert_eq!(config.buffer_size(), 8192);
        assert_eq!(config.retry_attempts(), 3);
        assert_eq!(config.health_check_interval(), 30);
        assert_eq!(config.api_port(), 8080);
        assert_eq!(config.bind_host(), "127.0.0.1");
        assert_eq!(config.api_url(), "http://127.0.0.1:8080");
    }

    #[test]
    fn test_config_builders() {
        let config = SystemConfig::new()
            .with_timeout_ms(10000)
            .with_max_connections(2000)
            .with_buffer_size(16384)
            .with_retry_attempts(5)
            .with_health_check_interval(60)
            .with_api_port(9000)
            .with_bind_host("0.0.0.0".to_string())
            .with_api_url("http://example.com:9000".to_string());

        assert_eq!(config.timeout_ms(), 10000);
        assert_eq!(config.max_connections(), 2000);
        assert_eq!(config.buffer_size(), 16384);
        assert_eq!(config.retry_attempts(), 5);
        assert_eq!(config.health_check_interval(), 60);
        assert_eq!(config.api_port(), 9000);
        assert_eq!(config.bind_host(), "0.0.0.0");
        assert_eq!(config.api_url(), "http://example.com:9000");
    }

    #[test]
    fn test_config_arc() {
        let config = Arc::new(SystemConfig::new());
        assert_eq!(config.timeout_ms(), 5000);
        assert_eq!(config.max_connections(), 1000);
        assert_eq!(config.buffer_size(), 8192);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            SystemConfig::new()
                .with_timeout_ms(7500)
                .with_max_connections(1500)
                .with_buffer_size(10240),
        );

        let mut handles = vec![];
        for _ in 0..100 {
            let config_clone = Arc::clone(&config);
            let handle = tokio::spawn(async move {
                assert_eq!(config_clone.timeout_ms(), 7500);
                assert_eq!(config_clone.max_connections(), 1500);
                assert_eq!(config_clone.buffer_size(), 10240);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[test]
    fn test_api_url_construction() {
        let config = SystemConfig::new()
            .with_bind_host("192.168.1.100".to_string())
            .with_api_port(7070);

        assert_eq!(config.api_url(), "http://192.168.1.100:7070");
    }

    #[test]
    fn test_api_url_override() {
        let config = SystemConfig::new()
            .with_bind_host("192.168.1.100".to_string())
            .with_api_port(7070)
            .with_api_url("https://custom.example.com".to_string());

        assert_eq!(config.api_url(), "https://custom.example.com");
    }
}
