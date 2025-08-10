use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// **TEST EXECUTION CONFIGURATION MODULE**
///
/// Extracted from the monolithic test_config.rs to achieve better separation
/// of concerns. Handles test execution, resource limits, and timeouts.
use std::time::Duration;

/// **TEST EXECUTION CONFIGURATION**
/// Consolidated execution settings for all test scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExecutionConfig {
    /// Test environment type
    pub environment: TestEnvironment,
    /// Enable parallel test execution
    pub parallel_execution: bool,
    /// Test isolation level
    pub isolation_level: TestIsolationLevel,
    /// Cleanup strategy after tests
    pub cleanup_strategy: TestCleanupStrategy,
    /// Maximum test duration
    pub max_duration: Duration,
    /// Maximum concurrent tests
    pub max_concurrent_tests: usize,
    /// Number of retry attempts for flaky tests
    pub retry_attempts: u32,
    /// Timeout configurations
    pub timeouts: TestTimeouts,
    /// Retry configurations
    pub retries: TestRetryConfig,
    /// Resource limits
    pub resource_limits: TestResourceLimits,
}

/// Test environment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestEnvironment {
    /// Unit tests - isolated, fast
    Unit,
    /// Integration tests - with external services
    Integration,
    /// End-to-end tests - full system
    E2E,
    /// Performance tests - load and stress
    Performance,
    /// Chaos tests - fault injection
    Chaos,
}

/// Test isolation levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestIsolationLevel {
    /// No isolation - tests may interfere
    None,
    /// Process isolation - separate processes
    Process,
    /// Container isolation - separate containers
    Container,
    /// Full isolation - separate VMs/environments
    Full,
}

/// Test cleanup strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestCleanupStrategy {
    /// No cleanup - fastest but may leave artifacts
    None,
    /// Cleanup per test - cleanest but slower
    PerTest,
    /// Cleanup per suite - balanced approach
    PerSuite,
    /// Cleanup on failure only
    OnFailure,
}

/// Comprehensive timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestTimeouts {
    /// Short operations (< 5 seconds)
    pub short: Duration,
    /// Medium operations (5-30 seconds)
    pub medium: Duration,
    /// Long operations (30+ seconds)
    pub long: Duration,
    /// Network connection timeout
    pub connection: Duration,
    /// Individual request timeout
    pub request: Duration,
}

/// Test retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Exponential backoff multiplier
    pub backoff_multiplier: f64,
}

/// Resource limits for tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResourceLimits {
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    /// Maximum CPU percentage
    pub max_cpu_percent: f64,
    /// Maximum disk usage in MB
    pub max_disk_mb: u64,
    /// Maximum network bandwidth in Mbps
    pub max_network_mbps: f64,
}

/// Test environment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestEnvironmentSettings {
    /// Test data directory
    pub test_data_dir: String,
    /// Temporary directory for tests
    pub temp_dir: String,
    /// Test log level
    pub log_level: String,
    /// Environment variables for tests
    pub env_vars: HashMap<String, String>,
}

impl Default for TestExecutionConfig {
    fn default() -> Self {
        Self {
            environment: TestEnvironment::Unit,
            parallel_execution: true,
            isolation_level: TestIsolationLevel::Process,
            cleanup_strategy: TestCleanupStrategy::PerTest,
            max_duration: Duration::from_secs(300), // 5 minutes
            max_concurrent_tests: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            retry_attempts: 3,
            timeouts: TestTimeouts::default(),
            retries: TestRetryConfig::default(),
            resource_limits: TestResourceLimits::default(),
        }
    }
}

impl Default for TestTimeouts {
    fn default() -> Self {
        Self {
            short: Duration::from_secs(5),
            medium: Duration::from_secs(30),
            long: Duration::from_secs(300),
            connection: Duration::from_secs(10),
            request: Duration::from_secs(30),
        }
    }
}

impl Default for TestRetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

impl Default for TestResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024, // 1GB
            max_cpu_percent: 80.0,
            max_disk_mb: 5120, // 5GB
            max_network_mbps: 100.0,
        }
    }
}

impl TestExecutionConfig {
    /// Create configuration for unit tests
    pub fn unit_tests() -> Self {
        Self {
            environment: TestEnvironment::Unit,
            max_duration: Duration::from_secs(60),
            max_concurrent_tests: std::thread::available_parallelism()
                .map(|n| n.get() * 2)
                .unwrap_or(8),
            isolation_level: TestIsolationLevel::Process,
            ..Default::default()
        }
    }

    /// Create configuration for integration tests
    pub fn integration_tests() -> Self {
        Self {
            environment: TestEnvironment::Integration,
            max_duration: Duration::from_secs(600), // 10 minutes
            isolation_level: TestIsolationLevel::Container,
            cleanup_strategy: TestCleanupStrategy::PerSuite,
            ..Default::default()
        }
    }

    /// Create configuration for e2e tests
    pub fn e2e_tests() -> Self {
        Self {
            environment: TestEnvironment::E2E,
            max_duration: Duration::from_secs(1800), // 30 minutes
            max_concurrent_tests: 2,                 // E2E tests are resource intensive
            isolation_level: TestIsolationLevel::Full,
            parallel_execution: false, // E2E tests often require sequential execution
            ..Default::default()
        }
    }

    /// Create configuration for performance tests
    pub fn performance_tests() -> Self {
        Self {
            environment: TestEnvironment::Performance,
            max_duration: Duration::from_secs(3600), // 1 hour
            max_concurrent_tests: 1,                 // Performance tests need dedicated resources
            isolation_level: TestIsolationLevel::Full,
            parallel_execution: false,
            resource_limits: TestResourceLimits {
                max_memory_mb: 8192, // 8GB for performance tests
                max_cpu_percent: 95.0,
                max_disk_mb: 20480, // 20GB
                max_network_mbps: 1000.0,
            },
            ..Default::default()
        }
    }
}
