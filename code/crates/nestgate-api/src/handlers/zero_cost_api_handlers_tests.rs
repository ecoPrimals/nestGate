//! Tests for zero-cost API handlers module

use super::zero_cost_api_handlers::*;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_zero_cost_api_request_creation() {
    let request = ZeroCostApiRequest {
        data: json!({"key": "value"}),
        request_id: Arc::new("req-001".to_string()),
        timestamp: std::time::SystemTime::now(),
        _metadata: Arc::new(HashMap::new()),
    };

    assert_eq!(request.request_id.as_str(), "req-001");
    assert!(request._metadata.is_empty());
}

#[test]
fn test_zero_cost_api_response_creation() {
    let response = ZeroCostApiResponse {
        data: json!({"result": "success"}),
        request_id: Arc::new("req-001".to_string()),
        status: ApiStatus::Success,
        processing_time_ms: 42,
        _metadata: HashMap::new(),
    };

    assert_eq!(response.request_id.as_str(), "req-001");
    assert_eq!(response.processing_time_ms, 42);
    assert!(matches!(response.status, ApiStatus::Success));
}

#[test]
fn test_api_status_success() {
    let status = ApiStatus::Success;
    assert!(matches!(status, ApiStatus::Success));
}

#[test]
fn test_api_status_warning() {
    let status = ApiStatus::Warning {
        message: "Test warning".to_string(),
    };
    match status {
        ApiStatus::Warning { message } => assert_eq!(message, "Test warning"),
        _ => panic!("Expected Warning status"),
    }
}

#[test]
fn test_api_status_error() {
    let status = ApiStatus::Error {
        code: "ERR001".to_string(),
        message: "Test error".to_string(),
    };
    match status {
        ApiStatus::Error { code, message } => {
            assert_eq!(code, "ERR001");
            assert_eq!(message, "Test error");
        }
        _ => panic!("Expected Error status"),
    }
}

#[test]
fn test_zero_cost_pool_handler_creation() {
    let _handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    assert_eq!(ZeroCostPoolHandler::<100, 5000>::max_requests(), 100);
    assert_eq!(ZeroCostPoolHandler::<100, 5000>::timeout_ms(), 5000);
}

#[test]
fn test_zero_cost_pool_handler_default() {
    let _handler: ZeroCostPoolHandler<50, 3000> = ZeroCostPoolHandler::default();
    assert_eq!(ZeroCostPoolHandler::<50, 3000>::max_requests(), 50);
    assert_eq!(ZeroCostPoolHandler::<50, 3000>::timeout_ms(), 3000);
}

#[test]
fn test_zero_cost_pool_handler_const_generics() {
    // Test different const generic values
    assert_eq!(ZeroCostPoolHandler::<10, 1000>::max_requests(), 10);
    assert_eq!(ZeroCostPoolHandler::<10, 1000>::timeout_ms(), 1000);

    assert_eq!(ZeroCostPoolHandler::<500, 10000>::max_requests(), 500);
    assert_eq!(ZeroCostPoolHandler::<500, 10000>::timeout_ms(), 10000);
}

#[test]
fn test_handle_list_pools() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let result = handler.handle_list_pools();

    assert!(result.is_ok());
    let pools = result.unwrap().0;
    assert_eq!(pools.len(), 2);

    // Check first pool
    assert_eq!(pools[0]["name"], "tank");
    assert_eq!(pools[0]["state"], "ONLINE");
    assert_eq!(pools[0]["health"], "OK");

    // Check second pool
    assert_eq!(pools[1]["name"], "backup");
    assert_eq!(pools[1]["state"], "ONLINE");
}

#[test]
fn test_handle_get_pool_tank() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let result = handler.handle_get_pool("tank".to_string());

    assert!(result.is_ok());
    let pool = result.unwrap().0;

    assert_eq!(pool["name"], "tank");
    assert_eq!(pool["state"], "ONLINE");
    assert_eq!(pool["health"], "OK");
    assert_eq!(pool["size"], "1TB");
    assert_eq!(pool["used"], "500GB");
    assert_eq!(pool["available"], "500GB");
    assert_eq!(pool["compression"], "lz4");
    assert_eq!(pool["deduplication"], false);
}

#[test]
fn test_handle_get_pool_backup() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let result = handler.handle_get_pool("backup".to_string());

    assert!(result.is_ok());
    let pool = result.unwrap().0;

    assert_eq!(pool["name"], "backup");
    assert_eq!(pool["size"], "2TB");
    assert_eq!(pool["used"], "1TB");
    assert_eq!(pool["compression"], "zstd");
    assert_eq!(pool["deduplication"], true);
}

#[test]
fn test_handle_get_pool_not_found() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let result = handler.handle_get_pool("nonexistent".to_string());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), axum::http::StatusCode::NOT_FOUND);
}

#[test]
fn test_handle_create_pool() {
    use crate::zfs::types::PoolConfig;

    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let config = PoolConfig {
        raid_level: Some("mirror".to_string()),
        compression: Some("zstd".to_string()),
        dedup: Some(true),
        encryption: Some(true),
    };

    let result = handler.handle_create_pool(config);
    assert!(result.is_ok());

    let response = result.unwrap().0;
    assert_eq!(response["status"], "created");
    assert_eq!(response["name"], "new_pool");
    assert_eq!(response["properties"]["compression"], "zstd");
    assert_eq!(response["properties"]["deduplication"], true);
    assert_eq!(response["properties"]["encryption"], true);
}

#[test]
fn test_handle_create_pool_default_config() {
    use crate::zfs::types::PoolConfig;

    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let config = PoolConfig {
        raid_level: None,
        compression: None,
        dedup: None,
        encryption: None,
    };

    let result = handler.handle_create_pool(config);
    assert!(result.is_ok());

    let response = result.unwrap().0;
    assert_eq!(response["status"], "created");
    assert_eq!(response["properties"]["compression"], "lz4");
    assert_eq!(response["properties"]["deduplication"], false);
    assert_eq!(response["properties"]["encryption"], false);
}

#[test]
fn test_handle_delete_pool_success() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let result = handler.handle_delete_pool("tank".to_string());

    assert!(result.is_ok());
    let response = result.unwrap().0;
    assert_eq!(response["status"], "deleted");
    assert_eq!(response["name"], "tank");
}

#[test]
fn test_handle_delete_pool_not_found() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let result = handler.handle_delete_pool("nonexistent".to_string());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), axum::http::StatusCode::NOT_FOUND);
}

#[test]
fn test_handle_delete_pool_empty_name() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let result = handler.handle_delete_pool(String::new());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_process_request_success() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let request = ZeroCostApiRequest {
        data: json!({"test": "data"}),
        request_id: Arc::new("req-001".to_string()),
        timestamp: std::time::SystemTime::now(),
        _metadata: Arc::new(HashMap::new()),
    };

    let result = handler.process_request(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.request_id.as_str(), "req-001");
    assert!(matches!(response.status, ApiStatus::Success));
    assert!(response.processing_time_ms < 5000);
}

#[tokio::test]
async fn test_process_request_caching() {
    let handler: ZeroCostPoolHandler<2, 5000> = ZeroCostPoolHandler::new();

    // Send multiple requests to test cache limit
    // The handler internally manages cache eviction
    for i in 0..3 {
        let request = ZeroCostApiRequest {
            data: json!({"iteration": i}),
            request_id: Arc::new(format!("req-{i:03}")),
            timestamp: std::time::SystemTime::now(),
            _metadata: Arc::new(HashMap::new()),
        };

        let result = handler.process_request(request).await;
        assert!(result.is_ok());
    }

    // Cache management is internal - we can only verify requests succeeded
    // The cache should have automatically evicted oldest entries when limit reached
}

#[tokio::test]
async fn test_process_request_with_metadata() {
    let handler: ZeroCostPoolHandler<100, 5000> = ZeroCostPoolHandler::new();
    let mut metadata = HashMap::new();
    metadata.insert("user_id".to_string(), "user-123".to_string());
    metadata.insert("trace_id".to_string(), "trace-456".to_string());

    let request = ZeroCostApiRequest {
        data: json!({"test": "data"}),
        request_id: Arc::new("req-001".to_string()),
        timestamp: std::time::SystemTime::now(),
        _metadata: Arc::new(metadata),
    };

    let result = handler.process_request(request).await;
    assert!(result.is_ok());
}

#[test]
fn test_api_error_types() {
    use crate::handlers::zero_cost_api_handlers::ApiError;

    let error = ApiError::ProcessingFailed;
    let error_string = format!("{error:?}");
    assert!(error_string.contains("ProcessingFailed"));

    let error = ApiError::Timeout;
    let error_string = format!("{error:?}");
    assert!(error_string.contains("Timeout"));
}

#[test]
fn test_zero_cost_request_with_different_types() {
    // Test with string data
    let string_request = ZeroCostApiRequest {
        data: "test string".to_string(),
        request_id: Arc::new("req-001".to_string()),
        timestamp: std::time::SystemTime::now(),
        _metadata: Arc::new(HashMap::new()),
    };
    assert_eq!(string_request.data, "test string");

    // Test with numeric data
    let numeric_request = ZeroCostApiRequest {
        data: 42,
        request_id: Arc::new("req-002".to_string()),
        timestamp: std::time::SystemTime::now(),
        _metadata: Arc::new(HashMap::new()),
    };
    assert_eq!(numeric_request.data, 42);

    // Test with complex JSON
    let json_request = ZeroCostApiRequest {
        data: json!({"nested": {"key": "value"}}),
        request_id: Arc::new("req-003".to_string()),
        timestamp: std::time::SystemTime::now(),
        _metadata: Arc::new(HashMap::new()),
    };
    assert_eq!(json_request.data["nested"]["key"], "value");
}

#[test]
fn test_zero_cost_response_with_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("cache_hit".to_string(), "true".to_string());
    metadata.insert("processing_node".to_string(), "node-1".to_string());

    let response = ZeroCostApiResponse {
        data: json!({"result": "ok"}),
        request_id: Arc::new("req-001".to_string()),
        status: ApiStatus::Success,
        processing_time_ms: 15,
        _metadata: metadata,
    };

    assert_eq!(response._metadata.get("cache_hit").unwrap(), "true");
    assert_eq!(response._metadata.get("processing_node").unwrap(), "node-1");
}

#[test]
fn test_pool_handler_different_const_configs() {
    // Small cache, fast timeout
    let handler1: ZeroCostPoolHandler<10, 100> = ZeroCostPoolHandler::new();
    assert_eq!(ZeroCostPoolHandler::<10, 100>::max_requests(), 10);
    assert_eq!(ZeroCostPoolHandler::<10, 100>::timeout_ms(), 100);

    // Large cache, slow timeout
    let handler2: ZeroCostPoolHandler<1000, 30000> = ZeroCostPoolHandler::new();
    assert_eq!(ZeroCostPoolHandler::<1000, 30000>::max_requests(), 1000);
    assert_eq!(ZeroCostPoolHandler::<1000, 30000>::timeout_ms(), 30000);

    // Both handlers can coexist
    let _ = (handler1, handler2);
}

#[tokio::test]
async fn test_concurrent_request_processing() {
    let handler = std::sync::Arc::new(ZeroCostPoolHandler::<100, 5000>::new());

    let mut handles = vec![];

    for i in 0..10 {
        let handler_clone = handler.clone();
        let handle = tokio::spawn(async move {
            let request = ZeroCostApiRequest {
                data: json!({"id": i}),
                request_id: Arc::new(format!("req-{i:03}")),
                timestamp: std::time::SystemTime::now(),
                _metadata: Arc::new(HashMap::new()),
            };
            handler_clone.process_request(request).await
        });
        handles.push(handle);
    }

    // Wait for all requests to complete
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());
    }
}
