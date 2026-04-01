// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **COMPREHENSIVE LOAD TESTING TESTS**
//!
//! Test coverage for load_testing module to increase overall coverage.
//! These tests cover configuration, metrics, scenarios, and test execution.

#[cfg(test)]
mod tests {
    use super::super::config::*;
    use super::super::metrics::*;
    use super::super::scenarios::*;
    use std::collections::HashMap;

    // ==================== LOAD TEST CONFIG TESTS ====================

    #[test]
    fn test_load_test_config_creation() {
        let config = LoadTestConfig {
            duration_seconds: 120,
            concurrent_users: 50,
            requests_per_second: 10.0,
            scenario: TestScenario::ConstantLoad,
            endpoints: vec!["/api/test".to_string()],
            test_data: TestDataConfig::default(),
            thresholds: PerformanceThresholds::default(),
        };

        assert_eq!(config.duration_seconds, 120);
        assert_eq!(config.concurrent_users, 50);
        assert_eq!(config.requests_per_second, 10.0);
    }

    #[test]
    fn test_load_test_config_default() {
        let config = LoadTestConfig::default();

        assert_eq!(config.duration_seconds, 60);
        assert_eq!(config.concurrent_users, 10);
        assert_eq!(config.requests_per_second, 1.0);
        assert_eq!(config.endpoints, vec!["/health".to_string()]);
    }

    #[test]
    fn test_load_test_config_serialization() {
        let config = LoadTestConfig::default();

        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok(), "Config should serialize");

        let json = serialized.expect("Test setup failed");
        assert!(json.contains("duration_seconds"));
        assert!(json.contains("concurrent_users"));
    }

    #[test]
    fn test_load_test_config_deserialization() {
        let json = r#"{
            "duration_seconds": 90,
            "concurrent_users": 25,
            "requests_per_second": 5.0,
            "scenario": "ConstantLoad",
            "endpoints": ["/api/health"],
            "test_data": {
                "payload_size_bytes": 1024,
                "expected_response_size_bytes": null,
                "custom_headers": {},
                "body_template": null
            },
            "thresholds": {
                "max_avg_response_time_ms": 1000.0,
                "max_p95_response_time_ms": 2000.0,
                "min_success_rate": 0.95,
                "max_error_rate": 0.05
            }
        }"#;

        let config: Result<LoadTestConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok(), "Should deserialize successfully");

        let config = config.expect("Test setup failed");
        assert_eq!(config.duration_seconds, 90);
        assert_eq!(config.concurrent_users, 25);
    }

    // ==================== TEST SCENARIO TESTS ====================

    #[test]
    fn test_constant_load_scenario() {
        let scenario = TestScenario::ConstantLoad;

        let serialized = serde_json::to_string(&scenario);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_ramp_scenario() {
        let scenario = TestScenario::Ramp {
            start_users: 10,
            end_users: 100,
            ramp_duration_seconds: 300,
        };

        if let TestScenario::Ramp {
            start_users,
            end_users,
            ramp_duration_seconds,
        } = scenario
        {
            assert_eq!(start_users, 10);
            assert_eq!(end_users, 100);
            assert_eq!(ramp_duration_seconds, 300);
        } else {
            panic!("Expected Ramp scenario");
        }
    }

    #[test]
    fn test_spike_scenario() {
        let scenario = TestScenario::Spike {
            baseline_users: 20,
            spike_users: 200,
            spike_duration_seconds: 60,
        };

        if let TestScenario::Spike {
            baseline_users,
            spike_users,
            spike_duration_seconds,
        } = scenario
        {
            assert_eq!(baseline_users, 20);
            assert_eq!(spike_users, 200);
            assert_eq!(spike_duration_seconds, 60);
        } else {
            panic!("Expected Spike scenario");
        }
    }

    #[test]
    fn test_step_scenario() {
        let scenario = TestScenario::Step {
            max_users: 100,
            step_users: 10,
            step_duration_seconds: 30,
        };

        if let TestScenario::Step {
            max_users,
            step_users,
            step_duration_seconds,
        } = scenario
        {
            assert_eq!(max_users, 100);
            assert_eq!(step_users, 10);
            assert_eq!(step_duration_seconds, 30);
        } else {
            panic!("Expected Step scenario");
        }
    }

    #[test]
    fn test_scenario_serialization() {
        let scenarios = vec![
            TestScenario::ConstantLoad,
            TestScenario::Ramp {
                start_users: 1,
                end_users: 10,
                ramp_duration_seconds: 60,
            },
            TestScenario::Spike {
                baseline_users: 5,
                spike_users: 50,
                spike_duration_seconds: 30,
            },
            TestScenario::Step {
                max_users: 100,
                step_users: 10,
                step_duration_seconds: 20,
            },
        ];

        for scenario in scenarios {
            let serialized = serde_json::to_string(&scenario);
            assert!(serialized.is_ok(), "Scenario should serialize");
        }
    }

    // ==================== TEST DATA CONFIG TESTS ====================

    #[test]
    fn test_test_data_config_creation() {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token".to_string());

        let config = TestDataConfig {
            payload_size_bytes: 2048,
            expected_response_size_bytes: Some(1024),
            custom_headers: headers,
            body_template: Some("{{payload}}".to_string()),
        };

        assert_eq!(config.payload_size_bytes, 2048);
        assert_eq!(config.expected_response_size_bytes, Some(1024));
        assert_eq!(config.custom_headers.len(), 1);
    }

    #[test]
    fn test_test_data_config_default() {
        let config = TestDataConfig::default();

        assert_eq!(config.payload_size_bytes, 1024);
        assert_eq!(config.expected_response_size_bytes, None);
        assert!(config.custom_headers.is_empty());
        assert_eq!(config.body_template, None);
    }

    #[test]
    fn test_test_data_config_serialization() {
        let config = TestDataConfig::default();

        let serialized = serde_json::to_string(&config);
        assert!(serialized.is_ok());
    }

    // ==================== PERFORMANCE THRESHOLDS TESTS ====================

    #[test]
    fn test_performance_thresholds_creation() {
        let thresholds = PerformanceThresholds {
            max_avg_response_time_ms: 500.0,
            max_p95_response_time_ms: 1500.0,
            min_success_rate: 0.99,
            max_error_rate: 0.01,
        };

        assert_eq!(thresholds.max_avg_response_time_ms, 500.0);
        assert_eq!(thresholds.min_success_rate, 0.99);
        assert_eq!(thresholds.max_error_rate, 0.01);
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
    fn test_performance_thresholds_strict() {
        let strict = PerformanceThresholds {
            max_avg_response_time_ms: 100.0,
            max_p95_response_time_ms: 300.0,
            min_success_rate: 0.999,
            max_error_rate: 0.001,
        };

        assert!(strict.max_avg_response_time_ms < 200.0);
        assert!(strict.min_success_rate > 0.99);
    }

    // ==================== LOAD TEST PARAMETERS TESTS ====================

    #[test]
    fn test_load_test_parameters_creation() {
        let params = LoadTestParameters {
            config: LoadTestConfig::default(),
            started_at: Some(std::time::SystemTime::now()),
            test_id: "test_123".to_string(),
        };

        assert_eq!(params.test_id, "test_123");
        assert!(params.started_at.is_some());
    }

    #[test]
    fn test_load_test_parameters_no_start_time() {
        let params = LoadTestParameters {
            config: LoadTestConfig::default(),
            started_at: None,
            test_id: "test_pending".to_string(),
        };

        assert!(params.started_at.is_none());
    }

    #[test]
    fn test_load_test_parameters_serialization() {
        let params = LoadTestParameters {
            config: LoadTestConfig::default(),
            started_at: Some(std::time::SystemTime::now()),
            test_id: "test_456".to_string(),
        };

        let serialized = serde_json::to_string(&params);
        assert!(serialized.is_ok());
    }

    // ==================== LOAD TEST EXECUTION TESTS ====================

    #[test]
    fn test_load_test_execution_creation() {
        let execution = LoadTestExecution {
            config: LoadTestConfig::default(),
            started_at: Some(std::time::SystemTime::now()),
            test_id: "exec_001".to_string(),
        };

        assert_eq!(execution.test_id, "exec_001");
        assert!(execution.started_at.is_some());
    }

    #[test]
    fn test_load_test_execution_serialization() {
        let execution = LoadTestExecution {
            config: LoadTestConfig::default(),
            started_at: None,
            test_id: "exec_002".to_string(),
        };

        let serialized = serde_json::to_string(&execution);
        assert!(serialized.is_ok());
    }

    // ==================== PERFORMANCE STATS TESTS ====================

    #[test]
    fn test_performance_stats_creation() {
        let stats = PerformanceStats {
            total_requests: 10000,
            successful_requests: 9500,
            failed_requests: 500,
            requests_per_second: 166.67,
        };

        assert_eq!(stats.total_requests, 10000);
        assert_eq!(stats.successful_requests, 9500);
        assert_eq!(stats.failed_requests, 500);
    }

    #[test]
    fn test_performance_stats_default() {
        let stats = PerformanceStats::default();

        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, 0);
        assert_eq!(stats.requests_per_second, 0.0);
    }

    #[test]
    fn test_performance_stats_success_rate() {
        let stats = PerformanceStats {
            total_requests: 1000,
            successful_requests: 950,
            failed_requests: 50,
            requests_per_second: 10.0,
        };

        let success_rate = stats.successful_requests as f64 / stats.total_requests as f64;
        assert!((success_rate - 0.95).abs() < 0.01);
    }

    #[test]
    fn test_performance_stats_serialization() {
        let stats = PerformanceStats::default();

        let serialized = serde_json::to_string(&stats);
        assert!(serialized.is_ok());
    }

    // ==================== RESPONSE TIME STATS TESTS ====================

    #[test]
    fn test_response_time_stats_creation() {
        let stats = ResponseTimeStats {
            min_ms: 10.5,
            max_ms: 500.3,
            avg_ms: 125.8,
            p50_ms: 100.0,
            p95_ms: 350.0,
            p99_ms: 475.0,
        };

        assert_eq!(stats.min_ms, 10.5);
        assert_eq!(stats.max_ms, 500.3);
        assert_eq!(stats.avg_ms, 125.8);
    }

    #[test]
    fn test_response_time_stats_default() {
        let stats = ResponseTimeStats::default();

        assert_eq!(stats.min_ms, 0.0);
        assert_eq!(stats.max_ms, 0.0);
        assert_eq!(stats.avg_ms, 0.0);
        assert_eq!(stats.p50_ms, 0.0);
        assert_eq!(stats.p95_ms, 0.0);
        assert_eq!(stats.p99_ms, 0.0);
    }

    #[test]
    fn test_response_time_stats_percentiles() {
        let stats = ResponseTimeStats {
            min_ms: 50.0,
            max_ms: 1000.0,
            avg_ms: 200.0,
            p50_ms: 150.0,
            p95_ms: 800.0,
            p99_ms: 950.0,
        };

        assert!(stats.p50_ms < stats.p95_ms);
        assert!(stats.p95_ms < stats.p99_ms);
        assert!(stats.min_ms < stats.avg_ms);
        assert!(stats.avg_ms < stats.max_ms);
    }

    #[test]
    fn test_response_time_stats_serialization() {
        let stats = ResponseTimeStats::default();

        let serialized = serde_json::to_string(&stats);
        assert!(serialized.is_ok());
    }

    // ==================== LOAD TEST METRICS TESTS ====================

    #[test]
    fn test_load_test_metrics_creation() {
        let metrics = LoadTestMetrics {
            performance_stats: PerformanceStats {
                total_requests: 5000,
                successful_requests: 4800,
                failed_requests: 200,
                requests_per_second: 83.33,
            },
            response_time_stats: ResponseTimeStats {
                min_ms: 20.0,
                max_ms: 600.0,
                avg_ms: 150.0,
                p50_ms: 120.0,
                p95_ms: 400.0,
                p99_ms: 550.0,
            },
        };

        assert_eq!(metrics.performance_stats.total_requests, 5000);
        assert_eq!(metrics.response_time_stats.avg_ms, 150.0);
    }

    #[test]
    fn test_load_test_metrics_default() {
        let metrics = LoadTestMetrics::default();

        assert_eq!(metrics.performance_stats.total_requests, 0);
        assert_eq!(metrics.response_time_stats.avg_ms, 0.0);
    }

    #[test]
    fn test_load_test_metrics_serialization() {
        let metrics = LoadTestMetrics::default();

        let serialized = serde_json::to_string(&metrics);
        assert!(serialized.is_ok());
    }

    // ==================== TEST RESULT TESTS ====================

    #[test]
    fn test_test_result_creation() {
        let result = TestResult {
            success: true,
            duration_seconds: 120,
            total_requests: 12000,
            successful_requests: 11400,
            failed_requests: 600,
            avg_response_time_ms: 145.3,
        };

        assert!(result.success);
        assert_eq!(result.duration_seconds, 120);
        assert_eq!(result.total_requests, 12000);
    }

    #[test]
    fn test_test_result_failure() {
        let result = TestResult {
            success: false,
            duration_seconds: 60,
            total_requests: 1000,
            successful_requests: 500,
            failed_requests: 500,
            avg_response_time_ms: 2500.0,
        };

        assert!(!result.success);
        let error_rate = result.failed_requests as f64 / result.total_requests as f64;
        assert!((error_rate - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_test_result_serialization() {
        let result = TestResult {
            success: true,
            duration_seconds: 90,
            total_requests: 9000,
            successful_requests: 8550,
            failed_requests: 450,
            avg_response_time_ms: 120.5,
        };

        let serialized = serde_json::to_string(&result);
        assert!(serialized.is_ok());

        let json = serialized.expect("Test setup failed");
        assert!(json.contains("\"success\":true"));
    }

    // ==================== SCENARIO RUNNER TESTS ====================

    #[test]
    fn test_scenario_runner_creation() {
        let config = LoadTestConfig::default();
        let runner = ScenarioRunner::new(config);

        // Runner created successfully
        assert!(std::mem::size_of_val(&runner) > 0);
    }

    #[test]
    fn test_scenario_runner_run() {
        let config = LoadTestConfig {
            duration_seconds: 30,
            concurrent_users: 5,
            requests_per_second: 2.0,
            scenario: TestScenario::ConstantLoad,
            endpoints: vec!["/test".to_string()],
            test_data: TestDataConfig::default(),
            thresholds: PerformanceThresholds::default(),
        };

        let runner = ScenarioRunner::new(config);
        let result = runner.run();

        assert!(result.is_ok(), "Runner should execute successfully");
        let result = result.expect("Test setup failed");
        assert!(result.success);
        assert_eq!(result.duration_seconds, 30);
    }

    // ==================== EDGE CASES ====================

    #[test]
    fn test_zero_duration_config() {
        let config = LoadTestConfig {
            duration_seconds: 0,
            concurrent_users: 10,
            requests_per_second: 1.0,
            scenario: TestScenario::ConstantLoad,
            endpoints: vec![],
            test_data: TestDataConfig::default(),
            thresholds: PerformanceThresholds::default(),
        };

        assert_eq!(config.duration_seconds, 0);
    }

    #[test]
    fn test_high_concurrency_config() {
        let config = LoadTestConfig {
            duration_seconds: 60,
            concurrent_users: 10000,
            requests_per_second: 1000.0,
            scenario: TestScenario::ConstantLoad,
            endpoints: vec!["/api/endpoint".to_string()],
            test_data: TestDataConfig::default(),
            thresholds: PerformanceThresholds::default(),
        };

        assert_eq!(config.concurrent_users, 10000);
        assert_eq!(config.requests_per_second, 1000.0);
    }

    #[test]
    fn test_extreme_ramp_scenario() {
        let scenario = TestScenario::Ramp {
            start_users: 1,
            end_users: 100000,
            ramp_duration_seconds: 3600,
        };

        if let TestScenario::Ramp {
            start_users,
            end_users,
            ..
        } = scenario
        {
            assert_eq!(start_users, 1);
            assert_eq!(end_users, 100000);
        }
    }

    #[test]
    fn test_performance_stats_all_failures() {
        let stats = PerformanceStats {
            total_requests: 100,
            successful_requests: 0,
            failed_requests: 100,
            requests_per_second: 10.0,
        };

        assert_eq!(stats.successful_requests, 0);
        assert_eq!(stats.failed_requests, stats.total_requests);
    }

    #[test]
    fn test_response_time_stats_identical_values() {
        let stats = ResponseTimeStats {
            min_ms: 100.0,
            max_ms: 100.0,
            avg_ms: 100.0,
            p50_ms: 100.0,
            p95_ms: 100.0,
            p99_ms: 100.0,
        };

        assert_eq!(stats.min_ms, stats.max_ms);
        assert_eq!(stats.avg_ms, stats.p50_ms);
    }
}
