//! **INTEGRATION TESTS**
//!
//! Tests for transport + protocol + handlers integration.

use nestgate_api::transport::{
    JsonRpcHandler, 
    JsonRpcHandler, JsonRpcRequest, NestGateRpcHandler, TransportConfig,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

// ============================================================================
// Transport + Handler Integration (3 tests)
// ============================================================================

#[tokio::test]
async fn test_handler_request_response_flow() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Create request
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    // Handle request
    let response = handler.handle_request(request).await;

    // Verify response
    assert!(response.error.is_none());
    assert!(response.result.is_some());
    assert_eq!(response.id, 1);

    if let Some(result) = response.result {
        assert_eq!(result["status"], "pong");
    }
}

#[tokio::test]
async fn test_config_to_handler_integration() {
    let config = TransportConfig::new("integration_test")
        .with_socket_path("/tmp/integration_test.sock")
        .with_verbose();

    assert!(config.validate().is_ok());

    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Handler should work with any config
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "identity.get".to_string(),
        params: json!({}),
        id: Value::from(2),
    };

    let response = handler.handle_request(request).await;
    assert!(response.error.is_none());
}

#[tokio::test]
async fn test_multiple_handlers_concurrent() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    let mut handles = vec![];

    for i in 0..20 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"id": i}),
                id: Value::from(i as i64),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    for (i, handle) in handles.into_iter().enumerate() {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
        assert_eq!(response.id, i as i64);
    }
}

// ============================================================================
// Config + Environment Integration (2 tests)
// ============================================================================

#[tokio::test]
async fn test_config_env_to_transport() {
    std::env::set_var("NESTGATE_FAMILY_ID", "integration");
    std::env::set_var("NESTGATE_SOCKET_PATH", "/tmp/integration.sock");
    std::env::set_var("NESTGATE_VERBOSE", "true");

    let config = TransportConfig::from_env().unwrap();

    assert_eq!(config.family_id, "integration");
    assert!(config.verbose);

    // Config should be valid
    assert!(config.validate().is_ok());

    // Cleanup
    std::env::remove_var("NESTGATE_FAMILY_ID");
    std::env::remove_var("NESTGATE_SOCKET_PATH");
    std::env::remove_var("NESTGATE_VERBOSE");
}

#[tokio::test]
async fn test_config_precedence() {
    // Set both env and manual config
    std::env::set_var("NESTGATE_FAMILY_ID", "env_family");

    let manual_config = TransportConfig::new("manual_family").with_socket_path("/tmp/manual.sock");

    // Manual config should take precedence
    assert_eq!(manual_config.family_id, "manual_family");
    assert!(manual_config
        .socket_path
        .to_str()
        .unwrap()
        .contains("manual"));

    // Cleanup
    std::env::remove_var("NESTGATE_FAMILY_ID");
}

// ============================================================================
// Error Path Integration (2 tests)
// ============================================================================

#[tokio::test]
async fn test_invalid_method_error_flow() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "invalid.nonexistent.method".to_string(),
        params: json!({}),
        id: Value::from(100),
    };

    let response = handler.handle_request(request).await;

    assert!(response.result.is_none());
    assert!(response.error.is_some());

    let error = response.error.unwrap();
    assert_eq!(error.code, -32601); // Method not found
}

#[tokio::test]
async fn test_error_recovery_and_next_request() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // First request fails
    let bad_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "bad.method".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    let error_response = handler.handle_request(bad_request).await;
    assert!(error_response.error.is_some());

    // Second request should succeed
    let good_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(2),
    };

    let success_response = handler.handle_request(good_request).await;
    assert!(success_response.error.is_none());
    assert!(success_response.result.is_some());
}

// ============================================================================
// End-to-End Scenarios (3 tests)
// ============================================================================

#[tokio::test]
async fn test_e2e_health_check_flow() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // 1. Ping
    let ping_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };
    let ping_resp = handler.handle_request(ping_req).await;
    assert_eq!(ping_resp.result.unwrap()["status"], "pong");

    // 2. Status
    let status_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.status".to_string(),
        params: json!({}),
        id: Value::from(2),
    };
    let status_resp = handler.handle_request(status_req).await;
    assert!(status_resp.result.is_some());

    // 3. Identity
    let identity_req = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "identity.get".to_string(),
        params: json!({}),
        id: Value::from(3),
    };
    let identity_resp = handler.handle_request(identity_req).await;
    assert!(identity_resp.result.is_some());
}

#[tokio::test]
async fn test_e2e_config_handler_lifecycle() {
    // 1. Create config from environment
    std::env::set_var("NESTGATE_FAMILY_ID", "e2e_test");
    let config = TransportConfig::from_env().unwrap();
    std::env::remove_var("NESTGATE_FAMILY_ID");

    // 2. Validate config
    assert!(config.validate().is_ok());

    // 3. Create handler
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // 4. Send request
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    // 5. Get response
    let response = handler.handle_request(request).await;

    // 6. Verify success
    assert!(response.error.is_none());
    assert!(response.result.is_some());
}

#[tokio::test]
async fn test_e2e_stress_sequential_requests() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    for i in 0..100 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({"iteration": i}),
            id: Value::from(i as i64),
        };

        let response = handler.handle_request(request).await;

        assert!(response.error.is_none(), "Request {} failed", i);
        assert_eq!(response.id, i as i64);
    }
}

// ============================================================================
// Performance & Load Tests (2 tests)
// ============================================================================

#[tokio::test]
async fn test_concurrent_load() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    let mut handles = vec![];

    // Spawn 50 concurrent requests
    for i in 0..50 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: if i % 3 == 0 {
                    "health.ping".to_string()
                } else if i % 3 == 1 {
                    "health.status".to_string()
                } else {
                    "identity.get".to_string()
                },
                params: json!({"id": i}),
                id: Value::from(i as i64),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    // All should succeed
    for handle in handles {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
    }
}

#[tokio::test]
async fn test_rapid_fire_requests() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let start = std::time::Instant::now();

    for i in 0..1000 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({}),
            id: Value::from(i),
        };

        let response = handler.handle_request(request).await;
        assert!(response.error.is_none());
    }

    let duration = start.elapsed();

    // Should complete 1000 requests in reasonable time
    assert!(duration.as_secs() < 10, "Too slow: {:?}", duration);
}
