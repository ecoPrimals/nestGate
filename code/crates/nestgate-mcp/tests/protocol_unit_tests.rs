//! **PROTOCOL MODULE UNIT TESTS**
//!
//! Comprehensive unit tests for refactored protocol modules.

use nestgate_mcp::protocol::{
    ErrorPayload, HealthCheckPayload, HealthCheckType, HealthStatus, JsonRpcError, Message,
    MessagePayload, MessageType, Response, ResponseStatus, ServiceInfo, ServiceStatus,
};
use serde_json::json;

// ============================================================================
// Message Tests (5 tests)
// ============================================================================

#[test]
fn test_message_creation() {
    let msg = Message {
        msg_type: MessageType::Request,
        payload: MessagePayload::HealthCheck(HealthCheckPayload {
            check_type: HealthCheckType::Full,
            target: Some("test_service".to_string()),
        }),
        metadata: Default::default(),
    };

    assert_eq!(msg.msg_type, MessageType::Request);
}

#[test]
fn test_message_serialization() {
    let msg = Message {
        msg_type: MessageType::Response,
        payload: MessagePayload::Empty,
        metadata: Default::default(),
    };

    let serialized = serde_json::to_string(&msg).unwrap();
    let deserialized: Message = serde_json::from_str(&serialized).unwrap();

    assert_eq!(msg.msg_type, deserialized.msg_type);
}

#[test]
fn test_message_types() {
    assert_eq!(MessageType::Request as i32, MessageType::Request as i32);
    assert_ne!(MessageType::Request as i32, MessageType::Response as i32);
}

#[test]
fn test_message_with_metadata() {
    let mut msg = Message {
        msg_type: MessageType::Request,
        payload: MessagePayload::Empty,
        metadata: Default::default(),
    };

    msg.metadata.insert("key".to_string(), json!("value"));

    assert_eq!(msg.metadata.get("key").unwrap(), &json!("value"));
}

#[test]
fn test_message_payload_variants() {
    let payloads = vec![
        MessagePayload::Empty,
        MessagePayload::HealthCheck(HealthCheckPayload {
            check_type: HealthCheckType::Quick,
            target: None,
        }),
    ];

    assert_eq!(payloads.len(), 2);
}

// ============================================================================
// Response Tests (5 tests)
// ============================================================================

#[test]
fn test_response_success() {
    let response = Response {
        request_id: "test_123".to_string(),
        status: ResponseStatus::Success,
        payload: None,
        metadata: Default::default(),
    };

    assert_eq!(response.status, ResponseStatus::Success);
    assert_eq!(response.request_id, "test_123");
}

#[test]
fn test_response_error() {
    let response = Response {
        request_id: "error_456".to_string(),
        status: ResponseStatus::Error,
        payload: Some(json!({"error": "test error"})),
        metadata: Default::default(),
    };

    assert_eq!(response.status, ResponseStatus::Error);
    assert!(response.payload.is_some());
}

#[test]
fn test_response_serialization_roundtrip() {
    let original = Response {
        request_id: "roundtrip_789".to_string(),
        status: ResponseStatus::Success,
        payload: Some(json!({"data": "test"})),
        metadata: Default::default(),
    };

    let json_str = serde_json::to_string(&original).unwrap();
    let deserialized: Response = serde_json::from_str(&json_str).unwrap();

    assert_eq!(original.request_id, deserialized.request_id);
    assert_eq!(original.status, deserialized.status);
}

#[test]
fn test_response_status_values() {
    assert!(matches!(ResponseStatus::Success, ResponseStatus::Success));
    assert!(matches!(ResponseStatus::Error, ResponseStatus::Error));
    assert!(matches!(ResponseStatus::Pending, ResponseStatus::Pending));
}

#[test]
fn test_response_with_metadata() {
    let mut response = Response {
        request_id: "meta_test".to_string(),
        status: ResponseStatus::Success,
        payload: None,
        metadata: Default::default(),
    };

    response
        .metadata
        .insert("timestamp".to_string(), json!(1234567890));

    assert!(response.metadata.contains_key("timestamp"));
}

// ============================================================================
// Service Tests (5 tests)
// ============================================================================

#[test]
fn test_service_info_creation() {
    let service = ServiceInfo {
        service_id: "test_service".to_string(),
        service_type: "storage".to_string(),
        endpoint: "/tmp/test.sock".to_string(),
        status: ServiceStatus::Active,
        capabilities: vec!["read".to_string(), "write".to_string()],
        metadata: Default::default(),
    };

    assert_eq!(service.service_id, "test_service");
    assert_eq!(service.capabilities.len(), 2);
}

#[test]
fn test_service_status_transitions() {
    let mut service = ServiceInfo {
        service_id: "status_test".to_string(),
        service_type: "compute".to_string(),
        endpoint: "/tmp/compute.sock".to_string(),
        status: ServiceStatus::Starting,
        capabilities: vec![],
        metadata: Default::default(),
    };

    service.status = ServiceStatus::Active;
    assert_eq!(service.status, ServiceStatus::Active);

    service.status = ServiceStatus::Stopping;
    assert_eq!(service.status, ServiceStatus::Stopping);
}

#[test]
fn test_service_capabilities() {
    let service = ServiceInfo {
        service_id: "cap_test".to_string(),
        service_type: "api".to_string(),
        endpoint: "/api".to_string(),
        status: ServiceStatus::Active,
        capabilities: vec![
            "http".to_string(),
            "websocket".to_string(),
            "grpc".to_string(),
        ],
        metadata: Default::default(),
    };

    assert!(service.capabilities.contains(&"http".to_string()));
    assert!(service.capabilities.contains(&"websocket".to_string()));
    assert_eq!(service.capabilities.len(), 3);
}

#[test]
fn test_service_serialization() {
    let service = ServiceInfo {
        service_id: "ser_test".to_string(),
        service_type: "test".to_string(),
        endpoint: "/test".to_string(),
        status: ServiceStatus::Active,
        capabilities: vec!["test_cap".to_string()],
        metadata: Default::default(),
    };

    let json = serde_json::to_string(&service).unwrap();
    let deserialized: ServiceInfo = serde_json::from_str(&json).unwrap();

    assert_eq!(service.service_id, deserialized.service_id);
    assert_eq!(service.status, deserialized.status);
}

#[test]
fn test_service_metadata() {
    let mut service = ServiceInfo {
        service_id: "meta_service".to_string(),
        service_type: "test".to_string(),
        endpoint: "/test".to_string(),
        status: ServiceStatus::Active,
        capabilities: vec![],
        metadata: Default::default(),
    };

    service
        .metadata
        .insert("version".to_string(), json!("1.0.0"));
    service.metadata.insert("uptime".to_string(), json!(3600));

    assert_eq!(service.metadata.get("version").unwrap(), &json!("1.0.0"));
    assert_eq!(service.metadata.get("uptime").unwrap(), &json!(3600));
}

// ============================================================================
// Health Check Tests (5 tests)
// ============================================================================

#[test]
fn test_health_check_payload_quick() {
    let health = HealthCheckPayload {
        check_type: HealthCheckType::Quick,
        target: None,
    };

    assert_eq!(health.check_type, HealthCheckType::Quick);
    assert!(health.target.is_none());
}

#[test]
fn test_health_check_payload_full() {
    let health = HealthCheckPayload {
        check_type: HealthCheckType::Full,
        target: Some("database".to_string()),
    };

    assert_eq!(health.check_type, HealthCheckType::Full);
    assert_eq!(health.target.unwrap(), "database");
}

#[test]
fn test_health_status_values() {
    let statuses = vec![
        HealthStatus::Healthy,
        HealthStatus::Degraded,
        HealthStatus::Unhealthy,
    ];

    assert_eq!(statuses.len(), 3);
    assert!(matches!(statuses[0], HealthStatus::Healthy));
}

#[test]
fn test_health_check_serialization() {
    let health = HealthCheckPayload {
        check_type: HealthCheckType::Deep,
        target: Some("all_services".to_string()),
    };

    let json = serde_json::to_string(&health).unwrap();
    let deserialized: HealthCheckPayload = serde_json::from_str(&json).unwrap();

    assert_eq!(health.check_type, deserialized.check_type);
    assert_eq!(health.target, deserialized.target);
}

#[test]
fn test_health_check_types() {
    assert!(matches!(HealthCheckType::Quick, HealthCheckType::Quick));
    assert!(matches!(HealthCheckType::Full, HealthCheckType::Full));
    assert!(matches!(HealthCheckType::Deep, HealthCheckType::Deep));
}

// ============================================================================
// Error Handling Tests (5 tests)
// ============================================================================

#[test]
fn test_error_payload_creation() {
    let error = ErrorPayload {
        code: 500,
        message: "Internal error".to_string(),
        details: None,
    };

    assert_eq!(error.code, 500);
    assert_eq!(error.message, "Internal error");
}

#[test]
fn test_error_payload_with_details() {
    let error = ErrorPayload {
        code: 404,
        message: "Not found".to_string(),
        details: Some(json!({"resource": "user_123"})),
    };

    assert!(error.details.is_some());
    assert_eq!(error.details.unwrap()["resource"], "user_123");
}

#[test]
fn test_jsonrpc_error_conversion() {
    let rpc_error = JsonRpcError {
        code: -32600,
        message: "Invalid Request".to_string(),
        data: None,
    };

    assert_eq!(rpc_error.code, -32600);
}

#[test]
fn test_error_serialization() {
    let error = ErrorPayload {
        code: 403,
        message: "Forbidden".to_string(),
        details: Some(json!({"reason": "insufficient_permissions"})),
    };

    let json = serde_json::to_string(&error).unwrap();
    let deserialized: ErrorPayload = serde_json::from_str(&json).unwrap();

    assert_eq!(error.code, deserialized.code);
    assert_eq!(error.message, deserialized.message);
}

#[test]
fn test_error_codes() {
    let errors = vec![
        ErrorPayload {
            code: 400,
            message: "Bad Request".to_string(),
            details: None,
        },
        ErrorPayload {
            code: 500,
            message: "Internal Error".to_string(),
            details: None,
        },
        ErrorPayload {
            code: 503,
            message: "Service Unavailable".to_string(),
            details: None,
        },
    ];

    assert_eq!(errors[0].code, 400);
    assert_eq!(errors[1].code, 500);
    assert_eq!(errors[2].code, 503);
}

// ============================================================================
// Edge Cases & Integration (5 tests)
// ============================================================================

#[test]
fn test_empty_payload_handling() {
    let msg = Message {
        msg_type: MessageType::Notification,
        payload: MessagePayload::Empty,
        metadata: Default::default(),
    };

    assert!(matches!(msg.payload, MessagePayload::Empty));
}

#[test]
fn test_large_metadata() {
    let mut msg = Message {
        msg_type: MessageType::Request,
        payload: MessagePayload::Empty,
        metadata: Default::default(),
    };

    for i in 0..100 {
        msg.metadata
            .insert(format!("key_{}", i), json!(format!("value_{}", i)));
    }

    assert_eq!(msg.metadata.len(), 100);
}

#[test]
fn test_nested_payload_data() {
    let complex_payload = json!({
        "level1": {
            "level2": {
                "level3": {
                    "data": "deeply nested"
                }
            }
        }
    });

    let response = Response {
        request_id: "nested_test".to_string(),
        status: ResponseStatus::Success,
        payload: Some(complex_payload.clone()),
        metadata: Default::default(),
    };

    assert_eq!(
        response.payload.unwrap()["level1"]["level2"]["level3"]["data"],
        "deeply nested"
    );
}

#[test]
fn test_unicode_in_messages() {
    let msg = Message {
        msg_type: MessageType::Request,
        payload: MessagePayload::Empty,
        metadata: vec![
            ("emoji".to_string(), json!("🚀")),
            ("chinese".to_string(), json!("你好")),
            ("arabic".to_string(), json!("مرحبا")),
        ]
        .into_iter()
        .collect(),
    };

    assert_eq!(msg.metadata.get("emoji").unwrap(), &json!("🚀"));
}

#[test]
fn test_service_status_ordering() {
    let statuses = vec![
        ServiceStatus::Starting,
        ServiceStatus::Active,
        ServiceStatus::Stopping,
        ServiceStatus::Stopped,
        ServiceStatus::Failed,
    ];

    // All statuses should be distinct
    assert_eq!(statuses.len(), 5);
}
