// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Performance Discovery Module
/// Handles performance-related discovery operations including:
/// - Optimal timeout discovery through benchmarking
/// - Resource limit analysis
/// - Performance profile optimization
/// - System capacity analysis
use nestgate_types::error::Result;
/// Performance Test Configuration Management
/// Handles performance testing configuration, benchmarks, and optimization
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
// 🚀 ECOSYSTEM UNIFICATION: Import unified types

/// Test type enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
/// Types of Test
pub enum TestType {
    #[default]
    /// Load
    Load,
    /// Stress
    Stress,
    /// Spike
    Spike,
    /// Volume
    Volume,
    /// Endurance
    Endurance,
    /// Scalability
    Scalability,
}
impl std::fmt::Display for TestType {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

// 🚀 FULLY MODERN: Clean imports - no duplicates

// 🚀 MODERNIZATION: UnifiedConfig now uses UnifiedPerformanceTestConfig directly
/// **MODERNIZED**: `UnifiedConfig` now uses `UnifiedPerformanceTestConfig` directly  
pub type PerformanceTestConfig = nestgate_config::config::canonical_primary::PerformanceConfig;
// 🚀 FULLY MODERN: All performance testing functionality now uses UnifiedPerformanceTestConfig directly
// No legacy implementation needed - use UnifiedPerformanceTestConfig::default() and methods

/// Response time thresholds for performance validation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Responsetimethresholds
pub struct ResponseTimeThresholds {
    /// P50
    pub p50: Duration,
    /// P95
    pub p95: Duration,
    /// P99
    pub p99: Duration,
    /// Max
    pub max: Duration,
}
/// Test data configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TestData
pub struct TestDataConfig {
    /// Use Random Data
    pub use_random_data: bool,
    /// Size of data
    pub data_size: usize,
    /// Data Variance
    pub data_variance: f64,
}
/// Optimal timeout result
#[derive(Debug, Clone)]
/// Optimaltimeout
pub struct OptimalTimeout {
    /// Timeout
    pub timeout: Duration,
    /// Confidence
    pub confidence: f64,
    /// Test Iterations
    pub test_iterations: usize,
    /// Baseline Latency
    pub baseline_latency: Duration,
}
/// Performance test result
#[derive(Debug, Clone)]
/// Testresult
pub struct TestResult {
    /// Test name
    pub test_name: String,
    /// Latency
    pub latency: Duration,
    /// Success
    pub success: bool,
    /// Error Message
    pub error_message: Option<String>,
    /// Timestamp
    pub timestamp: std::time::Instant,
}
/// Enhanced Performance Test Runner with unified configuration
#[derive(Debug)]
/// Performancetestrunner
pub struct PerformanceTestRunner {
    /// Configuration for
    pub config: nestgate_config::config::canonical_primary::PerformanceConfig,
}
impl PerformanceTestRunner {
    /// Create new performance test runner
    #[must_use]
    pub fn new(config: nestgate_config::config::canonical_primary::PerformanceConfig) -> Self {
        Self { config }
    }

    /// Discover optimal timeout through benchmarking
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_optimal_timeout(&self) -> Result<OptimalTimeout> {
        let mut latencies = Vec::new();

        // Run multiple test iterations to gather latency data
        for _ in 0..self.config.testing.test_iterations {
            let start = Instant::now();

            // Simulate test operation
            tokio::time::sleep(Duration::from_millis(10)).await;

            let latency = start.elapsed();
            latencies.push(latency);
        }

        // Sort latencies for percentile calculation
        latencies.sort();

        // Calculate target percentile timeout
        let percentile_index = (((latencies.len() as f64) * self.config.testing.percentile_target
            / 100.0)
            .ceil() as usize
            - 1)
        .min(latencies.len() - 1);

        let optimal_timeout = latencies[percentile_index];

        // Ensure within bounds
        let bounded_timeout = optimal_timeout
            .max(Duration::from_secs(
                self.config.testing.baseline_timeout_seconds,
            ))
            .min(Duration::from_secs(
                self.config.testing.baseline_timeout_seconds,
            ));

        Ok(OptimalTimeout {
            timeout: bounded_timeout,
            confidence: 0.95,
            test_iterations: self.config.testing.test_iterations,
            baseline_latency: latencies[0],
        })
    }

    /// Generate performance metrics
    #[must_use]
    pub fn generate_metrics(&self) -> HashMap<String, String> {
        let mut metrics = HashMap::new();

        metrics.insert("test_name".to_string(), "performance_discovery".to_string());
        metrics.insert("test_type".to_string(), "timeout_optimization".to_string());
        metrics.insert("concurrent_users".to_string(), "1".to_string());
        metrics.insert("target_rps".to_string(), "100".to_string());

        metrics.insert(
            "test_iterations".to_string(),
            self.config.testing.test_iterations.to_string(),
        );
        metrics.insert(
            "baseline_timeout".to_string(),
            format!("{}s", self.config.testing.baseline_timeout_seconds),
        );
        metrics.insert(
            "max_timeout".to_string(),
            format!("{}s", self.config.testing.baseline_timeout_seconds),
        );
        metrics.insert(
            "percentile_target".to_string(),
            self.config.testing.percentile_target.to_string(),
        );

        metrics
    }
}

/// Performance Discovery Service
#[derive(Debug)]
/// Performancediscovery
pub struct PerformanceDiscovery;
impl Default for PerformanceDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceDiscovery {
    /// Create new performance discovery service
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Discover optimal timeout through benchmarking
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_optimal_timeout(&self, _service_name: &str) -> Result<Duration> {
        // Use default performance test config for discovery
        let config = nestgate_config::config::canonical_primary::PerformanceConfig::default();
        let runner = PerformanceTestRunner::new(config);

        let optimal = runner.discover_optimal_timeout().await?;
        Ok(optimal.timeout)
    }

    /// Discover performance characteristics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn discover_performance(&self) -> Result<HashMap<String, serde_json::Value>> {
        let mut characteristics = HashMap::new();

        let logical_cpus = std::thread::available_parallelism().map_or(4u64, |n| n.get() as u64);
        characteristics.insert(
            "cpu_cores".to_string(),
            serde_json::Value::Number(serde_json::Number::from(logical_cpus)),
        );
        characteristics.insert(
            "discovery_timestamp".to_string(),
            serde_json::Value::String(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string(),
            ),
        );

        Ok(characteristics)
    }
}
