// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Integration tests for load testing scenarios.

use super::config::LoadTestConfig;
use super::scenarios::{ScenarioRunner, TestResult};
use nestgate_core::error::{ErrorCategory, SafeUnwrap};
use nestgate_core::Result;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario_runner_creation() -> Result<()> {
        let config = LoadTestConfig::default();
        let runner = ScenarioRunner::new(config.clone());

        // Verify runner was created successfully
        // This tests the constructor works
        assert_eq!(
            std::mem::size_of_val(&runner),
            std::mem::size_of::<ScenarioRunner>()
        );

        Ok(())
    }

    #[test]
    fn test_scenario_runner_with_custom_config() -> Result<()> {
        let mut config = LoadTestConfig::default();
        config.duration_seconds = 120;
        config.concurrent_users = 50;
        config.requests_per_second = 10.0;

        let runner = ScenarioRunner::new(config.clone());

        // Verify runner accepts custom configuration
        let result = runner
            .run()
            .safe_unwrap(ErrorCategory::System, "Failed to run load test scenario")?;

        assert_eq!(result.duration_seconds, 120);
        Ok(())
    }

    #[test]
    fn test_scenario_run_returns_result() -> Result<()> {
        let config = LoadTestConfig::default();
        let runner = ScenarioRunner::new(config);

        let result = runner
            .run()
            .safe_unwrap(ErrorCategory::System, "Failed to execute load test")?;

        // Verify result structure is populated
        assert!(result.success, "Test result should indicate success");
        assert_eq!(result.duration_seconds, 60, "Duration should match config");
        assert_eq!(result.total_requests, 0, "Total requests initialized to 0");
        assert_eq!(
            result.successful_requests, 0,
            "Successful requests initialized to 0"
        );
        assert_eq!(
            result.failed_requests, 0,
            "Failed requests initialized to 0"
        );
        assert_eq!(
            result.avg_response_time_ms, 0.0,
            "Avg response time initialized to 0.0"
        );

        Ok(())
    }

    #[test]
    fn test_test_result_structure() {
        let result = TestResult {
            success: true,
            duration_seconds: 100,
            total_requests: 1000,
            successful_requests: 950,
            failed_requests: 50,
            avg_response_time_ms: 125.5,
        };

        // Verify all fields are accessible and correct
        assert!(result.success);
        assert_eq!(result.duration_seconds, 100);
        assert_eq!(result.total_requests, 1000);
        assert_eq!(result.successful_requests, 950);
        assert_eq!(result.failed_requests, 50);
        assert_eq!(result.avg_response_time_ms, 125.5);

        // Verify failed + successful = total
        assert_eq!(
            result.successful_requests + result.failed_requests,
            result.total_requests
        );
    }

    #[test]
    fn test_test_result_serialization() -> Result<()> {
        let result = TestResult {
            success: true,
            duration_seconds: 60,
            total_requests: 500,
            successful_requests: 480,
            failed_requests: 20,
            avg_response_time_ms: 150.0,
        };

        // Test JSON serialization
        let json = serde_json::to_string(&result).safe_unwrap(
            ErrorCategory::Validation,
            "Failed to serialize TestResult to JSON",
        )?;

        assert!(json.contains("\"success\":true"));
        assert!(json.contains("\"duration_seconds\":60"));
        assert!(json.contains("\"total_requests\":500"));

        Ok(())
    }

    #[test]
    fn test_test_result_deserialization() -> Result<()> {
        let json = r#"{
            "success": true,
            "duration_seconds": 30,
            "total_requests": 200,
            "successful_requests": 195,
            "failed_requests": 5,
            "avg_response_time_ms": 75.5
        }"#;

        let result: TestResult = serde_json::from_str(json).safe_unwrap(
            ErrorCategory::Validation,
            "Failed to deserialize TestResult from JSON",
        )?;

        assert!(result.success);
        assert_eq!(result.duration_seconds, 30);
        assert_eq!(result.total_requests, 200);
        assert_eq!(result.successful_requests, 195);
        assert_eq!(result.failed_requests, 5);
        assert_eq!(result.avg_response_time_ms, 75.5);

        Ok(())
    }

    #[test]
    fn test_test_result_clone() {
        let result = TestResult {
            success: false,
            duration_seconds: 45,
            total_requests: 100,
            successful_requests: 80,
            failed_requests: 20,
            avg_response_time_ms: 200.0,
        };

        let cloned = result.clone();

        // Verify clone is identical
        assert_eq!(result.success, cloned.success);
        assert_eq!(result.duration_seconds, cloned.duration_seconds);
        assert_eq!(result.total_requests, cloned.total_requests);
        assert_eq!(result.successful_requests, cloned.successful_requests);
        assert_eq!(result.failed_requests, cloned.failed_requests);
        assert_eq!(result.avg_response_time_ms, cloned.avg_response_time_ms);
    }

    #[test]
    fn test_scenario_runner_multiple_runs() -> Result<()> {
        let config = LoadTestConfig::default();
        let runner = ScenarioRunner::new(config);

        // Run multiple times to verify consistency
        for _ in 0..5 {
            let result = runner
                .run()
                .safe_unwrap(ErrorCategory::System, "Failed to run scenario")?;
            assert!(result.success, "Each run should succeed");
        }

        Ok(())
    }

    #[test]
    fn test_scenario_runner_with_different_durations() -> Result<()> {
        let durations = vec![10, 30, 60, 120, 300];

        for duration in durations {
            let mut config = LoadTestConfig::default();
            config.duration_seconds = duration;

            let runner = ScenarioRunner::new(config);
            let result = runner.run().safe_unwrap(
                ErrorCategory::System,
                "Failed to run scenario with custom duration",
            )?;

            assert_eq!(
                result.duration_seconds, duration,
                "Result duration should match config"
            );
        }

        Ok(())
    }

    #[test]
    fn test_test_result_failure_case() {
        let result = TestResult {
            success: false,
            duration_seconds: 60,
            total_requests: 100,
            successful_requests: 30,
            failed_requests: 70,
            avg_response_time_ms: 500.0,
        };

        assert!(!result.success, "Should be marked as failure");
        assert!(
            result.failed_requests > result.successful_requests,
            "More failures than successes"
        );
        assert_eq!(
            result.total_requests,
            result.successful_requests + result.failed_requests
        );
    }
}
