// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Integration tests for load testing configuration.

use super::config::*;
use nestgate_core::error::{ErrorCategory, SafeUnwrap};
use nestgate_core::Result;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_test_config_default() {
        let config = LoadTestConfig::default();

        assert_eq!(config.duration_seconds, 60);
        assert_eq!(config.concurrent_users, 10);
        assert_eq!(config.requests_per_second, 1.0);
        assert_eq!(config.endpoints, vec!["/health"]);
        assert!(matches!(config.scenario, TestScenario::ConstantLoad));
    }

    #[test]
    fn test_load_test_config_custom() {
        let mut config = LoadTestConfig::default();
        config.duration_seconds = 300;
        config.concurrent_users = 100;
        config.requests_per_second = 10.0;
        config.endpoints = vec!["/api/users".to_string(), "/api/posts".to_string()];

        assert_eq!(config.duration_seconds, 300);
        assert_eq!(config.concurrent_users, 100);
        assert_eq!(config.requests_per_second, 10.0);
        assert_eq!(config.endpoints.len(), 2);
    }

    #[test]
    fn test_test_scenario_constant_load() {
        let scenario = TestScenario::ConstantLoad;

        // Verify variant can be created and matched
        match scenario {
            TestScenario::ConstantLoad => (),
            _ => panic!("Expected ConstantLoad variant"),
        }
    }

    #[test]
    fn test_test_scenario_ramp() {
        let scenario = TestScenario::Ramp {
            start_users: 10,
            end_users: 100,
            ramp_duration_seconds: 60,
        };

        match scenario {
            TestScenario::Ramp {
                start_users,
                end_users,
                ramp_duration_seconds,
            } => {
                assert_eq!(start_users, 10);
                assert_eq!(end_users, 100);
                assert_eq!(ramp_duration_seconds, 60);
            }
            _ => panic!("Expected Ramp variant"),
        }
    }

    #[test]
    fn test_test_scenario_spike() {
        let scenario = TestScenario::Spike {
            baseline_users: 20,
            spike_users: 200,
            spike_duration_seconds: 30,
        };

        match scenario {
            TestScenario::Spike {
                baseline_users,
                spike_users,
                spike_duration_seconds,
            } => {
                assert_eq!(baseline_users, 20);
                assert_eq!(spike_users, 200);
                assert_eq!(spike_duration_seconds, 30);
            }
            _ => panic!("Expected Spike variant"),
        }
    }

    #[test]
    fn test_test_scenario_step() {
        let scenario = TestScenario::Step {
            max_users: 500,
            step_users: 50,
            step_duration_seconds: 60,
        };

        match scenario {
            TestScenario::Step {
                max_users,
                step_users,
                step_duration_seconds,
            } => {
                assert_eq!(max_users, 500);
                assert_eq!(step_users, 50);
                assert_eq!(step_duration_seconds, 60);
            }
            _ => panic!("Expected Step variant"),
        }
    }

    #[test]
    fn test_load_test_parameters() {
        let config = LoadTestConfig::default();
        let params = LoadTestParameters {
            config: config.clone(),
            started_at: Some(std::time::SystemTime::now()),
            test_id: "test_123".to_string(),
        };

        assert_eq!(params.test_id, "test_123");
        assert!(params.started_at.is_some());
        assert_eq!(params.config.duration_seconds, config.duration_seconds);
    }

    #[test]
    fn test_test_data_config_default() {
        let data_config = TestDataConfig::default();

        assert_eq!(data_config.payload_size_bytes, 1024);
        assert!(data_config.expected_response_size_bytes.is_none());
        assert!(data_config.custom_headers.is_empty());
        assert!(data_config.body_template.is_none());
    }

    #[test]
    fn test_test_data_config_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let data_config = TestDataConfig {
            payload_size_bytes: 2048,
            expected_response_size_bytes: Some(1024),
            custom_headers: headers.clone(),
            body_template: Some(r#"{"test": "data"}"#.to_string()),
        };

        assert_eq!(data_config.payload_size_bytes, 2048);
        assert_eq!(data_config.expected_response_size_bytes, Some(1024));
        assert_eq!(data_config.custom_headers.len(), 2);
        assert!(data_config.body_template.is_some());
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
    fn test_performance_thresholds_custom() {
        let thresholds = PerformanceThresholds {
            max_avg_response_time_ms: 500.0,
            max_p95_response_time_ms: 1000.0,
            min_success_rate: 0.99,
            max_error_rate: 0.01,
        };

        assert_eq!(thresholds.max_avg_response_time_ms, 500.0);
        assert_eq!(thresholds.max_p95_response_time_ms, 1000.0);
        assert_eq!(thresholds.min_success_rate, 0.99);
        assert_eq!(thresholds.max_error_rate, 0.01);
    }

    #[test]
    fn test_load_test_config_serialization() -> Result<()> {
        let config = LoadTestConfig::default();

        let json = serde_json::to_string(&config).safe_unwrap(
            ErrorCategory::Validation,
            "Failed to serialize LoadTestConfig",
        )?;

        assert!(json.contains("\"duration_seconds\":60"));
        assert!(json.contains("\"concurrent_users\":10"));

        Ok(())
    }

    #[test]
    fn test_load_test_config_deserialization() -> Result<()> {
        let json = r#"{
            "duration_seconds": 180,
            "concurrent_users": 50,
            "requests_per_second": 5.0,
            "scenario": "ConstantLoad",
            "endpoints": ["/api/test"],
            "test_data": {
                "payload_size_bytes": 512,
                "expected_response_size_bytes": null,
                "custom_headers": {},
                "body_template": null
            },
            "thresholds": {
                "max_avg_response_time_ms": 800.0,
                "max_p95_response_time_ms": 1500.0,
                "min_success_rate": 0.98,
                "max_error_rate": 0.02
            }
        }"#;

        let config: LoadTestConfig = serde_json::from_str(json).safe_unwrap(
            ErrorCategory::Validation,
            "Failed to deserialize LoadTestConfig",
        )?;

        assert_eq!(config.duration_seconds, 180);
        assert_eq!(config.concurrent_users, 50);
        assert_eq!(config.requests_per_second, 5.0);

        Ok(())
    }

    #[test]
    fn test_test_scenario_serialization() -> Result<()> {
        let scenarios = vec![
            TestScenario::ConstantLoad,
            TestScenario::Ramp {
                start_users: 5,
                end_users: 50,
                ramp_duration_seconds: 120,
            },
            TestScenario::Spike {
                baseline_users: 10,
                spike_users: 100,
                spike_duration_seconds: 60,
            },
            TestScenario::Step {
                max_users: 200,
                step_users: 20,
                step_duration_seconds: 30,
            },
        ];

        for scenario in scenarios {
            let json = serde_json::to_string(&scenario).safe_unwrap(
                ErrorCategory::Validation,
                "Failed to serialize TestScenario",
            )?;

            assert!(!json.is_empty(), "Serialized JSON should not be empty");
        }

        Ok(())
    }

    #[test]
    fn test_load_test_execution() {
        let config = LoadTestConfig::default();
        let now = std::time::SystemTime::now();

        let execution = LoadTestExecution {
            config: config.clone(),
            started_at: Some(now),
            test_id: "exec_001".to_string(),
        };

        assert_eq!(execution.test_id, "exec_001");
        assert!(execution.started_at.is_some());
        assert_eq!(execution.config.duration_seconds, 60);
    }

    #[test]
    fn test_load_test_config_clone() {
        let config = LoadTestConfig::default();
        let cloned = config.clone();

        assert_eq!(config.duration_seconds, cloned.duration_seconds);
        assert_eq!(config.concurrent_users, cloned.concurrent_users);
        assert_eq!(config.requests_per_second, cloned.requests_per_second);
        assert_eq!(config.endpoints, cloned.endpoints);
    }

    #[test]
    fn test_multiple_endpoints() {
        let endpoints = vec![
            "/api/users".to_string(),
            "/api/posts".to_string(),
            "/api/comments".to_string(),
            "/health".to_string(),
        ];

        let mut config = LoadTestConfig::default();
        config.endpoints = endpoints.clone();

        assert_eq!(config.endpoints.len(), 4);
        assert_eq!(config.endpoints, endpoints);
    }

    #[test]
    fn test_performance_thresholds_validation() {
        let thresholds = PerformanceThresholds {
            max_avg_response_time_ms: 100.0,
            max_p95_response_time_ms: 200.0,
            min_success_rate: 0.95,
            max_error_rate: 0.05,
        };

        // Verify that min_success_rate + max_error_rate = 1.0
        assert!((thresholds.min_success_rate + thresholds.max_error_rate - 1.0).abs() < 0.001);

        // Verify P95 is greater than average
        assert!(thresholds.max_p95_response_time_ms > thresholds.max_avg_response_time_ms);
    }

    #[test]
    fn test_ramp_scenario_validation() {
        let scenario = TestScenario::Ramp {
            start_users: 10,
            end_users: 100,
            ramp_duration_seconds: 60,
        };

        if let TestScenario::Ramp {
            start_users,
            end_users,
            ramp_duration_seconds: _,
        } = scenario
        {
            assert!(
                end_users > start_users,
                "End users should be greater than start users"
            );
        }
    }

    #[test]
    fn test_spike_scenario_validation() {
        let scenario = TestScenario::Spike {
            baseline_users: 20,
            spike_users: 200,
            spike_duration_seconds: 30,
        };

        if let TestScenario::Spike {
            baseline_users,
            spike_users,
            spike_duration_seconds: _,
        } = scenario
        {
            assert!(
                spike_users > baseline_users,
                "Spike users should be greater than baseline"
            );
        }
    }

    #[test]
    fn test_step_scenario_validation() {
        let scenario = TestScenario::Step {
            max_users: 500,
            step_users: 50,
            step_duration_seconds: 60,
        };

        if let TestScenario::Step {
            max_users,
            step_users,
            step_duration_seconds: _,
        } = scenario
        {
            assert!(
                max_users % step_users == 0,
                "Max users should be divisible by step users for clean steps"
            );
        }
    }
}
