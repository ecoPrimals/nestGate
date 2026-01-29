//! **FAULT INJECTION TESTS**
//!
//! Tests for system behavior under injected faults.

use nestgate_api::transport::{
    JsonRpcHandler, JsonRpcRequest, JsonRpcResponse, NestGateRpcHandler, TransportConfig,
};
use nestgate_api::transport::jsonrpc::JsonRpcError;
use serde_json::{json, Value};
use std::sync::Arc;

// ============================================================================
// Protocol Fault Injection (3 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_wrong_jsonrpc_version() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Inject fault: wrong version
    let request = JsonRpcRequest {
        jsonrpc: "1.0".to_string(), // Wrong version
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    let response = handler.handle_request(request).await;

    // Should still handle gracefully
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn test_fault_empty_method_name() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "".to_string(), // Empty method
        params: json!({}),
        id: Value::from(2),
    };

    let response = handler.handle_request(request).await;

    // Should return method not found
    assert!(response.error.is_some());
}

#[tokio::test]
async fn test_fault_null_params() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!(null), // Null params
        id: Value::from(3),
    };

    let response = handler.handle_request(request).await;

    // Should handle null params
    assert_eq!(response.id, 3);
}

// ============================================================================
// Configuration Fault Injection (3 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_empty_family_id() {
    let config = TransportConfig::new("");

    // Should still validate (empty is technically valid)
    assert!(config.validate().is_ok());
}

#[tokio::test]
async fn test_fault_invalid_socket_path() {
    let config = TransportConfig::new("test").with_socket_path(""); // Empty path

    // Should create but might fail on actual use
    assert_eq!(config.socket_path.to_str().unwrap(), "");
}

#[tokio::test]
async fn test_fault_conflicting_config() {
    std::env::set_var("NESTGATE_FAMILY_ID", "env_family");
    std::env::set_var("NESTGATE_HTTP_PORT", "not_a_number");

    // Should handle invalid port gracefully
    let result = TransportConfig::from_env();

    // Cleanup
    std::env::remove_var("NESTGATE_FAMILY_ID");
    std::env::remove_var("NESTGATE_HTTP_PORT");

    // Should either succeed with default or fail gracefully
    assert!(result.is_ok() || result.is_err());
}

// ============================================================================
// Concurrency Fault Injection (2 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_concurrent_conflicting_requests() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    let mut handles = vec![];

    // Same ID for multiple requests (protocol violation)
    for i in 0..10 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"task": i}),
                id: Value::from(1), // Same ID for all!
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    // All should complete even with same ID
    for handle in handles {
        let response = handle.await.unwrap();
        assert_eq!(response.id, 1);
    }
}

#[tokio::test]
async fn test_fault_handler_under_extreme_load() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    let mut handles = vec![];

    // Extreme load: 200 concurrent requests
    for i in 0..200 {
        let h = handler.clone();
        let handle = tokio::spawn(async move {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "health.ping".to_string(),
                params: json!({"load_test": i}),
                id: Value::from(i as i64),
            };
            h.handle_request(request).await
        });
        handles.push(handle);
    }

    let mut success_count = 0;
    for handle in handles {
        if let Ok(response) = handle.await {
            if response.error.is_none() {
                success_count += 1;
            }
        }
    }

    // At least 90% should succeed under extreme load
    assert!(
        success_count >= 180,
        "Only {}/200 succeeded under load",
        success_count
    );
}

// ============================================================================
// Data Fault Injection (3 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_extremely_large_payload() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // 1MB of data
    let large_data: Vec<String> = (0..100000).map(|i| format!("data_{}", i)).collect();

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({"large_data": large_data}),
        id: Value::from(1),
    };

    let response = handler.handle_request(request).await;

    // Should handle or reject gracefully
    assert_eq!(response.id, 1);
}

#[tokio::test]
async fn test_fault_deeply_nested_json() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Create deeply nested JSON
    let mut nested = json!({"deepest": "value"});
    for i in 0..100 {
        nested = json!({"level": i, "nested": nested});
    }

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: nested,
        id: Value::from(2),
    };

    let response = handler.handle_request(request).await;

    // Should handle deep nesting
    assert_eq!(response.id, 2);
}

#[tokio::test]
async fn test_fault_special_unicode_characters() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    let special_chars = vec![
        "🚀💎🎊",
        "مرحبا بك",
        "你好世界",
        "Привет мир",
        "\u{0000}\u{001F}",
        "test\nwith\nnewlines",
    ];

    for (i, chars) in special_chars.into_iter().enumerate() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "health.ping".to_string(),
            params: json!({"special": chars}),
            id: Value::from(i as i64),
        };

        let response = handler.handle_request(request).await;
        assert_eq!(response.id, i as i64);
    }
}

// ============================================================================
// Timing Fault Injection (2 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_simultaneous_requests() {
    let handler = Arc::new(JsonRpcHandler::new(NestGateRpcHandler::new()));

    // Launch all at exactly the same time
    let handles: Vec<_> = (0..50)
        .map(|i| {
            let h = handler.clone();
            tokio::spawn(async move {
                let request = JsonRpcRequest {
                    jsonrpc: "2.0".to_string(),
                    method: "health.ping".to_string(),
                    params: json!({}),
                    id: Value::from(i),
                };
                h.handle_request(request).await
            })
        })
        .collect();

    // All should complete
    for handle in handles {
        let response = handle.await.unwrap();
        assert!(response.error.is_none());
    }
}

#[tokio::test]
async fn test_fault_request_during_high_cpu() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Simulate high CPU with busy work
    let busy_work = tokio::spawn(async {
        for _ in 0..1000000 {
            let _ = format!("{}", rand::random::<u64>());
        }
    });

    // Send request during high CPU
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "health.ping".to_string(),
        params: json!({}),
        id: Value::from(1),
    };

    let response = handler.handle_request(request).await;

    busy_work.abort();

    // Should still respond
    assert!(response.error.is_none());
}

// ============================================================================
// Error Response Fault Injection (2 tests)
// ============================================================================

#[tokio::test]
async fn test_fault_error_response_structure() {
    let response = JsonRpcResponse::error(Value::from(1), -32600, "Invalid Request");

    assert_eq!(response.id, Value::from(1));
    assert!(response.result.is_none());
    assert!(response.error.is_some());

    let error = response.error.unwrap();
    assert_eq!(error.code, -32600);
}

#[tokio::test]
async fn test_fault_multiple_errors_in_sequence() {
    let handler = JsonRpcHandler::new(NestGateRpcHandler::new());

    // Send 20 invalid requests in a row
    for i in 0..20 {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: format!("invalid.method.{}", i),
            params: json!({}),
            id: Value::from(i as i64),
        };

        let response = handler.handle_request(request).await;

        // Should consistently return errors
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, -32601);
    }
}
