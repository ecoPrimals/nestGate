/// Comprehensive Suite Module - Canonical Modernization
/// Uses unified test configuration system instead of duplicate structures
pub mod config;
pub mod tests;

// **CANONICAL MODERNIZATION**: Use unified test configuration
pub use crate::common::config::{TestPerformanceSettings, UnifiedTestConfig};

/// Comprehensive suite configuration builder
pub fn create_comprehensive_suite_config() -> UnifiedTestConfig {
    let mut config = crate::common::test_config::UnifiedTestConfigBuilder::performance_test(
        "comprehensive_suite",
    );
    config.extensions.performance.enable_metrics = true;
    config.extensions.performance.enable_stress_testing = true;
    config
}
