use nestgate_port_manager::{PortManager, PortManagerConfig, Result, ServiceType};

#[tokio::test]
async fn test_port_manager_lifecycle() -> Result<()> {
    // Create a test configuration
    let mut config = PortManagerConfig::default();
    config.server.port = 9999; // Use a different port for testing
    
    // Create port manager
    let mut port_manager = PortManager::new(config);
    
    // Initialize the port manager
    port_manager.initialize().await?;
    
    // Allocate a port 
    let port = port_manager.allocate_port(
        "test-service", 
        ServiceType::API,
        Some(4500)
    ).await?;
    
    // Verify port was allocated
    assert!(port >= 4000 && port <= 4999, "Port should be in the API range");
    
    // Stop port manager
    port_manager.stop().await?;
    
    Ok(())
} 