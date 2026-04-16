// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Configuration for environment detection module
//!
//! This module provides immutable configuration for environment detection,
//! eliminating runtime `env::var()` calls and enabling concurrent-safe testing.

use std::collections::HashMap;
use std::sync::Arc;

use nestgate_types::{EnvSource, ProcessEnv, env_parsed, env_var_or_default};

/// Configuration for environment detection
///
/// This struct captures all environment variables at initialization time,
/// eliminating the need for runtime `env::var()` calls.
#[derive(Debug, Clone)]
/// Configuration for Environment
pub struct EnvironmentConfig {
    // Operation mode detection
    orchestration_url_present: bool,
    orchestration_url: Option<String>,

    // Service configuration
    environment: String,
    service_name: String,

    // Network settings
    bind_interface_standalone: String,
    bind_interface_orchestration: String,
    port: u16,
    nestgate_service_name: String,

    // Discovery settings
    discovery_enabled_standalone: bool,
    discovery_enabled_orchestration: bool,

    // External service URLs
    security_url: Option<String>,
    ai_url: Option<String>,
    compute_url: Option<String>,
}

/// Shared, thread-safe configuration
pub type SharedEnvironmentConfig = Arc<EnvironmentConfig>;

impl EnvironmentConfig {
    /// Default bind interface for standalone mode
    pub const DEFAULT_BIND_STANDALONE: &'static str = "127.0.0.1";
    /// Default bind interface for orchestration mode
    pub const DEFAULT_BIND_ORCHESTRATION: &'static str = "0.0.0.0";
    /// Default port (matches `crate::constants::hardcoding::RuntimeDefaults::api_port` fallback
    /// when `NESTGATE_API_PORT` is unset; see also [`crate::constants::hardcoding::get_api_port`].)
    pub const DEFAULT_PORT: u16 = crate::constants::hardcoding::runtime_fallback_ports::HTTP;
    /// Default service name
    pub const DEFAULT_SERVICE_NAME: &'static str = "nestgate";
    /// Default environment
    pub const DEFAULT_ENVIRONMENT: &'static str = "development";

    /// Create a new configuration with default values (no env vars)
    #[must_use]
    pub fn new() -> Self {
        Self {
            orchestration_url_present: false,
            orchestration_url: None,
            environment: Self::DEFAULT_ENVIRONMENT.to_string(),
            service_name: Self::DEFAULT_SERVICE_NAME.to_string(),
            bind_interface_standalone: Self::DEFAULT_BIND_STANDALONE.to_string(),
            bind_interface_orchestration: Self::DEFAULT_BIND_ORCHESTRATION.to_string(),
            port: Self::DEFAULT_PORT,
            nestgate_service_name: Self::DEFAULT_SERVICE_NAME.to_string(),
            discovery_enabled_standalone: false,
            discovery_enabled_orchestration: true,
            security_url: None,
            ai_url: None,
            compute_url: None,
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
        let orchestration_url = env.get("ORCHESTRATION_URL");
        let orchestration_url_present = orchestration_url.is_some();

        let environment = env_var_or_default(env, "ENVIRONMENT", Self::DEFAULT_ENVIRONMENT);

        let service_name = env_var_or_default(env, "SERVICE_NAME", Self::DEFAULT_SERVICE_NAME);

        let bind_interface_standalone = env_var_or_default(
            env,
            "NESTGATE_BIND_INTERFACE",
            Self::DEFAULT_BIND_STANDALONE,
        );

        let bind_interface_orchestration = env_var_or_default(
            env,
            "NESTGATE_BIND_INTERFACE",
            Self::DEFAULT_BIND_ORCHESTRATION,
        );

        let port = env_parsed(env, "NESTGATE_PORT", Self::DEFAULT_PORT);

        let nestgate_service_name =
            env_var_or_default(env, "NESTGATE_SERVICE_NAME", Self::DEFAULT_SERVICE_NAME);

        let discovery_enabled_standalone = env
            .get("NESTGATE_DISCOVERY_ENABLED")
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);

        let discovery_enabled_orchestration = env
            .get("NESTGATE_DISCOVERY_ENABLED")
            .and_then(|v| v.parse().ok())
            .unwrap_or(true);

        let security_url = env.get("SECURITY_URL");
        let ai_url = env.get("AI_URL");
        let compute_url = env.get("COMPUTE_URL");

        Self {
            orchestration_url_present,
            orchestration_url,
            environment,
            service_name,
            bind_interface_standalone,
            bind_interface_orchestration,
            port,
            nestgate_service_name,
            discovery_enabled_standalone,
            discovery_enabled_orchestration,
            security_url,
            ai_url,
            compute_url,
        }
    }

    // ==================== GETTERS ====================

    /// Check if orchestration URL is present (determines operation mode)
    #[must_use]
    pub const fn is_orchestration_mode(&self) -> bool {
        self.orchestration_url_present
    }

    /// Get orchestration URL if present
    #[must_use]
    pub fn orchestration_url(&self) -> Option<String> {
        self.orchestration_url.clone()
    }

    /// Get environment name
    #[must_use]
    pub fn environment(&self) -> String {
        self.environment.clone()
    }

    /// Get service name
    #[must_use]
    pub fn service_name(&self) -> String {
        self.service_name.clone()
    }

    /// Get bind interface for standalone mode
    #[must_use]
    pub fn bind_interface_standalone(&self) -> String {
        self.bind_interface_standalone.clone()
    }

    /// Get bind interface for orchestration mode
    #[must_use]
    pub fn bind_interface_orchestration(&self) -> String {
        self.bind_interface_orchestration.clone()
    }

    /// Get bind interface based on operation mode
    #[must_use]
    pub fn bind_interface(&self, orchestration_mode: bool) -> String {
        if orchestration_mode {
            self.bind_interface_orchestration()
        } else {
            self.bind_interface_standalone()
        }
    }

    /// Get port
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Get `NestGate` service name (for networking)
    #[must_use]
    pub fn nestgate_service_name(&self) -> String {
        self.nestgate_service_name.clone()
    }

    /// Get discovery enabled setting for standalone mode
    #[must_use]
    pub const fn discovery_enabled_standalone(&self) -> bool {
        self.discovery_enabled_standalone
    }

    /// Get discovery enabled setting for orchestration mode
    #[must_use]
    pub const fn discovery_enabled_orchestration(&self) -> bool {
        self.discovery_enabled_orchestration
    }

    /// Get discovery enabled based on operation mode
    #[must_use]
    pub const fn discovery_enabled(&self, orchestration_mode: bool) -> bool {
        if orchestration_mode {
            self.discovery_enabled_orchestration
        } else {
            self.discovery_enabled_standalone
        }
    }

    /// Get security URL if present
    #[must_use]
    pub fn security_url(&self) -> Option<String> {
        self.security_url.clone()
    }

    /// Get AI URL if present
    #[must_use]
    pub fn ai_url(&self) -> Option<String> {
        self.ai_url.clone()
    }

    /// Get compute URL if present
    #[must_use]
    pub fn compute_url(&self) -> Option<String> {
        self.compute_url.clone()
    }

    /// Get all external service URLs as a `HashMap`
    #[must_use]
    pub fn external_services(&self, orchestration_mode: bool) -> HashMap<String, String> {
        let mut services = HashMap::new();

        if orchestration_mode {
            if let Some(url) = &self.orchestration_url {
                services.insert("orchestration".to_string(), url.clone());
            }
            if let Some(url) = &self.security_url {
                services.insert("security".to_string(), url.clone());
            }
            if let Some(url) = &self.ai_url {
                services.insert("ai".to_string(), url.clone());
            }
            if let Some(url) = &self.compute_url {
                services.insert("compute".to_string(), url.clone());
            }
        }

        services
    }

    // ==================== BUILDERS ====================

    /// Builder: Set orchestration URL
    #[must_use]
    pub fn with_orchestration_url(mut self, url: Option<String>) -> Self {
        self.orchestration_url_present = url.is_some();
        self.orchestration_url = url;
        self
    }

    /// Builder: Set environment
    #[must_use]
    pub fn with_environment(mut self, environment: String) -> Self {
        self.environment = environment;
        self
    }

    /// Builder: Set service name
    #[must_use]
    pub fn with_service_name(mut self, service_name: String) -> Self {
        self.service_name = service_name;
        self
    }

    /// Builder: Set port
    #[must_use]
    pub const fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Builder: Set discovery enabled for standalone mode
    #[must_use]
    pub const fn with_discovery_standalone(mut self, enabled: bool) -> Self {
        self.discovery_enabled_standalone = enabled;
        self
    }

    /// Builder: Set discovery enabled for orchestration mode
    #[must_use]
    pub const fn with_discovery_orchestration(mut self, enabled: bool) -> Self {
        self.discovery_enabled_orchestration = enabled;
        self
    }

    /// Builder: Set security URL
    #[must_use]
    pub fn with_security_url(mut self, url: Option<String>) -> Self {
        self.security_url = url;
        self
    }

    /// Builder: Set AI URL
    #[must_use]
    pub fn with_ai_url(mut self, url: Option<String>) -> Self {
        self.ai_url = url;
        self
    }

    /// Builder: Set compute URL
    #[must_use]
    pub fn with_compute_url(mut self, url: Option<String>) -> Self {
        self.compute_url = url;
        self
    }
}

impl Default for EnvironmentConfig {
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
        let config = EnvironmentConfig::new();
        assert!(!config.is_orchestration_mode());
        assert_eq!(config.environment(), "development");
        assert_eq!(config.service_name(), "nestgate");
        assert_eq!(config.bind_interface_standalone(), "127.0.0.1");
        assert_eq!(config.bind_interface_orchestration(), "0.0.0.0");
        assert_eq!(
            config.port(),
            crate::constants::hardcoding::runtime_fallback_ports::HTTP
        );
        assert!(!config.discovery_enabled_standalone());
        assert!(config.discovery_enabled_orchestration());
    }

    #[test]
    fn test_standalone_mode() {
        let config = EnvironmentConfig::new();
        assert!(!config.is_orchestration_mode());
        assert_eq!(config.bind_interface(false), "127.0.0.1");
        assert!(!config.discovery_enabled(false));
        assert!(config.external_services(false).is_empty());
    }

    #[test]
    fn test_orchestration_mode() {
        use crate::constants::hardcoding::runtime_fallback_ports;
        let config = EnvironmentConfig::new()
            .with_orchestration_url(Some(format!(
                "http://orch:{}",
                runtime_fallback_ports::HTTP
            )))
            .with_security_url(Some(format!(
                "http://sec:{}",
                runtime_fallback_ports::HEALTH
            )));

        assert!(config.is_orchestration_mode());
        assert_eq!(config.bind_interface(true), "0.0.0.0");
        assert!(config.discovery_enabled(true));

        let services = config.external_services(true);
        assert_eq!(services.len(), 2);
        assert_eq!(
            services.get("orchestration").unwrap(),
            &format!("http://orch:{}", runtime_fallback_ports::HTTP)
        );
        assert_eq!(
            services.get("security").unwrap(),
            &format!("http://sec:{}", runtime_fallback_ports::HEALTH)
        );
    }

    #[test]
    fn test_config_builders() {
        use crate::constants::hardcoding::runtime_fallback_ports;
        let config = EnvironmentConfig::new()
            .with_environment("production".to_string())
            .with_service_name("my-service".to_string())
            .with_port(9000)
            .with_discovery_standalone(true)
            .with_ai_url(Some(format!(
                "http://ai:{}",
                runtime_fallback_ports::WEBSOCKET
            )));

        assert_eq!(config.environment(), "production");
        assert_eq!(config.service_name(), "my-service");
        assert_eq!(config.port(), 9000);
        assert!(config.discovery_enabled_standalone());
        assert_eq!(
            config.ai_url().unwrap(),
            format!("http://ai:{}", runtime_fallback_ports::WEBSOCKET)
        );
    }

    #[test]
    fn test_config_arc() {
        let config = Arc::new(EnvironmentConfig::new());
        assert_eq!(config.environment(), "development");
        assert_eq!(
            config.port(),
            crate::constants::hardcoding::runtime_fallback_ports::HTTP
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            EnvironmentConfig::new()
                .with_environment("test".to_string())
                .with_port(7070)
                .with_orchestration_url(Some("http://orch:8888".to_string())),
        );

        let mut handles = vec![];
        for _ in 0..100 {
            let config_clone = Arc::clone(&config);
            let handle = tokio::spawn(async move {
                assert_eq!(config_clone.environment(), "test");
                assert_eq!(config_clone.port(), 7070);
                assert!(config_clone.is_orchestration_mode());
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }
}
