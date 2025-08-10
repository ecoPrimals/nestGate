/// Configuration for Comprehensive Integration Tests
/// 
/// 🚀 ECOSYSTEM UNIFICATION: This module now uses the centralized unified test config system
/// to eliminate test config fragmentation.

// Use the centralized unified test config system instead of fragmented structs
pub use crate::common::test_config::{
    UnifiedTestConfig, TestExecutionSettings as TestExecutionConfig, TestRetryConfig, 
    TestEnvironmentSettings as TestEnvironmentConfig, TestPerformanceSettings as TestPerformanceConfig, 
    TestChaosSettings as TestChaosConfig, TestIntegrationSettings as TestIntegrationConfig, 
    UnifiedTestConfigBuilder, TestIsolationLevel
};

// Re-export the main comprehensive test config builder
pub use super::create_comprehensive_test_config;

/// Create optimized comprehensive integration test configuration
pub fn create_optimized_comprehensive_config() -> UnifiedTestConfig {
    UnifiedTestConfig::builder()
        .test_name("comprehensive-integration-optimized")
        .test_description("Optimized comprehensive integration testing configuration")
        .max_duration(std::time::Duration::from_secs(300))
        .isolation_level(TestIsolationLevel::Container)
        .build()
} 