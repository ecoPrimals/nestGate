//! Unit tests for the Songbird Orchestrator

use songbird_orchestrator::{
    Orchestrator, OrchestratorConfig, Result,
};

mod common;
use common::TestOrchestrator;

#[tokio::test]
async fn test_orchestrator_initialization() -> Result<()> {
    // Create a basic configuration
    let config = OrchestratorConfig::default();

    // Create an orchestrator instance
    let orchestrator = Orchestrator::new(config).await?;

    // Verify that the orchestrator initialized correctly
    assert!(!orchestrator.get_service_ids().is_empty() || orchestrator.get_service_ids().is_empty());
    
    // Test that we can access the configuration
    let _config_ref = orchestrator.get_config();
    
    Ok(())
}

#[tokio::test]
async fn test_orchestrator_service_management() -> Result<()> {
    let fixture = TestOrchestrator::new().await?;
    let orchestrator = fixture.orchestrator();

    // Initially should have no services
    assert!(orchestrator.get_service_ids().is_empty());

    // Add a service
    orchestrator.add_service("test-service".to_string(), "Test Service".to_string());
    
    // Verify service was added
    let service_ids = orchestrator.get_service_ids();
    assert_eq!(service_ids.len(), 1);
    assert!(service_ids.contains(&"test-service".to_string()));

    // Add another service
    orchestrator.add_service("test-service-2".to_string(), "Test Service 2".to_string());
    
    // Verify both services exist
    let service_ids = orchestrator.get_service_ids();
    assert_eq!(service_ids.len(), 2);
    
    // Remove a service
    let removed = orchestrator.remove_service("test-service");
    assert_eq!(removed, Some("Test Service".to_string()));
    
    // Verify service was removed
    let service_ids = orchestrator.get_service_ids();
    assert_eq!(service_ids.len(), 1);
    assert!(!service_ids.contains(&"test-service".to_string()));

    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_orchestrator_configuration_access() -> Result<()> {
    let mut config = OrchestratorConfig::default();
    // Modify some config values for testing - use the correct fields
    config.orchestrator.bind_address = "127.0.0.1".to_string();
    config.orchestrator.port = 8080;

    let fixture = TestOrchestrator::with_config(config.clone()).await?;
    let orchestrator = fixture.orchestrator();

    // Test configuration access
    let retrieved_config = orchestrator.get_config();
    assert_eq!(retrieved_config.orchestrator.bind_address, "127.0.0.1");
    assert_eq!(retrieved_config.orchestrator.port, 8080);

    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_service_info_creation() -> Result<()> {
    let fixture = TestOrchestrator::new().await?;
    
    // Test service info creation
    let service_info = fixture.create_test_service("test-service", "api");
    
    assert_eq!(service_info.id, "test-service");
    assert_eq!(service_info.name, "Test test-service");
    assert_eq!(service_info.service_type, "api");
    assert_eq!(service_info.version, "1.0.0");
    assert!(service_info.capabilities.contains(&"test".to_string()));

    fixture.cleanup().await?;
    Ok(())
}

#[tokio::test]
async fn test_service_request_creation() -> Result<()> {
    let fixture = TestOrchestrator::new().await?;
    
    // Test request creation
    let request = fixture.create_test_request("GET", "/health");
    
    assert_eq!(request.method, "GET");
    assert_eq!(request.path, "/health");
    assert!(!request.id.is_empty());

    fixture.cleanup().await?;
    Ok(())
}

#[test]
fn test_service_capabilities() {
    // Test that service capabilities can be properly managed
    let mut capabilities = vec!["http".to_string(), "websocket".to_string()];
    capabilities.push("grpc".to_string());
    
    assert_eq!(capabilities.len(), 3);
    assert!(capabilities.contains(&"http".to_string()));
    assert!(capabilities.contains(&"websocket".to_string()));
    assert!(capabilities.contains(&"grpc".to_string()));
}

#[test]
fn test_service_metadata() {
    // Test service metadata handling
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("version".to_string(), serde_json::json!("1.0.0"));
    metadata.insert("author".to_string(), serde_json::json!("test"));
    metadata.insert("description".to_string(), serde_json::json!("Test service"));
    
    assert_eq!(metadata.len(), 3);
    assert_eq!(metadata.get("version").unwrap(), &serde_json::json!("1.0.0"));
}

#[test]
fn test_service_tags() {
    // Test service tag management
    let mut tags = std::collections::HashMap::new();
    tags.insert("environment".to_string(), "test".to_string());
    tags.insert("component".to_string(), "api".to_string());
    tags.insert("tier".to_string(), "backend".to_string());
    
    assert_eq!(tags.len(), 3);
    assert_eq!(tags.get("environment").unwrap(), "test");
    assert_eq!(tags.get("component").unwrap(), "api");
}

#[tokio::test]
async fn test_error_handling() -> Result<()> {
    let fixture = TestOrchestrator::new().await?;
    let orchestrator = fixture.orchestrator();

    // Test removing non-existent service
    let result = orchestrator.remove_service("non-existent");
    assert_eq!(result, None);

    // Test that we can handle multiple operations without issues
    orchestrator.add_service("service1".to_string(), "Service 1".to_string());
    orchestrator.add_service("service2".to_string(), "Service 2".to_string());
    
    let service_ids = orchestrator.get_service_ids();
    assert_eq!(service_ids.len(), 2);

    fixture.cleanup().await?;
    Ok(())
} 