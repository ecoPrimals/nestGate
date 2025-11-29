// **LOAD TEST CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for LoadTest
pub struct LoadTestConfig {
    /// Scenarios
    pub scenarios: Vec<LoadTestScenario>,
    /// Ramp Up
    pub ramp_up: RampUpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadtestscenario
pub struct LoadTestScenario {
    /// Name
    pub name: String,
    /// Weight
    pub weight: f64,
    /// Steps
    pub steps: Vec<LoadTestStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadteststep
pub struct LoadTestStep {
    /// Name
    pub name: String,
    /// Method
    pub method: String,
    /// Url
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for RampUp
pub struct RampUpConfig {
    /// Duration
    pub duration: Duration,
    /// Strategy
    pub strategy: String,
}

impl Default for RampUpConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(60),
            strategy: "linear".to_string(),
        }
    }
}

impl LoadTestConfig {
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
