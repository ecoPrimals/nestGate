//! Universal ZFS Pools API - Storage Agnostic
//!
//! This handler provides ZFS pool API functionality that works with ANY storage backend,
//! not just ZFS. It uses the Universal Storage Bridge to translate ZFS concepts to
//! universal storage operations.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::routes::AppState;

/// List all storage pools (ZFS pools if available, filesystem mounts otherwise)
pub async fn list_universal_pools(State(state): State<AppState>) -> impl IntoResponse {
    info!("🔍 Listing universal storage pools");

    // Try to get the Universal Storage Bridge
    match &state.universal_storage_bridge {
        Some(bridge) => {
            // Use the Universal Storage Bridge to list pools
            match bridge.list_pools().await {
                Ok(pools) => {
                    info!(
                        "✅ Found {} storage pools via Universal Storage Bridge",
                        pools.len()
                    );

                    // Convert to JSON response
                    let pool_data: Vec<serde_json::Value> = pools
                        .into_iter()
                        .map(|pool| {
                            json!({
                                "name": pool.name,
                                "state": format!("{:?}", pool.state),
                                "health": format!("{:?}", pool.health),
                                "capacity": {
                                    "total_bytes": pool.capacity.total_bytes,
                                    "used_bytes": pool.capacity.used_bytes,
                                    "available_bytes": pool.capacity.available_bytes,
                                    "utilization_percent": pool.capacity.utilization_percent
                                },
                                "properties": pool.properties,
                                "created_at": pool.created_at
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs()
                            })
                        })
                        .collect();

                    (
                        StatusCode::OK,
                        Json(json!({
                            "success": true,
                            "data": pool_data,
                            "count": pool_data.len(),
                            "storage_backend": "universal",
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        })),
                    )
                }
                Err(e) => {
                    error!("❌ Universal Storage Bridge failed to list pools: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "success": false,
                            "error": format!("Failed to list storage pools: {}", e),
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        })),
                    )
                }
            }
        }
        None => {
            // Fallback: Try the traditional ZFS manager
            match &state.zfs_manager {
                Some(zfs_manager) => {
                    info!("🔄 Falling back to ZFS manager");
                    match zfs_manager.pool_manager.list_pools().await {
                        Ok(pools) => {
                            info!("✅ Found {} ZFS pools via ZFS manager", pools.len());
                            (
                                StatusCode::OK,
                                Json(json!({
                                    "success": true,
                                    "data": pools,
                                    "count": pools.len(),
                                    "storage_backend": "zfs",
                                    "timestamp": chrono::Utc::now().to_rfc3339()
                                })),
                            )
                        }
                        Err(e) => {
                            error!("❌ ZFS manager failed to list pools: {:?}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({
                                    "success": false,
                                    "error": format!("ZFS pools unavailable: {}", e),
                                    "timestamp": chrono::Utc::now().to_rfc3339()
                                })),
                            )
                        }
                    }
                }
                None => {
                    warn!("⚠️ No storage backends available");
                    (
                        StatusCode::SERVICE_UNAVAILABLE,
                        Json(json!({
                            "success": false,
                            "error": "No storage backends available - neither Universal Storage Bridge nor ZFS manager initialized",
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        })),
                    )
                }
            }
        }
    }
}

/// Get information about a specific storage pool
pub async fn get_universal_pool(
    State(state): State<AppState>,
    Path(pool_name): Path<String>,
) -> impl IntoResponse {
    info!("🔍 Getting universal storage pool: {}", pool_name);

    match &state.universal_storage_bridge {
        Some(bridge) => {
            // Get all pools and find the requested one
            match bridge.list_pools().await {
                Ok(pools) => {
                    if let Some(pool) = pools.into_iter().find(|p| p.name == pool_name) {
                        info!("✅ Found storage pool: {}", pool_name);
                        (
                            StatusCode::OK,
                            Json(json!({
                                "success": true,
                                "data": {
                                    "name": pool.name,
                                    "state": format!("{:?}", pool.state),
                                    "health": format!("{:?}", pool.health),
                                    "capacity": {
                                        "total_bytes": pool.capacity.total_bytes,
                                        "used_bytes": pool.capacity.used_bytes,
                                        "available_bytes": pool.capacity.available_bytes,
                                        "utilization_percent": pool.capacity.utilization_percent
                                    },
                                    "properties": pool.properties,
                                    "devices": pool.devices,
                                    "errors": pool.errors,
                                    "created_at": pool.created_at
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap_or_default()
                                        .as_secs(),
                                    "last_scrub": pool.last_scrub
                                },
                                "storage_backend": "universal",
                                "timestamp": chrono::Utc::now().to_rfc3339()
                            })),
                        )
                    } else {
                        warn!("⚠️ Storage pool not found: {}", pool_name);
                        (
                            StatusCode::NOT_FOUND,
                            Json(json!({
                                "success": false,
                                "error": format!("Storage pool '{}' not found", pool_name),
                                "timestamp": chrono::Utc::now().to_rfc3339()
                            })),
                        )
                    }
                }
                Err(e) => {
                    error!("❌ Failed to list pools to find '{}': {}", pool_name, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "success": false,
                            "error": format!("Failed to access storage pools: {}", e),
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        })),
                    )
                }
            }
        }
        None => {
            warn!(
                "⚠️ Universal Storage Bridge not available for pool: {}",
                pool_name
            );
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({
                    "success": false,
                    "error": "Universal Storage Bridge not available",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                })),
            )
        }
    }
}

/// Get health status of the universal storage system
pub async fn get_universal_storage_health(State(state): State<AppState>) -> impl IntoResponse {
    info!("🏥 Checking universal storage health");

    let mut health_info = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "universal_storage_bridge": false,
        "zfs_manager": false,
        "active_backend": "none",
        "status": "unhealthy"
    });

    let mut is_healthy = false;

    // Check Universal Storage Bridge
    if let Some(bridge) = &state.universal_storage_bridge {
        health_info["universal_storage_bridge"] = json!(true);

        // Try to detect the active backend
        let mut bridge_clone = bridge.as_ref().clone();
        match bridge_clone.detect_best_backend().await {
            Ok(backend) => {
                health_info["active_backend"] = json!(backend);
                is_healthy = true;
                info!(
                    "✅ Universal Storage Bridge healthy with backend: {}",
                    backend
                );
            }
            Err(e) => {
                warn!(
                    "⚠️ Universal Storage Bridge backend detection failed: {}",
                    e
                );
                health_info["bridge_error"] = json!(format!("{}", e));
            }
        }
    }

    // Check ZFS Manager
    if let Some(_zfs_manager) = &state.zfs_manager {
        health_info["zfs_manager"] = json!(true);
        if !is_healthy {
            // Only use ZFS as fallback if Universal Storage Bridge isn't working
            health_info["active_backend"] = json!("zfs");
            is_healthy = true;
        }
    }

    health_info["status"] = json!(if is_healthy { "healthy" } else { "unhealthy" });

    let status_code = if is_healthy {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(health_info))
}
