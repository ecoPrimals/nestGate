// **TEST BUILDERS**
//
// Provides builder patterns for creating test data and configurations
// in a consistent and maintainable way.

use nestgate_core::config::canonical_master::CanonicalMasterConfig;
use nestgate_core::config::defaults::*;
use nestgate_core::environment::Environment;

/// Builder for test configurations
pub struct ConfigBuilder {
    config: CanonicalMasterConfig,
}

impl ConfigBuilder {
    /// Create a new config builder with sensible defaults for testing
    pub fn new() -> Self {
        Self {
            config: CanonicalMasterConfig::default(),
        }
    }

    /// Configure for testing environment
    pub fn for_testing(mut self) -> Self {
        self.config.environment = Environment::Testing;
        self
    }

    /// Configure for development environment
    pub fn for_development(mut self) -> Self {
        self.config.environment = Environment::Development;
        self
    }

    /// Configure for production-like testing
    pub fn for_production_like(mut self) -> Self {
        self.config.environment = Environment::Production;
        // But use test-safe values
        self
    }

    /// Set network port
    pub fn with_port(mut self, port: u16) -> Self {
        self.config.system.api_port = port;
        self
    }

    /// Use random available port (good for parallel tests)
    pub fn with_random_port(mut self) -> Self {
        use std::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
        let port = listener
            .local_addr()
            .expect("Failed to get local addr")
            .port();
        drop(listener);
        self.config.system.api_port = port;
        self
    }

    /// Build the configuration
    pub fn build(self) -> CanonicalMasterConfig {
        self.config
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_default() {
        let config = ConfigBuilder::new().build();
        assert!(config.is_valid());
    }

    #[test]
    fn test_config_builder_for_testing() {
        let config = ConfigBuilder::new().for_testing().build();
        assert_eq!(config.environment, Environment::Testing);
    }

    #[test]
    fn test_config_builder_with_port() {
        let config = ConfigBuilder::new().with_port(9000).build();
        assert_eq!(config.system.api_port, 9000);
    }

    #[test]
    fn test_config_builder_random_port() {
        let config1 = ConfigBuilder::new().with_random_port().build();
        let config2 = ConfigBuilder::new().with_random_port().build();
        // Random ports should be different (with very high probability)
        assert_ne!(config1.system.api_port, config2.system.api_port);
    }
}
