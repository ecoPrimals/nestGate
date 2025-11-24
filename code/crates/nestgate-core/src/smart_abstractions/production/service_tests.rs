//! Comprehensive tests for ProductionSmartService
//! 
//! Tests cover all critical paths, error handling, and business logic.

use super::service::ProductionSmartService;
use super::config::ProductionServiceConfig;
use crate::smart_abstractions::{
    ServiceMetadata, SmartService, UniversalServiceRequest, UnifiedServiceState,
};
use crate::canonical::HealthStatus;

/// Helper to create test service metadata
fn create_test_metadata() -> ServiceMetadata {
    ServiceMetadata {
        service_id: "test-service-001".to_string(),
        service_type: "test_service".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec!["health_check".to_string(), "metrics".to_string()],
    }
}

/// Helper to create test request
fn create_test_request(operation: &str) -> UniversalServiceRequest {
    UniversalServiceRequest {
        request_id: format!("req-{}", uuid::Uuid::new_v4()),
        operation: operation.to_string(),
        data: None,
        metadata: std::collections::HashMap::new(),
    }
}

// ==================== SERVICE CREATION TESTS ====================

#[test]
fn test_production_service_creation() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata.clone());
    
    // Service should be created successfully
    assert_eq!(service.get_metadata().service_id, "test-service-001");
    assert_eq!(service.get_metadata().service_type, "test_service");
    assert_eq!(service.get_metadata().version, "1.0.0");
}

#[test]
fn test_production_service_with_custom_config() {
    let metadata = create_test_metadata();
    let config = ProductionServiceConfig {
        health_check_interval_secs: 60,
        max_concurrent_requests: 500,
        request_timeout_secs: 45,
        enable_detailed_logging: false,
    };
    
    let service = ProductionSmartService::with_config(metadata, config);
    assert_eq!(service.get_metadata().service_id, "test-service-001");
}

#[test]
fn test_service_metadata_preservation() {
    let metadata = ServiceMetadata {
        service_id: "svc-123".to_string(),
        service_type: "data_processor".to_string(),
        version: "2.5.1".to_string(),
        capabilities: vec!["transform".to_string(), "validate".to_string()],
    };
    
    let service = ProductionSmartService::new(metadata.clone());
    let retrieved = service.get_metadata();
    
    assert_eq!(retrieved.service_id, metadata.service_id);
    assert_eq!(retrieved.service_type, metadata.service_type);
    assert_eq!(retrieved.version, metadata.version);
    assert_eq!(retrieved.capabilities, metadata.capabilities);
}

// ==================== LIFECYCLE TESTS ====================

#[tokio::test]
async fn test_service_initialization() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    let result = service.initialize().await;
    assert!(result.is_ok(), "Service initialization should succeed");
}

#[tokio::test]
async fn test_service_start() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    // Initialize first
    service.initialize().await.expect("Init should succeed");
    
    // Then start
    let result = service.start().await;
    assert!(result.is_ok(), "Service start should succeed");
    
    // Verify state changed
    let state = service.get_state().await;
    assert!(matches!(state, UnifiedServiceState::Running), "Service should be running");
}

#[tokio::test]
async fn test_service_stop() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    // Initialize and start
    service.initialize().await.expect("Init should succeed");
    service.start().await.expect("Start should succeed");
    
    // Then stop
    let result = service.stop().await;
    assert!(result.is_ok(), "Service stop should succeed");
    
    // Verify state changed
    let state = service.get_state().await;
    assert!(matches!(state, UnifiedServiceState::Stopped), "Service should be stopped");
}

#[tokio::test]
async fn test_service_restart() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    // Initialize and start
    service.initialize().await.expect("Init should succeed");
    service.start().await.expect("Start should succeed");
    
    // Stop
    service.stop().await.expect("Stop should succeed");
    
    // Restart
    let result = service.start().await;
    assert!(result.is_ok(), "Service restart should succeed");
}

#[tokio::test]
async fn test_service_full_lifecycle() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    // Complete lifecycle
    assert!(service.initialize().await.is_ok());
    assert!(service.start().await.is_ok());
    assert!(matches!(service.get_state().await, UnifiedServiceState::Running));
    assert!(service.stop().await.is_ok());
    assert!(matches!(service.get_state().await, UnifiedServiceState::Stopped));
}

// ==================== HEALTH CHECK TESTS ====================

#[tokio::test]
async fn test_health_check_request() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init should succeed");
    service.start().await.expect("Start should succeed");
    
    let request = create_test_request("health_check");
    let response = service.process_request(request).await;
    
    assert!(response.is_ok(), "Health check should succeed");
    let resp = response.unwrap();
    assert!(matches!(resp.status, HealthStatus::Healthy | HealthStatus::Degraded));
    assert!(resp.error.is_none(), "Health check should not have errors");
}

#[tokio::test]
async fn test_health_check_returns_status() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let request = create_test_request("health_check");
    let response = service.process_request(request).await.expect("Should process");
    
    assert!(response.data.is_some(), "Health check should return data");
    let data = response.data.unwrap();
    assert!(data.get("health_status").is_some());
}

// ==================== METRICS TESTS ====================

#[tokio::test]
async fn test_metrics_request() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let request = create_test_request("get_metrics");
    let response = service.process_request(request).await;
    
    assert!(response.is_ok(), "Metrics request should succeed");
    let resp = response.unwrap();
    assert!(resp.data.is_some(), "Metrics should return data");
}

#[tokio::test]
async fn test_metrics_tracking() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    // Make several requests to increment metrics
    for _ in 0..5 {
        let request = create_test_request("health_check");
        service.process_request(request).await.expect("Request should succeed");
    }
    
    // Get metrics
    let request = create_test_request("get_metrics");
    let response = service.process_request(request).await.expect("Should get metrics");
    
    let data = response.data.unwrap();
    let total_requests = data.get("total_requests").and_then(|v| v.as_u64());
    assert!(total_requests.is_some(), "Should have total_requests metric");
    assert!(total_requests.unwrap() > 0, "Total requests should be > 0");
}

// ==================== STATUS TESTS ====================

#[tokio::test]
async fn test_status_request() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let request = create_test_request("get_status");
    let response = service.process_request(request).await;
    
    assert!(response.is_ok(), "Status request should succeed");
    let resp = response.unwrap();
    assert!(resp.data.is_some(), "Status should return data");
}

#[tokio::test]
async fn test_status_contains_uptime() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    // Wait a moment for uptime to accumulate
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let request = create_test_request("get_status");
    let response = service.process_request(request).await.expect("Should get status");
    
    let data = response.data.unwrap();
    assert!(data.get("uptime_seconds").is_some(), "Should include uptime");
    assert!(data.get("service_state").is_some(), "Should include state");
}

// ==================== DATA PROCESSING TESTS ====================

#[tokio::test]
async fn test_data_processing_request() {
    let metadata = ServiceMetadata {
        service_id: "proc-001".to_string(),
        service_type: "data_transformer".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec!["transform".to_string()],
    };
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let mut request = create_test_request("process_data");
    request.data = Some(serde_json::json!({"input": "test_data"}));
    
    let response = service.process_request(request).await;
    assert!(response.is_ok(), "Data processing should succeed");
}

#[tokio::test]
async fn test_data_processing_without_data() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let request = create_test_request("process_data");
    let response = service.process_request(request).await.expect("Should process");
    
    // Should return degraded status when no data provided
    assert!(matches!(response.status, HealthStatus::Degraded));
    assert!(response.error.is_some(), "Should have error message");
}

#[tokio::test]
async fn test_data_processing_various_service_types() {
    let service_types = vec!["data_transformer", "data_validator", "data_aggregator"];
    
    for service_type in service_types {
        let metadata = ServiceMetadata {
            service_id: format!("{}-001", service_type),
            service_type: service_type.to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["process".to_string()],
        };
        let service = ProductionSmartService::new(metadata);
        
        service.initialize().await.expect("Init");
        service.start().await.expect("Start");
        
        let mut request = create_test_request("process_data");
        request.data = Some(serde_json::json!({"test": "data"}));
        
        let response = service.process_request(request).await;
        assert!(response.is_ok(), "Processing should succeed for {}", service_type);
    }
}

// ==================== CAPABILITIES TESTS ====================

#[tokio::test]
async fn test_capabilities_request() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let request = create_test_request("get_capabilities");
    let response = service.process_request(request).await;
    
    assert!(response.is_ok(), "Capabilities request should succeed");
    let resp = response.unwrap();
    assert!(resp.data.is_some(), "Should return capabilities data");
}

#[tokio::test]
async fn test_capabilities_list() {
    let metadata = ServiceMetadata {
        service_id: "cap-test".to_string(),
        service_type: "multi_capable".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![
            "health_check".to_string(),
            "metrics".to_string(),
            "transform".to_string(),
            "validate".to_string(),
        ],
    };
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let request = create_test_request("get_capabilities");
    let response = service.process_request(request).await.expect("Should get capabilities");
    
    let data = response.data.unwrap();
    let caps = data.get("capabilities").and_then(|v| v.as_array());
    assert!(caps.is_some(), "Should have capabilities array");
    assert!(caps.unwrap().len() == 4, "Should have 4 capabilities");
}

// ==================== ERROR HANDLING TESTS ====================

#[tokio::test]
async fn test_unknown_operation() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let request = create_test_request("unknown_operation");
    let response = service.process_request(request).await.expect("Should handle unknown");
    
    assert!(matches!(response.status, HealthStatus::Degraded | HealthStatus::Unhealthy));
    assert!(response.error.is_some(), "Should have error for unknown operation");
}

#[tokio::test]
async fn test_multiple_operations_in_sequence() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let operations = vec!["health_check", "get_metrics", "get_status", "get_capabilities"];
    
    for op in operations {
        let request = create_test_request(op);
        let response = service.process_request(request).await;
        assert!(response.is_ok(), "Operation {} should succeed", op);
    }
}

// ==================== CONCURRENT REQUEST TESTS ====================

#[tokio::test]
async fn test_concurrent_health_checks() {
    use std::sync::Arc;
    
    let metadata = create_test_metadata();
    let service = Arc::new(ProductionSmartService::new(metadata));
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let mut handles = vec![];
    
    for i in 0..10 {
        let service_clone = Arc::clone(&service);
        let handle = tokio::spawn(async move {
            let request = create_test_request("health_check");
            service_clone.process_request(request).await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert!(result.is_ok(), "Concurrent health check should succeed");
    }
}

#[tokio::test]
async fn test_concurrent_mixed_operations() {
    use std::sync::Arc;
    
    let metadata = create_test_metadata();
    let service = Arc::new(ProductionSmartService::new(metadata));
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let operations = vec!["health_check", "get_metrics", "get_status"];
    let mut handles = vec![];
    
    for op in operations {
        let service_clone = Arc::clone(&service);
        let operation = op.to_string();
        let handle = tokio::spawn(async move {
            let request = create_test_request(&operation);
            service_clone.process_request(request).await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert!(result.is_ok(), "Concurrent operation should succeed");
    }
}

// ==================== STATE MANAGEMENT TESTS ====================

#[tokio::test]
async fn test_initial_state_is_initializing() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    let state = service.get_state().await;
    assert!(matches!(state, UnifiedServiceState::Initializing), "Initial state should be Initializing");
}

#[tokio::test]
async fn test_state_transitions() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    // Initial state
    assert!(matches!(service.get_state().await, UnifiedServiceState::Initializing));
    
    // After initialization
    service.initialize().await.expect("Init");
    // State might be Ready or still Initializing depending on impl
    
    // After start
    service.start().await.expect("Start");
    assert!(matches!(service.get_state().await, UnifiedServiceState::Running));
    
    // After stop
    service.stop().await.expect("Stop");
    assert!(matches!(service.get_state().await, UnifiedServiceState::Stopped));
}

// ==================== REQUEST ID PRESERVATION TESTS ====================

#[tokio::test]
async fn test_request_id_preserved_in_response() {
    let metadata = create_test_metadata();
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init");
    service.start().await.expect("Start");
    
    let request_id = "test-req-12345".to_string();
    let mut request = create_test_request("health_check");
    request.request_id = request_id.clone();
    
    let response = service.process_request(request).await.expect("Should process");
    assert_eq!(response.request_id, request_id, "Request ID should be preserved");
}

// ==================== EDGE CASE TESTS ====================

#[tokio::test]
async fn test_empty_capabilities() {
    let metadata = ServiceMetadata {
        service_id: "empty-cap".to_string(),
        service_type: "minimal".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![],
    };
    let service = ProductionSmartService::new(metadata);
    
    service.initialize().await.expect("Init should succeed even with no capabilities");
}

#[tokio::test]
async fn test_service_with_long_version_string() {
    let metadata = ServiceMetadata {
        service_id: "ver-test".to_string(),
        service_type: "test".to_string(),
        version: "1.2.3-alpha.1+build.20251114.git.abc123def".to_string(),
        capabilities: vec![],
    };
    let service = ProductionSmartService::new(metadata.clone());
    
    assert_eq!(service.get_metadata().version, metadata.version);
}

#[tokio::test]
async fn test_service_with_special_characters_in_id() {
    let metadata = ServiceMetadata {
        service_id: "test-service_001.prod@namespace".to_string(),
        service_type: "test".to_string(),
        version: "1.0.0".to_string(),
        capabilities: vec![],
    };
    let service = ProductionSmartService::new(metadata.clone());
    
    assert_eq!(service.get_metadata().service_id, metadata.service_id);
}

