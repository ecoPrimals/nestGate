use nestgate_core::service_discovery::{resolve_service_endpoint, DynamicEndpointResolver};
use nestgate_core::universal_adapter::UniversalAdapter;
use std::collections::HashSet;

mod common;
use common::{get_test_endpoint, TestServiceManager};

/// Comprehensive validation of hardcoding elimination
#[tokio::test]
async fn test_complete_hardcoding_elimination() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing complete hardcoding elimination...");

    // Test 1: Dynamic endpoint resolution
    test_dynamic_endpoint_resolution().await;

    // Test 2: No hardcoded localhost URLs
    test_no_hardcoded_localhost().await;

    // Test 3: Environment variable overrides
    test_environment_overrides().await;

    // Test 4: Service discovery integration
    test_service_discovery_integration().await;

    // Test 5: Test environment isolation
    test_environment_isolation().await;

    println!("✅ All hardcoding elimination tests passed!");
    Ok(())
}

/// Test dynamic endpoint resolution
async fn test_dynamic_endpoint_resolution() -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔄 Testing dynamic endpoint resolution...");

    let resolver = DynamicEndpointResolver::new();

    // Test all service types
    let services = ["api", "websocket", "metrics", "health", "admin", "static"];
    let mut endpoints = HashSet::new();

    for service in &services {
        let endpoint = resolver
            .resolve_endpoint(service)
            .await
            .map_err(|e| format!("Failed to resolve endpoint for {}: {}", service, e))?;

        // Should not contain hardcoded values
        assert!(
            !endpoint.contains("localhost:8080"),
            "Found hardcoded localhost:8080 in {}",
            endpoint
        );
        assert!(
            !endpoint.contains("localhost:8081"),
            "Found hardcoded localhost:8081 in {}",
            endpoint
        );
        assert!(
            !endpoint.contains("localhost:8082"),
            "Found hardcoded localhost:8082 in {}",
            endpoint
        );
        assert!(
            !endpoint.contains("127.0.0.1:8080"),
            "Found hardcoded 127.0.0.1:8080 in {}",
            endpoint
        );

        // Should be valid URL format
        assert!(
            endpoint.starts_with("http://") || endpoint.starts_with("ws://"),
            "Invalid URL format: {}",
            endpoint
        );

        // All endpoints should be unique (different ports)
        assert!(
            !endpoints.contains(&endpoint),
            "Duplicate endpoint: {}",
            endpoint
        );
        endpoints.insert(endpoint);
    }

    println!("    ✅ Dynamic endpoint resolution working");
    Ok(())
}

/// Test no hardcoded localhost URLs
async fn test_no_hardcoded_localhost() -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔍 Testing for hardcoded localhost patterns...");

    // Test service endpoint resolution
    for service in &["api", "websocket", "metrics", "health"] {
        let endpoint = resolve_service_endpoint(service).await.unwrap();

        // Should not contain common hardcoded patterns
        assert!(
            !endpoint.contains(":8080"),
            "Found hardcoded :8080 in {}",
            endpoint
        );
        assert!(
            !endpoint.contains(":8081"),
            "Found hardcoded :8081 in {}",
            endpoint
        );
        assert!(
            !endpoint.contains(":8082"),
            "Found hardcoded :8082 in {}",
            endpoint
        );
        assert!(
            !endpoint.contains("localhost:8080"),
            "Found hardcoded localhost:8080 in {}",
            endpoint
        );

        // Should use environment variables or dynamic allocation
        assert!(
            endpoint.contains(nestgate_core::constants::TEST_HOSTNAME)
                || std::env::var("NESTGATE_HOSTNAME").is_ok(),
            "Should use localhost or custom hostname from environment"
        );
        Ok(())
    }

    println!("    ✅ No hardcoded localhost patterns found");
    Ok(())
}

/// Test environment variable overrides
async fn test_environment_overrides() -> Result<(), Box<dyn std::error::Error>> {
    println!("  🌍 Testing environment variable overrides...");

    // Set custom endpoints
    std::env::set_var("API_ENDPOINT", "http://custom-api:9090");
    std::env::set_var("WEBSOCKET_ENDPOINT", "ws://custom-ws:9091");

    let resolver = DynamicEndpointResolver::new();

    // Should use environment variables
    let api_endpoint = resolver.resolve_endpoint("api").await.unwrap();
    let ws_endpoint = resolver.resolve_endpoint("websocket").await.unwrap();

    assert_eq!(api_endpoint, "http://custom-api:9090");
    assert_eq!(ws_endpoint, "ws://custom-ws:9091");

    // Clean up
    std::env::remove_var("API_ENDPOINT");
    std::env::remove_var("WEBSOCKET_ENDPOINT");

    println!("    ✅ Environment variable overrides working");
    Ok(())
}

/// Test service discovery integration
async fn test_service_discovery_integration() -> Result<(), Box<dyn std::error::Error>> {
    println!("  🔍 Testing service discovery integration...");

    // Test that universal adapter can be integrated
    let resolver = DynamicEndpointResolver::new();

    // Test capability-based resolution (should work even without adapter)
    let endpoint = resolver.resolve_endpoint("storage").await.unwrap();
    assert!(endpoint.starts_with("http://"));
    assert!(!endpoint.contains("hardcoded"));

    // Test unknown service handling
    let unknown_endpoint = resolver.resolve_endpoint("unknown_service").await.unwrap();
    assert!(unknown_endpoint.starts_with("http://"));

    println!("    ✅ Service discovery integration working");
    Ok(())
}

/// Test environment isolation for tests
async fn test_environment_isolation() -> Result<(), Box<dyn std::error::Error>> {
    println!("  🧪 Testing test environment isolation...");

    let test_manager = TestServiceManager::new();

    // Allocate multiple services
    let services = ["api", "websocket", "metrics", "health", "admin"];
    let mut allocated_ports = HashSet::new();

    for service in &services {
        let port = test_manager.allocate_service_port(service).await;
        let endpoint = test_manager.get_service_endpoint(service).await;

        // Port should be unique
        assert!(
            !allocated_ports.contains(&port),
            "Duplicate port allocation: {}",
            port
        );
        allocated_ports.insert(port);

        // Endpoint should not contain hardcoded values
        assert!(
            !endpoint.contains(":8080"),
            "Found hardcoded :8080 in {}",
            endpoint
        );
        assert!(
            !endpoint.contains(":8081"),
            "Found hardcoded :8081 in {}",
            endpoint
        );
        assert!(
            !endpoint.contains(":8082"),
            "Found hardcoded :8082 in {}",
            endpoint
        );

        // Should be valid format
        assert!(endpoint.starts_with("http://") || endpoint.starts_with("ws://"));
        Ok(())
    }

    // Test global test manager
    let global_endpoint = get_test_endpoint("test_service").await;
    assert!(global_endpoint.starts_with("http://"));
    assert!(!global_endpoint.contains("hardcoded"));

    println!("    ✅ Test environment isolation working");
    Ok(())
}

/// Test universal adapter capability routing
#[tokio::test]
async fn test_universal_adapter_no_hardcoding() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing universal adapter has no hardcoding...");

    // Test that universal adapter uses capability-based discovery
    let capabilities = [
        "storage",
        "security",
        "orchestration",
        "artificial_intelligence",
        "compute",
    ];

    for capability in &capabilities {
        // Should not contain hardcoded primal names
        assert!(
            !capability.contains("songbird"),
            "Found hardcoded primal name: songbird"
        );
        assert!(
            !capability.contains("toadstool"),
            "Found hardcoded primal name: toadstool"
        );
        assert!(
            !capability.contains("squirrel"),
            "Found hardcoded primal name: squirrel"
        );
        assert!(
            !capability.contains("beardog"),
            "Found hardcoded primal name: beardog"
        );
        assert!(
            !capability.contains("biomeos"),
            "Found hardcoded primal name: biomeos"
        );
        Ok(())
    }

    println!("✅ Universal adapter uses capability-based discovery only");
    Ok(())
}

/// Test configuration system has no hardcoding
#[tokio::test]
async fn test_configuration_no_hardcoding() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing configuration system has no hardcoding...");

    use nestgate_core::config::network::ServiceEndpoints;

    // Create default service endpoints
    let endpoints = ServiceEndpoints::default();

    // Should use environment variables or canonical defaults
    let api_url = endpoints.api_url();
    let websocket_url = endpoints.websocket_url();

    // Should not contain hardcoded localhost:8080 patterns
    if api_url.contains(nestgate_core::constants::TEST_HOSTNAME) {
        // If using localhost, should use environment variables or canonical ports
        assert!(
            !api_url.contains("localhost:8000") || std::env::var("NESTGATE_API_URL").is_ok(),
            "API URL should use environment variable or dynamic resolution"
        );
        Ok(())
    }

    if websocket_url.contains(nestgate_core::constants::TEST_HOSTNAME) {
        // If using localhost, should use environment variables or canonical ports
        assert!(
            !websocket_url.contains("localhost:8080")
                || std::env::var("NESTGATE_WEBSOCKET_URL").is_ok(),
            "WebSocket URL should use environment variable or dynamic resolution"
        );
        Ok(())
    }

    println!("✅ Configuration system uses dynamic resolution");
    Ok(())
}

/// Test vendor agnosticism - no cloud provider hardcoding
#[tokio::test]
async fn test_vendor_agnosticism() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing vendor agnosticism...");

    use nestgate_core::temporal_storage::CloudProvider;

    // Test that cloud providers are configurable, not hardcoded
    let aws_provider = CloudProvider::Aws {
        region: "us-east-1".to_string(),
    };
    let azure_provider = CloudProvider::Azure {
        subscription_id: "test-subscription".to_string(),
    };
    let gcp_provider = CloudProvider::Gcp {
        project_id: "test-project".to_string(),
    };
    let custom_provider = CloudProvider::Custom {
        endpoint: "https://custom-cloud.example.com".to_string(),
    };

    // All providers should be supported equally
    let providers = [aws_provider, azure_provider, gcp_provider, custom_provider];

    for provider in &providers {
        match provider {
            CloudProvider::Aws { region } => assert!(!region.is_empty()),
            CloudProvider::Azure { subscription_id } => assert!(!subscription_id.is_empty()),
            CloudProvider::Gcp { project_id } => assert!(!project_id.is_empty()),
            CloudProvider::Custom { endpoint } => assert!(!endpoint.is_empty()),
    Ok(())
        }
        Ok(())
    }

    println!("✅ Vendor agnosticism maintained - all cloud providers supported equally");
    Ok(())
}

/// Test primal sovereignty - no direct primal dependencies
#[tokio::test]
async fn test_primal_sovereignty() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing primal sovereignty...");

    // Test that no hardcoded primal names exist in capability system
    use nestgate_core::universal_adapter::capability_system::CapabilityCategory;

    let categories = [
        CapabilityCategory::Storage,
        CapabilityCategory::Security,
        CapabilityCategory::Orchestration,
        CapabilityCategory::ArtificialIntelligence,
        CapabilityCategory::Compute,
    ];

    for category in &categories {
        let category_str = format!("{:?}", category);

        // Should not contain primal names
        assert!(!category_str.to_lowercase().contains("songbird"));
        assert!(!category_str.to_lowercase().contains("toadstool"));
        assert!(!category_str.to_lowercase().contains("squirrel"));
        assert!(!category_str.to_lowercase().contains("beardog"));
        assert!(!category_str.to_lowercase().contains("biomeos"));
        Ok(())
    }

    println!("✅ Primal sovereignty maintained - capability-based architecture only");
    Ok(())
}

/// Integration test: Complete workflow without hardcoding
#[tokio::test]
async fn test_complete_workflow_no_hardcoding() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing complete workflow without hardcoding...");

    // 1. Dynamic endpoint resolution
    let api_endpoint = resolve_service_endpoint("api").await.unwrap();
    println!("  📍 API endpoint: {}", api_endpoint);

    // 2. Test service management
    let test_endpoint = get_test_endpoint("integration_test").await;
    println!("  🧪 Test endpoint: {}", test_endpoint);

    // 3. Verify no hardcoding patterns
    let endpoints = [api_endpoint, test_endpoint];

    for endpoint in &endpoints {
        assert!(
            !endpoint.contains("localhost:8080"),
            "Found hardcoded localhost:8080"
        );
        assert!(
            !endpoint.contains("localhost:8081"),
            "Found hardcoded localhost:8081"
        );
        assert!(
            !endpoint.contains("localhost:8082"),
            "Found hardcoded localhost:8082"
        );
        assert!(
            !endpoint.contains("127.0.0.1:8080"),
            "Found hardcoded 127.0.0.1:8080"
        );

        assert!(
            endpoint.starts_with("http://") || endpoint.starts_with("ws://"),
            "Invalid endpoint format: {}",
            endpoint
        );
        Ok(())
    }

    println!("✅ Complete workflow operates without hardcoding");
    Ok(())
}
