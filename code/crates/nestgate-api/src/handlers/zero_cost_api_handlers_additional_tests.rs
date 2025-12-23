//! **ADDITIONAL ZERO-COST API HANDLER TESTS**
//!
//! Comprehensive tests for zero-cost API handler functionality.

use super::*;

use std::sync::Arc;
// Pool Handler Tests

#[test]
fn test_pool_handler_list_pools() {
    let handler = ZeroCostPoolHandler::<100, 5000>::new();
    let result = handler.handle_list_pools();

    assert!(result.is_ok());
    let pools = result.expect("Test setup failed").0;
    assert_eq!(pools.len(), 2);
    assert!(pools[0].get("name").is_some());
}

#[test]
fn test_pool_handler_get_pool_tank() {
    let handler = ZeroCostPoolHandler::<100, 5000>::new();
    let result = handler.handle_get_pool("tank".to_string());

    assert!(result.is_ok());
    let pool = result.expect("Test setup failed").0;
    assert_eq!(pool.get("name").and_then(|v| v.as_str()), Some("tank"));
    assert_eq!(pool.get("state").and_then(|v| v.as_str()), Some("ONLINE"));
}

#[test]
fn test_pool_handler_get_pool_backup() {
    let handler = ZeroCostPoolHandler::<100, 5000>::new();
    let result = handler.handle_get_pool("backup".to_string());

    assert!(result.is_ok());
    let pool = result.expect("Test setup failed").0;
    assert_eq!(pool.get("name").and_then(|v| v.as_str()), Some("backup"));
}

#[test]
fn test_pool_handler_get_pool_not_found() {
    let handler = ZeroCostPoolHandler::<100, 5000>::new();
    let result = handler.handle_get_pool("nonexistent".to_string());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);
}

#[test]
fn test_pool_handler_create_pool() {
    let handler = ZeroCostPoolHandler::<100, 5000>::new();
    let config = PoolConfig {
        compression: Some("lz4".to_string()),
        dedup: Some(false),
        encryption: Some(true),
        ..Default::default()
    };

    let result = handler.handle_create_pool(config);
    assert!(result.is_ok());

    let response = result.expect("Test setup failed").0;
    assert_eq!(
        response.get("status").and_then(|v| v.as_str()),
        Some("created")
    );
}

#[test]
fn test_pool_handler_delete_pool_tank() {
    let handler = ZeroCostPoolHandler::<100, 5000>::new();
    let result = handler.handle_delete_pool("tank".to_string());

    assert!(result.is_ok());
    let response = result.expect("Test setup failed").0;
    assert_eq!(
        response.get("status").and_then(|v| v.as_str()),
        Some("deleted")
    );
}

#[test]
fn test_pool_handler_delete_pool_empty_name() {
    let handler = ZeroCostPoolHandler::<100, 5000>::new();
    let result = handler.handle_delete_pool("".to_string());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
}

#[test]
fn test_pool_handler_delete_pool_not_found() {
    let handler = ZeroCostPoolHandler::<100, 5000>::new();
    let result = handler.handle_delete_pool("nonexistent".to_string());

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), StatusCode::NOT_FOUND);
}

// Const Generic Configuration Tests

#[test]
fn test_pool_handler_max_requests() {
    assert_eq!(ZeroCostPoolHandler::<100, 5000>::max_requests(), 100);
    assert_eq!(ZeroCostPoolHandler::<1000, 10000>::max_requests(), 1000);
}

#[test]
fn test_pool_handler_timeout_ms() {
    assert_eq!(ZeroCostPoolHandler::<100, 5000>::timeout_ms(), 5000);
    assert_eq!(ZeroCostPoolHandler::<100, 10000>::timeout_ms(), 10000);
}

#[test]
fn test_development_pool_handler_limits() {
    let _handler = DevelopmentPoolHandler::new();
    assert_eq!(DevelopmentPoolHandler::max_requests(), 100);
    assert_eq!(DevelopmentPoolHandler::timeout_ms(), 5000);
}

#[test]
fn test_production_pool_handler_limits() {
    let _handler = ProductionPoolHandler::new();
    assert_eq!(ProductionPoolHandler::max_requests(), 1000);
    assert_eq!(ProductionPoolHandler::timeout_ms(), 10_000);
}

#[test]
fn test_enterprise_pool_handler_limits() {
    let _handler = EnterprisePoolHandler::new();
    assert_eq!(EnterprisePoolHandler::max_requests(), 10_000);
    assert_eq!(EnterprisePoolHandler::timeout_ms(), 30000);
}

#[test]
fn test_high_throughput_pool_handler_limits() {
    let _handler = HighThroughputPoolHandler::new();
    assert_eq!(HighThroughputPoolHandler::max_requests(), 50000);
    assert_eq!(HighThroughputPoolHandler::timeout_ms(), 60000);
}

// Router Builder Tests

#[test]
fn test_router_builder_creation() {
    let builder = ZeroCostRouterBuilder::<100, 10>::new();
    assert!(builder.can_add_route());
    assert!(builder.can_add_middleware());
}

#[test]
fn test_router_builder_max_routes() {
    assert_eq!(ZeroCostRouterBuilder::<100, 10>::max_routes(), 100);
    assert_eq!(ZeroCostRouterBuilder::<50, 5>::max_routes(), 50);
}

#[test]
fn test_router_builder_max_middleware() {
    assert_eq!(ZeroCostRouterBuilder::<100, 10>::max_middleware(), 10);
    assert_eq!(ZeroCostRouterBuilder::<50, 20>::max_middleware(), 20);
}

#[test]
fn test_router_builder_default() {
    let builder = ZeroCostRouterBuilder::<100, 10>::default();
    assert!(builder.can_add_route());
}

// Dataset Type Tests

#[test]
fn test_dataset_type_filesystem() {
    let dataset_type = DatasetType::Filesystem;
    let json = serde_json::to_string(&dataset_type);
    assert!(json.is_ok());
}

#[test]
fn test_dataset_type_volume() {
    let dataset_type = DatasetType::Volume {
        size: 1024 * 1024 * 1024,
    };
    let json = serde_json::to_string(&dataset_type);
    assert!(json.is_ok());

    let json_str = json.expect("Test setup failed");
    assert!(json_str.contains("Volume"));
}

#[test]
fn test_dataset_config_creation() {
    let config = DatasetConfig {
        name: "test-dataset".to_string(),
        pool: "test-pool".to_string(),
        dataset_type: DatasetType::Filesystem,
        properties: HashMap::new(),
    };

    assert_eq!(config.name, "test-dataset");
    assert_eq!(config.pool, "test-pool");
}

#[test]
fn test_dataset_config_with_properties() {
    let mut properties = HashMap::new();
    properties.insert("compression".to_string(), "lz4".to_string());
    properties.insert("quota".to_string(), "10G".to_string());

    let config = DatasetConfig {
        name: "data".to_string(),
        pool: "tank".to_string(),
        dataset_type: DatasetType::Filesystem,
        properties,
    };

    assert_eq!(config.properties.len(), 2);
    assert_eq!(
        config.properties.get("compression"),
        Some(&"lz4".to_string())
    );
}

#[test]
fn test_dataset_info_structure() {
    let info = DatasetInfo {
        name: "tank/data".to_string(),
        pool: "tank".to_string(),
        dataset_type: DatasetType::Filesystem,
        size: 1024 * 1024 * 1024 * 100,     // 100GB
        used: 1024 * 1024 * 1024 * 50,      // 50GB
        available: 1024 * 1024 * 1024 * 50, // 50GB
        mount_point: Some("/mnt/tank/data".to_string()),
        created_at: std::time::SystemTime::now(),
    };

    assert_eq!(info.name, "tank/data");
    assert_eq!(info.used, 1024 * 1024 * 1024 * 50);
    assert!(info.mount_point.is_some());
}

// Error Type Tests

#[test]
fn test_api_error_processing_failed() {
    let error = ApiError::ProcessingFailed;
    let display = format!("{error}");
    assert_eq!(display, "Request processing failed");
}

#[test]
fn test_api_error_timeout() {
    let error = ApiError::Timeout;
    let display = format!("{error}");
    assert_eq!(display, "Request timeout");
}

#[test]
fn test_api_error_validation() {
    let error = ApiError::Validation("Invalid input".to_string());
    let display = format!("{error}");
    assert!(display.contains("Invalid input"));
}

#[test]
fn test_api_error_internal() {
    let error = ApiError::Internal("Database error".to_string());
    let display = format!("{error}");
    assert!(display.contains("Database error"));
}

#[test]
fn test_zero_cost_api_error_variants() {
    let errors = vec![
        ZeroCostApiError::ProcessingFailed,
        ZeroCostApiError::Timeout,
        ZeroCostApiError::Validation("test".to_string()),
        ZeroCostApiError::Internal("test".to_string()),
    ];

    for error in errors {
        let display = format!("{error}");
        assert!(!display.is_empty());
    }
}

// Migration Guide Tests

#[test]
fn test_migration_steps() {
    let steps = ApiHandlerMigrationGuide::migration_steps();
    assert_eq!(steps.len(), 8);
    assert!(steps[0].contains("async_trait"));
    assert!(steps[1].contains("const generics"));
}

#[test]
fn test_expected_improvements() {
    let (perf, memory, latency) = ApiHandlerMigrationGuide::expected_improvements();
    assert_eq!(perf, 35.0);
    assert_eq!(memory, 25.0);
    assert_eq!(latency, 20.0);
}

// Additional Request/Response Tests

#[test]
fn test_request_clone() {
    let request = ZeroCostApiRequest {
        data: serde_json::json!({"test": "data"}),
        request_id: Arc::new("clone-test".to_string()),
        timestamp: std::time::SystemTime::now(),
        _metadata: Arc::new(HashMap::new()),
    };

    let cloned = request.clone();
    assert_eq!(cloned.request_id, "clone-test");
}

#[test]
fn test_response_clone() {
    let response = ZeroCostApiResponse {
        data: serde_json::json!({"result": "ok"}),
        request_id: Arc::new("resp-clone".to_string()),
        status: ApiStatus::Success,
        processing_time_ms: 10,
        _metadata: Arc::new(HashMap::new()),
    };

    let cloned = response.clone();
    assert_eq!(cloned.request_id, "resp-clone");
    assert_eq!(cloned.processing_time_ms, 10);
}

#[test]
fn test_api_status_success_serialization() {
    let status = ApiStatus::Success;
    let json = serde_json::to_string(&status);
    assert!(json.is_ok());
}

#[test]
fn test_api_status_warning_serialization() {
    let status = ApiStatus::Warning {
        message: "Deprecation warning".to_string(),
    };
    let json = serde_json::to_string(&status);
    assert!(json.is_ok());
    assert!(json.expect("Test setup failed").contains("Deprecation"));
}

#[test]
fn test_api_status_error_serialization() {
    let status = ApiStatus::Error {
        code: "ERR500".to_string(),
        message: "Internal server error".to_string(),
    };
    let json = serde_json::to_string(&status);
    assert!(json.is_ok());

    let json_str = json.expect("Test setup failed");
    assert!(json_str.contains("ERR500"));
    assert!(json_str.contains("Internal server error"));
}

#[tokio::test]
async fn test_pool_handler_default() {
    let handler = ZeroCostPoolHandler::<100, 5000>::default();
    let result = handler.handle_list_pools();
    assert!(result.is_ok());
}

#[test]
fn test_dataset_config_serialization() {
    let config = DatasetConfig {
        name: "test".to_string(),
        pool: "tank".to_string(),
        dataset_type: DatasetType::Filesystem,
        properties: HashMap::new(),
    };

    let json = serde_json::to_string(&config);
    assert!(json.is_ok());

    let json_str = json.expect("Test setup failed");
    assert!(json_str.contains("test"));
    assert!(json_str.contains("tank"));
}

#[test]
fn test_dataset_info_serialization() {
    let info = DatasetInfo {
        name: "data".to_string(),
        pool: "pool".to_string(),
        dataset_type: DatasetType::Volume { size: 1024 },
        size: 2048,
        used: 512,
        available: 1536,
        mount_point: None,
        created_at: std::time::SystemTime::now(),
    };

    let json = serde_json::to_string(&info);
    assert!(json.is_ok());
}

#[test]
fn test_multiple_pool_handlers_different_configs() {
    let dev = DevelopmentPoolHandler::new();
    let prod = ProductionPoolHandler::new();
    let ent = EnterprisePoolHandler::new();

    // All should list pools successfully
    assert!(dev.handle_list_pools().is_ok());
    assert!(prod.handle_list_pools().is_ok());
    assert!(ent.handle_list_pools().is_ok());
}

#[test]
fn test_pool_creation_with_default_config() {
    let handler = ProductionPoolHandler::new();
    let config = PoolConfig::default();

    let result = handler.handle_create_pool(config);
    assert!(result.is_ok());
}

#[test]
fn test_pool_handler_get_multiple_pools() {
    let handler = EnterprisePoolHandler::new();

    let tank = handler.handle_get_pool("tank".to_string());
    let backup = handler.handle_get_pool("backup".to_string());

    assert!(tank.is_ok());
    assert!(backup.is_ok());
}

#[tokio::test]
async fn test_benchmark_zero_requests() {
    let duration = ApiHandlerBenchmark::benchmark_api_operations(0).await;
    // Should complete quickly with no requests
    assert!(duration.as_millis() < 10);
}

#[tokio::test]
async fn test_benchmark_many_requests() {
    let duration = ApiHandlerBenchmark::benchmark_api_operations(100).await;
    // Should take at least 10ms (100 * 100μs)
    assert!(duration.as_micros() >= 10000);
}
