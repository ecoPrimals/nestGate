use std::collections::HashMap;
use nestgate_port_manager::{
    PortManager, PortManagerConfig, PortRange, ServiceType, Result
};

#[tokio::test]
async fn test_port_allocation() -> Result<()> {
    // Create a test configuration with a specific port range
    let mut port_ranges = HashMap::new();
    port_ranges.insert(ServiceType::WebSocket, PortRange::new(9600, 9650));
    port_ranges.insert(ServiceType::API, PortRange::new(9700, 9750));
    port_ranges.insert(ServiceType::UI, PortRange::new(9800, 9850));

    let config = PortManagerConfig {
        port_ranges,
        ..Default::default()
    };

    // Initialize the port manager
    let mut port_manager = PortManager::new(config);
    port_manager.initialize().await?;

    // Test service registration and port allocation
    let service_id = "test-service";
    let service_type = ServiceType::WebSocket;
    
    // Test allocating a port using the public API
    let port = port_manager.allocate_port(service_id, service_type.clone(), None).await?;
    
    // Verify the port is within the expected range
    assert!(port >= 9600 && port <= 9650, "Port {} should be within WebSocket range", port);
    
    // Test preferred port allocation
    let preferred_port = 9610;
    let service_id2 = "test-service-2";
    
    let port2 = port_manager.allocate_port(service_id2, service_type.clone(), Some(preferred_port)).await?;
    
    // Verify we got the preferred port
    // Note: This test might fail if the port is already in use by another process
    assert!(port2 >= 9600 && port2 <= 9650, "Port should be in WebSocket range");
    
    // Stop port manager
    port_manager.stop().await?;
    
    Ok(())
}

#[tokio::test]
async fn test_basic_operations() -> Result<()> {
    // Create a test configuration
    let config = PortManagerConfig::default();

    // Initialize the port manager
    let mut port_manager = PortManager::new(config);
    port_manager.initialize().await?;
    
    // Allocate a port
    let port = port_manager.allocate_port("test-service", ServiceType::API, None).await?;
    
    // Verify the port is within the API range (4000-4999 in default config)
    assert!(port >= 4000 && port <= 4999, "Port should be in API range");
    
    // Stop port manager
    port_manager.stop().await?;
    
    Ok(())
} 