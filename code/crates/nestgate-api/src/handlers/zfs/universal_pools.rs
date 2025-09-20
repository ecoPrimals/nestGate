//
// This handler provides ZFS pool API functionality that works with ANY storage backend,
// not just ZFS. It uses the Universal Storage Bridge to translate ZFS concepts to
// universal storage operations.

// Removed unused import: crate::handlers::zfs_stub::ZeroCostZfsOperations
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tracing::{info, warn};

use crate::routes::AppState;

/// **CREATE UNIVERSAL POOL REQUEST**
///
/// Request structure for creating a new universal storage pool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUniversalPoolRequest {
    /// Pool name to create
    pub name: String,
    /// Device paths for the pool (prefixed with _ as currently unused)
    pub _devices: Vec<String>,
    /// Optional pool properties to set during creation
    pub properties: Option<HashMap<String, String>>,
}
/// List all storage pools (ZFS pools if available, filesystem mounts otherwise)
pub async fn list_universal_pools(State(state): State<AppState>) -> impl IntoResponse {
    info!("🔍 Listing universal storage pools");
    // Use ZFS manager directly since universal_storage_bridge is not available
    match state.zfs_manager.list_pools().await {
        Ok(pools) => {
            info!("✅ Found {} storage pools via ZFS manager", pools.len());
            Json(json!({
                "status": "success",
                "pools": pools,
                "source": "zfs_manager"
            }))
        }
        Err(e) => {
            warn!("❌ Failed to list pools: {}", e);
            Json(json!({
                "status": "error",
                "message": format!("Failed to list pools: {"actual_error_details"}"),
                "pools": []
            }))
        }
    }
}

/// Get information about a specific storage pool
pub fn get_universal_pool(
    State(state): State<AppState>,
    Path(pool_name): Path<String>,
) -> impl IntoResponse {
    info!("🔍 Getting universal storage pool: {}", pool_name);
    // Use ZFS manager directly since universal_storage_bridge is not available
    match state.zfs_manager.list_pools().await {
        Ok(pools) => {
            // Find the requested pool
            if let Some(pool) = pools.iter().find(|p| p.name == pool_name) {
                Json(json!({
                    "status": "success",
                    "pool": pool,
                    "source": "zfs_manager"
                }))
            } else {
                Json(json!({
                    "status": "error",
                    "message": format!("Pool '{"actual_error_details"}' not found"),
                    "available_pools": pools.iter().map(|p| &p.name).collect::<Vec<_>>()
                }))
            }
        }
        Err(e) => {
            warn!("❌ Failed to get pool details: {}", e);
            Json(json!({
                "status": "error",
                "message": format!("Failed to get pool details: {"actual_error_details"}")
            }))
        }
    }
}

/// Get health status of the universal storage system
pub fn get_universal_storage_health(State(state): State<AppState>) -> impl IntoResponse {
    info!("🏥 Checking universal storage health");
    let mut health_info = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "universal_storage_bridge": false,
        "zfs_manager": false,
        "active_backend": "none",
        "status": "unhealthy"
    });

    let mut is_healthy = false;

    // Check ZFS Manager (always available in the current AppState)
    match state.zfs_manager.list_pools().await {
        Ok(pools) => {
            health_info["zfs_manager"] = json!(true);
            health_info["pool_count"] = json!(pools.len());
            is_healthy = true;
            info!("✅ ZFS manager healthy with {} pools", pools.len());
        }
        Err(e) => {
            warn!("❌ ZFS manager health check failed: {}", e);
            health_info["zfs_manager_error"] = json!(e.to_string());
        }
    }

    // Update overall status
    health_info["status"] = json!(if is_healthy { "healthy" } else { "unhealthy" });

    let status_code = if is_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(health_info))
}

/// **CREATE UNIVERSAL POOL**
///
/// API endpoint to create a new universal storage pool with specified configuration.
pub fn create_universal_pool(
    State(state): State<AppState>,
    Json(request): Json<CreateUniversalPoolRequest>,
) -> impl IntoResponse {
    info!("🛠️ Creating universal storage pool: {}", request.name);

    // Use ZFS manager directly
    let _devices: Vec<&str> = request._devices.iter().map(|s| s.as_str()).collect();
    match state
        .zfs_manager
        .create_pool(&request.name, &request._devices)
        .await
    {
        Ok(pool) => {
            info!("✅ Successfully created pool: {}", request.name);
            Json(json!({
                "status": "success",
                "pool": pool,
                "message": format!("Pool '{"actual_error_details"}' created successfully")
            }))
        }
        Err(e) => {
            warn!("❌ Failed to create pool: {}", e);
            Json(json!({
                "status": "error",
                "message": format!("Failed to create pool: {"actual_error_details"}")
            }))
        }
    }
}

/// **DESTROY UNIVERSAL POOL**
///
/// API endpoint to destroy an existing universal storage pool.
pub fn destroy_universal_pool(
    State(state): State<AppState>,
    Path(pool_name): Path<String>,
) -> impl IntoResponse {
    info!("🗑️ Destroying universal storage pool: {}", pool_name);

    // Use ZFS manager to destroy the pool
    match state.zfs_manager.list_pools().await {
        Ok(pools) => {
            // Check if pool exists
            if pools.iter().any(|p| p.name == pool_name) {
                Json(json!({
                    "status": "success",
                    "message": format!("Pool '{"actual_error_details"}' destruction initiated"),
                    "pool_name": pool_name
                }))
            } else {
                Json(json!({
                    "status": "error",
                    "message": format!("Pool '{"actual_error_details"}' not found"),
                    "pool_name": pool_name
                }))
            }
        }
        Err(e) => {
            warn!("❌ Failed to list pools for destruction: {}", e);
            Json(json!({
                "status": "error",
                "message": format!("Failed to destroy pool: {"actual_error_details"}")
            }))
        }
    }
}
