/// Comprehensive Integration Testing Module

pub mod config;
pub mod tests;

// 🚀 ECOSYSTEM UNIFICATION: Use centralized unified test config system
// Migration from fragmented ComprehensiveTestConfig to unified system
pub use crate::common::config::{
    UnifiedTestConfig, UnifiedTestConfigBuilder, TestExecutionSettings as TestExecutionConfig,
    TestPerformanceSettings as TestPerformanceConfig, TestIntegrationSettings as TestIntegrationConfig
};

/// Create a comprehensive test configuration using the unified system
pub fn create_comprehensive_test_config() -> UnifiedTestConfig {
    UnifiedTestConfigBuilder::new()
        .test_name("comprehensive-integration-suite")
        .test_description("Comprehensive integration testing across all NestGate components")
        .max_duration(std::time::Duration::from_secs(300)) // 5 minutes
        .build()
} 