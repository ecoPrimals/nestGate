// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Load testing configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Load testing configuration for API handler–level load tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestConfig {
    /// Test duration in seconds
    pub duration_seconds: u64,
    /// Number of concurrent users
    pub concurrent_users: u32,
    /// Requests per second per user
    pub requests_per_second: f64,
    /// Test scenario type
    pub scenario: TestScenario,
    /// Target endpoints to test
    pub endpoints: Vec<String>,
    /// Test data parameters
    pub test_data: TestDataConfig,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
}

/// Test scenario types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Testscenario
pub enum TestScenario {
    /// **CONSTANT LOAD**
    ///
    /// Configuration for constant load testing.
    ConstantLoad,
    /// **RAMP CONFIGURATION**
    ///
    /// Configuration for gradual load increase testing.
    Ramp {
        /// Starting number of concurrent users
        start_users: u32,
        /// Ending number of concurrent users
        end_users: u32,
        /// Duration for ramping up load in seconds
        ramp_duration_seconds: u64,
    },
    /// **SPIKE CONFIGURATION**
    ///
    /// Configuration for sudden load spike testing.
    Spike {
        /// Baseline number of users before spike
        baseline_users: u32,
        /// Peak number of users during spike
        spike_users: u32,
        /// Duration of the spike in seconds
        spike_duration_seconds: u64,
    },
    /// **STEP CONFIGURATION**
    ///
    /// Configuration for stepped load increase testing.
    Step {
        /// Maximum number of users to reach
        max_users: u32,
        /// Number of users to add in each step
        step_users: u32,
        /// Duration of each step in seconds
        step_duration_seconds: u64,
    },
}

/// Load test parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadtestparameters
pub struct LoadTestParameters {
    /// Load test configuration settings
    pub config: LoadTestConfig,
    /// Timestamp when the test was started, if running
    pub started_at: Option<std::time::SystemTime>,
    /// Unique identifier for this test run
    pub test_id: String,
}

/// Test data configuration for load tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestDataConfig {
    /// Request payload size in bytes
    pub payload_size_bytes: usize,
    /// Response size expectations
    pub expected_response_size_bytes: Option<usize>,
    /// Custom headers to include
    pub custom_headers: HashMap<String, String>,
    /// Request body template
    pub body_template: Option<String>,
}

/// Performance thresholds for pass/fail criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancethresholds
pub struct PerformanceThresholds {
    /// Maximum acceptable average response time (ms)
    pub max_avg_response_time_ms: f64,
    /// Maximum acceptable 95th percentile response time (ms)
    pub max_p95_response_time_ms: f64,
    /// Minimum acceptable success rate (0.0 - 1.0)
    pub min_success_rate: f64,
    /// Maximum acceptable error rate (0.0 - 1.0)
    pub max_error_rate: f64,
}

/// **LOAD TEST EXECUTION**
///
/// Represents an active load test execution with configuration and timing.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Loadtestexecution
pub struct LoadTestExecution {
    /// Load test configuration parameters
    pub config: LoadTestConfig,
    /// Timestamp when the test execution started
    pub started_at: Option<std::time::SystemTime>,
    /// Unique identifier for this test execution
    pub test_id: String,
}

impl Default for LoadTestConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            duration_seconds: 60,
            concurrent_users: 10,
            requests_per_second: 1.0,
            scenario: TestScenario::ConstantLoad,
            endpoints: vec!["/health".to_string()],
            test_data: TestDataConfig::default(),
            thresholds: PerformanceThresholds::default(),
        }
    }
}

impl Default for TestDataConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            payload_size_bytes: 1024,
            expected_response_size_bytes: None,
            custom_headers: HashMap::new(),
            body_template: None,
        }
    }
}

impl Default for PerformanceThresholds {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            max_avg_response_time_ms: 1000.0,
            max_p95_response_time_ms: 2000.0,
            min_success_rate: 0.95,
            max_error_rate: 0.05,
        }
    }
}

/// Canonical name for [`LoadTestConfig`] in the handlers load-testing module.
pub type LoadTestConfigCanonical = LoadTestConfig;

/// Canonical name for [`TestDataConfig`] in the handlers load-testing module.
pub type TestDataConfigCanonical = TestDataConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_test_config_default() {
        let config = LoadTestConfig::default();
        assert_eq!(config.duration_seconds, 60);
        assert_eq!(config.concurrent_users, 10);
        assert_eq!(config.requests_per_second, 1.0);
        assert_eq!(config.endpoints.len(), 1);
        assert_eq!(config.endpoints[0], "/health");
    }

    #[test]
    fn test_test_data_config_default() {
        let config = TestDataConfig::default();
        assert_eq!(config.payload_size_bytes, 1024);
        assert!(config.expected_response_size_bytes.is_none());
        assert!(config.custom_headers.is_empty());
        assert!(config.body_template.is_none());
    }

    #[test]
    fn test_performance_thresholds_default() {
        let thresholds = PerformanceThresholds::default();
        assert_eq!(thresholds.max_avg_response_time_ms, 1000.0);
        assert_eq!(thresholds.max_p95_response_time_ms, 2000.0);
        assert_eq!(thresholds.min_success_rate, 0.95);
        assert_eq!(thresholds.max_error_rate, 0.05);
    }

    #[test]
    fn test_load_test_config_serialization() {
        let config = LoadTestConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
    }

    #[test]
    fn test_test_scenario_constant_load() {
        let scenario = TestScenario::ConstantLoad;
        let json = serde_json::to_string(&scenario);
        assert!(json.is_ok());
    }

    #[test]
    fn test_test_scenario_ramp() {
        let scenario = TestScenario::Ramp {
            start_users: 10,
            end_users: 100,
            ramp_duration_seconds: 300,
        };
        let json = serde_json::to_string(&scenario);
        assert!(json.is_ok());
    }

    #[test]
    fn test_test_scenario_spike() {
        let scenario = TestScenario::Spike {
            baseline_users: 50,
            spike_users: 500,
            spike_duration_seconds: 60,
        };
        let json = serde_json::to_string(&scenario);
        assert!(json.is_ok());
    }

    #[test]
    fn test_test_scenario_step() {
        let scenario = TestScenario::Step {
            max_users: 200,
            step_users: 20,
            step_duration_seconds: 60,
        };
        let json = serde_json::to_string(&scenario);
        assert!(json.is_ok());
    }

    #[test]
    fn test_load_test_execution() {
        let execution = LoadTestExecution {
            config: LoadTestConfig::default(),
            started_at: Some(std::time::SystemTime::now()),
            test_id: "test_456".to_string(),
        };

        assert_eq!(execution.test_id, "test_456");
        assert!(execution.started_at.is_some());
    }
}
