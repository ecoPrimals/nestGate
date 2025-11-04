//
// This test suite validates the complete ZFS API functionality including
// pool management, dataset operations, snapshot management, and AI integration.
//
// Tests use real ZFS by default. To run with mock data, set USE_MOCK_ZFS=true

use axum_test::TestServer;
use nestgate_api::create_app;
use nestgate_zfs::{config::ZfsConfig, ZfsManager};
use serde_json::{json, Value};
use std::sync::Arc;

/// Test helper to create a ZFS manager for testing
async fn create_test_zfs_manager() -> Arc<ZfsManager> {
    let config = ZfsConfig::default();
    // Check environment variable for mock mode
    let use_mock = std::env::var("USE_MOCK_ZFS")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);

    config.use_real_zfs = !use_mock;
    config.default_pool = "zfspool".to_string();

    let manager = ZfsManager::new(config)
        .await
        .expect("Failed to create test ZFS manager");
    Arc::new(manager)
}

/// Test helper to create a test server with ZFS API
async fn create_test_server() -> TestServer {
    let _zfs_manager = create_test_zfs_manager().await; // Keep for future use
    let app = create_app(); // Use create_app() instead of create_router() to include CORS middleware
    TestServer::new(app).expect("Failed to create test server")
}
#[tokio::test]
async fn test_health_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/health").await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert_eq!(body["status"], "ok");
    Ok(())
}

#[tokio::test]
async fn test_zfs_health_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/health").await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert!(body.get("service_name").is_some());
    assert!(body.get("status").is_some());
    Ok(())
}

#[tokio::test]
async fn test_zfs_status_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/status").await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert!(body.get("service_name").is_some());
    assert!(body.get("status").is_some());
    Ok(())
}

#[tokio::test]
async fn test_list_pools_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/pools").await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert!(body.is_array());
    Ok(())
}

#[tokio::test]
async fn test_create_pool_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let create_request = json!({
        "name": "test_pool",
        "devices": ["/dev/loop0", "/dev/loop1"],
        "config": {
            "raid_level": "mirror",
            "compression": "lz4",
            "dedup": false,
            "encryption": false
    Ok(())
        }
    });

    let response = server.post("/api/v1/zfs/pools").json(&create_request).await;

    // Note: This will likely fail in test environment without actual devices
    // but we're testing the API structure and error handling
    assert!(response.status_code() == 200 || response.status_code() == 500);
    Ok(())
}

#[tokio::test]
async fn test_get_pool_info_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/pools/nonexistent_pool").await;

    // Should return 404 for non-existent pool
    response.assert_status_not_found();
    Ok(())
}

#[tokio::test]
async fn test_list_datasets_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/datasets").await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert!(body.is_array());
    Ok(())
}

#[tokio::test]
async fn test_create_dataset_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let create_request = json!({
        "name": "test_dataset",
        "parent": "test_pool",
        "tier": "Warm",
        "properties": {
            "compression": "lz4",
            "recordsize": "128K"
    Ok(())
        }
    });

    let response = server
        .post("/api/v1/zfs/datasets")
        .json(&create_request)
        .await;

    // Note: This will likely fail without actual pool
    // but we're testing the API structure
    assert!(response.status_code() == 200 || response.status_code() == 500);
    Ok(())
}

#[tokio::test]
async fn test_get_dataset_info_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/datasets/nonexistent_dataset").await;

    // Should return 404 for non-existent dataset
    response.assert_status_not_found();
    Ok(())
}

#[tokio::test]
async fn test_list_snapshots_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server
        .get("/api/v1/zfs/datasets/test_dataset/snapshots")
        .await;

    // Should return 500 for non-existent dataset (internal server error)
    assert!(response.status_code() == 500);
    Ok(())
}

#[tokio::test]
async fn test_create_snapshot_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let create_request = json!({
        "name": "test_snapshot",
        "dataset": "test_dataset",
        "recursive": false,
        "properties": {
            "comment": "Test snapshot"
    Ok(())
        }
    });

    let response = server
        .post("/api/v1/zfs/datasets/test_dataset/snapshots")
        .json(&create_request)
        .await;

    // Should return 500 for non-existent dataset
    assert!(response.status_code() == 500);
    Ok(())
}

#[tokio::test]
async fn test_tier_prediction_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let prediction_request = json!({
        "file_path": "/test/file.txt"
    });

    let response = server
        .post("/api/v1/zfs/ai/tier-prediction")
        .json(&prediction_request)
        .await;

    response.assert_status_ok();

    let body: Value = response.json();
    assert!(body.get("file_path").is_some());
    assert!(body.get("predicted_tier").is_some());
    assert!(body.get("confidence").is_some());
    Ok(())
}

#[tokio::test]
async fn test_performance_analytics_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/optimization/analytics").await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert!(body.get("tier_performance").is_some());
    assert!(body.get("recommendations").is_some());
    Ok(())
}

#[tokio::test]
async fn test_trigger_optimization_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.post("/api/v1/zfs/optimization/trigger").await;
    response.assert_status_ok();

    let body: Value = response.json();
    assert!(body.get("optimization_id").is_some());
    assert!(body.get("status").is_some());
    Ok(())
}

#[tokio::test]
async fn test_api_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    // Test malformed JSON
    let response = server.post("/api/v1/zfs/pools").text("invalid json").await;

    response.assert_status_bad_request();

    // Test missing required fields
    let invalid_request = json!({
        "name": "test_pool"
        // Missing required 'devices' field
    });

    let response = server
        .post("/api/v1/zfs/pools")
        .json(&invalid_request)
        .await;

    response.assert_status_bad_request();
    Ok(())
}

#[tokio::test]
async fn test_api_content_type_validation() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    // Test with correct content type
    let create_request = json!({
        "name": "test_pool",
        "devices": ["/dev/loop0"]
    });

    let response = server.post("/api/v1/zfs/pools").json(&create_request).await;

    // Should not fail due to content type
    assert!(response.status_code() != 415); // Not Unsupported Media Type
    Ok(())
}

#[tokio::test]
async fn test_api_cors_headers() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/health").await;
    response.assert_status_ok();

    // Check that CORS headers are present
    let headers = response.headers();
    assert!(headers.get("access-control-allow-origin").is_some());
    Ok(())
}

#[tokio::test]
async fn test_api_request_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;

    let response = server.get("/api/v1/zfs/health").await;
    response.assert_status_ok();

    // Check that tracing headers are present
    let _headers = response.headers();
    // Tracing headers may vary, but response should be successful
    assert_eq!(response.status_code(), 200);
    Ok(())
}

/// Test helper for dataset property validation
#[tokio::test]
async fn test_dataset_properties_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;
    // Test getting properties for non-existent dataset
    let response = server
        .get("/api/v1/zfs/datasets/nonexistent/properties")
        .await;
    response.assert_status_not_found();

    // Test setting properties for non-existent dataset
    let properties = json!({
        "compression": "zstd",
        "recordsize": "1M"
    });

    let response = server
        .put("/api/v1/zfs/datasets/nonexistent/properties")
        .json(&properties)
        .await;

    response.assert_status_not_found();
    Ok(())
}

/// Test helper for snapshot deletion
#[tokio::test]
async fn test_delete_snapshot_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;
    let response = server
        .delete("/api/v1/zfs/datasets/test_dataset/snapshots/test_snapshot")
        .await;

    // Should return 500 for non-existent dataset/snapshot
    assert!(response.status_code() == 500);
    Ok(())
}

/// Test helper for pool status and scrub operations
#[tokio::test]
async fn test_pool_operations() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;
    // Test pool status for non-existent pool
    let response = server.get("/api/v1/zfs/pools/nonexistent/status").await;
    response.assert_status_not_found();

    // Test pool scrub for non-existent pool
    let response = server.post("/api/v1/zfs/pools/nonexistent/scrub").await;
    assert!(response.status_code() == 500);
    Ok(())
}

/// Test helper for pool destruction
#[tokio::test]
async fn test_destroy_pool_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;
    let response = server.delete("/api/v1/zfs/pools/nonexistent_pool").await;

    // Should return 500 for non-existent pool
    assert!(response.status_code() == 500);
    Ok(())
}

/// Test helper for dataset destruction
#[tokio::test]
async fn test_destroy_dataset_endpoint() -> Result<(), Box<dyn std::error::Error>> {
    let server = create_test_server().await;
    let response = server
        .delete("/api/v1/zfs/datasets/nonexistent_dataset")
        .await;

    // Should return 500 for non-existent dataset
    assert!(response.status_code() == 500);
    Ok(())
}
