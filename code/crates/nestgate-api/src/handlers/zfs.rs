//! ZFS API handlers for NestGate
//!
//! Provides HTTP endpoints for ZFS pool management, dataset operations,
//! snapshot management, and tier optimization.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use nestgate_core::StorageTier;
use nestgate_zfs::{
    dataset::DatasetInfo,
    manager::{EnhancedServiceStatus, ZfsManager},
    pool::PoolInfo,
    snapshot::SnapshotInfo,
};

/// ZFS API state container
#[derive(Clone, Debug)]
pub struct ZfsApiState {
    /// ZFS manager instance
    pub zfs_manager: Arc<ZfsManager>,
}

/// Pool creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePoolRequest {
    /// Pool name
    pub name: String,
    /// Device paths
    pub devices: Vec<String>,
    /// Pool configuration
    pub config: Option<PoolConfig>,
}

/// Pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// RAID level (mirror, raidz1, raidz2, raidz3)
    pub raid_level: Option<String>,
    /// Compression algorithm
    pub compression: Option<String>,
    /// Deduplication enabled
    pub dedup: Option<bool>,
    /// Encryption enabled
    pub encryption: Option<bool>,
}

/// Dataset creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDatasetRequest {
    /// Dataset name
    pub name: String,
    /// Parent pool or dataset
    pub parent: String,
    /// Storage tier
    pub tier: StorageTier,
    /// Dataset properties
    pub properties: Option<HashMap<String, String>>,
}

/// Snapshot creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    /// Snapshot name
    pub name: String,
    /// Dataset to snapshot
    pub dataset: String,
    /// Recursive snapshot
    pub recursive: Option<bool>,
    /// Snapshot properties
    pub properties: Option<HashMap<String, String>>,
}

/// Tier migration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMigrationRequest {
    /// Dataset path
    pub dataset_path: String,
    /// Source tier
    pub source_tier: StorageTier,
    /// Target tier
    pub target_tier: StorageTier,
    /// Migration priority
    pub priority: Option<u8>,
    /// Force migration even if not recommended
    pub force: Option<bool>,
}

/// Query parameters for listing operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQuery {
    /// Limit number of results
    pub limit: Option<usize>,
    /// Skip number of results
    pub skip: Option<usize>,
    /// Filter by status
    pub status: Option<String>,
    /// Filter by tier
    pub tier: Option<StorageTier>,
}

/// API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success status
    pub success: bool,
    /// Response data
    pub data: Option<T>,
    /// Error message
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    /// Create a successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create an error response
    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create an error response with no data (alias for error)
    pub fn error_empty(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Create ZFS API router
pub fn create_zfs_routes() -> Router<ZfsApiState> {
    Router::new()
        // Health and status endpoints
        .route("/health", get(get_zfs_health))
        .route("/status", get(get_zfs_status))

        // Pool management endpoints
        .route("/pools", get(list_pools))
        .route("/pools", post(create_pool))
        .route("/pools/:name", get(get_pool_info))
        .route("/pools/:name", delete(destroy_pool))
        .route("/pools/:name/status", get(get_pool_status))
        .route("/pools/:name/scrub", post(scrub_pool))

        // Dataset management endpoints
        .route("/datasets", get(list_datasets))
        .route("/datasets", post(create_dataset))
        .route("/datasets/:name", get(get_dataset_info))
        .route("/datasets/:name", delete(destroy_dataset))
        .route("/datasets/:name/properties", get(get_dataset_properties))
        .route("/datasets/:name/properties", put(set_dataset_properties))

        // Snapshot management endpoints
        .route("/datasets/:name/snapshots", get(list_snapshots))
        .route("/datasets/:name/snapshots", post(create_snapshot))
        .route("/datasets/:name/snapshots/:snapshot", delete(delete_snapshot))

        // AI and optimization endpoints
        .route("/ai/tier-prediction", post(get_tier_prediction))
        .route("/optimization/analytics", get(get_performance_analytics))
        .route("/optimization/trigger", post(trigger_optimization))
}

// Health and Status Endpoints

/// Get ZFS system health
pub async fn get_zfs_health(
    State(state): State<ZfsApiState>,
) -> Result<Json<ApiResponse<EnhancedServiceStatus>>, StatusCode> {
    debug!("Getting ZFS health status");

    match state.zfs_manager.get_zfs_health().await {
        Ok(health) => {
            info!("ZFS health check successful");
            Ok(Json(ApiResponse::success(health)))
        }
        Err(e) => {
            error!("ZFS health check failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get ZFS system status
pub async fn get_zfs_status(
    State(state): State<ZfsApiState>,
) -> Result<Json<ApiResponse<EnhancedServiceStatus>>, StatusCode> {
    debug!("Getting ZFS system status");

    match state.zfs_manager.get_service_status().await {
        Ok(status) => {
            info!("ZFS status retrieval successful");
            Ok(Json(ApiResponse::success(status)))
        }
        Err(e) => {
            error!("ZFS status retrieval failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Pool Management Endpoints

/// List all ZFS pools
pub async fn list_pools(
    State(state): State<ZfsApiState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<ApiResponse<Vec<PoolInfo>>>, StatusCode> {
    debug!("Listing ZFS pools with query: {:?}", query);

    match state.zfs_manager.pool_manager.list_pools().await {
        Ok(mut pools) => {
            // Apply query filters
            if let Some(limit) = query.limit {
                pools.truncate(limit);
            }

            info!("Listed {} ZFS pools", pools.len());
            Ok(Json(ApiResponse::success(pools)))
        }
        Err(e) => {
            error!("Failed to list pools: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new ZFS pool
pub async fn create_pool(
    State(state): State<ZfsApiState>,
    Json(request): Json<CreatePoolRequest>,
) -> Result<Json<ApiResponse<PoolInfo>>, StatusCode> {
    debug!("Creating ZFS pool: {:?}", request);

    match state
        .zfs_manager
        .pool_manager
        .create_pool(&request.name, &request.devices)
        .await
    {
        Ok(pool_info) => {
            info!("Successfully created ZFS pool: {}", request.name);
            Ok(Json(ApiResponse::success(pool_info)))
        }
        Err(e) => {
            error!("Failed to create pool {}: {}", request.name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get information about a specific pool
pub async fn get_pool_info(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
) -> Result<Json<ApiResponse<PoolInfo>>, StatusCode> {
    debug!("Getting info for pool: {}", name);

    match state.zfs_manager.pool_manager.get_pool_info(&name).await {
        Ok(pool_info) => {
            info!("Retrieved info for pool: {}", name);
            Ok(Json(ApiResponse::success(pool_info)))
        }
        Err(e) => {
            error!("Failed to get pool info for {}: {}", name, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Destroy a ZFS pool
pub async fn destroy_pool(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    debug!("Destroying pool: {}", name);

    match state.zfs_manager.pool_manager.destroy_pool(&name).await {
        Ok(()) => {
            info!("Successfully destroyed pool: {}", name);
            Ok(Json(ApiResponse::success(())))
        }
        Err(e) => {
            error!("Failed to destroy pool {}: {}", name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get detailed pool status
pub async fn get_pool_status(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
) -> Result<Json<PoolStatusResponse>, StatusCode> {
    debug!("Getting pool status for: {}", name);

    match state.zfs_manager.get_pool_status(&name).await {
        Ok(status) => {
            info!("Successfully retrieved pool status for: {}", name);
            Ok(Json(PoolStatusResponse { status }))
        }
        Err(e) => {
            error!("Failed to get pool status for {}: {}", name, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Start pool scrub operation
pub async fn scrub_pool(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
) -> Result<StatusCode, StatusCode> {
    info!("Starting scrub for pool: {}", name);

    match state.zfs_manager.scrub_pool(&name).await {
        Ok(_) => {
            info!("Successfully started scrub for pool: {}", name);
            Ok(StatusCode::ACCEPTED)
        }
        Err(e) => {
            error!("Failed to start scrub for pool {}: {}", name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Dataset Management Endpoints

/// List all datasets
pub async fn list_datasets(
    State(state): State<ZfsApiState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<ApiResponse<Vec<DatasetInfo>>>, StatusCode> {
    debug!("Listing datasets with query: {:?}", query);

    match state.zfs_manager.dataset_manager.list_datasets().await {
        Ok(mut datasets) => {
            // Apply tier filter if specified
            if let Some(tier) = query.tier {
                datasets.retain(|d| d.tier == tier);
            }

            // Apply limit
            if let Some(limit) = query.limit {
                datasets.truncate(limit);
            }

            info!("Listed {} datasets", datasets.len());
            Ok(Json(ApiResponse::success(datasets)))
        }
        Err(e) => {
            error!("Failed to list datasets: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new dataset
pub async fn create_dataset(
    State(state): State<ZfsApiState>,
    Json(request): Json<CreateDatasetRequest>,
) -> Result<Json<ApiResponse<DatasetInfo>>, StatusCode> {
    debug!("Creating dataset: {:?}", request);

    match state
        .zfs_manager
        .dataset_manager
        .create_dataset(&request.name, &request.parent, request.tier)
        .await
    {
        Ok(dataset_info) => {
            info!(
                "Successfully created dataset: {}/{}",
                request.parent, request.name
            );
            Ok(Json(ApiResponse::success(dataset_info)))
        }
        Err(e) => {
            error!(
                "Failed to create dataset {}/{}: {}",
                request.parent, request.name, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get dataset information
pub async fn get_dataset_info(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
) -> Result<Json<ApiResponse<DatasetInfo>>, StatusCode> {
    debug!("Getting dataset info: {}", name);

    match state
        .zfs_manager
        .dataset_manager
        .get_dataset_info_with_fallback(&name)
        .await
    {
        Ok(dataset_info) => {
            info!("Retrieved dataset info: {}", name);
            Ok(Json(ApiResponse::success(dataset_info)))
        }
        Err(e) => {
            error!("Failed to get dataset info for {}: {}", name, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Destroy a dataset
pub async fn destroy_dataset(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    debug!("Destroying dataset: {}", name);

    match state
        .zfs_manager
        .dataset_manager
        .destroy_dataset(&name)
        .await
    {
        Ok(()) => {
            info!("Successfully destroyed dataset: {}", name);
            Ok(Json(ApiResponse::success(())))
        }
        Err(e) => {
            error!("Failed to destroy dataset {}: {}", name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get dataset properties
pub async fn get_dataset_properties(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
) -> Result<Json<ApiResponse<HashMap<String, String>>>, StatusCode> {
    debug!("Getting properties for dataset: {}", name);

    match state
        .zfs_manager
        .dataset_manager
        .get_dataset_properties(&name)
        .await
    {
        Ok(properties) => {
            info!(
                "Retrieved {} properties for dataset: {}",
                properties.len(),
                name
            );
            Ok(Json(ApiResponse::success(properties)))
        }
        Err(e) => {
            error!("Failed to get properties for dataset {}: {}", name, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Set dataset properties
pub async fn set_dataset_properties(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
    Json(properties): Json<HashMap<String, String>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    debug!(
        "Setting {} properties for dataset: {}",
        properties.len(),
        name
    );

    match state
        .zfs_manager
        .dataset_manager
        .set_dataset_properties(&name, &properties)
        .await
    {
        Ok(()) => {
            info!("Successfully set properties for dataset: {}", name);
            Ok(Json(ApiResponse::success(())))
        }
        Err(e) => {
            error!("Failed to set properties for dataset {}: {}", name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Snapshot Management Endpoints

/// List snapshots for a dataset
pub async fn list_snapshots(
    State(state): State<ZfsApiState>,
    Path(name): Path<String>,
) -> Result<Json<ApiResponse<Vec<SnapshotInfo>>>, StatusCode> {
    debug!("Listing snapshots for dataset: {}", name);

    match state.zfs_manager.list_snapshots(&name).await {
        Ok(snapshots) => {
            info!("Listed {} snapshots for dataset: {}", snapshots.len(), name);
            Ok(Json(ApiResponse::success(snapshots)))
        }
        Err(e) => {
            error!("Failed to list snapshots for dataset {}: {}", name, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a snapshot
pub async fn create_snapshot(
    State(state): State<ZfsApiState>,
    Path(dataset): Path<String>,
    Json(request): Json<CreateSnapshotRequest>,
) -> Result<Json<ApiResponse<SnapshotOperationResponse>>, StatusCode> {
    debug!(
        "Creating snapshot {} for dataset: {}",
        request.name, dataset
    );

    let recursive = request.recursive.unwrap_or(false);

    match state
        .zfs_manager
        .snapshot_manager
        .create_snapshot(&dataset, &request.name, recursive)
        .await
    {
        Ok(operation_id) => {
            info!(
                "Successfully created snapshot: {}@{}",
                dataset, request.name
            );
            let response = SnapshotOperationResponse { operation_id };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!(
                "Failed to create snapshot {}@{}: {}",
                dataset, request.name, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Delete a snapshot
pub async fn delete_snapshot(
    State(state): State<ZfsApiState>,
    Path((dataset, snapshot)): Path<(String, String)>,
) -> Result<Json<ApiResponse<SnapshotOperationResponse>>, StatusCode> {
    debug!("Deleting snapshot {} from dataset: {}", snapshot, dataset);

    match state
        .zfs_manager
        .snapshot_manager
        .delete_snapshot(&dataset, &snapshot)
        .await
    {
        Ok(operation_id) => {
            info!("Successfully deleted snapshot: {}@{}", dataset, snapshot);
            let response = SnapshotOperationResponse { operation_id };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to delete snapshot {}@{}: {}", dataset, snapshot, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// AI and Optimization Endpoints

/// Tier prediction request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPredictionRequest {
    pub file_path: String,
}

/// Get AI tier prediction for a file
pub async fn get_tier_prediction(
    State(state): State<ZfsApiState>,
    Json(request): Json<TierPredictionRequest>,
) -> Result<Json<ApiResponse<Option<nestgate_zfs::ai_integration::TierPrediction>>>, StatusCode> {
    debug!("Getting tier prediction for file: {}", request.file_path);

    match state
        .zfs_manager
        .get_ai_tier_recommendation(&request.file_path)
        .await
    {
        Ok(prediction) => {
            info!("Retrieved tier prediction for file: {}", request.file_path);
            Ok(Json(ApiResponse::success(prediction)))
        }
        Err(e) => {
            error!(
                "Failed to get tier prediction for {}: {}",
                request.file_path, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get performance analytics
pub async fn get_performance_analytics(
    State(state): State<ZfsApiState>,
) -> Result<Json<ApiResponse<nestgate_zfs::manager::PerformanceAnalytics>>, StatusCode> {
    debug!("Getting performance analytics");

    match state.zfs_manager.get_performance_analytics().await {
        Ok(analytics) => {
            info!("Retrieved performance analytics");
            Ok(Json(ApiResponse::success(analytics)))
        }
        Err(e) => {
            error!("Failed to get performance analytics: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Trigger optimization
pub async fn trigger_optimization(
    State(state): State<ZfsApiState>,
) -> Result<Json<ApiResponse<nestgate_zfs::manager::OptimizationResult>>, StatusCode> {
    debug!("Triggering optimization");

    match state.zfs_manager.trigger_optimization().await {
        Ok(result) => {
            info!("Optimization triggered successfully");
            Ok(Json(ApiResponse::success(result)))
        }
        Err(e) => {
            error!("Failed to trigger optimization: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Response Types

/// Pool status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStatusResponse {
    pub status: String,
}

/// Snapshot operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotOperationResponse {
    pub operation_id: String,
}
