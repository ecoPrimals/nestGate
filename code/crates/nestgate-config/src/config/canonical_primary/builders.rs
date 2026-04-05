// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::domains::network::CanonicalNetworkConfig;
/// **CONFIGURATION BUILDERS**
///
/// Configuration builders and factories for creating canonical configurations.
use super::{NestGateCanonicalConfig, SecurityConfig, StorageConfig, SystemConfig};

/// Configuration builder for `NestGateCanonicalConfig`
pub struct CanonicalConfigBuilder<
    const MAX_CONNECTIONS: usize = 1000,
    const BUFFER_SIZE: usize = 65536,
    const TIMEOUT_MS: u64 = 30000,
    // Api Port (const generic parameter)
    const API_PORT: u16 = 8080,
> {
    config: NestGateCanonicalConfig<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>,
}
impl<
    const MAX_CONNECTIONS: usize,
    const BUFFER_SIZE: usize,
    const TIMEOUT_MS: u64,
    // Api Port (const generic parameter)
    const API_PORT: u16,
> CanonicalConfigBuilder<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>
{
    /// Create a new builder with default values
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: NestGateCanonicalConfig::default(),
        }
    }

    /// Build the configuration
    #[must_use]
    pub fn build(
        self,
    ) -> NestGateCanonicalConfig<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT> {
        self.config
    }

    /// Set system configuration
    #[must_use]
    pub fn with_system(mut self, system: SystemConfig<MAX_CONNECTIONS, BUFFER_SIZE>) -> Self {
        self.config.system = system;
        self
    }

    /// Set network configuration
    #[must_use]
    pub fn with_network(mut self, network: CanonicalNetworkConfig) -> Self {
        self.config.network = network;
        self
    }

    /// Set storage configuration
    #[must_use]
    pub fn with_storage(mut self, storage: StorageConfig) -> Self {
        self.config.storage = storage;
        self
    }

    /// Set security configuration
    #[must_use]
    pub fn with_security(mut self, security: SecurityConfig) -> Self {
        self.config.security = security;
        self
    }
}

impl<
    const MAX_CONNECTIONS: usize,
    const BUFFER_SIZE: usize,
    const TIMEOUT_MS: u64,
    // Api Port (const generic parameter)
    const API_PORT: u16,
> Default for CanonicalConfigBuilder<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>
{
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_config_builder_new_build() {
        let built = CanonicalConfigBuilder::<1000, 65536, 30000, 8080>::new().build();
        let _: NestGateCanonicalConfig<1000, 65536, 30000, 8080> = built;
    }

    #[test]
    fn canonical_config_builder_default() {
        let b = CanonicalConfigBuilder::<1000, 65536, 30000, 8080>::default();
        let _ = b.build();
    }

    #[test]
    fn canonical_config_builder_with_system() {
        let sys = SystemConfig::<1000, 65536>::default();
        let expected_id = sys.instance_id.clone();
        let cfg = CanonicalConfigBuilder::<1000, 65536, 30000, 8080>::new()
            .with_system(sys)
            .build();
        assert_eq!(cfg.system.instance_id, expected_id);
        assert_eq!(SystemConfig::<1000, 65536>::max_connections(), 1000);
    }

    #[test]
    fn canonical_config_builder_with_storage_chain() {
        let storage = StorageConfig::default();
        let security = SecurityConfig::default();
        let net = CanonicalNetworkConfig::default();
        let cfg = CanonicalConfigBuilder::<1000, 65536, 30000, 8080>::new()
            .with_network(net)
            .with_storage(storage)
            .with_security(security)
            .build();
        assert!(cfg.validate().is_ok());
    }
}
