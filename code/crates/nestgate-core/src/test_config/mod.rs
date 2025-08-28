use std::collections::HashMap;
///
/// This module provides a modular test configuration system, replacing the large
/// unified_test_config_consolidation.rs file with focused, maintainable modules.
///
/// **ELIMINATES 50+ FRAGMENTED TEST CONFIGS** through systematic consolidation
/// into domain-specific modules.
///
/// **PROVIDES**:
/// - Single UnifiedTestConfig as the root test configuration
/// - Domain-specific test extensions using StandardDomainConfig pattern
/// - Environment-driven test configuration loading
/// - Test suite orchestration and coordination
/// - Mock service and test double management
// NOTE: StandardDomainConfig pattern has been moved to unified_final_config
// use crate::unified_config_consolidation::StandardDomainConfig;
use serde::{Deserialize, Serialize};

// ==================== SECTION ====================

pub mod chaos;
pub mod environment;
pub mod execution;
pub mod integration;
pub mod performance;
pub mod security;

// Re-export all configuration types for easy access
pub use chaos::{
    FailureInjectionConfig, NetworkChaosConfig, RecoveryTestConfig, ResourceChaosConfig,
    TestChaosConfig,
};
pub use environment::{ContainerConfig, NetworkConfig, TestEnvironmentConfig};
pub use execution::{TestCleanupConfig, TestExecutionConfig, TestReportingConfig};
pub use integration::{
    CircuitBreakerConfig, DatabaseConfig, ExternalServiceConfig, HealthCheckConfig,
    LoadBalancingConfig, MessageQueueConfig, MigrationConfig, OAuthConfig, RetryConfig,
    ServiceAuthConfig, ServiceDiscoveryConfig, ServiceMeshConfig, TestIntegrationConfig,
};
pub use performance::{
    BenchmarkingConfig, LoadTestScenario, LoadTestingConfig, PerformanceMetricsConfig,
    StressTestingConfig, TestPerformanceConfig,
};
pub use security::{
    AuthTestConfig, AuthzTestConfig, PenetrationTestConfig, SecurityScanConfig, TestSecurityConfig,
    VulnerabilityTestConfig,
};

// ==================== SECTION ====================

/// **THE** unified test configuration for all testing scenarios
/// This replaces the large unified_test_config_consolidation.rs file
pub type UnifiedTestConfig = serde_json::Value; // Placeholder for removed StandardDomainConfig

/// **Test-specific configuration extensions**
/// All testing configuration consolidated into focused domains
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestExtensions {
    /// Test execution configuration
    pub execution: TestExecutionConfig,
    /// Performance and load testing configuration
    pub performance: TestPerformanceConfig,
    /// Security testing configuration
    pub security: TestSecurityConfig,
    /// Chaos engineering configuration
    pub chaos: TestChaosConfig,
    /// Mock services and test doubles configuration
    pub mocking: HashMap<String, String>, // Simplified for canonical modernization
    /// Integration testing configuration
    pub integration: TestIntegrationConfig,
    /// Test environment configuration
    pub environment: TestEnvironmentConfig,
}

/// **Configuration Builder Pattern**
/// Provides a fluent interface for building test configurations
pub struct TestConfigBuilder {
    extensions: TestExtensions,
}

impl TestConfigBuilder {
    pub fn new() -> Self {
        Self {
            extensions: TestExtensions::default(),
        }
    }

    pub fn with_execution(mut self, execution: TestExecutionConfig) -> Self {
        self.extensions.execution = execution;
        self
    }

    pub fn with_performance(mut self, performance: TestPerformanceConfig) -> Self {
        self.extensions.performance = performance;
        self
    }

    pub fn with_security(mut self, security: TestSecurityConfig) -> Self {
        self.extensions.security = security;
        self
    }

    pub fn with_chaos(mut self, chaos: TestChaosConfig) -> Self {
        self.extensions.chaos = chaos;
        self
    }

    pub fn with_mocking(mut self, mocking: HashMap<String, String>) -> Self {
        self.extensions.mocking = mocking;
        self
    }

    pub fn with_integration(mut self, integration: TestIntegrationConfig) -> Self {
        self.extensions.integration = integration;
        self
    }

    pub fn with_environment(mut self, environment: TestEnvironmentConfig) -> Self {
        self.extensions.environment = environment;
        self
    }

    pub fn build(self) -> TestExtensions {
        self.extensions
    }
}

impl Default for TestConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
