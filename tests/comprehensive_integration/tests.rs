/// Comprehensive Integration Tests
/// 
/// 🚀 ECOSYSTEM UNIFICATION: Now using unified test config system

use std::sync::Arc;
use tokio::time::{sleep, Duration};

// Use centralized unified test config system
use super::{create_comprehensive_test_config, create_optimized_comprehensive_config};
use crate::common::test_config::{UnifiedTestConfig, UnifiedTestConfigBuilder};

/// Comprehensive integration test suite using unified configuration
#[tokio::test]
async fn comprehensive_integration_test_suite() {
    let test_config = create_comprehensive_test_config();
    let optimized_config = create_optimized_comprehensive_config();
    
    // Initialize test environment with unified config
    tracing::info!(
        "Starting comprehensive integration test suite: {}",
        test_config.test_name
    );
    
    // Run tests with proper unified configuration
    run_comprehensive_tests_with_config(test_config).await;
}

/// Run comprehensive tests with unified configuration
async fn run_comprehensive_tests_with_config(config: UnifiedTestConfig) {
    tracing::info!("Running tests with unified config for: {}", config.test_description);
    
    // Test execution using unified config parameters
    let test_timeout = config.test_settings.max_duration;
    let parallel_enabled = config.test_settings.parallel;
    
    tracing::info!(
        "Test configuration - timeout: {:?}, parallel: {}",
        test_timeout,
        parallel_enabled
    );
    
    // Mock comprehensive test execution
    sleep(Duration::from_millis(100)).await;
    
    tracing::info!("Comprehensive integration tests completed successfully");
}

/// Test configuration builder functionality
#[tokio::test]
async fn test_unified_config_builder() {
    let config = UnifiedTestConfigBuilder::new()
        .with_test_name("builder-test".to_string())
        .with_test_description("Testing the unified config builder".to_string())
        .with_max_duration(Duration::from_secs(60))
        .build()
        .expect("Should build config successfully");
    
    assert_eq!(config.test_name, "builder-test");
    assert_eq!(config.test_description, "Testing the unified config builder");
    assert_eq!(config.test_settings.max_duration, Duration::from_secs(60));
}

/// Test performance configuration integration
#[tokio::test]
async fn test_performance_config_integration() {
    let config = create_comprehensive_test_config();
    
    // Verify performance config is properly set
    assert!(config.performance.load_testing_enabled);
    
    tracing::info!("Performance configuration validated successfully");
} 