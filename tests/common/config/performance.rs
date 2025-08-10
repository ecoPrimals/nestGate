use nestgate_core::smart_abstractions::prelude::*;
use serde::{Deserialize, Serialize};
/// **PERFORMANCE & CHAOS TESTING CONFIGURATION MODULE**
///
/// Extracted from the monolithic test_config.rs to achieve better separation
/// of concerns. Handles performance testing and chaos engineering configuration.
use std::time::Duration;

/// **PERFORMANCE TESTING CONFIGURATION**
/// Comprehensive settings for performance and load testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestPerformanceSettings {
    /// Number of concurrent users/operations
    pub concurrent_users: usize,
    /// Test duration
    pub duration: Duration,
    /// Target requests per second
    pub target_rps: f64,
    /// Ramp-up duration to reach target load
    pub ramp_up_duration: Duration,
    /// Performance thresholds for validation
    pub thresholds: PerformanceThresholds,
    /// Number of concurrent operations per test
    pub concurrent_operations: usize,
    /// Operations per test (for workflow tests)
    pub operations_per_test: Option<usize>,
    /// Test timeout duration
    pub test_timeout: Duration,
    /// Stress test intensity (0.0 to 1.0)
    pub stress_intensity: f64,
    /// Enable performance validation
    pub enable_performance_validation: bool,
}

/// Performance validation thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum acceptable response time
    pub max_response_time: Duration,
    /// Maximum acceptable error rate (percentage)
    pub max_error_rate: f64,
    /// Minimum acceptable throughput (RPS)
    pub min_throughput: f64,
    /// Maximum acceptable memory usage (MB)
    pub max_memory_mb: u64,
}

/// **CHAOS TESTING CONFIGURATION**
/// Settings for chaos engineering and fault injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestChaosSettings {
    /// Enable chaos testing globally
    pub enabled: bool,
    /// Probability of chaos events (0.0 to 1.0)
    pub injection_probability: f64,
    /// Types of chaos to inject
    pub chaos_types: Vec<ChaosType>,
    /// Duration for each chaos scenario
    pub scenario_duration: Duration,
    /// Recovery time after chaos events
    pub recovery_time: Duration,
    /// Enable chaos injection
    pub enable_chaos_injection: bool,
}

/// Types of chaos to inject during testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChaosType {
    /// Network partitions and connectivity issues
    NetworkPartition,
    /// High CPU load simulation
    CpuStress,
    /// Memory pressure simulation
    MemoryPressure,
    /// Disk I/O saturation
    DiskStress,
    /// Process crashes and restarts
    ProcessCrash,
    /// Service dependency failures
    ServiceFailure,
    /// Timeout and latency injection
    LatencyInjection,
    /// Resource exhaustion
    ResourceExhaustion,
}

/// Load testing patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadPattern {
    /// Constant load throughout test
    Constant,
    /// Gradually increasing load
    Ramp,
    /// Spike testing with sudden load increases
    Spike,
    /// Wave pattern with periodic load variations
    Wave,
    /// Step-wise load increases
    Step,
}

/// Performance metrics collection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval
    pub collection_interval: Duration,
    /// Metrics to collect
    pub metrics_types: Vec<MetricType>,
    /// Export format for metrics
    pub export_format: MetricsExportFormat,
}

/// Types of performance metrics to collect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    /// Response time metrics
    ResponseTime,
    /// Throughput metrics
    Throughput,
    /// Error rate metrics
    ErrorRate,
    /// Memory usage metrics
    MemoryUsage,
    /// CPU utilization metrics
    CpuUtilization,
    /// Disk I/O metrics
    DiskIO,
    /// Network metrics
    Network,
}

/// Metrics export formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsExportFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Prometheus format
    Prometheus,
    /// InfluxDB line protocol
    InfluxDB,
}

/// Stress testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressTestConfig {
    /// Maximum load to apply
    pub max_load: LoadSpec,
    /// Duration to maintain maximum load
    pub max_load_duration: Duration,
    /// Resource limits during stress testing
    pub resource_limits: StressResourceLimits,
    /// Failure criteria for stress tests
    pub failure_criteria: StressFailureCriteria,
}

/// Load specification for stress testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadSpec {
    /// Requests per second
    pub rps: f64,
    /// Concurrent connections
    pub concurrent_connections: usize,
    /// Data volume per request (bytes)
    pub data_volume_bytes: u64,
}

/// Resource limits during stress testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressResourceLimits {
    /// Maximum memory usage during stress test
    pub max_memory_mb: u64,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,
    /// Maximum disk I/O rate (MB/s)
    pub max_disk_io_mbps: f64,
    /// Maximum network bandwidth (Mbps)
    pub max_network_mbps: f64,
}

/// Failure criteria for stress tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressFailureCriteria {
    /// Maximum acceptable error rate during stress
    pub max_error_rate: f64,
    /// Maximum acceptable response time degradation
    pub max_response_time_degradation: f64,
    /// Memory leak detection threshold
    pub memory_leak_threshold_mb: u64,
}

impl Default for TestPerformanceSettings {
    fn default() -> Self {
        Self {
            concurrent_users: 10,
            duration: Duration::from_secs(60),
            target_rps: 100.0,
            ramp_up_duration: Duration::from_secs(10),
            thresholds: PerformanceThresholds::default(),
            concurrent_operations: 10,
            operations_per_test: Some(10),
            test_timeout: Duration::from_secs(300),
            stress_intensity: 0.5,
            enable_performance_validation: true,
        }
    }
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_response_time: Duration::from_millis(500),
            max_error_rate: 1.0, // 1%
            min_throughput: 50.0,
            max_memory_mb: 512,
        }
    }
}

impl Default for TestChaosSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            injection_probability: 0.1,
            chaos_types: Vec::new(),
            scenario_duration: Duration::from_secs(30),
            recovery_time: Duration::from_secs(10),
            enable_chaos_injection: false,
        }
    }
}

impl TestPerformanceSettings {
    /// Create configuration for light performance testing
    pub fn light_load() -> Self {
        Self {
            concurrent_users: 5,
            duration: Duration::from_secs(30),
            target_rps: 50.0,
            stress_intensity: 0.3,
            thresholds: PerformanceThresholds {
                max_response_time: Duration::from_millis(200),
                max_error_rate: 0.5,
                min_throughput: 25.0,
                max_memory_mb: 256,
            },
            ..Default::default()
        }
    }

    /// Create configuration for heavy performance testing
    pub fn heavy_load() -> Self {
        Self {
            concurrent_users: 100,
            duration: Duration::from_secs(300), // 5 minutes
            target_rps: 1000.0,
            ramp_up_duration: Duration::from_secs(60),
            stress_intensity: 0.9,
            thresholds: PerformanceThresholds {
                max_response_time: Duration::from_secs(2),
                max_error_rate: 2.0,
                min_throughput: 500.0,
                max_memory_mb: 2048,
            },
            ..Default::default()
        }
    }

    /// Create configuration for stress testing
    pub fn stress_test() -> Self {
        Self {
            concurrent_users: 500,
            duration: Duration::from_secs(600), // 10 minutes
            target_rps: 5000.0,
            ramp_up_duration: Duration::from_secs(120),
            stress_intensity: 1.0,
            thresholds: PerformanceThresholds {
                max_response_time: Duration::from_secs(5),
                max_error_rate: 5.0,
                min_throughput: 100.0, // Lower threshold for stress test
                max_memory_mb: 4096,
            },
            ..Default::default()
        }
    }
}

impl TestChaosSettings {
    /// Create configuration for light chaos testing
    pub fn light_chaos() -> Self {
        Self {
            enabled: true,
            injection_probability: 0.05, // 5% chance
            chaos_types: vec![ChaosType::LatencyInjection, ChaosType::ServiceFailure],
            scenario_duration: Duration::from_secs(15),
            recovery_time: Duration::from_secs(5),
            enable_chaos_injection: true,
        }
    }

    /// Create configuration for comprehensive chaos testing
    pub fn comprehensive_chaos() -> Self {
        Self {
            enabled: true,
            injection_probability: 0.2, // 20% chance
            chaos_types: vec![
                ChaosType::NetworkPartition,
                ChaosType::CpuStress,
                ChaosType::MemoryPressure,
                ChaosType::DiskStress,
                ChaosType::ProcessCrash,
                ChaosType::ServiceFailure,
                ChaosType::LatencyInjection,
                ChaosType::ResourceExhaustion,
            ],
            scenario_duration: Duration::from_secs(60),
            recovery_time: Duration::from_secs(30),
            enable_chaos_injection: true,
        }
    }
}
