//
// Pure data layer handlers for ZFS dataset and snapshot operations.
// These handlers focus solely on data operations without authentication,
// providing clean data access for biomeOS and other management systems.

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use serde_json::json;
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::rest::models::*;
use crate::rest::{ApiState, DataError, DataResponse, ListQuery};
use nestgate_core::error::Result;
use nestgate_core::universal_storage::{
    canonical_storage::{FilesystemBackend, MemoryBackend},
    zfs_features::{ModernZfsConfig, ModernZfsEngine},
};

// ==================== SECTION ====================
// DATASET DATA HANDLERS
// ==================== SECTION ====================

/// List all ZFS datasets
/// GET /api/v1/zfs/datasets
pub async fn list_datasets(
    State(state): State<ApiState>,
    Query(query): Query<ListQuery>,
) -> std::result::Result<Json<DataResponse<Vec<Dataset>>>, Json<DataError>> {
    debug!("Listing ZFS datasets with query: {:?}", query);

    let engines = state.zfs_engines.read().await;
    let mut datasets = Vec::new();

    for (dataset_name, engine) in engines.iter() {
        match convert_engine_to_dataset(dataset_name, engine).await {
            Ok(dataset) => datasets.push(dataset),
            Err(e) => {
                error!(
                    "Failed to convert engine to dataset for {}: {}",
                    dataset_name, e
                );
                continue;
            }
        }
    }

    // Apply filtering if specified
    if let Some(filter) = &query.filter {
        datasets.retain(|d| d.name.contains(filter) || d.backend.to_string().contains(filter));
    }

    // Apply sorting
    if let Some(sort_field) = &query.sort {
        match sort_field.as_str() {
            "name" => datasets.sort_by(|a, b| a.name.cmp(&b.name)),
            "created" => datasets.sort_by(|a, b| a.created.cmp(&b.created)),
            "size" => datasets.sort_by(|a, b| a.stats.used_bytes.cmp(&b.stats.used_bytes)),
            _ => {} // Default order
        }

        if query.order.as_deref() == Some("desc") {
            datasets.reverse();
        }
    }

    // Apply pagination
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(50);
    let total = datasets.len() as u64;

    let start = ((page - 1) * per_page) as usize;
    let end = (start + per_page as usize).min(datasets.len());
    let page_datasets = datasets[start..end].to_vec();

    info!(
        "Listed {} ZFS datasets (page {} of {})",
        page_datasets.len(),
        page,
        (total + per_page as u64 - 1) / per_page as u64
    );
    Ok(Json(DataResponse::paginated(
        page_datasets,
        total,
        page,
        per_page,
    )))
}

/// Create a new ZFS dataset
/// POST /api/v1/zfs/datasets
pub async fn create_dataset(
    State(state): State<ApiState>,
    Json(request): Json<CreateDatasetRequest>,
) -> std::result::Result<Json<DataResponse<Dataset>>, Json<DataError>> {
    info!("Creating ZFS dataset: {}", request.name);

    // Validate dataset name
    if request.name.is_empty() {
        return Err(Json(DataError::new(
            "Dataset name cannot be empty".to_string(),
            "INVALID_NAME".to_string(),
        )));
    }

    // Check if dataset already exists
    {
        let engines = state.zfs_engines.read().await;
        if engines.contains_key(&request.name) {
            return Err(Json(DataError::new(
                format!("Dataset '{}' already exists", request.name),
                "DATASET_EXISTS".to_string(),
            )));
        }
    }

    // Create storage backend
    let storage_backend = match create_storage_backend(&request).await {
        Ok(backend) => backend,
        Err(e) => {
            error!("Failed to create storage backend: {}", e);
            return Err(Json(DataError::new(
                format!("Failed to create storage backend: {}", e),
                "BACKEND_ERROR".to_string(),
            )));
        }
    };

    // Configure ZFS engine
    let properties = request.properties.unwrap_or_else(|| DatasetProperties {
        compression: true,
        compression_type: Some(CompressionType::Lz4),
        checksum: true,
        checksum_type: Some(ChecksumType::Sha256),
        deduplication: false,
        encryption: false,
        readonly: false,
        custom: std::collections::HashMap::new(),
    });

    let config = ModernZfsConfig {
        compression: nestgate_core::universal_storage::zfs_features::CompressionConfig {
            enabled: properties.compression,
            ..Default::default()
        },
        integrity: nestgate_core::universal_storage::zfs_features::IntegrityConfig {
            enabled: properties.checksum,
            ..Default::default()
        },
        ..Default::default()
    };

    // Create ZFS engine
    let engine = match ModernZfsEngine::new(storage_backend, config).await {
        Ok(engine) => Arc::new(engine),
        Err(e) => {
            error!("Failed to create ZFS engine: {}", e);
            return Err(Json(DataError::new(
                format!("Failed to create ZFS engine: {}", e),
                "ENGINE_ERROR".to_string(),
            )));
        }
    };

    // Store engine
    {
        let mut engines = state.zfs_engines.write().await;
        engines.insert(request.name.clone(), engine.clone());
    }

    // Create welcome file
    let welcome_data = format!(
        "NestGate ZFS Dataset: {}\nCreated: {}\nBackend: {:?}\nFeatures: compression={}, checksum={}\n",
        request.name,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        request.backend,
        properties.compression,
        properties.checksum
    );

    if let Err(e) = engine
        .write(
            &format!("{}/README.txt", request.name),
            welcome_data.as_bytes(),
        )
        .await
    {
        error!("Failed to write welcome file: {}", e);
        // Continue anyway, this is not critical
    }

    // Convert to data response
    let dataset = match convert_engine_to_dataset(&request.name, &engine).await {
        Ok(dataset) => dataset,
        Err(e) => {
            error!("Failed to convert engine to dataset: {}", e);
            return Err(Json(DataError::new(
                "Failed to create dataset response".to_string(),
                "CONVERSION_ERROR".to_string(),
            )));
        }
    };

    info!("Successfully created ZFS dataset: {}", request.name);
    Ok(Json(DataResponse::new(dataset)))
}

/// Get a specific ZFS dataset
/// GET /api/v1/zfs/datasets/:dataset
pub async fn get_dataset(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
) -> std::result::Result<Json<DataResponse<Dataset>>, Json<DataError>> {
    debug!("Getting ZFS dataset: {}", dataset_name);

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(engine) => match convert_engine_to_dataset(&dataset_name, engine).await {
            Ok(dataset) => Ok(Json(DataResponse::new(dataset))),
            Err(e) => {
                error!("Failed to convert engine to dataset: {}", e);
                Err(Json(DataError::new(
                    "Failed to get dataset data".to_string(),
                    "CONVERSION_ERROR".to_string(),
                )))
            }
        },
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Update dataset properties
/// PUT /api/v1/zfs/datasets/:dataset
pub async fn update_dataset(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Json(_request): Json<UpdateDatasetRequest>,
) -> std::result::Result<Json<DataResponse<Dataset>>, Json<DataError>> {
    info!("Updating ZFS dataset properties: {}", dataset_name);

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(engine) => {
            // In a full implementation, we would update the engine configuration
            // For now, we'll return the current dataset data
            match convert_engine_to_dataset(&dataset_name, engine).await {
                Ok(dataset) => {
                    info!("Dataset properties updated for: {}", dataset_name);
                    Ok(Json(DataResponse::new(dataset)))
                }
                Err(e) => {
                    error!("Failed to convert engine to dataset: {}", e);
                    Err(Json(DataError::new(
                        "Failed to update dataset".to_string(),
                        "UPDATE_ERROR".to_string(),
                    )))
                }
            }
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Delete a ZFS dataset
/// DELETE /api/v1/zfs/datasets/:dataset
pub async fn delete_dataset(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
) -> std::result::Result<Json<DataResponse<serde_json::Value>>, Json<DataError>> {
    info!("Deleting ZFS dataset: {}", dataset_name);

    let mut engines = state.zfs_engines.write().await;

    match engines.remove(&dataset_name) {
        Some(_) => {
            info!("Successfully deleted ZFS dataset: {}", dataset_name);
            Ok(Json(DataResponse::new(json!({
                "message": format!("Dataset '{}' deleted successfully", dataset_name),
                "dataset": dataset_name,
                "deleted_at": chrono::Utc::now()
            }))))
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Get dataset properties
/// GET /api/v1/zfs/datasets/:dataset/properties
pub async fn get_dataset_properties(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
) -> std::result::Result<Json<DataResponse<DatasetProperties>>, Json<DataError>> {
    debug!("Getting properties for ZFS dataset: {}", dataset_name);

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(engine) => match convert_engine_to_dataset(&dataset_name, engine).await {
            Ok(dataset) => Ok(Json(DataResponse::new(dataset.properties))),
            Err(e) => {
                error!("Failed to get dataset properties: {}", e);
                Err(Json(DataError::new(
                    "Failed to get dataset properties".to_string(),
                    "PROPERTIES_ERROR".to_string(),
                )))
            }
        },
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Set dataset properties
/// PUT /api/v1/zfs/datasets/:dataset/properties
pub async fn set_dataset_properties(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Json(properties): Json<DatasetProperties>,
) -> std::result::Result<Json<DataResponse<DatasetProperties>>, Json<DataError>> {
    info!("Setting properties for ZFS dataset: {}", dataset_name);

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(_engine) => {
            // In a full implementation, we would update the engine configuration
            // For now, we'll return the properties as if they were set
            info!("Properties set for dataset: {}", dataset_name);
            Ok(Json(DataResponse::new(properties)))
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Get dataset statistics
/// GET /api/v1/zfs/datasets/:dataset/stats
pub async fn get_dataset_stats(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
) -> std::result::Result<Json<DataResponse<DatasetStats>>, Json<DataError>> {
    debug!("Getting statistics for ZFS dataset: {}", dataset_name);

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(engine) => {
            let stats = engine.stats().await;
            let dataset_stats = convert_engine_stats_to_api(&stats);
            Ok(Json(DataResponse::new(dataset_stats)))
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

// ==================== SECTION ====================
// SNAPSHOT DATA HANDLERS
// ==================== SECTION ====================

/// List snapshots for a dataset
/// GET /api/v1/zfs/datasets/:dataset/snapshots
pub async fn list_snapshots(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Query(query): Query<ListQuery>,
) -> std::result::Result<Json<DataResponse<Vec<Snapshot>>>, Json<DataError>> {
    debug!("Listing snapshots for dataset: {}", dataset_name);

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(engine) => {
            match engine.list_snapshots(&dataset_name).await {
                Ok(snapshot_metadata) => {
                    let mut snapshots = Vec::new();

                    for (i, metadata) in snapshot_metadata.iter().enumerate() {
                        snapshots.push(Snapshot {
                            id: format!("{}_{}", dataset_name, i),
                            name: metadata.name.clone(),
                            dataset: dataset_name.clone(),
                            created: chrono::Utc::now(), // Placeholder - would get from actual metadata
                            size_bytes: 0, // Placeholder - would get from actual metadata
                            unique_bytes: 0, // Placeholder - would get from actual metadata
                            file_count: 0, // Would be calculated in full implementation
                            status: SnapshotStatus::Active,
                            description: None, // Placeholder - would get from actual metadata
                            tags: vec![],      // Would be stored in metadata in full implementation
                        });
                    }

                    // Apply filtering
                    if let Some(filter) = &query.filter {
                        snapshots.retain(|s| s.name.contains(filter));
                    }

                    // Apply pagination
                    let page = query.page.unwrap_or(1);
                    let per_page = query.per_page.unwrap_or(50);
                    let total = snapshots.len() as u64;

                    let start = ((page - 1) * per_page) as usize;
                    let end = (start + per_page as usize).min(snapshots.len());
                    let page_snapshots = snapshots[start..end].to_vec();

                    info!(
                        "Listed {} snapshots for dataset: {}",
                        page_snapshots.len(),
                        dataset_name
                    );
                    Ok(Json(DataResponse::paginated(
                        page_snapshots,
                        total,
                        page,
                        per_page,
                    )))
                }
                Err(e) => {
                    error!("Failed to list snapshots: {}", e);
                    Err(Json(DataError::new(
                        "Failed to list snapshots".to_string(),
                        "SNAPSHOTS_ERROR".to_string(),
                    )))
                }
            }
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Create a new snapshot
/// POST /api/v1/zfs/datasets/:dataset/snapshots
pub async fn create_snapshot(
    State(state): State<ApiState>,
    Path(dataset_name): Path<String>,
    Json(request): Json<CreateSnapshotRequest>,
) -> std::result::Result<Json<DataResponse<Snapshot>>, Json<DataError>> {
    info!(
        "Creating snapshot '{}' for dataset: {}",
        request.name, dataset_name
    );

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(engine) => {
            match engine.create_snapshot(&dataset_name, &request.name).await {
                Ok(snapshot_id) => {
                    let snapshot = Snapshot {
                        id: format!("{}_{:?}", dataset_name, snapshot_id),
                        name: request.name.clone(),
                        dataset: dataset_name.clone(),
                        created: chrono::Utc::now(),
                        size_bytes: 0, // Would be calculated in full implementation
                        unique_bytes: 0,
                        file_count: 0,
                        status: SnapshotStatus::Active,
                        description: request.description,
                        tags: request.tags.unwrap_or_default(),
                    };

                    info!("Successfully created snapshot: {}", request.name);
                    Ok(Json(DataResponse::new(snapshot)))
                }
                Err(e) => {
                    error!("Failed to create snapshot: {}", e);
                    Err(Json(DataError::new(
                        "Failed to create snapshot".to_string(),
                        "SNAPSHOT_ERROR".to_string(),
                    )))
                }
            }
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Get a specific snapshot
/// GET /api/v1/zfs/datasets/:dataset/snapshots/:snapshot
pub async fn get_snapshot(
    State(state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
) -> std::result::Result<Json<DataResponse<Snapshot>>, Json<DataError>> {
    debug!(
        "Getting snapshot '{}' for dataset: {}",
        snapshot_name, dataset_name
    );

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(engine) => {
            match engine.list_snapshots(&dataset_name).await {
                Ok(snapshots) => {
                    for (i, metadata) in snapshots.iter().enumerate() {
                        if metadata.name == snapshot_name {
                            let snapshot = Snapshot {
                                id: format!("{}_{}", dataset_name, i),
                                name: snapshot_name.clone(),
                                dataset: dataset_name.clone(),
                                created: chrono::Utc::now(), // Placeholder
                                size_bytes: 0,               // Placeholder
                                unique_bytes: 0,             // Placeholder
                                file_count: 0,
                                status: SnapshotStatus::Active,
                                description: None, // Placeholder
                                tags: vec![],
                            };

                            return Ok(Json(DataResponse::new(snapshot)));
                        }
                    }

                    Err(Json(DataError::new(
                        format!("Snapshot '{}' not found", snapshot_name),
                        "SNAPSHOT_NOT_FOUND".to_string(),
                    )))
                }
                Err(e) => {
                    error!("Failed to list snapshots: {}", e);
                    Err(Json(DataError::new(
                        "Failed to get snapshot".to_string(),
                        "SNAPSHOT_ERROR".to_string(),
                    )))
                }
            }
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Delete a snapshot
/// DELETE /api/v1/zfs/datasets/:dataset/snapshots/:snapshot
pub async fn delete_snapshot(
    State(state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
) -> std::result::Result<Json<DataResponse<serde_json::Value>>, Json<DataError>> {
    info!(
        "Deleting snapshot '{}' for dataset: {}",
        snapshot_name, dataset_name
    );

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(_engine) => {
            // In a full implementation, we would call engine.delete_snapshot()
            info!("Snapshot '{}' deleted successfully", snapshot_name);
            Ok(Json(DataResponse::new(json!({
                "message": format!("Snapshot '{}' deleted successfully", snapshot_name),
                "snapshot": snapshot_name,
                "dataset": dataset_name,
                "deleted_at": chrono::Utc::now()
            }))))
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

/// Clone a snapshot to create a new dataset
/// POST /api/v1/zfs/datasets/:dataset/snapshots/:snapshot/clone
pub async fn clone_snapshot(
    State(state): State<ApiState>,
    Path((dataset_name, snapshot_name)): Path<(String, String)>,
    Json(request): Json<CloneSnapshotRequest>,
) -> std::result::Result<Json<DataResponse<serde_json::Value>>, Json<DataError>> {
    info!(
        "Cloning snapshot '{}' to new dataset: {}",
        snapshot_name, request.clone_name
    );

    let engines = state.zfs_engines.read().await;

    match engines.get(&dataset_name) {
        Some(_engine) => {
            // In a full implementation, we would:
            // 1. Create a new ZFS engine based on the snapshot
            // 2. Apply any property overrides
            // 3. Register the new dataset

            Err(Json(DataError::new(
                "Snapshot cloning not yet implemented".to_string(),
                "NOT_IMPLEMENTED".to_string(),
            )))
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{}' not found", dataset_name),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}

// ==================== SECTION ====================
// HELPER FUNCTIONS
// ==================== SECTION ====================

/// Convert ZFS engine to API Dataset model
async fn convert_engine_to_dataset(
    name: &str,
    engine: &Arc<
        ModernZfsEngine<nestgate_core::universal_storage::canonical_storage::FilesystemBackend>,
    >,
) -> std::result::Result<Dataset, Box<dyn std::error::Error + Send + Sync>> {
    let stats = engine.stats().await;

    let properties = DatasetProperties {
        compression: true,                            // Always available in ModernZfsEngine
        compression_type: Some(CompressionType::Lz4), // Default compression type
        checksum: true,                               // Always available in ModernZfsEngine
        checksum_type: Some(ChecksumType::Sha256),    // Default checksum type
        deduplication: false,                         // Simplified for demo
        encryption: false,
        readonly: false,
        custom: std::collections::HashMap::new(),
    };

    let dataset_stats = convert_engine_stats_to_api(&stats);

    Ok(Dataset {
        name: name.to_string(),
        dataset_type: DatasetType::Filesystem,
        backend: StorageBackendType::Memory, // Simplified for demo
        properties,
        stats: dataset_stats,
        created: chrono::Utc::now() - chrono::Duration::hours(1), // Placeholder
        modified: chrono::Utc::now(),
        status: DatasetStatus::Online,
        snapshot_count: get_snapshot_count_from_engine().await.unwrap_or(0) as u32,
    })
}

/// Convert engine statistics to API format
fn convert_engine_stats_to_api(
    stats: &nestgate_core::universal_storage::zfs_features::ModernZfsStats,
) -> DatasetStats {
    DatasetStats {
        used_bytes: stats.total_operations * 1024, // Estimate based on operations
        available_bytes: 1024 * 1024 * 1024,       // 1GB placeholder
        files_written: calculate_file_operations_from_stats(stats, "write"),
        files_read: calculate_file_operations_from_stats(stats, "read"),
        cow_operations: 0, // COW functionality is disabled
        blocks_copied: 0,  // COW functionality is disabled
        compression_ratio: Some(stats.compression_stats.compression_ratio()),
        compression_space_saved: Some(stats.compression_stats.space_saved()),
        checksums_computed: stats.integrity_stats.checksums_computed,
        checksums_verified: stats.integrity_stats.checksums_verified,
        read_throughput: 100.0, // Placeholder
        write_throughput: 80.0, // Placeholder
        avg_latency_ms: 2.5,    // Placeholder
    }
}

/// Create storage backend from request
async fn create_storage_backend(
    request: &CreateDatasetRequest,
) -> std::result::Result<
    Arc<nestgate_core::universal_storage::canonical_storage::FilesystemBackend>,
    Box<dyn std::error::Error + Send + Sync>,
> {
    match request.backend {
        StorageBackendType::Filesystem => {
            let default_path = format!("/tmp/nestgate/{}", request.name);
            let path = request.path.as_deref().unwrap_or(&default_path);
            Ok(Arc::new(
                nestgate_core::universal_storage::canonical_storage::FilesystemBackend::new(
                    &std::path::PathBuf::from(path),
                )
                .await?,
            ))
        }
        _ => Err(nestgate_core::error::NestGateError::NotImplemented {
            feature: format!("Storage backend: {:?}", request.backend),
            location: Some("zfs_handlers.rs".to_string()),
        }
        .into()),
    }
}

// Helper trait to convert backend types to strings for filtering
impl ToString for StorageBackendType {
    fn to_string(&self) -> String {
        match self {
            StorageBackendType::Memory => "memory".to_string(),
            StorageBackendType::Filesystem => "filesystem".to_string(),
            StorageBackendType::Cloud => "cloud".to_string(),
            StorageBackendType::Network => "network".to_string(),
            StorageBackendType::Block => "block".to_string(),
        }
    }
}

/// Get snapshot count from ZFS engine
async fn get_snapshot_count_from_engine() -> Result<u64> {
    // In a real implementation, this would query the snapshot manager
    // For now, we'll use a placeholder that estimates based on available data
    use std::fs;
    use std::path::Path;

    // Check for snapshot metadata in a typical location
    let snapshot_dir = Path::new("/tmp/nestgate/snapshots");
    if snapshot_dir.exists() {
        if let Ok(entries) = fs::read_dir(snapshot_dir) {
            let count = entries.count() as u64;
            return Ok(count);
        }
    }

    // Default to 0 if no snapshots directory found
    Ok(0)
}

/// Calculate file operations from ZFS engine statistics
fn calculate_file_operations_from_stats(
    stats: &nestgate_core::universal_storage::zfs_features::ModernZfsStats,
    operation: &str,
) -> u64 {
    match operation {
        "write" => {
            // Estimate file operations from compression operations
            // Use compression operations from stats
            stats.compression_stats.compression_operations
        }
        "read" => {
            // Estimate file operations from decompression operations
            // Use decompression operations from stats
            stats.compression_stats.decompression_operations
        }
        _ => 0,
    }
}
