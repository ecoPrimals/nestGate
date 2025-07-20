//! Storage Management Handler Functions
//!
//! This module contains all the HTTP handlers for storage-related operations
//! including provisioning, listing, and managing storage resources.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::OnceLock;
use tracing::{error, info};
use uuid::Uuid;

use super::super::types::{ByobStorageProvider, ListQuery, ProvisionRequest, UsageQuery};
use super::super::ZfsStorageProvider;
use crate::routes::AppState;

// Global storage provider instance
static STORAGE_PROVIDER: OnceLock<ZfsStorageProvider> = OnceLock::new();

fn get_storage_provider() -> &'static ZfsStorageProvider {
    STORAGE_PROVIDER.get_or_init(ZfsStorageProvider::new)
}

/// Get storage overview
pub async fn get_storage_overview(State(_state): State<AppState>) -> impl IntoResponse {
    info!("📊 Getting storage overview");

    let provider = get_storage_provider();
    match provider.get_storage_overview().await {
        Ok(overview) => Json(overview),
        Err(e) => {
            error!("❌ Failed to get storage overview: {}", e);
            Json(json!({
                "error": "Failed to get storage overview",
                "message": e,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Get storage health
pub async fn get_storage_health(State(_state): State<AppState>) -> impl IntoResponse {
    info!("🏥 Getting storage health");

    let provider = get_storage_provider();
    match provider.get_health().await {
        Ok(health) => Json(health),
        Err(e) => {
            error!("❌ Failed to get storage health: {}", e);
            Json(json!({
                "error": "Failed to get storage health",
                "message": e,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Provision storage
pub async fn provision_storage(
    State(_state): State<AppState>,
    Json(request): Json<ProvisionRequest>,
) -> impl IntoResponse {
    info!(
        "🏗️ Provisioning storage for deployment: {}",
        request.deployment_id
    );

    let provider = get_storage_provider();
    match provider.provision_storage(&request).await {
        Ok(response) => (StatusCode::CREATED, Json(response)),
        Err(e) => {
            error!("❌ Failed to provision storage: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(super::super::types::ByobStorageResponse {
                    deployment_id: request.deployment_id,
                    status: "failed".to_string(),
                    message: format!("Storage provisioning failed: {e}"),
                    dataset_name: None,
                    mount_point: None,
                    created_at: Some(chrono::Utc::now()),
                    metadata: std::collections::HashMap::new(),
                }),
            )
        }
    }
}

/// List storage resources
pub async fn list_storage(
    State(_state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> impl IntoResponse {
    info!("📋 Listing storage resources");

    let provider = get_storage_provider();
    match provider.list_storage(&query).await {
        Ok(storage_list) => Json(json!({
            "storage": storage_list,
            "count": storage_list.len(),
            "timestamp": chrono::Utc::now()
        })),
        Err(e) => {
            error!("❌ Failed to list storage: {}", e);
            Json(json!({
                "error": "Failed to list storage",
                "message": e,
                "storage": [],
                "count": 0,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Get storage status
pub async fn get_storage_status(
    State(_state): State<AppState>,
    Path(deployment_id): Path<Uuid>,
) -> impl IntoResponse {
    info!(
        "📖 Getting storage status for deployment: {}",
        deployment_id
    );

    let provider = get_storage_provider();
    match provider.get_storage_status(&deployment_id).await {
        Ok(response) => Json(response),
        Err(e) => {
            error!("❌ Failed to get storage status: {}", e);
            Json(super::super::types::ByobStorageResponse {
                deployment_id,
                status: "error".to_string(),
                message: format!("Failed to get storage status: {e}"),
                dataset_name: None,
                mount_point: None,
                created_at: Some(chrono::Utc::now()),
                metadata: std::collections::HashMap::new(),
            })
        }
    }
}

/// Remove storage
pub async fn remove_storage(
    State(_state): State<AppState>,
    Path(deployment_id): Path<Uuid>,
) -> impl IntoResponse {
    info!("🗑️ Removing storage for deployment: {}", deployment_id);

    let provider = get_storage_provider();
    match provider.remove_storage(&deployment_id).await {
        Ok(()) => Json(json!({
            "deployment_id": deployment_id,
            "status": "removed",
            "message": "Storage removed successfully",
            "timestamp": chrono::Utc::now()
        })),
        Err(e) => {
            error!("❌ Failed to remove storage: {}", e);
            Json(json!({
                "error": "Failed to remove storage",
                "message": e,
                "deployment_id": deployment_id,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}

/// Get storage usage
pub async fn get_storage_usage(
    State(_state): State<AppState>,
    Path(deployment_id): Path<Uuid>,
    Query(_query): Query<UsageQuery>,
) -> impl IntoResponse {
    info!("📊 Getting storage usage for deployment: {}", deployment_id);

    let provider = get_storage_provider();
    match provider.get_storage_usage(&deployment_id).await {
        Ok(usage) => Json(usage),
        Err(e) => {
            error!("❌ Failed to get storage usage: {}", e);
            Json(json!({
                "error": "Failed to get storage usage",
                "message": e,
                "deployment_id": deployment_id,
                "timestamp": chrono::Utc::now()
            }))
        }
    }
}
