//! **PRODUCTION ZFS HANDLERS**
//!
//! Real ZFS operations using `nestgate-zfs` crate for production deployments.
//! These handlers execute actual ZFS commands and provide real functionality.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use nestgate_core::{NestGateError, Result};
use nestgate_zfs::{ProductionZfsManager, ZfsConfig, ZfsError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Shared ZFS manager state
pub type ZfsManagerState = Arc<RwLock<ProductionZfsManager>>;

/// Create ZFS manager from configuration
///
/// # Errors
///
/// Returns error if ZFS is not available or configuration is invalid
pub async fn create_zfs_manager(config: ZfsConfig) -> Result<ProductionZfsManager> {
    // Verify ZFS is available on the system
    if !nestgate_zfs::native::is_zfs_available().await {
        return Err(NestGateError::storage_error(
            "ZFS is not available on this system. Please install ZFS first.",
        ));
    }

    Ok(ProductionZfsManager::new(config))
}

/// List all ZFS pools
///
/// # Errors
///
/// Returns error if pool listing fails
pub async fn list_universal_pools(
    State(manager): State<ZfsManagerState>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let manager = manager.read().await;

    match manager.list_pools() {
        Ok(pools) => {
            info!("Successfully listed {} ZFS pools", pools.len());
            Ok(Json(json!({
                "status": "success",
                "pools": pools,
                "count": pools.len()
            })))
        }
        Err(e) => {
            error!("Failed to list ZFS pools: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to list ZFS pools",
                    "details": e.to_string()
                })),
            ))
        }
    }
}

/// Create a new ZFS pool
#[derive(Debug, Deserialize)]
pub struct CreatePoolRequest {
    pub name: String,
    pub devices: Vec<String>,
    #[serde(default)]
    pub raid_type: String,
}

pub async fn create_pool(
    State(manager): State<ZfsManagerState>,
    Json(request): Json<CreatePoolRequest>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate request
    if request.name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Pool name cannot be empty"
            })),
        ));
    }

    if request.devices.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "At least one device must be specified"
            })),
        ));
    }

    let manager = manager.read().await;

    match manager.create_pool(&request.name, &request.devices) {
        Ok(pool_info) => {
            info!("Successfully created ZFS pool: {}", request.name);
            Ok(Json(json!({
                "status": "success",
                "message": format!("Pool '{}' created successfully", request.name),
                "pool": pool_info
            })))
        }
        Err(e) => {
            error!("Failed to create ZFS pool '{}': {}", request.name, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create ZFS pool",
                    "details": e.to_string()
                })),
            ))
        }
    }
}

/// Get details about a specific pool
pub async fn get_universal_pool(
    State(manager): State<ZfsManagerState>,
    Path(pool_name): Path<String>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let manager = manager.read().await;

    match manager.get_pool_status(&pool_name) {
        Ok(status) => {
            info!("Retrieved status for pool: {}", pool_name);
            Ok(Json(json!({
                "status": "success",
                "pool": pool_name,
                "pool_status": status
            })))
        }
        Err(e) => {
            warn!("Pool '{}' not found or inaccessible: {}", pool_name, e);
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": format!("Pool '{}' not found", pool_name),
                    "details": e.to_string()
                })),
            ))
        }
    }
}

/// Delete a ZFS pool
pub async fn delete_pool(
    State(_manager): State<ZfsManagerState>,
    Path(pool_name): Path<String>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Pool destruction is a dangerous operation - require explicit confirmation
    warn!(
        "Pool destruction requested for '{}' - not implemented for safety",
        pool_name
    );
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Pool destruction not implemented",
            "message": "For safety, pool destruction must be done via CLI",
            "recommendation": "Use: sudo zpool destroy <pool_name>"
        })),
    ))
}

/// List datasets in a pool
pub async fn list_datasets(
    State(manager): State<ZfsManagerState>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let manager = manager.read().await;

    // List datasets from all pools
    match manager.list_pools() {
        Ok(pools) => {
            let mut all_datasets = Vec::new();
            for pool in pools {
                if let Ok(datasets) = manager.list_datasets(&pool.name) {
                    all_datasets.extend(datasets);
                }
            }

            info!("Listed {} datasets across all pools", all_datasets.len());
            Ok(Json(json!({
                "status": "success",
                "datasets": all_datasets,
                "count": all_datasets.len()
            })))
        }
        Err(e) => {
            error!("Failed to list datasets: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to list datasets",
                    "details": e.to_string()
                })),
            ))
        }
    }
}

/// Create a new dataset
#[derive(Debug, Deserialize)]
pub struct CreateDatasetRequest {
    pub pool: String,
    pub name: String,
    #[serde(default)]
    pub tier: nestgate_core::canonical_types::StorageTier,
}

pub async fn create_dataset(
    State(manager): State<ZfsManagerState>,
    Json(request): Json<CreateDatasetRequest>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let manager = manager.read().await;

    match manager.create_dataset_with_tier(&request.pool, &request.name, request.tier) {
        Ok(dataset_info) => {
            info!("Created dataset: {}/{}", request.pool, request.name);
            Ok(Json(json!({
                "status": "success",
                "message": format!("Dataset '{}/{}' created", request.pool, request.name),
                "dataset": dataset_info
            })))
        }
        Err(e) => {
            error!("Failed to create dataset: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to create dataset",
                    "details": e.to_string()
                })),
            ))
        }
    }
}

/// Get dataset information
pub async fn get_dataset(
    State(manager): State<ZfsManagerState>,
    Path(dataset_path): Path<String>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let manager = manager.read().await;

    // Parse pool and dataset from path
    let parts: Vec<&str> = dataset_path.split('/').collect();
    if parts.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid dataset path"})),
        ));
    }

    let pool = parts[0];
    match manager.list_datasets(pool) {
        Ok(datasets) => {
            if let Some(dataset) = datasets.iter().find(|d| d.name == dataset_path) {
                Ok(Json(json!({
                    "status": "success",
                    "dataset": dataset
                })))
            } else {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "error": format!("Dataset '{}' not found", dataset_path)
                    })),
                ))
            }
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to retrieve dataset",
                "details": e.to_string()
            })),
        )),
    }
}

/// Delete a dataset
pub async fn delete_dataset(
    State(_manager): State<ZfsManagerState>,
    Path(dataset_path): Path<String>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    warn!(
        "Dataset destruction requested for '{}' - not implemented for safety",
        dataset_path
    );
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Dataset destruction not implemented",
            "message": "For safety, dataset destruction must be done via CLI",
            "recommendation": "Use: sudo zfs destroy <dataset>"
        })),
    ))
}

/// List snapshots
pub async fn list_snapshots(
    State(manager): State<ZfsManagerState>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let manager = manager.read().await;

    // List snapshots from all datasets
    let mut all_snapshots = Vec::new();
    if let Ok(pools) = manager.list_pools() {
        for pool in pools {
            if let Ok(datasets) = manager.list_datasets(&pool.name) {
                for dataset in datasets {
                    if let Ok(snapshots) = manager.list_snapshots(&dataset.name) {
                        all_snapshots.extend(snapshots);
                    }
                }
            }
        }
    }

    Ok(Json(json!({
        "status": "success",
        "snapshots": all_snapshots,
        "count": all_snapshots.len()
    })))
}

/// Create a snapshot
#[derive(Debug, Deserialize)]
pub struct CreateSnapshotRequest {
    pub dataset: String,
    pub name: String,
}

pub async fn create_snapshot(
    State(manager): State<ZfsManagerState>,
    Json(request): Json<CreateSnapshotRequest>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let manager = manager.read().await;

    match manager.create_snapshot(&request.dataset, &request.name) {
        Ok(snapshot_info) => {
            info!("Created snapshot: {}@{}", request.dataset, request.name);
            Ok(Json(json!({
                "status": "success",
                "snapshot": snapshot_info
            })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to create snapshot",
                "details": e.to_string()
            })),
        )),
    }
}

/// Delete a snapshot
pub async fn delete_snapshot(
    State(_manager): State<ZfsManagerState>,
    Path(snapshot_path): Path<String>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    warn!(
        "Snapshot destruction requested for '{}' - not implemented for safety",
        snapshot_path
    );
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Snapshot destruction not implemented",
            "message": "For safety, snapshot destruction must be done via CLI",
            "recommendation": "Use: sudo zfs destroy <snapshot>"
        })),
    ))
}

/// Get overall ZFS health status
pub async fn get_zfs_health(
    State(manager): State<ZfsManagerState>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let manager = manager.read().await;

    match manager.list_pools() {
        Ok(pools) => {
            let total_pools = pools.len();
            let online_pools = pools.iter().filter(|p| p.status == "ONLINE").count();
            let degraded_pools = pools.iter().filter(|p| p.status == "DEGRADED").count();

            let health_status = if degraded_pools > 0 {
                "degraded"
            } else if online_pools == total_pools {
                "healthy"
            } else {
                "warning"
            };

            Ok(Json(json!({
                "status": health_status,
                "total_pools": total_pools,
                "online_pools": online_pools,
                "degraded_pools": degraded_pools,
                "pools": pools
            })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to check ZFS health",
                "details": e.to_string()
            })),
        )),
    }
}

// Placeholder implementations for advanced features

pub async fn get_universal_storage_health() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Universal storage health not yet implemented",
            "message": "Use /zfs/health endpoint for ZFS-specific health"
        })),
    )
}

pub async fn get_pool_status(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Pool status endpoint moved",
            "message": "Use /zfs/pools/{pool_name} endpoint instead"
        })),
    )
}

pub async fn get_performance_analytics() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Performance analytics not yet implemented",
            "message": "Feature scheduled for future release"
        })),
    )
}

pub async fn predict_tier(
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Tier prediction not yet implemented",
            "message": "ML-based tier prediction scheduled for future release"
        })),
    )
}

pub async fn trigger_optimization(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Pool optimization not yet implemented",
            "message": "Automatic optimization scheduled for future release"
        })),
    )
}

pub async fn get_dataset_properties(_path: Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Dataset properties endpoint not yet implemented",
            "message": "Use CLI: zfs get all <dataset>"
        })),
    )
}

pub async fn set_dataset_properties(
    _path: Path<String>,
    _body: Json<HashMap<String, serde_json::Value>>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "Dataset properties modification not yet implemented",
            "message": "Use CLI: zfs set <property>=<value> <dataset>"
        })),
    )
}
