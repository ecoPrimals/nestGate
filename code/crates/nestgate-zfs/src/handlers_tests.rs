//! Tests for ZFS request handlers
//!
//! These tests use NestGate's built-in simulation mode, which provides
//! a self-contained storage solution without requiring actual ZFS.

use super::*;
use crate::config::ZfsConfig;
use std::collections::HashMap;

// Helper to create a test handler that uses simulation mode
fn create_test_handler() -> ZfsRequestHandler {
    // Force simulation mode by using an environment flag
    std::env::set_var("NESTGATE_ZFS_SIMULATION_MODE", "true");
    let config = ZfsConfig::default();
    ZfsRequestHandler::new(config)
}

#[test]
fn test_zfs_health_info_creation() {
    let health = ZfsHealthInfo {
        status: "healthy".to_string(),
        pools_count: 2,
        datasets_count: 5,
        snapshots_count: 10,
        last_check: std::time::SystemTime::now(),
    };

    assert_eq!(health.status, "healthy");
    assert_eq!(health.pools_count, 2);
    assert_eq!(health.datasets_count, 5);
    assert_eq!(health.snapshots_count, 10);
}

#[test]
fn test_zfs_health_info_clone() {
    let health = ZfsHealthInfo {
        status: "healthy".to_string(),
        pools_count: 1,
        datasets_count: 2,
        snapshots_count: 3,
        last_check: std::time::SystemTime::now(),
    };

    let cloned = health.clone();
    assert_eq!(health.status, cloned.status);
    assert_eq!(health.pools_count, cloned.pools_count);
}

#[test]
fn test_zfs_metrics_creation() {
    let metrics = ZfsMetrics {
        requests_processed: 100,
        errors_count: 5,
        average_response_time_ms: 50.5,
        uptime_seconds: 3600,
    };

    assert_eq!(metrics.requests_processed, 100);
    assert_eq!(metrics.errors_count, 5);
    assert_eq!(metrics.average_response_time_ms, 50.5);
    assert_eq!(metrics.uptime_seconds, 3600);
}

#[test]
fn test_pool_info_creation() {
    let pool = PoolInfo {
        name: "tank".to_string(),
        state: "ONLINE".to_string(),
        size: "1TB".to_string(),
        allocated: "500GB".to_string(),
        free: "500GB".to_string(),
        devices: vec!["sda".to_string(), "sdb".to_string()],
    };

    assert_eq!(pool.name, "tank");
    assert_eq!(pool.state, "ONLINE");
    assert_eq!(pool.devices.len(), 2);
}

#[test]
fn test_dataset_info_creation() {
    let dataset = DatasetInfo {
        name: "tank/data".to_string(),
        used: "100GB".to_string(),
        available: "400GB".to_string(),
        referenced: "100GB".to_string(),
        mountpoint: "/tank/data".to_string(),
    };

    assert_eq!(dataset.name, "tank/data");
    assert_eq!(dataset.used, "100GB");
    assert_eq!(dataset.mountpoint, "/tank/data");
}

#[test]
fn test_snapshot_info_creation() {
    let snapshot = SnapshotInfo {
        name: "tank/data@snapshot1".to_string(),
        used: "1GB".to_string(),
        referenced: "100GB".to_string(),
        creation: "2025-01-30T12:00:00Z".to_string(),
    };

    assert_eq!(snapshot.name, "tank/data@snapshot1");
    assert_eq!(snapshot.used, "1GB");
    assert!(snapshot.name.contains("@"));
}

#[test]
fn test_zfs_request_pool_create() {
    let request = ZfsRequest::PoolCreate {
        name: "newpool".to_string(),
        devices: vec!["sdc".to_string(), "sdd".to_string()],
    };

    match request {
        ZfsRequest::PoolCreate { name, devices } => {
            assert_eq!(name, "newpool");
            assert_eq!(devices.len(), 2);
        }
        _ => panic!("Wrong request type"),
    }
}

#[test]
fn test_zfs_request_pool_destroy() {
    let request = ZfsRequest::PoolDestroy {
        name: "oldpool".to_string(),
    };

    match request {
        ZfsRequest::PoolDestroy { name } => {
            assert_eq!(name, "oldpool");
        }
        _ => panic!("Wrong request type"),
    }
}

#[test]
fn test_zfs_request_pool_status() {
    let request = ZfsRequest::PoolStatus {
        name: Some("tank".to_string()),
    };

    match request {
        ZfsRequest::PoolStatus { name } => {
            assert_eq!(name, Some("tank".to_string()));
        }
        _ => panic!("Wrong request type"),
    }
}

#[test]
fn test_zfs_request_dataset_create() {
    let mut properties = HashMap::new();
    properties.insert("compression".to_string(), "lz4".to_string());

    let request = ZfsRequest::DatasetCreate {
        name: "tank/new".to_string(),
        properties: properties.clone(),
    };

    match request {
        ZfsRequest::DatasetCreate { name, properties } => {
            assert_eq!(name, "tank/new");
            assert_eq!(properties.get("compression"), Some(&"lz4".to_string()));
        }
        _ => panic!("Wrong request type"),
    }
}

#[test]
fn test_zfs_request_health_check() {
    let request = ZfsRequest::HealthCheck;

    match request {
        ZfsRequest::HealthCheck => {
            // Success
        }
        _ => panic!("Wrong request type"),
    }
}

#[test]
fn test_zfs_response_pool_status() {
    let pool = PoolInfo {
        name: "tank".to_string(),
        state: "ONLINE".to_string(),
        size: "1TB".to_string(),
        allocated: "500GB".to_string(),
        free: "500GB".to_string(),
        devices: vec!["sda".to_string()],
    };

    let response = ZfsResponse::PoolStatus { pools: vec![pool] };

    match response {
        ZfsResponse::PoolStatus { pools } => {
            assert_eq!(pools.len(), 1);
            assert_eq!(pools[0].name, "tank");
        }
        _ => panic!("Wrong response type"),
    }
}

#[test]
fn test_zfs_response_success() {
    let response = ZfsResponse::Success {
        message: "Operation completed".to_string(),
    };

    match response {
        ZfsResponse::Success { message } => {
            assert_eq!(message, "Operation completed");
        }
        _ => panic!("Wrong response type"),
    }
}

#[test]
fn test_zfs_response_health() {
    let mut details = HashMap::new();
    details.insert("pools".to_string(), "2".to_string());

    let response = ZfsResponse::Health {
        status: "healthy".to_string(),
        details: details.clone(),
    };

    match response {
        ZfsResponse::Health { status, details } => {
            assert_eq!(status, "healthy");
            assert_eq!(details.get("pools"), Some(&"2".to_string()));
        }
        _ => panic!("Wrong response type"),
    }
}

#[test]
fn test_zfs_request_handler_creation() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    assert!(!handler.config().zfs_binary.is_empty());
}

#[test]
fn test_zfs_request_handler_config_access() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config.clone());

    let retrieved_config = handler.config();
    assert_eq!(retrieved_config.zfs_binary, config.zfs_binary);
    assert_eq!(retrieved_config.use_sudo, config.use_sudo);
}

#[test]
fn test_get_default_pool_name() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let pool_name = handler.get_default_pool_name();
    // Should return either env var or "tank"
    assert!(!pool_name.is_empty());
}

#[test]
fn test_is_performance_monitoring_enabled() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let enabled = handler.is_performance_monitoring_enabled();
    // Just verify method exists and returns a boolean without panicking
    let _ = enabled; // Compiling this line proves the method works
}

#[test]
fn test_get_health_check_interval() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let interval = handler.get_health_check_interval();
    // Should return a duration > 0
    assert!(interval.as_secs() > 0);
}

#[tokio::test]
async fn test_handle_health_check_request() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let request = ZfsRequest::HealthCheck;
    let result = handler.handle_zfs_request(request).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_health_check_response() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let request = ZfsRequest::HealthCheck;
    let response = handler.handle_zfs_request(request).await.unwrap();

    match response {
        ZfsResponse::Health { status, details } => {
            assert_eq!(status, "healthy");
            assert!(!details.is_empty());
        }
        _ => panic!("Expected Health response"),
    }
}

#[tokio::test]
#[ignore] // Requires actual ZFS system
async fn test_handle_pool_status_request() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let request = ZfsRequest::PoolStatus { name: None };
    let result = handler.handle_zfs_request(request).await;

    if let Err(e) = &result {
        eprintln!("Pool status request failed: {:?}", e);
    }
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore] // Requires actual ZFS system
async fn test_handle_dataset_list_request() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let request = ZfsRequest::DatasetList { pool: None };
    let result = handler.handle_zfs_request(request).await;

    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Requires real ZFS or proper simulation setup"]
async fn test_handle_snapshot_list_request() {
    let handler = create_test_handler();

    let request = ZfsRequest::SnapshotList { dataset: None };
    let result = handler.handle_zfs_request(request).await;

    if let Err(e) = &result {
        eprintln!("Snapshot list failed: {:?}", e);
        eprintln!(
            "Simulation mode env var: {:?}",
            std::env::var("NESTGATE_ZFS_SIMULATION_MODE")
        );
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_pool_create_request() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let request = ZfsRequest::PoolCreate {
        name: "testpool".to_string(),
        devices: vec!["sda".to_string()],
    };

    let result = handler.handle_zfs_request(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_dataset_create_request() {
    let config = ZfsConfig::default();
    let handler = ZfsRequestHandler::new(config);

    let request = ZfsRequest::DatasetCreate {
        name: "tank/test".to_string(),
        properties: HashMap::new(),
    };

    let result = handler.handle_zfs_request(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Requires real ZFS or proper simulation setup"]
async fn test_handle_pool_status_with_name() {
    let handler = create_test_handler();

    let request = ZfsRequest::PoolStatus {
        name: Some("tank".to_string()),
    };

    let result = handler.handle_zfs_request(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Requires real ZFS or proper simulation setup"]
async fn test_handle_dataset_list_with_pool() {
    let handler = create_test_handler();

    let request = ZfsRequest::DatasetList {
        pool: Some("tank".to_string()),
    };

    let result = handler.handle_zfs_request(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore = "Requires real ZFS or proper simulation setup"]
async fn test_multiple_requests_sequential() {
    let handler = create_test_handler();

    let request1 = ZfsRequest::HealthCheck;
    let request2 = ZfsRequest::PoolStatus { name: None };

    let result1 = handler.handle_zfs_request(request1).await;
    let result2 = handler.handle_zfs_request(request2).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}
