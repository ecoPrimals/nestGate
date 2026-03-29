// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Services Configuration
//!
//! ## ⚠️ MIGRATION TO CAPABILITY-BASED DISCOVERY
//!
//! This module is in transition from primal-name-based to capability-based service discovery.
//!
//! ### Deprecated Pattern (Primal Names):
//! ```rust,ignore
//! // ❌ DEPRECATED: Hardcoded primal names
//! let songbird_url = config.get_songbird_url();
//! let beardog_url = config.get_beardog_url();
//! ```
//!
//! ### New Pattern (Capability-Based):
//! ```rust,ignore
//! // ✅ CORRECT: Capability-based discovery
//! let orchestration_url = config.get_capability_url("orchestration");
//! let security_url = config.get_capability_url("security");
//! ```
//!
//! ### Environment Variables:
//! - **Legacy (supported)**: `NESTGATE_SONGBIRD_URL`, `NESTGATE_BEARDOG_URL`, etc.
//! - **New (preferred)**: `NESTGATE_CAPABILITY_ORCHESTRATION`, `NESTGATE_CAPABILITY_SECURITY`, etc.
//!
//! ### Migration Timeline:
//! - **v0.11.x**: Both patterns supported (backward compatibility)
//! - **v0.12.x**: Deprecation warnings for primal-specific methods
//! - **v0.13.x**: Primal-specific methods may be removed
//!
//! Use `get_capability_url()` for new code.

use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe configuration for service endpoints
/// Captures environment variables at initialization to prevent race conditions
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Configuration for Services
pub struct ServicesConfig {
    // Core service URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_url: Option<String>,
    adapter_url: Option<String>,
    health_url: Option<String>,
    metrics_url: Option<String>,
    config_url: Option<String>,

    // ⚠️ DEPRECATED: Primal-specific URLs (use capability-based discovery instead)
    // Kept for backward compatibility only
    songbird_url: Option<String>,
    toadstool_url: Option<String>,
    beardog_url: Option<String>,
    squirrel_url: Option<String>,
    biomeos_url: Option<String>,

    // Capability-based service URLs (NESTGATE_CAPABILITY_*)
    capabilities: HashMap<String, String>,

    // External service URLs (dynamic NESTGATE_EXTERNAL_*)
    external_services: HashMap<String, String>,
}

/// Shared immutable reference to `ServicesConfig`
pub type SharedServicesConfig = Arc<ServicesConfig>;

impl ServicesConfig {
    /// Create a new empty configuration (all values None)
    #[must_use]
    pub fn new() -> Self {
        Self {
            discovery_url: None,
            adapter_url: None,
            health_url: None,
            metrics_url: None,
            config_url: None,
            songbird_url: None,
            toadstool_url: None,
            beardog_url: None,
            squirrel_url: None,
            biomeos_url: None,
            capabilities: HashMap::new(),
            external_services: HashMap::new(),
        }
    }

    /// Create configuration from current environment variables
    /// This captures env vars at initialization time, making it thread-safe
    #[must_use]
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Core services
        config.discovery_url = std::env::var("NESTGATE_DISCOVERY_URL").ok();
        config.adapter_url = std::env::var("NESTGATE_ADAPTER_URL").ok();
        config.health_url = std::env::var("NESTGATE_HEALTH_URL").ok();
        config.metrics_url = std::env::var("NESTGATE_METRICS_URL").ok();
        config.config_url = std::env::var("NESTGATE_CONFIG_URL").ok();

        // Legacy primal services (deprecated, but supported for backward compatibility)
        config.songbird_url = std::env::var("NESTGATE_SONGBIRD_URL").ok();
        config.toadstool_url = std::env::var("NESTGATE_TOADSTOOL_URL").ok();
        config.beardog_url = std::env::var("NESTGATE_BEARDOG_URL").ok();
        config.squirrel_url = std::env::var("NESTGATE_SQUIRREL_URL").ok();
        config.biomeos_url = std::env::var("NESTGATE_BIOMEOS_URL").ok();

        // Scan for capability-based NESTGATE_CAPABILITY_* entries (NEW)
        for (key, value) in std::env::vars() {
            if let Some(name) = key.strip_prefix("NESTGATE_CAPABILITY_") {
                config.capabilities.insert(name.to_lowercase(), value);
            }
        }

        // Scan for dynamic NESTGATE_EXTERNAL_* entries
        for (key, value) in std::env::vars() {
            if let Some(name) = key.strip_prefix("NESTGATE_EXTERNAL_") {
                config.external_services.insert(name.to_lowercase(), value);
            }
        }

        // Map legacy primal names to capabilities (automatic migration)
        if let Some(url) = &config.songbird_url {
            config
                .capabilities
                .entry("orchestration".to_string())
                .or_insert_with(|| url.clone());
        }
        if let Some(url) = &config.beardog_url {
            config
                .capabilities
                .entry("security".to_string())
                .or_insert_with(|| url.clone());
        }
        if let Some(url) = &config.squirrel_url {
            config
                .capabilities
                .entry("ai".to_string())
                .or_insert_with(|| url.clone());
        }
        if let Some(url) = &config.toadstool_url {
            config
                .capabilities
                .entry("compute".to_string())
                .or_insert_with(|| url.clone());
        }
        if let Some(url) = &config.biomeos_url {
            config
                .capabilities
                .entry("ecosystem".to_string())
                .or_insert_with(|| url.clone());
        }

        config
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

    // ═══════════════════════════════════════════════════════════════════
    // DEPRECATED: Primal-specific service accessors (Use get_capability_url instead!)
    // ═══════════════════════════════════════════════════════════════════

    /// Gets Songbird Url
    ///
    /// **⚠️ DEPRECATED**: Use `get_capability_url("orchestration")` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use get_capability_url(\"orchestration\") for capability-based discovery"
    )]
    #[must_use]
    pub fn get_songbird_url(&self) -> Option<&str> {
        self.songbird_url.as_deref()
    }

    /// Gets Toadstool Url
    ///
    /// **⚠️ DEPRECATED**: Use `get_capability_url("compute")` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use get_capability_url(\"compute\") for capability-based discovery"
    )]
    #[must_use]
    pub fn get_toadstool_url(&self) -> Option<&str> {
        self.toadstool_url.as_deref()
    }

    /// Gets Beardog Url
    ///
    /// **⚠️ DEPRECATED**: Use `get_capability_url("security")` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use get_capability_url(\"security\") for capability-based discovery"
    )]
    #[must_use]
    pub fn get_beardog_url(&self) -> Option<&str> {
        self.beardog_url.as_deref()
    }

    /// Gets Squirrel Url
    ///
    /// **⚠️ DEPRECATED**: Use `get_capability_url("ai")` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use get_capability_url(\"ai\") for capability-based discovery"
    )]
    #[must_use]
    pub fn get_squirrel_url(&self) -> Option<&str> {
        self.squirrel_url.as_deref()
    }

    /// Gets Biomeos Url
    ///
    /// **⚠️ DEPRECATED**: Use `get_capability_url("ecosystem")` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use get_capability_url(\"ecosystem\") for capability-based discovery"
    )]
    #[must_use]
    pub fn get_biomeos_url(&self) -> Option<&str> {
        self.biomeos_url.as_deref()
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

    /// Builder method to set Songbird Url
    ///
    /// **⚠️ DEPRECATED**: Use `with_capability("orchestration", url)` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use with_capability(\"orchestration\", url) for capability-based discovery"
    )]
    #[must_use]
    pub fn with_songbird_url(mut self, url: String) -> Self {
        self.songbird_url = Some(url.clone());
        self.capabilities.insert("orchestration".to_string(), url);
        self
    }

    /// Builder method to set Toadstool Url
    ///
    /// **⚠️ DEPRECATED**: Use `with_capability("compute", url)` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use with_capability(\"compute\", url) for capability-based discovery"
    )]
    #[must_use]
    pub fn with_toadstool_url(mut self, url: String) -> Self {
        self.toadstool_url = Some(url.clone());
        self.capabilities.insert("compute".to_string(), url);
        self
    }

    /// Builder method to set Beardog Url
    ///
    /// **⚠️ DEPRECATED**: Use `with_capability("security", url)` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use with_capability(\"security\", url) for capability-based discovery"
    )]
    #[must_use]
    pub fn with_beardog_url(mut self, url: String) -> Self {
        self.beardog_url = Some(url.clone());
        self.capabilities.insert("security".to_string(), url);
        self
    }

    /// Builder method to set Squirrel Url
    ///
    /// **⚠️ DEPRECATED**: Use `with_capability("ai", url)` instead
    #[deprecated(
        since = "0.12.0",
        note = "Use with_capability(\"ai\", url) for capability-based discovery"
    )]
    #[must_use]
    pub fn with_squirrel_url(mut self, url: String) -> Self {
        self.squirrel_url = Some(url.clone());
        self.capabilities.insert("ai".to_string(), url);
        self
    }

    /// Builder method to set Biomeos Url
    #[must_use]
    pub fn with_biomeos_url(mut self, url: String) -> Self {
        self.biomeos_url = Some(url);
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

    #[test]
    fn test_services_config_new() {
        let config = ServicesConfig::new();

        // Should use default URLs (ServiceDiscoveryConfig defaults to 127.0.0.1)
        assert_eq!(
            config.get_discovery_url(),
            "http://127.0.0.1:8080/discovery"
        );
        assert_eq!(config.get_adapter_url(), "http://127.0.0.1:8080/adapter");
        // ✅ MODERNIZED: Use capability-based access
        assert!(config.get_capability_url("orchestration").is_none());
    }

    #[test]
    fn test_services_config_builder() {
        // ✅ MODERNIZED: Use capability-based builder pattern
        let config = ServicesConfig::new()
            .with_discovery_url("http://discovery:8080".to_string())
            .with_capability("orchestration", "http://test-orchestration:9000")
            .with_external_service("custom".to_string(), "http://custom:8000".to_string());

        assert_eq!(config.get_discovery_url(), "http://discovery:8080");
        // ✅ MODERNIZED: Check capability instead of primal name
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
        // Create two different configurations
        let config1 = Arc::new(
            ServicesConfig::new().with_discovery_url("http://discovery1:8080".to_string()),
        );
        let config2 = Arc::new(
            ServicesConfig::new().with_discovery_url("http://discovery2:8080".to_string()),
        );

        // Spawn concurrent tasks accessing different configs
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

        // All core services should have 127.0.0.1 defaults (from ServiceDiscoveryConfig)
        assert!(config.get_discovery_url().contains("127.0.0.1"));
        assert!(config.get_adapter_url().contains("127.0.0.1"));
        assert!(config.get_health_url().contains("127.0.0.1"));
        assert!(config.get_metrics_url().contains("127.0.0.1"));
        assert!(config.get_config_url().contains("127.0.0.1"));
    }
}
