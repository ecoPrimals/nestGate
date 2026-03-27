//! Storage domain semantic methods
//!
//! Handles storage.* and storage.dataset.* semantic method routing.

use super::SemanticRouter;
use crate::error::{NestGateError, Result};
use crate::rpc::tarpc_types::DatasetParams;
use serde_json::{json, Value};

/// Route storage.put → store_object
pub(super) async fn storage_put(router: &SemanticRouter, params: Value) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;
    let data_b64 = params["data"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("data", "base64 string required"))?;

    // Decode base64
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let data = STANDARD.decode(data_b64).map_err(|e| {
        NestGateError::invalid_input_with_field("data", format!("Invalid base64: {}", e))
    })?;

    // Call internal implementation
    let result = router.client.store_object(dataset, key, data, None).await?;

    serde_json::to_value(&result).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize object info: {}", e),
            "semantic_router",
        )
    })
}

/// Route storage.get → retrieve_object
pub(super) async fn storage_get(router: &SemanticRouter, params: Value) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;

    let data = router.client.retrieve_object(dataset, key).await?;

    // Encode to base64 for transport
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    let data_b64 = STANDARD.encode(&data);

    Ok(json!({
        "data": data_b64,
        "size": data.len()
    }))
}

/// Route storage.delete → delete_object
pub(super) async fn storage_delete(router: &SemanticRouter, params: Value) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;

    let result = router.client.delete_object(dataset, key).await?;

    Ok(json!({
        "success": result.success,
        "message": result.message
    }))
}

/// Route storage.list → list_objects
pub(super) async fn storage_list(router: &SemanticRouter, params: Value) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let prefix = params["prefix"].as_str().map(String::from);

    let objects = router.client.list_objects(dataset, prefix, None).await?;

    Ok(json!({
        "objects": objects,
        "count": objects.len()
    }))
}

/// Route storage.exists → check if object exists
pub(super) async fn storage_exists(router: &SemanticRouter, params: Value) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;

    let exists = router
        .client
        .get_object_metadata(dataset, key)
        .await
        .is_ok();

    Ok(json!({ "exists": exists }))
}

/// Route storage.metadata → get object metadata
pub(super) async fn storage_metadata(router: &SemanticRouter, params: Value) -> Result<Value> {
    let dataset = params["dataset"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("dataset", "string required"))?;
    let key = params["key"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("key", "string required"))?;

    let meta = router.client.get_object_metadata(dataset, key).await?;

    serde_json::to_value(&meta).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize object metadata: {}", e),
            "semantic_router",
        )
    })
}

// ==================== DATASET OPERATIONS ====================

/// Route storage.dataset.create → create_dataset
pub(super) async fn dataset_create(router: &SemanticRouter, params: Value) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "string required"))?;
    let description = params["description"].as_str().unwrap_or("").to_string();

    let dataset_params = DatasetParams {
        description: Some(description),
        ..Default::default()
    };

    let dataset = router.client.create_dataset(name, dataset_params).await?;

    Ok(json!({
        "name": dataset.name,
        "created_at": dataset.created_at,
        "status": dataset.status
    }))
}

/// Route storage.dataset.get → get_dataset
pub(super) async fn dataset_get(router: &SemanticRouter, params: Value) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "string required"))?;

    let dataset = router.client.get_dataset(name).await?;

    serde_json::to_value(dataset).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize dataset: {}", e),
            "semantic_router",
        )
    })
}

/// Route storage.dataset.list → list_datasets
pub(super) async fn dataset_list(router: &SemanticRouter, _params: Value) -> Result<Value> {
    let datasets = router.client.list_datasets().await?;

    Ok(json!({
        "datasets": datasets,
        "count": datasets.len()
    }))
}

/// Route storage.dataset.delete → delete_dataset
pub(super) async fn dataset_delete(router: &SemanticRouter, params: Value) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "string required"))?;

    let result = router.client.delete_dataset(name).await?;

    Ok(json!({
        "success": result.success,
        "message": result.message
    }))
}
