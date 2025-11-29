//
// These tests verify the behavior of the NetworkApi and related components
// in integration scenarios.

//! Integration Tests module

use nestgate_network::api::ServiceStatus;
use nestgate_network::NetworkService;

#[tokio::test]
async fn test_network_service_basic_functionality() -> Result<(), Box<dyn std::error::Error>> {
    // Basic test to verify the module can be instantiated
    let config = nestgate_network::default_network_config();
    let _service = NetworkService::new(config);
    // More comprehensive tests would be added here
    Ok(())
}

#[test]
fn test_service_status_enum() -> Result<(), Box<dyn std::error::Error>> {
    // Test ServiceStatus enum variants from api.rs
    let _starting = ServiceStatus::Starting;
    let _running = ServiceStatus::Running;
    let _stopping = ServiceStatus::Stopping;
    let _stopped = ServiceStatus::Stopped;
    let _failed = ServiceStatus::Failed;

    // Test equality
    assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
    assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
    Ok(())
}
