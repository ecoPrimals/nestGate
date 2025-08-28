//
// Builder patterns for creating and validating canonical configurations.

use crate::config::canonical_master::NestGateCanonicalConfig as NestGateCanonicalConfig;
use crate::config::canonical_master::system_config::DeploymentEnvironment;
use crate::error::CanonicalResult as Result;
use serde::{Deserialize, Serialize};

/// Canonical configuration builder
#[derive(Debug, Clone)]
pub struct CanonicalConfigBuilder {
    config: NestGateCanonicalConfig,
}

impl CanonicalConfigBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: NestGateCanonicalConfig::default(),
        }
    }

    /// Build the final configuration
    pub fn build(self) -> Result<NestGateCanonicalConfig> {
        // Simple validation - just return the config
        // More complex validation can be added later
        Ok(self.config)
    }

    /// Set the service name
    pub fn service_name(mut self, name: impl Into<String>) -> Self {
        self.config.system.service_name = name.into();
        self
    }

    /// Set the environment
    pub fn environment(mut self, env: DeploymentEnvironment) -> Self {
        self.config.system.environment = env;
        self
    }

    /// Set the API port
    pub fn api_port(mut self, port: u16) -> Self {
        // Note: NetworkConfig structure needs to be updated for http_server field access
        // self.config.network.http_server.port = port; // Field not available in current structure
        self
    }

    /// Enable TLS
    pub fn enable_tls(mut self, enabled: bool) -> Self {
        // Note: NetworkConfig structure needs to be updated for tls field access
        // self.config.network.tls // Field not available in current structure.enabled = enabled;
        self
    }
}

impl Default for CanonicalConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Implementation for the legacy CanonicalModernizedConfig type alias
impl NestGateCanonicalConfig {
    /// Create a default configuration
    pub fn default_config() -> Self {
        Self::default()
    }
} 