//! **PROTOCOL MODULE UNIT TESTS**
//!
//! Comprehensive unit tests for MCP protocol modules.
//! Evolved to match actual protocol type definitions.

use nestgate_mcp::protocol::{
    ErrorPayload, HealthCheckPayload, HealthCheckType, HealthStatus, Message, MessagePayload,
    MessageType, Response, ResponsePayload, ResponseStatus, ServiceInfo, ServiceStatus,
    StatusUpdatePayload,
};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// ============================================================================
// Message Tests
// ============================================================================

#[test]
fn test_message_creation_via_constructor() {
    let msg = Message::new(
        MessageType::HealthCheck,
        MessagePayload::HealthCheck(HealthCheckPayload {
            check_type: HealthCheckType::Shallow,
        }),
    );

    assert!(matches!(msg.message_type, MessageType::HealthCheck));
    assert!(!msg.id.is_empty());
    assert_eq!(msg.source, "nestgate-v2");
    assert!(msg.destination.is_none());
}

#[test]
fn test_message_with_destination() {
    let msg = Message::new(
        MessageType::HealthCheck,
        MessagePayload::StatusUpdate(StatusUpdatePayload {
            service_id: "test".to_string(),
            status: ServiceStatus::Online,
        }),
    )
    .with_destination("beardog-v1".to_string());

    assert_eq!(msg.destination, Some("beardog-v1".to_string()));
}

#[test]
fn test_message_with_metadata() {
    let msg = Message::new(
        MessageType::HealthCheck,
        MessagePayload::HealthCheck(HealthCheckPayload {
            check_type: HealthCheckType::Deep,
        }),
    )
    .with_metadata("priority".to_string(), "high".to_string());

    assert_eq!(msg.metadata.get("priority").unwrap(), "high");
}

#[test]
fn test_message_serialization_roundtrip() {
    let original = Message::new(
        MessageType::HealthCheck,
        MessagePayload::HealthCheck(HealthCheckPayload {
            check_type: HealthCheckType::Storage,
        }),
    );

    let serialized = serde_json::to_string(&original).unwrap();
    let deserialized: Message = serde_json::from_str(&serialized).unwrap();

    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.source, deserialized.source);
}

#[test]
fn test_message_type_variants() {
    // Verify all major message type variants exist and are distinct
    let types = vec![
        MessageType::CapabilityRegistration,
        MessageType::CapabilityQuery,
        MessageType::VolumeCreate,
        MessageType::VolumeDelete,
        MessageType::HealthCheck,
        MessageType::StatusUpdate,
        MessageType::FederationJoin,
        MessageType::OrchestratorRoute,
        MessageType::ServiceDiscovery,
        MessageType::Error,
        MessageType::Acknowledgment,
    ];

    assert_eq!(types.len(), 11);
}

// ============================================================================
// Response Tests
// ============================================================================

#[test]
fn test_response_success() {
    let response = Response::success("req-123".to_string(), ResponsePayload::Empty);

    assert!(matches!(response.status, ResponseStatus::Success));
    assert_eq!(response.request_id, "req-123");
    assert!(response.payload.is_some());
    assert!(response.error.is_none());
}

#[test]
fn test_response_error() {
    let error = ErrorPayload {
        error_code: "NOT_FOUND".to_string(),
        error_message: "Resource not found".to_string(),
        details: HashMap::new(),
        timestamp: SystemTime::now(),
    };

    let response = Response::error("req-456".to_string(), error);

    assert!(matches!(response.status, ResponseStatus::Error));
    assert!(response.error.is_some());
    assert!(response.payload.is_none());
}

#[test]
fn test_response_serialization_roundtrip() {
    let original = Response::success("roundtrip-789".to_string(), ResponsePayload::Empty);

    let json_str = serde_json::to_string(&original).unwrap();
    let deserialized: Response = serde_json::from_str(&json_str).unwrap();

    assert_eq!(original.request_id, deserialized.request_id);
}

#[test]
fn test_response_status_values() {
    assert!(matches!(ResponseStatus::Success, ResponseStatus::Success));
    assert!(matches!(ResponseStatus::Error, ResponseStatus::Error));
    assert!(matches!(ResponseStatus::Pending, ResponseStatus::Pending));
    assert!(matches!(ResponseStatus::Timeout, ResponseStatus::Timeout));
}

#[test]
fn test_response_metadata() {
    let mut response = Response::success("meta-test".to_string(), ResponsePayload::Empty);

    response
        .metadata
        .insert("timestamp".to_string(), "1234567890".to_string());

    assert!(response.metadata.contains_key("timestamp"));
}

// ============================================================================
// Service Tests
// ============================================================================

#[test]
fn test_service_info_creation() {
    let service = ServiceInfo {
        service_id: "test_service".to_string(),
        service_name: "Test Storage Service".to_string(),
        service_type: "storage".to_string(),
        endpoint: "/tmp/test.sock".to_string(),
        status: ServiceStatus::Online,
        timestamp: SystemTime::now(),
    };

    assert_eq!(service.service_id, "test_service");
    assert!(matches!(service.status, ServiceStatus::Online));
}

#[test]
fn test_service_status_transitions() {
    let mut service = ServiceInfo {
        service_id: "status_test".to_string(),
        service_name: "Status Test".to_string(),
        service_type: "compute".to_string(),
        endpoint: "/tmp/compute.sock".to_string(),
        status: ServiceStatus::Offline,
        timestamp: SystemTime::now(),
    };

    service.status = ServiceStatus::Online;
    assert!(matches!(service.status, ServiceStatus::Online));

    service.status = ServiceStatus::Degraded;
    assert!(matches!(service.status, ServiceStatus::Degraded));

    service.status = ServiceStatus::Maintenance;
    assert!(matches!(service.status, ServiceStatus::Maintenance));
}

#[test]
fn test_service_serialization() {
    let service = ServiceInfo {
        service_id: "ser_test".to_string(),
        service_name: "Serialization Test".to_string(),
        service_type: "test".to_string(),
        endpoint: "/test".to_string(),
        status: ServiceStatus::Online,
        timestamp: SystemTime::now(),
    };

    let json = serde_json::to_string(&service).unwrap();
    let deserialized: ServiceInfo = serde_json::from_str(&json).unwrap();

    assert_eq!(service.service_id, deserialized.service_id);
}

#[test]
fn test_service_status_all_variants() {
    let statuses = [
        ServiceStatus::Online,
        ServiceStatus::Offline,
        ServiceStatus::Degraded,
        ServiceStatus::Maintenance,
    ];

    assert_eq!(statuses.len(), 4);
}

// ============================================================================
// Health Check Tests
// ============================================================================

#[test]
fn test_health_check_payload_shallow() {
    let health = HealthCheckPayload {
        check_type: HealthCheckType::Shallow,
    };

    assert!(matches!(health.check_type, HealthCheckType::Shallow));
}

#[test]
fn test_health_check_payload_deep() {
    let health = HealthCheckPayload {
        check_type: HealthCheckType::Deep,
    };

    assert!(matches!(health.check_type, HealthCheckType::Deep));
}

#[test]
fn test_health_check_type_variants() {
    let types = [
        HealthCheckType::Shallow,
        HealthCheckType::Deep,
        HealthCheckType::Storage,
        HealthCheckType::Network,
    ];

    assert_eq!(types.len(), 4);
}

#[test]
fn test_health_status_creation() {
    let status = HealthStatus {
        status: ServiceStatus::Online,
        uptime: Duration::from_secs(3600),
        last_check: SystemTime::now(),
        details: HashMap::from([
            ("cpu_usage".to_string(), "12%".to_string()),
            ("memory_usage".to_string(), "45%".to_string()),
        ]),
    };

    assert!(matches!(status.status, ServiceStatus::Online));
    assert_eq!(status.uptime, Duration::from_secs(3600));
    assert_eq!(status.details.len(), 2);
}

#[test]
fn test_health_check_serialization() {
    let health = HealthCheckPayload {
        check_type: HealthCheckType::Network,
    };

    let json = serde_json::to_string(&health).unwrap();
    let deserialized: HealthCheckPayload = serde_json::from_str(&json).unwrap();

    assert!(matches!(deserialized.check_type, HealthCheckType::Network));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_error_payload_creation() {
    let error = ErrorPayload {
        error_code: "INTERNAL_ERROR".to_string(),
        error_message: "Internal error".to_string(),
        details: HashMap::new(),
        timestamp: SystemTime::now(),
    };

    assert_eq!(error.error_code, "INTERNAL_ERROR");
    assert_eq!(error.error_message, "Internal error");
}

#[test]
fn test_error_payload_with_details() {
    let error = ErrorPayload {
        error_code: "NOT_FOUND".to_string(),
        error_message: "Not found".to_string(),
        details: HashMap::from([("resource".to_string(), "user_123".to_string())]),
        timestamp: SystemTime::now(),
    };

    assert_eq!(error.details.get("resource").unwrap(), "user_123");
}

#[test]
fn test_error_serialization() {
    let error = ErrorPayload {
        error_code: "FORBIDDEN".to_string(),
        error_message: "Forbidden".to_string(),
        details: HashMap::from([("reason".to_string(), "insufficient_permissions".to_string())]),
        timestamp: SystemTime::now(),
    };

    let json = serde_json::to_string(&error).unwrap();
    let deserialized: ErrorPayload = serde_json::from_str(&json).unwrap();

    assert_eq!(error.error_code, deserialized.error_code);
    assert_eq!(error.error_message, deserialized.error_message);
}

#[test]
fn test_error_codes_convention() {
    let errors = [
        ErrorPayload {
            error_code: "BAD_REQUEST".to_string(),
            error_message: "Bad Request".to_string(),
            details: HashMap::new(),
            timestamp: SystemTime::now(),
        },
        ErrorPayload {
            error_code: "INTERNAL_ERROR".to_string(),
            error_message: "Internal Error".to_string(),
            details: HashMap::new(),
            timestamp: SystemTime::now(),
        },
        ErrorPayload {
            error_code: "SERVICE_UNAVAILABLE".to_string(),
            error_message: "Service Unavailable".to_string(),
            details: HashMap::new(),
            timestamp: SystemTime::now(),
        },
    ];

    assert_eq!(errors[0].error_code, "BAD_REQUEST");
    assert_eq!(errors[1].error_code, "INTERNAL_ERROR");
    assert_eq!(errors[2].error_code, "SERVICE_UNAVAILABLE");
}

// ============================================================================
// Edge Cases & Integration
// ============================================================================

#[test]
fn test_status_update_payload() {
    let payload = StatusUpdatePayload {
        service_id: "nestgate-core".to_string(),
        status: ServiceStatus::Online,
    };

    assert_eq!(payload.service_id, "nestgate-core");
    assert!(matches!(payload.status, ServiceStatus::Online));
}

#[test]
fn test_large_metadata_on_message() {
    let mut msg = Message::new(
        MessageType::HealthCheck,
        MessagePayload::HealthCheck(HealthCheckPayload {
            check_type: HealthCheckType::Shallow,
        }),
    );

    for i in 0..100 {
        msg.metadata
            .insert(format!("key_{i}"), format!("value_{i}"));
    }

    assert_eq!(msg.metadata.len(), 100);
}

#[test]
fn test_unicode_in_messages() {
    let msg = Message::new(
        MessageType::HealthCheck,
        MessagePayload::HealthCheck(HealthCheckPayload {
            check_type: HealthCheckType::Shallow,
        }),
    )
    .with_metadata("emoji".to_string(), "🚀".to_string())
    .with_metadata("chinese".to_string(), "你好".to_string())
    .with_metadata("arabic".to_string(), "مرحبا".to_string());

    assert_eq!(msg.metadata.get("emoji").unwrap(), "🚀");
    assert_eq!(msg.metadata.get("chinese").unwrap(), "你好");
}

#[test]
fn test_error_payload_as_message() {
    let error_payload = ErrorPayload {
        error_code: "TIMEOUT".to_string(),
        error_message: "Request timed out".to_string(),
        details: HashMap::new(),
        timestamp: SystemTime::now(),
    };

    let msg = Message::new(MessageType::Error, MessagePayload::Error(error_payload));

    assert!(matches!(msg.message_type, MessageType::Error));
    assert!(matches!(msg.payload, MessagePayload::Error(_)));
}

#[test]
fn test_response_payload_empty() {
    let response = Response::success("empty-test".to_string(), ResponsePayload::Empty);

    assert!(matches!(response.payload, Some(ResponsePayload::Empty)));
}
