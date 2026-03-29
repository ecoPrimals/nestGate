// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Network defaults configuration module
//!
//! Provides thread-safe configuration for network defaults including bind addresses,
//! hostnames, and environment detection. All values are loaded from environment
//! variables at initialization time to prevent race conditions.
//!
//! # Example
//!
//! ```rust,ignore
//! use nestgate_core::constants::network_defaults_config::NetworkDefaultsConfig;
//!
//! // Load from environment variables
//! let config = NetworkDefaultsConfig::from_env();
//! println!("Bind address: {}", config.get_bind_address());
//!
//! // Or build manually for testing
//! let test_config = NetworkDefaultsConfig::new()
//!     .with_api_host("test.example.com".to_string())
//!     .with_environment("production".to_string());
//! ```
use super::network_defaults::{DEFAULT_BIND_ADDRESS, LOCALHOST_NAME};
use std::sync::Arc;

/// Thread-safe configuration for network defaults
///
/// Captures environment variables at initialization to prevent race conditions.
/// All values are immutable after construction, making this safe to share across threads.
#[derive(Debug, Clone)]
pub struct NetworkDefaultsConfig {
    bind_address: Option<String>,
    api_host: Option<String>,
    db_host: Option<String>,
    redis_host: Option<String>,
    environment: Option<String>,
}

/// Shared immutable reference to `NetworkDefaultsConfig`
pub type SharedNetworkDefaultsConfig = Arc<NetworkDefaultsConfig>;

impl NetworkDefaultsConfig {
    /// Create a new empty configuration (all values None, will use hardcoded defaults)
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bind_address: None,
            api_host: None,
            db_host: None,
            redis_host: None,
            environment: None,
        }
    }

    /// Create configuration from current environment variables
    /// This captures env vars at initialization time, making it thread-safe
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            bind_address: std::env::var("NESTGATE_BIND_ADDRESS").ok(),
            api_host: std::env::var("NESTGATE_API_HOST").ok(),
            db_host: std::env::var("NESTGATE_DB_HOST").ok(),
            redis_host: std::env::var("NESTGATE_REDIS_HOST").ok(),
            environment: std::env::var("NESTGATE_ENVIRONMENT").ok(),
        }
    }

    // Accessors with fallback to defaults

    /// Gets Bind Address
    #[must_use]
    pub fn get_bind_address(&self) -> String {
        self.bind_address
            .clone()
            .unwrap_or_else(|| DEFAULT_BIND_ADDRESS.to_string())
    }

    /// Gets Api Host
    #[must_use]
    pub fn get_api_host(&self) -> String {
        self.api_host
            .clone()
            .unwrap_or_else(|| LOCALHOST_NAME.to_string())
    }

    /// Gets Db Host
    #[must_use]
    pub fn get_db_host(&self) -> String {
        self.db_host
            .clone()
            .unwrap_or_else(|| LOCALHOST_NAME.to_string())
    }

    /// Gets Redis Host
    #[must_use]
    pub fn get_redis_host(&self) -> String {
        self.redis_host
            .clone()
            .unwrap_or_else(|| LOCALHOST_NAME.to_string())
    }

    /// Checks if Production
    #[must_use]
    pub fn is_production(&self) -> bool {
        self.environment.as_ref().is_some_and(|env| {
            let env_lower = env.to_lowercase();
            env_lower == "production" || env_lower == "prod"
        })
    }

    /// Checks if Development
    #[must_use]
    pub fn is_development(&self) -> bool {
        self.environment.as_ref().is_none_or(|env| {
            let env_lower = env.to_lowercase();
            env_lower == "development" || env_lower == "dev"
        }) // Default to development for safety
    }

    // Builder methods for tests

    /// Builder method to set Bind Address
    #[must_use]
    pub fn with_bind_address(mut self, address: String) -> Self {
        self.bind_address = Some(address);
        self
    }

    /// Builder method to set Api Host
    #[must_use]
    pub fn with_api_host(mut self, host: String) -> Self {
        self.api_host = Some(host);
        self
    }

    /// Builder method to set Db Host
    #[must_use]
    pub fn with_db_host(mut self, host: String) -> Self {
        self.db_host = Some(host);
        self
    }

    /// Builder method to set Redis Host
    #[must_use]
    pub fn with_redis_host(mut self, host: String) -> Self {
        self.redis_host = Some(host);
        self
    }

    /// Builder method to set Environment
    #[must_use]
    pub fn with_environment(mut self, env: String) -> Self {
        self.environment = Some(env);
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

    #[test]
    fn test_network_defaults_config_new() {
        let config = NetworkDefaultsConfig::new();

        // Should use hardcoded defaults
        assert_eq!(config.get_bind_address(), "0.0.0.0");
        assert_eq!(config.get_api_host(), "localhost");
        assert_eq!(config.get_db_host(), "localhost");
        assert_eq!(config.get_redis_host(), "localhost");
        assert!(config.is_development());
        assert!(!config.is_production());
    }

    #[test]
    fn test_network_defaults_config_builder() {
        let config = NetworkDefaultsConfig::new()
            .with_bind_address("192.168.1.1".to_string())
            .with_api_host("api.example.com".to_string())
            .with_db_host("db.example.com".to_string())
            .with_redis_host("redis.example.com".to_string())
            .with_environment("production".to_string());

        assert_eq!(config.get_bind_address(), "192.168.1.1");
        assert_eq!(config.get_api_host(), "api.example.com");
        assert_eq!(config.get_db_host(), "db.example.com");
        assert_eq!(config.get_redis_host(), "redis.example.com");
        assert!(config.is_production());
        assert!(!config.is_development());
    }

    #[test]
    fn test_environment_detection() {
        // Production
        let prod_config = NetworkDefaultsConfig::new().with_environment("production".to_string());
        assert!(prod_config.is_production());
        assert!(!prod_config.is_development());

        // Production (short form)
        let prod_short_config = NetworkDefaultsConfig::new().with_environment("prod".to_string());
        assert!(prod_short_config.is_production());

        // Development
        let dev_config = NetworkDefaultsConfig::new().with_environment("development".to_string());
        assert!(dev_config.is_development());
        assert!(!dev_config.is_production());

        // Development (short form)
        let dev_short_config = NetworkDefaultsConfig::new().with_environment("dev".to_string());
        assert!(dev_short_config.is_development());

        // Default (no environment)
        let default_config = NetworkDefaultsConfig::new();
        assert!(default_config.is_development());
        assert!(!default_config.is_production());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_network_defaults_config_access() {
        // Create two different configurations
        let config1 = Arc::new(
            NetworkDefaultsConfig::new()
                .with_api_host("host1.example.com".to_string())
                .with_environment("production".to_string()),
        );
        let config2 = Arc::new(
            NetworkDefaultsConfig::new()
                .with_api_host("host2.example.com".to_string())
                .with_environment("development".to_string()),
        );

        // Spawn concurrent tasks accessing different configs
        let handle1 = {
            let config = Arc::clone(&config1);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_api_host(), "host1.example.com");
                    assert!(config.is_production());
                }
            })
        };

        let handle2 = {
            let config = Arc::clone(&config2);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_api_host(), "host2.example.com");
                    assert!(config.is_development());
                }
            })
        };

        handle1.await.unwrap();
        handle2.await.unwrap();
    }

    #[test]
    fn test_all_hosts_configurable() {
        let config = NetworkDefaultsConfig::new()
            .with_bind_address("10.0.0.1".to_string())
            .with_api_host("api.local".to_string())
            .with_db_host("db.local".to_string())
            .with_redis_host("redis.local".to_string());

        // All hosts should be customizable
        assert_eq!(config.get_bind_address(), "10.0.0.1");
        assert_eq!(config.get_api_host(), "api.local");
        assert_eq!(config.get_db_host(), "db.local");
        assert_eq!(config.get_redis_host(), "redis.local");
    }

    #[test]
    fn test_case_insensitive_environment() {
        let config_upper = NetworkDefaultsConfig::new().with_environment("PRODUCTION".to_string());
        assert!(config_upper.is_production());

        let config_mixed = NetworkDefaultsConfig::new().with_environment("Production".to_string());
        assert!(config_mixed.is_production());

        let config_dev_upper =
            NetworkDefaultsConfig::new().with_environment("DEVELOPMENT".to_string());
        assert!(config_dev_upper.is_development());
    }
}
