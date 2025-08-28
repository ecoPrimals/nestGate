///
/// This module provides configuration for performance testing including load testing,
/// stress testing, benchmarking, and performance metrics collection.
/// Consolidates: PerformanceMetricsConfig, StressTestConfig, LoadTestConfig, BenchmarkConfig
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== SECTION ====================

/// **Unified performance testing configuration**
/// Consolidates: PerformanceMetricsConfig, StressTestConfig, LoadTestConfig, BenchmarkConfig
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestPerformanceConfig {
    /// Load testing configuration
    pub load_testing: LoadTestingConfig,
    /// Stress testing configuration
    pub stress_testing: StressTestingConfig,
    /// Benchmark configuration
    pub benchmarking: BenchmarkingConfig,
    /// Performance metrics collection
    pub metrics: PerformanceMetricsConfig,
}

/// **Load testing configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestingConfig {
    /// Number of virtual users
    pub virtual_users: usize,
    /// Test duration
    pub duration: Duration,
    /// Ramp-up time
    pub ramp_up_time: Duration,
    /// Request rate per second
    pub requests_per_second: f64,
    /// Load testing scenarios
    pub scenarios: Vec<LoadTestScenario>,
}

impl Default for LoadTestingConfig {
    fn default() -> Self {
        Self {
            virtual_users: 100,
            duration: Duration::from_secs(300),
            ramp_up_time: Duration::from_secs(60),
            requests_per_second: 10.0,
            scenarios: vec![LoadTestScenario::default()],
        }
    }
}

/// **Load test scenario**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestScenario {
    /// Scenario name
    pub name: String,
    /// Target endpoint
    pub endpoint: String,
    /// HTTP method
    pub method: String,
    /// Request payload
    pub payload: Option<String>,
    /// Expected response time (ms)
    pub expected_response_time: u64,
    /// Weight in load distribution
    pub weight: f64,
}

impl Default for LoadTestScenario {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            endpoint: "/health".to_string(),
            method: "GET".to_string(),
            payload: None,
            expected_response_time: 100,
            weight: 1.0,
        }
    }
}

/// **Stress testing configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressTestingConfig {
    /// Maximum load multiplier
    pub max_load_multiplier: f64,
    /// Stress test duration
    pub stress_duration: Duration,
    /// Recovery test duration
    pub recovery_duration: Duration,
    /// Resource limits
    pub resource_limits: ResourceLimits,
}

impl Default for StressTestingConfig {
    fn default() -> Self {
        Self {
            max_load_multiplier: 10.0,
            stress_duration: Duration::from_secs(600),
            recovery_duration: Duration::from_secs(300),
            resource_limits: ResourceLimits::default(),
        }
    }
}

/// **Resource limits for testing**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    /// Maximum disk usage in MB
    pub max_disk_mb: u64,
    /// Maximum network bandwidth in Mbps
    pub max_network_mbps: f64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_percent: 80.0,
            max_memory_mb: 4096,
            max_disk_mb: 10240,
            max_network_mbps: 100.0,
        }
    }
}

/// **Benchmarking configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkingConfig {
    /// Benchmark iterations
    pub iterations: usize,
    /// Warmup iterations
    pub warmup_iterations: usize,
    /// Benchmark timeout
    pub benchmark_timeout: Duration,
    /// Statistical confidence level
    pub confidence_level: f64,
    /// Benchmark suites
    pub suites: Vec<String>,
}

impl Default for BenchmarkingConfig {
    fn default() -> Self {
        Self {
            iterations: 1000,
            warmup_iterations: 100,
            benchmark_timeout: Duration::from_secs(3600),
            confidence_level: 0.95,
            suites: vec!["performance".to_string(), "memory".to_string()],
        }
    }
}

/// **Performance metrics configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsConfig {
    /// Enable metrics collection
    pub enable_collection: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Metrics retention period
    pub retention_period: Duration,
    /// Collected metric types
    pub metric_types: Vec<String>,
    /// Performance thresholds
    pub thresholds: PerformanceThresholds,
}

impl Default for PerformanceMetricsConfig {
    fn default() -> Self {
        Self {
            enable_collection: true,
            collection_interval: Duration::from_secs(1),
            retention_period: Duration::from_secs(3600),
            metric_types: vec![
                "cpu".to_string(),
                "memory".to_string(),
                "network".to_string(),
                "disk".to_string(),
                "response_time".to_string(),
            ],
            thresholds: PerformanceThresholds::default(),
        }
    }
}

/// **Performance thresholds**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum response time in milliseconds
    pub max_response_time_ms: u64,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    /// Minimum throughput (requests/second)
    pub min_throughput: f64,
    /// Maximum error rate percentage
    pub max_error_rate: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_response_time_ms: 1000,
            max_cpu_percent: 70.0,
            max_memory_mb: 2048,
            min_throughput: 100.0,
            max_error_rate: 1.0,
        }
    }
}
