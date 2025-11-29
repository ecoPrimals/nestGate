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
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
}
