//
// Builder patterns for creating and validating canonical configurations.

use crate::config::canonical_unified::{NestGateCanonicalUnifiedConfig, DeploymentEnvironment};
use crate::error::CanonicalResult as Result;
// Removed unused serde imports - will be added back when needed

/// Canonical configuration builder
#[derive(Debug, Clone)]
pub struct CanonicalConfigBuilder {
    config: NestGateCanonicalUnifiedConfig,
}

impl CanonicalConfigBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: NestGateCanonicalUnifiedConfig::default(),
        }
    }

    /// Build the final configuration
    pub fn build(self) -> Result<NestGateCanonicalUnifiedConfig> {
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
        self.config.network.http_server.port = port;
        self
    }

    /// Enable TLS
    pub fn enable_tls(mut self, enabled: bool) -> Self {
        self.config.network.tls.enabled = enabled;
        self
    }
}

impl Default for CanonicalConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Implementation for the legacy CanonicalModernizedConfig type alias
impl NestGateCanonicalUnifiedConfig {
    /// Create a default configuration
    pub fn default_config() -> Self {
        Self::default()
    }
} 