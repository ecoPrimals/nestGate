use nestgate_core::*;
use tokio;

/// Integration tests for the NestGate core functionality
/// These tests verify that the major components work together correctly

#[tokio::test]
async fn test_basic_configuration_loading() -> Result<(), Box<dyn std::error::Error>> {
    // Test that we can load and validate basic configuration
    let config = config::canonical_master::NestGateNestGateCanonicalConfig::default();

    // Verify essential configuration fields exist
    assert!(config.network.port > 0);
    assert!(!config.network.host.is_empty());
    assert!(config.storage.enabled);

    println!("✅ Configuration loading test passed");
    Ok(())
}

#[tokio::test]
async fn test_error_handling_patterns() -> Result<(), Box<dyn std::error::Error>> {
    // Test that our error handling patterns work correctly
    use error::NestGateError;

    // Test internal error creation
    let internal_err = NestGateError::internal_error("Test error", "integration_test");
    assert!(format!("{}", internal_err).contains("Test error"));

    // Test configuration error creation
    let config_err = NestGateError::config_error("test_field", "Invalid value");
    assert!(format!("{}", config_err).contains("test_field"));

    println!("✅ Error handling patterns test passed");
    Ok(())
}

#[tokio::test]
async fn test_service_discovery_basic() -> Result<(), Box<dyn std::error::Error>> {
    // Test basic service discovery functionality
    let registry = service_discovery::InMemoryServiceRegistry::new();

    // Test service registration
    let service_info = service_discovery::ServiceInfo {
        name: "test-service".to_string(),
        version: "1.0.0".to_string(),
        endpoints: vec![std::env::var("NESTGATE_TEST_ENDPOINT")
            .unwrap_or_else(|_| nestgate_core::constants::TEST_API_BASE.to_string())],
        health_check_url: Some(format!(
            "{}/health",
            std::env::var("NESTGATE_TEST_ENDPOINT")
                .unwrap_or_else(|_| nestgate_core::constants::TEST_API_BASE.to_string())
        )),
        metadata: std::collections::HashMap::new(),
    };

    registry.register_service(service_info.clone()).await?;

    // Test service discovery
    let discovered = registry.discover_service("test-service").await?;
    assert_eq!(discovered.name, "test-service");
    assert_eq!(discovered.version, "1.0.0");

    println!("✅ Service discovery test passed");
    Ok(())
}

#[tokio::test]
async fn test_zero_cost_abstractions() -> Result<(), Box<dyn std::error::Error>> {
    // Test that zero-cost abstractions compile and work
    use zero_cost::*;

    // Test zero-cost configuration
    let config = ZeroCostConfig::production();
    assert!(config.is_optimized());

    // Test compile-time optimization flags
    assert!(ZeroCostConfig::ENABLE_SIMD);
    assert!(ZeroCostConfig::ENABLE_ZERO_COPY);

    println!("✅ Zero-cost abstractions test passed");
    Ok(())
}

#[tokio::test]
async fn test_canonical_types() -> Result<(), Box<dyn std::error::Error>> {
    // Test canonical type system
    use canonical_types::*;

    // Test unified service state
    let state = UnifiedServiceState::Running;
    assert_eq!(format!("{:?}", state), "Running");

    // Test unified health status
    let health = UnifiedHealthStatus::Healthy;
    assert!(health.is_healthy());

    println!("✅ Canonical types test passed");
    Ok(())
}

#[tokio::test]
async fn test_memory_safety() -> Result<(), Box<dyn std::error::Error>> {
    // Test memory safety patterns
    use safe_operations::*;

    // Test safe string operations
    let safe_string = safe_string_operation("test input", "memory_safety_test")?;
    assert_eq!(safe_string, "test input");

    // Test safe numeric operations
    let result = safe_numeric_operation(42, 8, "memory_safety_test")?;
    assert_eq!(result, 50);

    println!("✅ Memory safety test passed");
    Ok(())
}

#[tokio::test]
async fn test_performance_metrics() -> Result<(), Box<dyn std::error::Error>> {
    // Test performance monitoring
    use observability::*;

    // Test metrics collection
    let metrics = PerformanceMetrics::new();
    metrics.record_operation_time("test_operation", std::time::Duration::from_millis(100));

    let stats = metrics.get_stats();
    assert!(stats.total_operations > 0);

    println!("✅ Performance metrics test passed");
    Ok(())
}

#[tokio::test]
async fn test_ecosystem_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test ecosystem integration framework
    use ecosystem_integration::*;

    // Test universal adapter pattern
    let adapter = UniversalAdapter::new_mock();
    let capabilities = adapter.get_capabilities().await?;

    assert!(!capabilities.supported_protocols.is_empty());

    println!("✅ Ecosystem integration test passed");
    Ok(())
}

#[tokio::test]
async fn test_security_framework() -> Result<(), Box<dyn std::error::Error>> {
    // Test security provider
    use security::*;

    // Test authentication
    let provider = SecurityProvider::new_test();
    let token = provider.generate_test_token("test_user")?;

    assert!(provider.validate_token(&token)?);

    println!("✅ Security framework test passed");
    Ok(())
}

#[tokio::test]
async fn test_full_system_integration() -> Result<(), Box<dyn std::error::Error>> {
    // Test full system integration
    println!("🚀 Starting full system integration test...");

    // Initialize configuration
    let config = config::canonical_master::NestGateNestGateCanonicalConfig::default();

    // Initialize core services
    let service_registry = service_discovery::InMemoryServiceRegistry::new();
    let security_provider = security::SecurityProvider::new_test();
    let metrics = observability::PerformanceMetrics::new();

    // Test service registration and discovery
    let test_service = service_discovery::ServiceInfo {
        name: "integration-test-service".to_string(),
        version: "1.0.0".to_string(),
        endpoints: vec![std::env::var("NESTGATE_TEST_ENDPOINT_ALT")
            .unwrap_or_else(|_| nestgate_core::constants::TEST_API_ALT.to_string())],
        health_check_url: Some(format!(
            "{}/health",
            std::env::var("NESTGATE_TEST_ENDPOINT_ALT")
                .unwrap_or_else(|_| nestgate_core::constants::TEST_API_ALT.to_string())
        )),
        metadata: std::collections::HashMap::new(),
    };

    service_registry
        .register_service(test_service.clone())
        .await?;
    let discovered = service_registry
        .discover_service("integration-test-service")
        .await?;
    assert_eq!(discovered.name, "integration-test-service");

    // Test security integration
    let auth_token = security_provider.generate_test_token("integration_test_user")?;
    assert!(security_provider.validate_token(&auth_token)?);

    // Test metrics collection
    let start = std::time::Instant::now();
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    metrics.record_operation_time("full_integration_test", start.elapsed());

    let stats = metrics.get_stats();
    assert!(stats.total_operations > 0);

    println!("✅ Full system integration test passed");
    println!("📊 Test Statistics:");
    println!("   - Services registered: 1");
    println!("   - Authentication tokens validated: 1");
    println!("   - Operations measured: {}", stats.total_operations);

    Ok(())
}
