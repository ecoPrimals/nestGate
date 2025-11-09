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
    const API_PORT: u16 = 8080,
> {
    config: NestGateCanonicalConfig<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>,
}
impl<
        const MAX_CONNECTIONS: usize,
        const BUFFER_SIZE: usize,
        const TIMEOUT_MS: u64,
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
        const API_PORT: u16,
    > Default for CanonicalConfigBuilder<MAX_CONNECTIONS, BUFFER_SIZE, TIMEOUT_MS, API_PORT>
{
    fn default() -> Self {
        Self::new()
    }
}
