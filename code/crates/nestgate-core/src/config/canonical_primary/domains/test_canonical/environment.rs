// **TEST ENVIRONMENT CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for TestEnvironment
pub struct TestEnvironmentConfig {
    /// Infrastructure
    pub infrastructure: TestInfrastructureConfig,
    /// Resources
    pub resources: TestResourceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TestInfrastructure
pub struct TestInfrastructureConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TestResource
pub struct TestResourceConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for TestInfrastructureConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl Default for TestResourceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl TestEnvironmentConfig {
    /// Creates a CI-optimized test environment configuration
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }
    /// Creates a development-optimized test environment configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Merges this configuration with another, taking precedence
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
}
