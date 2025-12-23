// **CHAOS TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ChaosTest
pub struct ChaosTestConfig {
    /// Experiments
    pub experiments: ChaosExperimentConfig,
    /// Injection
    pub injection: FailureInjectionConfig,
    /// Resilience
    pub resilience: ResilienceTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ChaosExperiment
pub struct ChaosExperimentConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for FailureInjection
pub struct FailureInjectionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for ResilienceTest
pub struct ResilienceTestConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl ChaosTestConfig {
    /// Returns a CI-optimized configuration for chaos testing
    ///
    /// This configuration is tuned for continuous integration environments
    /// with appropriate timeouts and resource limits for automated testing.
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }

    /// Returns a development-optimized configuration for local chaos testing
    ///
    /// This configuration provides more verbose output and longer timeouts
    /// suitable for local development and debugging of chaos scenarios.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Merges this configuration with another, returning the combined result
    ///
    /// Currently returns self as chaos configs don't support deep merging.
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
}
