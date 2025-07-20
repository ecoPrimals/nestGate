//! Universal ZFS API Handlers
//!
//! Provides universal, agnostic ZFS operations with proper abstraction,
//! dependency injection, and fail-safe mechanisms using the service factory pattern.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

use crate::handlers::zfs::types::{
    CreateDatasetRequest, CreatePoolRequest, CreateSnapshotRequest, TierPredictionRequest,
};
use crate::routes::AppState;
use nestgate_core::ResponseBuilder;

/// Get ZFS service instance
async fn get_zfs_service(state: &AppState) -> Result<Arc<nestgate_zfs::ZfsManager>, String> {
    state
        .get_zfs_manager()
        .ok_or_else(|| "ZFS service not available".to_string())
}

/// List all ZFS pools
pub async fn get_zfs_pools(State(state): State<AppState>) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.pool_manager.list_pools().await {
            Ok(pools) => {
                info!("Listed {} ZFS pools", pools.len());
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "data": pools
                    })),
                )
            }
            Err(e) => {
                error!("Failed to list pools: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to list pools: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Get a specific ZFS pool
pub async fn get_zfs_pool(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.pool_manager.get_pool_info(&name).await {
            Ok(pool) => {
                info!("Retrieved info for pool {}", name);
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "data": pool
                    })),
                )
            }
            Err(e) => {
                error!("Failed to get pool {}: {:?}", name, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to get pool: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Create a new ZFS pool
pub async fn create_zfs_pool(
    State(state): State<AppState>,
    Json(request): Json<CreatePoolRequest>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.create_pool(&request.name, &request.devices).await {
            Ok(pool) => {
                info!("Pool {} created successfully", request.name);
                (
                    StatusCode::CREATED,
                    Json(serde_json::json!({
                        "status": "success",
                        "data": pool,
                        "message": format!("Pool {} created successfully", request.name)
                    })),
                )
            }
            Err(e) => {
                error!("Failed to create pool {}: {:?}", request.name, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to create pool: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Delete a ZFS pool
pub async fn delete_zfs_pool(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.destroy_pool(&name).await {
            Ok(()) => {
                info!("Pool {} deleted successfully", name);
                (
                    StatusCode::OK,
                    ResponseBuilder::success_json(format!("Pool {name} deleted")),
                )
            }
            Err(e) => {
                error!("Failed to delete pool {}: {:?}", name, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to delete pool: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// List all ZFS datasets
pub async fn get_zfs_datasets(State(state): State<AppState>) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.dataset_manager.list_datasets().await {
            Ok(datasets) => {
                info!("Listed {} ZFS datasets", datasets.len());
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "data": datasets
                    })),
                )
            }
            Err(e) => {
                error!("Failed to list datasets: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to list datasets: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Get a specific ZFS dataset
pub async fn get_zfs_dataset(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            match service
                .dataset_manager
                .get_dataset_info_with_fallback(&name)
                .await
            {
                Ok(dataset) => {
                    info!("Retrieved info for dataset {}", name);
                    (
                        StatusCode::OK,
                        Json(serde_json::json!({
                            "status": "success",
                            "data": dataset
                        })),
                    )
                }
                Err(e) => {
                    error!("Failed to get dataset {}: {:?}", name, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!("Failed to get dataset: {e}")),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Create a new ZFS dataset
pub async fn create_zfs_dataset(
    State(state): State<AppState>,
    Json(request): Json<CreateDatasetRequest>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            match service
                .dataset_manager
                .create_dataset(
                    &request.name,
                    &request.parent,
                    nestgate_core::StorageTier::Hot,
                )
                .await
            {
                Ok(dataset) => {
                    info!("Dataset {} created successfully", request.name);
                    (
                        StatusCode::CREATED,
                        Json(serde_json::json!({
                            "status": "success",
                            "data": dataset,
                            "message": format!("Dataset {} created successfully", request.name)
                        })),
                    )
                }
                Err(e) => {
                    error!("Failed to create dataset {}: {:?}", request.name, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!("Failed to create dataset: {e}")),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Delete a ZFS dataset
pub async fn delete_zfs_dataset(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.dataset_manager.destroy_dataset(&name).await {
            Ok(()) => {
                info!("Dataset {} deleted successfully", name);
                (
                    StatusCode::OK,
                    ResponseBuilder::success_json(format!("Dataset {name} deleted")),
                )
            }
            Err(e) => {
                error!("Failed to delete dataset {}: {:?}", name, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to delete dataset: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// List all ZFS snapshots
pub async fn get_zfs_snapshots(State(state): State<AppState>) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            // List all snapshots for all datasets
            match service.snapshot_manager.list_snapshots("").await {
                Ok(snapshots) => {
                    info!("Listed {} ZFS snapshots", snapshots.len());
                    (
                        StatusCode::OK,
                        Json(serde_json::json!({
                            "status": "success",
                            "data": snapshots
                        })),
                    )
                }
                Err(e) => {
                    error!("Failed to list snapshots: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!("Failed to list snapshots: {e}")),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Create a new ZFS snapshot
pub async fn create_zfs_snapshot(
    State(state): State<AppState>,
    Json(request): Json<CreateSnapshotRequest>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            match service
                .snapshot_manager
                .create_snapshot(
                    &request.name,
                    &request.dataset,
                    request.recursive.unwrap_or(false),
                )
                .await
            {
                Ok(snapshot) => {
                    info!("Snapshot {} created successfully", request.name);
                    (
                        StatusCode::CREATED,
                        Json(serde_json::json!({
                            "status": "success",
                            "data": snapshot,
                            "message": format!("Snapshot {} created successfully", request.name)
                        })),
                    )
                }
                Err(e) => {
                    error!("Failed to create snapshot {}: {:?}", request.name, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!("Failed to create snapshot: {e}")),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Delete a ZFS snapshot
pub async fn delete_zfs_snapshot(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            // Parse dataset and snapshot from full name
            let parts: Vec<&str> = name.split('@').collect();
            if parts.len() == 2 {
                match service
                    .snapshot_manager
                    .delete_snapshot(parts[0], parts[1])
                    .await
                {
                    Ok(_) => {
                        info!("Snapshot {} deleted successfully", name);
                        (
                            StatusCode::OK,
                            ResponseBuilder::success_json(format!("Snapshot {name} deleted")),
                        )
                    }
                    Err(e) => {
                        error!("Failed to delete snapshot {}: {:?}", name, e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ResponseBuilder::error_json(format!("Failed to delete snapshot: {e}")),
                        )
                    }
                }
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    ResponseBuilder::error_json(
                        "Invalid snapshot name format. Expected dataset@snapshot".to_string(),
                    ),
                )
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Get ZFS health information
pub async fn get_zfs_health(State(state): State<AppState>) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            // Use the ZfsManager's get_zfs_health method
            match service.get_zfs_health().await {
                Ok(health) => {
                    info!("Retrieved ZFS health information");
                    (
                        StatusCode::OK,
                        Json(serde_json::json!({
                            "status": "success",
                            "data": health
                        })),
                    )
                }
                Err(e) => {
                    error!("Failed to get health information: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!("Failed to get health: {e}")),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Get ZFS status
pub async fn get_zfs_status(State(state): State<AppState>) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(_service) => {
            // Create a comprehensive status response
            let status = serde_json::json!({
                "service_name": "ZFS Manager",
                "version": "1.0.0",
                "status": "running",
                "uptime": chrono::Utc::now().to_rfc3339(),
                "pools_available": true,
                "datasets_available": true,
                "snapshots_available": true
            });

            info!("Retrieved ZFS status");
            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "status": "success",
                    "data": status
                })),
            )
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Get ZFS optimization analytics
pub async fn get_zfs_optimization_analytics(State(state): State<AppState>) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.get_performance_analytics().await {
            Ok(analytics) => {
                info!("Retrieved ZFS optimization analytics");
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "data": analytics
                    })),
                )
            }
            Err(e) => {
                error!("Failed to get optimization analytics: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to get analytics: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Trigger ZFS optimization
pub async fn trigger_zfs_optimization(State(state): State<AppState>) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.trigger_optimization().await {
            Ok(result) => {
                info!("ZFS optimization triggered successfully");
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "data": result,
                        "message": "Optimization triggered successfully"
                    })),
                )
            }
            Err(e) => {
                error!("Failed to trigger optimization: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to trigger optimization: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Predict ZFS tier for a file
pub async fn predict_zfs_tier(
    State(state): State<AppState>,
    Json(request): Json<TierPredictionRequest>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            // Use the AI tier optimization method
            match service.get_ai_tier_recommendation(&request.file_path).await {
                Ok(prediction_opt) => match prediction_opt {
                    Some(prediction) => {
                        info!("Predicted tier for file: {}", request.file_path);
                        (
                            StatusCode::OK,
                            Json(serde_json::json!({
                                "status": "success",
                                "data": {
                                    "file_path": request.file_path,
                                    "predicted_tier": prediction.recommended_tier.to_string(),
                                    "confidence": format!("{:?}", prediction.confidence),
                                    "reasoning": prediction.reasoning,
                                    "prediction_score": prediction.prediction_score,
                                    "alternative_tiers": prediction.alternative_tiers
                                }
                            })),
                        )
                    }
                    None => {
                        warn!(
                            "No tier prediction available for file: {}",
                            request.file_path
                        );
                        (
                            StatusCode::OK,
                            Json(serde_json::json!({
                                "status": "success",
                                "data": {
                                    "file_path": request.file_path,
                                    "predicted_tier": "unknown",
                                    "confidence": "low",
                                    "reasoning": "No prediction available",
                                    "prediction_score": 0.0,
                                    "alternative_tiers": []
                                }
                            })),
                        )
                    }
                },
                Err(e) => {
                    error!("Failed to predict tier for {}: {:?}", request.file_path, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!("Failed to predict tier: {e}")),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Get dataset properties
pub async fn get_dataset_properties(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.dataset_manager.get_dataset_properties(&name).await {
            Ok(properties) => {
                info!("Retrieved properties for dataset {}", name);
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "data": properties
                    })),
                )
            }
            Err(e) => {
                error!("Failed to get dataset properties for {}: {:?}", name, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to get dataset properties: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Set dataset properties
pub async fn set_dataset_properties(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(properties): Json<HashMap<String, String>>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            match service
                .dataset_manager
                .set_dataset_properties(&name, &properties)
                .await
            {
                Ok(()) => {
                    info!("Properties set for dataset {}", name);
                    (
                        StatusCode::OK,
                        ResponseBuilder::success_json(format!("Properties set for dataset {name}")),
                    )
                }
                Err(e) => {
                    error!("Failed to set dataset properties for {}: {:?}", name, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!(
                            "Failed to set dataset properties: {e}"
                        )),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Scrub a ZFS pool
pub async fn scrub_zfs_pool(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.pool_manager.scrub_pool(&name).await {
            Ok(()) => {
                info!("Pool {} scrub initiated", name);
                (
                    StatusCode::OK,
                    ResponseBuilder::success_json(format!("Scrub initiated for pool {name}")),
                )
            }
            Err(e) => {
                error!("Failed to scrub pool {}: {:?}", name, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to scrub pool: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// List dataset snapshots
pub async fn list_dataset_snapshots(
    State(state): State<AppState>,
    Path(dataset): Path<String>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => match service.snapshot_manager.list_snapshots(&dataset).await {
            Ok(snapshots) => {
                info!(
                    "Listed {} snapshots for dataset {}",
                    snapshots.len(),
                    dataset
                );
                (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "status": "success",
                        "data": snapshots
                    })),
                )
            }
            Err(e) => {
                error!("Failed to list snapshots for dataset {}: {:?}", dataset, e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseBuilder::error_json(format!("Failed to list snapshots: {e}")),
                )
            }
        },
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Delete a dataset snapshot
pub async fn delete_dataset_snapshot(
    State(state): State<AppState>,
    Path((dataset, snapshot)): Path<(String, String)>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            match service
                .snapshot_manager
                .delete_snapshot(&dataset, &snapshot)
                .await
            {
                Ok(_) => {
                    info!("Snapshot {}@{} deleted successfully", dataset, snapshot);
                    (
                        StatusCode::OK,
                        ResponseBuilder::success_json(format!(
                            "Snapshot {dataset}@{snapshot} deleted"
                        )),
                    )
                }
                Err(e) => {
                    error!(
                        "Failed to delete snapshot {}@{}: {:?}",
                        dataset, snapshot, e
                    );
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!("Failed to delete snapshot: {e}")),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}

/// Create a dataset snapshot
pub async fn create_dataset_snapshot(
    State(state): State<AppState>,
    Path(dataset): Path<String>,
    Json(request): Json<CreateSnapshotRequest>,
) -> impl IntoResponse {
    match get_zfs_service(&state).await {
        Ok(service) => {
            match service
                .snapshot_manager
                .create_snapshot(&request.name, &dataset, request.recursive.unwrap_or(false))
                .await
            {
                Ok(snapshot) => {
                    info!("Snapshot created for dataset {}", dataset);
                    (
                        StatusCode::CREATED,
                        Json(serde_json::json!({
                            "status": "success",
                            "data": snapshot,
                            "message": format!("Snapshot {} created for dataset {}", request.name, dataset)
                        })),
                    )
                }
                Err(e) => {
                    error!("Failed to create snapshot for dataset {}: {:?}", dataset, e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ResponseBuilder::error_json(format!("Failed to create snapshot: {e}")),
                    )
                }
            }
        }
        Err(e) => {
            error!("ZFS service error: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                ResponseBuilder::error_json(e),
            )
        }
    }
}
