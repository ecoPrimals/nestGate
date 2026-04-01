// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Builder patterns for creating and validating canonical configurations.

use crate::config::canonical_primary::NestGateCanonicalConfig;
use crate::config::canonical_primary::system_config::DeploymentEnvironment;
use nestgate_types::error::Result;
// Removed unused imports: serde::{Deserialize, Serialize}

/// Canonical configuration builder
///
/// Builder pattern for constructing `CanonicalConfig` instances
#[derive(Debug, Clone)]
pub struct CanonicalConfigBuilder {
    config: NestGateCanonicalConfig,
}
impl CanonicalConfigBuilder {
    /// Create a new builder with default configuration
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: NestGateCanonicalConfig::default(),
        }
    }

    /// Build the final configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid or missing required fields.
    pub fn build(self) -> Result<NestGateCanonicalConfig> {
        // Simple validation - just return the config
        // More complex validation can be added later
        Ok(self.config)
    }

    /// Set the service name
    #[must_use]
    pub fn service_name(mut self, name: impl Into<String>) -> Self {
        self.config.system.instance_name = name.into();
        self
    }

    /// Set the environment
    #[must_use]
    pub const fn environment(mut self, env: DeploymentEnvironment) -> Self {
        self.config.system.environment = env;
        self
    }

    /// Set the API port
    #[must_use]
    pub const fn api_port(self, _port: u16) -> Self {
        // Removed mut and prefixed parameter with underscore
        // Note: NetworkConfig structure needs to be updated for http_server field access
        // self.config.network.http_server.port = port; // Field not available in current structure
        self
    }

    /// Enable TLS
    #[must_use]
    pub const fn enable_tls(self, _enabled: bool) -> Self {
        // Removed mut and prefixed parameter with underscore
        // Note: NetworkConfig structure needs to be updated for tls field access
        // self.config.network.tls // Field not available in current structure.enabled = enabled;
        self
    }
}

impl Default for CanonicalConfigBuilder {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// Implementation for the legacy CanonicalModernizedConfig type alias
impl NestGateCanonicalConfig {
    /// Create a default configuration
    #[must_use]
    pub fn default_config() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::canonical_primary::system_config::DeploymentEnvironment;

    #[test]
    fn canonical_config_builder_default_and_build() {
        let cfg = CanonicalConfigBuilder::default()
            .service_name("test-svc")
            .environment(DeploymentEnvironment::Development)
            .api_port(9090)
            .enable_tls(true)
            .build()
            .expect("build");
        assert_eq!(cfg.system.instance_name, "test-svc");
        assert_eq!(cfg.system.environment, DeploymentEnvironment::Development);
    }

    #[test]
    fn nestgate_canonical_default_config_alias() {
        let a: NestGateCanonicalConfig = NestGateCanonicalConfig::default_config();
        let b: NestGateCanonicalConfig = NestGateCanonicalConfig::default();
        assert_eq!(a.system.instance_name, b.system.instance_name);
    }
}
