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

/// Provision storage from biome.yaml manifest
/// This is the key function specified in the biomeOS Integration Specification
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

    // Create dataset path based on biome context
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

    match state
        .zfs_manager
        .dataset_manager
        .create_dataset(
            &request.volume_spec.name,
            &dataset_parent,
            storage_tier.clone(),
        )
        .await
    {
        Ok(dataset_info) => {
            // Determine mount point
            let mount_point = request.volume_spec.mount_path.clone().unwrap_or_else(|| {
                format!(
                    "{}/{}/{}",
                    nestgate_core::constants::biomeos_defaults::DEFAULT_BIOMEOS_MOUNT_PREFIX,
                    request.biome_context.biome_id,
                    request.volume_spec.name
                )
            });

            // Create volume info response
            let volume_info = nestgate_core::biomeos::VolumeInfo {
                id: format!(
                    "vol-{}-{}",
                    request.biome_context.biome_id, request.volume_spec.name
                ),
                name: request.volume_spec.name.clone(),
                size_bytes,
                used_bytes: dataset_info.used_space,
                available_bytes: dataset_info.available_space,
                tier: storage_tier,
                mount_point,
                status: nestgate_core::biomeos::VolumeStatus::Available,
                created_at: chrono::Utc::now(),
                metadata: {
                    let mut metadata = HashMap::new();
                    metadata.insert(
                        "biome_id".to_string(),
                        request.biome_context.biome_id.clone(),
                    );
                    metadata.insert("node_id".to_string(), request.biome_context.node_id.clone());
                    metadata.insert(
                        "provisioner".to_string(),
                        request.volume_spec.provisioner.clone(),
                    );
                    metadata.insert(
                        "environment".to_string(),
                        request.biome_context.environment.clone(),
                    );
                    metadata
                },
            };

            info!(
                "Successfully provisioned volume {} for biome {}",
                request.volume_spec.name, request.biome_context.biome_id
            );

            // Register with Songbird service mesh for cross-Primal discovery
            // This enables other Primals (Toadstool, Squirrel) to discover this volume
            if let Err(e) =
                register_volume_with_songbird(&volume_info, &request.biome_context).await
            {
                warn!("Failed to register volume with Songbird: {}", e);
                // Continue execution as this is not critical for basic volume provisioning
            }

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
                            status: nestgate_core::biomeos::VolumeStatus::Available,
                            created_at: chrono::Utc::now(),
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
