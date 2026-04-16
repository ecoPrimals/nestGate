// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
//! ```rust,ignore
//! use nestgate_core::config::agnostic_config::ConfigBuilder;
//!
//! # async fn example() -> nestgate_core::Result<()> {
//! let config = ConfigBuilder::new()
//!     .with_capability_discovery()
//!     .with_environment_fallback()
//!     .with_safe_defaults()
//!     .build()?;
//!
//! println!("API endpoint: {:?}", config.api_endpoint());
//! # Ok(())
//! # }
//! ```

use crate::config::capability_discovery;
use crate::constants::hardcoding::runtime_fallback_ports;
use crate::constants::{DEFAULT_API_PORT, DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT, LOCALHOST};
use nestgate_types::EnvSource;
use nestgate_types::ProcessEnv;
use nestgate_types::error::NestGateError;
use nestgate_types::error::Result;
use std::collections::HashMap;

/// Maps logical service names to `NESTGATE_CAPABILITY_*_ENDPOINT` keys used in port migration.
fn capability_endpoint_env_key(service: &str) -> Option<&'static str> {
    match service {
        "api" => Some("NESTGATE_CAPABILITY_HTTP_API_ENDPOINT"),
        "storage" => Some("NESTGATE_CAPABILITY_STORAGE_ENDPOINT"),
        "metrics" => Some("NESTGATE_CAPABILITY_METRICS_ENDPOINT"),
        "health" => Some("NESTGATE_CAPABILITY_HEALTH_CHECK_ENDPOINT"),
        _ => None,
    }
}

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
    const DEFAULT_WEBSOCKET_PORT: u16 = 8081;
    const DEFAULT_FALLBACK_PORT: u16 = 8000;

    /// Create a new configuration builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable capability-based discovery
    #[must_use]
    pub const fn with_capability_discovery(mut self) -> Self {
        self.enable_capability_discovery = true;
        self
    }

    /// Enable environment variable fallback
    #[must_use]
    pub const fn with_environment_fallback(mut self) -> Self {
        self.enable_environment = true;
        self
    }

    /// Enable safe default values
    #[must_use]
    pub const fn with_safe_defaults(mut self) -> Self {
        self.enable_defaults = true;
        self
    }

    /// Set custom default for a configuration key
    #[must_use]
    pub fn with_default(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_defaults.insert(key.into(), value.into());
        self
    }

    /// Build the configuration
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] when capability discovery, environment resolution, or port
    /// discovery fails.
    pub fn build(self) -> Result<AgnosticConfig> {
        self.build_from_env_source(&ProcessEnv)
    }

    /// Build using an injectable environment source (see [`MapEnv`](nestgate_types::MapEnv) for tests).
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] when capability discovery, environment resolution, or port
    /// discovery fails.
    pub fn build_from_env_source(self, env: &(impl EnvSource + ?Sized)) -> Result<AgnosticConfig> {
        let mut config = AgnosticConfig {
            endpoints: HashMap::new(),
            ports: HashMap::new(),
            features: HashMap::new(),
        };

        // Discover API endpoint
        if let Ok(endpoint) = self.discover_endpoint("api", env) {
            config.endpoints.insert("api".to_string(), endpoint);
        }

        // Discover storage endpoint
        if let Ok(endpoint) = self.discover_endpoint("storage", env) {
            config.endpoints.insert("storage".to_string(), endpoint);
        }

        // Load ports
        config
            .ports
            .insert("api".to_string(), self.discover_port("api", env)?);
        config
            .ports
            .insert("metrics".to_string(), self.discover_port("metrics", env)?);
        config
            .ports
            .insert("health".to_string(), self.discover_port("health", env)?);

        Ok(config)
    }

    // Internal discovery methods

    fn discover_endpoint(&self, service: &str, env: &(impl EnvSource + ?Sized)) -> Result<String> {
        // Try capability discovery
        if self.enable_capability_discovery
            && let Ok(endpoint) = capability_discovery::discover_service_with_env(service, env)
        {
            return Ok(endpoint.endpoint);
        }

        // Try environment
        if self.enable_environment {
            let env_var = format!("NESTGATE_{}_ENDPOINT", service.to_uppercase());
            if let Some(value) = env.get(&env_var) {
                return Ok(value);
            }
        }

        // Try custom defaults
        let key = format!("{service}_endpoint");
        if let Some(value) = self.custom_defaults.get(&key) {
            return Ok(value.clone());
        }

        // Capability env (NESTGATE_CAPABILITY_*_ENDPOINT) before generic dev defaults
        if let Some(cap_env) = capability_endpoint_env_key(service)
            && let Some(value) = env.get(cap_env)
        {
            return Ok(value);
        }

        // Use safe defaults (host from env — never assume a fixed peer address)
        if self.enable_defaults {
            let host = env
                .get("NESTGATE_DEV_HOST")
                .or_else(|| env.get("NESTGATE_DISCOVERY_DEV_HOST"))
                .or_else(|| env.get("NESTGATE_DISCOVERY_FALLBACK_HOST"))
                .unwrap_or_else(|| {
                    tracing::warn!(
                        "Agnostic safe defaults: no NESTGATE_DEV_HOST, NESTGATE_DISCOVERY_DEV_HOST, \
                         or NESTGATE_DISCOVERY_FALLBACK_HOST; using {} for service '{}'.",
                        LOCALHOST,
                        service
                    );
                    LOCALHOST.to_string()
                });
            return Ok(format!("http://{}:{}", host, Self::default_port(service)));
        }

        Err(NestGateError::network_error(format!(
            "Could not discover endpoint for service '{service}'"
        )))
    }

    fn discover_port(&self, service: &str, env: &(impl EnvSource + ?Sized)) -> Result<u16> {
        // Try capability discovery
        if self.enable_capability_discovery
            && let Ok(endpoint) = capability_discovery::discover_service_with_env(service, env)
            && let Ok((_, port)) = capability_discovery::parse_endpoint(&endpoint.endpoint)
        {
            return Ok(port);
        }

        // Try environment
        if self.enable_environment {
            let env_var = format!("NESTGATE_{}_PORT", service.to_uppercase());
            if let Some(value) = env.get(&env_var)
                && let Ok(port) = value.parse::<u16>()
            {
                return Ok(port);
            }
        }

        // Try custom defaults
        let key = format!("{service}_port");
        if let Some(value) = self.custom_defaults.get(&key)
            && let Ok(port) = value.parse::<u16>()
        {
            return Ok(port);
        }

        // Use safe defaults
        if self.enable_defaults {
            return Ok(Self::default_port(service));
        }

        Err(NestGateError::network_error(format!(
            "Could not discover port for service '{service}'"
        )))
    }

    fn default_port(service: &str) -> u16 {
        match service {
            "api" => DEFAULT_API_PORT,
            "metrics" => DEFAULT_METRICS_PORT,
            "health" => DEFAULT_HEALTH_PORT,
            "websocket" => Self::DEFAULT_WEBSOCKET_PORT,
            "storage" => runtime_fallback_ports::STORAGE,
            _ => Self::DEFAULT_FALLBACK_PORT,
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
    #[must_use]
    pub fn api_endpoint(&self) -> Option<String> {
        self.api_endpoint_for_env_source(&ProcessEnv)
    }

    /// Like [`Self::api_endpoint`], but reads `NESTGATE_API_HOST` from `env` (for tests with [`MapEnv`](nestgate_types::MapEnv)).
    #[must_use]
    pub fn api_endpoint_for_env_source(&self, env: &(impl EnvSource + ?Sized)) -> Option<String> {
        self.endpoints.get("api").cloned().or_else(|| {
            env.get("NESTGATE_API_HOST")
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
    #[must_use]
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
    /// ✅ MIGRATED: Now uses centralized `get_api_port()` function
    pub fn api_port(&self) -> u16 {
        use crate::constants::get_api_port;
        self.ports.get("api").copied().unwrap_or_else(get_api_port)
    }

    /// Get storage endpoint
    #[must_use]
    pub fn storage_endpoint(&self) -> Option<String> {
        self.endpoints.get("storage").cloned()
    }

    /// Get metrics port
    ///
    /// ✅ MIGRATED: Now uses centralized `get_metrics_port()` function
    pub fn metrics_port(&self) -> u16 {
        use crate::constants::get_metrics_port;
        self.ports
            .get("metrics")
            .copied()
            .unwrap_or_else(get_metrics_port)
    }

    /// Get health check port
    ///
    /// ✅ MIGRATED: Now uses centralized `get_health_port()` function
    pub fn health_port(&self) -> u16 {
        use crate::constants::get_health_port;
        self.ports
            .get("health")
            .copied()
            .unwrap_or_else(get_health_port)
    }

    /// Check if feature is enabled
    #[must_use]
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
/// let port = migrate_port("api", 8080)?; // Agnostic
/// ```
///
/// # Errors
///
/// Returns [`NestGateError`] when [`ConfigBuilder::build`] fails or the service port is missing
/// after migration.
pub fn migrate_port(service: &str, hardcoded_fallback: u16) -> Result<u16> {
    migrate_port_from_env_source(service, hardcoded_fallback, &ProcessEnv)
}

/// Like [`migrate_port`], but uses `env` for variable resolution.
pub fn migrate_port_from_env_source(
    service: &str,
    hardcoded_fallback: u16,
    env: &(impl EnvSource + ?Sized),
) -> Result<u16> {
    ConfigBuilder::new()
        .with_capability_discovery()
        .with_environment_fallback()
        .with_default(format!("{service}_port"), hardcoded_fallback.to_string())
        .build_from_env_source(env)?
        .ports
        .get(service)
        .copied()
        .ok_or_else(|| {
            NestGateError::network_error(format!("Could not migrate port for service '{service}'"))
        })
}

/// Migration helper: Convert hardcoded endpoint to agnostic config
///
/// # Errors
///
/// Returns [`NestGateError`] when [`ConfigBuilder::build`] fails or the service endpoint is missing
/// after migration.
pub fn migrate_endpoint(service: &str, hardcoded_fallback: &str) -> Result<String> {
    migrate_endpoint_from_env_source(service, hardcoded_fallback, &ProcessEnv)
}

/// Like [`migrate_endpoint`], but uses `env` for variable resolution.
pub fn migrate_endpoint_from_env_source(
    service: &str,
    hardcoded_fallback: &str,
    env: &(impl EnvSource + ?Sized),
) -> Result<String> {
    let config = ConfigBuilder::new()
        .with_capability_discovery()
        .with_environment_fallback()
        .with_default(
            format!("{service}_endpoint"),
            hardcoded_fallback.to_string(),
        )
        .build_from_env_source(env)?;

    config.endpoints.get(service).cloned().ok_or_else(|| {
        NestGateError::network_error(format!(
            "Could not migrate endpoint for service '{service}'"
        ))
    })
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

    #[test]
    fn test_config_builder_with_defaults() {
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .build()
            .expect("Should build config with defaults");

        assert_eq!(config.api_port(), 8080);
        assert_eq!(config.metrics_port(), 9090);
        assert_eq!(config.health_port(), 8082);
    }

    #[test]
    fn test_config_builder_with_custom_defaults() {
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .with_default("api_port", "3000")
            .build()
            .expect("Should build config");

        assert_eq!(config.api_port(), 3000);
    }

    #[test]
    fn test_config_builder_with_environment() {
        let env = MapEnv::from([("NESTGATE_API_PORT", "9999")]);
        let config = ConfigBuilder::new()
            .with_environment_fallback()
            .with_safe_defaults()
            .build_from_env_source(&env)
            .expect("Should build config");

        assert_eq!(config.api_port(), 9999);
    }

    #[test]
    fn test_migrate_port() {
        // With safe defaults enabled, should always get a valid port
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .with_default("api_port", "8080")
            .build()
            .expect("Should build config with defaults");
        let port = config.api_port();
        assert!(port > 0, "Port should be valid (non-zero): {}", port);
    }

    #[test]
    fn test_migrate_endpoint() {
        // With safe defaults, should always get an endpoint
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .with_default("api_endpoint", "http://localhost:8080")
            .build()
            .expect("Should build config with defaults");
        let endpoint = config.api_endpoint().expect("Endpoint should be present");
        assert!(!endpoint.is_empty(), "Endpoint should not be empty");
        assert!(endpoint.contains("localhost") || endpoint.contains("127.0.0.1"));
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

    #[test]
    fn build_fails_without_port_sources() {
        let err = ConfigBuilder::new().build();
        assert!(err.is_err(), "expected missing port discovery to fail");
    }

    #[test]
    fn build_succeeds_with_ports_only_endpoints_empty_without_endpoint_sources() {
        let config = ConfigBuilder::new()
            .with_default("api_port", "8080")
            .with_default("metrics_port", "9090")
            .with_default("health_port", "8082")
            .build()
            .expect("ports from custom defaults should be enough to build");

        assert!(config.api_endpoint().is_none());
        assert!(config.storage_endpoint().is_none());
        assert_eq!(config.api_port(), 8080);
    }

    #[test]
    fn invalid_api_port_env_falls_through_to_safe_defaults() {
        let env = MapEnv::from([
            ("NESTGATE_API_PORT", "not-a-u16"),
            ("NESTGATE_METRICS_PORT", "also-bad"),
            ("NESTGATE_HEALTH_PORT", "xyz"),
        ]);
        let config = ConfigBuilder::new()
            .with_environment_fallback()
            .with_safe_defaults()
            .build_from_env_source(&env)
            .expect("ports should fall back to safe defaults when env parse fails");

        assert_eq!(config.api_port(), DEFAULT_API_PORT);
        assert_eq!(config.metrics_port(), DEFAULT_METRICS_PORT);
        assert_eq!(config.health_port(), DEFAULT_HEALTH_PORT);
    }

    #[test]
    fn capability_endpoint_env_resolves_without_other_flags() {
        let env = MapEnv::from([(
            "NESTGATE_CAPABILITY_HTTP_API_ENDPOINT",
            "http://cap.example:9999",
        )]);
        let config = ConfigBuilder::new()
            .with_default("api_port", "9999")
            .with_default("metrics_port", "9090")
            .with_default("health_port", "8082")
            .build_from_env_source(&env)
            .expect("capability endpoint env should supply api endpoint");

        assert_eq!(
            config.api_endpoint_for_env_source(&env).as_deref(),
            Some("http://cap.example:9999")
        );
    }

    #[test]
    fn api_endpoint_built_from_nestgate_api_host_when_endpoint_missing() {
        let env = MapEnv::from([
            ("NESTGATE_API_PORT", "8080"),
            ("NESTGATE_API_HOST", "10.0.0.42"),
        ]);
        let config = ConfigBuilder::new()
            .with_environment_fallback()
            .with_default("api_port", "8080")
            .with_default("metrics_port", "9090")
            .with_default("health_port", "8082")
            .build_from_env_source(&env)
            .expect("custom ports should allow build when endpoint omitted");

        assert_eq!(
            config.api_endpoint_for_env_source(&env).as_deref(),
            Some("http://10.0.0.42:8080")
        );
    }

    #[test]
    fn migrate_port_returns_hardcoded_fallback_when_isolated() {
        // `migrate_port` only sets `{service}_port`; `build` still resolves metrics/health ports.
        let env = MapEnv::from([
            ("NESTGATE_METRICS_PORT", "9090"),
            ("NESTGATE_HEALTH_PORT", "8082"),
        ]);
        let port = migrate_port_from_env_source("api", 4242, &env).expect("migrate_port");
        assert_eq!(port, 4242);
    }

    #[test]
    fn migrate_endpoint_returns_hardcoded_fallback_when_isolated() {
        let env = MapEnv::from([
            ("NESTGATE_API_PORT", "8080"),
            ("NESTGATE_METRICS_PORT", "9090"),
            ("NESTGATE_HEALTH_PORT", "8082"),
        ]);
        let ep = migrate_endpoint_from_env_source("api", "http://migrate.example:7777", &env)
            .expect("migrate_endpoint");
        assert_eq!(ep, "http://migrate.example:7777");
    }

    #[test]
    fn default_port_unknown_service_is_fallback_8000() {
        assert_eq!(
            ConfigBuilder::default_port("websocket"),
            ConfigBuilder::DEFAULT_WEBSOCKET_PORT
        );
        assert_eq!(
            ConfigBuilder::default_port("nosuch"),
            ConfigBuilder::DEFAULT_FALLBACK_PORT
        );
    }

    #[test]
    fn safe_defaults_without_host_env_uses_localhost_constant() {
        let env = MapEnv::from([]);
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .build_from_env_source(&env)
            .expect("build with safe defaults");
        let ep = config
            .api_endpoint()
            .expect("api endpoint from safe defaults");
        assert!(
            ep.contains(LOCALHOST) || ep.contains("localhost"),
            "endpoint should use dev localhost fallback: {ep}"
        );
    }

    #[test]
    fn nestgate_api_endpoint_env_wins_over_capability_endpoint_env() {
        let env = MapEnv::from([
            ("NESTGATE_API_ENDPOINT", "http://from-generic-env:1111"),
            (
                "NESTGATE_CAPABILITY_HTTP_API_ENDPOINT",
                "http://from-cap-env:2222",
            ),
        ]);
        let config = ConfigBuilder::new()
            .with_environment_fallback()
            .with_default("api_port", "8080")
            .with_default("metrics_port", "9090")
            .with_default("health_port", "8082")
            .build_from_env_source(&env)
            .expect("build with env");

        assert_eq!(
            config.endpoints.get("api").map(String::as_str),
            Some("http://from-generic-env:1111")
        );
    }

    #[test]
    fn custom_default_invalid_port_string_falls_back_to_safe_default() {
        let env = MapEnv::from([]);
        let config = ConfigBuilder::new()
            .with_safe_defaults()
            .with_default("api_port", "not-a-u16")
            .build_from_env_source(&env)
            .expect("build");
        assert_eq!(config.api_port(), DEFAULT_API_PORT);
    }

    #[test]
    fn build_without_endpoint_sources_has_no_api_endpoint_in_map_env() {
        let env = MapEnv::from([]);
        let config = ConfigBuilder::new()
            .with_default("api_port", "8080")
            .with_default("metrics_port", "9090")
            .with_default("health_port", "8082")
            .build_from_env_source(&env)
            .expect("ports allow build");
        assert!(
            config
                .api_endpoint_for_env_source(&MapEnv::from([]))
                .is_none()
        );
    }
}
