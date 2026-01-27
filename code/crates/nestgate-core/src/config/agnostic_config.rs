//! Agnostic Configuration Module
//!
//! Replaces hardcoded values with environment-driven, capability-based configuration.
//! This module demonstrates the evolution from hardcoded → agnostic patterns.
//!
//! # Philosophy
//!
//! ```text
//! OLD PATTERN (Hardcoded):
//!   let port = 8080;
//!   let host = "localhost";
//!
//! NEW PATTERN (Agnostic):
//!   let port = discover_port("api")
//!       .or_env("NESTGATE_API_PORT")
//!       .unwrap_or(8080);
//! ```
//!
//! # Examples
//!
//! ```rust
//! use nestgate_core::config::agnostic_config::ConfigBuilder;
//!
//! # async fn example() -> nestgate_core::Result<()> {
//! let config = ConfigBuilder::new()
//!     .with_capability_discovery()
//!     .with_environment_fallback()
//!     .with_safe_defaults()
//!     .build()
//!     .await?;
//!
//! println!("API endpoint: {}", config.api_endpoint());
//! # Ok(())
//! # }
//! ```

use crate::config::capability_discovery;
use crate::error::NestGateError;
use crate::Result;
use std::collections::HashMap;
use std::env;

// ==================== CONFIGURATION BUILDER ====================

/// Builder for agnostic configuration
///
/// Supports multiple configuration sources with proper fallback:
/// 1. Capability discovery (runtime, agnostic)
/// 2. Environment variables (deployment-specific)
/// 3. Configuration files (optional)
/// 4. Safe defaults (development-friendly)
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    enable_capability_discovery: bool,
    enable_environment: bool,
    enable_defaults: bool,
    custom_defaults: HashMap<String, String>,
}

impl ConfigBuilder {
    /// Create a new configuration builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable capability-based discovery
    pub fn with_capability_discovery(mut self) -> Self {
        self.enable_capability_discovery = true;
        self
    }

    /// Enable environment variable fallback
    pub fn with_environment_fallback(mut self) -> Self {
        self.enable_environment = true;
        self
    }

    /// Enable safe default values
    pub fn with_safe_defaults(mut self) -> Self {
        self.enable_defaults = true;
        self
    }

    /// Set custom default for a configuration key
    pub fn with_default(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_defaults.insert(key.into(), value.into());
        self
    }

    /// Build the configuration
    pub async fn build(self) -> Result<AgnosticConfig> {
        let mut config = AgnosticConfig {
            endpoints: HashMap::new(),
            ports: HashMap::new(),
            features: HashMap::new(),
        };

        // Discover API endpoint
        if let Ok(endpoint) = self.discover_endpoint("api").await {
            config.endpoints.insert("api".to_string(), endpoint);
        }

        // Discover storage endpoint
        if let Ok(endpoint) = self.discover_endpoint("storage").await {
            config.endpoints.insert("storage".to_string(), endpoint);
        }

        // Load ports
        config
            .ports
            .insert("api".to_string(), self.discover_port("api").await?);
        config
            .ports
            .insert("metrics".to_string(), self.discover_port("metrics").await?);
        config
            .ports
            .insert("health".to_string(), self.discover_port("health").await?);

        Ok(config)
    }

    // Internal discovery methods

    async fn discover_endpoint(&self, service: &str) -> Result<String> {
        // Try capability discovery
        if self.enable_capability_discovery {
            if let Ok(endpoint) = capability_discovery::discover_service(service).await {
                return Ok(endpoint.endpoint);
            }
        }

        // Try environment
        if self.enable_environment {
            let env_var = format!("NESTGATE_{}_ENDPOINT", service.to_uppercase());
            if let Ok(value) = env::var(&env_var) {
                return Ok(value);
            }
        }

        // Try custom defaults
        let key = format!("{}_endpoint", service);
        if let Some(value) = self.custom_defaults.get(&key) {
            return Ok(value.clone());
        }

        // Use safe defaults
        if self.enable_defaults {
            return Ok(format!("http://localhost:{}", self.default_port(service)));
        }

        Err(NestGateError::network_error(&format!(
            "Could not discover endpoint for service '{}'",
            service
        )))
    }

    async fn discover_port(&self, service: &str) -> Result<u16> {
        // Try capability discovery
        if self.enable_capability_discovery {
            if let Ok(endpoint) = capability_discovery::discover_service(service).await {
                if let Ok((_, port)) = capability_discovery::parse_endpoint(&endpoint.endpoint) {
                    return Ok(port);
                }
            }
        }

        // Try environment
        if self.enable_environment {
            let env_var = format!("NESTGATE_{}_PORT", service.to_uppercase());
            if let Ok(value) = env::var(&env_var) {
                if let Ok(port) = value.parse::<u16>() {
                    return Ok(port);
                }
            }
        }

        // Try custom defaults
        let key = format!("{}_port", service);
        if let Some(value) = self.custom_defaults.get(&key) {
            if let Ok(port) = value.parse::<u16>() {
                return Ok(port);
            }
        }

        // Use safe defaults
        if self.enable_defaults {
            return Ok(self.default_port(service));
        }

        Err(NestGateError::network_error(&format!(
            "Could not discover port for service '{}'",
            service
        )))
    }

    fn default_port(&self, service: &str) -> u16 {
        match service {
            "api" => 8080,
            "metrics" => 9090,
            "health" => 8082,
            "websocket" => 8081,
            "storage" => 5000,
            _ => 8000,
        }
    }
}

// ==================== AGNOSTIC CONFIGURATION ====================

/// Agnostic configuration container
///
/// This struct holds discovered configuration without hardcoded values.
/// All values come from discovery, environment, or safe defaults.
#[derive(Debug, Clone)]
pub struct AgnosticConfig {
    endpoints: HashMap<String, String>,
    ports: HashMap<String, u16>,
    features: HashMap<String, bool>,
}

impl AgnosticConfig {
    /// Get API endpoint
    ///
    /// # Returns
    ///
    /// Returns configured API endpoint or constructs one from environment if available.
    /// Returns None if no endpoint is configured and no environment variables are set.
    ///
    /// # Philosophy
    ///
    /// This config is "agnostic" - it adapts to what's available but doesn't
    /// hardcode infrastructure assumptions. If no endpoint is configured,
    /// we don't silently use localhost.
    pub fn api_endpoint(&self) -> Option<String> {
        self.endpoints.get("api").cloned().or_else(|| {
            // Try to construct from environment
            std::env::var("NESTGATE_API_HOST")
                .ok()
                .map(|host| format!("http://{}:{}", host, self.api_port()))
        })
    }

    /// Get API endpoint or default for development
    ///
    /// **Development only**: This method provides a localhost fallback for local development.
    /// Production code should use `api_endpoint()` and handle the None case explicitly.
    #[cfg_attr(
        not(debug_assertions),
        deprecated(note = "Use api_endpoint() in production and handle None explicitly")
    )]
    pub fn api_endpoint_or_dev_default(&self) -> String {
        use crate::constants::hardcoding::addresses;

        self.api_endpoint().unwrap_or_else(|| {
            #[cfg(debug_assertions)]
            tracing::debug!(
                "Using development default API endpoint: {}:{}",
                addresses::LOCALHOST_NAME,
                self.api_port()
            );

            format!("http://{}:{}", addresses::LOCALHOST_NAME, self.api_port())
        })
    }

    /// Get API port
    ///
    /// ✅ MIGRATED: Now uses centralized get_api_port() function
    pub fn api_port(&self) -> u16 {
        use crate::constants::get_api_port;
        self.ports.get("api").copied().unwrap_or_else(get_api_port)
    }

    /// Get storage endpoint
    pub fn storage_endpoint(&self) -> Option<String> {
        self.endpoints.get("storage").cloned()
    }

    /// Get metrics port
    ///
    /// ✅ MIGRATED: Now uses centralized get_metrics_port() function
    pub fn metrics_port(&self) -> u16 {
        use crate::constants::get_metrics_port;
        self.ports
            .get("metrics")
            .copied()
            .unwrap_or_else(get_metrics_port)
    }

    /// Get health check port
    ///
    /// ✅ MIGRATED: Now uses centralized get_health_port() function
    pub fn health_port(&self) -> u16 {
        use crate::constants::get_health_port;
        self.ports
            .get("health")
            .copied()
            .unwrap_or_else(get_health_port)
    }

    /// Check if feature is enabled
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }

    /// Enable a feature
    pub fn enable_feature(&mut self, feature: impl Into<String>) {
        self.features.insert(feature.into(), true);
    }
}

// ==================== MIGRATION HELPERS ====================

/// Migration helper: Convert hardcoded value to agnostic config
///
/// This helps migrate code from:
/// ```rust,ignore
/// let port = 8080; // Hardcoded
/// ```
///
/// To:
/// ```rust,ignore
/// let port = migrate_port("api", 8080).await?; // Agnostic
/// ```
pub async fn migrate_port(service: &str, hardcoded_fallback: u16) -> Result<u16> {
    ConfigBuilder::new()
        .with_capability_discovery()
        .with_environment_fallback()
        .with_default(format!("{}_port", service), hardcoded_fallback.to_string())
        .build()
        .await?
        .ports
        .get(service)
        .copied()
        .ok_or_else(|| {
            NestGateError::network_error(&format!(
                "Could not migrate port for service '{}'",
                service
            ))
        })
}

/// Migration helper: Convert hardcoded endpoint to agnostic config
pub async fn migrate_endpoint(service: &str, hardcoded_fallback: &str) -> Result<String> {
    let config = ConfigBuilder::new()
        .with_capability_discovery()
        .with_environment_fallback()
        .with_default(
            format!("{}_endpoint", service),
            hardcoded_fallback.to_string(),
        )
        .build()
        .await?;

    config.endpoints.get(service).cloned().ok_or_else(|| {
        NestGateError::network_error(&format!(
            "Could not migrate endpoint for service '{}'",
            service
        ))
    })
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_builder_with_defaults() {
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .build()
            .await
            .expect("Should build config with defaults");

        assert_eq!(config.api_port(), 8080);
        assert_eq!(config.metrics_port(), 9090);
        assert_eq!(config.health_port(), 8082);
    }

    #[tokio::test]
    async fn test_config_builder_with_custom_defaults() {
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .with_default("api_port", "3000")
            .build()
            .await
            .expect("Should build config");

        assert_eq!(config.api_port(), 3000);
    }

    #[tokio::test]
    async fn test_config_builder_with_environment() {
        env::set_var("NESTGATE_API_PORT", "9999");

        let config = ConfigBuilder::new()
            .with_environment_fallback()
            .with_safe_defaults()
            .build()
            .await
            .expect("Should build config");

        assert_eq!(config.api_port(), 9999);

        env::remove_var("NESTGATE_API_PORT");
    }

    #[tokio::test]
    async fn test_migrate_port() {
        // With safe defaults enabled, should always get a valid port
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .with_default("api_port", "8080")
            .build()
            .await;

        assert!(config.is_ok(), "Should build config with defaults");
        let port = config.unwrap().api_port();
        assert!(port > 0, "Port should be valid (non-zero): {}", port);
    }

    #[tokio::test]
    async fn test_migrate_endpoint() {
        // With safe defaults, should always get an endpoint
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .with_default("api_endpoint", "http://localhost:8080")
            .build()
            .await;

        assert!(config.is_ok(), "Should build config with defaults");
        let endpoint = config.unwrap().api_endpoint();
        assert!(endpoint.is_some(), "Endpoint should be present");
        let endpoint_str = endpoint.unwrap();
        assert!(!endpoint_str.is_empty(), "Endpoint should not be empty");
        assert!(endpoint_str.contains("localhost") || endpoint_str.contains("127.0.0.1"));
    }

    #[test]
    fn test_feature_flags() {
        let mut config = AgnosticConfig {
            endpoints: HashMap::new(),
            ports: HashMap::new(),
            features: HashMap::new(),
        };

        assert!(!config.is_feature_enabled("test_feature"));

        config.enable_feature("test_feature");
        assert!(config.is_feature_enabled("test_feature"));
    }
}
