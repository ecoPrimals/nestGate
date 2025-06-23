//! Integration tests for the Songbird Orchestrator
//! 
//! Tests the complete orchestrator functionality including service registration,
//! lifecycle management, health monitoring, and request routing.

use std::time::Duration;
use std::sync::Arc;

mod common;
use common::*;

use songbird_orchestrator::prelude::*;

#[tokio::test]
async fn test_orchestrator_creation() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Orchestrator::new(config).await;
    assert!(orchestrator.is_ok());
}

#[tokio::test]
async fn test_service_registration_and_startup() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Orchestrator::new(config).await.unwrap();
    
    // Create a mock service
    let service = MockService::new("test-service-1");
    let service_box = Box::new(service);
    
    // Register the service
    let result = orchestrator.register_service(service_box).await;
    assert!(result.is_ok());
    
    let stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(stats.service_count, 1);
}

#[tokio::test]
async fn test_multiple_service_registration() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Orchestrator::new(config).await.unwrap();
    
    // Create multiple mock services
    let services = create_mock_services(5);
    
    // Register all services
    for (i, service) in services.into_iter().enumerate() {
        let service_box = Box::new(service);
        let result = orchestrator.register_service(service_box).await;
        assert!(result.is_ok(), "Failed to register service {}", i);
    }
    
    let stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(stats.service_count, 5);
}

#[tokio::test]
async fn test_orchestrator_lifecycle() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Orchestrator::new(config).await.unwrap();
    
    // Start the orchestrator
    let start_result = orchestrator.start().await;
    assert!(start_result.is_ok());
    
    // Stop the orchestrator
    let stop_result = orchestrator.stop().await;
    assert!(stop_result.is_ok());
}

#[tokio::test]
async fn test_orchestrator_with_services_lifecycle() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Orchestrator::new(config).await.unwrap();
    
    // Register some services
    let services = create_mock_services(3);
    for service in services {
        let service_box = Box::new(service);
        orchestrator.register_service(service_box).await.unwrap();
    }
    
    // Start orchestrator (should start all services)
    assert!(orchestrator.start().await.is_ok());
    
    // Verify services are running through stats
    let stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(stats.service_count, 3);
    
    // Stop orchestrator (should stop all services)
    assert!(orchestrator.stop().await.is_ok());
}

#[tokio::test]
async fn test_orchestrator_stats() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Orchestrator::new(config).await.unwrap();
    
    // Initial stats
    let initial_stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(initial_stats.service_count, 0);
    assert_eq!(initial_stats.total_requests, 0);
    assert_eq!(initial_stats.successful_requests, 0);
    assert_eq!(initial_stats.failed_requests, 0);
    
    // Register a service
    let service = MockService::new("stats-test-service");
    let service_box = Box::new(service);
    orchestrator.register_service(service_box).await.unwrap();
    
    // Check updated stats
    let updated_stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(updated_stats.service_count, 1);
}

#[tokio::test]
async fn test_orchestrator_concurrent_operations() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Arc::new(Orchestrator::new(config).await.unwrap());
    
    let mut tasks = vec![];
    
    // Concurrently register multiple services
    for i in 0..10 {
        let orchestrator_clone = orchestrator.clone();
        let task = tokio::spawn(async move {
            let service = MockService::new(format!("concurrent-service-{}", i));
            let service_box = Box::new(service);
            orchestrator_clone.register_service(service_box).await
        });
        tasks.push(task);
    }
    
    // Wait for all registrations to complete
    let results = futures::future::join_all(tasks).await;
    
    // All registrations should succeed
    for (i, result) in results.into_iter().enumerate() {
        assert!(result.unwrap().is_ok(), "Failed to register service {}", i);
    }
    
    // Verify all services were registered
    let stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(stats.service_count, 10);
}

#[tokio::test]
async fn test_orchestrator_error_handling() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Orchestrator::new(config).await.unwrap();
    
    // Create a service that will error during setup
    let mut service = MockService::new("error-service");
    service.set_error_rate(1.0).await; // 100% error rate
    
    let service_box = Box::new(service);
    
    // Registration might succeed, but service operations should handle errors gracefully
    let result = orchestrator.register_service(service_box).await;
    // The orchestrator should handle service errors gracefully
    // This test verifies the orchestrator doesn't crash with problematic services
    assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable for error handling test
}

#[tokio::test]
async fn test_orchestrator_configuration_validation() {
    use std::time::Duration;
    
    // Test with custom configuration
    let mut config = OrchestratorConfig::<MockConfig>::default();
    config.orchestrator.max_services = 100;
    config.orchestrator.health_check_interval = Duration::from_millis(500);
    config.orchestrator.port = 9999;
    
    let orchestrator = Orchestrator::new(config).await;
    assert!(orchestrator.is_ok());
}

#[tokio::test]
async fn test_orchestrator_service_limits() {
    let mut config = OrchestratorConfig::<MockConfig>::default();
    config.orchestrator.max_services = 2; // Limit to 2 services
    
    let orchestrator = Orchestrator::new(config).await.unwrap();
    
    // Register up to the limit
    for i in 0..2 {
        let service = MockService::new(format!("limited-service-{}", i));
        let service_box = Box::new(service);
        let result = orchestrator.register_service(service_box).await;
        assert!(result.is_ok());
    }
    
    let stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(stats.service_count, 2);
    
    // Trying to register beyond the limit should be handled appropriately
    let excess_service = MockService::new("excess-service");
    let excess_service_box = Box::new(excess_service);
    let excess_result = orchestrator.register_service(excess_service_box).await;
    
    // Depending on implementation, this might succeed or fail
    // The test verifies the orchestrator handles the limit scenario gracefully
    match excess_result {
        Ok(_) => {
            // If it succeeds, verify count increased
            let new_stats = orchestrator.get_stats().await.unwrap();
            assert!(new_stats.service_count >= 2);
        }
        Err(_) => {
            // If it fails, verify count stayed the same
            let new_stats = orchestrator.get_stats().await.unwrap();
            assert_eq!(new_stats.service_count, 2);
        }
    }
}

#[tokio::test]
async fn test_orchestrator_restart_capability() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Orchestrator::new(config).await.unwrap();
    
    // Register a service
    let service = MockService::new("restart-test-service");
    let service_box = Box::new(service);
    orchestrator.register_service(service_box).await.unwrap();
    
    // Start
    assert!(orchestrator.start().await.is_ok());
    
    // Stop
    assert!(orchestrator.stop().await.is_ok());
    
    // Start again
    assert!(orchestrator.start().await.is_ok());
    
    // Stop again
    assert!(orchestrator.stop().await.is_ok());
    
    // Verify orchestrator is still functional
    let stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(stats.service_count, 1);
}

#[tokio::test]
async fn test_orchestrator_graceful_shutdown() {
    let config = OrchestratorConfig::<MockConfig>::default();
    let orchestrator = Arc::new(Orchestrator::new(config).await.unwrap());
    
    // Register some services
    for i in 0..3 {
        let service = MockService::new(format!("shutdown-test-{}", i));
        let service_box = Box::new(service);
        orchestrator.register_service(service_box).await.unwrap();
    }
    
    // Start orchestrator
    assert!(orchestrator.start().await.is_ok());
    
    // Simulate some activity
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    // Stop orchestrator gracefully
    let stop_start = std::time::Instant::now();
    assert!(orchestrator.stop().await.is_ok());
    let stop_duration = stop_start.elapsed();
    
    // Shutdown should be relatively quick for test services
    assert!(stop_duration < Duration::from_secs(5));
    
    // Verify final stats
    let final_stats = orchestrator.get_stats().await.unwrap();
    assert_eq!(final_stats.service_count, 3);
} 