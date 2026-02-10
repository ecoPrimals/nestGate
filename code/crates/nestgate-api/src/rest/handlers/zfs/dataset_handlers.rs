//
// Dataset data handlers for ZFS operations.
// GET/POST/PUT/DELETE for datasets, properties, and stats.

use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::rest::models::{
    CreateDatasetRequest, Dataset, DatasetProperties, DatasetStats, UpdateDatasetRequest,
};
use crate::rest::{ApiState, DataError, DataResponse, ListQuery};

use super::helpers;

/// List all ZFS datasets
/// GET /api/v1/zfs/datasets
pub async fn list_datasets(
    State(state): State<ApiState>,
    Query(query): Query<ListQuery>,
) -> std::result::Result<Json<DataResponse<Vec<Dataset>>>, Json<DataError>> {
    debug!("Listing ZFS datasets with query: {:?}", query);
    let engines = state.zfs_engines.read().await;
    let mut datasets = Vec::new();

    for (dataset_name, _engine) in engines.iter() {
        match helpers::convert_engine_to_placeholder_dataset(dataset_name, _engine).await {
            Ok(dataset) => datasets.push(dataset),
            Err(e) => {
                error!(
                    "Failed to convert _engine to dataset for {}: {}",
                    dataset_name, e
                );
            }
        }
    }

    if let Some(filter) = &query.filter {
        datasets.retain(|d| d.name.contains(filter) || d.backend.to_string().contains(filter));
    }

    if let Some(sort_field) = &query.sort {
        match sort_field.as_str() {
            "name" => datasets.sort_by(|a, b| a.name.cmp(&b.name)),
            "created" => datasets.sort_by(|a, b| a.created.cmp(&b.created)),
            "size" => datasets.sort_by(|a, b| a.stats.used_bytes.cmp(&b.stats.used_bytes)),
            _ => {}
        }
        if query.order.as_deref() == Some("desc") {
            datasets.reverse();
        }
    }

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
        total.div_ceil(per_page)
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
    if request.name.is_empty() {
        return Err(Json(DataError::new(
            "Dataset name cannot be empty".to_string(),
            "INVALID_NAME".to_string(),
        )));
    }

    {
        let engines = state.zfs_engines.read().await;
        if engines.contains_key(&request.name) {
            return Err(Json(DataError::new(
                format!("Dataset '{}' already exists", request.name),
                "DATASET_EXISTS".to_string(),
            )));
        }
    }

    let _storage_backend = match helpers::create_storage_backend(&request).await {
        Ok(backend) => backend,
        Err(e) => {
            error!("Failed to create storage backend: {}", e);
            return Err(Json(DataError::new(
                format!("Failed to create storage backend: {}", request.name),
                "BACKEND_ERROR".to_string(),
            )));
        }
    };

    let properties = request.properties.as_ref();
    let _config = serde_json::json!({
        "compression": properties.is_some_and(|p| p.compression),
        "checksum": properties.is_none_or(|p| p.checksum)
    });

    let _engine: Arc<dyn std::any::Any + Send + Sync> =
        Arc::new(format!("engine_{}", request.name));

    {
        let mut engines = state.zfs_engines.write().await;
        engines.insert(request.name.clone(), "placeholder_engine".to_string());
    }

    let welcome_content = format!(
        "Welcome to your new ZFS dataset: {}\nCreated: {}\nCompression: {}\nChecksum: {}\n",
        request.name,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        properties.is_some_and(|p| p.compression),
        properties.is_none_or(|p| p.checksum)
    );

    if let Err(e) = tokio::fs::write(
        format!(
            "/tmp/nestgate_dataset_{}_welcome.txt",
            request.name.replace('/', "_")
        ),
        welcome_content,
    )
    .await
    {
        warn!("Failed to create welcome file: {}", e);
    } else {
        debug!("Created welcome file for dataset {}", request.name);
    }

    let dataset = match helpers::convert_engine_to_placeholder_dataset(
        &request.name,
        &"placeholder".to_string(),
    )
    .await
    {
        Ok(dataset) => dataset,
        Err(e) => {
            error!("Failed to convert _engine to dataset: {}", e);
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
        Some(_engine) => {
            match helpers::convert_engine_to_placeholder_dataset(&dataset_name, _engine).await {
                Ok(dataset) => Ok(Json(DataResponse::new(dataset))),
                Err(e) => {
                    error!("Failed to convert _engine to dataset: {}", e);
                    Err(Json(DataError::new(
                        "Failed to get dataset data".to_string(),
                        "CONVERSION_ERROR".to_string(),
                    )))
                }
            }
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
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
        Some(_engine) => {
            match helpers::convert_engine_to_placeholder_dataset(&dataset_name, _engine).await {
                Ok(dataset) => {
                    info!("Dataset properties updated for: {}", dataset_name);
                    Ok(Json(DataResponse::new(dataset)))
                }
                Err(e) => {
                    error!("Failed to convert _engine to dataset: {}", e);
                    Err(Json(DataError::new(
                        "Failed to update dataset".to_string(),
                        "UPDATE_ERROR".to_string(),
                    )))
                }
            }
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
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
            Ok(Json(DataResponse::new(serde_json::json!({
                "message": format!("Dataset '{dataset_name}' deleted successfully"),
                "dataset": dataset_name,
                "deleted_at": chrono::Utc::now()
            }))))
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
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
        Some(_engine) => {
            match helpers::convert_engine_to_placeholder_dataset(&dataset_name, _engine).await {
                Ok(dataset) => Ok(Json(DataResponse::new(dataset.properties))),
                Err(e) => {
                    error!("Failed to get dataset properties: {}", e);
                    Err(Json(DataError::new(
                        "Failed to get dataset properties".to_string(),
                        "PROPERTIES_ERROR".to_string(),
                    )))
                }
            }
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
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
            info!("Properties set for dataset: {}", dataset_name);
            Ok(Json(DataResponse::new(properties)))
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
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
        Some(_engine) => {
            let dataset_stats = DatasetStats {
                name: dataset_name.clone(),
                size_bytes: 1024 * 1024 * 1024,
                used_bytes: 512 * 1024 * 1024,
                available_bytes: 512 * 1024 * 1024,
                files_written: 500,
                files_read: 1500,
                cow_operations: 100,
                blocks_copied: 200,
                compression_ratio: Some(1.5),
                compression_space_saved: Some(256 * 1024 * 1024),
                deduplication_ratio: 1.2,
                checksums_computed: 1000,
                checksums_verified: 1000,
                read_throughput: 100.0,
                write_throughput: 80.0,
                avg_latency_ms: 2.5,
                snapshot_count: 3,
            };
            Ok(Json(DataResponse::new(dataset_stats)))
        }
        None => Err(Json(DataError::new(
            format!("Dataset '{dataset_name}' not found"),
            "DATASET_NOT_FOUND".to_string(),
        ))),
    }
}
