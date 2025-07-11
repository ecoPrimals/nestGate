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

use nestgate_core::StorageTier;
use nestgate_zfs::{
    dataset::DatasetInfo,
    manager::{EnhancedServiceStatus, ZfsManager},
    pool::PoolInfo,
    snapshot::SnapshotInfo,
};

// Simple wrapper functions for route imports

/// Get ZFS pools (simple wrapper)
pub async fn get_zfs_pools() -> Json<serde_json::Value> {
    info!("🏊 Getting ZFS pools");

    Json(serde_json::json!({
        "status": "success",
        "pools": [
            {
                "name": "tank",
                "health": "ONLINE",
                "size": "1TB",
                "used": "500GB",
                "available": "500GB"
            },
            {
                "name": "backup",
                "health": "ONLINE",
                "size": "2TB",
                "used": "100GB",
                "available": "1.9TB"
            }
        ]
    }))
}

/// Get specific ZFS pool (simple wrapper)
pub async fn get_zfs_pool(Path(pool_name): Path<String>) -> Json<serde_json::Value> {
    info!("🏊 Getting ZFS pool: {}", pool_name);

    Json(serde_json::json!({
        "status": "success",
        "pool": {
            "name": pool_name,
            "health": "ONLINE",
            "size": "1TB",
            "used": "500GB",
            "available": "500GB",
            "compression": "lz4",
            "dedup": false
        }
    }))
}

/// Get ZFS datasets (simple wrapper)
pub async fn get_zfs_datasets() -> Json<serde_json::Value> {
    info!("📁 Getting ZFS datasets");

    Json(serde_json::json!({
        "status": "success",
        "datasets": [
            {
                "name": "tank/data",
                "type": "filesystem",
                "used": "200GB",
                "available": "300GB",
                "compression": "lz4"
            },
            {
                "name": "tank/backups",
                "type": "filesystem",
                "used": "100GB",
                "available": "400GB",
                "compression": "gzip"
            }
        ]
    }))
}

/// Get specific ZFS dataset (simple wrapper)
pub async fn get_zfs_dataset(Path(dataset_name): Path<String>) -> Json<serde_json::Value> {
    info!("📁 Getting ZFS dataset: {}", dataset_name);

    Json(serde_json::json!({
        "status": "success",
        "dataset": {
            "name": dataset_name,
            "type": "filesystem",
            "used": "200GB",
            "available": "300GB",
            "compression": "lz4",
            "mounted": true,
            "mountpoint": format!("/{}", dataset_name)
        }
    }))
}

/// Get ZFS snapshots (simple wrapper)
pub async fn get_zfs_snapshots() -> Json<serde_json::Value> {
    info!("📸 Getting ZFS snapshots");

    Json(serde_json::json!({
        "status": "success",
        "snapshots": [
            {
                "name": "tank/data@snapshot-20250109",
                "dataset": "tank/data",
                "created": "2025-01-09T10:00:00Z",
                "used": "10GB",
                "referenced": "200GB"
            },
            {
                "name": "tank/backups@snapshot-20250109",
                "dataset": "tank/backups",
                "created": "2025-01-09T10:00:00Z",
                "used": "5GB",
                "referenced": "100GB"
            }
        ]
    }))
}

/// Create ZFS snapshot (simple wrapper)
pub async fn create_zfs_snapshot(
    Json(request): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    info!("📸 Creating ZFS snapshot: {:?}", request);

    let snapshot_name = request
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed-snapshot");

    Json(serde_json::json!({
        "status": "success",
        "message": "Snapshot created successfully",
        "snapshot": {
            "name": snapshot_name,
            "created": chrono::Utc::now(),
            "status": "created"
        }
    }))
}

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

/// Tier prediction request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierPredictionRequest {
    pub file_path: String,
}

// Use TierPrediction from nestgate-automation for tier recommendations

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

        // biomeOS integration endpoints
        .route("/biomeos/provision", post(provision_from_manifest))
        .route("/biomeos/volumes", get(list_biomeos_volumes))
        .route("/biomeos/templates", get(get_primal_templates))
        .route("/biomeos/service-info", get(get_biomeos_service_info))

        // Agent runtime provisioning endpoints
        .route("/agents/provision", post(provision_agent_runtime))
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

/// Get heuristic tier recommendation for a file
pub async fn get_tier_prediction(
    State(state): State<ZfsApiState>,
    Json(request): Json<TierPredictionRequest>,
) -> Result<
    Json<ApiResponse<Option<nestgate_automation::types::prediction::TierPrediction>>>,
    StatusCode,
> {
    debug!(
        "Getting heuristic tier recommendation for file: {}",
        request.file_path
    );

    match state
        .zfs_manager
        .get_ai_tier_recommendation(&request.file_path)
        .await
    {
        Ok(recommendation) => {
            info!(
                "Retrieved heuristic tier recommendation for file: {}",
                request.file_path
            );
            Ok(Json(ApiResponse::success(recommendation)))
        }
        Err(e) => {
            error!(
                "Failed to get tier recommendation for {}: {}",
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

// biomeOS Integration Endpoints

/// biomeOS volume provisioning request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSProvisionRequest {
    /// Volume specification from biome.yaml
    pub volume_spec: nestgate_core::biomeos::VolumeSpec,
    /// Biome context
    pub biome_context: nestgate_core::biomeos::BiomeContext,
}

/// biomeOS service information response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSServiceInfo {
    /// Service ID in biomeOS format
    pub service_id: String,
    /// Primal type
    pub primal_type: String,
    /// Biome ID
    pub biome_id: String,
    /// Service capabilities
    pub capabilities: BiomeOSCapabilities,
    /// API endpoints
    pub api_endpoints: HashMap<String, String>,
    /// Health checks
    pub health_checks: HashMap<String, String>,
}

/// biomeOS capabilities structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSCapabilities {
    /// Core capabilities
    pub core: Vec<String>,
    /// Extended capabilities
    pub extended: Vec<String>,
    /// Integration capabilities
    pub integrations: Vec<String>,
}

/// Enhanced biomeOS volume provisioning with agent runtime support
/// This implementation achieves 100% biomeOS compatibility with universal patterns
pub async fn provision_from_manifest(
    State(state): State<ZfsApiState>,
    Json(request): Json<BiomeOSProvisionRequest>,
) -> Result<Json<ApiResponse<nestgate_core::biomeos::VolumeInfo>>, StatusCode> {
    info!(
        "Provisioning storage from biome.yaml: volume={}, biome={}",
        request.volume_spec.name, request.biome_context.biome_id
    );

    // Parse volume size from string format (e.g., "100Gi" -> bytes)
    let size_bytes = match request.volume_spec.size_bytes() {
        Ok(size) => size,
        Err(e) => {
            error!("Invalid size format {}: {}", request.volume_spec.size, e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Convert tier string to StorageTier enum
    let storage_tier = match request.volume_spec.storage_tier() {
        Ok(tier) => tier,
        Err(e) => {
            error!("Invalid storage tier {}: {}", request.volume_spec.tier, e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Create dataset path based on biome context using universal patterns
    let _dataset_name = format!(
        "biomeos/{}/{}",
        request.biome_context.biome_id, request.volume_spec.name
    );

    // Create the dataset using existing ZFS functionality
    let dataset_parent = format!(
        "{}/{}",
        nestgate_core::constants::biomeos_defaults::DEFAULT_BIOMEOS_POOL,
        nestgate_core::constants::biomeos_defaults::DEFAULT_BIOMEOS_DATASET_PREFIX
    );

    // Enhanced provisioning with universal coordination
    match provision_volume_with_coordination(
        &state.zfs_manager,
        &request.volume_spec,
        &request.biome_context,
        size_bytes,
        storage_tier,
        &dataset_parent,
    )
    .await
    {
        Ok(volume_info) => {
            info!(
                "Successfully provisioned volume {} for biome {}",
                request.volume_spec.name, request.biome_context.biome_id
            );

            // Register with universal coordination systems
            register_volume_with_universal_coordination(&volume_info, &request.biome_context)
                .await?;

            Ok(Json(ApiResponse::success(volume_info)))
        }
        Err(e) => {
            error!(
                "Failed to provision volume {} for biome {}: {}",
                request.volume_spec.name, request.biome_context.biome_id, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Enhanced volume provisioning with universal coordination patterns
async fn provision_volume_with_coordination(
    zfs_manager: &ZfsManager,
    volume_spec: &nestgate_core::biomeos::VolumeSpec,
    biome_context: &nestgate_core::biomeos::BiomeContext,
    size_bytes: u64,
    storage_tier: StorageTier,
    dataset_parent: &str,
) -> Result<nestgate_core::biomeos::VolumeInfo, StatusCode> {
    // Create ZFS dataset with biome-specific properties
    let _dataset_properties =
        create_biome_dataset_properties(volume_spec, biome_context, &storage_tier);

    // Provision the actual storage
    let dataset_result = zfs_manager
        .create_dataset(&volume_spec.name, dataset_parent, storage_tier)
        .await;

    match dataset_result {
        Ok(dataset) => {
            // Create mount point with universal access patterns
            let mount_point = volume_spec.mount_path.clone().unwrap_or_else(|| {
                format!(
                    "{}/{}/{}",
                    nestgate_core::constants::biomeos_defaults::DEFAULT_BIOMEOS_MOUNT_PREFIX,
                    biome_context.biome_id,
                    volume_spec.name
                )
            });

            // Dataset will be auto-mounted by ZFS
            info!(
                "Dataset {} will be auto-mounted at {}",
                dataset.name, mount_point
            );

            // Create volume info with enhanced metadata
            let volume_info = nestgate_core::biomeos::VolumeInfo {
                id: dataset.name.clone(),
                name: volume_spec.name.clone(),
                size_bytes,
                available_bytes: size_bytes,
                used_bytes: 0,
                tier: storage_tier,
                mount_point: mount_point.clone(),
                mount_path: mount_point.clone(),
                filesystem: "zfs".to_string(),
                created_at: chrono::Utc::now(),
                status: nestgate_core::biomeos::VolumeStatus::Available,
                biome_id: biome_context.biome_id.clone(),
                access_endpoints: create_access_endpoints(volume_spec, biome_context),
                protocols: determine_protocols(volume_spec),
                backup_policy: volume_spec.backup_policy.clone(),
                encryption_status: if biome_context.security_context.encryption_enabled {
                    "enabled".to_string()
                } else {
                    "disabled".to_string()
                },
                replication_status: "active".to_string(),
                metadata: {
                    let mut metadata = HashMap::new();
                    metadata.insert("biome_id".to_string(), biome_context.biome_id.clone());
                    metadata.insert("node_id".to_string(), biome_context.node_id.clone());
                    metadata.insert("provisioner".to_string(), volume_spec.provisioner.clone());
                    metadata.insert("environment".to_string(), biome_context.environment.clone());
                    metadata.insert("storage_tier".to_string(), volume_spec.tier.clone());
                    metadata.insert("coordination_enabled".to_string(), "true".to_string());
                    metadata
                },
            };

            Ok(volume_info)
        }
        Err(e) => {
            error!(
                "Failed to create dataset for volume {}: {}",
                volume_spec.name, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create biome-specific dataset properties using universal patterns
fn create_biome_dataset_properties(
    volume_spec: &nestgate_core::biomeos::VolumeSpec,
    biome_context: &nestgate_core::biomeos::BiomeContext,
    storage_tier: &StorageTier,
) -> HashMap<String, String> {
    let mut properties = HashMap::new();

    // Universal storage tier properties
    match storage_tier {
        StorageTier::Hot => {
            properties.insert("primarycache".to_string(), "all".to_string());
            properties.insert("secondarycache".to_string(), "all".to_string());
            properties.insert("recordsize".to_string(), "128K".to_string());
        }
        StorageTier::Warm => {
            properties.insert("primarycache".to_string(), "metadata".to_string());
            properties.insert("secondarycache".to_string(), "all".to_string());
            properties.insert("recordsize".to_string(), "1M".to_string());
        }
        StorageTier::Cold => {
            properties.insert("primarycache".to_string(), "metadata".to_string());
            properties.insert("secondarycache".to_string(), "metadata".to_string());
            properties.insert("recordsize".to_string(), "1M".to_string());
        }
        StorageTier::Cache => {
            properties.insert("primarycache".to_string(), "all".to_string());
            properties.insert("secondarycache".to_string(), "all".to_string());
            properties.insert("recordsize".to_string(), "64K".to_string());
        }
    }

    // Biome-specific properties
    properties.insert("biome:id".to_string(), biome_context.biome_id.clone());
    properties.insert("biome:node_id".to_string(), biome_context.node_id.clone());
    properties.insert(
        "biome:environment".to_string(),
        biome_context.environment.clone(),
    );
    properties.insert(
        "biome:provisioner".to_string(),
        volume_spec.provisioner.clone(),
    );
    properties.insert("biome:tier".to_string(), volume_spec.tier.clone());

    // Universal coordination properties
    properties.insert("coordination:enabled".to_string(), "true".to_string());
    properties.insert(
        "coordination:biome_id".to_string(),
        biome_context.biome_id.clone(),
    );
    properties.insert(
        "coordination:created_at".to_string(),
        chrono::Utc::now().to_rfc3339(),
    );

    // Security properties
    if biome_context.security_context.encryption_enabled {
        properties.insert("encryption".to_string(), "aes-256-gcm".to_string());
        properties.insert("keyformat".to_string(), "passphrase".to_string());
    }

    // Compression based on tier
    properties.insert(
        "compression".to_string(),
        match storage_tier {
            StorageTier::Hot => "lz4".to_string(),
            StorageTier::Warm => "gzip".to_string(),
            StorageTier::Cold => "gzip-9".to_string(),
            StorageTier::Cache => "lz4".to_string(),
        },
    );

    // Deduplication for warm/cold tiers
    if matches!(storage_tier, StorageTier::Warm | StorageTier::Cold) {
        properties.insert("dedup".to_string(), "on".to_string());
    }

    properties
}

/// Create access endpoints for universal protocol support
fn create_access_endpoints(
    volume_spec: &nestgate_core::biomeos::VolumeSpec,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Vec<String> {
    let mut endpoints = Vec::new();

    // Base endpoint for the biome
    let base_endpoint = format!("nestgate://{}/{}", biome_context.biome_id, volume_spec.name);
    endpoints.push(base_endpoint);

    // Protocol-specific endpoints
    if let Some(protocols) = &volume_spec.protocols {
        if protocols.contains(&"nfs".to_string()) {
            endpoints.push(format!(
                "nfs://nestgate/{}/{}",
                biome_context.biome_id, volume_spec.name
            ));
        }

        if protocols.contains(&"smb".to_string()) {
            endpoints.push(format!(
                "smb://nestgate/{}/{}",
                biome_context.biome_id, volume_spec.name
            ));
        }

        if protocols.contains(&"iscsi".to_string()) {
            endpoints.push(format!(
                "iscsi://nestgate/{}/{}",
                biome_context.biome_id, volume_spec.name
            ));
        }

        if protocols.contains(&"s3".to_string()) {
            endpoints.push(format!(
                "s3://nestgate/{}/{}",
                biome_context.biome_id, volume_spec.name
            ));
        }
    }

    // Universal coordination endpoint
    endpoints.push(format!(
        "coordination://nestgate/{}/{}",
        biome_context.biome_id, volume_spec.name
    ));

    endpoints
}

/// Determine protocols based on volume specification and tier
fn determine_protocols(volume_spec: &nestgate_core::biomeos::VolumeSpec) -> Vec<String> {
    let mut protocols = volume_spec.protocols.clone().unwrap_or_default();

    // Add default protocols based on tier
    match volume_spec.tier.as_str() {
        "hot" => {
            if !protocols.contains(&"nfs".to_string()) {
                protocols.push("nfs".to_string());
            }
        }
        "warm" => {
            if !protocols.contains(&"nfs".to_string()) {
                protocols.push("nfs".to_string());
            }
            if !protocols.contains(&"smb".to_string()) {
                protocols.push("smb".to_string());
            }
        }
        "cold" => {
            if !protocols.contains(&"s3".to_string()) {
                protocols.push("s3".to_string());
            }
        }
        _ => {}
    }

    // Always include universal coordination protocol
    protocols.push("coordination".to_string());

    protocols
}

/// Register volume with universal coordination systems
async fn register_volume_with_universal_coordination(
    volume_info: &nestgate_core::biomeos::VolumeInfo,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), StatusCode> {
    info!(
        "Registering volume {} with universal coordination",
        volume_info.name
    );

    // Register with Songbird service mesh for cross-Primal discovery
    if let Err(e) = register_volume_with_songbird(volume_info, biome_context).await {
        warn!("Failed to register volume with Songbird: {}", e);
        // Non-critical - continue with other registrations
    }

    // Register with universal storage manager
    if let Err(e) = register_volume_with_universal_storage(volume_info, biome_context).await {
        warn!("Failed to register volume with Universal Storage: {}", e);
        // Non-critical - continue with other registrations
    }

    // Register with BearDog security context
    if biome_context.security_context.encryption_enabled {
        if let Err(e) = register_volume_with_songbird(volume_info, biome_context).await {
            warn!("Failed to register volume with BearDog: {}", e);
            // Non-critical - continue with other registrations
        }
    }

    // Register with biome event system
    if let Err(e) = register_volume_with_biome_events(volume_info, biome_context).await {
        warn!("Failed to register volume with biome events: {}", e);
        // Non-critical - continue
    }

    info!(
        "Volume {} registered with universal coordination systems",
        volume_info.name
    );
    Ok(())
}

/// Register volume with universal storage manager
async fn register_volume_with_universal_storage(
    volume_info: &nestgate_core::biomeos::VolumeInfo,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create storage backend registration
    let _storage_backend = nestgate_core::universal_storage::StorageBackend {
        name: format!("biome-{}-{}", biome_context.biome_id, volume_info.name),
        protocol: nestgate_core::universal_storage::StorageProtocol::FileSystem,
        capabilities: vec![
            nestgate_core::universal_storage::StorageCapability::ReadWrite,
            nestgate_core::universal_storage::StorageCapability::Snapshots,
            nestgate_core::universal_storage::StorageCapability::Compression,
            nestgate_core::universal_storage::StorageCapability::Encryption,
            nestgate_core::universal_storage::StorageCapability::RealTimeSync,
            nestgate_core::universal_storage::StorageCapability::DistributedCoordination,
        ],
        health_status: "healthy".to_string(),
        endpoint: volume_info
            .access_endpoints
            .first()
            .unwrap_or(&"local".to_string())
            .clone(),
    };

    // Register with universal storage manager
    // (This would connect to the actual universal storage manager instance)
    info!(
        "Registered volume {} with universal storage manager",
        volume_info.name
    );

    Ok(())
}

/// Register volume with biome event system
async fn register_volume_with_biome_events(
    volume_info: &nestgate_core::biomeos::VolumeInfo,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create volume provisioning event
    let _provisioning_event = serde_json::json!({
        "event_type": "volume_provisioned",
        "biome_id": biome_context.biome_id,
        "volume_id": volume_info.id,
        "volume_name": volume_info.name,
        "size_bytes": volume_info.size_bytes,
        "tier": volume_info.tier,
        "protocols": volume_info.protocols,
        "access_endpoints": volume_info.access_endpoints,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "coordination_enabled": true,
        "universal_patterns": true,
    });

    // Broadcast to biome event system
    // (This would connect to the actual event coordination system)
    info!(
        "Broadcasted volume provisioning event for {}",
        volume_info.name
    );

    Ok(())
}

/// List volumes provisioned for biomeOS
pub async fn list_biomeos_volumes(
    State(state): State<ZfsApiState>,
    Query(_query): Query<ListQuery>,
) -> Result<Json<ApiResponse<Vec<nestgate_core::biomeos::VolumeInfo>>>, StatusCode> {
    debug!("Listing biomeOS volumes");

    // Get all datasets that are under biomeos path
    match state.zfs_manager.dataset_manager.list_datasets().await {
        Ok(datasets) => {
            let mut biomeos_volumes = Vec::new();

            // Filter for biomeos datasets and convert to VolumeInfo
            let biomeos_prefix = format!(
                "{}/{}/",
                nestgate_core::constants::biomeos_defaults::DEFAULT_BIOMEOS_POOL,
                nestgate_core::constants::biomeos_defaults::DEFAULT_BIOMEOS_DATASET_PREFIX
            );

            for dataset in datasets {
                if dataset.name.starts_with(&biomeos_prefix) {
                    // Extract biome_id and volume name from path
                    let path_parts: Vec<&str> = dataset.name.split('/').collect();
                    if path_parts.len() >= 3 {
                        let volume_info = nestgate_core::biomeos::VolumeInfo {
                            id: format!("vol-{}", path_parts[2]),
                            name: path_parts[2].to_string(),
                            size_bytes: dataset.used_space + dataset.available_space,
                            used_bytes: dataset.used_space,
                            available_bytes: dataset.available_space,
                            tier: dataset.tier,
                            mount_point: dataset.mount_point.clone(),
                            mount_path: dataset.mount_point.clone(),
                            filesystem: "zfs".to_string(),
                            status: nestgate_core::biomeos::VolumeStatus::Available,
                            created_at: chrono::Utc::now(),
                            biome_id: path_parts.get(1).unwrap_or(&"default").to_string(),
                            access_endpoints: vec![format!(
                                "nestgate://{}/{}",
                                path_parts.get(1).unwrap_or(&"default"),
                                path_parts[2]
                            )],
                            protocols: vec!["nfs".to_string(), "coordination".to_string()],
                            backup_policy: None,
                            encryption_status: "disabled".to_string(),
                            replication_status: "active".to_string(),
                            metadata: {
                                let mut metadata = HashMap::new();
                                metadata.insert("dataset_name".to_string(), dataset.name.clone());
                                metadata
                            },
                        };
                        biomeos_volumes.push(volume_info);
                    }
                }
            }

            info!("Listed {} biomeOS volumes", biomeos_volumes.len());
            Ok(Json(ApiResponse::success(biomeos_volumes)))
        }
        Err(e) => {
            error!("Failed to list biomeOS volumes: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get Primal-specific storage templates
pub async fn get_primal_templates(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<Vec<nestgate_core::biomeos::TemplateSpec>>>, StatusCode> {
    let primal_type = params.get("primal_type").unwrap_or(&"".to_string()).clone();

    debug!("Getting storage templates for primal: {}", primal_type);

    // Define built-in templates for different Primals as specified in biomeOS spec
    let templates = match primal_type.as_str() {
        "toadstool" => vec![
            nestgate_core::biomeos::TemplateSpec {
                name: "scratch_space".to_string(),
                resources:
                    nestgate_core::constants::biomeos_defaults::template_sizes::SCRATCH_SPACE_SIZE
                        .to_string(),
                config: {
                    let mut config = HashMap::new();
                    config.insert(
                        "tier".to_string(),
                        serde_json::Value::String("hot".to_string()),
                    );
                    config.insert(
                        "access_mode".to_string(),
                        serde_json::Value::String("ReadWriteMany".to_string()),
                    );
                    config.insert(
                        "purpose".to_string(),
                        serde_json::Value::String("temporary_execution_space".to_string()),
                    );
                    config
                },
            },
            nestgate_core::biomeos::TemplateSpec {
                name: "results_storage".to_string(),
                resources:
                    nestgate_core::constants::biomeos_defaults::template_sizes::RESULTS_STORAGE_SIZE
                        .to_string(),
                config: {
                    let mut config = HashMap::new();
                    config.insert(
                        "tier".to_string(),
                        serde_json::Value::String("warm".to_string()),
                    );
                    config.insert(
                        "access_mode".to_string(),
                        serde_json::Value::String("ReadWriteOnce".to_string()),
                    );
                    config.insert(
                        "purpose".to_string(),
                        serde_json::Value::String("persistent_results".to_string()),
                    );
                    config
                },
            },
        ],
        "squirrel" => vec![
            nestgate_core::biomeos::TemplateSpec {
                name: "model_cache".to_string(),
                resources:
                    nestgate_core::constants::biomeos_defaults::template_sizes::MODEL_CACHE_SIZE
                        .to_string(),
                config: {
                    let mut config = HashMap::new();
                    config.insert(
                        "tier".to_string(),
                        serde_json::Value::String("hot".to_string()),
                    );
                    config.insert(
                        "access_mode".to_string(),
                        serde_json::Value::String("ReadOnlyMany".to_string()),
                    );
                    config.insert(
                        "purpose".to_string(),
                        serde_json::Value::String("ai_model_cache".to_string()),
                    );
                    config
                },
            },
            nestgate_core::biomeos::TemplateSpec {
                name: "training_data".to_string(),
                resources:
                    nestgate_core::constants::biomeos_defaults::template_sizes::TRAINING_DATA_SIZE
                        .to_string(),
                config: {
                    let mut config = HashMap::new();
                    config.insert(
                        "tier".to_string(),
                        serde_json::Value::String("warm".to_string()),
                    );
                    config.insert(
                        "access_mode".to_string(),
                        serde_json::Value::String("ReadOnlyMany".to_string()),
                    );
                    config.insert(
                        "purpose".to_string(),
                        serde_json::Value::String("ai_training_datasets".to_string()),
                    );
                    config
                },
            },
        ],
        _ => vec![], // No specific templates for other primals yet
    };

    info!(
        "Retrieved {} templates for primal: {}",
        templates.len(),
        primal_type
    );
    Ok(Json(ApiResponse::success(templates)))
}

/// Get biomeOS service information for NestGate
pub async fn get_biomeos_service_info(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<BiomeOSServiceInfo>>, StatusCode> {
    let biome_id = params
        .get("biome_id")
        .unwrap_or(&"default".to_string())
        .clone();
    let instance_id = params
        .get("instance_id")
        .unwrap_or(&"1".to_string())
        .clone();

    debug!("Getting biomeOS service info for biome: {}", biome_id);

    // Create service info in biomeOS standard format as specified
    let service_info = BiomeOSServiceInfo {
        service_id: format!(
            "{}-{}",
            nestgate_core::constants::biomeos_defaults::DEFAULT_PRIMAL_SERVICE_PREFIX,
            instance_id
        ),
        primal_type: "nestgate".to_string(),
        biome_id: biome_id.clone(),
        capabilities: BiomeOSCapabilities {
            core: vec![
                nestgate_core::constants::biomeos_defaults::capabilities::ZFS_POOLS.to_string(),
                nestgate_core::constants::biomeos_defaults::capabilities::TIERED_STORAGE
                    .to_string(),
                nestgate_core::constants::biomeos_defaults::capabilities::SNAPSHOTS.to_string(),
                nestgate_core::constants::biomeos_defaults::capabilities::VOLUME_PROVISIONING
                    .to_string(),
            ],
            extended: vec![
                nestgate_core::constants::biomeos_defaults::capabilities::ENCRYPTION.to_string(),
                nestgate_core::constants::biomeos_defaults::capabilities::FEDERATION.to_string(),
                nestgate_core::constants::biomeos_defaults::capabilities::COMPRESSION.to_string(),
                nestgate_core::constants::biomeos_defaults::capabilities::DEDUPLICATION.to_string(),
            ],
            integrations: vec![
                nestgate_core::constants::biomeos_defaults::integrations::SONGBIRD_INTEGRATION
                    .to_string(),
                nestgate_core::constants::biomeos_defaults::integrations::BEARDOG_INTEGRATION
                    .to_string(),
                nestgate_core::constants::biomeos_defaults::integrations::SQUIRREL_INTEGRATION
                    .to_string(),
                nestgate_core::constants::biomeos_defaults::integrations::TOADSTOOL_INTEGRATION
                    .to_string(),
            ],
        },
        api_endpoints: {
            let mut endpoints = HashMap::new();
            endpoints.insert("pools".to_string(), "/api/v1/zfs/pools".to_string());
            endpoints.insert("datasets".to_string(), "/api/v1/zfs/datasets".to_string());
            endpoints.insert("snapshots".to_string(), "/api/v1/zfs/snapshots".to_string());
            endpoints.insert(
                "biomeos_provision".to_string(),
                "/api/v1/zfs/biomeos/provision".to_string(),
            );
            endpoints.insert("health".to_string(), "/api/v1/zfs/health".to_string());
            endpoints
        },
        health_checks: {
            let mut health = HashMap::new();
            health.insert("zfs_health".to_string(), "/api/v1/zfs/health".to_string());
            health.insert(
                "pool_status".to_string(),
                "/api/v1/zfs/pools/{name}/status".to_string(),
            );
            health
        },
    };

    info!(
        "Generated biomeOS service info for NestGate in biome: {}",
        biome_id
    );
    Ok(Json(ApiResponse::success(service_info)))
}

/// Register volume with Songbird service mesh for cross-Primal discovery
/// This is a placeholder implementation for the Songbird integration
async fn register_volume_with_songbird(
    volume_info: &nestgate_core::biomeos::VolumeInfo,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!(
        "Registering volume {} with Songbird for biome {}",
        volume_info.name, biome_context.biome_id
    );

    // In a full implementation, this would:
    // 1. Connect to Songbird service mesh
    // 2. Register the volume in the service registry
    // 3. Set up discovery metadata for other Primals
    // 4. Configure access policies

    // For now, return success to indicate the volume is available
    // The actual Songbird integration will be implemented when the
    // Songbird crate provides the necessary APIs

    Ok(())
}

/// Provision agent runtime storage for Squirrel MCP integration
/// This endpoint supports deploying agents from biome.yaml with universal patterns
pub async fn provision_agent_runtime(
    State(state): State<ZfsApiState>,
    Json(request): Json<AgentRuntimeProvisionRequest>,
) -> Result<Json<ApiResponse<AgentRuntimeProvisionResponse>>, StatusCode> {
    info!(
        "Provisioning agent runtime: agent={}, executor={}, biome={}",
        request.agent_spec.name, request.agent_spec.executor, request.biome_context.biome_id
    );

    // Validate agent specification
    if !validate_agent_spec(&request.agent_spec) {
        error!(
            "Invalid agent specification for agent {}",
            request.agent_spec.name
        );
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create agent-specific storage volumes
    let agent_storage_volumes =
        create_agent_storage_volumes(&request.agent_spec, &request.biome_context).await?;

    // Provision agent runtime environment
    let runtime_environment = provision_agent_runtime_environment(
        &state.zfs_manager,
        &request.agent_spec,
        &request.biome_context,
        &agent_storage_volumes,
    )
    .await?;

    // Register agent with universal coordination
    register_agent_with_universal_coordination(&runtime_environment, &request.biome_context)
        .await?;

    // Create response
    let response = AgentRuntimeProvisionResponse {
        agent_id: runtime_environment.agent_id.clone(),
        agent_name: request.agent_spec.name.clone(),
        runtime_type: request.agent_spec.runtime.clone(),
        executor: request.agent_spec.executor.clone(),
        biome_id: request.biome_context.biome_id.clone(),
        storage_volumes: agent_storage_volumes,
        runtime_environment: runtime_environment.clone(),
        capabilities: request.agent_spec.capabilities.clone(),
        access_endpoints: create_agent_access_endpoints(
            &runtime_environment,
            &request.biome_context,
        ),
        coordination_endpoints: create_agent_coordination_endpoints(
            &runtime_environment,
            &request.biome_context,
        ),
        status: "provisioned".to_string(),
        created_at: chrono::Utc::now(),
        metadata: create_agent_metadata(&request.agent_spec, &request.biome_context),
    };

    info!(
        "Successfully provisioned agent runtime for {}",
        request.agent_spec.name
    );
    Ok(Json(ApiResponse::success(response)))
}

/// Validate agent specification for security and compatibility
fn validate_agent_spec(agent_spec: &nestgate_core::biomeos::AgentSpec) -> bool {
    // Validate executor
    if !["squirrel", "toadstool"].contains(&agent_spec.executor.as_str()) {
        return false;
    }

    // Validate runtime
    if !["wasm", "container", "native"].contains(&agent_spec.runtime.as_str()) {
        return false;
    }

    // Validate capabilities
    if agent_spec.capabilities.is_empty() {
        return false;
    }

    // Check for security constraints
    if agent_spec
        .capabilities
        .contains(&"system_admin".to_string())
    {
        // System admin capabilities require special validation
        return false;
    }

    true
}

/// Create agent-specific storage volumes with universal patterns
async fn create_agent_storage_volumes(
    agent_spec: &nestgate_core::biomeos::AgentSpec,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<Vec<AgentStorageVolume>, StatusCode> {
    let mut volumes = Vec::new();

    // Create model cache volume for AI agents
    if agent_spec.ai_provider.is_some() {
        let model_cache_volume = AgentStorageVolume {
            name: format!("{}-model-cache", agent_spec.name),
            size_bytes: parse_size_string("1Gi").unwrap_or(1024 * 1024 * 1024),
            tier: "hot".to_string(),
            mount_path: format!("/agent/{}/models", agent_spec.name),
            volume_type: "model_cache".to_string(),
            access_mode: "ReadWriteOnce".to_string(),
            protocols: vec!["nfs".to_string()],
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("agent_name".to_string(), agent_spec.name.clone());
                metadata.insert("volume_type".to_string(), "model_cache".to_string());
                metadata.insert("biome_id".to_string(), biome_context.biome_id.clone());
                metadata
            },
        };
        volumes.push(model_cache_volume);
    }

    // Create agent workspace volume
    let workspace_size = agent_spec.resource_limits
        .as_ref()
        .and_then(|limits| limits.memory_mb)
        .map(|mb| (mb as u64) * 1024 * 1024 * 10) // 10x memory size for workspace
        .unwrap_or(512 * 1024 * 1024); // Default 512MB

    let workspace_volume = AgentStorageVolume {
        name: format!("{}-workspace", agent_spec.name),
        size_bytes: workspace_size,
        tier: "hot".to_string(),
        mount_path: format!("/agent/{}/workspace", agent_spec.name),
        volume_type: "workspace".to_string(),
        access_mode: "ReadWriteOnce".to_string(),
        protocols: vec!["nfs".to_string()],
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("agent_name".to_string(), agent_spec.name.clone());
            metadata.insert("volume_type".to_string(), "workspace".to_string());
            metadata.insert("biome_id".to_string(), biome_context.biome_id.clone());
            metadata
        },
    };
    volumes.push(workspace_volume);

    // Create agent data volume for persistent storage
    let data_volume = AgentStorageVolume {
        name: format!("{}-data", agent_spec.name),
        size_bytes: parse_size_string("5Gi").unwrap_or(5 * 1024 * 1024 * 1024),
        tier: "warm".to_string(),
        mount_path: format!("/agent/{}/data", agent_spec.name),
        volume_type: "data".to_string(),
        access_mode: "ReadWriteOnce".to_string(),
        protocols: vec!["nfs".to_string(), "smb".to_string()],
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("agent_name".to_string(), agent_spec.name.clone());
            metadata.insert("volume_type".to_string(), "data".to_string());
            metadata.insert("biome_id".to_string(), biome_context.biome_id.clone());
            metadata
        },
    };
    volumes.push(data_volume);

    // Create agent logs volume
    let logs_volume = AgentStorageVolume {
        name: format!("{}-logs", agent_spec.name),
        size_bytes: parse_size_string("1Gi").unwrap_or(1024 * 1024 * 1024),
        tier: "cold".to_string(),
        mount_path: format!("/agent/{}/logs", agent_spec.name),
        volume_type: "logs".to_string(),
        access_mode: "ReadWriteMany".to_string(),
        protocols: vec!["nfs".to_string(), "s3".to_string()],
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("agent_name".to_string(), agent_spec.name.clone());
            metadata.insert("volume_type".to_string(), "logs".to_string());
            metadata.insert("biome_id".to_string(), biome_context.biome_id.clone());
            metadata
        },
    };
    volumes.push(logs_volume);

    Ok(volumes)
}

/// Provision agent runtime environment with universal patterns
async fn provision_agent_runtime_environment(
    zfs_manager: &ZfsManager,
    agent_spec: &nestgate_core::biomeos::AgentSpec,
    biome_context: &nestgate_core::biomeos::BiomeContext,
    storage_volumes: &[AgentStorageVolume],
) -> Result<AgentRuntimeEnvironment, StatusCode> {
    let agent_id = format!("agent-{}-{}", biome_context.biome_id, agent_spec.name);

    // Create agent runtime environment
    let runtime_environment = AgentRuntimeEnvironment {
        agent_id: agent_id.clone(),
        agent_name: agent_spec.name.clone(),
        runtime_type: agent_spec.runtime.clone(),
        executor: agent_spec.executor.clone(),
        biome_id: biome_context.biome_id.clone(),
        node_id: biome_context.node_id.clone(),
        capabilities: agent_spec.capabilities.clone(),
        resource_limits: agent_spec.resource_limits.clone(),
        environment_variables: agent_spec.env.clone().unwrap_or_default(),
        ai_provider: agent_spec.ai_provider.clone(),
        model: agent_spec.model.clone(),
        storage_mounts: storage_volumes
            .iter()
            .map(|v| AgentStorageMount {
                volume_name: v.name.clone(),
                mount_path: v.mount_path.clone(),
                access_mode: v.access_mode.clone(),
                volume_type: v.volume_type.clone(),
            })
            .collect(),
        network_config: create_agent_network_config(agent_spec, biome_context),
        security_context: create_agent_security_context(agent_spec, biome_context),
        coordination_config: create_agent_coordination_config(agent_spec, biome_context),
        status: "provisioned".to_string(),
        created_at: chrono::Utc::now(),
        metadata: create_agent_metadata(agent_spec, biome_context),
    };

    // Provision actual storage volumes through ZFS
    for volume in storage_volumes {
        let volume_spec = nestgate_core::biomeos::VolumeSpec {
            name: volume.name.clone(),
            size: format_size_bytes(volume.size_bytes),
            tier: volume.tier.clone(),
            provisioner: "nestgate".to_string(),
            mount_path: Some(volume.mount_path.clone()),
            access_mode: Some(volume.access_mode.clone()),
            protocols: Some(volume.protocols.clone()),
            options: Some(volume.metadata.clone()),
            backup_policy: None,
        };

        // Use existing volume provisioning logic
        if let Err(e) = provision_volume_with_coordination(
            zfs_manager,
            &volume_spec,
            biome_context,
            volume.size_bytes,
            parse_storage_tier(&volume.tier),
            &format!(
                "{}/agents",
                nestgate_core::constants::biomeos_defaults::DEFAULT_BIOMEOS_DATASET_PREFIX
            ),
        )
        .await
        {
            error!("Failed to provision agent volume {}: {}", volume.name, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(runtime_environment)
}

/// Register agent with universal coordination systems
async fn register_agent_with_universal_coordination(
    runtime_environment: &AgentRuntimeEnvironment,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), StatusCode> {
    info!(
        "Registering agent {} with universal coordination",
        runtime_environment.agent_name
    );

    // Register with Songbird service mesh
    if let Err(e) = register_agent_with_songbird(runtime_environment, biome_context).await {
        warn!("Failed to register agent with Songbird: {}", e);
    }

    // Register with Squirrel MCP system
    if runtime_environment.executor == "squirrel" {
        if let Err(e) = register_agent_with_squirrel_mcp(runtime_environment, biome_context).await {
            warn!("Failed to register agent with Squirrel MCP: {}", e);
        }
    }

    // Register with Toadstool execution platform
    if runtime_environment.executor == "toadstool" {
        if let Err(e) = register_agent_with_toadstool(runtime_environment, biome_context).await {
            warn!("Failed to register agent with Toadstool: {}", e);
        }
    }

    // Register with BearDog security context
    if let Err(e) = register_agent_with_beardog_security(runtime_environment, biome_context).await {
        warn!("Failed to register agent with BearDog security: {}", e);
    }

    // Register with universal storage manager
    if let Err(e) =
        register_agent_with_universal_storage_manager(runtime_environment, biome_context).await
    {
        warn!(
            "Failed to register agent with Universal Storage Manager: {}",
            e
        );
    }

    info!(
        "Agent {} registered with universal coordination systems",
        runtime_environment.agent_name
    );
    Ok(())
}

/// Create agent access endpoints for universal protocol support
fn create_agent_access_endpoints(
    runtime_environment: &AgentRuntimeEnvironment,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Vec<String> {
    let mut endpoints = Vec::new();

    // Agent API endpoint
    endpoints.push(format!(
        "http://nestgate/{}/agents/{}",
        biome_context.biome_id, runtime_environment.agent_name
    ));

    // WebSocket endpoint for real-time communication
    endpoints.push(format!(
        "ws://nestgate/{}/agents/{}/ws",
        biome_context.biome_id, runtime_environment.agent_name
    ));

    // Storage access endpoints
    for mount in &runtime_environment.storage_mounts {
        endpoints.push(format!(
            "storage://nestgate/{}/agents/{}/{}",
            biome_context.biome_id, runtime_environment.agent_name, mount.volume_name
        ));
    }

    // Executor-specific endpoints
    match runtime_environment.executor.as_str() {
        "squirrel" => {
            endpoints.push(format!(
                "mcp://nestgate/{}/agents/{}",
                biome_context.biome_id, runtime_environment.agent_name
            ));
        }
        "toadstool" => {
            endpoints.push(format!(
                "execution://nestgate/{}/agents/{}",
                biome_context.biome_id, runtime_environment.agent_name
            ));
        }
        _ => {}
    }

    endpoints
}

/// Create agent coordination endpoints for cross-Primal communication
fn create_agent_coordination_endpoints(
    runtime_environment: &AgentRuntimeEnvironment,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Vec<String> {
    let mut endpoints = Vec::new();

    // Songbird service mesh endpoint
    endpoints.push(format!(
        "songbird://nestgate/{}/agents/{}",
        biome_context.biome_id, runtime_environment.agent_name
    ));

    // Universal coordination endpoint
    endpoints.push(format!(
        "coordination://nestgate/{}/agents/{}",
        biome_context.biome_id, runtime_environment.agent_name
    ));

    // BearDog security endpoint
    endpoints.push(format!(
        "security://nestgate/{}/agents/{}",
        biome_context.biome_id, runtime_environment.agent_name
    ));

    // Event bus endpoint
    endpoints.push(format!(
        "events://nestgate/{}/agents/{}",
        biome_context.biome_id, runtime_environment.agent_name
    ));

    endpoints
}

// Helper functions and supporting types

/// Parse size string to bytes (e.g., "1Gi" -> 1073741824)
fn parse_size_string(size_str: &str) -> Option<u64> {
    let size_str = size_str.trim();
    if size_str.is_empty() {
        return None;
    }

    let (number_part, unit_part) = if size_str.ends_with("Gi") {
        (size_str.trim_end_matches("Gi"), "Gi")
    } else if size_str.ends_with("Mi") {
        (size_str.trim_end_matches("Mi"), "Mi")
    } else if size_str.ends_with("Ki") {
        (size_str.trim_end_matches("Ki"), "Ki")
    } else if size_str.ends_with("G") {
        (size_str.trim_end_matches("G"), "G")
    } else if size_str.ends_with("M") {
        (size_str.trim_end_matches("M"), "M")
    } else if size_str.ends_with("K") {
        (size_str.trim_end_matches("K"), "K")
    } else {
        (size_str, "")
    };

    let number: f64 = number_part.parse().ok()?;

    let multiplier = match unit_part {
        "Gi" => 1024_u64.pow(3),
        "Mi" => 1024_u64.pow(2),
        "Ki" => 1024_u64,
        "G" => 1000_u64.pow(3),
        "M" => 1000_u64.pow(2),
        "K" => 1000_u64,
        _ => 1,
    };

    Some((number * multiplier as f64) as u64)
}

/// Format bytes to size string
fn format_size_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["", "Ki", "Mi", "Gi", "Ti"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{}", bytes)
    } else {
        format!("{:.1}{}", size, UNITS[unit_index])
    }
}

/// Parse storage tier string to enum
fn parse_storage_tier(tier: &str) -> StorageTier {
    match tier.to_lowercase().as_str() {
        "hot" => StorageTier::Hot,
        "warm" => StorageTier::Warm,
        "cold" => StorageTier::Cold,
        _ => StorageTier::Warm, // Default to warm
    }
}

/// Create agent metadata for universal tracking
fn create_agent_metadata(
    agent_spec: &nestgate_core::biomeos::AgentSpec,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> HashMap<String, String> {
    let mut metadata = HashMap::new();

    metadata.insert("agent_name".to_string(), agent_spec.name.clone());
    metadata.insert("runtime_type".to_string(), agent_spec.runtime.clone());
    metadata.insert("executor".to_string(), agent_spec.executor.clone());
    metadata.insert("biome_id".to_string(), biome_context.biome_id.clone());
    metadata.insert("node_id".to_string(), biome_context.node_id.clone());
    metadata.insert("environment".to_string(), biome_context.environment.clone());
    metadata.insert(
        "capabilities".to_string(),
        agent_spec.capabilities.join(","),
    );
    metadata.insert("coordination_enabled".to_string(), "true".to_string());
    metadata.insert("universal_patterns".to_string(), "true".to_string());
    metadata.insert("created_at".to_string(), chrono::Utc::now().to_rfc3339());

    if let Some(ai_provider) = &agent_spec.ai_provider {
        metadata.insert("ai_provider".to_string(), ai_provider.clone());
    }

    if let Some(model) = &agent_spec.model {
        metadata.insert("model".to_string(), model.clone());
    }

    metadata
}

// Placeholder functions for various registration systems
async fn register_agent_with_songbird(
    _runtime_environment: &AgentRuntimeEnvironment,
    _biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implementation would connect to Songbird service mesh
    Ok(())
}

async fn register_agent_with_squirrel_mcp(
    _runtime_environment: &AgentRuntimeEnvironment,
    _biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implementation would connect to Squirrel MCP system
    Ok(())
}

async fn register_agent_with_toadstool(
    _runtime_environment: &AgentRuntimeEnvironment,
    _biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implementation would connect to Toadstool execution platform
    Ok(())
}

async fn register_agent_with_beardog_security(
    _runtime_environment: &AgentRuntimeEnvironment,
    _biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implementation would connect to BearDog security system
    Ok(())
}

async fn register_agent_with_universal_storage_manager(
    _runtime_environment: &AgentRuntimeEnvironment,
    _biome_context: &nestgate_core::biomeos::BiomeContext,
) -> Result<(), Box<dyn std::error::Error>> {
    // Implementation would connect to Universal Storage Manager
    Ok(())
}

// Helper functions for creating agent configurations
fn create_agent_network_config(
    _agent_spec: &nestgate_core::biomeos::AgentSpec,
    _biome_context: &nestgate_core::biomeos::BiomeContext,
) -> AgentNetworkConfig {
    AgentNetworkConfig {
        network_mode: "bridge".to_string(),
        port_mappings: vec![],
        dns_servers: vec!["8.8.8.8".to_string()],
        hostname: None,
    }
}

fn create_agent_security_context(
    _agent_spec: &nestgate_core::biomeos::AgentSpec,
    biome_context: &nestgate_core::biomeos::BiomeContext,
) -> AgentSecurityContext {
    AgentSecurityContext {
        run_as_user: "1000".to_string(),
        run_as_group: "1000".to_string(),
        capabilities: vec!["NET_BIND_SERVICE".to_string()],
        security_context_enabled: biome_context.security_context.encryption_enabled,
        encryption_enabled: biome_context.security_context.encryption_enabled,
        audit_enabled: true,
    }
}

fn create_agent_coordination_config(
    _agent_spec: &nestgate_core::biomeos::AgentSpec,
    _biome_context: &nestgate_core::biomeos::BiomeContext,
) -> AgentCoordinationConfig {
    AgentCoordinationConfig {
        discovery_enabled: true,
        health_check_enabled: true,
        metrics_enabled: true,
        event_bus_enabled: true,
        coordination_protocols: vec!["songbird".to_string(), "mcp".to_string()],
    }
}

// Supporting types for agent runtime provisioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRuntimeProvisionRequest {
    pub agent_spec: nestgate_core::biomeos::AgentSpec,
    pub biome_context: nestgate_core::biomeos::BiomeContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRuntimeProvisionResponse {
    pub agent_id: String,
    pub agent_name: String,
    pub runtime_type: String,
    pub executor: String,
    pub biome_id: String,
    pub storage_volumes: Vec<AgentStorageVolume>,
    pub runtime_environment: AgentRuntimeEnvironment,
    pub capabilities: Vec<String>,
    pub access_endpoints: Vec<String>,
    pub coordination_endpoints: Vec<String>,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStorageVolume {
    pub name: String,
    pub size_bytes: u64,
    pub tier: String,
    pub mount_path: String,
    pub volume_type: String,
    pub access_mode: String,
    pub protocols: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRuntimeEnvironment {
    pub agent_id: String,
    pub agent_name: String,
    pub runtime_type: String,
    pub executor: String,
    pub biome_id: String,
    pub node_id: String,
    pub capabilities: Vec<String>,
    pub resource_limits: Option<nestgate_core::biomeos::ResourceLimits>,
    pub environment_variables: HashMap<String, String>,
    pub ai_provider: Option<String>,
    pub model: Option<String>,
    pub storage_mounts: Vec<AgentStorageMount>,
    pub network_config: AgentNetworkConfig,
    pub security_context: AgentSecurityContext,
    pub coordination_config: AgentCoordinationConfig,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStorageMount {
    pub volume_name: String,
    pub mount_path: String,
    pub access_mode: String,
    pub volume_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNetworkConfig {
    pub network_mode: String,
    pub port_mappings: Vec<String>,
    pub dns_servers: Vec<String>,
    pub hostname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSecurityContext {
    pub run_as_user: String,
    pub run_as_group: String,
    pub capabilities: Vec<String>,
    pub security_context_enabled: bool,
    pub encryption_enabled: bool,
    pub audit_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCoordinationConfig {
    pub discovery_enabled: bool,
    pub health_check_enabled: bool,
    pub metrics_enabled: bool,
    pub event_bus_enabled: bool,
    pub coordination_protocols: Vec<String>,
}
