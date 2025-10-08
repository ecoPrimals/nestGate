#![cfg(feature = "dev-stubs")]

/// Unified Benchmark Configuration System for NestGate
/// This module provides standardized benchmark configuration that replaces the
/// fragmented benchmark config structs scattered across the benchmark test suite.
/// **PROBLEM SOLVED**: Eliminates duplicate benchmark config structs with
/// inconsistent fields and approaches.
///
/// **⚠️ BENCHMARKING ONLY**: This module is only available with `dev-stubs` feature

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
// Import the standardized config pattern
use crate::unified_config_consolidation::StandardDomainConfig;
use crate::unified_enums::service_types::UnifiedServiceType;

// ==================== SECTION ====================

/// Benchmark-specific configuration extensions
/// Domain-specific fields for comprehensive benchmark testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkExtensions {
    /// Performance testing configuration
    pub performance: BenchmarkPerformanceSettings,
    /// Memory optimization benchmarks
    pub memory: BenchmarkMemorySettings,
    /// CPU optimization benchmarks
    pub cpu: BenchmarkCpuSettings,
    /// I/O optimization benchmarks
    pub io: BenchmarkIoSettings,
    /// Network benchmarks
    pub network: BenchmarkNetworkSettings,
    /// Mock service benchmarks
    pub mocking: BenchmarkMockingSettings,
    /// Zero-copy optimization benchmarks
    pub zero_copy: BenchmarkZeroCopySettings,
    /// Stress testing configuration
    pub stress: BenchmarkStressSettings,
    }
/// Performance benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkPerformanceSettings {
    /// Test duration in seconds
    pub test_duration_seconds: u64,
    /// Number of concurrent threads
    pub concurrent_threads: usize,
    /// Target operations per second
    pub target_ops_per_second: u64,
    /// Throughput measurement interval
    pub measurement_interval: Duration,
    /// Warmup duration
    pub warmup_duration: Duration,
    /// Sample size for measurements
    pub sample_size: u32,
    /// Enable detailed profiling
    pub enable_profiling: bool,
    /// Performance thresholds
    pub thresholds: BenchmarkThresholds,
    }
/// Memory benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMemorySettings {
    /// Enable memory stress testing
    pub memory_stress_enabled: bool,
    /// Initial memory allocation size
    pub initial_allocation_mb: u64,
    /// Maximum memory usage limit
    pub max_memory_mb: u64,
    /// Memory allocation patterns
    pub allocation_patterns: Vec<MemoryAllocationPattern>,
    /// Enable memory leak detection
    pub leak_detection_enabled: bool,
    /// Memory pool testing
    pub pool_testing_enabled: bool,
    }
/// CPU benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkCpuSettings {
    /// Enable CPU stress testing
    pub cpu_stress_enabled: bool,
    /// CPU utilization target percentage
    pub target_cpu_percent: f64,
    /// Number of CPU-intensive threads
    pub cpu_threads: usize,
    /// CPU workload types to test
    pub workload_types: Vec<CpuWorkloadType>,
    /// Enable SIMD optimization testing
    pub simd_testing_enabled: bool,
    }
/// I/O benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkIoSettings {
    /// Enable I/O stress testing
    pub io_stress_enabled: bool,
    /// File buffer size for I/O operations
    pub buffer_size: usize,
    /// Number of concurrent I/O operations
    pub concurrent_io_ops: usize,
    /// I/O operation types to test
    pub io_operation_types: Vec<IoOperationType>,
    /// Enable disk usage monitoring
    pub disk_monitoring_enabled: bool,
    /// Target IOPS (Input/Output Operations Per Second)
    pub target_iops: u64,
    }
/// Network benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkNetworkSettings {
    /// Network bandwidth target (MB/s)
    pub target_bandwidth_mbps: f64,
    /// Network latency target (milliseconds)
    pub target_latency_ms: f64,
    /// Number of concurrent connections
    pub concurrent_connections: usize,
    /// Network protocols to test
    pub protocols: Vec<NetworkProtocolType>,
    /// Enable packet loss simulation
    pub packet_loss_simulation: bool,
    /// Maximum packet size to test
    pub max_packet_size: usize,
    }
/// Mock service benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMockingSettings {
    /// Mock services to test
    pub mock_services: Vec<BenchmarkMockService>,
    /// Mock response delay simulation
    pub response_delay_ms: u64,
    /// Mock failure rate (0.0 to 1.0)
    pub failure_rate: f32,
    /// Enable mock service validation
    pub validation_enabled: bool,
    /// Mock data generation patterns
    pub data_patterns: Vec<MockDataPattern>,
    }
/// Zero-copy optimization benchmark settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkZeroCopySettings {
    /// Enable zero-copy testing
    pub enabled: bool,
    /// Buffer sizes to test
    pub buffer_sizes: Vec<usize>,
    /// String processing patterns
    pub string_patterns: Vec<String>,
    /// Memory mapping testing
    pub memory_mapping_enabled: bool,
    /// Arc vs traditional clone comparison
    pub arc_comparison_enabled: bool,
    }
/// Stress testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStressSettings {
    /// Enable comprehensive stress testing
    pub enabled: bool,
    /// Stress test duration
    pub duration: Duration,
    /// Memory pressure levels to test
    pub memory_pressure_levels: Vec<f64>,
    /// CPU load levels to test
    pub cpu_load_levels: Vec<f64>,
    /// Enable resource exhaustion testing
    pub resource_exhaustion_enabled: bool,
    /// Recovery time after stress
    pub recovery_time: Duration,
    }
// ==================== SECTION ====================

/// Performance benchmark thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkThresholds {
    /// Maximum acceptable response time
    pub max_response_time: Duration,
    /// Minimum acceptable throughput (ops/sec)
    pub min_throughput: f64,
    /// Maximum acceptable error rate (percentage)
    pub max_error_rate: f64,
    /// Maximum acceptable CPU usage (percentage)
    pub max_cpu_percent: f64,
    /// Maximum acceptable memory usage (MB)
    pub max_memory_mb: u64,
    }
/// Memory allocation patterns for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryAllocationPattern {
    /// Sequential allocations
    Sequential,
    /// Random size allocations
    Random,
    /// Fixed size blocks
    FixedSize(usize),
    /// Exponential growth pattern
    Exponential,
    /// Fragmentation inducing pattern
    Fragmentation,
    }
/// CPU workload types for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CpuWorkloadType {
    /// Integer arithmetic operations
    IntegerArithmetic,
    /// Floating point operations
    FloatingPoint,
    /// Matrix multiplication
    MatrixMultiplication,
    /// String processing
    StringProcessing,
    /// Hash computation
    HashComputation,
    /// Cryptographic operations
    Cryptographic,
    }
/// I/O operation types for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IoOperationType {
    /// Sequential read operations
    SequentialRead,
    /// Sequential write operations
    SequentialWrite,
    /// Random read operations
    RandomRead,
    /// Random write operations
    RandomWrite,
    /// Mixed read/write operations
    Mixed,
    }
/// Network protocol types for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocolType {
    /// HTTP protocol testing
    Http,
    /// HTTPS protocol testing
    Https,
    /// WebSocket protocol testing
    WebSocket,
    /// TCP protocol testing
    Tcp,
    /// UDP protocol testing
    Udp,
    /// gRPC protocol testing
    Grpc,
    }
/// Mock service for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMockService {
    /// Service identifier
    pub id: String,
    /// Service name
    pub name: String,
    /// Service version
    pub version: String,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Mock service configuration
    pub configuration: BenchmarkMockConfiguration,
    }
/// Mock service configuration for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkMockConfiguration {
    /// Service name
    pub service_name: String,
    /// Database URL for mock
    pub database_url: String,
    /// Port number
    pub port: u16,
    /// Debug mode enabled
    pub debug_mode: bool,
    /// Enabled features
    pub features: Vec<String>,
    /// Environment name
    /// Response timeout
    pub timeout: Duration,
    }
/// Mock data generation patterns
#[cfg(test)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MockDataPattern {
    /// Fixed size data
    FixedSize(usize),
    /// Random size data within range
    RandomSize(usize, usize),
    /// JSON structured data
    JsonStructured,
    /// Binary data
    Binary,
    /// Text data with patterns
    TextPattern(String),
    }
/// Benchmark results structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    /// Benchmark name
    pub benchmark_name: String,
    /// Test duration
    pub duration: Duration,
    /// Total operations performed
    pub total_operations: u64,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Actual operations per second
    pub actual_ops_per_second: f64,
    /// Target operations per second
    pub target_ops_per_second: u64,
    /// CPU utilization percentage
    pub cpu_utilization_percent: f64,
    /// Memory usage in MB
    pub memory_usage_mb: f64,
    /// Response time percentiles
    pub response_time_percentiles: HashMap<String, f64>,
    /// Error rate percentage
    pub error_rate_percent: f64,
    /// Additional metrics
    pub additional_metrics: HashMap<String, f64>,
    }
// ==================== SECTION ====================

impl Default for BenchmarkExtensions {
    fn default() -> Self {
        Self {
            performance: BenchmarkPerformanceSettings::default(),
            memory: BenchmarkMemorySettings::default(),
            cpu: BenchmarkCpuSettings::default(),
            io: BenchmarkIoSettings::default(),
            network: BenchmarkNetworkSettings::default(),
            mocking: BenchmarkMockingSettings::default(),
            zero_copy: BenchmarkZeroCopySettings::default(),
            stress: BenchmarkStressSettings::default(),
    }
    }
    }

impl Default for BenchmarkPerformanceSettings {
    fn default() -> Self {
        Self {
            test_duration_seconds: 60,
            concurrent_threads: 50,
            target_ops_per_second: 1000,
            measurement_interval: Duration::from_secs(5),
            warmup_duration: Duration::from_secs(10),
            sample_size: 100,
            enable_profiling: true,
            thresholds: BenchmarkThresholds::default(),
    }
    }
    }

impl Default for BenchmarkMemorySettings {
    fn default() -> Self {
        Self {
            memory_stress_enabled: true,
            initial_allocation_mb: 100,
            max_memory_mb: 2048,
            allocation_patterns: vec![
                MemoryAllocationPattern::Sequential,
                MemoryAllocationPattern::Random,
                MemoryAllocationPattern::FixedSize(4096),
            ],
            leak_detection_enabled: true,
            pool_testing_enabled: true,
    }
    }
    }

impl Default for BenchmarkCpuSettings {
    fn default() -> Self {
        Self {
            cpu_stress_enabled: true,
            target_cpu_percent: 80.0,
            cpu_threads: num_cpus::get(),
            workload_types: vec![
                CpuWorkloadType::IntegerArithmetic,
                CpuWorkloadType::FloatingPoint,
                CpuWorkloadType::HashComputation,
            ],
            simd_testing_enabled: true,
    }
    }
    }

impl Default for BenchmarkIoSettings {
    fn default() -> Self {
        Self {
            io_stress_enabled: true,
            buffer_size: 8192,
            concurrent_io_ops: 20,
            io_operation_types: vec![
                IoOperationType::SequentialRead,
                IoOperationType::SequentialWrite,
                IoOperationType::RandomRead,
            ],
            disk_monitoring_enabled: true,
            target_iops: 10000,
    }
    }
    }

impl Default for BenchmarkNetworkSettings {
    fn default() -> Self {
        Self {
            target_bandwidth_mbps: 1000.0,
            target_latency_ms: 10.0,
            concurrent_connections: 100,
            protocols: vec![
                NetworkProtocolType::Http,
                NetworkProtocolType::Tcp,
                NetworkProtocolType::WebSocket,
            ],
            packet_loss_simulation: false,
            max_packet_size: 65536,
    }
    }
    }

impl Default for BenchmarkMockingSettings {
    fn default() -> Self {
        Self {
            mock_services: Vec::new(),
            response_delay_ms: 10,
            failure_rate: 0.01, // 1% failure rate
            validation_enabled: true,
            data_patterns: vec![
                MockDataPattern::JsonStructured,
                MockDataPattern::FixedSize(1024),
            ],
    }
    }
    }

impl Default for BenchmarkZeroCopySettings {
    fn default() -> Self {
        Self {
            enabled: true,
            buffer_sizes: vec![1024, 4096, 8192, 16384],
            string_patterns: vec![
                "prefix_test".to_string(),
                "no_prefix".to_string(),
                "prefix_benchmark".to_string(),
            ],
            memory_mapping_enabled: true,
            arc_comparison_enabled: true,
    }
    }
    }

impl Default for BenchmarkStressSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            duration: Duration::from_secs(300), // 5 minutes
            memory_pressure_levels: vec![0.5, 0.7, 0.9],
            cpu_load_levels: vec![0.5, 0.8, 0.95],
            resource_exhaustion_enabled: true,
            recovery_time: Duration::from_secs(30),
    }
    }
    }

impl Default for BenchmarkThresholds {
    fn default() -> Self {
        Self {
            max_response_time: Duration::from_millis(500),
            min_throughput: 100.0,
            max_error_rate: 1.0, // 1%
            max_cpu_percent: 95.0,
            max_memory_mb: 4096,
    }
    }
    }

impl Default for BenchmarkMockService {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "mock-benchmark-service".to_string(),
            version: "1.0.0".to_string(),
            endpoints: vec!["http://localhost:".to_string() + &env::var("NESTGATE_API_PORT").unwrap_or_else(|_| "8080".to_string()).to_string()],
            metadata: HashMap::new(),
            configuration: BenchmarkMockConfiguration::default(),
    }
    }
    }

impl Default for BenchmarkMockConfiguration {
    fn default() -> Self {
        Self {
            service_name: "nestgate-benchmark".to_string(),
// DEPRECATED: PostgreSQL database - migrate to capability-based persistence
// Capability-based discovery implemented
            database_url: "postgresql://localhost/benchmark".to_string(),
            port: 8080,
            debug_mode: false,
            features: vec!["api".to_string(), "benchmark".to_string()],
            timeout: Duration::from_secs(30),
    }
    }
    }

// ==================== SECTION ====================

/// Standardized Benchmark configuration
pub type UnifiedBenchmarkConfig = StandardDomainConfig<BenchmarkExtensions>;
// ==================== SECTION ====================

impl UnifiedBenchmarkConfig {
    /// Create a comprehensive benchmark configuration
    #[must_use]
    pub fn comprehensive() -> Self {
        let mut config = StandardDomainConfig::with_service(
            BenchmarkExtensions::default(),
            "nestgate-benchmark-suite",
            env!("CARGO_PKG_VERSION"),
        );

        // Configure service settings for benchmarking
        config.service.description = "NestGate Comprehensive Benchmark Test Suite".to_string();
        config.service.service_type = UnifiedServiceType::Custom("benchmark-framework".to_string());
        config.service.environment = "benchmarking".to_string();

        // Configure benchmark-specific network settings
        config.network.port = 0; // Random port for benchmarking
        config.network.bind_endpoint = "127.0.0.1".parse().unwrap_or_else(|e| {
            tracing::error!("Failed to parse IP endpoint: {:?}", e);
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1))
        );
        // config.network.enable_tls = false; // Field doesn't exist in current UnifiedNetworkConfig
        config.network.max_connections = 1000;

        // Configure benchmark-specific security settings
        config.security.require_auth = false; // Benchmarking typically doesn't require auth
        config.security.enable_tls = false;
        config.security.allowed_origins = vec!["*".to_string()];

        config
    }

    /// Create configuration for performance benchmarking
    #[must_use]
    pub fn performance_benchmarking() -> Self {
        let mut config = Self::comprehensive();
        config.service.name = "performance-benchmark-test".to_string();
        config.extensions.performance.enable_profiling = true;
        config.extensions.performance.concurrent_threads = 100;
        config.extensions.performance.target_ops_per_second = 10000;
        config
    }

    /// Create configuration for zero-copy optimization benchmarking
    #[must_use]
    pub fn zero_copy_benchmarking() -> Self {
        let mut config = Self::comprehensive();
        config.service.name = "zero-copy-benchmark-test".to_string();
        config.extensions.zero_copy.enabled = true;
        config.extensions.zero_copy.arc_comparison_enabled = true;
        config.extensions.zero_copy.memory_mapping_enabled = true;
        config
    }

    /// Create configuration for stress testing
    #[must_use]
    pub fn stress_benchmarking() -> Self {
        let mut config = Self::comprehensive();
        config.service.name = "stress-benchmark-test".to_string();
        config.extensions.stress.enabled = true;
        config.extensions.stress.duration = Duration::from_secs(600); // 10 minutes
        config.extensions.memory.memory_stress_enabled = true;
        config.extensions.cpu.cpu_stress_enabled = true;
        config.extensions.io.io_stress_enabled = true;
        config
    }

    /// Builder pattern for custom benchmark configurations
    pub fn builder() -> UnifiedBenchmarkConfigBuilder {
        UnifiedBenchmarkConfigBuilder::new()
    }
    }

/// Builder for UnifiedBenchmarkConfig
pub struct UnifiedBenchmarkConfigBuilder {
    config: UnifiedBenchmarkConfig,
    }
impl UnifiedBenchmarkConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: UnifiedBenchmarkConfig::comprehensive(),
    }
    }

    #[must_use]
    pub fn test_name(mut self, name: &str) -> Self {
        self.config.service.name = name.to_string();
        self
    }

    #[must_use]
    pub fn test_duration(mut self, duration: Duration) -> Self {
        self.config.extensions.performance.measurement_interval = duration;
        self
    }

    #[must_use]
    pub fn concurrent_threads(mut self, threads: usize) -> Self {
        self.config.extensions.performance.concurrent_threads = threads;
        self
    }

    #[must_use]
    pub fn target_ops_per_second(mut self, ops: u64) -> Self {
        self.config.extensions.performance.target_ops_per_second = ops;
        self
    }

    #[must_use]
    pub fn enable_memory_stress(mut self, enabled: bool) -> Self {
        self.config.extensions.memory.memory_stress_enabled = enabled;
        self
    }

    #[must_use]
    pub fn enable_cpu_stress(mut self, enabled: bool) -> Self {
        self.config.extensions.cpu.cpu_stress_enabled = enabled;
        self
    }

    #[must_use]
    pub fn enable_io_stress(mut self, enabled: bool) -> Self {
        self.config.extensions.io.io_stress_enabled = enabled;
        self
    }

    #[must_use]
    pub fn enable_zero_copy_testing(mut self, enabled: bool) -> Self {
        self.config.extensions.zero_copy.enabled = enabled;
        self
    }

    #[must_use]
    pub fn enable_profiling(mut self, enabled: bool) -> Self {
        self.config.extensions.performance.enable_profiling = enabled;
        self
    }

    pub fn build(self) -> UnifiedBenchmarkConfig {
        self.config
    }
    }
