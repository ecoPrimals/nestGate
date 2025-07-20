//! Integration tests for the Network module
//!
//! These tests verify the behavior of the NetworkApi and related components
//! in integration scenarios.

use nestgate_network::api::NetworkApi;
use nestgate_network::ServiceStatus;

#[tokio::test]
async fn test_network_api_basic_functionality() {
    // Basic test to verify the module can be instantiated
    let _api = NetworkApi::new();
    // More comprehensive tests would be added here
}

#[test]
fn test_service_status_enum() {
    // Test ServiceStatus enum variants from types.rs
    let _running = ServiceStatus::Running;
    let _healthy = ServiceStatus::Healthy;
    let _unhealthy = ServiceStatus::Unhealthy;
    let _unknown = ServiceStatus::Unknown;
    let _starting = ServiceStatus::Starting;
    let _stopping = ServiceStatus::Stopping;
    let _failed = ServiceStatus::Failed;
    // This ensures the enum is properly defined and has the expected variants
}
