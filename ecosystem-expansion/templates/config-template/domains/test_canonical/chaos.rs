//! **CHAOS TEST CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ChaosTestConfig {
    pub experiments: ChaosExperimentConfig,
    pub injection: FailureInjectionConfig,
    pub resilience: ResilienceTestConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ChaosExperimentConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct FailureInjectionConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ResilienceTestConfig {
    pub enabled: bool,
}





impl ChaosTestConfig {
    pub fn ci_optimized() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
} 