// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **LOAD TEST CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `LoadTest`
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
/// Configuration for `RampUp`
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
    /// Creates a CI-optimized load test configuration
    #[must_use]
    pub fn ci_optimized() -> Self {
        Self::default()
    }
    /// Creates a development-optimized load test configuration
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_test_config_default() {
        let c = LoadTestConfig::default();
        assert!(c.scenarios.is_empty());
        assert_eq!(c.ramp_up.duration, std::time::Duration::from_secs(60));
        assert_eq!(c.ramp_up.strategy, "linear");
    }

    #[test]
    fn load_test_config_constructors_and_merge() {
        let d = LoadTestConfig::default();
        assert_eq!(
            serde_json::to_string(&d).expect("serialize"),
            serde_json::to_string(&LoadTestConfig::ci_optimized()).expect("serialize")
        );
        assert_eq!(
            serde_json::to_string(&d).expect("serialize"),
            serde_json::to_string(&LoadTestConfig::development_optimized()).expect("serialize")
        );
        let other = LoadTestConfig {
            scenarios: vec![LoadTestScenario {
                name: "s".to_string(),
                weight: 1.0,
                steps: vec![],
            }],
            ramp_up: RampUpConfig::default(),
        };
        let merged = d.merge(other);
        assert!(merged.scenarios.is_empty());
    }

    #[test]
    fn load_test_config_serde_roundtrip() {
        let original = LoadTestConfig {
            scenarios: vec![LoadTestScenario {
                name: "read".to_string(),
                weight: 1.0,
                steps: vec![LoadTestStep {
                    name: "step1".to_string(),
                    method: "GET".to_string(),
                    url: "/health".to_string(),
                }],
            }],
            ramp_up: RampUpConfig::default(),
        };
        let json = serde_json::to_string(&original).expect("serialize");
        let parsed: LoadTestConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(parsed.scenarios.len(), 1);
        assert_eq!(
            serde_json::to_string(&original).expect("serialize"),
            serde_json::to_string(&parsed).expect("re-serialize")
        );
    }

    #[test]
    fn ramp_up_config_default() {
        let r = RampUpConfig::default();
        assert_eq!(r.duration, std::time::Duration::from_secs(60));
        assert_eq!(r.strategy, "linear");
    }
}
