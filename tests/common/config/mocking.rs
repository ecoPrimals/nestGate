use nestgate_core::unified_enums::{UnifiedHealthStatus, UnifiedServiceType};
use serde::{Deserialize, Serialize};
/// **TEST MOCKING CONFIGURATION MODULE**
///
/// Extracted from the monolithic test_config.rs to achieve better separation
/// of concerns. Handles mock service configuration and test doubles.
use std::collections::HashMap;
use std::time::Duration;

/// **UNIFIED MOCK SERVICE CONFIGURATION**
/// Comprehensive configuration for all mock services in tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMockConfig {
    /// Default mock service configuration
    pub default_service: UnifiedMockServiceConfig,
    /// Service-specific mock configurations
    pub service_configs: HashMap<String, UnifiedMockServiceConfig>,
    /// Global mock behavior settings
    pub global_settings: MockGlobalSettings,
}

/// **COMPREHENSIVE MOCK SERVICE CONFIGURATION**
/// Consolidated configuration for individual mock services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedMockServiceConfig {
    /// Service identification
    pub service_id: String,
    /// Service type being mocked
    pub service_type: UnifiedServiceType,
    /// Mock service endpoint
    pub endpoint: String,
    /// Health status to simulate
    pub health_status: UnifiedHealthStatus,
    /// Response behavior configuration
    pub response_behavior: MockResponseBehavior,
    /// Failure simulation configuration
    pub failure_simulation: MockFailureConfig,
    /// Custom response mappings
    pub custom_responses: HashMap<String, serde_json::Value>,
    /// Performance characteristics
    pub performance: MockPerformanceConfig,
}

/// Mock response behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockResponseBehavior {
    /// Response delay simulation
    pub response_delay: Duration,
    /// Delay variance for realistic behavior
    pub delay_variance: Duration,
    /// Response consistency level
    pub consistency: MockConsistencyLevel,
}

/// Mock failure simulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockFailureConfig {
    /// Enable failure simulation
    pub enable_failures: bool,
    /// Failure probability (0.0 to 1.0)
    pub failure_rate: f64,
    /// Types of failures to simulate
    pub failure_types: Vec<MockFailureType>,
    /// Recovery time after failures
    pub recovery_time: Duration,
}

/// Mock performance characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockPerformanceConfig {
    /// Simulated latency
    pub latency: Duration,
    /// Throughput limitation (requests per second)
    pub max_rps: f64,
    /// Memory usage simulation
    pub memory_usage_mb: u64,
    /// CPU usage simulation percentage
    pub cpu_usage_percent: f64,
}

/// Global mock settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockGlobalSettings {
    /// Enable all mocking
    pub enable_mocking: bool,
    /// Mock endpoint configurations
    pub mock_endpoints: HashMap<String, String>,
    /// Mock data directory
    pub mock_data_dir: String,
    /// Default response delay
    pub mock_response_delay: Duration,
    /// Mock services registry
    pub mock_services: HashMap<String, UnifiedMockServiceConfig>,
    /// Global failure rate
    pub mock_failure_rate: f64,
}

/// Mock consistency levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MockConsistencyLevel {
    /// Always return the same response
    Consistent,
    /// Vary responses slightly
    SlightlyVariable,
    /// Significantly variable responses
    HighlyVariable,
    /// Completely random responses
    Random,
}

/// Types of failures to simulate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MockFailureType {
    /// Network timeout
    Timeout,
    /// Connection refused
    ConnectionRefused,
    /// HTTP 500 errors
    InternalServerError,
    /// HTTP 404 errors
    NotFound,
    /// HTTP 429 rate limiting
    RateLimited,
    /// Partial response corruption
    CorruptedResponse,
    /// Service unavailable
    ServiceUnavailable,
}

/// Test mocking settings for different scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestMockingSettings {
    /// Enable mocking globally
    pub enable_mocking: bool,
    /// Mock endpoint configurations
    pub mock_endpoints: HashMap<String, String>,
    /// Mock data directory
    pub mock_data_dir: String,
    /// Default response delay
    pub mock_response_delay: Duration,
    /// Mock services configuration
    pub mock_services: HashMap<String, UnifiedMockServiceConfig>,
    /// Global failure rate for chaos testing
    pub mock_failure_rate: f64,
}

impl Default for TestMockConfig {
    fn default() -> Self {
        Self {
            default_service: UnifiedMockServiceConfig::default(),
            service_configs: HashMap::new(),
            global_settings: MockGlobalSettings::default(),
        }
    }
}

impl Default for UnifiedMockServiceConfig {
    fn default() -> Self {
        Self {
            service_id: "mock-service".to_string(),
            service_type: UnifiedServiceType::Testing,
            endpoint: "http://localhost:8080".to_string(),
            health_status: UnifiedHealthStatus::Healthy,
            response_behavior: MockResponseBehavior::default(),
            failure_simulation: MockFailureConfig::default(),
            custom_responses: HashMap::new(),
            performance: MockPerformanceConfig::default(),
        }
    }
}

impl Default for MockResponseBehavior {
    fn default() -> Self {
        Self {
            response_delay: Duration::from_millis(10),
            delay_variance: Duration::from_millis(5),
            consistency: MockConsistencyLevel::Consistent,
        }
    }
}

impl Default for MockFailureConfig {
    fn default() -> Self {
        Self {
            enable_failures: false,
            failure_rate: 0.0,
            failure_types: Vec::new(),
            recovery_time: Duration::from_secs(5),
        }
    }
}

impl Default for MockPerformanceConfig {
    fn default() -> Self {
        Self {
            latency: Duration::from_millis(10),
            max_rps: 1000.0,
            memory_usage_mb: 100,
            cpu_usage_percent: 5.0,
        }
    }
}

impl Default for MockGlobalSettings {
    fn default() -> Self {
        Self {
            enable_mocking: true,
            mock_endpoints: HashMap::new(),
            mock_data_dir: std::env::var("NESTGATE_TEST_MOCK_DIR")
                .unwrap_or_else(|_| "./tests/fixtures/mocks".to_string()),
            mock_response_delay: Duration::from_millis(10),
            mock_services: HashMap::new(),
            mock_failure_rate: 0.0,
        }
    }
}

impl Default for TestMockingSettings {
    fn default() -> Self {
        Self {
            enable_mocking: true,
            mock_endpoints: HashMap::new(),
            mock_data_dir: std::env::var("NESTGATE_TEST_MOCK_DIR")
                .unwrap_or_else(|_| "./tests/fixtures/mocks".to_string()),
            mock_response_delay: Duration::from_millis(10),
            mock_services: HashMap::new(),
            mock_failure_rate: 0.0,
        }
    }
}

impl UnifiedMockServiceConfig {
    /// Create a mock configuration for ZFS services
    pub fn zfs_service() -> Self {
        Self {
            service_id: "mock-zfs".to_string(),
            service_type: UnifiedServiceType::Storage,
            endpoint: "http://localhost:8081".to_string(),
            health_status: UnifiedHealthStatus::Healthy,
            performance: MockPerformanceConfig {
                latency: Duration::from_millis(50), // ZFS operations are slower
                max_rps: 100.0,
                memory_usage_mb: 500,
                cpu_usage_percent: 20.0,
            },
            ..Default::default()
        }
    }

    /// Create a mock configuration for network services
    pub fn network_service() -> Self {
        Self {
            service_id: "mock-network".to_string(),
            service_type: UnifiedServiceType::Network,
            endpoint: "http://localhost:8082".to_string(),
            health_status: UnifiedHealthStatus::Healthy,
            performance: MockPerformanceConfig {
                latency: Duration::from_millis(5), // Network is fast
                max_rps: 10000.0,
                memory_usage_mb: 50,
                cpu_usage_percent: 2.0,
            },
            ..Default::default()
        }
    }

    /// Create a mock configuration for security services
    pub fn security_service() -> Self {
        Self {
            service_id: "mock-security".to_string(),
            service_type: UnifiedServiceType::Security,
            endpoint: "http://localhost:8083".to_string(),
            health_status: UnifiedHealthStatus::Healthy,
            performance: MockPerformanceConfig {
                latency: Duration::from_millis(100), // Security operations take time
                max_rps: 50.0,
                memory_usage_mb: 200,
                cpu_usage_percent: 15.0,
            },
            ..Default::default()
        }
    }

    /// Create a mock with simulated failures for chaos testing
    pub fn chaos_service() -> Self {
        Self {
            service_id: "mock-chaos".to_string(),
            service_type: UnifiedServiceType::Testing,
            endpoint: "http://localhost:8084".to_string(),
            health_status: UnifiedHealthStatus::Degraded,
            failure_simulation: MockFailureConfig {
                enable_failures: true,
                failure_rate: 0.1, // 10% failure rate
                failure_types: vec![
                    MockFailureType::Timeout,
                    MockFailureType::InternalServerError,
                    MockFailureType::ServiceUnavailable,
                ],
                recovery_time: Duration::from_secs(10),
            },
            ..Default::default()
        }
    }
}
