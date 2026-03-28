// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! API module tests for nestgate-network
//!
//! Tests for service registration, port allocation, and health status reporting.

use chrono::Utc;
use nestgate_network::api::{OrchestrationCapability, ServiceInstance, ServiceStatus};

// Test port constant
const TEST_PORT: u16 = 18080;

#[test]
fn test_service_status_equality() {
    assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
    assert_eq!(ServiceStatus::Stopped, ServiceStatus::Stopped);
    assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
}

#[test]
fn test_service_instance_creation() {
    let instance = ServiceInstance {
        id: "test-001".to_string(),
        name: "test-service".to_string(),
        host: "localhost".to_string(),
        port: TEST_PORT,
        status: ServiceStatus::Running,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert_eq!(instance.id, "test-001");
    assert_eq!(instance.name, "test-service");
    assert_eq!(instance.port, TEST_PORT);
    assert_eq!(instance.status, ServiceStatus::Running);
}

#[test]
fn test_service_instance_serialization() {
    let instance = ServiceInstance {
        id: "test-002".to_string(),
        name: "api-service".to_string(),
        host: "127.0.0.1".to_string(),
        port: 9090,
        status: ServiceStatus::Starting,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test that instance can be serialized
    let json = serde_json::to_string(&instance);
    assert!(json.is_ok());

    // Test that JSON contains expected fields
    let json_str = json.expect("Network operation failed");
    assert!(json_str.contains("test-002"));
    assert!(json_str.contains("api-service"));
    assert!(json_str.contains("9090"));
}

#[test]
fn test_orchestration_capability_creation() {
    let capability = OrchestrationCapability::new("http://localhost:8080".to_string());
    assert_eq!(capability.base_url, "http://localhost:8080");
}

#[test]
fn test_service_status_states() {
    // Test all status variants exist and are distinct
    let statuses = vec![
        ServiceStatus::Starting,
        ServiceStatus::Running,
        ServiceStatus::Stopping,
        ServiceStatus::Stopped,
        ServiceStatus::Failed,
    ];

    assert_eq!(statuses.len(), 5);

    // Verify each status can be cloned
    for status in &statuses {
        let cloned = status.clone();
        assert_eq!(status, &cloned);
    }
}

#[test]
fn test_service_instance_status_transitions() {
    let mut instance = ServiceInstance {
        id: "test-003".to_string(),
        name: "test-service".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        status: ServiceStatus::Starting,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Test status transitions
    assert_eq!(instance.status, ServiceStatus::Starting);

    instance.status = ServiceStatus::Running;
    assert_eq!(instance.status, ServiceStatus::Running);

    instance.status = ServiceStatus::Stopping;
    assert_eq!(instance.status, ServiceStatus::Stopping);

    instance.status = ServiceStatus::Stopped;
    assert_eq!(instance.status, ServiceStatus::Stopped);
}

#[test]
fn test_service_instance_port_ranges() {
    // Test various port numbers
    let ports = vec![80, 443, 8080, 3000, 9090, 65535];

    for port in ports {
        let instance = ServiceInstance {
            id: format!("test-{}", port),
            name: "port-test".to_string(),
            host: "localhost".to_string(),
            port,
            status: ServiceStatus::Running,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(instance.port, port);
        assert!(instance.port > 0);
        // Note: No need to check <= 65535 since port is a u16
    }
}

#[test]
fn test_service_instance_deserialization() {
    let json = r#"{
        "id": "test-004",
        "name": "deserialize-test",
        "host": "192.168.1.1",
        "port": 5000,
        "status": "Running",
        "created_at": "2025-10-16T00:00:00Z",
        "updated_at": "2025-10-16T00:00:00Z"
    }"#;

    let result: Result<ServiceInstance, _> = serde_json::from_str(json);
    assert!(result.is_ok());

    let instance = result.expect("Network operation failed");
    assert_eq!(instance.id, "test-004");
    assert_eq!(instance.name, "deserialize-test");
    assert_eq!(instance.host, "192.168.1.1");
    assert_eq!(instance.port, 5000);
    assert_eq!(instance.status, ServiceStatus::Running);
}

#[test]
fn test_service_instance_clone() {
    let original = ServiceInstance {
        id: "test-005".to_string(),
        name: "clone-test".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        status: ServiceStatus::Running,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let cloned = original.clone();

    assert_eq!(original.id, cloned.id);
    assert_eq!(original.name, cloned.name);
    assert_eq!(original.host, cloned.host);
    assert_eq!(original.port, cloned.port);
    assert_eq!(original.status, cloned.status);
}

#[test]
fn test_service_instance_debug_formatting() {
    let instance = ServiceInstance {
        id: "test-006".to_string(),
        name: "debug-test".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        status: ServiceStatus::Running,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let debug_str = format!("{:?}", instance);
    assert!(debug_str.contains("test-006"));
    assert!(debug_str.contains("debug-test"));
    assert!(debug_str.contains("8080"));
}

#[test]
fn test_multiple_service_instances() {
    let instances: Vec<ServiceInstance> = (0..10)
        .map(|i| ServiceInstance {
            id: format!("service-{}", i),
            name: format!("test-service-{}", i),
            host: "localhost".to_string(),
            port: 8080 + i as u16,
            status: ServiceStatus::Running,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .collect();

    assert_eq!(instances.len(), 10);

    // Verify each instance has unique ID and port
    for (i, instance) in instances.iter().enumerate() {
        assert_eq!(instance.id, format!("service-{}", i));
        assert_eq!(instance.port, 8080 + i as u16);
    }
}
