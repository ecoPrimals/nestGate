/// Test using the new clean test infrastructure
/// Demonstrates the rebuilt test modules work properly
mod common;

use crate::common::MockUniversalService;
use nestgate_core::Result;
use std::time::Duration;
use tests::canonical_modernization::UnifiedServiceType;
use tests::common::test_doubles::UniversalService;
use tests::common::{
    CleanTestConfig, CompleteTestConfig, MockServiceRegistry, MockStorageService,
    SimpleTestService, TestHelpers, TestSetup, TestUtils,
};
use tokio;

#[tokio::test]
async fn test_clean_infrastructure_basic_functionality() -> Result<()> {
    println!("🧪 Testing clean test infrastructure");

    // Test config creation
    let config = CleanTestConfig::default();
    println!("✅ Created clean test config: {}", config.name);

    // Test utilities
    let unique_name = TestUtils::unique_test_name("test_infra");
    println!("✅ Generated unique name: {}", unique_name);

    // Create unified config for testing
    let unified_config = TestUtils::simple_unified_config();
    println!(
        "✅ Created unified config with network port: {}",
        unified_config.network.port
    );

    println!("🎉 Clean infrastructure basic test passed!");
    Ok(())
}

#[tokio::test]
async fn test_mock_services_functionality() -> Result<()> {
    println!("🧪 Testing mock services");

    // Test mock universal service
    let mut storage_service = MockUniversalService::storage("test_storage");
    let response = storage_service.handle_request("test_request").await?;
    println!("✅ Storage service response: {}", response);

    let call_count = storage_service.get_call_count();
    assert_eq!(call_count, 1, "Call count should be 1");
    println!("✅ Call count tracking works: {}", call_count);

    // Test service disabling
    storage_service.set_enabled(false);
    let result = storage_service.handle_request("disabled_test").await;
    assert!(result.is_err(), "Disabled service should return error");
    println!("✅ Service disabling works correctly");

    // Test mock storage service
    let mut storage = MockStorageService::new("test_store".to_string());
    storage
        .store("key1".to_string(), b"value1".to_vec())
        .await?;

    let retrieved = storage.retrieve("key1").await?;
    assert!(retrieved.is_some(), "Should retrieve stored value");
    assert_eq!(retrieved?, b"value1".to_vec());
    println!("✅ Storage operations work correctly");

    let ops_count = storage.get_operation_count();
    assert_eq!(ops_count, 2, "Should have 2 operations (store + retrieve)");
    println!("✅ Operation counting works: {} operations", ops_count);

    println!("🎉 Mock services test passed!");
    Ok(())
}

#[tokio::test]
async fn test_service_registry() -> Result<()> {
    println!("🧪 Testing service registry");

    let mut registry = MockServiceRegistry::new();

    // Register multiple services
    let storage_service = MockUniversalService::storage("primary_storage");
    let network_service = MockUniversalService::network("primary_network");
    let compute_service = MockUniversalService::compute("primary_compute");

    registry.register_universal_service(storage_service).await;
    registry.register_universal_service(network_service).await;
    registry.register_universal_service(compute_service).await;

    let service_count = registry.get_service_count().await;
    assert_eq!(service_count, 3, "Should have 3 registered services");
    println!("✅ Registered {} services", service_count);

    // Test service retrieval
    let retrieved_storage = registry.get_universal_service("primary_storage").await;
    assert!(
        retrieved_storage.is_some(),
        "Should retrieve registered service"
    );

    let service = retrieved_storage?;
    assert_eq!(service.service_type, UnifiedServiceType::Storage);
    assert_eq!(service.name, "primary_storage");
    println!("✅ Service retrieval works correctly");

    // Test service listing
    let service_names = registry.list_service_names().await;
    assert_eq!(service_names.len(), 3, "Should list all service names");
    println!("✅ Service listing works: {:?}", service_names);

    println!("🎉 Service registry test passed!");
    Ok(())
}

#[tokio::test]
async fn test_helper_functions() -> Result<()> {
    println!("🧪 Testing helper functions");

    // Test assertion helpers
    TestHelpers::assert_test(true, "This should pass")?;
    println!("✅ Assertion helper works");

    TestHelpers::assert_eq!(42, 42, "Numbers should be equal")?;
    println!("✅ Equality assertion works");

    // Test failing assertion (should return error)
    let result = TestHelpers::assert_test(false, "This should fail");
    assert!(result.is_err(), "Should return error for failed assertion");
    println!("✅ Failed assertion handling works");

    // Test test data creation
    let test_data = TestHelpers::create_test_data(5);
    assert_eq!(test_data.len(), 5, "Should create 5 test items");
    println!("✅ Test data creation works: {} items", test_data.len());

    // Test work simulation
    let work_result = TestHelpers::simulate_work(Duration::from_millis(1)).await?;
    assert_eq!(work_result, "work_completed");
    println!("✅ Work simulation works: {}", work_result);

    println!("🎉 Helper functions test passed!");
    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> Result<()> {
    println!("🧪 Testing concurrent operations with clean infrastructure");

    let registry = MockServiceRegistry::new();

    // Create multiple services
    for i in 0..5 {
        let service = MockUniversalService::storage(&format!("storage_{}", i));
        registry.register_universal_service(service).await;
        Ok(())
    }

    // Test concurrent service requests
    let results = TestHelpers::run_concurrent_tests(
        "concurrent_service",
        move |task_id| {
            let registry_clone = registry.clone();
            async move {
                let service_name = format!("storage_{}", task_id % 5);
                if let Some(service) = registry_clone.get_universal_service(&service_name).await {
                    let request = format!("request_from_task_{}", task_id);
                    service.handle_request(&request).await?;
                    Ok(())
                }
                Ok(())
            }
        },
        10, // 10 concurrent tasks
        Duration::from_secs(5),
    )
    .await;

    let successful_tests = results.iter().filter(|r| r.success).count();
    println!(
        "✅ Concurrent test results: {}/{} successful",
        successful_tests,
        results.len()
    );

    assert!(
        successful_tests >= 8,
        "At least 80% of concurrent tests should succeed"
    );

    println!("🎉 Concurrent operations test passed!");
    Ok(())
}

#[tokio::test]
async fn test_configuration_variants() -> Result<()> {
    println!("🧪 Testing configuration variants");

    // Test different config types
    let perf_config = CleanTestConfig::performance_test();
    assert!(perf_config.performance.enable_metrics);
    assert!(perf_config.performance.expected_ops_per_sec.is_some());
    println!("✅ Performance config created");

    // Test chaos configuration
    let chaos_config = CleanTestConfig::chaos_test();
    assert!(chaos_config.chaos.enabled);
    assert!(chaos_config.basic.enable_chaos);
    println!("✅ Chaos config created");

    // Test integration configuration
    let integration_config = CleanTestConfig::integration_test();
    assert!(!integration_config.environment.isolated);
    assert!(!integration_config.performance.enable_metrics);
    println!("✅ Integration config created");

    println!("🎉 Configuration variants test passed!");
    Ok(())
}

#[tokio::test]
async fn test_environment_setup_and_cleanup() -> Result<()> {
    println!("🧪 Testing environment setup and cleanup");

    let config = CleanTestConfig {
        name: "cleanup_test".to_string(),
        timeout: Duration::from_secs(10),
        max_concurrent: 5,
        enable_chaos: false,
    };

    // Test environment setup
    let mut env = TestSetup::initialize(&config).await?;
    println!("✅ Environment initialized: {}", env.name);

    // Add test resources
    println!("🔧 Setting up test resources...");
    env.add_resource("resource_1".to_string());
    env.add_resource("resource_2".to_string());
    assert_eq!(env.resources.len(), 2);
    println!("✅ Added test resources");

    // Test environment operations
    println!("⚡ Testing environment operations...");

    // Check uptime - give it a moment to ensure measurable uptime
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    let uptime = env.uptime();
    assert!(uptime.as_micros() > 0, "Uptime should be greater than 0");
    println!("✅ Environment uptime: {:?}", uptime);

    // Test cleanup
    TestSetup::cleanup(env).await?;
    println!("✅ Environment cleanup completed");

    println!("🎉 Environment setup/cleanup test passed!");
    Ok(())
}
