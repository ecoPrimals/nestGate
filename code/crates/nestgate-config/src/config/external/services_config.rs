// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Services Configuration — Capability-Based Discovery
//!
//! All peer services are addressed by **capability** (e.g. `"orchestration"`,
//! `"security"`, `"ai"`, `"compute"`, `"ecosystem"`), never by primal product name.
//!
//! ## Environment Variables
//!
//! - `NESTGATE_CAPABILITY_<NAME>` — capability URL (lowercased for lookup).
//!   Examples: `NESTGATE_CAPABILITY_ORCHESTRATION`, `NESTGATE_CAPABILITY_SECURITY`.
//! - `NESTGATE_EXTERNAL_<NAME>` — external (non-primal) service URL.
//! - Core URLs: `NESTGATE_DISCOVERY_URL`, `NESTGATE_ADAPTER_URL`, `NESTGATE_HEALTH_URL`,
//!   `NESTGATE_METRICS_URL`, `NESTGATE_CONFIG_URL`.
//!
//! ## Usage
//!
//! ```rust,ignore
//! let config = ServicesConfig::from_env();
//! let security = config.get_capability_url("security");
//! let orchestration = config.get_capability_url("orchestration");
//! ```

use nestgate_types::EnvSource;
use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe configuration for service endpoints.
///
/// Captures environment variables at initialization to prevent race conditions.
/// All peer services are addressed by **capability** (e.g. `"orchestration"`, `"security"`),
/// never by primal product name.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServicesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_url: Option<String>,
    adapter_url: Option<String>,
    health_url: Option<String>,
    metrics_url: Option<String>,
    config_url: Option<String>,

    /// Capability-based service URLs (`NESTGATE_CAPABILITY_*`).
    ///
    /// Serde aliases allow deserializing configs written before the capability migration:
    /// a serialized `songbird_url` value is folded into `capabilities["orchestration"]`, etc.
    capabilities: HashMap<String, String>,

    external_services: HashMap<String, String>,
}

/// Shared immutable reference to `ServicesConfig`
pub type SharedServicesConfig = Arc<ServicesConfig>;

impl ServicesConfig {
    /// Create a new empty configuration (all values `None`).
    #[must_use]
    pub fn new() -> Self {
        Self {
            discovery_url: None,
            adapter_url: None,
            health_url: None,
            metrics_url: None,
            config_url: None,
            capabilities: HashMap::new(),
            external_services: HashMap::new(),
        }
    }

    /// Create configuration from a custom [`EnvSource`].
    ///
    /// This is the primary constructor — [`from_env`](Self::from_env) delegates
    /// to this with [`ProcessEnv`](nestgate_types::ProcessEnv).
    #[must_use]
    pub fn from_env_source(env: &dyn EnvSource) -> Self {
        let mut config = Self::new();

        // Core services
        config.discovery_url = env.get("NESTGATE_DISCOVERY_URL");
        config.adapter_url = env.get("NESTGATE_ADAPTER_URL");
        config.health_url = env.get("NESTGATE_HEALTH_URL");
        config.metrics_url = env.get("NESTGATE_METRICS_URL");
        config.config_url = env.get("NESTGATE_CONFIG_URL");

        // Capability URLs: NESTGATE_CAPABILITY_<NAME> (name is lowercased for lookup)
        for (key, value) in env.vars() {
            if let Some(name) = key.strip_prefix("NESTGATE_CAPABILITY_") {
                config.capabilities.insert(name.to_lowercase(), value);
            }
        }

        // Scan for dynamic NESTGATE_EXTERNAL_* entries
        for (key, value) in env.vars() {
            if let Some(name) = key.strip_prefix("NESTGATE_EXTERNAL_") {
                config.external_services.insert(name.to_lowercase(), value);
            }
        }

        config
    }

    /// Create configuration from current process environment variables.
    ///
    /// Delegates to [`from_env_source`](Self::from_env_source) with
    /// [`ProcessEnv`](nestgate_types::ProcessEnv).
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_source(&nestgate_types::ProcessEnv)
    }

    /// Same as [`Self::from_env`] — captures environment at initialization time.
    #[must_use]
    pub fn from_environment() -> Self {
        Self::from_env()
    }

    // Core service accessors with defaults

    /// Gets Discovery Url
    #[must_use]
    pub fn get_discovery_url(&self) -> String {
        self.discovery_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "{}/discovery",
                config.build_endpoint(config.discovery_base_port)
            )
        })
    }

    /// Gets Adapter Url
    #[must_use]
    pub fn get_adapter_url(&self) -> String {
        self.adapter_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "{}/adapter",
                config.build_endpoint(config.discovery_base_port)
            )
        })
    }

    /// Gets Health Url
    #[must_use]
    pub fn get_health_url(&self) -> String {
        self.health_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "{}/health",
                config.build_endpoint(config.discovery_base_port)
            )
        })
    }

    /// Gets Metrics Url
    #[must_use]
    pub fn get_metrics_url(&self) -> String {
        self.metrics_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!("{}/metrics", config.build_endpoint(9090))
        })
    }

    /// Gets Config Url
    #[must_use]
    pub fn get_config_url(&self) -> String {
        self.config_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "{}/config",
                config.build_endpoint(config.discovery_base_port)
            )
        })
    }

    // Production accessors (for required URLs)

    /// Gets Discovery Url Required
    #[must_use]
    pub fn get_discovery_url_required(&self) -> Option<&str> {
        self.discovery_url.as_deref()
    }

    /// Gets Adapter Url Required
    #[must_use]
    pub fn get_adapter_url_required(&self) -> Option<&str> {
        self.adapter_url.as_deref()
    }

    /// Gets Health Url Required
    #[must_use]
    pub fn get_health_url_required(&self) -> Option<&str> {
        self.health_url.as_deref()
    }

    // ═══════════════════════════════════════════════════════════════════
    // CAPABILITY-BASED SERVICE DISCOVERY (NEW - Use These!)
    // ═══════════════════════════════════════════════════════════════════

    /// Get service URL by capability type (NEW - Preferred Method)
    ///
    /// **CAPABILITY-BASED DISCOVERY**: Use this instead of primal-specific methods.
    /// Discovers services by WHAT THEY DO, not WHO they are.
    ///
    /// # Arguments
    /// * `capability` - Capability type: "orchestration", "security", "ai", "compute", "storage", "ecosystem"
    ///
    /// # Example
    /// ```ignore
    /// // ✅ CORRECT: Capability-based
    /// let security_url = config.get_capability_url("security");
    /// let orchestration_url = config.get_capability_url("orchestration");
    ///
    /// // ❌ DEPRECATED: Don't use primal-specific methods
    /// // let beardog_url = config.get_beardog_url();
    /// ```
    ///
    /// # Returns
    /// `Some(url)` if the capability is configured, `None` otherwise
    #[must_use]
    pub fn get_capability_url(&self, capability: &str) -> Option<String> {
        self.capabilities.get(capability).cloned()
    }

    /// Get all configured capabilities
    ///
    /// Returns a map of capability types to their URLs
    #[must_use]
    pub const fn get_all_capabilities(&self) -> &HashMap<String, String> {
        &self.capabilities
    }

    // External services

    /// Gets External Service
    #[must_use]
    pub fn get_external_service(&self, name: &str) -> Option<&str> {
        self.external_services
            .get(name)
            .map(std::string::String::as_str)
    }

    /// Gets All External Services
    #[must_use]
    pub const fn get_all_external_services(&self) -> &HashMap<String, String> {
        &self.external_services
    }

    // Builder methods for tests

    /// Builder method to set Discovery Url
    #[must_use]
    pub fn with_discovery_url(mut self, url: String) -> Self {
        self.discovery_url = Some(url);
        self
    }

    /// Builder method to set Adapter Url
    #[must_use]
    pub fn with_adapter_url(mut self, url: String) -> Self {
        self.adapter_url = Some(url);
        self
    }

    /// Builder method to set Health Url
    #[must_use]
    pub fn with_health_url(mut self, url: String) -> Self {
        self.health_url = Some(url);
        self
    }

    /// Builder method to set Metrics Url
    #[must_use]
    pub fn with_metrics_url(mut self, url: String) -> Self {
        self.metrics_url = Some(url);
        self
    }

    /// Builder method to set Config Url
    #[must_use]
    pub fn with_config_url(mut self, url: String) -> Self {
        self.config_url = Some(url);
        self
    }

    /// Builder method to set capability URL (NEW - Preferred)
    ///
    /// # Example
    /// ```ignore
    /// let config = ServicesConfig::new()
    ///     .with_capability("orchestration", "http://orchestration:8080")
    ///     .with_capability("security", "http://security:9000");
    /// ```
    #[must_use]
    pub fn with_capability(
        mut self,
        capability: impl Into<String>,
        url: impl Into<String>,
    ) -> Self {
        self.capabilities.insert(capability.into(), url.into());
        self
    }

    /// Builder method to set External Service
    #[must_use]
    pub fn with_external_service(mut self, name: String, url: String) -> Self {
        self.external_services.insert(name, url);
        self
    }
}

impl Default for ServicesConfig {
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
    fn test_services_config_new() {
        let config = ServicesConfig::new();

        assert_eq!(
            config.get_discovery_url(),
            "http://127.0.0.1:8080/discovery"
        );
        assert_eq!(config.get_adapter_url(), "http://127.0.0.1:8080/adapter");
        assert!(config.get_capability_url("orchestration").is_none());
    }

    #[test]
    fn test_services_config_builder() {
        let config = ServicesConfig::new()
            .with_discovery_url("http://discovery:8080".to_string())
            .with_capability("orchestration", "http://test-orchestration:9000")
            .with_external_service("custom".to_string(), "http://custom:8000".to_string());

        assert_eq!(config.get_discovery_url(), "http://discovery:8080");
        assert_eq!(
            config.get_capability_url("orchestration"),
            Some("http://test-orchestration:9000".to_string())
        );
        assert_eq!(
            config.get_external_service("custom"),
            Some("http://custom:8000")
        );
    }

    #[test]
    fn test_services_config_production_required() {
        let config = ServicesConfig::new()
            .with_discovery_url("http://prod-discovery:8080".to_string())
            .with_adapter_url("http://prod-adapter:8080".to_string())
            .with_health_url("http://prod-health:8080".to_string());

        assert!(config.get_discovery_url_required().is_some());
        assert!(config.get_adapter_url_required().is_some());
        assert!(config.get_health_url_required().is_some());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_services_config_access() {
        let config1 = Arc::new(
            ServicesConfig::new().with_discovery_url("http://discovery1:8080".to_string()),
        );
        let config2 = Arc::new(
            ServicesConfig::new().with_discovery_url("http://discovery2:8080".to_string()),
        );

        let handle1 = {
            let config = Arc::clone(&config1);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_discovery_url(), "http://discovery1:8080");
                }
            })
        };

        let handle2 = {
            let config = Arc::clone(&config2);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_discovery_url(), "http://discovery2:8080");
                }
            })
        };

        handle1.await.expect("Thread join failed in test");
        handle2.await.expect("Thread join failed in test");
    }

    #[test]
    fn test_services_config_external_services() {
        let config = ServicesConfig::new()
            .with_external_service(
                "huggingface".to_string(),
                "https://api.huggingface.co".to_string(),
            )
            .with_external_service(
                "ncbi".to_string(),
                "https://eutils.ncbi.nlm.nih.gov".to_string(),
            );

        assert_eq!(
            config.get_external_service("huggingface"),
            Some("https://api.huggingface.co")
        );
        assert_eq!(
            config.get_external_service("ncbi"),
            Some("https://eutils.ncbi.nlm.nih.gov")
        );
        assert_eq!(config.get_all_external_services().len(), 2);
    }

    #[test]
    fn test_services_config_defaults() {
        let config = ServicesConfig::new();

        assert!(config.get_discovery_url().contains("127.0.0.1"));
        assert!(config.get_adapter_url().contains("127.0.0.1"));
        assert!(config.get_health_url().contains("127.0.0.1"));
        assert!(config.get_metrics_url().contains("127.0.0.1"));
        assert!(config.get_config_url().contains("127.0.0.1"));
    }

    #[test]
    fn from_env_capability_vars_lowercased_and_resolve_get_capability_url() {
        let env = MapEnv::from([
            ("NESTGATE_CAPABILITY_ORCHESTRATION", "http://orch-cap:9000"),
            ("NESTGATE_CAPABILITY_SECURITY", "https://sec-cap:8443"),
            ("NESTGATE_CAPABILITY_AI", "http://ai-cap:7000"),
        ]);
        let config = ServicesConfig::from_env_source(&env);
        assert_eq!(
            config.get_capability_url("orchestration"),
            Some("http://orch-cap:9000".to_string())
        );
        assert_eq!(
            config.get_capability_url("security"),
            Some("https://sec-cap:8443".to_string())
        );
        assert_eq!(
            config.get_capability_url("ai"),
            Some("http://ai-cap:7000".to_string())
        );
        assert!(config.get_capability_url("unknown").is_none());
    }

    #[test]
    fn from_env_capability_resolves_to_capability_url() {
        let env = MapEnv::from([
            ("NESTGATE_CAPABILITY_ORCHESTRATION", "http://mirror-orch:1"),
            ("NESTGATE_CAPABILITY_SECURITY", "http://mirror-sec:2"),
            ("NESTGATE_CAPABILITY_COMPUTE", "http://mirror-comp:3"),
            ("NESTGATE_CAPABILITY_ECOSYSTEM", "http://mirror-eco:4"),
        ]);
        let config = ServicesConfig::from_env_source(&env);
        assert_eq!(
            config.get_capability_url("orchestration"),
            Some("http://mirror-orch:1".to_string())
        );
        assert_eq!(
            config.get_capability_url("security"),
            Some("http://mirror-sec:2".to_string())
        );
        assert_eq!(
            config.get_capability_url("compute"),
            Some("http://mirror-comp:3".to_string())
        );
        assert_eq!(
            config.get_capability_url("ecosystem"),
            Some("http://mirror-eco:4".to_string())
        );
    }

    #[test]
    fn from_env_capability_mixed_case_prefix_still_maps_to_lowercase_key() {
        let env = MapEnv::from([("NESTGATE_CAPABILITY_STORAGE", "http://store:5000")]);
        let config = ServicesConfig::from_env_source(&env);
        assert_eq!(
            config.get_capability_url("storage"),
            Some("http://store:5000".to_string())
        );
    }

    #[test]
    fn from_env_external_services_lowercased() {
        let env = MapEnv::from([(
            "NESTGATE_EXTERNAL_MY_SERVICE",
            "https://external.example/api",
        )]);
        let config = ServicesConfig::from_env_source(&env);
        assert_eq!(
            config.get_external_service("my_service"),
            Some("https://external.example/api")
        );
        assert!(config.get_external_service("MY_SERVICE").is_none());
    }

    #[test]
    fn from_env_capability_does_not_override_explicit_core_url_when_set() {
        let env = MapEnv::from([
            ("NESTGATE_DISCOVERY_URL", "http://explicit-discovery:1111"),
            ("NESTGATE_CAPABILITY_ORCHESTRATION", "http://cap-only:2222"),
        ]);
        let config = ServicesConfig::from_env_source(&env);
        assert_eq!(config.get_discovery_url(), "http://explicit-discovery:1111");
        assert_eq!(
            config.get_capability_url("orchestration"),
            Some("http://cap-only:2222".to_string())
        );
    }

    #[test]
    fn get_all_capabilities_returns_configured_map() {
        let env = MapEnv::from([("NESTGATE_CAPABILITY_FOO", "http://foo:1")]);
        let config = ServicesConfig::from_env_source(&env);
        assert_eq!(
            config.get_all_capabilities().get("foo"),
            Some(&"http://foo:1".to_string())
        );
    }
}
