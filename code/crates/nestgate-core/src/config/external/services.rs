//! Service Endpoints Configuration
//!
//! Replaces hardcoded URLs with environment-driven configuration.
//!
//! # Replaced Hardcoded Values
//!
//! This module eliminates 303 hardcoded URLs (http://, https://)
//!
//! # Environment Variables
//!
//! - `NESTGATE_DISCOVERY_URL`: Discovery service endpoint
//! - `NESTGATE_ADAPTER_URL`: Universal adapter endpoint
//! - `NESTGATE_HEALTH_URL`: Health check endpoint
//! - `NESTGATE_SONGBIRD_URL`: Songbird orchestration service
//! - `NESTGATE_TOADSTOOL_URL`: ToadStool compute service
//! - `NESTGATE_BEARDOG_URL`: Beardog security service
//! - `NESTGATE_SQUIRREL_URL`: Squirrel AI service
//! - `NESTGATE_BIOMEOS_URL`: BiomeOS substrate service

use super::services_config::ServicesConfig;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serviceendpoints
pub struct ServiceEndpoints {
    /// Core NestGate services
    pub core: CoreServices,
    /// EcoPrimals primal services
    pub primals: PrimalServices,
    /// External integrations
    pub external: HashMap<String, String>,
}

/// Core NestGate service endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Coreservices
pub struct CoreServices {
    /// Service discovery endpoint
    pub discovery: String,
    /// Universal adapter endpoint
    pub adapter: String,
    /// Health check endpoint
    pub health: String,
    /// Metrics endpoint
    pub metrics: String,
    /// Configuration service endpoint
    pub config: String,
}

/// EcoPrimals primal service endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Primalservices
pub struct PrimalServices {
    /// Songbird orchestration
    pub songbird: Option<String>,
    /// ToadStool compute
    pub toadstool: Option<String>,
    /// Beardog security
    pub beardog: Option<String>,
    /// Squirrel AI
    pub squirrel: Option<String>,
    /// BiomeOS substrate
    pub biomeos: Option<String>,
}

impl ServiceEndpoints {
    /// Load from environment variables
    /// NOTE: Creates config from env each time. For tests, use with_config() directly.
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are invalid
    pub fn from_env() -> Result<Self> {
        let config = ServicesConfig::from_env();
        Ok(Self {
            core: CoreServices::from_config(&config)?,
            primals: PrimalServices::from_config(&config),
            external: config.get_all_external_services().clone(),
        })
    }

    /// Development defaults
    #[must_use]
    pub fn default_dev() -> Self {
        Self {
            core: CoreServices::default_dev(),
            primals: PrimalServices::default_dev(),
            external: HashMap::new(),
        }
    }

    /// Production configuration (requires environment variables)
    /// NOTE: Creates config from env each time. For tests, use with_config() directly.
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are missing
    pub fn from_env_production() -> Result<Self> {
        let config = ServicesConfig::from_env();
        Ok(Self {
            core: CoreServices::from_config_production(&config)?,
            primals: PrimalServices::from_config(&config),
            external: config.get_all_external_services().clone(),
        })
    }

    /// Get discovery endpoint
    #[must_use]
    pub fn discovery_endpoint(&self) -> &str {
        &self.core.discovery
    }

    /// Get adapter endpoint
    #[must_use]
    pub fn adapter_endpoint(&self) -> &str {
        &self.core.adapter
    }

    /// Get health endpoint
    #[must_use]
    pub fn health_endpoint(&self) -> &str {
        &self.core.health
    }
}

impl CoreServices {
    /// Load from config with defaults
    ///
    /// # Errors
    ///
    /// Returns error if configuration values are invalid
    pub fn from_config(config: &ServicesConfig) -> Result<Self> {
        Ok(Self {
            discovery: config.get_discovery_url(),
            adapter: config.get_adapter_url(),
            health: config.get_health_url(),
            metrics: config.get_metrics_url(),
            config: config.get_config_url(),
        })
    }

    /// Load from environment with defaults (backward compatibility)
    ///
    /// # Errors
    ///
    /// Returns error if environment variable values are invalid
    pub fn from_env() -> Result<Self> {
        Self::from_config(&ServicesConfig::from_env())
    }

    /// Development defaults
    #[must_use]
    pub fn default_dev() -> Self {
        // Use ServiceDiscoveryConfig for consistent endpoint configuration
        let discovery_config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
        let base_endpoint = discovery_config.build_endpoint(discovery_config.discovery_base_port);
        let metrics_endpoint = discovery_config.build_endpoint(9090);

        Self {
            discovery: format!("{}/discovery", base_endpoint),
            adapter: format!("{}/adapter", base_endpoint),
            health: format!("{}/health", base_endpoint),
            metrics: format!("{}/metrics", metrics_endpoint),
            config: format!("{}/config", base_endpoint),
        }
    }

    /// Production configuration from config (no defaults)
    ///
    /// # Errors
    ///
    /// Returns error if required configuration values are missing
    pub fn from_config_production(config: &ServicesConfig) -> Result<Self> {
        let discovery = config.get_discovery_url_required().ok_or_else(|| {
            crate::error::NestGateUnifiedError::Configuration(Box::new(
                crate::error::ConfigurationErrorDetails {
                    field: "NESTGATE_DISCOVERY_URL".to_string(),
                    message: "Required environment variable not set".to_string(),
                    currentvalue: None,
                    expected: Some("Valid HTTP(S) URL for discovery service".to_string()),
                    user_error: true,
                },
            ))
        })?;

        let adapter = config.get_adapter_url_required().ok_or_else(|| {
            crate::error::NestGateUnifiedError::Configuration(Box::new(
                crate::error::ConfigurationErrorDetails {
                    field: "NESTGATE_ADAPTER_URL".to_string(),
                    message: "Required environment variable not set".to_string(),
                    currentvalue: None,
                    expected: Some("Valid HTTP(S) URL for adapter service".to_string()),
                    user_error: true,
                },
            ))
        })?;

        let health = config.get_health_url_required().ok_or_else(|| {
            crate::error::NestGateUnifiedError::Configuration(Box::new(
                crate::error::ConfigurationErrorDetails {
                    field: "NESTGATE_HEALTH_URL".to_string(),
                    message: "Required environment variable not set".to_string(),
                    currentvalue: None,
                    expected: Some("Valid HTTP(S) URL for health check service".to_string()),
                    user_error: true,
                },
            ))
        })?;

        let metrics = config.get_metrics_url();
        let config_url = config.get_config_url();

        Ok(Self {
            discovery: discovery.to_string(),
            adapter: adapter.to_string(),
            health: health.to_string(),
            metrics,
            config: config_url,
        })
    }

    /// Production configuration (no defaults) - backward compatibility
    ///
    /// # Errors
    ///
    /// Returns error if required environment variables are missing
    pub fn from_env_production() -> Result<Self> {
        Self::from_config_production(&ServicesConfig::from_env())
    }
}

impl PrimalServices {
    /// Load from config (all optional)
    #[must_use]
    pub fn from_config(config: &ServicesConfig) -> Self {
        Self {
            songbird: config.get_songbird_url().map(|s| s.to_string()),
            toadstool: config.get_toadstool_url().map(|s| s.to_string()),
            beardog: config.get_beardog_url().map(|s| s.to_string()),
            squirrel: config.get_squirrel_url().map(|s| s.to_string()),
            biomeos: config.get_biomeos_url().map(|s| s.to_string()),
        }
    }

    /// Load from environment (all optional) - backward compatibility
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_config(&ServicesConfig::from_env())
    }

    /// Development defaults (all None - discovery-based)
    #[must_use]
    pub fn default_dev() -> Self {
        Self {
            songbird: None,
            toadstool: None,
            beardog: None,
            squirrel: None,
            biomeos: None,
        }
    }

    /// Production configuration (discovery-based)
    ///
    /// In production, primals are discovered via Infant Discovery.
    /// Static URLs are optional overrides only.
    #[must_use]
    pub fn from_env_production() -> Self {
        Self::from_env()
    }

    /// Check if primal is configured
    #[must_use]
    pub fn has_primal(&self, name: &str) -> bool {
        match name.to_lowercase().as_str() {
            "songbird" => self.songbird.is_some(),
            "toadstool" => self.toadstool.is_some(),
            "beardog" => self.beardog.is_some(),
            "squirrel" => self.squirrel.is_some(),
            "biomeos" => self.biomeos.is_some(),
            _ => false,
        }
    }

    /// Get primal URL if configured
    #[must_use]
    pub fn get_primal(&self, name: &str) -> Option<&str> {
        match name.to_lowercase().as_str() {
            "songbird" => self.songbird.as_deref(),
            "toadstool" => self.toadstool.as_deref(),
            "beardog" => self.beardog.as_deref(),
            "squirrel" => self.squirrel.as_deref(),
            "biomeos" => self.biomeos.as_deref(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_dev_services() {
        let services = ServiceEndpoints::default_dev();
        // Verify endpoints are properly formed with ServiceDiscoveryConfig
        assert!(services.discovery_endpoint().starts_with("http://"));
        assert!(services.discovery_endpoint().ends_with("/discovery"));
        assert!(services.adapter_endpoint().starts_with("http://"));
        assert!(services.adapter_endpoint().ends_with("/adapter"));
    }

    #[test]
    fn test_core_services_defaults() {
        let core = CoreServices::default_dev();
        assert!(core.discovery.starts_with("http://"));
        assert!(core.adapter.contains("adapter"));
    }

    #[test]
    fn test_primal_services_empty() {
        let primals = PrimalServices::default_dev();
        assert!(primals.songbird.is_none());
        assert!(primals.toadstool.is_none());
    }

    #[test]
    fn test_has_primal() {
        let mut primals = PrimalServices::default_dev();
        assert!(!primals.has_primal("songbird"));

        primals.songbird = Some("http://songbird:8081".to_string());
        assert!(primals.has_primal("songbird"));
        assert!(primals.has_primal("Songbird")); // Case insensitive
    }

    #[test]
    fn test_get_primal() {
        let mut primals = PrimalServices::default_dev();
        primals.beardog = Some("http://beardog:8082".to_string());

        assert_eq!(primals.get_primal("beardog"), Some("http://beardog:8082"));
        assert_eq!(primals.get_primal("songbird"), None);
    }
}
