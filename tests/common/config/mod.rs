/// **REFACTORED TEST CONFIGURATION MODULE SYSTEM**
///
/// Refactored from the monolithic 1589-line test_config.rs into focused modules
/// for better maintainability and separation of concerns.
///
/// **Architecture**:
/// - execution.rs: Test execution, timeouts, and resource management
/// - mocking.rs: Mock service configuration and test doubles
/// - performance.rs: Performance testing and chaos engineering
/// - Main unified config orchestrates all focused modules
// Core configuration modules
pub mod execution;
pub mod mocking;
pub mod performance;

// Re-export key types for convenience
pub use execution::{
    TestCleanupStrategy, TestEnvironment, TestEnvironmentSettings, TestExecutionConfig,
    TestIsolationLevel, TestResourceLimits, TestRetryConfig, TestTimeouts,
};

pub use mocking::{
    MockConsistencyLevel, MockFailureConfig, MockFailureType, MockGlobalSettings,
    MockPerformanceConfig, MockResponseBehavior, TestMockConfig, TestMockingSettings,
    UnifiedMockServiceConfig,
};

pub use performance::{
    ChaosType, LoadPattern, LoadSpec, MetricType, MetricsExportFormat, PerformanceMetricsConfig,
    PerformanceThresholds, StressFailureCriteria, StressResourceLimits, StressTestConfig,
    TestChaosSettings, TestPerformanceSettings,
};

use nestgate_core::smart_abstractions::prelude::*;
use nestgate_core::unified_enums::UnifiedServiceType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **UNIFIED TEST CONFIGURATION**
///
/// Streamlined configuration that orchestrates all focused modules.
/// Significantly reduced from the original monolithic implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTestConfig {
    /// Test execution configuration
    pub execution: TestExecutionConfig,
    /// Mock service configuration
    pub mock_services: TestMockConfig,
    /// Network and security configuration
    pub network: TestNetworkConfig,
    /// Security and authentication configuration
    pub security: TestSecurityConfig,
    /// Extended configuration for specialized testing
    pub extensions: TestExtensions,
}

/// Network configuration for tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestNetworkConfig {
    /// Test endpoints
    pub endpoints: HashMap<String, String>,
    /// Connection timeouts
    pub connection_timeout: Duration,
    /// Request timeouts
    pub request_timeout: Duration,
    /// Enable TLS for tests
    pub enable_tls: bool,
    /// Custom headers for requests
    pub custom_headers: HashMap<String, String>,
}

/// Security configuration for tests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSecurityConfig {
    /// Enable authentication in tests
    pub enable_auth: bool,
    /// Test credentials
    pub credentials: HashMap<String, String>,
    /// Security headers
    pub security_headers: HashMap<String, String>,
    /// Enable encryption
    pub enable_encryption: bool,
}

/// Extended configuration for specialized testing scenarios
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestExtensions {
    /// Performance testing settings
    pub performance: TestPerformanceSettings,
    /// Chaos engineering settings
    pub chaos: TestChaosSettings,
    /// Integration testing settings
    pub integration: TestIntegrationSettings,
    /// BiomeOS specific settings
    pub biomeos: BiomeOSTestSettings,
    /// ZFS specific settings
    pub zfs: ZfsTestSettings,
    /// Environment settings
    pub environment: TestEnvironmentSettings,
}

/// Integration testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestIntegrationSettings {
    /// External services to test against
    pub external_services: Vec<String>,
    /// Test datasets
    pub test_datasets: Vec<String>,
    /// BiomeOS integration settings
    pub biomeos: BiomeOSTestSettings,
    /// ZFS integration settings
    pub zfs: ZfsTestSettings,
    /// Enable integration tests
    pub enable_integration: bool,
    /// BiomeOS test credentials
    pub test_credentials: Option<String>,
}

/// BiomeOS specific test settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSTestSettings {
    /// BiomeOS test endpoint
    pub endpoint: String,
    /// Enable BiomeOS integration
    pub enable_integration: bool,
    /// Test credentials
    pub test_credentials: Option<String>,
}

/// ZFS specific test settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsTestSettings {
    /// Test pool name
    pub test_pool_name: String,
    /// Enable ZFS tests
    pub enable_zfs_tests: bool,
    /// ZFS test dataset prefix
    pub test_dataset_prefix: String,
    /// Cleanup test data after tests
    pub cleanup_after_tests: bool,
}

/// Default implementations for all configuration types
impl SmartDefault for UnifiedTestConfig {
    fn smart_default() -> Self {
        Self {
            execution: TestExecutionConfig::smart_default(),
            mock_services: TestMockConfig::smart_default(),
            network: TestNetworkConfig::smart_default(),
            security: TestSecurityConfig::smart_default(),
            extensions: TestExtensions::smart_default(),
        }
    }
}

impl Default for UnifiedTestConfig {
    fn default() -> Self {
        Self::smart_default()
    }
}

impl Default for TestNetworkConfig {
    fn default() -> Self {
        Self {
            endpoints: HashMap::new(),
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            enable_tls: false,
            custom_headers: HashMap::new(),
        }
    }
}

impl Default for TestSecurityConfig {
    fn default() -> Self {
        Self {
            enable_auth: false,
            credentials: HashMap::new(),
            security_headers: HashMap::new(),
            enable_encryption: false,
        }
    }
}

impl Default for TestExtensions {
    fn default() -> Self {
        Self {
            performance: TestPerformanceSettings::default(),
            chaos: TestChaosSettings::default(),
            integration: TestIntegrationSettings::default(),
            biomeos: BiomeOSTestSettings::default(),
            zfs: ZfsTestSettings::default(),
            environment: TestEnvironmentSettings::default(),
        }
    }
}

impl Default for TestIntegrationSettings {
    fn default() -> Self {
        Self {
            external_services: Vec::new(),
            test_datasets: Vec::new(),
            biomeos: BiomeOSTestSettings::default(),
            zfs: ZfsTestSettings::default(),
            enable_integration: false,
            test_credentials: None,
        }
    }
}

impl Default for BiomeOSTestSettings {
    fn default() -> Self {
        Self {
            endpoint: std::env::var("BIOMEOS_TEST_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            enable_integration: false,
            test_credentials: None,
        }
    }
}

impl Default for ZfsTestSettings {
    fn default() -> Self {
        Self {
            test_pool_name: "test_pool".to_string(),
            enable_zfs_tests: false,
            test_dataset_prefix: "test_".to_string(),
            cleanup_after_tests: true,
        }
    }
}

/// Configuration builders for common test scenarios
impl UnifiedTestConfig {
    /// Create configuration for development testing
    pub fn development() -> Self {
        Self {
            execution: TestExecutionConfig::unit_tests(),
            extensions: TestExtensions {
                performance: TestPerformanceSettings::light_load(),
                chaos: TestChaosSettings::default(), // No chaos in development
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create configuration for CI/CD pipeline
    pub fn ci_cd() -> Self {
        Self {
            execution: TestExecutionConfig::integration_tests(),
            extensions: TestExtensions {
                performance: TestPerformanceSettings::light_load(),
                chaos: TestChaosSettings::light_chaos(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create configuration for production-like testing
    pub fn production_like() -> Self {
        Self {
            execution: TestExecutionConfig::e2e_tests(),
            extensions: TestExtensions {
                performance: TestPerformanceSettings::heavy_load(),
                chaos: TestChaosSettings::comprehensive_chaos(),
                integration: TestIntegrationSettings {
                    enable_integration: true,
                    external_services: vec![
                        "biomeos".to_string(),
                        "zfs".to_string(),
                        "network".to_string(),
                    ],
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create configuration for performance testing
    pub fn performance_testing() -> Self {
        Self {
            execution: TestExecutionConfig::performance_tests(),
            extensions: TestExtensions {
                performance: TestPerformanceSettings::stress_test(),
                chaos: TestChaosSettings::default(), // No chaos during pure performance tests
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create configuration for chaos engineering
    pub fn chaos_testing() -> Self {
        Self {
            execution: TestExecutionConfig::integration_tests(),
            extensions: TestExtensions {
                performance: TestPerformanceSettings::default(),
                chaos: TestChaosSettings::comprehensive_chaos(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
