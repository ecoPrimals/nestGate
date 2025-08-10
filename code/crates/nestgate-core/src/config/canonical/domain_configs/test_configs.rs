/// Test Configuration Domain
///
/// Replaces: UnifiedTestConfig, TestExecutionConfig, TestMockConfig, TestPerformanceConfig,
/// TestNetworkConfig, TestSecurityConfig, and 15+ other test config structures
use super::CanonicalDomainConfig;
use crate::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL TEST CONFIGURATION**
/// Replaces: UnifiedTestConfig, TestExecutionConfig, TestMockConfig, TestPerformanceConfig,
/// TestNetworkConfig, TestSecurityConfig, and 15+ other test config structures
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalTestConfig {
    /// Test execution settings
    pub execution: TestExecution,
    /// Mock service settings
    pub mocking: TestMocking,
    /// Performance testing settings
    pub performance: TestPerformance,
    /// Security testing settings
    pub security: TestSecurity,
    /// Network testing settings
    pub network: TestNetwork,
    /// Integration testing settings
    pub integration: TestIntegration,
    /// Chaos engineering settings
    pub chaos: TestChaos,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, serde_json::Value>,
}

impl CanonicalDomainConfig for CanonicalTestConfig {
    fn validate(&self) -> Result<()> {
        // Validate test execution settings
        if self.execution.max_concurrent_tests == 0 {
            return Err(NestGateError::config_error(
                "max_concurrent_tests",
                "must be greater than 0",
                Some("Set a positive integer value".to_string()),
            ));
        }

        // Validate timeout settings
        if self.execution.default_timeout.as_secs() == 0 {
            return Err(NestGateError::config_error(
                "default_timeout",
                "must be greater than 0",
                Some("Set a positive duration value".to_string()),
            ));
        }

        // Validate performance settings
        if self.performance.target_throughput == 0.0 {
            return Err(NestGateError::config_error(
                "target_throughput",
                "must be greater than 0",
                Some("Set a positive throughput value".to_string()),
            ));
        }

        Ok(())
    }

    fn merge(mut self, other: Self) -> Self {
        // Merge execution settings
        self.execution.max_concurrent_tests = other
            .execution
            .max_concurrent_tests
            .max(self.execution.max_concurrent_tests);
        self.execution.default_timeout = other
            .execution
            .default_timeout
            .max(self.execution.default_timeout);

        // Merge environment overrides
        self.environment_overrides
            .extend(other.environment_overrides);

        self
    }

    fn from_environment() -> Result<Self> {
        Ok(Self::default())
    }

    fn domain() -> &'static str {
        "test"
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "execution": {
                    "type": "object",
                    "description": "Test execution settings"
                },
                "mocking": {
                    "type": "object",
                    "description": "Mock service settings"
                },
                "performance": {
                    "type": "object",
                    "description": "Performance testing settings"
                }
            },
            "required": ["execution", "mocking", "performance"]
        })
    }
}

// ==================== TEST CONFIGURATION STRUCTURES ====================

/// Test execution configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestExecution {
    pub max_duration: Duration,
    pub parallel_execution: bool,
    pub retry_attempts: u32,
    pub isolation_level: TestIsolationLevel,
    pub cleanup_strategy: TestCleanupStrategy,
    pub resource_limits: TestResourceLimits,
    pub max_concurrent_tests: usize,
    pub default_timeout: Duration,
}

/// Test mocking configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestMocking {
    pub enable_mocking: bool,
    pub default_response_delay: Duration,
    pub failure_rate: f64,
    pub mock_services: HashMap<String, MockServiceConfig>,
    pub global_settings: MockGlobalSettings,
}

/// Test performance configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestPerformance {
    pub enable_performance_tests: bool,
    pub load_patterns: Vec<LoadPattern>,
    pub stress_limits: StressLimits,
    pub benchmark_suites: Vec<String>,
    pub metrics_collection: bool,
    pub target_throughput: f64,
}

/// Test security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestSecurity {
    pub enable_auth: bool,
    pub test_credentials: HashMap<String, String>,
    pub security_headers: HashMap<String, String>,
    pub enable_encryption: bool,
    pub penetration_testing: bool,
}

/// Test network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestNetwork {
    pub test_endpoints: Vec<String>,
    pub network_simulation: bool,
    pub latency_simulation: Duration,
    pub packet_loss_rate: f64,
    pub bandwidth_limit: u64,
    // Legacy field names for backward compatibility
    pub endpoints: HashMap<String, String>,
    pub connection_timeout: Duration,
    pub request_timeout: Duration,
    pub enable_tls: bool,
    pub custom_headers: HashMap<String, String>,
}

/// Test integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestIntegration {
    pub enable_integration_tests: bool,
    pub external_services: Vec<String>,
    pub database_cleanup: bool,
    pub test_data_isolation: bool,
    // Legacy field names for backward compatibility
    pub test_datasets: Vec<String>,
    pub enable_integration: bool,
    pub biomeos_settings: BiomeOSTestSettings,
    pub zfs_settings: ZfsTestSettings,
}

/// Test chaos configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestChaos {
    pub enable_chaos_engineering: bool,
    pub chaos_types: Vec<ChaosType>,
    pub blast_radius: ChaosBlastRadius,
    pub failure_percentage: f64,
    pub recovery_time: Duration,
    pub cleanup_after_tests: bool,
    // Legacy field names for backward compatibility
    pub enable_chaos_testing: bool,
    pub failure_injection_rate: f64,
}

// ==================== SUPPORTING TYPES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestIsolationLevel {
    None,
    Process,
    Container,
    VM,
    Full, // Alias for VM for backward compatibility
}

impl Default for TestIsolationLevel {
    fn default() -> Self {
        Self::Process
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestCleanupStrategy {
    None,
    AfterEach,
    AfterAll,
    OnFailure,
    Complete, // Alias for AfterAll for backward compatibility
}

impl Default for TestCleanupStrategy {
    fn default() -> Self {
        Self::AfterEach
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResourceLimits {
    pub max_memory_mb: u64,
    pub max_cpu_cores: u32,
    pub max_disk_space_mb: u64,
    pub max_network_connections: u32,
    // Legacy field names for backward compatibility
    pub max_disk_mb: u64,
    pub max_network_mbps: u32,
}

impl Default for TestResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024,
            max_cpu_cores: 4,
            max_disk_space_mb: 10240,
            max_network_connections: 100,
            max_disk_mb: 10240,
            max_network_mbps: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MockServiceConfig {
    pub service_name: String,
    pub endpoints: Vec<String>,
    pub response_templates: HashMap<String, String>,
    pub failure_modes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockGlobalSettings {
    pub enable_request_logging: bool,
    pub log_level: String,
    pub persist_state: bool,
    // Legacy field names for backward compatibility
    pub enable_logging: bool,
    pub consistency_level: MockConsistencyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadPattern {
    pub pattern_type: String,
    pub duration: Duration,
    pub target_rps: u64,
    pub ramp_up_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressLimits {
    pub max_connections: u32,
    pub max_requests_per_second: u64,
    pub memory_pressure_threshold: f64,
    pub cpu_pressure_threshold: f64,
    // Legacy field names for backward compatibility
    pub max_concurrent_requests: u32,
    pub max_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChaosType {
    NetworkPartition,
    ServiceFailure,
    ResourceExhaustion,
    LatencyInjection,
    DataCorruption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChaosBlastRadius {
    Service,
    Node,
    Cluster,
    Region,
}

impl Default for ChaosBlastRadius {
    fn default() -> Self {
        Self::Service
    }
}

// ==================== ADDITIONAL LEGACY TYPES FOR BUILDERS COMPATIBILITY ====================

/// Legacy test settings for BiomeOS integration (used by builders)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiomeOSTestSettings {
    pub endpoint: String,
    pub enable_integration: bool,
    pub test_credentials: Option<String>,
}

/// Legacy test settings for ZFS integration (used by builders)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsTestSettings {
    pub test_pool_name: String,
    pub enable_zfs_tests: bool,
    pub test_dataset_prefix: String,
    pub cleanup_after_tests: bool,
}

/// Legacy consistency level for mock services (used by builders)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MockConsistencyLevel {
    Eventual,
    Strong,
    Weak,
}

impl Default for MockConsistencyLevel {
    fn default() -> Self {
        Self::Eventual
    }
}

// Default implementations for backward compatibility

impl Default for TestNetwork {
    fn default() -> Self {
        Self {
            test_endpoints: vec!["http://localhost:8080".to_string()],
            network_simulation: false,
            latency_simulation: Duration::from_millis(0),
            packet_loss_rate: 0.0,
            bandwidth_limit: 0,
            endpoints: HashMap::new(),
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(10),
            enable_tls: false,
            custom_headers: HashMap::new(),
        }
    }
}

impl Default for TestIntegration {
    fn default() -> Self {
        Self {
            enable_integration_tests: true,
            external_services: Vec::new(),
            database_cleanup: true,
            test_data_isolation: true,
            test_datasets: Vec::new(),
            enable_integration: false,
            biomeos_settings: BiomeOSTestSettings::default(),
            zfs_settings: ZfsTestSettings::default(),
        }
    }
}

impl Default for TestChaos {
    fn default() -> Self {
        Self {
            enable_chaos_engineering: false,
            chaos_types: Vec::new(),
            blast_radius: ChaosBlastRadius::Service,
            failure_percentage: 0.1,
            recovery_time: Duration::from_secs(30),
            cleanup_after_tests: true,
            enable_chaos_testing: false,
            failure_injection_rate: 0.0,
        }
    }
}

impl Default for MockGlobalSettings {
    fn default() -> Self {
        Self {
            enable_request_logging: true,
            log_level: "INFO".to_string(),
            persist_state: false,
            enable_logging: true,
            consistency_level: MockConsistencyLevel::Eventual,
        }
    }
}

impl Default for StressLimits {
    fn default() -> Self {
        Self {
            max_connections: 100,
            max_requests_per_second: 1000,
            memory_pressure_threshold: 80.0,
            cpu_pressure_threshold: 80.0,
            max_concurrent_requests: 100,
            max_duration: Duration::from_secs(300),
        }
    }
}
