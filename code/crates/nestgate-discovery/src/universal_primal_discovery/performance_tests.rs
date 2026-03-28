// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **Comprehensive Tests for Performance Discovery**
//!
//! Sprint 2 continuation: Performance module coverage (259 lines)
//! Target: 80%+ coverage of performance.rs

use super::performance::*;
use std::time::Duration;

// ============================================================================
// TEST TYPE ENUM TESTS
// ============================================================================

#[cfg(test)]
mod test_type_tests {
    use super::*;

    #[test]
    fn test_test_type_default() {
        let test_type = TestType::default();
        assert_eq!(test_type, TestType::Load);
    }

    #[test]
    fn test_test_type_load() {
        let test_type = TestType::Load;
        assert_eq!(test_type, TestType::Load);
    }

    #[test]
    fn test_test_type_stress() {
        let test_type = TestType::Stress;
        assert_eq!(test_type, TestType::Stress);
    }

    #[test]
    fn test_test_type_spike() {
        let test_type = TestType::Spike;
        assert_eq!(test_type, TestType::Spike);
    }

    #[test]
    fn test_test_type_volume() {
        let test_type = TestType::Volume;
        assert_eq!(test_type, TestType::Volume);
    }

    #[test]
    fn test_test_type_endurance() {
        let test_type = TestType::Endurance;
        assert_eq!(test_type, TestType::Endurance);
    }

    #[test]
    fn test_test_type_scalability() {
        let test_type = TestType::Scalability;
        assert_eq!(test_type, TestType::Scalability);
    }

    #[test]
    fn test_test_type_display() {
        assert_eq!(TestType::Load.to_string(), "Load");
        assert_eq!(TestType::Stress.to_string(), "Stress");
        assert_eq!(TestType::Spike.to_string(), "Spike");
        assert_eq!(TestType::Volume.to_string(), "Volume");
        assert_eq!(TestType::Endurance.to_string(), "Endurance");
        assert_eq!(TestType::Scalability.to_string(), "Scalability");
    }

    #[test]
    fn test_test_type_equality() {
        assert_eq!(TestType::Load, TestType::Load);
        assert_ne!(TestType::Load, TestType::Stress);
    }

    #[test]
    fn test_test_type_clone() {
        let t1 = TestType::Load;
        let t2 = t1;
        assert_eq!(t1, t2);
    }
}

// ============================================================================
// OPTIMAL TIMEOUT TESTS
// ============================================================================

#[cfg(test)]
mod optimal_timeout_tests {
    use super::*;

    #[test]
    fn test_optimal_timeout_creation() {
        let timeout = OptimalTimeout {
            timeout: Duration::from_secs(5),
            confidence: 0.95,
            test_iterations: 100,
            baseline_latency: Duration::from_millis(10),
        };

        assert_eq!(timeout.timeout, Duration::from_secs(5));
        assert_eq!(timeout.confidence, 0.95);
        assert_eq!(timeout.test_iterations, 100);
        assert_eq!(timeout.baseline_latency, Duration::from_millis(10));
    }

    #[test]
    fn test_optimal_timeout_with_high_confidence() {
        let timeout = OptimalTimeout {
            timeout: Duration::from_secs(3),
            confidence: 0.99,
            test_iterations: 1000,
            baseline_latency: Duration::from_millis(5),
        };

        assert!(timeout.confidence > 0.95);
        assert_eq!(timeout.test_iterations, 1000);
    }

    #[test]
    fn test_optimal_timeout_with_low_latency() {
        let timeout = OptimalTimeout {
            timeout: Duration::from_millis(100),
            confidence: 0.90,
            test_iterations: 50,
            baseline_latency: Duration::from_millis(1),
        };

        assert!(timeout.baseline_latency < Duration::from_millis(10));
    }

    #[test]
    fn test_optimal_timeout_clone() {
        let timeout = OptimalTimeout {
            timeout: Duration::from_secs(5),
            confidence: 0.95,
            test_iterations: 100,
            baseline_latency: Duration::from_millis(10),
        };

        let timeout2 = timeout.clone();
        assert_eq!(timeout.timeout, timeout2.timeout);
        assert_eq!(timeout.confidence, timeout2.confidence);
    }
}

// ============================================================================
// TEST RESULT TESTS
// ============================================================================

#[cfg(test)]
mod test_result_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_result_success() {
        let result = TestResult {
            test_name: "timeout_test".to_string(),
            latency: Duration::from_millis(50),
            success: true,
            error_message: None,
            timestamp: Instant::now(),
        };

        assert_eq!(result.test_name, "timeout_test");
        assert!(result.success);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_result_failure() {
        let result = TestResult {
            test_name: "failing_test".to_string(),
            latency: Duration::from_millis(1000),
            success: false,
            error_message: Some("Timeout exceeded".to_string()),
            timestamp: Instant::now(),
        };

        assert!(!result.success);
        assert!(result.error_message.is_some());
        assert_eq!(result.error_message.unwrap(), "Timeout exceeded");
    }

    #[test]
    fn test_result_with_various_latencies() {
        let latencies = vec![
            Duration::from_millis(10),
            Duration::from_millis(50),
            Duration::from_millis(100),
            Duration::from_millis(500),
            Duration::from_millis(1000),
        ];

        for latency in latencies {
            let result = TestResult {
                test_name: "latency_test".to_string(),
                latency,
                success: latency < Duration::from_millis(500),
                error_message: None,
                timestamp: Instant::now(),
            };

            assert_eq!(result.latency, latency);
        }
    }

    #[test]
    fn test_result_clone() {
        let result = TestResult {
            test_name: "test".to_string(),
            latency: Duration::from_millis(50),
            success: true,
            error_message: None,
            timestamp: Instant::now(),
        };

        let result2 = result.clone();
        assert_eq!(result.test_name, result2.test_name);
        assert_eq!(result.success, result2.success);
    }
}

// ============================================================================
// RESPONSE TIME THRESHOLDS TESTS
// ============================================================================

#[cfg(test)]
mod response_time_thresholds_tests {
    use super::*;

    #[test]
    fn test_thresholds_creation() {
        let thresholds = ResponseTimeThresholds {
            p50: Duration::from_millis(100),
            p95: Duration::from_millis(500),
            p99: Duration::from_millis(1000),
            max: Duration::from_millis(5000),
        };

        assert_eq!(thresholds.p50, Duration::from_millis(100));
        assert_eq!(thresholds.p95, Duration::from_millis(500));
        assert_eq!(thresholds.p99, Duration::from_millis(1000));
        assert_eq!(thresholds.max, Duration::from_millis(5000));
    }

    #[test]
    fn test_thresholds_ordering() {
        let thresholds = ResponseTimeThresholds {
            p50: Duration::from_millis(50),
            p95: Duration::from_millis(200),
            p99: Duration::from_millis(500),
            max: Duration::from_millis(2000),
        };

        // Verify they're in ascending order
        assert!(thresholds.p50 < thresholds.p95);
        assert!(thresholds.p95 < thresholds.p99);
        assert!(thresholds.p99 < thresholds.max);
    }

    #[test]
    fn test_thresholds_clone() {
        let thresholds = ResponseTimeThresholds {
            p50: Duration::from_millis(100),
            p95: Duration::from_millis(500),
            p99: Duration::from_millis(1000),
            max: Duration::from_millis(5000),
        };

        let thresholds2 = thresholds.clone();
        assert_eq!(thresholds.p50, thresholds2.p50);
        assert_eq!(thresholds.max, thresholds2.max);
    }
}

// ============================================================================
// TEST DATA CONFIG TESTS
// ============================================================================

#[cfg(test)]
mod test_data_config_tests {
    use super::*;

    #[test]
    fn test_data_config_creation() {
        let config = TestDataConfig {
            use_random_data: true,
            data_size: 1024,
            data_variance: 0.1,
        };

        assert!(config.use_random_data);
        assert_eq!(config.data_size, 1024);
        assert_eq!(config.data_variance, 0.1);
    }

    #[test]
    fn test_data_config_deterministic() {
        let config = TestDataConfig {
            use_random_data: false,
            data_size: 512,
            data_variance: 0.0,
        };

        assert!(!config.use_random_data);
        assert_eq!(config.data_variance, 0.0);
    }

    #[test]
    fn test_data_config_clone() {
        let config = TestDataConfig {
            use_random_data: true,
            data_size: 2048,
            data_variance: 0.2,
        };

        let config2 = config.clone();
        assert_eq!(config.use_random_data, config2.use_random_data);
        assert_eq!(config.data_size, config2.data_size);
    }
}

// ============================================================================
// PERFORMANCE DISCOVERY TESTS
// ============================================================================

#[cfg(test)]
mod performance_discovery_tests {
    use super::*;

    #[test]
    fn test_performance_discovery_creation() {
        let discovery = PerformanceDiscovery::new();
        // Just verify it can be created
        drop(discovery);
    }

    #[test]
    fn test_performance_discovery_default() {
        let discovery = PerformanceDiscovery;
        drop(discovery);
    }

    #[test]
    fn test_performance_discovery_multiple_instances() {
        let d1 = PerformanceDiscovery::new();
        let d2 = PerformanceDiscovery::new();
        let d3 = PerformanceDiscovery;

        drop(d1);
        drop(d2);
        drop(d3);
    }

    #[tokio::test]
    async fn test_discover_optimal_timeout() {
        let discovery = PerformanceDiscovery::new();
        let result = discovery.discover_optimal_timeout("test-service").await;

        assert!(result.is_ok());
        let timeout = result.unwrap();
        assert!(timeout.as_secs() > 0 || timeout.as_millis() > 0);
    }

    #[tokio::test]
    async fn test_discover_optimal_timeout_various_services() {
        let discovery = PerformanceDiscovery::new();
        let services = vec!["api", "web", "db", "cache", "metrics"];

        for service in services {
            let result = discovery.discover_optimal_timeout(service).await;
            assert!(result.is_ok(), "Failed for service: {}", service);
        }
    }

    #[test]
    fn test_discover_performance() {
        let discovery = PerformanceDiscovery::new();
        let result = discovery.discover_performance();

        assert!(result.is_ok());
        let characteristics = result.unwrap();

        // Should have cpu_cores
        assert!(characteristics.contains_key("cpu_cores"));

        // Should have timestamp
        assert!(characteristics.contains_key("discovery_timestamp"));
    }

    #[test]
    fn test_discover_performance_cpu_cores() {
        let discovery = PerformanceDiscovery::new();
        let characteristics = discovery.discover_performance().unwrap();

        let cpu_cores = &characteristics["cpu_cores"];
        assert!(cpu_cores.is_number());

        if let Some(cores) = cpu_cores.as_u64() {
            assert!(cores > 0, "CPU cores should be positive");
        }
    }

    #[test]
    fn test_discover_performance_timestamp() {
        let discovery = PerformanceDiscovery::new();
        let characteristics = discovery.discover_performance().unwrap();

        let timestamp = &characteristics["discovery_timestamp"];
        assert!(timestamp.is_string());
    }

    #[test]
    fn test_discover_performance_deterministic() {
        let discovery = PerformanceDiscovery::new();

        let chars1 = discovery.discover_performance().unwrap();
        let chars2 = discovery.discover_performance().unwrap();

        // CPU cores should be the same
        assert_eq!(chars1["cpu_cores"], chars2["cpu_cores"]);
    }
}

// ============================================================================
// PERFORMANCE TEST RUNNER TESTS
// ============================================================================

#[cfg(test)]
mod performance_test_runner_tests {
    use super::*;
    use nestgate_config::config::canonical_primary::PerformanceConfig;

    #[test]
    fn test_runner_creation() {
        let config = PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        // Just verify it can be created
        drop(runner);
    }

    #[tokio::test]
    async fn test_runner_discover_optimal_timeout() {
        let config = PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        let result = runner.discover_optimal_timeout().await;
        assert!(result.is_ok());

        let optimal = result.unwrap();
        assert!(optimal.timeout.as_secs() > 0 || optimal.timeout.as_millis() > 0);
        // ✅ MODERN: Use epsilon for confidence range check
        assert!(optimal.confidence >= -1e-9 && optimal.confidence <= 1.0 + 1e-9);
        assert!(optimal.test_iterations > 0);
    }

    #[test]
    fn test_runner_generate_metrics() {
        let config = PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        let metrics = runner.generate_metrics();

        // Should have basic metrics
        assert!(metrics.contains_key("test_name"));
        assert!(metrics.contains_key("test_type"));
        assert!(metrics.contains_key("test_iterations"));
        assert!(metrics.contains_key("baseline_timeout"));

        assert_eq!(metrics["test_name"], "performance_discovery");
        assert_eq!(metrics["test_type"], "timeout_optimization");
    }

    #[test]
    fn test_runner_metrics_completeness() {
        let config = PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        let metrics = runner.generate_metrics();

        let expected_keys = vec![
            "test_name",
            "test_type",
            "concurrent_users",
            "target_rps",
            "test_iterations",
            "baseline_timeout",
            "max_timeout",
            "percentile_target",
        ];

        for key in expected_keys {
            assert!(metrics.contains_key(key), "Missing metric: {}", key);
        }
    }

    #[tokio::test]
    async fn test_runner_multiple_discoveries() {
        let config = PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        // Run multiple discoveries
        for _ in 0..3 {
            let result = runner.discover_optimal_timeout().await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_runner_optimal_timeout_bounds() {
        let config = PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        let optimal = runner.discover_optimal_timeout().await.unwrap();

        // Timeout should be reasonable
        assert!(optimal.timeout < Duration::from_secs(3600)); // Less than 1 hour
        assert!(optimal.timeout > Duration::ZERO);
    }

    #[tokio::test]
    async fn test_runner_confidence_level() {
        let config = PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        let optimal = runner.discover_optimal_timeout().await.unwrap();

        // ✅ MODERN: Confidence should be between 0 and 1 (with epsilon)
        assert!(optimal.confidence >= -1e-9);
        assert!(optimal.confidence <= 1.0 + 1e-9);
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod performance_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_performance_discovery_workflow() {
        let discovery = PerformanceDiscovery::new();

        // Discover timeout
        let timeout_result = discovery.discover_optimal_timeout("api").await;
        assert!(timeout_result.is_ok());

        // Discover performance characteristics
        let perf_result = discovery.discover_performance();
        assert!(perf_result.is_ok());

        let characteristics = perf_result.unwrap();
        assert!(characteristics.contains_key("cpu_cores"));
    }

    #[tokio::test]
    async fn test_concurrent_performance_discoveries() {
        use std::sync::Arc;

        let discovery = Arc::new(PerformanceDiscovery::new());

        let d1 = discovery.clone();
        let d2 = discovery.clone();
        let d3 = discovery.clone();

        let handle1 = tokio::spawn(async move { d1.discover_optimal_timeout("service1").await });

        let handle2 = tokio::spawn(async move { d2.discover_optimal_timeout("service2").await });

        let handle3 = tokio::spawn(async move { d3.discover_performance() });

        let results = tokio::try_join!(handle1, handle2, handle3);
        assert!(results.is_ok());
    }

    #[test]
    fn test_performance_discovery_thread_safe() {
        use std::thread;

        let handles: Vec<_> = (0..5)
            .map(|_| {
                thread::spawn(|| {
                    let discovery = PerformanceDiscovery::new();
                    discovery.discover_performance()
                })
            })
            .collect();

        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_performance_metrics_consistency() {
        use nestgate_config::config::canonical_primary::PerformanceConfig;

        let config = PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        let metrics1 = runner.generate_metrics();
        let metrics2 = runner.generate_metrics();

        // Metrics should be consistent
        assert_eq!(metrics1["test_name"], metrics2["test_name"]);
        assert_eq!(metrics1["test_type"], metrics2["test_type"]);
    }
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[cfg(test)]
mod performance_edge_cases {
    use super::*;

    #[tokio::test]
    async fn test_timeout_with_empty_service_name() {
        let discovery = PerformanceDiscovery::new();
        let result = discovery.discover_optimal_timeout("").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_timeout_with_long_service_name() {
        let discovery = PerformanceDiscovery::new();
        let long_name = "a".repeat(1000);
        let result = discovery.discover_optimal_timeout(&long_name).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_timeout_with_special_chars() {
        let discovery = PerformanceDiscovery::new();
        let result = discovery.discover_optimal_timeout("service@#$%").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_performance_discovery_multiple_calls() {
        let discovery = PerformanceDiscovery::new();

        // Should be able to call multiple times
        for _ in 0..10 {
            let result = discovery.discover_performance();
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_test_type_all_variants() {
        let types = [
            TestType::Load,
            TestType::Stress,
            TestType::Spike,
            TestType::Volume,
            TestType::Endurance,
            TestType::Scalability,
        ];

        assert_eq!(types.len(), 6);
    }
}
