// **LOAD TEST CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadTestConfig {
    pub scenarios: Vec<LoadTestScenario>,
    pub ramp_up: RampUpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestScenario {
    pub name: String,
    pub weight: f64,
    pub steps: Vec<LoadTestStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestStep {
    pub name: String,
    pub method: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RampUpConfig {
    pub duration: Duration,
    pub strategy: String,
}

impl Default for RampUpConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(60),
            strategy: "linear".to_string(),
        }
    }
}

impl LoadTestConfig {
    #[must_use]
    pub const fn ci_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
}
